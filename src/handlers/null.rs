use filters::Filter;
use handlers::Handle;
use record::ExtendedLogRecord;

pub struct NullHandler;


impl Filter for NullHandler {
    fn filter(&self, record: &ExtendedLogRecord) -> bool { true }
}

impl Handle for NullHandler {
    fn handle(&mut self, record: &ExtendedLogRecord) {}
    fn emit(&mut self, record: &ExtendedLogRecord) {}
}