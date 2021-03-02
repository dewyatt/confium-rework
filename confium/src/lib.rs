#![feature(backtrace)]
extern crate libloading;

#[macro_use]
pub extern crate slog;
extern crate slog_async;
extern crate slog_stdlog;
extern crate slog_term;

pub mod error;
#[macro_use]
pub mod utils;
#[macro_use]
pub mod ffi;

use slog::Drain;
use std::os::raw::c_char;

use error::Error;
use ffi::error::FFIError;
use ffi::utils::cstring;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub type Result<T> = std::result::Result<T, Error>;

pub struct Confium {
    logger: slog::Logger,
}

impl Confium {
    pub fn new_custom<L: Into<Option<slog::Logger>>>(logger: L) -> Self {
        Confium {
            logger: logger
                .into()
                .unwrap_or(slog::Logger::root(slog_stdlog::StdLog.fuse(), o!())),
        }
    }

    pub fn new() -> Self {
        let decorator = slog_term::TermDecorator::new().build();
        let drain = slog_term::FullFormat::new(decorator).build().fuse();
        let drain = slog_async::Async::new(drain).build().fuse();
        let log = slog::Logger::root(drain, o!("version" => VERSION));
        Confium::new_custom(log)
    }
}
