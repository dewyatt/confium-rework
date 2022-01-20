use crate::ffi::plugin::PluginInterface;
use std::rc::Rc;

use libloading::Library;

use crate::pk::ffi::rsa::FFIRSAPublicKey;

pub struct RSAPublicKey {
    obj: *mut FFIRSAPublicKey,
    lib: Rc<Library>,
    interface: Rc<PluginInterface>,
}
