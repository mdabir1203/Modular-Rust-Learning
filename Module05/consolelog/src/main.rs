// include the structs only
pub mod utility;

use utility::{Logger, ConsoleLog};

fn main() {
    let console_logger = ConsoleLog;
    let logging = Logger::new(console_logger);

    logging.log("This is a message for logging");
    logging.log("42");
}
