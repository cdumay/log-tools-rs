use rustc_serialize::json::{self, as_pretty_json};
use record::ExtendedLogRecord;

pub fn default(record: &ExtendedLogRecord) -> String {
    format!("{:?}\n", record)
}

pub fn json(record: &ExtendedLogRecord) -> String {
    format!("{}\n", json::encode(&record).unwrap())
}

pub fn pretty_json(record: &ExtendedLogRecord) -> String {
    format!("{}\n", as_pretty_json(&record))
}