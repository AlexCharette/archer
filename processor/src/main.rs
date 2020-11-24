use archer::NAME;
use archer_config::get_configuration;
use log::info;
use sawtooth_sdk::processor::TransactionProcessor;

pub mod handler;
pub mod payload;
pub mod state;

use handler::ArcherTransactionHandler;

fn main() {
    info!("Starting the processor");

    let settings = get_configuration().expect("Could not retrieve configuration");

    let endpoint = format!("tcp://{}:{}", settings.validator.host, settings.validator.port);

    let mut processor: TransactionProcessor = TransactionProcessor::new(&endpoint);

    let handler: ArcherTransactionHandler = ArcherTransactionHandler::new(NAME);

    processor.add_handler(&handler);
    processor.start();
}
