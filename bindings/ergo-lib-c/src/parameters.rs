use ergo_lib_c_core::parameters::{
    parameters_default, parameters_from_json, parameters_new, ParametersPtr,
};
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

/// Create new parameters from provided blockchain parameters
#[no_mangle]
pub unsafe extern "C" fn ergo_lib_parameters_new(
    block_version: i32,
    storage_fee_factor: i32,
    min_value_per_byte: i32,
    max_block_size: i32,
    max_block_cost: i32,
    token_access_cost: i32,
    input_cost: i32,
    data_input_cost: i32,
    output_cost: i32,
    parameters_out: *mut ParametersPtr,
) {
    #[allow(clippy::unwrap_used)]
    parameters_new(
        block_version,
        storage_fee_factor,
        min_value_per_byte,
        max_block_size,
        max_block_cost,
        token_access_cost,
        input_cost,
        data_input_cost,
        output_cost,
        parameters_out,
    )
    .unwrap();
}

#[no_mangle]
pub unsafe extern "C" fn ergo_lib_parameters_delete(parameters: ParametersPtr) {
    delete_ptr(parameters)
}
