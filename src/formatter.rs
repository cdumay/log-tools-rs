//! Module which provides basics formatter to serialize log record.
//!
//! A formatter is a callback used by handlers of type:
//!
//! ```rust
//! fn(&ExtendedLogRecord) -> bool
//! ```

use rustc_serialize::json::{self, as_pretty_json};
use ExtendedLogRecord;

///
/// Default formatter which uses Debug formatter to format log record.
///
/// # Example
///
/// ```rust
/// let rec = ExtendedLogRecord::new(
///     file!(),
///     LogLevel::Info,
///     line!(),
///     module_path!(),
///     "test".to_string(),
///     "TestFactory".to_string()
/// );
/// println!("{}", default(&rec));
/// ```
/// # Result
/// ```
/// ExtendedLogRecord { level: "INFO", levelno: 3, msg: "test", target: "TestFactory", timestamp: 1493048120, module: "log_handlers::tests", file: "src/tests.rs", line: 15, date: "2017-04-24T15:35:20Z" }
/// ```
pub fn default(record: &ExtendedLogRecord) -> String {
    format!("{:?}\n", record)
}

///
/// Format log record into JSON using `RustcEncodable`.
///
/// # Example
///
/// ```rust
/// let rec = ExtendedLogRecord::new(
///     file!(),
///     LogLevel::Info,
///     line!(),
///     module_path!(),
///     "test".to_string(),
///     "TestFactory".to_string()
/// );
/// println!("{}", json(&rec));
/// ```
/// # Result
/// ```json
/// {"level":"INFO","levelno":3,"msg":"test","target":"TestFactory","timestamp":1493048347,"module":"log_handlers::tests","file":"src/tests.rs","line":15,"date":"2017-04-24T15:39:07Z"}
/// ```
pub fn json(record: &ExtendedLogRecord) -> String {
    format!("{}\n", json::encode(&record).unwrap())
}

///
/// Pretty print of the log record into JSON using `RustcEncodable`.
///
/// # Example
///
/// ```rust
/// let rec = ExtendedLogRecord::new(
///     file!(),
///     LogLevel::Info,
///     line!(),
///     module_path!(),
///     "test".to_string(),
///     "TestFactory".to_string()
/// );
/// println!("{}", pretty_json(&rec));
/// ```
/// # Result
/// ```json
/// {
///     "date": "2017-04-24T15:45:29Z",
///     "file": "src/tests.rs",
///     "level": "INFO",
///     "levelno": 3,
///     "line": 15,
///     "module": "log_handlers::tests",
///     "msg": "test",
///     "target": "TestFactory",
///     "timestamp": 1493048729
/// }
/// ```
pub fn pretty_json(record: &ExtendedLogRecord) -> String {
    format!("{}\n", as_pretty_json(&record))
}