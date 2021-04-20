use std::os::raw::c_char;
use std::rc::Rc;

use libloading::{Library, Symbol};

use crate::error::Error;
use crate::error::*;
use crate::ffi::utils::cstring;
use crate::options::Options;
use crate::Confium;
use crate::Plugin;
use crate::Result;
use snafu::ResultExt;

type InterfaceVersionFn = extern "C" fn(*mut Confium) -> u32;
const INTERFACE_VERSION_FN_NAME: &'static [u8] = b"cfmp_interface_version\0";

type InitializeFnV0 =
    extern "C" fn(*mut Confium, opts: *const Options, capabilities: *mut Options) -> u32;
const INITIALIZE_FN_V0_NAME: &'static [u8] = b"cfmp_initialize\0";

type FinalizeFnV0 = extern "C" fn(*mut Confium) -> u32;
const FINALIZE_FN_V0_NAME: &'static [u8] = b"cfmp_finalize\0";

type QueryInterfacesFnV0 = extern "C" fn(*mut Confium, *mut Options);
const QUERY_INTERFACES_FN_V0_NAME: &'static [u8] = b"cfmp_query_interfaces\0";

pub struct PluginV0 {
    initialize: Box<InitializeFnV0>,
    query_interfaces: Box<QueryInterfacesFnV0>,
}

pub enum PluginVTable {
    V0(PluginV0),
}

#[macro_escape]
macro_rules! check_not_null {
    ($param:ident, $errptr:ident) => {{
        if $param.is_null() {
            return Err($crate::error::NullPointer {
                param: stringify!($param),
            }
            .build());
        }
    }};
}

fn load_plugin_v0(
    cfm: &mut Confium,
    path: &std::path::PathBuf,
    lib: Library,
    opts: &mut Options,
    errptr: *mut *mut Error,
) -> Result<Plugin> {
    let initialize: Symbol<InitializeFnV0> =
        unsafe { lib.get::<InitializeFnV0>(INITIALIZE_FN_V0_NAME) }
            .context(PluginLoadFailed { plugin: &path })?;
    let initialize = Box::new(*initialize);
    let query_interfaces: Symbol<QueryInterfacesFnV0> =
        unsafe { lib.get::<QueryInterfacesFnV0>(QUERY_INTERFACES_FN_V0_NAME) }
            .context(PluginLoadFailed { plugin: &path })?;
    let query_interfaces = Box::new(*query_interfaces);
    let mut caps: Options = Options::new();
    let code = initialize(cfm, opts, &mut caps);
    if code != 0 {
        return Err(PluginInitializationFailed {
            plugin: path,
            reason: "initialize failed",
        }
        .build());
    }
    if caps.is_empty() {
        return Err(PluginInitializationFailed {
            plugin: &path,
            reason: "empty capabilities",
        }
        .build());
    }
    Ok(Plugin {
        library: Rc::new(lib),
        capabilities: caps,
        vtable: PluginVTable::V0(PluginV0 {
            initialize: Box::new(*initialize),
            query_interfaces: Box::new(*query_interfaces),
        }),
    })
}

fn cfm_plugin_load_(
    cfm: *mut Confium,
    c_path: *const c_char,
    opts: *mut Options,
    errptr: *mut *mut Error,
) -> Result<()> {
    check_not_null!(cfm, errptr);
    check_not_null!(c_path, errptr);
    let cfm = unsafe { &mut *cfm };
    let path = std::path::PathBuf::from(cstring(c_path)?);
    let lib = unsafe { Library::new(&path).context(PluginLoadFailed { plugin: &path })? };
    let plugin_iface_ver = unsafe { lib.get::<InterfaceVersionFn>(INTERFACE_VERSION_FN_NAME) }
        .context(PluginLoadFailed { plugin: &path })?;
    match plugin_iface_ver(cfm) {
        0 => {
            let plugin = load_plugin_v0(cfm, &path, lib, unsafe { &mut *opts }, errptr)?;
            cfm.plugins.push(plugin);
        }
        _ => return Err(PluginVersionUnsupported { plugin: &path }.build()),
    }
    Ok(())
}

#[no_mangle]
pub extern "C" fn cfm_plugin_load(
    cfm: *mut Confium,
    c_path: *const c_char,
    opts: *mut Options,
    errptr: *mut *mut Error,
) -> u32 {
    cfm_plugin_load_(cfm, c_path, opts, errptr).map_or_else(|e| ffi_return_err!(e, errptr), |_| 0)
}

#[no_mangle]
pub extern "C" fn cfm_plugin_unload(cfm: *mut Confium, c_name: *const c_char) -> u32 {
    unimplemented!();
}
