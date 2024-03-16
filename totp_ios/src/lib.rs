use std::os::raw::{c_char};
use std::ffi::{CString, CStr};

use totp_multiplatform_module::totp;


#[no_mangle]
pub extern fn get_pin_number(otp_digit: usize, time_threshold: u8, time_expiry: u64, secret: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(secret) };
    let secret = match c_str.to_str() {
        Err(_) => "there is an error in the input string",
        Ok(string) => string,
    };
    let str = totp::get_pin_number(otp_digit, time_threshold, time_expiry, secret.to_string());
    CString::new(str.to_owned()).unwrap().into_raw()
}
