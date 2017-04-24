use formatters::default;
use record::ExtendedLogRecord;
use std::fs::{File, OpenOptions};
use handlers::streams::StreamHandler;

pub type FileHandler = StreamHandler<File>;

impl FileHandler {
    pub fn new(formatter: Option<fn(&ExtendedLogRecord) -> String>, filename: &'static str) -> FileHandler {
        FileHandler {
            filters: vec![],
            formatter: formatter.unwrap_or(default),
            stream: OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(filename).unwrap()
        }
    }
}