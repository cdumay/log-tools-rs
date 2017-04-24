use log::{LogLevelFilter, LogLevel};
use logger::ExtendedLogger;
use handlers::Handler;
use formatters::{json, pretty_json};
use handlers::streams::stdout::StdoutHandler;
use handlers::streams::file::FileHandler;
use handlers::streams::net::TCPHandler;
use record::ExtendedLogRecord;
use handlers::Handle;

fn create_record(msg: &'static str) -> ExtendedLogRecord {
    ExtendedLogRecord::new(
        file!(),
        LogLevel::Info,
        line!(),
        module_path!(),
        msg.to_string(),
        "TestFactory".to_string()
    )
}

#[test]
fn test_stdout_logger() {
    ExtendedLogger::init(LogLevelFilter::Info).unwrap();
    ExtendedLogger::add_handler(Handler::from(StdoutHandler::new(Some(LogLevelFilter::Info), Some(json))));
    ExtendedLogger::add_handler(Handler::from(FileHandler::new("/tmp/log-error.txt", Some(LogLevelFilter::Error), Some(json))));

    info!("done");
    error!(":-(");
}

#[test]
fn test_stdout_json() {
    let rec = create_record("Test - StdoutHandler");

    let mut hdlr = StdoutHandler::new(
        Some(LogLevelFilter::Info),
        Some(json)
    );
    hdlr.handle(&rec);
}

#[test]
fn test_file_json() {
    let rec = create_record("Test - FileHandler");

    let mut hdlr = FileHandler::new(
        "/tmp/log.txt",
        Some(LogLevelFilter::Info),
        Some(json),
    );
    hdlr.handle(&rec);
}

#[test]
fn test_stdout_pretty_json() {
    ExtendedLogger::init(LogLevelFilter::Info).unwrap();
    ExtendedLogger::add_handler(
        Handler::from(StdoutHandler::new(Some(LogLevelFilter::Info), Some(pretty_json)))
    );

    info!("done");
}
