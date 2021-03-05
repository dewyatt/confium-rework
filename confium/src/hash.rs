use crate::Confium;
use crate::Result;

pub struct Hash {
    name: String,
}

//new
//reset
//clone/copy
//digest/output/result
//blocksize
//outputsize/bytes/bits
//feed/input/update/add

impl Hash {
    pub fn new(cfm: &Confium, name: &str) -> Hash {
        // use the plugins at the FFI layer to create this...
        panic!();
    }

    pub fn reset() -> Result<()> {
        panic!();
    }

    // Hash::clone? or hash.clone
    pub fn clone() -> Result<()> {
        panic!();
    }

    // TODO: name - digest, output?
    pub fn digest() -> Result<()> {
        panic!();
    }

    // TODO:
}

impl Drop for Hash {
    fn drop(&mut self) {}
}
