use crate::error::{Error, ErrorCommon, Result};
use std::backtrace::Backtrace;
use std::os::raw::c_char;

fn cstring(cstr: *const c_char) -> Result<String> {
    if cstr.is_null() {
        return Err(Error::NullPointer {
            common: ErrorCommon {
                source: None,
                backtrace: Some(Backtrace::capture()),
            },
        });
    }
    let cstr = unsafe { std::ffi::CStr::from_ptr(cstr).to_str() };
    match cstr {
        Ok(s) => Ok(s.to_string()),
        Err(_) => {
            return Err(Error::InvalidUTF8 {
                common: ErrorCommon {
                    source: None,
                    backtrace: Some(Backtrace::capture()),
                },
            });
        }
    }
}
