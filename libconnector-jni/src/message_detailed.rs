use connector::pods::message::CrMessageRef;
use connector::result::CrResult;
use connector::worker::rcv::{
    cr_detailed_message_creation_time, cr_detailed_message_free,
    cr_detailed_message_reception_time, CrDetailedMessage,
};
use jni::objects::JClass;
use jni::sys::{jint, jlong, jlongArray};
use jni::JNIEnv;

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeDetailedMessage_nativeDelete(
    _env: JNIEnv,
    _class: JClass,
    message: jlong,
) -> jlong {
    cr_detailed_message_free(message as _) as _
}

unsafe fn map_deref<R>(
    ptr: jlong,
    default: R,
    f: impl FnOnce(&CrDetailedMessage) -> R,
) -> (CrResult, R) {
    if ptr != 0 {
        let obj = ptr as *const CrDetailedMessage;
        let obj = &*obj;
        (CrResult::Ok, f(obj))
    } else {
        (CrResult::ErrParameterMessageIsNull, default)
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeDetailedMessage_nativeGetContent(
    env: JNIEnv,
    _class: JClass,
    message: jlong,
) -> jlongArray {
    let (result, content) = map_deref(message, 0, |m| &m.content as *const CrMessageRef as jlong);

    let array = env.new_long_array(2).unwrap();
    env.set_long_array_region(array, 0, &[result as jlong, content as jlong])
        .unwrap();
    array
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeDetailedMessage_nativeGetContentMessageId(
    env: JNIEnv,
    _class: JClass,
    message: jlong,
) -> jlongArray {
    let (result, content) = map_deref(message, 0, |m| (&m.content as &CrMessageRef).id as jint);

    let array = env.new_long_array(2).unwrap();
    env.set_long_array_region(array, 0, &[result as jlong, content as jlong])
        .unwrap();
    array
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeDetailedMessage_nativeGetCreationTimeMillis(
    env: JNIEnv,
    _class: JClass,
    message: jlong,
) -> jlongArray {
    let mut creation_time = 0_u64;
    let result = cr_detailed_message_creation_time(message as _, &mut creation_time as _);

    let array = env.new_long_array(2).unwrap();
    env.set_long_array_region(array, 0, &[result as jlong, creation_time as _])
        .unwrap();
    array
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeDetailedMessage_nativeGetReceptionTimeMillis(
    env: JNIEnv,
    _class: JClass,
    message: jlong,
) -> jlongArray {
    let mut reception_time = 0_u64;
    let result = cr_detailed_message_reception_time(message as _, &mut reception_time as _);

    let array = env.new_long_array(2).unwrap();
    env.set_long_array_region(array, 0, &[result as jlong, reception_time as _])
        .unwrap();
    array
}
