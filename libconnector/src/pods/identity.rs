use common_edge::messages;
use messages::itd_data_protocol::Identity;
use num_traits::FromPrimitive;

#[repr(C)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, CStr, FromPrimitive)]
pub enum CrIdentity {
    Vehicle = 100,
    NomadicDevice = 101,
    FusionModule = 200,
    PlanningModule = 201,
    WarningModule = 202,
    // Sensor = 300,
    // SensorLidar = 301,
    // SensorCamera = 302,
}

impl CrIdentity {
    pub fn opt_from_protocol_self(identity: Identity) -> Option<Self> {
        Some(match identity {
            Identity::Vehicle => Self::Vehicle,
            Identity::NomadicDevice => Self::NomadicDevice,
            Identity::FusionAlgo => Self::FusionModule,
            Identity::PlanningModule => Self::PlanningModule,
            Identity::WarningModule => Self::WarningModule,
            _ => return None,
        })
    }

    pub fn into_protocol_self(self) -> Identity {
        match self {
            CrIdentity::Vehicle => Identity::Vehicle,
            CrIdentity::NomadicDevice => Identity::NomadicDevice,
            CrIdentity::FusionModule => Identity::FusionAlgo,
            CrIdentity::PlanningModule => Identity::PlanningModule,
            CrIdentity::WarningModule => Identity::WarningModule,
        }
    }

    pub fn default_address(self) -> &'static str {
        match self {
            CrIdentity::Vehicle => "127.0.0.1:5672",
            CrIdentity::NomadicDevice => "127.0.0.1:5672",
            CrIdentity::FusionModule => "127.0.0.1:5672",
            CrIdentity::PlanningModule => "127.0.0.1:5672",
            CrIdentity::WarningModule => "127.0.0.1:5672",
        }
    }
}

/// Returns the string representation of the given value. The returned pointer is references a
/// null-terminated string. For an invalid variant of [`CrIdentity`] the returned pointer is
/// "\<Invalid>", but never `NULL`.
#[no_mangle]
pub extern "C" fn cr_identity_str(identity: CrIdentity) -> *const std::os::raw::c_char {
    // C can pass an invalid value so treat it as isize value (which it is for C)
    match CrIdentity::from_isize(identity as isize) {
        Some(result) => result.as_cstr().as_ptr(),
        None => cstr::cstr!("<Invalid>").as_ptr(),
    }
}
