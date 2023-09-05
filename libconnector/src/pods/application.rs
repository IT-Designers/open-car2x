use crate::pods::identity::CrIdentity;
use crate::pods::version::CrVersion;
use std::os::raw::c_char;

#[repr(C)]
#[derive(Debug)]
pub struct CrApplicationInfo {
    pub identity: CrIdentity,
    pub version: CrVersion,
    pub name: *const c_char,
}
