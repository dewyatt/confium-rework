use std::os::raw::c_char;

use crate::error::Error;
use crate::hash::Hash;
use crate::Confium;

pub type HashCreateFn = extern "C" fn(*mut Confium, *const c_char, *mut *mut Hash) -> u32;

struct HashV0 {
    create: Box<HashCreateFn>,
}

extern "C" fn cfm_hash_create(
    cfm: *const Confium,
    hash: *mut *mut Hash,
    name: *const c_char,
    err: *mut *mut Error,
) -> u32 {
    unimplemented!();
}

extern "C" fn cfm_hash_update(hash: *mut Hash, data: *const u8, size: u32) -> u32 {
    unimplemented!();
}

extern "C" fn cfm_hash_destroy(hash: *mut Hash) -> u32 {
    unimplemented!();
}
