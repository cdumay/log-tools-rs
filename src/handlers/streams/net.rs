use formatter::default;
use handlers::streams::StreamHandler;
use log::LogLevelFilter;
use ExtendedLogRecord;
use std::net::TcpStream;

/// Type based on StreamHandler to handle the `TcpStream` stream.
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
/// let mut hdlr = TCPHandler::new(
///     "127.0.0.1:8080",
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
pub type TCPHandler = StreamHandler<TcpStream>;

impl TCPHandler {
    pub fn new(address: &'static str, level: Option<LogLevelFilter>, formatter: Option<fn(&ExtendedLogRecord) -> String>) -> TCPHandler {
        TCPHandler {
            filters: vec![],
            formatter: formatter.unwrap_or(default),
            level: level.unwrap_or(LogLevelFilter::Off),
            stream: TcpStream::connect(address).unwrap(),
        }
    }
}