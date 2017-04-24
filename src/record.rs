use log::{LogRecord, LogLevel};
use time;
use std::str::FromStr;

/// Extended log record.
///
/// A `ExtendedLogRecord` derive from Debug and RustcEncodable to facilitate format.
#[derive(Debug, RustcEncodable)]
pub struct ExtendedLogRecord<'a> {
    /// The verbosity level name of the message.
    pub level: String,
    /// The verbosity level value of the message.
    pub levelno: u32,
    /// The message body.
    pub msg: String,
    /// The message factory.
    pub target: String,
    /// The message creation timestamp.
    pub timestamp: i64,
    /// The module path of the message.
    pub module: &'a str,
    /// The source file containing the message.
    pub file: &'a str,
    /// The line containing the message.
    pub line: u32,
    /// The message creation formatted according to RFC 3339. RFC 3339 is compatible with ISO 8601.
    pub date: String
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