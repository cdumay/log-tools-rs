pub mod null;
pub mod streams;

use handlers::null::NullHandler;
use handlers::streams::file::FileHandler;
use handlers::streams::net::TCPHandler;
use handlers::streams::stdout::StdoutHandler;
use record::ExtendedLogRecord;
use std::sync::Mutex;

lazy_static! {
    pub static ref HANDLERS: Mutex<Vec<Handler>> = Mutex::new(vec![]);
}

pub trait Handle {
    fn handle(&mut self, record: &ExtendedLogRecord);
    fn emit(&mut self, record: &ExtendedLogRecord);
}

pub enum Handler {
    Null(NullHandler),
    Stdout(StdoutHandler),
    File(FileHandler),
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