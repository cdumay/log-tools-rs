use record::ExtendedLogRecord;

pub trait Filter {
    fn filter(&self, &ExtendedLogRecord) -> bool;
}
