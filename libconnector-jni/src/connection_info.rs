use connector::pods::connection::CrConnectionInfo;
use connector::pods::message::CrMessageIdOverloaded;
use connector::result::CrResult;
use jni::objects::JClass;
use jni::sys::{jlong, jlongArray, jsize};
use jni::JNIEnv;

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeConnectionInfo_nativeDelete(
    _env: JNIEnv,
    _class: JClass,
    info: jlong,
) {
    if info != 0 {
        drop(Box::from_raw(info as *mut CrConnectionInfo));
    }
}

unsafe fn map_deref<R>(
    ptr: jlong,
    default: R,
    f: impl FnOnce(&CrConnectionInfo) -> R,
) -> (CrResult, R) {
    if ptr != 0 {
        let obj = ptr as *const CrConnectionInfo;
        let obj = &*obj;
        (CrResult::Ok, f(obj))
    } else {
        (CrResult::ErrParameterConnectorInfoIsNull, default)
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeConnectionInfo_nativeGetStatus(
    env: JNIEnv,
    _class: JClass,
    info: jlong,
) -> jlongArray {
    let (result, status) = map_deref(info, 0, |info| info.status as jlong);

    let array = env.new_long_array(2).unwrap();
    env.set_long_array_region(array, 0, &[result as jlong, status])
        .unwrap();
    array
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeConnectionInfo_nativeGetTimesConnectedCounter(
    env: JNIEnv,
    _class: JClass,
    info: jlong,
) -> jlongArray {
    let (result, status) = map_deref(info, 0, |info| info.times_connected_counter as jlong);

    let array = env.new_long_array(2).unwrap();
    env.set_long_array_region(array, 0, &[result as jlong, status])
        .unwrap();
    array
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeConnectionInfo_nativeGetConnectionEpochMillis(
    env: JNIEnv,
    _class: JClass,
    info: jlong,
) -> jlongArray {
    let (result, status) = map_deref(info, 0, |info| {
        info.connection_epoch_millis_timestamp as jlong
    });

    let array = env.new_long_array(2).unwrap();
    env.set_long_array_region(array, 0, &[result as jlong, status])
        .unwrap();
    array
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeConnectionInfo_nativeGetMessageReceiverQueueSize(
    env: JNIEnv,
    _class: JClass,
    info: jlong,
) -> jlongArray {
    let (result, status) = map_deref(info, 0, |info| info.message_receiver_queue_size as jlong);

    let array = env.new_long_array(2).unwrap();
    env.set_long_array_region(array, 0, &[result as jlong, status])
        .unwrap();
    array
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeConnectionInfo_nativeGetMessageReceiverQueueTypes(
    env: JNIEnv,
    _class: JClass,
    info: jlong,
) -> jlongArray {
    let (result, status) = map_deref(info, CrMessageIdOverloaded::empty(), |info| {
        info.message_receiver_queue_types
    });

    let mut result_and_values = vec![result as jlong];
    for id in status.iter_set() {
        result_and_values.push(id as jlong);
    }

    let array = env
        .new_long_array(result_and_values.len() as jsize)
        .unwrap();
    env.set_long_array_region(array, 0, &result_and_values)
        .unwrap();
    array
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeConnectionInfo_nativeGetMessageSenderQueueSize(
    env: JNIEnv,
    _class: JClass,
    info: jlong,
) -> jlongArray {
    let (result, status) = map_deref(info, 0, |info| info.message_sender_queue_size as jlong);

    let array = env.new_long_array(2).unwrap();
    env.set_long_array_region(array, 0, &[result as jlong, status])
        .unwrap();
    array
}
