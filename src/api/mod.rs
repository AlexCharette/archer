
// use std::convert::TryFrom;
use std::sync::Arc;
use log::{error, info};
use serde::Deserialize;
// use diesel::pg::PgConnection;
use actix_web::{App, Error, get, HttpResponse, HttpServer, post, put, Responder, web};
// use tracing_subscriber::{ fmt::Subscriber as TracingSubscriber, EnvFilter as TracingEnvFilter};
// use config::{ Config, Environment };

pub mod services;
pub mod messenger;

use crate::database::{fetch_balance, PgPool};
use messenger::Messenger;

struct ServerData {
    pool: PgPool,
}

#[derive(Deserialize)]
struct AccountData {
    name: String,
    number: u32,
}

#[derive(Deserialize)]
struct MerchantData {
    public_key: String,
    password: String,
}

#[derive(Deserialize)]
struct UpdateBalanceJson {
    name: String,
    number: u32,
    amount: i32,
}

#[get("/balance")]
async fn get_balance(pool: web::Data<PgPool>, account_data: web::Json<AccountData>) -> Result<HttpResponse, Error> {

    let connection = pool.get().expect("Could not get connection from pool");

    let name = account_data.name.clone();
    let number = account_data.number;

    let balance = web::block(move || fetch_balance(name, number, &*connection))
        .await
        .map_err(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        });

    match balance {
        Ok(balance) => Ok(HttpResponse::Ok().json(balance)),
        Err(_) => {
            let res = HttpResponse::NotFound()
                .body(format!("No account found with name and number: {}, {}", account_data.name, account_data.number));
            Ok(res)
        } 
    }
}

#[put("/deposit")]
async fn deposit(_pool: web::Data<PgPool>, account_data: web::Json<UpdateBalanceJson>) -> impl Responder {
    let messenger: Messenger = Messenger::new("secp256k1");

    // TODO verify private key creation process
    let (_public_key, private_key): (String, String) = messenger.get_new_key_pair();

    messenger.send_deposit_txn(&private_key, account_data.name.to_owned(), account_data.number, account_data.amount).await;

    HttpResponse::Ok().body("Payload received") // TODO
}

#[put("/withdraw")]
async fn withdraw(_pool: web::Data<PgPool>, account_data: web::Json<UpdateBalanceJson>) -> impl Responder {
    let messenger: Messenger = Messenger::new("secp256k1");

    // TODO verify private key creation process
    let (_public_key, private_key): (String, String) = messenger.get_new_key_pair();

    messenger.send_withdraw_txn(&private_key, account_data.name.to_owned(), account_data.number, account_data.amount).await;

    HttpResponse::Ok().body("Payload received") // TODO
}

#[post("/add-account")]
async fn add_account(_pool: web::Data<PgPool>, account_data: web::Json<AccountData>) -> impl Responder {
    let messenger: Messenger = Messenger::new("secp256k1");

    // TODO verify private key creation process
    let (_public_key, private_key): (String, String) = messenger.get_new_key_pair();

    messenger.send_add_account_txn(&private_key, account_data.name.to_owned(), account_data.number).await;
    HttpResponse::Ok().body("Payload received") // TODO
}

#[post("/auth")]
async fn authenticate(_pool: web::Data<PgPool>, merchant_data: web::Json<MerchantData>) -> impl Responder {
    HttpResponse::Ok().body("Payload received") // TODO
}

#[actix_web::main]
pub async fn run_actix(pool: PgPool, endpoint: String) -> std::io::Result<()> {

    info!("Running actix");

    info!("Starting server at {}", &endpoint);

    HttpServer::new(move || {
        App::new()
            .data(ServerData { pool: pool.clone(), })
            .service(get_balance)
    })
    .bind(&endpoint)?
    .run()
    .await
}

/*
def parse_args(args):
    parser = argparse.ArgumentParser(
        description='Starts the Simple Supply REST API')

    parser.add_argument(
        '-B', '--bind',
        help='identify host and port for api to run on',
        default='localhost:8000')
    parser.add_argument(
        '-C', '--connect',
        help='specify URL to connect to a running validator',
        default='tcp://localhost:4004')
    parser.add_argument(
        '-t', '--timeout',
        help='set time (in seconds) to wait for a validator response',
        default=500)
    parser.add_argument(
        '--db-name',
        help='The name of the database',
        default='simple-supply')
    parser.add_argument(
        '--db-host',
        help='The host of the database',
        default='localhost')
    parser.add_argument(
        '--db-port',
        help='The port of the database',
        default='5432')
    parser.add_argument(
        '--db-user',
        help='The authorized user of the database',
        default='sawtooth')
    parser.add_argument(
        '--db-password',
        help="The authorized user's password for database access",
        default='sawtooth')
    parser.add_argument(
        '-v', '--verbose',
        action='count',
        default=0,
        help='enable more verbose output to stderr')

    return parser.parse_args(args)


def start_rest_api(host, port, messenger, database):
    loop = asyncio.get_event_loop()
    asyncio.ensure_future(database.connect())

    app = web.Application(loop=loop)
    # WARNING: UNSAFE KEY STORAGE
    # In a production application these keys should be passed in more securely
    app['aes_key'] = 'ffffffffffffffffffffffffffffffff'
    app['secret_key'] = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890'

    messenger.open_validator_connection()

    handler = RouteHandler(loop, messenger, database)

    app.router.add_post('/authentication', handler.authenticate)

    app.router.add_post('/agents', handler.create_agent)
    app.router.add_get('/agents', handler.list_agents)
    app.router.add_get('/agents/{agent_id}', handler.fetch_agent)

    app.router.add_post('/records', handler.create_record)
    app.router.add_get('/records', handler.list_records)
    app.router.add_get('/records/{record_id}', handler.fetch_record)
    app.router.add_post(
        '/records/{record_id}/transfer', handler.transfer_record)
    app.router.add_post('/records/{record_id}/update', handler.update_record)

    LOGGER.info('Starting Simple Supply REST API on %s:%s', host, port)
    web.run_app(
        app,
        host=host,
        port=port,
        access_log=LOGGER,
        access_log_format='%r: %s status, %b size, in %Tf s')


def main():
    loop = ZMQEventLoop()
    asyncio.set_event_loop(loop)

    try:
        opts = parse_args(sys.argv[1:])

        init_console_logging(verbose_level=opts.verbose)

        validator_url = opts.connect
        if "tcp://" not in validator_url:
            validator_url = "tcp://" + validator_url
        messenger = Messenger(validator_url)

        database = Database(
            opts.db_host,
            opts.db_port,
            opts.db_name,
            opts.db_user,
            opts.db_password,
            loop)

        try:
            host, port = opts.bind.split(":")
            port = int(port)
        except ValueError:
            print("Unable to parse binding {}: Must be in the format"
                  " host:port".format(opts.bind))
            sys.exit(1)

        start_rest_api(host, port, messenger, database)
    except Exception as err:  # pylint: disable=broad-except
        LOGGER.exception(err)
        sys.exit(1)
    finally:
        database.disconnect()
        messenger.close_validator_connection()

*/