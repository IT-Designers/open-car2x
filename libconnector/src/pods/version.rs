use crate::result::CrResult;
use crate::util::display_to_extern_char_buffer;
use common_edge::messages;
use messages::itd_data_protocol;
use std::ffi::CStr;
use std::fmt::{Display, Formatter};
use std::os::raw::c_char;

#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct CrVersion {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
    pub build: *const c_char,
}

impl Display for CrVersion {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "v{}.{}.{}", self.major, self.minor, self.patch)?;
        if !self.build.is_null() {
            if let Ok(build) = unsafe { CStr::from_ptr(self.build) }.to_str() {
                return write!(f, "+{}", build);
            }
        }
        Ok(())
    }
}

impl CrVersion {
    pub fn into_protocol_self(self) -> itd_data_protocol::Version {
        itd_data_protocol::Version {
            major: self.major,
            minor: self.minor,
            patch: self.patch,
        }
    }
}

/// Writes the string representation of the version to the given buffer. The string
/// representation will be truncated so that it and the zero byte `\0` fit into it - if
/// the buffer length is greater zero.
/// Also, if the given buffer is invalid (`NULL`) the call will be ignored.
///
/// # Safety
///
/// The given `target_buffer` pointer must point to a writable memory location that is at least
/// `buffer_len`-bytes large.
#[no_mangle]
pub unsafe extern "C" fn cr_version_to_string(
    target_buffer: *mut c_char,
    buffer_len: usize,
    version: CrVersion,
) -> CrResult {
    display_to_extern_char_buffer(target_buffer, buffer_len, version)
}
