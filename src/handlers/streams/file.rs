use formatters::default;
use record::ExtendedLogRecord;
use std::fs::{File, OpenOptions};
use handlers::streams::StreamHandler;
use log::LogLevelFilter;

pub type FileHandler = StreamHandler<File>;

impl FileHandler {
    pub fn new(filename: &'static str, level: Option<LogLevelFilter>, formatter: Option<fn(&ExtendedLogRecord) -> String>) -> FileHandler {
        FileHandler {
            filters: vec![],
            formatter: formatter.unwrap_or(default),
            stream: OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(filename).unwrap(),
            level: level.unwrap_or(LogLevelFilter::Off)
        }
    }
}