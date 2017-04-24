use formatters::default;
use handlers::streams::StreamHandler;
use log::LogLevelFilter;
use record::ExtendedLogRecord;
use std::io::{self, Stdout};

pub type StdoutHandler = StreamHandler<Stdout>;

impl StdoutHandler {
    pub fn new(level: Option<LogLevelFilter>, formatter: Option<fn(&ExtendedLogRecord) -> String>) -> StdoutHandler {
        StdoutHandler {
            filters: vec![],
            formatter: formatter.unwrap_or(default),
            stream: io::stdout(),
            level: level.unwrap_or(LogLevelFilter::Off)
        }
    }
}
