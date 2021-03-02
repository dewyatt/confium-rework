use crate::error::Error;
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

#[no_mangle]
pub extern "C" fn cfm_err_get_msg(err: *const FFIError, msg: *mut *mut c_char) -> u32 {
    unsafe {
        *msg = std::ptr::null_mut();
        let errmsg = format!("{}", *err);
        match CString::new(errmsg) {
            Ok(s) => *msg = s.into_raw(),
            Err(e) => {
                eprintln!("{:?}", e);
                panic!("fail");
            }
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
    unsafe {
        *backtrace = match (*err).backtrace {
            None => std::ptr::null_mut(),
            Some(ref bt) => match CString::new(&**bt) {
                Ok(s) => s.into_raw(),
                Err(e) => {
                    // TODO
                    panic!("fail");
                }
            },
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
