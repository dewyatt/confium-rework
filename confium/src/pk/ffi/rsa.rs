use crate::options::Options;
use crate::pk::rsa::RSAPublicKey;
use crate::Confium;

pub enum FFIRSAPublicKey {}

pub type RSAPubKeyCreateFnV0 = extern "C" fn(
    cfm: *const Confium,
    obj: *mut *mut FFIRSAPublicKey,
    n: *const u8,
    n_len: u32,
    e: *const u8,
    e_len: u32,
    opts: Option<&Options>,
) -> u32;
const RSA_PUBKEY_CREATE_FN_V0_NAME: &'static [u8] = b"cfmp_rsa_pubkey_create\0";

#[no_mangle]
pub extern "C" fn cfm_pk_rsa_pubkey_create(
    cfm: *const Confium,
    key: *mut *mut RSAPublicKey,
    n: *const u8,
    n_len: u32,
    e: *const u8,
    e_len: u32,
) -> u32 {
    //unsafe { *cfm = Box::into_raw(Box::new(Confium::new())) }
    unimplemented!();
    0
}

#[no_mangle]
pub extern "C" fn cfm_pk_rsa_pubkey_destroy(key: *mut RSAPublicKey) -> u32 {
    unimplemented!();
    unsafe {
        //Box::from_raw(cfm);
    }
    0
}
