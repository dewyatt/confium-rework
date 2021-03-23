use std::os::raw::c_char;
use std::rc::Rc;

use libloading::Library;

use crate::error::Error;
use crate::error::*;
use crate::ffi::utils::cstring;
use crate::options::Options;
use crate::Confium;
use crate::Plugin;
use crate::Result;
use snafu::ResultExt;

// Confium, Options, PluginInfo
type InitializeFn = extern "C" fn(*mut Confium, *const Options, *mut Options) -> u32;

#[no_mangle]
pub extern "C" fn cfm_plugin_load(
    cfm: *mut Confium,
    c_path: *const c_char,
    opts: *mut Options,
    errptr: *mut *mut Error,
) -> u32 {
    // plugin query features using options list?
    // cfmp_query_features()
    ffi_check_not_null!(cfm, errptr);
    ffi_check_not_null!(c_path, errptr);
    let cfm = unsafe { &mut *cfm };
    let path = match cstring(c_path) {
        Ok(s) => s,
        Err(e) => ffi_return_err!(e, errptr),
    };
    let path = std::path::PathBuf::from(path);
    let lib = unsafe { Library::new(&path).context(PluginLoadFailed { plugin: &path }) };
    if let Err(e) = lib {
        ffi_return_err!(e, errptr);
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
            ffi_return_err!(e, errptr);
        }
    };
    let mut plugin_info: Options = Options::new();
    println!("Loading plugin...");
    let code = initialize(cfm, opts, &mut plugin_info);
    if code != 0 {
        error!(
            cfm.logger,
            "Initialization of plugin '{}' returned code {}",
            &path.to_string_lossy(),
            code
        );
        return code;
    }
    if plugin_info.is_empty() {
        error!(
            cfm.logger,
            "Plugin '{}' failed to initialize (empty plugin info)",
            &path.to_string_lossy()
        );
        //PluginLoadFailed { plugin: &path }.build();
        let e = PluginInitializationFailed {
            plugin: &path,
            reason: "empty plugin info",
        }
        .build();
        ffi_return_err!(e, errptr);
    }
    cfm.libraries.push(Plugin {
        library: Rc::new(lib),
        info: plugin_info,
    });
    0
}

#[no_mangle]
pub extern "C" fn cfm_plugin_unload(cfm: *mut Confium, c_name: *const c_char) -> u32 {
    unimplemented!();
}
