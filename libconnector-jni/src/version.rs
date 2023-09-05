use connector::pods::version::CrVersion;
use connector::result::CrResult;
use jni::objects::JClass;
use jni::sys::{jbyteArray, jlong, jstring};
use jni::JNIEnv;
use std::ffi::CStr;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeVersion_nativeGetMajorMinorPatch(
    env: JNIEnv,
    _class: JClass,
    version: jlong,
) -> jbyteArray {
    let version: *mut CrVersion = version as _;

    let (result, version) = if !version.is_null() {
        let version = unsafe { &*version };
        (CrResult::Ok, [version.major, version.minor, version.patch])
    } else {
        (CrResult::ErrParameterConnectorInfoIsNull, [0, 0, 0])
    };

    let array = env.new_long_array(4).unwrap();
    env.set_long_array_region(
        array,
        0,
        &[
            result as usize as jlong,
            version[0] as jlong,
            version[1] as jlong,
            version[2] as jlong,
        ],
    )
    .unwrap();
    array
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeVersion_nativeGetBuildOrNull(
    env: JNIEnv,
    _class: JClass,
    version: jlong,
) -> jstring {
    let version: *mut CrVersion = version as _;

    if !version.is_null() {
        let version = unsafe { &*version };
        if version.build.is_null() {
            std::ptr::null_mut()
        } else {
            let string = unsafe { CStr::from_ptr(version.build) }.to_string_lossy();
            env.new_string(&*string).unwrap().into_inner()
        }
    } else {
        std::ptr::null_mut()
    }
}
