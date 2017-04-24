use record::ExtendedLogRecord;

/// A trait encapsulating the filtering operation of handlers
pub trait Filter {
    /// determines if a log message would be logged by the handler.
    fn filter(&self, record: &ExtendedLogRecord) -> bool;
}
