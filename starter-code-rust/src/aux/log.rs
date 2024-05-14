use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};
use log::Log;

use crate::{serial_print, serial_println};

// use log::{debug, ...}
// debug!("{}", message);
// error!("{}", message);
// info!("{}", message);
// warn!("{}", message);
// trace!("{}", message);

struct Logger;

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool { metadata.level() <= Level::Trace }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) { return; }

        match record.level() {
            Level::Debug => serial_print!("debug: "),
            Level::Error => serial_print!("error: "),
            Level::Info => serial_print!("info: "),
            Level::Warn => serial_print!("warn: "),
            Level::Trace => serial_print!("trace: "),
        }
        
        serial_println!("{}", record.args());
    }

    fn flush(&self) {}
}

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&Logger)?;
    log::set_max_level(LevelFilter::Trace);

    Ok(())
}
