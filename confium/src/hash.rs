use crate::Confium;
use crate::Result;

//new
//update
//reset
//clone
//finalize
//block_size
//output_size

pub struct Hash {
    name: String,
}

impl Hash {
    pub fn new(cfm: &Confium, name: &str) -> Result<Hash> {
        // use the plugins at the FFI layer to create this...
        unimplemented!();
    }

    pub fn update(&mut self, data: impl AsRef<[u8]>) -> Result<()> {
        unimplemented!();
    }

    pub fn reset(&mut self) -> Result<()> {
        unimplemented!();
    }

    // TODO: name - digest, output?
    pub fn finalize(&mut self) -> Result<Vec<u8>> {
        unimplemented!();
    }

    pub fn block_size(&self) -> Result<u8> {
        unimplemented!();
    }

    pub fn output_size(&self) -> Result<usize> {
        unimplemented!();
    }

    pub fn digest(cfm: &Confium, name: &str, data: &[u8]) -> Result<Vec<u8>> {
        let mut hash = Hash::new(cfm, name)?;
        hash.update(data)?;
        hash.finalize()
    }
}

impl Drop for Hash {
    fn drop(&mut self) {}
}

impl Clone for Hash {
    fn clone(&self) -> Self {
        unimplemented!();
    }
}
