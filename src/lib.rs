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
//! To create a basic logger to print log records to stdout:
//!
//! ```rust
//! use log::LogLevelFilter;
//! use log_handlers::handlers::Handler;
//! use log_handlers::handlers::streams::stdout::StdoutHandler;
//! use log_handlers::logger::ExtendedLogger;
//!
//! fn main() {
//!     ExtendedLogger::init(LogLevelFilter::Info).unwrap();
//!     ExtendedLogger::add_handler(Handler::from(StdoutHandler::new(Some(LogLevelFilter::Info), None)));
//!
//!     info!("done");
//! }
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