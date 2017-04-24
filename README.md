log-tools-rs
============

[![Build Status](https://travis-ci.org/cdumay/log-tools-rs.svg?branch=master)](https://travis-ci.org/cdumay/log-tools-rs)

Still unstable do not use yet!

Extend the rust library log. It provides support for various handlers to send the log records to the appropriate 
destination.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
log-tools = "0.0.1"
```

and this to your crate root:

```rust
extern crate log_tools;
```

## Example

To print log record using stdout with the default formatter:

```rust
extern crate log_tools;
extern crate log;

use log::LogLevelFilter;
use log_handlers::handlers::Handler;
use log_handlers::handlers::streams::stdout::StdoutHandler;
use log_handlers::logger::ExtendedLogger;

fn main() {
    ExtendedLogger::init(LogLevelFilter::Info).unwrap();
    ExtendedLogger::add_handler(
        Handler::from(StdoutHandler::new(Some(LogLevelFilter::Info), None))
    );
    
    info!("done");
}
```