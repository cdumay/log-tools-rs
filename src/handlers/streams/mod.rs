pub mod file;
pub mod stdout;
pub mod net;

use filters::Filter;
use handlers::Handle;
use log::LogLevelFilter;
use record::ExtendedLogRecord;
use std::io::Write;

/// Base handler for streams
///
/// # Warning
///
/// stream must implement the std::io::Write
///
pub struct StreamHandler<W: Write> {
    /// Vector of filter callback.
    pub filters: Vec<fn(&ExtendedLogRecord) -> bool>,
    /// Callback to format log record.
    pub formatter: fn(&ExtendedLogRecord) -> String,
    /// The current maximum log level of the handler.
    pub level: LogLevelFilter,
    /// The managed stream.
    pub stream: W,
}

impl<W> Filter for StreamHandler<W> where W: Write {
    /// Apply all stored filters to emit or not the log record
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
    /// Check if the log record may be emitted. `self.level` and filters will be checked.
    fn handle(&mut self, record: &ExtendedLogRecord) {
        if self.level >= record.level() && self.filter(record) == true {
            self.emit(record)
        }
    }
    /// Format the record into the stream using the formatter.
    fn emit(&mut self, record: &ExtendedLogRecord) {
        self.stream.write_all((self.formatter)(record).as_bytes()).unwrap();
    }
}