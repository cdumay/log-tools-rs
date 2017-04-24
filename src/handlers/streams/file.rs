use formatter::default;
use handlers::streams::StreamHandler;
use log::LogLevelFilter;
use ExtendedLogRecord;
use std::fs::{File, OpenOptions};

/// Type based on StreamHandler to handle a `File` stream.
///
/// # Examples
///
/// Manual test the handler:
///
/// ```rust
///
/// let rec = ExtendedLogRecord::new(
///     file!(),
///     LogLevel::Info,
///     line!(),
///     module_path!(),
///     "Test".to_string(),
///     "MyFactory".to_string()
/// );
/// let mut hdlr = FileHandler::new(
///     "/tmp/log.txt",
///     Some(LogLevelFilter::Info),
///     Some(json),
/// );
///
/// hdlr.handle(&rec);
/// ```
///
/// It will format the log record as JSON and store it into `/tmp/log.txt` like:
///
/// ```json
///{"level":"INFO","levelno":3,"msg":"Test","target":"MyFactory","timestamp":1493042710,"module":"log_handlers::tests","file":"src/tests.rs","line":24,"date":"2017-04-24T14:05:10Z"}
/// ```
pub type FileHandler = StreamHandler<File>;

impl FileHandler {
    /// Create a new handler instance and initialize the file stream.
    pub fn new(filename: &'static str, level: Option<LogLevelFilter>, formatter: Option<fn(&ExtendedLogRecord) -> String>) -> FileHandler {
        FileHandler {
            filters: vec![],
            formatter: formatter.unwrap_or(default),
            stream: OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(filename).unwrap(),
            level: level.unwrap_or(LogLevelFilter::Off)
        }
    }
}