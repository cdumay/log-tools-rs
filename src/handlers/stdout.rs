use formatters::default;
use record::ExtendedLogRecord;
use std::io::{self, Stdout, Write};

pub type StdoutHandler = StreamHandler<Stdout>;

impl StdoutHandler {
    pub fn new(formatter: Option<fn(&ExtendedLogRecord) -> String>) -> StdoutHandler {
        StdoutHandler {
            filters: vec![],
            formatter: formatter.unwrap_or(default),
            stream: io::stdout()
        }
    }
    pub fn filter(&self, record: &ExtendedLogRecord) -> bool {
        for filter in self.filters.as_slice() {
            if filter(record) == false {
                return false
            }
        }
        return true
    }
    pub fn add_filter(&mut self, filter: fn(&ExtendedLogRecord) -> bool) {
        self.filters.push(filter);
    }
}
