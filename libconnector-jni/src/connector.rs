use jni::objects::JClass;
use jni::JNIEnv;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeConnector_nativeInit(
    _env: JNIEnv,
    _class: JClass,
) {
    #[cfg(target_os = "android")]
    {
        crate::android::setup_stdio_redirection();
        println!("stdio redirection has been setup");
        eprintln!("stdio redirection has been setup");
    }
}
