use crate::Error;
use ergo_lib::wallet::mnemonic::Mnemonic as InnerMnemonic;

/// Convert a mnemonic phrase into a mnemonic seed
/// mnemonic_pass is optional and is used to salt the seed
pub unsafe fn mnemonic_to_seed(
    mnemonic_phrase: &str,
    mnemonic_pass: &str,
    output: *mut u8,
) -> Result<(), Error> {
    let src: Vec<u8> = InnerMnemonic::to_seed(mnemonic_phrase, mnemonic_pass).into();
    std::ptr::copy_nonoverlapping(src.as_ptr(), output, src.len());
    Ok(())
}
