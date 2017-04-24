use formatter::default;
use handlers::streams::StreamHandler;
use log::LogLevelFilter;
use ExtendedLogRecord;
use std::io::{self, Stdout};

/// Type based on StreamHandler to handle the `Stdout` stream.
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
/// let mut hdlr = StdoutHandler::new(
///     Some(LogLevelFilter::Info),
///     Some(json),
/// );
///
/// hdlr.handle(&rec);
/// ```
///
/// It will format the log record as JSON and print it into stdout:
///
/// ```json
///{"level":"INFO","levelno":3,"msg":"Test","target":"MyFactory","timestamp":1493042710,"module":"log_handlers::tests","file":"src/tests.rs","line":24,"date":"2017-04-24T14:05:10Z"}
/// ```
pub type StdoutHandler = StreamHandler<Stdout>;

impl StdoutHandler {
    pub fn new(level: Option<LogLevelFilter>, formatter: Option<fn(&ExtendedLogRecord) -> String>) -> StdoutHandler {
        StdoutHandler {
            filters: vec![],
            formatter: formatter.unwrap_or(default),
            stream: io::stdout(),
            level: level.unwrap_or(LogLevelFilter::Off)
        }
    }
}
