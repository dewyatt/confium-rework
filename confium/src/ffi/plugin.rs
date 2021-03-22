use std::os::raw::c_char;
use std::rc::Rc;

use libloading::Library;

use crate::error::Error;
use crate::error::*;
use crate::ffi::utils::cstring;
use crate::options::Options;
use crate::Confium;
use snafu::ResultExt;

type InitializeFn = extern "C" fn(*mut Confium, *mut Options) -> u32;

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
    let cfm = unsafe { &mut *cfm };
    let path = match cstring(c_path) {
        Ok(s) => s,
        Err(e) => ffi_return_err!(e, err),
    };
    let path = std::path::PathBuf::from(path);
    let lib = Library::new(&path).context(PluginLoadFailed { plugin: &path });
    if let Err(e) = lib {
        ffi_return_err!(e, err);
    }
    let lib = lib.unwrap();
    // TODO: initialize
    let initialize = match unsafe { lib.get::<InitializeFn>(b"cfmp_initialize\0") }
        .context(PluginLoadFailed { plugin: &path })
    {
        Ok(fun) => fun,
        Err(e) => {
            error!(
                cfm.logger,
                "Plugin '{}' missing initialization function",
                &path.to_string_lossy(),
            );
            ffi_return_err!(e, err);
        }
    };
    let code = initialize(cfm, opts);
    if code != 0 {
        error!(
            cfm.logger,
            "Initialization of plugin '{}' returned code {}",
            &path.to_string_lossy(),
            code
        );
        // TODO: log
        return code;
    }
    cfm.libraries.push(Rc::new(lib));
    // TODO: query plugin features
    0
}

#[no_mangle]
pub extern "C" fn cfm_plugin_unload(cfm: *mut Confium, c_name: *const c_char) -> u32 {
    unimplemented!();
}
