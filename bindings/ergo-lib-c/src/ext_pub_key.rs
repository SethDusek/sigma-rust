//! Extended Public Key functionality

use crate::{delete_ptr, ErrorPtr};
use ergo_lib_c_core::address::AddressPtr;
use ergo_lib_c_core::derivation_path::ConstDerivationPathPtr;
use ergo_lib_c_core::ext_pub_key::{
    ext_pub_key_address, ext_pub_key_child, ext_pub_key_derive, ext_pub_key_new, ConstExtPubKeyPtr,
    ExtPubKeyPtr,
};
use ergo_lib_c_core::Error;

/// Create ExtPubKey from public key bytes, chain code and derivation path
#[no_mangle]
pub unsafe extern "C" fn ergo_lib_ext_pub_key_new(
    public_key_bytes: *const u8,
    chain_code_ptr: *const u8,
    derivation_path_ptr: ConstDerivationPathPtr,
    ext_pub_key_out: *mut ExtPubKeyPtr,
) -> ErrorPtr {
    let res = ext_pub_key_new(
        public_key_bytes,
        chain_code_ptr,
        derivation_path_ptr,
        ext_pub_key_out,
    );
    Error::c_api_from(res)
}

/// Derive a new extended public key from the provided index
/// The index is in the form of soft or hardened indices
/// For example: 4 or 4' respectively
#[no_mangle]
pub unsafe extern "C" fn ergo_lib_ext_pub_key_child(
    derive_from_key_ptr: ConstExtPubKeyPtr,
    child_index: u32,
    ext_pub_key_out: *mut ExtPubKeyPtr,
) -> ErrorPtr {
    let res = ext_pub_key_child(derive_from_key_ptr, child_index, ext_pub_key_out);
    Error::c_api_from(res)
}

/// Derive a new extended public key from the derivation path
#[no_mangle]
pub unsafe extern "C" fn ergo_lib_ext_pub_key_derive(
    ext_pub_key_ptr: ConstExtPubKeyPtr,
    derivation_path_ptr: ConstDerivationPathPtr,
    ext_pub_key_out: *mut ExtPubKeyPtr,
) -> ErrorPtr {
    let res = ext_pub_key_derive(ext_pub_key_ptr, derivation_path_ptr, ext_pub_key_out);
    Error::c_api_from(res)
}

/// Get address for extended public key
#[no_mangle]
pub unsafe extern "C" fn ergo_lib_ext_pub_key_address(
    ext_pub_key_ptr: ConstExtPubKeyPtr,
    address_out: *mut AddressPtr,
) {
    #[allow(clippy::unwrap_used)]
    ext_pub_key_address(ext_pub_key_ptr, address_out).unwrap()
}

/// Drop `ExtPubKey`
#[no_mangle]
pub extern "C" fn ergo_lib_ext_pub_key_delete(ptr: ExtPubKeyPtr) {
    unsafe { delete_ptr(ptr) }
}
