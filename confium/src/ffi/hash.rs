use crate::hash::Hash;
use crate::Confium;

use std::os::raw::c_char;

pub type HashCreateFn = extern "C" fn(*mut Confium, *const c_char, *mut *mut Hash) -> u32;
