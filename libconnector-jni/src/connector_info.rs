use connector::pods::connector::{cr_load_connector_info, CrConnectorInfo};
use connector::result::CrResult;
use jni::objects::JClass;
use jni::sys::{jbyteArray, jlong};
use jni::JNIEnv;
use std::mem::MaybeUninit;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeConnectorInfo_nativeLoad(
    env: JNIEnv,
    _class: JClass,
) -> jbyteArray {
    let mut info = MaybeUninit::<CrConnectorInfo>::uninit();

    let result = unsafe { cr_load_connector_info(info.as_mut_ptr()) };

    let info = if result == CrResult::Ok {
        let info = unsafe { info.assume_init() };
        Box::into_raw(Box::new(info))
    } else {
        std::ptr::null_mut()
    };

    let array = env.new_long_array(2).unwrap();
    env.set_long_array_region(array, 0, &[result as usize as jlong, info as jlong])
        .unwrap();
    array
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeConnectorInfo_nativeDelete(
    _env: JNIEnv,
    _class: JClass,
    info: jlong,
) {
    let info: *mut CrConnectorInfo = info as _;

    if !info.is_null() {
        drop(unsafe { Box::from_raw(info) })
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeConnectorInfo_nativeGetVersionPtr(
    env: JNIEnv,
    _class: JClass,
    info: jlong,
) -> jbyteArray {
    let info: *mut CrConnectorInfo = info as _;

    let (result, version) = if !info.is_null() {
        let info = unsafe { &*info };
        (CrResult::Ok, &info.connector_version as *const _)
    } else {
        (CrResult::ErrParameterConnectorInfoIsNull, std::ptr::null())
    };

    let array = env.new_long_array(2).unwrap();
    env.set_long_array_region(array, 0, &[result as usize as jlong, version as jlong])
        .unwrap();
    array
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeConnectorInfo_nativeGetProtocolPtr(
    env: JNIEnv,
    _class: JClass,
    info: jlong,
) -> jbyteArray {
    let info: *mut CrConnectorInfo = info as _;

    let (result, version) = if !info.is_null() {
        let info = unsafe { &*info };
        (CrResult::Ok, &info.protocol_version as *const _)
    } else {
        (CrResult::ErrParameterConnectorInfoIsNull, std::ptr::null())
    };

    let array = env.new_long_array(2).unwrap();
    env.set_long_array_region(array, 0, &[result as usize as jlong, version as jlong])
        .unwrap();
    array
}
