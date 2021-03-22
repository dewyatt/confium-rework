#![feature(backtrace)]

#[macro_use]
pub extern crate slog;
extern crate libloading;
extern crate slog_async;
extern crate slog_stdlog;
extern crate slog_term;

#[macro_use]
pub mod utils;
pub mod error;
#[macro_use]
pub mod ffi;
pub mod hash;
pub mod options;

use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

use libloading::Library;
use slog::Drain;

use error::Error;

type StringOptions = HashMap<String, String>;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub type Result<T> = std::result::Result<T, Error>;

pub struct Confium {
    libraries: Vec<Rc<Library>>,
    logger: slog::Logger,
}

impl Confium {
    pub fn new_custom<L: Into<Option<slog::Logger>>>(logger: L) -> Self {
        Confium {
            logger: logger
                .into()
                .unwrap_or(slog::Logger::root(slog_stdlog::StdLog.fuse(), o!())),
            libraries: vec![],
        }
    }

    pub fn new() -> Self {
        let decorator = slog_term::TermDecorator::new().build();
        let drain = slog_term::FullFormat::new(decorator).build().fuse();
        let drain = slog_async::Async::new(drain).build().fuse();
        let log = slog::Logger::root(drain, o!("version" => VERSION));
        Confium::new_custom(log)
    }

    // TODO: Support Rust plugins
    pub fn load_plugin(&self, path: &Path, options: &StringOptions) -> Result<()> {
        panic!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //TODO
    //#[test]
    fn test_load_plugin() {
        let cfm = Confium::new();
        let mut opts = StringOptions::new();
        opts.insert("test".to_string(), "value".to_string());
        cfm.load_plugin(Path::new("/a/b/c"), &opts);
    }
}
