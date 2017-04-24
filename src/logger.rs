use log::{self, LogRecord, LogLevel, LogMetadata, LogLevelFilter, SetLoggerError};
use handlers::{Handler, HANDLERS};
use record::ExtendedLogRecord;

// A custom logger
pub struct ExtendedLogger {
    /// The current maximum log level of the logger.
    level: LogLevel
}

impl log::Log for ExtendedLogger {
    /// Determines if a log message with the specified metadata would be
    /// logged.
    ///
    /// This is used by the `log_enabled!` macro to allow callers to avoid
    /// expensive computation of log message arguments if the message would be
    /// discarded anyway.
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= self.level
    }

    /// Convert the `LogRecord` into `ExtendedLogRecord` and send it to the handlers.
    ///
    /// Implementations of `log` should perform all necessary filtering internally.
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
    /// Create a new instance of the logger.
    ///
    /// `level` is the maximum log level of the logger.
    fn new(level: LogLevel) -> ExtendedLogger {
        ExtendedLogger { level: level }
    }

    /// Initialize the logger factory.
    ///
    /// `level` is the maximum log level filter of the logger.
    pub fn init(level: LogLevelFilter) -> Result<(), SetLoggerError> {
        log::set_logger(|max_log_level| {
            max_log_level.set(level);
            Box::new(ExtendedLogger::new(level.to_log_level().unwrap_or(LogLevel::Info)))
        })
    }

    /// Append a new handler.
    pub fn add_handler(hdlr: Handler) {
        HANDLERS.lock().unwrap().push(hdlr);
    }
}