use connector::pods::application::CrApplicationInfo;
use connector::pods::identity::CrIdentity;
use connector::pods::version::CrVersion;
use jni::objects::{JClass, JString};
use jni::sys::{jbyte, jint, jlong};
use jni::JNIEnv;
use num_traits::cast::FromPrimitive;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeApplicationInfo_create(
    env: JNIEnv,
    _class: JClass,
    identity: jint,
    versionMajor: jbyte,
    versionMinor: jbyte,
    versionPatch: jbyte,
    versionBuild: JString,
    name: JString,
) -> jlong {
    let build = cstring_or_utf8_error!(env, versionBuild);
    let name = cstring_or_utf8_error!(env, name);

    Box::into_raw(Box::new(CrApplicationInfo {
        identity: CrIdentity::from_i32(identity).unwrap(),
        version: CrVersion {
            major: versionMajor as u8,
            minor: versionMinor as u8,
            patch: versionPatch as u8,
            build: build.as_ptr(),
        },
        name: name.as_ptr(),
    })) as jlong
}
