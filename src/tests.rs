use log::{LogLevelFilter, LogLevel};
use ExtendedLogger;
use handlers::Handler;
use formatter::{default, json, pretty_json};
use handlers::streams::stdout::StdoutHandler;
use handlers::streams::file::FileHandler;
use handlers::streams::net::TCPHandler;
use ExtendedLogRecord;
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
    ExtendedLogger::add_stdout_handler(Some(LogLevelFilter::Info), Some(json));
    ExtendedLogger::add_file_handler("/tmp/log-error.txt", Some(LogLevelFilter::Error), Some(json));

    info!("done");
    error!(":-(");
}

#[test]
fn test_stdout_json() {
    let rec = create_record("Test - StdoutHandler - json");

    let mut hdlr = StdoutHandler::new(
        Some(LogLevelFilter::Info),
        Some(json)
    );
    hdlr.handle(&rec);
}

#[test]
fn test_file_json() {
    let rec = create_record("Test - FileHandler - json");

    let mut hdlr = FileHandler::new(
        "/tmp/log.txt",
        Some(LogLevelFilter::Info),
        Some(json),
    );
    hdlr.handle(&rec);
}

#[test]
fn test_stdout_pretty_json() {
    let rec = create_record("Test - StdoutHandler - pretty_json");
    let mut hdlr = StdoutHandler::new(Some(LogLevelFilter::Info), Some(pretty_json));
    hdlr.handle(&rec);
}

fn custom_formatter(record: &ExtendedLogRecord) -> String {
    format!("{} - {} - {}\n", record.date, record.level, record.msg)
}

#[test]
fn test_stdout_custom() {
    let rec = create_record("Test - StdoutHandler - custom_formatter");
    let mut hdlr = StdoutHandler::new(Some(LogLevelFilter::Info), Some(custom_formatter));
    hdlr.handle(&rec);
}

#[test]
fn format_default() {
    let rec = create_record("test");
    println!("{}", default(&rec));
}

#[test]
fn format_json() {
    let rec = create_record("test");
    println!("{}", json(&rec));
}

#[test]
fn format_pretty_json() {
    let rec = create_record("test");
    println!("{}", pretty_json(&rec));
}