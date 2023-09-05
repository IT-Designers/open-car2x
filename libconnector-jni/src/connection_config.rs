use connector::worker::conf::{
    cr_config_set_address, cr_config_set_login_anonymous, cr_config_set_login_password,
    cr_config_set_login_user, cr_config_set_receive_filter, cr_config_set_receive_own,
    cr_config_set_reconnect_timeout_millis, cr_config_set_send_timeout_millis,
    cr_config_set_source_exchange, cr_config_set_station_id,
    cr_config_set_station_id_receive_filter, cr_config_set_target_exchange, CrConnectionConfig,
};
use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jlong, JNI_TRUE};
use jni::JNIEnv;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeConnectionConfig_nativeCreateDefault(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    Box::into_raw(Box::<CrConnectionConfig>::default()) as _
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeConnectionConfig_nativeDelete(
    _env: JNIEnv,
    _class: JClass,
    obj: jlong,
) {
    if obj != 0 {
        drop(unsafe { Box::from_raw(obj as usize as *mut CrConnectionConfig) });
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeConnectionConfig_nativeSetAddress(
    env: JNIEnv,
    _class: JClass,
    obj: jlong,
    address: JString,
) -> jlong {
    let address = cstring_or_utf8_error!(env, address);
    unsafe { cr_config_set_address(obj as _, address.as_ptr()) as jlong }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeConnectionConfig_nativeSetReconnectTimeoutMillis(
    _env: JNIEnv,
    _class: JClass,
    obj: jlong,
    millis: jlong,
) -> jlong {
    unsafe { cr_config_set_reconnect_timeout_millis(obj as _, millis as u64) as jlong }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeConnectionConfig_nativeSetSendTimeoutMillis(
    _env: JNIEnv,
    _class: JClass,
    obj: jlong,
    millis: jlong,
) -> jlong {
    unsafe { cr_config_set_send_timeout_millis(obj as _, millis as u64) as jlong }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeConnectionConfig_nativeSetReceiveOwnMessage(
    _env: JNIEnv,
    _class: JClass,
    obj: jlong,
    receive: jboolean,
) -> jlong {
    unsafe { cr_config_set_receive_own(obj as _, receive == JNI_TRUE) as jlong }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeConnectionConfig_nativeSetFilterOptions(
    _env: JNIEnv,
    _class: JClass,
    obj: jlong,
    filterOverloaded: jlong,
) -> jlong {
    unsafe {
        cr_config_set_receive_filter(obj as _, core::mem::transmute(filterOverloaded)) as jlong
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeConnectionConfig_nativeSetLoginUser(
    env: JNIEnv,
    _class: JClass,
    obj: jlong,
    user: JString,
) -> jlong {
    let user = cstring_or_utf8_error!(env, user);
    unsafe { cr_config_set_login_user(obj as _, user.as_ptr()) as jlong }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeConnectionConfig_nativeSetLoginPassword(
    env: JNIEnv,
    _class: JClass,
    obj: jlong,
    password: JString,
) -> jlong {
    let password = cstring_or_utf8_error!(env, password);
    unsafe { cr_config_set_login_password(obj as _, password.as_ptr()) as jlong }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeConnectionConfig_nativeSetLoginAnonymous(
    _env: JNIEnv,
    _class: JClass,
    obj: jlong,
    anonymous: jboolean,
) -> jlong {
    unsafe { cr_config_set_login_anonymous(obj as _, anonymous == JNI_TRUE) as jlong }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeConnectionConfig_nativeTargetExchange(
    env: JNIEnv,
    _class: JClass,
    obj: jlong,
    exchange: JString,
) -> jlong {
    let exchange = cstring_or_utf8_error!(env, exchange);
    unsafe { cr_config_set_target_exchange(obj as _, exchange.as_ptr()) as jlong }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeConnectionConfig_nativeSourceExchange(
    env: JNIEnv,
    _class: JClass,
    obj: jlong,
    exchange: JString,
) -> jlong {
    let exchange = cstring_or_utf8_error!(env, exchange);
    unsafe { cr_config_set_source_exchange(obj as _, exchange.as_ptr()) as jlong }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeConnectionConfig_nativeStationId(
    _env: JNIEnv,
    _class: JClass,
    obj: jlong,
    station_id: i32,
) -> jlong {
    unsafe { cr_config_set_station_id(obj as _, station_id as u32) as jlong }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeConnectionConfig_nativeStationIdReceiveFilter(
    _env: JNIEnv,
    _class: JClass,
    obj: jlong,
    station_id: i32,
) -> jlong {
    unsafe { cr_config_set_station_id_receive_filter(obj as _, station_id as u32) as jlong }
}
