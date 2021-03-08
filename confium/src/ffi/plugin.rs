use crate::Confium;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn cfm_load_plugin(
    cfm: *mut Confium,
    c_path: *const c_char,
    //opts: *mut StringOptions,
) -> u32 {
    panic!();
}
