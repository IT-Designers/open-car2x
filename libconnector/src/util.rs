use crate::result::CrResult;
use std::fmt::{Debug, Display};
use std::io::Write;
use std::os::raw::c_char;
use std::time::UNIX_EPOCH;

/// # Safety
///
/// The given `target_buffer` pointer must point to a writable memory location that is at least
/// `buffer_len`-bytes large.
#[allow(unused)]
pub(crate) unsafe fn debug_to_extern_char_buffer<T: Debug>(
    target_buffer: *mut c_char,
    buffer_len: usize,
    value: T,
) -> CrResult {
    if target_buffer.is_null() {
        CrResult::ErrParameterCharTargetBufferIsNull
    } else if buffer_len > 0 {
        let target = std::slice::from_raw_parts_mut(target_buffer as *mut u8, buffer_len);
        if write!(&mut target[..buffer_len - 1], "{:?}\x00", value).is_err() {
            // truncated, ensure proper c-style string
            target[buffer_len - 1] = 0x00;
        }
        CrResult::Ok
    } else {
        // nothing to do
        CrResult::Ok
    }
}

/// # Safety
///
/// The given `target_buffer` pointer must point to a writable memory location that is at least
/// `buffer_len`-bytes large.
pub(crate) unsafe fn display_to_extern_char_buffer<T: Display>(
    target_buffer: *mut c_char,
    buffer_len: usize,
    value: T,
) -> CrResult {
    if target_buffer.is_null() {
        CrResult::ErrParameterCharTargetBufferIsNull
    } else if buffer_len > 0 {
        let target = std::slice::from_raw_parts_mut(target_buffer as *mut u8, buffer_len);
        if write!(&mut target[..buffer_len - 1], "{}\x00", value).is_err() {
            // truncated, ensure proper c-style string
            target[buffer_len - 1] = 0x00;
        }
        CrResult::Ok
    } else {
        // nothing to do
        CrResult::Ok
    }
}

/// Sets this thread to sleep for the given amount of millis
#[no_mangle]
pub extern "C" fn cr_util_sleep_millis(millis: u64) {
    std::thread::sleep(std::time::Duration::from_millis(millis));
}

/// Returns the milliseconds since UNIX_EPOCH time (1st Jan, 1970)
#[no_mangle]
pub extern "C" fn cr_util_unix_epoch_time_millis() -> u64 {
    UNIX_EPOCH.elapsed().unwrap_or_default().as_millis() as u64
}
