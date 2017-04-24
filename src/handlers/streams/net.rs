use formatters::default;
use handlers::streams::StreamHandler;
use log::LogLevelFilter;
use record::ExtendedLogRecord;
use std::net::TcpStream;

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