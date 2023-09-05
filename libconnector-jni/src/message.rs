use connector::pods::message::CrMessage;
use connector::result::CrResult;
use jni::objects::JClass;
use jni::sys::{jbyte, jbyteArray, jlong, jlongArray};
use jni::JNIEnv;

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeMessage_nativeDelete(
    _env: JNIEnv,
    _class: JClass,
    message: jlong,
) {
    if message != 0 {
        drop(Box::from_raw(message as *mut CrMessage));
    }
}

unsafe fn map_deref<R>(ptr: jlong, default: R, f: impl FnOnce(&CrMessage) -> R) -> (CrResult, R) {
    if ptr != 0 {
        let obj = ptr as *const CrMessage;
        let obj = &*obj;
        (CrResult::Ok, f(obj))
    } else {
        (CrResult::ErrParameterMessageIsNull, default)
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeMessage_nativeGetMessageId(
    env: JNIEnv,
    _class: JClass,
    message: jlong,
) -> jlongArray {
    let (result, size) = map_deref(message, 0, |m| m.id.into_single().unwrap() as jlong);

    let array = env.new_long_array(2).unwrap();
    env.set_long_array_region(array, 0, &[result as jlong, size as jlong])
        .unwrap();
    array
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeMessage_nativeGetDataSize(
    env: JNIEnv,
    _class: JClass,
    message: jlong,
) -> jlongArray {
    let (result, size) = map_deref(message, 0, |m| m.size as jlong);

    let array = env.new_long_array(2).unwrap();
    env.set_long_array_region(array, 0, &[result as jlong, size as jlong])
        .unwrap();
    array
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeMessage_nativeGetData(
    env: JNIEnv,
    _class: JClass,
    message: jlong,
    array: jbyteArray,
) -> jlongArray {
    let (result, size) = map_deref(message, 0, |m| {
        let body = m.body.as_ptr() as *const jbyte;
        let result = env.set_byte_array_region(array, 0, core::slice::from_raw_parts(body, m.size));
        match result {
            Ok(_) => m.size,
            Err(_) => 0,
        }
    });

    let array = env.new_long_array(2).unwrap();
    env.set_long_array_region(array, 0, &[result as jlong, size as jlong])
        .unwrap();
    array
}
