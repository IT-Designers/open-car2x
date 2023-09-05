use asn1rs::prelude::*;

#[asn(enumerated)]

#[derive(Debug, Clone, PartialEq, Hash, Copy, PartialOrd, Eq, Serialize, Deserialize, Default)]
pub enum Identity {
    #[default] Client,
    Server,
    DataServer,
    Persistence,
    FusionAlgo,
    RoadClearanceModule,
    FusionAlgoTimeoutDetector,
    EdgeApplication,
    Vehicle,
    NomadicDevice,
    PlanningModule,
    WarningModule,
    Reserved,
}

impl Identity {
    pub fn variant(index: usize) -> Option<Self> {
        match index {
            0 => Some(Identity::Client),
            1 => Some(Identity::Server),
            2 => Some(Identity::DataServer),
            3 => Some(Identity::Persistence),
            4 => Some(Identity::FusionAlgo),
            5 => Some(Identity::RoadClearanceModule),
            6 => Some(Identity::FusionAlgoTimeoutDetector),
            7 => Some(Identity::EdgeApplication),
            8 => Some(Identity::Vehicle),
            9 => Some(Identity::NomadicDevice),
            10 => Some(Identity::PlanningModule),
            11 => Some(Identity::WarningModule),
            12 => Some(Identity::Reserved),
            _ => None,
        }
    }

    pub const fn variants() -> [Self; 13] {
        [
        Identity::Client,
        Identity::Server,
        Identity::DataServer,
        Identity::Persistence,
        Identity::FusionAlgo,
        Identity::RoadClearanceModule,
        Identity::FusionAlgoTimeoutDetector,
        Identity::EdgeApplication,
        Identity::Vehicle,
        Identity::NomadicDevice,
        Identity::PlanningModule,
        Identity::WarningModule,
        Identity::Reserved,
        ]
    }

    pub fn value_index(self) -> usize {
        match self {
            Identity::Client => 0,
            Identity::Server => 1,
            Identity::DataServer => 2,
            Identity::Persistence => 3,
            Identity::FusionAlgo => 4,
            Identity::RoadClearanceModule => 5,
            Identity::FusionAlgoTimeoutDetector => 6,
            Identity::EdgeApplication => 7,
            Identity::Vehicle => 8,
            Identity::NomadicDevice => 9,
            Identity::PlanningModule => 10,
            Identity::WarningModule => 11,
            Identity::Reserved => 12,
        }
    }
}

#[asn(enumerated)]

#[derive(Debug, Clone, PartialEq, Hash, Copy, PartialOrd, Eq, Serialize, Deserialize, Default)]
pub enum MessageMode {
    #[default] Received,
    Sent,
    Connect,
    Disconnect,
}

impl MessageMode {
    pub fn variant(index: usize) -> Option<Self> {
        match index {
            0 => Some(MessageMode::Received),
            1 => Some(MessageMode::Sent),
            2 => Some(MessageMode::Connect),
            3 => Some(MessageMode::Disconnect),
            _ => None,
        }
    }

    pub const fn variants() -> [Self; 4] {
        [
        MessageMode::Received,
        MessageMode::Sent,
        MessageMode::Connect,
        MessageMode::Disconnect,
        ]
    }

    pub fn value_index(self) -> usize {
        match self {
            MessageMode::Received => 0,
            MessageMode::Sent => 1,
            MessageMode::Connect => 2,
            MessageMode::Disconnect => 3,
        }
    }
}

#[asn(enumerated)]

#[derive(Debug, Clone, PartialEq, Hash, Copy, PartialOrd, Eq, Serialize, Deserialize, Default)]
pub enum Action {
    #[default] Received,
    Sent,
    Connect,
    Disconnect,
    DecodingFailed,
    Created,
    DecodingSucceeded,
    EncodingFailed,
    EncodingSucceeded,
}

impl Action {
    pub fn variant(index: usize) -> Option<Self> {
        match index {
            0 => Some(Action::Received),
            1 => Some(Action::Sent),
            2 => Some(Action::Connect),
            3 => Some(Action::Disconnect),
            4 => Some(Action::DecodingFailed),
            5 => Some(Action::Created),
            6 => Some(Action::DecodingSucceeded),
            7 => Some(Action::EncodingFailed),
            8 => Some(Action::EncodingSucceeded),
            _ => None,
        }
    }

    pub const fn variants() -> [Self; 9] {
        [
        Action::Received,
        Action::Sent,
        Action::Connect,
        Action::Disconnect,
        Action::DecodingFailed,
        Action::Created,
        Action::DecodingSucceeded,
        Action::EncodingFailed,
        Action::EncodingSucceeded,
        ]
    }

    pub fn value_index(self) -> usize {
        match self {
            Action::Received => 0,
            Action::Sent => 1,
            Action::Connect => 2,
            Action::Disconnect => 3,
            Action::DecodingFailed => 4,
            Action::Created => 5,
            Action::DecodingSucceeded => 6,
            Action::EncodingFailed => 7,
            Action::EncodingSucceeded => 8,
        }
    }
}

#[asn(sequence)]

#[derive(Default, Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct TrackingEvent {
    #[asn(integer(min..max))] pub timestamp: u64,
    #[asn(integer(0..999999))] pub submilli_nanos: u32,
    #[asn(integer(min..max))] pub session_id: u64,
    #[asn(utf8string)] pub session_ip: String,
    #[asn(complex(Identity, tag(UNIVERSAL(10))))] pub session_identity: Identity,
    #[asn(complex(Action, tag(UNIVERSAL(10))))] pub action: Action,
}

impl TrackingEvent {
    pub const fn timestamp_min() -> u64 {
        0
    }

    pub const fn timestamp_max() -> u64 {
        9_223_372_036_854_775_807
    }

    pub const fn submilli_nanos_min() -> u32 {
        0
    }

    pub const fn submilli_nanos_max() -> u32 {
        999_999
    }

    pub const fn session_id_min() -> u64 {
        0
    }

    pub const fn session_id_max() -> u64 {
        9_223_372_036_854_775_807
    }
}

#[asn(sequence)]

#[derive(Default, Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct DataMessage {
    #[asn(integer(min..max))] pub session_id: u64,
    #[asn(utf8string)] pub session_ip: String,
    #[asn(complex(Identity, tag(UNIVERSAL(10))))] pub session_identity: Identity,
    #[asn(complex(MessageMode, tag(UNIVERSAL(10))))] pub msg_mode: MessageMode,
    #[asn(integer(min..max))] pub timestamp: u64,
    #[asn(optional(octet_string))] pub encoded_data: Option<Vec<u8>>,
}

impl DataMessage {
    pub const fn session_id_min() -> u64 {
        0
    }

    pub const fn session_id_max() -> u64 {
        9_223_372_036_854_775_807
    }

    pub const fn timestamp_min() -> u64 {
        0
    }

    pub const fn timestamp_max() -> u64 {
        9_223_372_036_854_775_807
    }
}

#[asn(sequence)]

#[derive(Default, Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct TrackingInformation {
    #[asn(integer(min..max))] pub timestamp: u64,
    #[asn(integer(0..65535))] pub protocol: u16,
    #[asn(integer(0..65535))] pub message_id: u16,
    #[asn(octet_string)] pub message_body: Vec<u8>,
    #[asn(sequence_of(complex(TrackingEvent, tag(UNIVERSAL(16)))))] pub events: Vec<TrackingEvent>,
}

impl TrackingInformation {
    pub const fn timestamp_min() -> u64 {
        0
    }

    pub const fn timestamp_max() -> u64 {
        9_223_372_036_854_775_807
    }

    pub const fn protocol_min() -> u16 {
        0
    }

    pub const fn protocol_max() -> u16 {
        65_535
    }

    pub const fn message_id_min() -> u16 {
        0
    }

    pub const fn message_id_max() -> u16 {
        65_535
    }
}

#[asn(sequence)]

#[derive(Default, Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct DataSupplement {
    #[asn(integer(min..max))] pub session_id: u64,
    #[asn(utf8string)] pub session_ip: String,
    #[asn(complex(Identity, tag(UNIVERSAL(10))))] pub session_identity: Identity,
    #[asn(complex(MessageMode, tag(UNIVERSAL(10))))] pub msg_mode: MessageMode,
    #[asn(integer(min..max))] pub timestamp: u64,
    #[asn(integer(0..8000))] pub encoded_data_length: u16,
}

impl DataSupplement {
    pub const fn session_id_min() -> u64 {
        0
    }

    pub const fn session_id_max() -> u64 {
        9_223_372_036_854_775_807
    }

    pub const fn timestamp_min() -> u64 {
        0
    }

    pub const fn timestamp_max() -> u64 {
        9_223_372_036_854_775_807
    }

    pub const fn encoded_data_length_min() -> u16 {
        0
    }

    pub const fn encoded_data_length_max() -> u16 {
        8_000
    }
}

#[asn(sequence)]

#[derive(Default, Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct MalformedMessage {
    #[asn(integer(min..max))] pub session_id: u64,
    #[asn(utf8string)] pub session_ip: String,
    #[asn(complex(Identity, tag(UNIVERSAL(10))))] pub session_identity: Identity,
    #[asn(complex(MessageMode, tag(UNIVERSAL(10))))] pub msg_mode: MessageMode,
    #[asn(integer(min..max))] pub timestamp: u64,
    #[asn(octet_string)] pub encoded_data: Vec<u8>,
}

impl MalformedMessage {
    pub const fn session_id_min() -> u64 {
        0
    }

    pub const fn session_id_max() -> u64 {
        9_223_372_036_854_775_807
    }

    pub const fn timestamp_min() -> u64 {
        0
    }

    pub const fn timestamp_max() -> u64 {
        9_223_372_036_854_775_807
    }
}

#[asn(sequence)]

#[derive(Default, Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct MalformedContent {
    #[asn(octet_string)] pub encoded_data: Vec<u8>,
}

impl MalformedContent {
}

#[asn(sequence)]

#[derive(Default, Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct Version {
    #[asn(integer(0..255))] pub major: u8,
    #[asn(integer(0..255))] pub minor: u8,
    #[asn(integer(0..255))] pub patch: u8,
}

impl Version {
    pub const fn major_min() -> u8 {
        0
    }

    pub const fn major_max() -> u8 {
        255
    }

    pub const fn minor_min() -> u8 {
        0
    }

    pub const fn minor_max() -> u8 {
        255
    }

    pub const fn patch_min() -> u8 {
        0
    }

    pub const fn patch_max() -> u8 {
        255
    }
}

#[asn(sequence)]

#[derive(Default, Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct BuildInfo {
    #[asn(utf8string)] pub git_hash: String,
    #[asn(utf8string)] pub ci_platform: String,
    #[asn(utf8string)] pub profile: String,
    #[asn(utf8string)] pub time: String,
    #[asn(utf8string)] pub compiler: String,
}

impl BuildInfo {
}

#[asn(sequence)]

#[derive(Default, Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct ApplicationInfo {
    #[asn(complex(Identity, tag(UNIVERSAL(10))))] pub identity: Identity,
    #[asn(complex(Version, tag(UNIVERSAL(16))))] pub version: Version,
    #[asn(complex(BuildInfo, tag(UNIVERSAL(16))))] pub build: BuildInfo,
    #[asn(sequence_of(utf8string))] pub args: Vec<String>,
    #[asn(utf8string)] pub additional: String,
}

impl ApplicationInfo {
}