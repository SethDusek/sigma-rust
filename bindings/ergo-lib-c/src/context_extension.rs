use ergo_lib_c_core::constant::{ConstConstantPtr, ConstantPtr};
use ergo_lib_c_core::context_extension::*;
use ergo_lib_c_core::Error;

use crate::{delete_ptr, ReturnOption};

/// Create new empty ContextExtension instance
#[no_mangle]
pub unsafe extern "C" fn ergo_lib_context_extension_empty(
    context_extension_out: *mut ContextExtensionPtr,
) {
    #[allow(clippy::unwrap_used)]
    context_extension_empty(context_extension_out).unwrap();
}

/// Returns the number of elements in the collection
#[no_mangle]
pub unsafe extern "C" fn ergo_lib_context_extension_len(
    context_extension_ptr: ConstContextExtensionPtr,
) -> usize {
    #[allow(clippy::unwrap_used)]
    context_extension_len(context_extension_ptr).unwrap()
}

/// Returns all keys (represented as u8 values) in the map
#[no_mangle]
pub unsafe extern "C" fn ergo_lib_context_extension_keys(
    context_extension_ptr: ConstContextExtensionPtr,
    output: *mut u8,
) {
    #[allow(clippy::unwrap_used)]
    context_extension_keys(context_extension_ptr, output).unwrap();
}

/// Returns constant with given key
/// or None if key doesn't exist
/// or error if constants parsing were failed
#[no_mangle]
pub unsafe extern "C" fn ergo_lib_context_extension_get(
    context_extension_ptr: ConstContextExtensionPtr,
    key: u8,
    constant_out: *mut ConstantPtr,
) -> ReturnOption {
    match context_extension_get(context_extension_ptr, key, constant_out) {
        Ok(is_some) => ReturnOption {
            is_some,
            error: std::ptr::null_mut(),
        },
        Err(e) => ReturnOption {
            is_some: false, // Just a dummy value
            error: Error::c_api_from(Err(e)),
        },
    }
}

/// Set the supplied pair in the ContextExtension
#[no_mangle]
pub unsafe extern "C" fn ergo_lib_context_extension_set_pair(
    constant_ptr: ConstConstantPtr,
    key: u8,
    context_extension_ptr: ContextExtensionPtr,
) {
    #[allow(clippy::unwrap_used)]
    context_extension_set_pair(constant_ptr, key, context_extension_ptr).unwrap()
}

/// Drop `ContextExtension`
#[no_mangle]
pub unsafe extern "C" fn ergo_lib_context_extension_delete(ptr: ContextExtensionPtr) {
    delete_ptr(ptr)
}
