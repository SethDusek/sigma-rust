use ergo_lib_c_core::mnemonic::mnemonic_to_seed;
use std::ffi::CStr;
use std::os::raw::c_char;

/// Convert a mnemonic phrase into a mnemonic seed
/// mnemonic_pass is optional and is used to salt the seed
#[no_mangle]
pub unsafe extern "C" fn ergo_lib_mnemonic_to_seed(
    mnemonic_phrase: *const c_char,
    mnemonic_pass: *const c_char,
    output: *mut u8,
) {
    let mnemonic_phrase = CStr::from_ptr(mnemonic_phrase).to_string_lossy();
    let mnemonic_pass = CStr::from_ptr(mnemonic_pass).to_string_lossy();
    #[allow(clippy::unwrap_used)]
    mnemonic_to_seed(&mnemonic_phrase, &mnemonic_pass, output).unwrap()
}
