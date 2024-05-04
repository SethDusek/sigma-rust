//! Extended Secret Key functionality

use std::convert::TryInto;

use derive_more::{From, Into};

use ergo_lib::wallet::derivation_path::ChildIndex;
use ergo_lib::wallet::ext_secret_key::{
    ChainCode, ExtSecretKey as InnerExtSecretKey, SecretKeyBytes,
};
use ergo_lib::wallet::mnemonic::MnemonicSeed;
use ergo_lib::ArrLength;

use crate::derivation_path::{ConstDerivationPathPtr, DerivationPath, DerivationPathPtr};
use crate::ext_pub_key::{ExtPubKey, ExtPubKeyPtr};
use crate::secret_key::{SecretKey, SecretKeyPtr};
use crate::util::const_ptr_as_ref;
use crate::{util::mut_ptr_as_mut, Error};

#[derive(From, Into)]
pub struct ExtSecretKey(InnerExtSecretKey);
pub type ExtSecretKeyPtr = *mut ExtSecretKey;
pub type ConstExtSecretKeyPtr = *const ExtSecretKey;

/// Create ExtSecretKey from secret key bytes, chain code and derivation path
/// secret_key_bytes needs to be the length of SecretKeyBytes::LEN (32 bytes)
/// chain_code needs to be the length of ChainCode::LEN (32 bytes)
pub unsafe fn ext_secret_key_new(
    secret_key_bytes: *const u8,
    chain_code: *const u8,
    derivation_path_ptr: ConstDerivationPathPtr,
    ext_secret_key_out: *mut ExtSecretKeyPtr,
) -> Result<(), Error> {
    let ext_secret_key_out = mut_ptr_as_mut(ext_secret_key_out, "ext_secret_key_out")?;
    let derivation_path = const_ptr_as_ref(derivation_path_ptr, "derivation_path_ptr")?;
    let secret_key_bytes = std::slice::from_raw_parts(secret_key_bytes, SecretKeyBytes::LEN);
    let chain_code = std::slice::from_raw_parts(chain_code, ChainCode::LEN);
    let key = InnerExtSecretKey::new(
        secret_key_bytes.try_into().map_err(Error::misc)?,
        chain_code.try_into().map_err(Error::misc)?,
        derivation_path.0.clone(),
    )
    .map_err(Error::misc)?;
    *ext_secret_key_out = Box::into_raw(Box::new(ExtSecretKey(key)));
    Ok(())
}

/// Derive root extended secret key
pub unsafe fn ext_secret_key_derive_master(
    seed: *const u8,
    ext_secret_key_out: *mut ExtSecretKeyPtr,
) -> Result<(), Error> {
    let ext_secret_key_out = mut_ptr_as_mut(ext_secret_key_out, "ext_secret_key_out")?;
    let seed = std::slice::from_raw_parts(seed, MnemonicSeed::LEN);
    let key = InnerExtSecretKey::derive_master(seed.try_into().map_err(Error::misc)?)
        .map_err(Error::misc)?;
    *ext_secret_key_out = Box::into_raw(Box::new(ExtSecretKey(key)));
    Ok(())
}

/// Derive a new extended secret key from the provided index
/// The index is in the form of soft or hardened indices
/// For example: 4 or 4' respectively
pub unsafe fn ext_secret_key_child(
    derive_from_key_ptr: ConstExtSecretKeyPtr,
    child_index: &str,
    ext_secret_key_out: *mut ExtSecretKeyPtr,
) -> Result<(), Error> {
    let ext_secret_key = const_ptr_as_ref(derive_from_key_ptr, "derive_from_key_ptr")?;
    let ext_secret_key_out = mut_ptr_as_mut(ext_secret_key_out, "ext_secret_key_out")?;
    let index = child_index.parse::<ChildIndex>().map_err(Error::misc)?;
    let key = ext_secret_key.0.child(index).map_err(Error::misc)?;
    *ext_secret_key_out = Box::into_raw(Box::new(ExtSecretKey(key)));
    Ok(())
}

/// Get derivation path for extended secret key
pub unsafe fn ext_secret_key_path(
    ext_secret_key_ptr: ConstExtSecretKeyPtr,
    derivation_path_out: *mut DerivationPathPtr,
) -> Result<(), Error> {
    let ext_secret_key = const_ptr_as_ref(ext_secret_key_ptr, "ext_secret_key_ptr")?;
    let derivation_path_out = mut_ptr_as_mut(derivation_path_out, "derivation_path_out")?;
    *derivation_path_out = Box::into_raw(Box::new(DerivationPath(ext_secret_key.0.path())));
    Ok(())
}

/// Get secret key for extended secret key
pub unsafe fn ext_secret_key_get_secret_key(
    ext_secret_key_ptr: ConstExtSecretKeyPtr,
    secret_key_out: *mut SecretKeyPtr,
) -> Result<(), Error> {
    let ext_secret_key = const_ptr_as_ref(ext_secret_key_ptr, "ext_secret_key_ptr")?;
    let secret_key_out = mut_ptr_as_mut(secret_key_out, "secret_key_out")?;
    *secret_key_out = Box::into_raw(Box::new(SecretKey(ext_secret_key.0.secret_key())));
    Ok(())
}

/// The extended public key associated with this secret key
pub unsafe fn ext_secret_key_public_key(
    ext_secret_key_ptr: ConstExtSecretKeyPtr,
    ext_pub_key_out: *mut ExtPubKeyPtr,
) -> Result<(), Error> {
    let ext_secret_key = const_ptr_as_ref(ext_secret_key_ptr, "ext_secret_key_ptr")?;
    let ext_pub_key_out = mut_ptr_as_mut(ext_pub_key_out, "ext_pub_key_out")?;
    let ext_pub_key = ExtPubKey(ext_secret_key.0.public_key().map_err(Error::misc)?);
    *ext_pub_key_out = Box::into_raw(Box::new(ext_pub_key));
    Ok(())
}

/// Derive a new extended secret key from the derivation path
pub unsafe fn ext_secret_key_derive(
    ext_secret_key_ptr: ConstExtSecretKeyPtr,
    derivation_path_ptr: ConstDerivationPathPtr,
    ext_secret_key_out: *mut ExtSecretKeyPtr,
) -> Result<(), Error> {
    let ext_secret_key = const_ptr_as_ref(ext_secret_key_ptr, "ext_secret_key_ptr")?;
    let derivation_path = const_ptr_as_ref(derivation_path_ptr, "derivation_path_ptr")?;
    let ext_secret_key_out = mut_ptr_as_mut(ext_secret_key_out, "ext_secret_key_out")?;
    *ext_secret_key_out = Box::into_raw(Box::new(ExtSecretKey(
        ext_secret_key
            .0
            .derive(derivation_path.0.clone())
            .map_err(Error::misc)?,
    )));
    Ok(())
}
