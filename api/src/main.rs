// use std::convert::TryFrom;
use serde::Deserialize;
use std::sync::Arc;
use tracing::{error, info};
// use diesel::pg::PgConnection;
use actix_web::dev::{Server, ServiceRequest};
use actix_web::{web, App, Error, HttpServer};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub mod auth;
pub mod messenger;
pub mod routes;
pub mod services;

use archer_config::get_configuration;
use database::{establish_connection, PgPool};

pub struct AppData {
    pool: PgPool,
    aes_key: String,
    secret_key: String,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let pool = establish_connection();

    let configuration = get_configuration().expect("Failed to read configuration.");

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    let listener = TcpListener::bind(address)?;

    run(listener, pool)?.await?;

    Ok(())
}

fn run(listener: TcpListener, pool: PgPool) -> Result<Server, std::io::Error> {
    

    // TODO !! find solution for keys in production
    let data = web::Data::new(AppData {
        pool,
        aes_key: String::from("ffffffffffffffffffffffffffffffff"),
        secret_key: String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890"),
    });

    let server = HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(validator);
        
        App::new()
            .wrap(auth)
            .wrap(TracingLogger)
            .data(data.clone())
            .route("/health-check", web::get().to(routes::health_check))
            .route("/balance", web::get().to(routes::get_balance))
            .route("/withdraw", web::put().to(routes::withdraw))
            .route("/deposit", web::put().to(routes::deposit))
            .route("/add-acount", web::post().to(routes::add_account))
            .route("/add-merchant", web::post().to(routes::add_merchant))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

async fn validator(
    request: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let config = request
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);

    match auth::validate_token(credentials.token()).await {
        Ok(res) => {
            if res == true {
                Ok(request)
            } else {
                Err(AuthenticationError::from(config).into())
            }
        }
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}

/*
def encrypt_private_key(aes_key, public_key, private_key):
    init_vector = bytes.fromhex(public_key[:32])
    cipher = AES.new(bytes.fromhex(aes_key), AES.MODE_CBC, init_vector)
    return cipher.encrypt(private_key)
*/

fn encrypt_private_key(aes_key: String, public_key: String, private_key: String) -> String {
    // get bytes from hex (public key[..32])
    // generate a cipher using AES
    // encrypt private key
    String::from("")
}

/*
def decrypt_private_key(aes_key, public_key, encrypted_private_key):
    init_vector = bytes.fromhex(public_key[:32])
    cipher = AES.new(bytes.fromhex(aes_key), AES.MODE_CBC, init_vector)
    private_key = cipher.decrypt(bytes.fromhex(encrypted_private_key))
    return private_key
*/

fn decrypt_private_key(aes_key: String, public_key: String, private_key: String) -> String {
    // get bytes from hex (public key[..32])
    // generate a cipher using AES
    // get private key by decrypting it using the cipher
    String::from("")
}

/*
async def _authorize(self, request):
        token = request.headers.get('AUTHORIZATION')
        if token is None:
            raise ApiUnauthorized('No auth token provided')
        token_prefixes = ('Bearer', 'Token')
        for prefix in token_prefixes:
            if prefix in token:
                token = token.partition(prefix)[2].strip()
        try:
            token_dict = deserialize_auth_token(request.app['secret_key'],
                                                token)
        except BadSignature:
            raise ApiUnauthorized('Invalid auth token')
        public_key = token_dict.get('public_key')

        auth_resource = await self._database.fetch_auth_resource(public_key)
        if auth_resource is None:
            raise ApiUnauthorized('Token is not associated with an agent')
        return decrypt_private_key(request.app['aes_key'],
                                   public_key,
                                   auth_resource['encrypted_private_key'])
*/

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
