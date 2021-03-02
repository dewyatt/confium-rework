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
        let log = slog::Logger::root(drain, o!());
        Confium::new_custom(log)
    }
}

#[no_mangle]
pub extern "C" fn do_test(input: *const c_char, err: *mut *mut FFIError) -> u32 {
    ffi_check_not_null!(input, err);
    match cstring(input) {
        Ok(s) => {
            println!("input: {}", s);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    0
}
/*
fn parsehex(s: &str) -> Result<u8> {
    if s.is_empty() {
        return Err(Error::InvalidFormat {
            common: ErrorCommon {
                source: None,
                backtrace: Some(Backtrace::capture()),
            },
        });
    }
    let mut result: u8 = 0;
    for (idx, ch) in s.trim_start_matches("0x").char_indices() {
        let x = match ch.to_digit(16) {
            Some(x) => x,
            None => {
                return Err(Error::InvalidHexDigit {
                    common: ErrorCommon {
                        source: None,
                        backtrace: Some(Backtrace::capture()),
                    },
                    ch: ch,
                });
            }
        } as u8;
        result = match result.checked_mul(16) {
            Some(result) => result,
            None => {
                return Err(Error::Overflow {
                    common: ErrorCommon {
                        source: None,
                        backtrace: Some(Backtrace::capture()),
                    },
                });
            }
        };
        result = match result.checked_add(x) {
            Some(result) => result,
            None => {
                return Err(Error::Overflow {
                    common: ErrorCommon {
                        source: None,
                        backtrace: Some(Backtrace::capture()),
                    },
                });
            }
        };
    }
    Ok(result)
}
*/
