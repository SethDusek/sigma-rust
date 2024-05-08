use ergo_lib_c_core::parameters::{parameters_default, parameters_from_json, ParametersPtr};
use ergo_lib_c_core::Error;
use std::ffi::CStr;
use std::os::raw::c_char;

use crate::{delete_ptr, ErrorPtr};

/// Return default blockchain parameters that were set at genesis
#[no_mangle]
pub unsafe extern "C" fn ergo_lib_parameters_default(parameters_out: *mut ParametersPtr) {
    parameters_default(parameters_out);
}

/// Parse parameters from JSON. Supports Ergo Node API/Explorer API
#[no_mangle]
pub unsafe extern "C" fn ergo_lib_parameters_from_json(
    json_str: *const c_char,
    parameters_out: *mut ParametersPtr,
) -> ErrorPtr {
    let json = CStr::from_ptr(json_str).to_string_lossy();
    let res = parameters_from_json(&json, parameters_out);
    Error::c_api_from(res)
}

#[no_mangle]
pub unsafe extern "C" fn ergo_lib_parameters_delete(parameters: ParametersPtr) {
    delete_ptr(parameters)
}
