//! Extend the rust library log.
//!
//! Provides support for various handlers to send the log records to the appropriate destination.
//!
//! # Dependencies
//!
//! * `log` - base logging library.
//! * `time` - simple time handling to extent log records.
//! * `lazy-static` - a macro for declaring lazily evaluated statics. This is used to manage
//! handlers.
//! * `rustc-serialize` - adds the ability to serialize and deserialize a `Uuid`
//!   using the `rustc-serialize` crate.
//!
//! By default, `log-tools` can be depended on with:
//!
//! ```toml
//! [dependencies]
//! log-tools = "0.0.1"
//! ```
//!
//! # Example
//!
//! Let's create a logger which will format log records into JSON and print Info in stdout and
//! Errors into a file.
//!
//! ```rust
//! use log::LogLevelFilter;
//! use log_handlers::handlers::Handler;
//! use log_handlers::handlers::streams::stdout::StdoutHandler;
//! use log_handlers::handlers::streams::file::FileHandler;
//!
//! fn main() {
//!    ExtendedLogger::init(LogLevelFilter::Info).unwrap();
//!    ExtendedLogger::add_handler(Handler::from(StdoutHandler::new(Some(LogLevelFilter::Info), Some(json))));
//!    ExtendedLogger::add_handler(Handler::from(FileHandler::new("/tmp/log-error.txt", Some(LogLevelFilter::Error), Some(json))));
//!
//!     info!("done");
//!     error!(":-(");
//! }
//! ```
//!
//! It will print into stdout:
//!
//! ```
//! {"level":"INFO","levelno":3,"msg":"done","target":"log_handlers::tests","timestamp":1493044134,"module":"log_handlers::tests","file":"src/tests.rs","line":28,"date":"2017-04-24T14:28:54Z"}
//! {"level":"ERROR","levelno":1,"msg":":-(","target":"log_handlers::tests","timestamp":1493044134,"module":"log_handlers::tests","file":"src/tests.rs","line":29,"date":"2017-04-24T14:28:54Z"}
//! ```
//!
//! And in the file `/tmp/log-error.txt`:
//!
//! ```
//! {"level":"ERROR","levelno":1,"msg":":-(","target":"log_handlers::tests","timestamp":1493044134,"module":"log_handlers::tests","file":"src/tests.rs","line":29,"date":"2017-04-24T14:28:54Z"}
//! ```
//!
//! # References
//!
//! * [Log library](https://doc.rust-lang.org/log/log/index.html)
//!

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate rustc_serialize;
extern crate time;

mod handlers;
mod record;
mod formatters;
mod logger;
mod filters;

#[cfg(test)]
mod tests;