pub mod file;
pub mod stdout;
pub mod net;

use filters::Filter;
use handlers::Handle;
use log::LogLevelFilter;
use record::ExtendedLogRecord;
use std::io::Write;

pub struct StreamHandler<W: Write> {
    pub filters: Vec<fn(&ExtendedLogRecord) -> bool>,
    pub formatter: fn(&ExtendedLogRecord) -> String,
    pub level: LogLevelFilter,
    pub stream: W,
}

impl<W> Filter for StreamHandler<W> where W: Write {
    fn filter(&self, record: &ExtendedLogRecord) -> bool {
        for filter in self.filters.as_slice() {
            if filter(record) == false {
                return false
            }
        }
        return true
    }
}

impl<W> Handle for StreamHandler<W> where W: Write {
    fn handle(&mut self, record: &ExtendedLogRecord) {
        if self.level >= record.level() {
            if self.filter(record) == true {
                self.emit(record)
            }
        }
    }
    fn emit(&mut self, record: &ExtendedLogRecord) {
        self.stream.write_all((self.formatter)(record).as_bytes()).unwrap();
    }
}