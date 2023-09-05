/// ```
/// assert_eq!(common_message::id::OFFSET_LUKAS, 2048);
/// assert_eq!(common_message::id::OFFSET_LUKAS, 0b0000_1000_0000_0000);
/// ```
pub const OFFSET_LUKAS: u16 = 0x08_00;

/// ```
/// assert_eq!(common_message::id::OFFSET_LUKAS_DEBUG, 3072);
/// assert_eq!(common_message::id::OFFSET_LUKAS_DEBUG, 0b0000_1100_0000_0000);
/// ```
pub const OFFSET_LUKAS_DEBUG: u16 = 0x0C_00;

/// ```
/// assert_eq!(common_message::id::OFFSET_ITD, 32768);
/// assert_eq!(common_message::id::OFFSET_ITD, 0b1000_0000_0000_0000);
/// ```
pub const OFFSET_ITD: u16 = 0x80_00;

/// ```
/// assert_eq!(common_message::id::OFFSET_ITD_COM, 32768);
/// assert_eq!(common_message::id::OFFSET_ITD_COM, 0b1000_0000_0000_0000);
/// ```
pub const OFFSET_ITD_COM: u16 = OFFSET_ITD;

/// ```
/// assert_eq!(common_message::id::OFFSET_ITD_WEB, 36864);
/// assert_eq!(common_message::id::OFFSET_ITD_WEB, 0b1001_0000_0000_0000);
/// ```
pub const OFFSET_ITD_WEB: u16 = OFFSET_ITD | 0x10_00;

/// ```
/// assert_eq!(common_message::id::OFFSET_ITD_FUSION, 40960);
/// assert_eq!(common_message::id::OFFSET_ITD_FUSION, 0b1010_0000_0000_0000);
/// ```
pub const OFFSET_ITD_FUSION: u16 = OFFSET_ITD | 0x20_00;

pub mod public {
    pub const CLIENT_REGISTRATION: u16 = 1;
    pub const SENSOR_FRAME: u16 = 2;
    pub const ENVIRONMENT_FRAME: u16 = 3;
    pub const UPDATE_SUBSCRIPTION: u16 = 4;
    pub const INIT_MESSAGE: u16 = 5;
    pub const ROAD_CLEARANCE_FRAME: u16 = 6;
    pub const SENSOR_IDLE_FRAME: u16 = 7;
    // pub const DATA_MESSAGE: u16 = 8;
    // pub const ERROR: u16 = 9;
    // pub const STATUS: u16 = 10;
    // pub const CLIENT_MESSAGE: u16 = 11;
    // pub const RECORDING: u16 = 12;
}

pub mod lukas {
    pub const DENM: u16 = super::OFFSET_LUKAS;
    pub const CPM: u16 = super::OFFSET_LUKAS + 1;
    pub const CAM: u16 = super::OFFSET_LUKAS + 2;
    pub const VAM: u16 = super::OFFSET_LUKAS + 3;
    pub const MCM: u16 = super::OFFSET_LUKAS + 4;
}

pub mod lukas_debug {
    pub const DEBUG_REQUEST: u16 = super::OFFSET_LUKAS_DEBUG;
    pub const COMPONENT_STATUS: u16 = super::OFFSET_LUKAS_DEBUG + 1;
}

#[deprecated(note = "Don't use legacy ids")]
pub mod legacy {
    #[deprecated(note = "Use `itd::com::DATA_MESSAGE` instead")]
    pub const DATA_MESSAGE: u16 = 8;
    #[deprecated]
    pub const ERROR: u16 = 9;
    #[deprecated(note = "Use `itd::web::STATUS` instead")]
    pub const STATUS: u16 = 10;
    #[deprecated(note = "Use `itd::web::CLIENT_MESSAGE` instead")]
    pub const CLIENT_MESSAGE: u16 = 11;
    #[deprecated(note = "Use `itd::com::RECORDING` instead")]
    pub const RECORDING: u16 = 12;
}

pub mod itd {
    pub const OFFSET: u16 = super::OFFSET_ITD;
    pub const OFFSET_COM: u16 = super::OFFSET_ITD_COM;
    pub const OFFSET_WEB: u16 = super::OFFSET_ITD_WEB;
    pub const OFFSET_FUSION: u16 = super::OFFSET_ITD_FUSION;

    /// Ids of messages used for communication between
    /// ITD components (like dataserver and server)
    pub mod com {
        pub const OFFSET: u16 = super::OFFSET_COM;

        #[deprecated(note = "Use `itd::com::TRACKING_INFORMATION` instead")]
        pub const DATA_MESSAGE: u16 = OFFSET;
        #[deprecated(note = "Use `itd::com::NewRecord` instead")]
        pub const RECORD: u16 = OFFSET + 1;
        #[deprecated(note = "Use `itd::com::TRACKING_INFORMATION` instead")]
        pub const DATA_SUPPLEMENT: u16 = OFFSET + 2;
        #[deprecated(note = "Use `itd::com::TRACKING_INFORMATION` instead")]
        pub const MALFORMED_MESSAGE: u16 = OFFSET + 3;
        pub const TRACKING_INFORMATION: u16 = OFFSET + 4;
        pub const TRACKING_EVENT: u16 = OFFSET + 5;
        pub const TRACKED_RECORD: u16 = OFFSET + 6;
        pub const SYSTEM_MESSAGE: u16 = OFFSET + 7;
        pub const APPLICATION_INFO: u16 = OFFSET + 8;
        pub const LOG_RECORD: u16 = OFFSET + 9;
        pub const UTILIZATION_REPORT: u16 = OFFSET + 10;
        pub const PING: u16 = OFFSET + 11;
        pub const PONG: u16 = OFFSET + 12;
        pub const SHUTDOWN_REQUEST: u16 = OFFSET + 13;
    }

    /// Ids of messages used to communicate with
    /// web components (like the visualisation)
    pub mod web {
        pub const OFFSET: u16 = super::OFFSET_WEB;

        pub const STATUS: u16 = OFFSET;
        pub const CLIENT_MESSAGE: u16 = OFFSET + 1;
    }

    /// Ids of messages used to collect additional
    /// statistic information
    pub mod fusion {
        pub const OFFSET: u16 = super::OFFSET_FUSION;

        pub const SENSOR_DEREGISTRATION: u16 = OFFSET;
        pub const FUSION_INFORMATION: u16 = OFFSET + 1;
        pub const UULM_SPU_BSON: u16 = OFFSET + 2;
    }
}
