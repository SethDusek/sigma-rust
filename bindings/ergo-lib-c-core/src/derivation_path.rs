//! Derivation Path functionality

use crate::util::const_ptr_as_ref;
use crate::{util::mut_ptr_as_mut, Error};
use derive_more::{From, Into};
use ergo_lib::wallet;
use ergo_lib::wallet::derivation_path::{
    ChildIndexError, ChildIndexHardened, ChildIndexNormal, DerivationPath as InnerDerivationPath,
};
use std::str::FromStr;

#[derive(From, Into)]
pub struct DerivationPath(pub InnerDerivationPath);
pub type DerivationPathPtr = *mut DerivationPath;
pub type ConstDerivationPathPtr = *const DerivationPath;

/// Create DerivationPath from account index and address indices
pub unsafe fn derivation_path_new(
    account: u32,
    address_indices: &[u32],
    derivation_path_out: *mut DerivationPathPtr,
) -> Result<(), Error> {
    let derivation_path_out = mut_ptr_as_mut(derivation_path_out, "derivation_path_out")?;
    let acc = ChildIndexHardened::from_31_bit(account)?;
    let address_indices = address_indices
        .iter()
        .map(|i| ChildIndexNormal::normal(*i))
        .collect::<Result<Vec<ChildIndexNormal>, ChildIndexError>>()
        .map_err(Error::misc)?;
    let derivation_path = DerivationPath(InnerDerivationPath::new(acc, address_indices));
    *derivation_path_out = Box::into_raw(Box::new(derivation_path));
    Ok(())
}

/// Create derivation path from string
/// String should be in the form of: m/44/429/acc'/0/addr
pub unsafe fn derivation_path_from_str(
    derivation_path_str: &str,
    derivation_path_out: *mut DerivationPathPtr,
) -> Result<(), Error> {
    let derivation_path_out = mut_ptr_as_mut(derivation_path_out, "derivation_path_out")?;
    let derivation_path = wallet::derivation_path::DerivationPath::from_str(derivation_path_str)
        .map_err(Error::misc)?;
    *derivation_path_out = Box::into_raw(Box::new(DerivationPath(derivation_path)));
    Ok(())
}

/// Get derivation path as string in the m/44/429/acc'/0/addr format
pub unsafe fn derivation_path_to_str(
    derivation_path_ptr: ConstDerivationPathPtr,
) -> Result<String, Error> {
    let derivation_path = const_ptr_as_ref(derivation_path_ptr, "derivation_path_ptr")?;
    let s = derivation_path.0.to_string();
    Ok(s)
}

/// Returns the length of the derivation path
pub unsafe fn derivation_path_depth(
    derivation_path_ptr: ConstDerivationPathPtr,
) -> Result<usize, Error> {
    let derivation_path = const_ptr_as_ref(derivation_path_ptr, "derivation_path_ptr")?;
    Ok(derivation_path.0.depth())
}

/// Returns a new derivation path with the last element of the derivation path being increased, e.g. m/1/2 -> m/1/3
pub unsafe fn derivation_path_next(
    derivation_path_ptr: ConstDerivationPathPtr,
    derivation_path_out: *mut DerivationPathPtr,
) -> Result<(), Error> {
    let derivation_path = const_ptr_as_ref(derivation_path_ptr, "derivation_path_ptr")?;
    let derivation_path_out = mut_ptr_as_mut(derivation_path_out, "derivation_path_out")?;
    *derivation_path_out = Box::into_raw(Box::new(DerivationPath(
        derivation_path.0.next().map_err(Error::misc)?,
    )));
    Ok(())
}
