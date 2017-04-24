//! Extend the rust library log.
//!
//! Provides support for various handlers to send the log records to the appropriate destination.
//!
//! # Dependencies
//!
//! * [log](https://doc.rust-lang.org/log/log/index.html) - base logging library.
//! * [time](https://doc.rust-lang.org/time/time/index.html) - simple time handling to extent log records.
//! * [lazy-static](http://rust-lang-nursery.github.io/lazy-static.rs/lazy_static/index.html) - a macro for declaring lazily evaluated statics. This is used to manage
//! handlers.
//! * [rustc-serialize](https://doc.rust-lang.org/rustc-serialize) - adds the ability to serialize and deserialize a `ExtendedLogRecord`
//!   using the `rustc-serialize` crate.
//!
//! By default, `log-tools` can be depended on with:
//!
//! ```toml
//! [dependencies]
//! log-tools = "0.0.1"
//! ```
//!
//! # Example
//!
//! Let's create a logger which will format log records into JSON and print Info in stdout and
//! Errors into a file.
//!
//! ```rust
//! use log::LogLevelFilter;
//! use log_handlers::ExtendedLogger;
//! use log_handlers::formatter::json;
//!
//! fn main() {
//!     ExtendedLogger::init(LogLevelFilter::Info).unwrap();
//!     ExtendedLogger::add_stdout_handler(Some(LogLevelFilter::Info), Some(json));
//!     ExtendedLogger::add_file_handler("/tmp/log-error.txt", Some(LogLevelFilter::Error), Some(json));
//!
//!     info!("done");
//!     error!(":-(");
//! }
//! ```
//!
//! It will print into stdout:
//!
//! ```
//! {"level":"INFO","levelno":3,"msg":"done","target":"log_handlers::tests","timestamp":1493044134,"module":"log_handlers::tests","file":"src/tests.rs","line":28,"date":"2017-04-24T14:28:54Z"}
//! {"level":"ERROR","levelno":1,"msg":":-(","target":"log_handlers::tests","timestamp":1493044134,"module":"log_handlers::tests","file":"src/tests.rs","line":29,"date":"2017-04-24T14:28:54Z"}
//! ```
//!
//! And in the file `/tmp/log-error.txt`:
//!
//! ```
//! {"level":"ERROR","levelno":1,"msg":":-(","target":"log_handlers::tests","timestamp":1493044134,"module":"log_handlers::tests","file":"src/tests.rs","line":29,"date":"2017-04-24T14:28:54Z"}
//! ```
//!
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate rustc_serialize;
extern crate time;

pub mod handlers;
pub mod formatter;

#[cfg(test)]
mod tests;

use handlers::streams::file::FileHandler;
use handlers::streams::net::TCPHandler;
use handlers::streams::stdout::StdoutHandler;
use handlers::{Handler, HANDLERS, NullHandler};
use log::{LogRecord, LogLevel, LogMetadata, LogLevelFilter, SetLoggerError};
use std::str::FromStr;

/// A custom logger
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

    pub fn add_null_handler() {
        ExtendedLogger::add_handler(Handler::from(NullHandler {}))
    }
    pub fn add_stdout_handler(level: Option<LogLevelFilter>, formatter: Option<fn(&ExtendedLogRecord) -> String>) {
        ExtendedLogger::add_handler(Handler::from(StdoutHandler::new(level, formatter)))
    }
    pub fn add_file_handler(filename: &'static str, level: Option<LogLevelFilter>, formatter: Option<fn(&ExtendedLogRecord) -> String>) {
        ExtendedLogger::add_handler(Handler::from(FileHandler::new(filename, level, formatter)))
    }
    pub fn add_tcp_handler(address: &'static str, level: Option<LogLevelFilter>, formatter: Option<fn(&ExtendedLogRecord) -> String>) {
        ExtendedLogger::add_handler(Handler::from(TCPHandler::new(address, level, formatter)))
    }
    /// Append a new handler.
    fn add_handler(hdlr: Handler) {
        HANDLERS.lock().unwrap().push(hdlr);
    }
}

/// Extended log record.
///
/// A `ExtendedLogRecord` derive from Debug and RustcEncodable to facilitate format.
#[derive(Debug, RustcEncodable)]
pub struct ExtendedLogRecord<'a> {
    /// The message creation formatted according to RFC 3339. RFC 3339 is compatible with ISO 8601.
    pub date: String,
    /// The source file containing the message.
    pub file: &'a str,
    /// The verbosity level name of the message.
    pub level: String,
    /// The verbosity level value of the message.
    pub levelno: u32,
    /// The line containing the message.
    pub line: u32,
    /// The module path of the message.
    pub module: &'a str,
    /// The message body.
    pub msg: String,
    /// The message factory.
    pub target: String,
    /// The message creation timestamp.
    pub timestamp: i64,
}

/// Construct a `ExtendedLogRecord` via a conversion from a `LogRecord`.
impl<'a> From<&'a LogRecord<'a>> for ExtendedLogRecord<'a> {
    fn from(record: &'a LogRecord<'a>) -> ExtendedLogRecord<'a> {
        ExtendedLogRecord::new(
            record.location().__file,
            record.level(),
            record.location().__line,
            record.location().__module_path,
            format!("{}", record.args()),
            String::from(record.target()),
        )
    }
}

impl<'a> ExtendedLogRecord<'a> {
    /// `ExtendedLogRecord` factory, may not be used directly
    pub fn new(file: &'a str, level: LogLevel, line: u32, module: &'a str, msg: String, target: String) -> ExtendedLogRecord<'a> {
        /// We get the current UTC time as log message creation date.
        let now = time::now_utc();

        ExtendedLogRecord {
            date: format!("{}", now.rfc3339()),
            file: file,
            level: level.to_string(),
            levelno: level as u32,
            line: line,
            module: module,
            msg: msg,
            target: target,
            timestamp: now.to_timespec().sec,
        }
    }

    /// Recover log record level by its name to allow level comparison.
    pub fn level(&self) -> LogLevel {
        LogLevel::from_str(self.level.as_str()).unwrap()
    }
}
