//!
//! Module which provide handlers to send the log records to the appropriate destination.
//!
pub mod streams;

use handlers::streams::file::FileHandler;
use handlers::streams::net::TCPHandler;
use handlers::streams::stdout::StdoutHandler;
use log::LogLevelFilter;
use ExtendedLogRecord;
use std::sync::Mutex;

/// A trait encapsulating the filtering operation of the handler.
pub trait Filter {
    /// determines if a log message would be logged by the handler.
    fn filter(&self, record: &ExtendedLogRecord) -> bool;
}


lazy_static! {
    /// We define handlers as static to be executed at runtime.
    pub static ref HANDLERS: Mutex<Vec<Handler>> = Mutex::new(vec![]);
}

/// A trait encapsulating the operations required of a handler
pub trait Handle {
    /// Determines if a log record may be handled by the handler.
    fn handle(&mut self, record: &ExtendedLogRecord);
    /// Emit the log record.
    fn emit(&mut self, record: &ExtendedLogRecord);
}

/// Available handlers
pub enum Handler {
    /// A dummy handler use to do nothing.
    Null(NullHandler),
    /// A handler to send the log record into stdout.
    Stdout(StdoutHandler),
    /// A handler to send the log record into a file.
    File(FileHandler),
    /// A handler to send the log record into a TCP socket.
    TCP(TCPHandler)
}

impl Handler {
    pub fn handle(&mut self, record: &ExtendedLogRecord) {
        match *self {
            Handler::Null(ref mut hdlr) => hdlr.handle(record),
            Handler::Stdout(ref mut hdlr) => hdlr.handle(record),
            Handler::File(ref mut hdlr) => hdlr.handle(record),
            Handler::TCP(ref mut hdlr) => hdlr.handle(record),
        };
    }
}

impl From<StdoutHandler> for Handler {
    fn from(hdlr: StdoutHandler) -> Handler {
        Handler::Stdout(hdlr)
    }
}

impl From<NullHandler> for Handler {
    fn from(hdlr: NullHandler) -> Handler {
        Handler::Null(hdlr)
    }
}

impl From<FileHandler> for Handler {
    fn from(hdlr: FileHandler) -> Handler {
        Handler::File(hdlr)
    }
}

impl From<TCPHandler> for Handler {
    fn from(hdlr: TCPHandler) -> Handler {
        Handler::TCP(hdlr)
    }
}

///
/// A dummy handler which does nothing
///
pub struct NullHandler;

impl Filter for NullHandler {
    /// Always accept the record
    fn filter(&self, record: &ExtendedLogRecord) -> bool { true }
}

impl Handle for NullHandler {
    fn handle(&mut self, record: &ExtendedLogRecord) {}
    fn emit(&mut self, record: &ExtendedLogRecord) {}
}