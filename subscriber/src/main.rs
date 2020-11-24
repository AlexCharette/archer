use log::info;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, atomic::AtomicBool};
// use structopt::StructOpt;

use database::{fetch_last_known_blocks, PgPool};

pub mod event_handling;
pub mod subscriber;

use event_handling::get_events_handler;
use subscriber::Subscriber;

const KNOWN_COUNT: i64 = 15;

// #[derive(StructOpt)]
// #[structopt(rename_all = "kebab-case")]
// pub struct DatabaseArgs {
//     #[structopt(default_value, long)]
//     pub db_name: String,
//     #[structopt(default_value, long)]
//     pub db_user: String,
//     #[structopt(default_value, long)]
//     pub db_password: String,
//     #[structopt(default_value, long)]
//     pub db_host: String,
//     #[structopt(default_value, long)]
//     pub db_port: String,

// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let configuration = archer_config::get_configuration().expect("Could not retrieve config");

    // let options = DatabaseArgs::from_args();

    let pool = database::establish_connection();

    // "ws:localhost:8008/subscriptions"
    let endpoint = format!(
        "ws:{}:{}/subscriptions",
        configuration.subscriber.host, configuration.subscriber.port
    );

    subscribe(pool, &endpoint)?;

    Ok(())
}

pub fn subscribe(pool: PgPool, endpoint: &str) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting subscriber");

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
    // stop subscriber
    subscriber.stop()?;

    info!("Subscriber shut down successfully");

    Ok(())
}
