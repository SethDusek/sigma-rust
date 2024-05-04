//! Derivation Path functionality

use crate::{delete_ptr, ErrorPtr};
use ergo_lib_c_core::derivation_path::{
    derivation_path_depth, derivation_path_from_str, derivation_path_new, derivation_path_next,
    derivation_path_to_str, ConstDerivationPathPtr, DerivationPathPtr,
};
use ergo_lib_c_core::Error;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

/// Create DerivationPath from account index and address indices
#[no_mangle]
pub unsafe extern "C" fn ergo_lib_derivation_path_new(
    account: u32,
    address_indices: *const u32,
    len: usize,
    derivation_path_out: *mut DerivationPathPtr,
) -> ErrorPtr {
    let address_indices = std::slice::from_raw_parts(address_indices, len);
    let res = derivation_path_new(account, address_indices, derivation_path_out);
    Error::c_api_from(res)
}

/// Create derivation path from string
/// String should be in the form of: m/44/429/acc'/0/addr
#[no_mangle]
pub unsafe extern "C" fn ergo_lib_derivation_path_from_str(
    derivation_path_str: *const c_char,
    derivation_path_out: *mut DerivationPathPtr,
) -> ErrorPtr {
    let derivation_path_str = CStr::from_ptr(derivation_path_str).to_string_lossy();
    let res = derivation_path_from_str(&derivation_path_str, derivation_path_out);
    Error::c_api_from(res)
}

/// Get derivation path as string in the m/44/429/acc'/0/addr format
#[no_mangle]
pub unsafe extern "C" fn ergo_lib_derivation_path_to_str(
    derivation_path_ptr: ConstDerivationPathPtr,
    _derivation_path_str: *mut *const c_char,
) {
    #[allow(clippy::unwrap_used)]
    {
        let s = derivation_path_to_str(derivation_path_ptr).unwrap();
        *_derivation_path_str = CString::new(s).unwrap().into_raw();
    }
}

/// Returns the length of the derivation path
#[no_mangle]
pub unsafe extern "C" fn ergo_lib_derivation_path_depth(
    derivation_path_ptr: ConstDerivationPathPtr,
) -> usize {
    #[allow(clippy::unwrap_used)]
    derivation_path_depth(derivation_path_ptr).unwrap()
}

/// Returns a new derivation path with the last element of the derivation path being increased, e.g. m/1/2 -> m/1/3
#[no_mangle]
pub unsafe extern "C" fn ergo_lib_derivation_path_next(
    derivation_path_ptr: ConstDerivationPathPtr,
    derivation_path_out: *mut DerivationPathPtr,
) -> ErrorPtr {
    let res = derivation_path_next(derivation_path_ptr, derivation_path_out);
    Error::c_api_from(res)
}

/// Drop `DerivationPath`
#[no_mangle]
pub extern "C" fn ergo_lib_derivation_path_delete(ptr: DerivationPathPtr) {
    unsafe { delete_ptr(ptr) }
}
