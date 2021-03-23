use std::ffi::CString;
use std::os::raw::c_char;

use crate::error::Error;
use crate::options::Options;
use crate::Confium;

#[no_mangle]
pub extern "C" fn cfm_create(cfm: *mut *mut Confium) -> u32 {
    unsafe { *cfm = Box::into_raw(Box::new(Confium::new())) }
    0
}

#[no_mangle]
pub extern "C" fn cfm_destroy(cfm: *mut Confium) -> u32 {
    unsafe {
        Box::from_raw(cfm);
    }
    0
}

#[no_mangle]
pub extern "C" fn cfm_version_string(version: *mut *mut c_char, errptr: *mut *mut Error) -> u32 {
    let vers = CString::new(crate::VERSION).unwrap();
    unsafe {
        *version = vers.into_raw();
    }
    0
}
