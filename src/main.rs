#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
extern crate rustc_serialize;
extern crate time;

mod handlers;
mod record;
mod formatters;
mod logger;
mod filters;

use log::LogLevelFilter;
use logger::ExtendedLogger;
use handlers::Handler;
use formatters::json;
use handlers::streams::stdout::StdoutHandler;
use handlers::streams::file::FileHandler;
use handlers::streams::net::TCPHandler;
use record::ExtendedLogRecord;

fn custom_formatter(record: &ExtendedLogRecord) -> String {
    format!("{} - {} - {}\n", record.date, record.level, record.msg)
}

fn main() {
    ExtendedLogger::init(LogLevelFilter::Info).unwrap();
    //    ExtendedLogger::add_handler(Handler::from(StdoutHandler::new(Some(LogLevelFilter::Info), Some(json))));
        ExtendedLogger::add_handler(Handler::from(FileHandler::new("/tmp/log-error.txt", Some(LogLevelFilter::Error), Some(json))));
    //    ExtendedLogger::add_handler(Handler::from(StdoutHandler::new(Some(LogLevelFilter::Debug), Some(custom_formatter))));
//    ExtendedLogger::add_handler(Handler::from(TCPHandler::new("127.0.0.1:8080", Some(LogLevelFilter::Info), Some(json))));

//    for i in 1..100000 {
//        error!("cpt - {}", i);
//    }
    error!("test")
}

