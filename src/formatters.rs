use rustc_serialize::json::{self, as_pretty_json};
use record::ExtendedLogRecord;

/// Default formatter which uses Debug formatter to format log record.
pub fn default(record: &ExtendedLogRecord) -> String {
    format!("{:?}\n", record)
}

/// Format log record into JSON using `RustcEncodable`.
pub fn json(record: &ExtendedLogRecord) -> String {
    format!("{}\n", json::encode(&record).unwrap())
}

// Pretty print of the log record into JSON using `RustcEncodable`.
pub fn pretty_json(record: &ExtendedLogRecord) -> String {
    format!("{}\n", as_pretty_json(&record))
}