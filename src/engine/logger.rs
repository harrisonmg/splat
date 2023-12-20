use std::sync::Mutex;

use log::{LevelFilter, Log, Metadata, Record, SetLoggerError};

struct LoggerInner {
    buffer: Mutex<Vec<String>>,
}

impl LoggerInner {
    fn new() -> Self {
        Self {
            buffer: Mutex::new(Vec::new()),
        }
    }
}

impl Log for LoggerInner {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if let Ok(mut buf) = self.buffer.lock() {
            buf.push(record.args().to_string());
        }
    }

    fn flush(&self) {}
}

pub struct Logger(LoggerInner);

impl Logger {
    pub fn setup() -> Result<&'static Self, SetLoggerError> {
        let logger = Box::leak(Box::new(Self(LoggerInner::new())));
        log::set_logger(&logger.0)?;
        log::set_max_level(LevelFilter::Debug);
        Ok(logger)
    }

    pub fn drain(&self) -> Vec<String> {
        if let Ok(mut buf) = self.0.buffer.lock() {
            buf.drain(..).collect()
        } else {
            Vec::new()
        }
    }
}

#[macro_export]
macro_rules! dbg {
    ($val: expr) => {
        if cfg!(debug_assertions) {
            log::debug!("{:?} = {:?}", stringify!($val), $val)
        }
    };
}
