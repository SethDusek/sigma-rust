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
