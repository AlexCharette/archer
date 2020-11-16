
#[macro_use]
extern crate diesel;

use log::info;
use std::error::Error;
use std::collections::HashMap;

mod archer;
mod processor;
mod protobuf;
mod database;
mod events;
mod api;

/* 
// TODO !!! Establish runtime/startup flow
// TODO !! When a merchant submits an update to the state (transaction),
* they must submit something to prove they are registered in the system.
* Therefore, some sort of record of merchant keys (?) should be kept.
// TODO !  Create point calculator
*/

fn main() -> Result<(), Box<dyn Error>> {

    let mut endpoints = HashMap::<archer::ArcherModules, String>::new();
    endpoints.insert(archer::ArcherModules::RestApi, String::from("localhost:8000"));
    endpoints.insert(archer::ArcherModules::Processor, String::from("tcp://localhost:4004"));
    endpoints.insert(archer::ArcherModules::Subscriber, String::from("ws:localhost:8008/subscriptions"));
    endpoints.insert(archer::ArcherModules::Sawtooth, String::from("http://localhost:8008"));
    
    env_logger::init();

    let connection_pool: database::PgPool = database::establish_connection();

    processor::run_processor(
        endpoints.get(&archer::ArcherModules::Processor).expect("No endpoint found for processor")
    );

    events::run_subscriber(
        endpoints.get(&archer::ArcherModules::Subscriber).expect("No endpoint found for subscriber")
    );

    api::run_actix(
        connection_pool.clone(), 
        String::from(endpoints.get(&archer::ArcherModules::RestApi).expect("No endpoint found for rest api"))
    ).expect("Error running Actix");

    Ok(())
}
