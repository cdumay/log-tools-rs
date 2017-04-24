use log::{LogRecord, LogLevel};
use time;
use std::str::FromStr;

#[derive(Debug, RustcEncodable)]
pub struct ExtendedLogRecord<'a> {
    pub level: String,
    pub levelno: u32,
    pub msg: String,
    pub target: String,
    pub timestamp: i64,
    pub module: &'a str,
    pub file: &'a str,
    pub line: u32,
    pub date: String
}

impl<'a> From<&'a LogRecord<'a>> for ExtendedLogRecord<'a> {
    fn from(record: &'a LogRecord<'a>) -> ExtendedLogRecord<'a> {
        let now = time::now_utc();

        ExtendedLogRecord {
            date: format!("{}", now.rfc3339()),
            file: record.location().__file,
            level: record.level().to_string(),
            levelno: record.level() as u32,
            line: record.location().__line,
            module: record.location().__module_path,
            msg: format!("{}", record.args()),
            target: String::from(record.target()),
            timestamp: now.to_timespec().sec,
        }
    }
}

impl<'a> ExtendedLogRecord<'a> {
    pub fn level(&self) -> LogLevel {
        LogLevel::from_str(self.level.as_str()).unwrap()
    }
}