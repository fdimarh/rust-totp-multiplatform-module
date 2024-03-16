extern crate jni;

use jni::JNIEnv;
use jni::objects::{JClass, JString};

use totp_multiplatform_module::totp;

#[no_mangle]
pub extern "system" fn Java_id_co_ngecamp_totp_TOTP_getPinNumber<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    otp_digit: i32,
    time_threshold: i32,
    time_expiry: i64,
    secret: JString<'local>,
) -> JString<'local> {
    let secret: String = env
        .get_string(&secret)
        .expect("Couldn't get java string!")
        .into();
    let pin_number = totp::get_pin_number(otp_digit as usize, time_threshold as u8, time_expiry as u64, secret);
    let output = env
        .new_string(format!("{}", pin_number))
        .expect("Couldn't create java string!");
    output
}

