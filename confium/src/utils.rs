use crate::error::{Error, ErrorCommon, Result};
use std::backtrace::Backtrace;
use std::os::raw::c_char;

use crate::error::ErrorCode;

#[macro_escape]
macro_rules! err_common {
    ($($source:tt)+) => {{
        ErrorCommon {
            source: $($source)+,
            backtrace: Some(Backtrace::capture()),
        }
    }};
}

#[macro_escape]
macro_rules! check_not_null {
    ($param:ident) => {{
        if $param.is_null() {
            use std::backtrace::Backtrace;
            return Err(Error::NullPointer {
                common: err_common!(None),
                param: stringify!($param),
            });
        }
    }};
}
