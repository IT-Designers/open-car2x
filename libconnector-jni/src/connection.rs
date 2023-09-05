use connector::connection::ffi::{
    cr_create_connection_with_config, cr_destroy_connection, cr_receive_detailed_message,
    cr_receive_message_by_ref, cr_send_message_by_ref,
};
use connector::connection::CrConnection;
use connector::conversion::{cr_message_convert, CrFormat};
use connector::pods::application::CrApplicationInfo;
use connector::pods::connection::{cr_load_connection_info, CrConnectionInfo};
use connector::pods::message::{
    CrMessage, CrMessageId, CrMessageIdOverloaded, CrMessageRef, CR_MESSAGE_BODY_SIZE_LIMIT,
};
use connector::result::CrResult;
use connector::worker::conf::CrConnectionConfig;
use connector::worker::rcv::CrDetailedMessage;
use jni::objects::{JClass, ReleaseMode};
use jni::sys::{jbyteArray, jlong, jlongArray};
use jni::JNIEnv;
use num_traits::FromPrimitive;
use std::ffi::c_void;
use std::mem::MaybeUninit;

/// This call will consume and free the `CrApplicationInfo` and `CrConnectionConfig`
/// regardless of the outcome.
#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeConnection_nativeCreate(
    env: JNIEnv,
    _class: JClass,
    info: jlong,
    conf: jlong,
) -> jbyteArray {
    let info = info as *mut CrApplicationInfo;
    let conf = conf as *mut CrConnectionConfig;

    let info = if info.is_null() {
        None
    } else {
        Some(Box::from_raw(info))
    };

    let conf = if conf.is_null() {
        None
    } else {
        Some(Box::from_raw(conf))
    };

    let mut connection: *mut CrConnection = std::ptr::null_mut();
    let result = cr_create_connection_with_config(
        &mut connection as _,
        info.as_ref()
            .map(|i| i.as_ref() as *const _)
            .unwrap_or_else(std::ptr::null),
        conf.as_ref()
            .map(|c| c.as_ref() as *const _)
            .unwrap_or_else(std::ptr::null),
    );

    let array = env.new_long_array(2).unwrap();
    env.set_long_array_region(array, 0, &[result as usize as i64, connection as i64])
        .unwrap();
    array
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeConnection_nativeStopAndDelete(
    _env: JNIEnv,
    _class: JClass,
    connection: jlong,
) -> jlong {
    let mut connection = connection as *mut CrConnection;
    cr_destroy_connection(&mut connection as *mut *mut CrConnection) as jlong
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeConnection_nativeCreateConnectionInfo(
    env: JNIEnv,
    _class: JClass,
    connection: jlong,
) -> jlongArray {
    let connection = connection as *const CrConnection;
    let mut info = MaybeUninit::<CrConnectionInfo>::uninit();

    let result = cr_load_connection_info(connection, info.as_mut_ptr());
    let info = if result == CrResult::Ok {
        let info = info.assume_init();
        Box::into_raw(Box::new(info))
    } else {
        std::ptr::null_mut()
    };

    let array = env.new_long_array(2).unwrap();
    env.set_long_array_region(array, 0, &[result as usize as i64, info as i64])
        .unwrap();
    array
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeConnection_nativeReceiveMessageProtobuf(
    env: JNIEnv,
    _class: JClass,
    connection: jlong,
    timeout_millis: jlong,
    mids_overloaded: jlong,
) -> jlongArray {
    let connection = connection as *mut CrConnection;
    let mut filter = CrMessageId::opt_from(mids_overloaded as u64)
        .map(CrMessageIdOverloaded::from)
        .unwrap_or_else(CrMessageIdOverloaded::empty);
    let mut data = std::ptr::null_mut::<CrMessage>();

    extern "C" fn copy_convert_to_protobuf(
        message: &CrMessageRef,
        data: *mut *mut c_void,
    ) -> CrResult {
        let data = unsafe { &mut *(data as *mut *mut CrMessage) as &mut *mut CrMessage };
        let mut protobuf = Box::new(CrMessage {
            id: CrMessageIdOverloaded::from(message.id),
            size: CR_MESSAGE_BODY_SIZE_LIMIT,
            body: [0; CR_MESSAGE_BODY_SIZE_LIMIT],
        });

        let result = unsafe {
            cr_message_convert(
                message.id,
                CrFormat::Uper,
                message.body as *const _,
                message.size,
                CrFormat::Protobuf,
                protobuf.body.as_mut_ptr() as *mut _,
                &mut protobuf.size as *mut _,
            )
        };

        if result == CrResult::Ok {
            *data = Box::into_raw(protobuf);
        }

        result
    }

    let result = cr_receive_message_by_ref(
        connection,
        &mut filter as *mut CrMessageIdOverloaded,
        copy_convert_to_protobuf,
        &mut data as *mut *mut CrMessage as *mut *mut c_void,
        timeout_millis.max(0) as u64,
    );

    let array = env.new_long_array(2).unwrap();
    env.set_long_array_region(array, 0, &[result as jlong, data as jlong])
        .unwrap();
    array
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeConnection_nativeReceiveDetailedMessage(
    env: JNIEnv,
    _class: JClass,
    connection: jlong,
    timeout_millis: jlong,
    mids_overloaded: jlong,
) -> jlongArray {
    let connection = connection as *mut CrConnection;

    let mut detailed_message = CrDetailedMessage {
        content: CrMessageRef {
            id: {
                debug_assert_eq!(
                    core::mem::size_of::<i32>(),
                    core::mem::size_of::<CrMessageId>()
                );
                core::mem::transmute(mids_overloaded as i32)
            },
            size: 0,
            body: core::ptr::null_mut(),
        },
        details: core::ptr::null_mut(),
    };

    let result = cr_receive_detailed_message(
        connection,
        &mut detailed_message as *mut CrDetailedMessage,
        timeout_millis.max(0) as u64,
    );

    let array = env.new_long_array(2).unwrap();
    env.set_long_array_region(
        array,
        0,
        &[
            result as jlong,
            if result == CrResult::Ok {
                Box::into_raw(Box::new(detailed_message))
            } else {
                core::ptr::null_mut()
            } as _,
        ],
    )
    .unwrap();
    array
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_de_itdesigners_lukas_connector_jni_NativeConnection_nativeSendMessage(
    env: JNIEnv,
    _class: JClass,
    connection: jlong,
    mid: jlong,
    format: jlong,
    data: jbyteArray,
) -> jlong {
    let connection = connection as *mut CrConnection;
    let result: Result<(), CrResult> = (move || {
        let mid = CrMessageId::from_i64(mid).ok_or(CrResult::ErrParameterMessageIdIsInvalid)?;
        let format = CrFormat::from_i64(format).ok_or(CrResult::ErrParameterFormatIsInvalid)?;

        let data = env
            .get_byte_array_elements(data, ReleaseMode::NoCopyBack)
            .unwrap();
        let data_size = data.size().unwrap() as usize;

        if CrFormat::Uper != format {
            let mut message = CrMessage {
                id: CrMessageIdOverloaded::from(mid),
                size: CR_MESSAGE_BODY_SIZE_LIMIT,
                body: [0; CR_MESSAGE_BODY_SIZE_LIMIT],
            };

            match cr_message_convert(
                mid,
                format,
                data.as_ptr() as *const _,
                data_size,
                CrFormat::Uper,
                &mut message.body as *mut u8 as *mut _,
                &mut message.size as *mut _,
            ) {
                CrResult::Ok => {}
                result => return Err(result),
            };

            let message_ref = CrMessageRef {
                id: mid,
                size: message.size,
                body: &message.body as *const u8,
            };

            match cr_send_message_by_ref(connection, message_ref) {
                CrResult::Ok => Ok(()),
                result => Err(result),
            }
        } else {
            let message_ref = CrMessageRef {
                id: mid,
                size: data_size,
                body: data.as_ptr() as *const u8,
            };

            match cr_send_message_by_ref(connection, message_ref) {
                CrResult::Ok => Ok(()),
                result => Err(result),
            }
        }
    })();

    result.err().unwrap_or(CrResult::Ok) as jlong
}
