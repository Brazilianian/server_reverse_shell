use log::{debug, error, info, trace, warn};
use log4rs;
use log::Level;

pub fn init_log_file() {
    log4rs::init_file("E:\\programming\\rust\\server_reverse_shell\\src\\logging_config.yaml", Default::default()).unwrap()
}

pub fn write_log(message: String, level: Level) {
    match level {
        Level::Info => {
            info!("{}", message);
        }
        Level::Error => {
            error!("{}", message);
        }
        Level::Trace => {
            trace!("{}", message);
        }
        Level::Debug => {
            debug!("{}", message);
        }
        Level::Warn => {
            warn!("{}", message);
        }
    }
}

