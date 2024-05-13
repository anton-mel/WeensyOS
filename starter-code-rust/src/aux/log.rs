use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};
use log::Log;

use crate::{serial_print, serial_println};

struct Logger;

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool { metadata.level() <= Level::Trace }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) { return; }

        match record.level() {
            Level::Debug => serial_print!("\x1b[1;32m debug:\x1b[0m "),
            Level::Error => serial_print!("\x1b[1;31m error:\x1b[0m "),
            Level::Info => serial_print!("\x1b[1;36m info:\x1b[0m "),
            Level::Warn => serial_print!("\x1b[1;33m warn:\x1b[0m "),
            Level::Trace => serial_print!("\x1b[1;37m trace:\x1b[0m "),
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
