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
