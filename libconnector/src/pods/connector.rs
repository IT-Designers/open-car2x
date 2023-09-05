use crate::pods::version::CrVersion;
use crate::result::CrResult;
use common_edge::common_message_codec;

include!(concat!(env!("OUT_DIR"), "/cbuilt.rs"));

#[repr(C)]
pub struct CrConnectorInfo {
    pub connector_version: CrVersion,
    pub protocol_version: CrVersion,
}

impl CrConnectorInfo {
    fn get() -> Self {
        CrConnectorInfo {
            connector_version: CrVersion {
                major: VERSION_MAJOR,
                minor: VERSION_MINOR,
                patch: VERSION_PATCH,
                build: VERSION_BUILD,
            },
            protocol_version: CrVersion {
                major: common_message_codec::protocol::LUKAS_CURRENT as u8,
                minor: 0,
                patch: 0,
                build: std::ptr::null(),
            },
        }
    }
}

/// Tries to load a copy of the [`CrConnectorInfo`] into the given target. Will fail
/// (and return `false`) if the given pointer is `NULL`.
///
/// # Safety
///
/// The given target pointer must point to valid and writable memory that is large enough
/// for an instance of [`CrConnectorInfo`] to be written to.
#[no_mangle]
pub unsafe extern "C" fn cr_load_connector_info(target: *mut CrConnectorInfo) -> CrResult {
    if !target.is_null() {
        *target = CrConnectorInfo::get();
        CrResult::Ok
    } else {
        CrResult::ErrParameterConnectionInfoIsNull
    }
}
