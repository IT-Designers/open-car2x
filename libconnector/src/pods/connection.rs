use crate::connection::CrConnection;
use crate::pods::message::CrMessageIdOverloaded;
use crate::result::CrResult;
use std::os::raw::c_char;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CrConnectionInfo {
    pub status: CrConnectionStatus,
    pub times_connected_counter: u16,
    pub connection_epoch_millis_timestamp: u64,
    pub message_receiver_queue_size: u16,
    pub message_receiver_queue_types: CrMessageIdOverloaded,
    /// This value might be outdated up to 1s.
    pub message_sender_queue_size: u16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq, CStr)]
pub enum CrConnectionStatus {
    Connected = 0,
    Disconnected,
    Initializing,
    Connecting,
    OpeningSession,
    OpeningSender,
    OpeningReceiver,
}

/// Returns the string representation of the given value. The returned pointer is references a
/// null-terminated byte string and is never `NULL`.
#[no_mangle]
pub extern "C" fn cr_connection_status_str(status: CrConnectionStatus) -> *const c_char {
    status.as_cstr().as_ptr()
}

/// Tries to load a copy of the current [`CrConnectionInfo`] into the given target. Will fail
/// (and return `false`) if either of the pointers is `NULL` or the lock to the internal struct
/// could not be acquired.
///
/// # Safety
///
/// The given `target` pointer must point to writable memory that is large enough for a
/// [`CrConnectionInfo`] to be written to.
#[no_mangle]
pub unsafe extern "C" fn cr_load_connection_info(
    connection: *const CrConnection,
    target: *mut CrConnectionInfo,
) -> CrResult {
    let connection = valid_ptr_or!(connection, CrResult::ErrParameterConnectionPointerIsNull);
    let target = valid_mut_ptr_or!(target, CrResult::ErrParameterConnectionInfoIsNull);

    match connection.load_status_info() {
        Some(info) => {
            *target = info;
            CrResult::Ok
        }
        None => CrResult::ErrConnectionStatusUnavailable,
    }
}
