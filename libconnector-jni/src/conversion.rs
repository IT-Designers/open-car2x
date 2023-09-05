use connector::conversion::{cr_message_convert, CrFormat};
use connector::pods::message::{CrMessageId, CrMessageRef};
use connector::result::CrResult;
use jni::objects::{JClass, ReleaseMode};
use jni::sys::{jbyteArray, jint, jlong, jlongArray};
use jni::JNIEnv;
use num_traits::FromPrimitive;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeConverter_nativeConvert(
    env: JNIEnv,
    _class: JClass,
    mid: jlong,
    src_type: jint,
    src: jbyteArray,
    dst_type: jint,
    dst: jbyteArray,
) -> jlongArray {
    let src = env
        .get_byte_array_elements(src, ReleaseMode::NoCopyBack)
        .unwrap();
    let src_len = src.size().unwrap() as usize;
    let src = src.as_ptr();

    let dst = env
        .get_byte_array_elements(dst, ReleaseMode::CopyBack)
        .unwrap();

    let mut dst_len = dst.size().unwrap() as usize;
    let dst = dst.as_ptr();

    let result = unsafe {
        cr_message_convert(
            CrMessageId::from_i64(mid).unwrap(),
            CrFormat::from_i32(src_type).unwrap(),
            src as *const _,
            src_len,
            CrFormat::from_i32(dst_type).unwrap(),
            dst as *mut _,
            (&mut dst_len) as *mut _,
        )
    };

    let array = env.new_long_array(2).unwrap();
    env.set_long_array_region(array, 0, &[result as usize as jlong, dst_len as jlong])
        .unwrap();
    array
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeConverter_nativeConvertFromMessageRef(
    env: JNIEnv,
    _class: JClass,
    message_ref: jlong,
    message_format: jint,
    dst_type: jint,
    dst: jbyteArray,
) -> jlongArray {
    let message_ref = message_ref as *const CrMessageRef;
    let message_ref = if message_ref.is_null() {
        let array = env.new_long_array(2).unwrap();
        env.set_long_array_region(
            array,
            0,
            &[CrResult::ErrParameterMessageIsNull as usize as jlong, 0],
        )
        .unwrap();
        return array;
    } else {
        &*message_ref
    };

    let dst = env
        .get_byte_array_elements(dst, ReleaseMode::CopyBack)
        .unwrap();

    let mut dst_len = dst.size().unwrap() as usize;
    let dst = dst.as_ptr();

    let result = cr_message_convert(
        message_ref.id,
        CrFormat::from_i32(message_format).unwrap(),
        message_ref.body as *const _,
        message_ref.size,
        CrFormat::from_i32(dst_type).unwrap(),
        dst as *mut _,
        (&mut dst_len) as *mut _,
    );

    let array = env.new_long_array(2).unwrap();
    env.set_long_array_region(array, 0, &[result as usize as jlong, dst_len as jlong])
        .unwrap();
    array
}
