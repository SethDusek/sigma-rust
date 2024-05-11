//! Ergo blockchain state (for ErgoTree evaluation)

use crate::util::mut_ptr_as_mut;
use crate::Error;
use ergo_lib::chain;

/// Blockchain parameters
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Parameters(pub(crate) chain::parameters::Parameters);
pub type ParametersPtr = *mut Parameters;
pub type ConstParametersPtr = *const Parameters;

/// Return default blockchain parameters that were set at genesis
pub unsafe fn parameters_default(parameters_out: *mut ParametersPtr) {
    *parameters_out = Box::into_raw(Box::new(Parameters(
        chain::parameters::Parameters::default(),
    )));
}

/// Create new parameters from provided blockchain parameters
#[allow(clippy::too_many_arguments)]
pub unsafe fn parameters_new(
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
) -> Result<(), Error> {
    let parameters_out = mut_ptr_as_mut(parameters_out, "parameters_out")?;
    let parameters = chain::parameters::Parameters::new(
        block_version,
        storage_fee_factor,
        min_value_per_byte,
        max_block_size,
        max_block_cost,
        token_access_cost,
        input_cost,
        data_input_cost,
        output_cost,
    );
    *parameters_out = Box::into_raw(Box::new(Parameters(parameters)));
    Ok(())
}

/// Parse parameters from JSON. Supports Ergo Node API/Explorer API
pub unsafe fn parameters_from_json(
    json: &str,
    parameters_out: *mut ParametersPtr,
) -> Result<(), Error> {
    let parameters_out = mut_ptr_as_mut(parameters_out, "parameters_out")?;
    let parameters = serde_json::from_str(json).map(Parameters)?;
    *parameters_out = Box::into_raw(Box::new(parameters));
    Ok(())
}

pub unsafe fn parameters_delete(parameters: ParametersPtr) {
    if !parameters.is_null() {
        let boxed = Box::from_raw(parameters);
        std::mem::drop(boxed);
    }
}
