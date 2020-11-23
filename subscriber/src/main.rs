use log::info;
use std::cell::RefCell;
use std::rc::Rc;

use database::{establish_connection, fetch_last_known_blocks, PgPool};

pub mod event_handling;
pub mod subscriber;

use event_handling::get_events_handler;
use subscriber::Subscriber;

const KNOWN_COUNT: i64 = 15;

pub struct Cli {
    pub db_name: String,
    pub db_user: String,
    pub db_password: String,
    pub db_host: String,
    pub db_port: String,
}

/*
def parse_args(args):
    parser = argparse.ArgumentParser(add_help=False)

    subparsers = parser.add_subparsers(title='subcommands', dest='command')
    subparsers.required = True

    database_parser = argparse.ArgumentParser(add_help=False)
    database_parser.add_argument(
        '--db-name',
        help='The name of the database',
        default='simple-supply')
    database_parser.add_argument(
        '--db-host',
        help='The host of the database',
        default='localhost')
    database_parser.add_argument(
        '--db-port',
        help='The port of the database',
        default='5432')
    database_parser.add_argument(
        '--db-user',
        help='The authorized user of the database',
        default='sawtooth')
    database_parser.add_argument(
        '--db-password',
        help="The authorized user's password for database access",
        default='sawtooth')
    database_parser.add_argument(
        '-v', '--verbose',
        action='count',
        default=0,
        help='Increase output sent to stderr')

    subparsers.add_parser(
        'init',
        parents=[database_parser])

    subscribe_parser = subparsers.add_parser(
        'subscribe',
        parents=[database_parser])
    subscribe_parser.add_argument(
        '-C', '--connect',
        help='The url of the validator to subscribe to',
        default='tcp://localhost:4004')

    return parser.parse_args(args)

def main():
    opts = parse_args(sys.argv[1:])
    init_logger(opts.verbose)

    if opts.command == 'subscribe':
        do_subscribe(opts)
    elif opts.command == 'init':
        do_init(opts)
    else:
        LOGGER.exception('Invalid command: "%s"', opts.command)
*/

fn main() {
    // let command = "subscribe";
    // let options = Cli;
    // match command {
    //     "subscribe" => subscribe(options),
    //     "init" => init(options),
    //     _ => error!("Invalid command"), // TODO convert to error
    // };
}

/*
def do_init(opts):
    LOGGER.info('Initializing subscriber...')
    try:
        dsn = 'dbname={} user={} password={} host={} port={}'.format(
            opts.db_name,
            opts.db_user,
            opts.db_password,
            opts.db_host,
            opts.db_port)
        database = Database(dsn)
        database.connect()
        database.create_tables()

    except Exception as err:  # pylint: disable=broad-except
        LOGGER.exception('Unable to initialize subscriber database: %s', err)

    finally:
        database.disconnect()
*/

pub fn init(options: Cli) -> Result<(), Box<dyn std::error::Error>> {
    info!("Initializing subscriber");

    let _dsn = format!(
        "dbname={} user={} password={} host={} port={}",
        options.db_name, options.db_user, options.db_password, options.db_host, options.db_port
    );
    // create connection with DSN
    // connect to database
    // create DB tables (optional?)

    Ok(())
}

/*
def do_subscribe(opts):
    LOGGER.info('Starting subscriber...')
    try:
        dsn = 'dbname={} user={} password={} host={} port={}'.format(
            opts.db_name,
            opts.db_user,
            opts.db_password,
            opts.db_host,
            opts.db_port)

        database = Database(dsn)
        database.connect()
        subscriber = Subscriber(opts.connect)
        subscriber.add_handler(get_events_handler(database))
        known_blocks = database.fetch_last_known_blocks(KNOWN_COUNT)
        known_ids = [block['block_id'] for block in known_blocks]
        subscriber.start(known_ids=known_ids)

    except KeyboardInterrupt:
        sys.exit(0)

    except Exception as err:  # pylint: disable=broad-except
        LOGGER.exception(err)
        sys.exit(1)

    finally:
        try:
            database.disconnect()
            subscriber.stop()
        except UnboundLocalError:
            pass

    LOGGER.info('Subscriber shut down successfully')
*/

pub fn subscribe(
    options: Cli,
    pool: PgPool,
    endpoint: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting subscriber");

    let _dsn = format!(
        "dbname={} user={} password={} host={} port={}",
        options.db_name, options.db_user, options.db_password, options.db_host, options.db_port
    );

    let connection = Rc::new(RefCell::new(
        pool.get()
            .expect("Error establishing a connection with the database"),
    ));

    // create connection with DSN
    // connect to database
    let mut subscriber = Subscriber::new(endpoint);

    let known_blocks = fetch_last_known_blocks(KNOWN_COUNT, &Rc::clone(&connection).borrow())?;

    subscriber.add_handler(get_events_handler(pool));

    let known_ids: Vec<String> = known_blocks
        .iter()
        .map(|block| block.block_id.clone())
        .collect();

    subscriber.start(Some(&known_ids))?;

    // On interrupt
    // disconnect from DB
    // stop subscriber

    info!("Subscriber shut down successfully");

    Ok(())
}
