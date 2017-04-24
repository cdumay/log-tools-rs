use log::{self, LogRecord, LogLevel, LogMetadata, LogLevelFilter, SetLoggerError};
use handlers::{Handler, HANDLERS};
use record::ExtendedLogRecord;

pub struct ExtendedLogger {
    level: LogLevel
}

impl log::Log for ExtendedLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            let ext_record = ExtendedLogRecord::from(record);
            for hdlr in HANDLERS.lock().unwrap().iter_mut() {
                hdlr.handle(&ext_record)
            }
        }
    }
}

impl ExtendedLogger {
    pub fn new(level: LogLevel) -> ExtendedLogger {
        ExtendedLogger { level: level}
    }
    pub fn init(level: LogLevelFilter) -> Result<(), SetLoggerError> {
        log::set_logger(|max_log_level| {
            max_log_level.set(level);
            Box::new(ExtendedLogger::new(level.to_log_level().unwrap_or(LogLevel::Info)))
        })
    }
    pub fn add_handler(hdlr: Handler) {
        HANDLERS.lock().unwrap().push(hdlr);
    }
}