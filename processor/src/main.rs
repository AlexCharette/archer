use log::info;
use sawtooth_sdk::processor::TransactionProcessor;

pub mod handler;
pub mod payload;
pub mod state;

use crate::archer::NAME;
use handler::ArcherTransactionHandler;

pub fn run_processor(endpoint: &str) {
    info!("Starting the processor");

    let mut processor: TransactionProcessor = TransactionProcessor::new(endpoint);

    let handler: ArcherTransactionHandler = ArcherTransactionHandler::new(NAME);

    processor.add_handler(&handler);
    processor.start();
}

/*
def parse_args(args):
    parser = argparse.ArgumentParser(
        formatter_class=argparse.RawTextHelpFormatter)

    parser.add_argument(
        '-C', '--connect',
        default='tcp://localhost:4004',
        help='Endpoint for the validator connection')

    parser.add_argument(
        '-v', '--verbose',
        action='count',
        default=0,
        help='Increase output sent to stderr')

    return parser.parse_args(args)


def main(args=None):
    if args is None:
        args = sys.argv[1:]
    opts = parse_args(args)
    processor = None
    try:
        init_console_logging(verbose_level=opts.verbose)

        processor = TransactionProcessor(url=opts.connect)
        handler = SimpleSupplyHandler()
        processor.add_handler(handler)
        processor.start()
    except KeyboardInterrupt:
        pass
    except Exception as err:  # pylint: disable=broad-except
        print("Error: {}".format(err))
    finally:
        if processor is not None:
            processor.stop()

*/
