use std::os::raw::c_void;

type Confium = c_void;
type Options = c_void;

#[no_mangle]
pub extern "C" fn cfmp_initialize(
    cfm: *mut Confium,
    opts: *const Options,
    info: *mut Options,
) -> u32 {
    println!("plugin");
    0
}
