use connector::result::{cr_result_str, CrResult};
use jni::objects::JClass;
use jni::sys::{jlong, jstring};
use jni::JNIEnv;
use num_traits::cast::FromPrimitive;
use std::ffi::CStr;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeResultException_getMessage(
    env: JNIEnv,
    _class: JClass,
    error_code: jlong,
) -> jstring {
    let string = cr_result_str(CrResult::from_i64(error_code).unwrap());
    let string = unsafe {
        // SAFETY: this just came out of a rust function and is therefore valid utf8
        CStr::from_ptr(string).to_string_lossy()
    };
    env.new_string(string).unwrap().into_inner()
}
