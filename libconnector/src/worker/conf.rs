use crate::pods::message::CrMessageIdOverloaded;
use crate::result::CrResult;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::time::Duration;

#[derive(Clone, Default)]
pub struct CrConnectionConfig {
    pub(crate) address: Option<String>,
    pub(crate) reconnect_timeout: Option<Duration>,
    pub(crate) send_timeout: Option<Duration>,
    pub(crate) receive_own_messages: Option<bool>,
    pub(crate) filter_options: Option<CrMessageIdOverloaded>,
    pub(crate) login_user: Option<String>,
    pub(crate) login_password: Option<String>,
    pub(crate) anonymous: Option<bool>,
    pub(crate) target_exchange: Option<String>,
    pub(crate) source_exchange: Option<String>,
    pub(crate) station_id: Option<u32>,
    pub(crate) station_id_receive_filter: Option<u32>,
}

impl CrConnectionConfig {
    pub const fn default_with_opt_address(address: Option<String>) -> Self {
        Self {
            address,
            reconnect_timeout: None,
            send_timeout: None,
            receive_own_messages: None,
            filter_options: None,
            login_user: None,
            login_password: None,
            anonymous: None,
            target_exchange: None,
            source_exchange: None,
            station_id: None,
            station_id_receive_filter: None,
        }
    }
}

/// Sets the target host address for the given [`CrConnectionConfig`]. Passing `NULL` will reset
/// this value to the default address. The address-string must be valid UTF8 and be formatted like
/// `<address>:<port>`.
///
/// # Safety
///
/// The given `config` pointer must point to valid [`CrConnectionConfig`] instance
#[no_mangle]
pub unsafe extern "C" fn cr_config_set_address(
    config: *mut CrConnectionConfig,
    address: *const c_char,
) -> CrResult {
    let config = valid_mut_ptr_or!(config, CrResult::ErrParameterConnectionConfigPointerIsNull);

    if address.is_null() {
        config.address = None;
        CrResult::Ok
    } else if let Ok(address) = CStr::from_ptr(address).to_str() {
        config.address = Some(address.to_string());
        CrResult::Ok
    } else {
        CrResult::ErrParameterAddressIsNotValidUtf8
    }
}

/// Sets the duration in millis between reconnect attempts that is being padded, so that the network
/// is not spammed with connection attempts if the target does not accept the request. Passing `0`
/// will reset this value to the default timeout.
///
/// # Safety
///
/// The given `config` pointer must point to valid [`CrConnectionConfig`] instance
#[no_mangle]
pub unsafe extern "C" fn cr_config_set_reconnect_timeout_millis(
    config: *mut CrConnectionConfig,
    timeout_millis: u64,
) -> CrResult {
    let config = valid_mut_ptr_or!(config, CrResult::ErrParameterConnectionConfigPointerIsNull);

    config.reconnect_timeout = Some(timeout_millis)
        .filter(|millis| *millis > 0)
        .map(Duration::from_millis);

    CrResult::Ok
}

/// Sets the duration in millis at which point a sent message without acknowledgement is considered
/// lost. Passing `0` will reset this value to the default timeout.
///
/// # Safety
///
/// The given `config` pointer must point to valid [`CrConnectionConfig`] instance
#[no_mangle]
pub unsafe extern "C" fn cr_config_set_send_timeout_millis(
    config: *mut CrConnectionConfig,
    timeout_millis: u64,
) -> CrResult {
    let config = valid_mut_ptr_or!(config, CrResult::ErrParameterConnectionConfigPointerIsNull);

    config.send_timeout = Some(timeout_millis)
        .filter(|millis| *millis > 0)
        .map(Duration::from_millis);

    CrResult::Ok
}

/// Sets whether receive filter shall accept messages that were sent by this client.
///
/// # Safety
///
/// The given `config` pointer must point to valid [`CrConnectionConfig`] instance
#[no_mangle]
pub unsafe extern "C" fn cr_config_set_receive_own(
    config: *mut CrConnectionConfig,
    receive_own: bool,
) -> CrResult {
    let config = valid_mut_ptr_or!(config, CrResult::ErrParameterConnectionConfigPointerIsNull);

    config.receive_own_messages = Some(receive_own);

    CrResult::Ok
}

/// Sets the receive filter on the broker to only forward the given
/// [`crate::pods::message::CrMessageId`]s to this client instance. Passing `0` will disable any
/// filter on the broker.
///
/// # Safety
///
/// The given `config` pointer must point to valid [`CrConnectionConfig`] instance
#[no_mangle]
pub unsafe extern "C" fn cr_config_set_receive_filter(
    config: *mut CrConnectionConfig,
    ids: CrMessageIdOverloaded,
) -> CrResult {
    let config = valid_mut_ptr_or!(config, CrResult::ErrParameterConnectionConfigPointerIsNull);

    config.filter_options = Some(ids).filter(|ids| ids.iter_set().count() > 0);

    CrResult::Ok
}

/// Sets the login method to not use a username and password but to try to login without
/// credentials instead. If this value is set to `true`, the username and password fields
/// will be ignored. If this is set to `false`, the given username and password is used or
/// the default values.
///
/// # Safety
///
/// The given `config` pointer must point to valid [`CrConnectionConfig`] instance
#[no_mangle]
pub unsafe extern "C" fn cr_config_set_login_anonymous(
    config: *mut CrConnectionConfig,
    anonymous: bool,
) -> CrResult {
    let config = valid_mut_ptr_or!(config, CrResult::ErrParameterConnectionConfigPointerIsNull);
    config.anonymous = Some(anonymous);
    CrResult::Ok
}

/// Sets the login user name for the broker to the  given value. Passing `NULL` will reset this
/// value to the default user name.
///
/// # Safety
///
/// The given `config` pointer must point to valid [`CrConnectionConfig`] instance
#[no_mangle]
pub unsafe extern "C" fn cr_config_set_login_user(
    config: *mut CrConnectionConfig,
    login_user: *const c_char,
) -> CrResult {
    let config = valid_mut_ptr_or!(config, CrResult::ErrParameterConnectionConfigPointerIsNull);

    match Some(login_user)
        .filter(|user| !user.is_null())
        .map(|user| CStr::from_ptr(user).to_str().map(ToString::to_string))
        .transpose()
    {
        Ok(value) => {
            config.login_user = value;
            CrResult::Ok
        }
        Err(_) => CrResult::ErrParameterIsNotValidUtf8,
    }
}

/// Sets the login password for the broker to the  given value. Passing `NULL` will reset this
/// value to the default password.
///
/// # Safety
///
/// The given `config` pointer must point to valid [`CrConnectionConfig`] instance
#[no_mangle]
pub unsafe extern "C" fn cr_config_set_login_password(
    config: *mut CrConnectionConfig,
    login_password: *const c_char,
) -> CrResult {
    let config = valid_mut_ptr_or!(config, CrResult::ErrParameterConnectionConfigPointerIsNull);

    match Some(login_password)
        .filter(|password| !password.is_null())
        .map(|password| CStr::from_ptr(password).to_str().map(ToString::to_string))
        .transpose()
    {
        Ok(value) => {
            config.login_password = value;
            CrResult::Ok
        }
        Err(_) => CrResult::ErrParameterIsNotValidUtf8,
    }
}

/// Sets the target exchange node name of the broker where to deliver sent messages to.
/// Passing `NULL` will reset this value to the default exchange target.
///
/// # Safety
///
/// The given `config` pointer must point to valid [`CrConnectionConfig`] instance
#[no_mangle]
pub unsafe extern "C" fn cr_config_set_target_exchange(
    config: *mut CrConnectionConfig,
    exchange_target: *const c_char,
) -> CrResult {
    let config = valid_mut_ptr_or!(config, CrResult::ErrParameterConnectionConfigPointerIsNull);

    match Some(exchange_target)
        .filter(|target| !target.is_null())
        .map(|target| CStr::from_ptr(target).to_str().map(ToString::to_string))
        .transpose()
    {
        Ok(value) => {
            config.target_exchange = value;
            CrResult::Ok
        }
        Err(_) => CrResult::ErrParameterIsNotValidUtf8,
    }
}

/// Sets the source exchange node name of the broker where to retrieve messages from.
/// Passing `NULL` will reset this value to the default exchange source.
///
/// # Safety
///
/// The given `config` pointer must point to valid [`CrConnectionConfig`] instance
#[no_mangle]
pub unsafe extern "C" fn cr_config_set_source_exchange(
    config: *mut CrConnectionConfig,
    source_exchange: *const c_char,
) -> CrResult {
    let config = valid_mut_ptr_or!(config, CrResult::ErrParameterConnectionConfigPointerIsNull);

    match Some(source_exchange)
        .filter(|source| !source.is_null())
        .map(|source| CStr::from_ptr(source).to_str().map(ToString::to_string))
        .transpose()
    {
        Ok(value) => {
            config.source_exchange = value;
            CrResult::Ok
        }
        Err(_) => CrResult::ErrParameterIsNotValidUtf8,
    }
}

/// Sets the station-id to mark every outgoing message with. With this set, the broker will be able
/// to apply the station-id filter (see [`cr_config_set_receive_filter`]).
///
/// # Safety
///
/// The given `config` pointer must point to valid [`CrConnectionConfig`] instance
#[no_mangle]
pub unsafe extern "C" fn cr_config_set_station_id(
    config: *mut CrConnectionConfig,
    station_id: u32,
) -> CrResult {
    let config = valid_mut_ptr_or!(config, CrResult::ErrParameterConnectionConfigPointerIsNull);
    config.station_id = Some(station_id);
    CrResult::Ok
}

/// Sets the station-id receive filter to the given value. With this set, the broker will only
/// transmit messages that are explicitly marked with the same station-id. Messages with another
/// station-id will not be received.
///
/// # Safety
///
/// The given `config` pointer must point to valid [`CrConnectionConfig`] instance
#[no_mangle]
pub unsafe extern "C" fn cr_config_set_station_id_receive_filter(
    config: *mut CrConnectionConfig,
    station_id: u32,
) -> CrResult {
    let config = valid_mut_ptr_or!(config, CrResult::ErrParameterConnectionConfigPointerIsNull);
    config.station_id_receive_filter = Some(station_id);
    CrResult::Ok
}

/// Creates a new [`CrConnectionConfig`]-instance with empty presets. Entries that are not
/// updated result in the default value to be used.
///
/// # Safety
///
/// The given `config` pointer must point to writable memory at which the pointer to the
/// [`CrConnectionConfig`]-instance will be written to on success.
#[no_mangle]
pub unsafe extern "C" fn cr_create_config(config: *mut *mut CrConnectionConfig) -> CrResult {
    let config = valid_mut_ptr_or!(config, CrResult::ErrParameterConnectionConfigPointerIsNull);

    if config.is_null() {
        *config = Box::into_raw(Box::default());
        CrResult::Ok
    } else {
        CrResult::ErrParameterConnectionConfigPointerPointerIsNotNull
    }
}

/// Destroys the given [`CrConnectionConfig`]-instance. On success, this operation will set the
/// pointer to the [`CrConnectionConfig`]-instance to `NULL` (`*config = NULL`).
///
/// # Safety
///
/// The given pointer must point to a valid [`CrConnectionConfig`] pointer. This means, the given
/// pointer is not allowed to be `NULL`, and the second level pointer must point to an alive
/// instance (being absolutely not `NULL`), that has not been destroyed already.
#[no_mangle]
pub unsafe extern "C" fn cr_destroy_config(config: *mut *mut CrConnectionConfig) -> CrResult {
    let config = valid_mut_ptr_or!(config, CrResult::ErrParameterConnectionConfigPointerIsNull);

    if config.is_null() {
        CrResult::ErrParameterConnectionConfigPointerPointerIsNull
    } else {
        let mut ptr = core::ptr::null_mut();
        core::mem::swap(&mut ptr, config);
        drop(Box::from_raw(ptr));
        CrResult::Ok
    }
}
