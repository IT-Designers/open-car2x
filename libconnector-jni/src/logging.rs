use connector::logging::{cr_configure_logger, CrLogLevel};
use jni::objects::JClass;
use jni::sys::{jint, jlong};
use jni::JNIEnv;
use num_traits::FromPrimitive;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeLogger_nativeConfigure(
    _env: JNIEnv,
    _class: JClass,
    level: jint,
) -> jlong {
    unsafe { cr_configure_logger(CrLogLevel::from_i32(level).unwrap()) as jlong }
}
