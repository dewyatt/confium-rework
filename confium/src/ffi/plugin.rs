use std::os::raw::c_char;
use std::rc::Rc;

use libloading::Library;

use crate::error::Error;
use crate::error::*;
use crate::ffi::utils::cstring;
use crate::options::Options;
use crate::Confium;
use snafu::ResultExt;

#[no_mangle]
pub extern "C" fn cfm_plugin_load(
    cfm: *mut Confium,
    c_path: *const c_char,
    opts: *mut Options,
    err: *mut *mut Error,
) -> u32 {
    // plugin query features using options list?
    // cfmp_query_features()
    ffi_check_not_null!(cfm, err);
    ffi_check_not_null!(c_path, err);
    let path = match cstring(c_path) {
        Ok(s) => s,
        Err(e) => ffi_return_err!(e, err),
    };
    let path = std::path::PathBuf::from(path);
    let lib = Rc::new(Library::new(&path).context(PluginLoadFailed { plugin: path }));
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn cfm_plugin_unload(cfm: *mut Confium, c_name: *const c_char) -> u32 {
    unimplemented!();
}
