use crate::connection::CrConnection;
use crate::pods::application::CrApplicationInfo;
#[cfg(doc)]
use crate::pods::message::CrMessageId;
use crate::pods::message::{
    CrMessage, CrMessageIdOverloaded, CrMessageRef, CR_MESSAGE_BODY_SIZE_LIMIT,
};
use crate::result::CrResult;
use crate::worker::conf::CrConnectionConfig;
use crate::worker::rcv::CrDetailedMessage;
use common_edge::messages::itd_data_protocol;
use log::error;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::os::raw::c_void;
use std::panic::catch_unwind;
use std::time::Duration;

/// Creates a new [`CrConnection`]-instance for the given [`CrApplicationInfo`]. If specified
/// (meaning not NULL), the given address-string is used to establish the connection. If not
/// specified, a suitable default address will be used. The address-string must be valid UTF8
/// and be formatted like `<address>:<port>`.
///
/// # Safety
///
/// The given `connection` pointer must point to writable memory at which a pointer can be stored.
#[no_mangle]
pub unsafe extern "C" fn cr_create_connection(
    connection: *mut *mut CrConnection,
    application_info: *const CrApplicationInfo,
    address: *const c_char,
) -> CrResult {
    let connection = valid_mut_ptr_or!(connection, CrResult::ErrParameterConnectionPointerIsNull);
    let application_info = valid_ptr_or!(
        application_info,
        CrResult::ErrParameterApplicationInfoIsNull
    );

    if !(*connection).is_null() {
        return CrResult::ErrParameterConnectionPointerPointerIsNotNull;
    }

    let address = if address.is_null() {
        None
    } else if let Ok(str) = CStr::from_ptr(address).to_str() {
        Some(str.to_string())
    } else {
        return CrResult::ErrParameterAddressIsNotValidUtf8;
    };

    create_connection(
        connection,
        application_info,
        CrConnectionConfig::default_with_opt_address(address),
    )
}

/// Creates a new [`CrConnection`]-instance for the given [`CrApplicationInfo`]. If specified
/// (meaning not NULL), the given [`CrConnectionConfig`] is used to establish the connection. If not
/// specified, suitable default values are used.
///
/// # Safety
///
/// The given `connection` pointer must point to writable memory at which a pointer can be stored.
#[no_mangle]
pub unsafe extern "C" fn cr_create_connection_with_config(
    connection: *mut *mut CrConnection,
    application_info: *const CrApplicationInfo,
    config: *const CrConnectionConfig,
) -> CrResult {
    let connection = valid_mut_ptr_or!(connection, CrResult::ErrParameterConnectionPointerIsNull);
    let application_info = valid_ptr_or!(
        application_info,
        CrResult::ErrParameterApplicationInfoIsNull
    );

    if !(*connection).is_null() {
        return CrResult::ErrParameterConnectionPointerPointerIsNotNull;
    }

    let config = if config.is_null() {
        CrConnectionConfig::default()
    } else {
        let config = &*config;
        config.clone()
    };

    create_connection(connection, application_info, config)
}

fn create_connection(
    connection: &mut *mut CrConnection,
    application_info: &CrApplicationInfo,
    config: CrConnectionConfig,
) -> CrResult {
    let additional = {
        let mut additional = String::default();
        if !application_info.name.is_null() {
            if let Ok(name) = unsafe { CStr::from_ptr(application_info.name) }.to_str() {
                additional += "name=";
                additional += name;
            } else {
                return CrResult::ErrParameterApplicationNameIsNotValidUtf8;
            }
        }
        additional
    };

    let boxed_connection = catch_unwind(|| {
        Box::new(CrConnection::new(
            config,
            itd_data_protocol::ApplicationInfo {
                identity: application_info.identity.into_protocol_self(),
                version: application_info.version.into_protocol_self(),
                build: common_build_info::info::load(),
                args: std::env::args().collect(),
                additional: {
                    let mut additional = additional;
                    additional.push_str(",libconnector_version=");
                    additional.push_str(env!("CARGO_PKG_VERSION"));
                    additional
                },
            },
        ))
    });

    match boxed_connection {
        Ok(boxed_connection) => {
            *connection = Box::into_raw(boxed_connection);
            CrResult::Ok
        }
        Err(e) => {
            eprintln!("{e:?}");
            error!("{e:?}");
            CrResult::ErrWorkerPanic
        }
    }
}

/// Destroys the given [`CrConnection`] instance. This means the connection will be aborted
/// associated memory freed and all pending operations ignored. This operation will set the
/// pointer to the [`CrConnection`] to `NULL` (`*connection = NULL`) after the connection was
/// destroyed successfully.
///
/// # Safety
///
/// The given pointer must point to a valid [`CrConnection`] pointer. This means, the given pointer
/// is not allowed to be `NULL`, and the second level pointer must point to an alive instance
/// (being absolutely not `NULL`), that has not been destroyed already.
#[no_mangle]
pub unsafe extern "C" fn cr_destroy_connection(connection: *mut *mut CrConnection) -> CrResult {
    if connection.is_null() {
        CrResult::ErrParameterConnectionPointerIsNull
    } else if (*connection).is_null() {
        CrResult::ErrParameterConnectionPointerPointerIsNull
    } else {
        Box::from_raw(std::mem::replace(&mut *connection, std::ptr::null_mut()))
            .stop()
            .map(|_| CrResult::Ok)
            .unwrap_or(CrResult::ErrWorkerDetached)
    }
}

/// Tries to receive the next message that matches the id-filter and within the given time.
/// The received message is stored at the `target` location. The `id` of the [`CrMessage`]
/// at the target location is used as filter - [`CrMessageId`] overloading is supported.
/// If no filter is specified (`id` being zero), the next available message of any id is loaded.
///
/// A zero `timeout_ms` value will perform a poll that will immediately return either a message
/// or [`CrResult::ErrWorkerRequestTimeoutReached`].
///
/// # Safety
///
/// The `connection` pointer must point to a valid - thus alive and not destroyed - [`CrConnection`]
/// instance.
/// The `target` pointer must point to a pre-allocated [`CrMessage`] of which the [`CrMessage#id`]
/// field must be zero, a valid [`CrMessageId`] variant or being overloaded with valid
/// [`CrMessageId`] variants.
#[no_mangle]
pub unsafe extern "C" fn cr_receive_message(
    connection: *mut CrConnection,
    target: *mut CrMessage,
    timeout_ms: u64,
) -> CrResult {
    let connection = valid_ptr_or!(connection, CrResult::ErrParameterConnectionPointerIsNull);
    let target = valid_mut_ptr_or!(target, CrResult::ErrParameterMessageIsNull);

    match connection.receive_message_into(
        target.id.separate(),
        Duration::from_millis(timeout_ms),
        &mut target.body[..],
    ) {
        Ok((mid, len)) => {
            target.id = mid.into();
            target.size = len;
            CrResult::Ok
        }
        Err(false) => CrResult::ErrWorkerRequestDropped,
        Err(true) => CrResult::ErrWorkerRequestTimeoutReached,
    }
}

/// Tries to receive the next message that matches the id-filter and within the given time.
/// The received message is stored at the `target` location. The `id` of the [`CrMessage`]
/// at the target location is used as filter - [`CrMessageId`] overloading is supported.
/// If no filter is specified (`id` being zero), the next available message of any id is loaded.
///
/// A zero `timeout_ms` value will perform a poll that will immediately return either a message
/// or [`CrResult::ErrWorkerRequestTimeoutReached`].
///
/// # Safety
///
/// The `connection` pointer must point to a valid - thus alive and not destroyed - [`CrConnection`]
/// instance.
/// The `target` pointer must point to a pre-allocated [`CrMessage`] of which the [`CrMessage#id`]
/// field must be zero, a valid [`CrMessageId`] variant or being overloaded with valid
/// [`CrMessageId`] variants.
#[no_mangle]
pub unsafe extern "C" fn cr_receive_detailed_message(
    connection: *mut CrConnection,
    target: *mut CrDetailedMessage,
    timeout_ms: u64,
) -> CrResult {
    let connection = valid_ptr_or!(connection, CrResult::ErrParameterConnectionPointerIsNull);
    let target = valid_mut_ptr_or!(target, CrResult::ErrParameterMessageIsNull);

    match connection.receive_message(target.id().separate(), Duration::from_millis(timeout_ms)) {
        Ok((mid, received_message)) => {
            *target = CrDetailedMessage::from(mid, received_message);
            CrResult::Ok
        }
        Err(false) => CrResult::ErrWorkerRequestDropped,
        Err(true) => CrResult::ErrWorkerRequestTimeoutReached,
    }
}

/// Tries to receive the next message that matches the given [`CrMessageId`]-filter - overloading
/// is supported. If no filter is specified (`id` being zero), the next available message of any id
/// is loaded.
///
/// In comparison to [`cr_receive_message`], this function does not write a copy of the message in
/// a given buffer but instead allows to operate on the original buffer directly via callback. The
/// [`CrMessageId`] that was received is stored at the destination of the given `filter` pointer.
///
/// The callback is passed the `dst` pointer without checks or modifications. The returned result
/// is either an error that occurred while fetching the message or (on success) the result returned
/// by the callback.
///
/// A zero `timeout_ms` value will perform a poll that will immediately return either the message
/// or [`CrResult::ErrWorkerRequestTimeoutReached`].
///
/// # Safety
///
/// - The `connection` pointer must point to a valid - thus alive and not destroyed -
///   [`CrConnection`] instance.
/// - The `filter` must point to a writable memory location
/// - The `callback` must be a valid function pointer
/// - Accessing memory outside the message-body is undefined behavior
#[no_mangle]
pub unsafe extern "C" fn cr_receive_message_by_ref(
    connection: *mut CrConnection,
    filter: *mut CrMessageIdOverloaded,
    callback: extern "C" fn(&CrMessageRef, dst: *mut *mut c_void) -> CrResult,
    dst: *mut *mut c_void,
    timeout_ms: u64,
) -> CrResult {
    let connection = valid_ptr_or!(connection, CrResult::ErrParameterConnectionPointerIsNull);
    let filter = valid_mut_ptr_or!(filter, CrResult::ErrParameterMessageIdIsNull);

    match connection.receive_message(filter.separate(), Duration::from_millis(timeout_ms)) {
        Ok((mid, message)) => {
            *filter = mid.into();
            let bytes = message.into_body();
            callback(
                &CrMessageRef {
                    id: mid,
                    size: bytes.len(),
                    body: bytes[..].as_ptr(),
                },
                dst,
            )
        }
        Err(false) => CrResult::ErrWorkerRequestDropped,
        Err(true) => CrResult::ErrWorkerRequestTimeoutReached,
    }
}

/// Send the given message through the given connection.
///
/// # Safety
///
/// - The pointers must point to valid instances and non-destroyed instances.
/// - The [`CrMessageId`] of the given [`CrMessage`] is not allowed to be overloaded.
#[no_mangle]
pub unsafe extern "C" fn cr_send_message(
    connection: *mut CrConnection,
    message: *const CrMessage,
) -> CrResult {
    let connection = valid_ptr_or!(connection, CrResult::ErrParameterConnectionPointerIsNull);
    let message = valid_ptr_or!(message, CrResult::ErrParameterMessageIsNull);

    if message.size > message.body.len() {
        CrResult::ErrParameterMessageSizeExceedsLimit
    } else if let Some(message_id) = message.id.into_single() {
        connection
            .send_message(message_id, message.body[..message.size].to_vec())
            .map(|_| CrResult::Ok)
            .unwrap_or(CrResult::ErrWorkerDetached)
    } else {
        CrResult::ErrParameterMessageIdIsInvalid
    }
}

/// Send the given message through the given connection.
///
/// # Safety
///
/// - The pointers must point to valid and non-destroyed instances.
/// - The [`CrMessageId`] of the given [`CrMessageRef`] is not allowed to be overloaded.
#[no_mangle]
pub unsafe extern "C" fn cr_send_message_by_ref(
    connection: *mut CrConnection,
    message: CrMessageRef,
) -> CrResult {
    let connection = valid_ptr_or!(connection, CrResult::ErrParameterConnectionPointerIsNull);

    if message.body.is_null() {
        CrResult::ErrParameterMessageBodyIsNull
    } else if message.size > CR_MESSAGE_BODY_SIZE_LIMIT {
        CrResult::ErrParameterMessageSizeExceedsLimit
    } else if let Some(message_id) = CrMessageIdOverloaded::from(message.id).into_single() {
        connection
            .send_message(
                message_id,
                core::slice::from_raw_parts(message.body, message.size).to_vec(),
            )
            .map(|_| CrResult::Ok)
            .unwrap_or(CrResult::ErrWorkerDetached)
    } else {
        CrResult::ErrParameterMessageIdIsInvalid
    }
}
