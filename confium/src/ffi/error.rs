use crate::error::{Error, ErrorCode};
use std::ffi::CString;
use std::os::raw::c_char;

#[derive(Clone)]
pub struct FFIError {
    pub message: String,
    pub code: u32,
    pub source: Option<Box<FFIError>>,
    pub backtrace: Option<String>,
}

impl std::fmt::Display for FFIError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

fn to_ffierror(err: &Error) -> FFIError {
    FFIError {
        message: err.to_string(),
        code: err.code(),
        source: match err.source() {
            None => None,
            Some(src) => Some(Box::new(to_ffierror(src))),
        },
        backtrace: err.backtrace().as_ref().map(|bt| bt.to_string()),
    }
}

impl From<&Error> for FFIError {
    #[inline]
    fn from(err: &Error) -> FFIError {
        to_ffierror(err)
    }
}

macro_rules! err_check_not_null {
    ($param:ident) => {{
        if $param.is_null() {
            let err = $crate::error::Error::NullPointer {
                common: err_common!(None),
                param: stringify!($param),
            };
            eprintln!("Error: {:?}", err);
            return err.code().into();
        }
    }};
}
#[no_mangle]
pub extern "C" fn cfm_err_get_msg(err: *const FFIError, msg: *mut *mut c_char) -> u32 {
    err_check_not_null!(err);
    err_check_not_null!(msg);
    let errmsg;
    unsafe {
        *msg = std::ptr::null_mut();
        errmsg = format!("{}", *err);
    }
    match CString::new(errmsg) {
        Ok(s) => unsafe { *msg = s.into_raw() },
        Err(e) => {
            eprintln!("Error: {:?}", e);
            return ErrorCode::UNKNOWN as u32;
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn cfm_err_get_code(err: *const FFIError, code: *mut u32) -> u32 {
    unsafe {
        *code = (*err).code;
    }
    0
}

#[no_mangle]
pub extern "C" fn cfm_err_get_source(err: *const FFIError, src: *mut *mut FFIError) -> u32 {
    unsafe {
        *src = match (*err).source {
            None => std::ptr::null_mut(),
            Some(ref source) => Box::into_raw(Box::new((**source).clone())),
        }
    }
    0
}

#[no_mangle]
pub extern "C" fn cfm_err_get_backtrace(err: *mut FFIError, backtrace: *mut *const c_char) -> u32 {
    err_check_not_null!(err);
    err_check_not_null!(backtrace);
    unsafe { *backtrace = std::ptr::null_mut() }
    if let Some(ref bt) = unsafe { (*err).backtrace.as_ref() } {
        match CString::new(bt.to_string()) {
            Ok(s) => unsafe { *backtrace = s.into_raw() },
            Err(e) => {
                eprintln!("Error: {}", e);
                return ErrorCode::UNKNOWN as u32;
            }
        };
    }
    0
}

#[no_mangle]
pub extern "C" fn cfm_err_destroy(err: *mut FFIError) {
    unsafe {
        Box::from_raw(err);
    }
}
