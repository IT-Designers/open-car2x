use asn1rs::prelude::*;

#[asn(enumerated)]

#[derive(Debug, Clone, PartialEq, Hash, Copy, PartialOrd, Eq, Serialize, Deserialize, Default)]
pub enum DebugRequestMode {
    #[default] RetrieveOnce,
    Subscribe,
    Unsubscribe,
}

impl DebugRequestMode {
    pub fn variant(index: usize) -> Option<Self> {
        match index {
            0 => Some(DebugRequestMode::RetrieveOnce),
            1 => Some(DebugRequestMode::Subscribe),
            2 => Some(DebugRequestMode::Unsubscribe),
            _ => None,
        }
    }

    pub const fn variants() -> [Self; 3] {
        [
        DebugRequestMode::RetrieveOnce,
        DebugRequestMode::Subscribe,
        DebugRequestMode::Unsubscribe,
        ]
    }

    pub fn value_index(self) -> usize {
        match self {
            DebugRequestMode::RetrieveOnce => 0,
            DebugRequestMode::Subscribe => 1,
            DebugRequestMode::Unsubscribe => 2,
        }
    }
}

#[asn(enumerated)]

#[derive(Debug, Clone, PartialEq, Hash, Copy, PartialOrd, Eq, Serialize, Deserialize, Default)]
pub enum DebugRequestMessageId {
    #[default] ComponentStatus,
    All,
}

impl DebugRequestMessageId {
    pub fn variant(index: usize) -> Option<Self> {
        match index {
            0 => Some(DebugRequestMessageId::ComponentStatus),
            1 => Some(DebugRequestMessageId::All),
            _ => None,
        }
    }

    pub const fn variants() -> [Self; 2] {
        [
        DebugRequestMessageId::ComponentStatus,
        DebugRequestMessageId::All,
        ]
    }

    pub fn value_index(self) -> usize {
        match self {
            DebugRequestMessageId::ComponentStatus => 0,
            DebugRequestMessageId::All => 1,
        }
    }
}

#[asn(sequence)]

#[derive(Default, Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct DebugRequest {
    #[asn(complex(DebugRequestMode, tag(UNIVERSAL(10))))] pub mode: DebugRequestMode,
    #[asn(complex(DebugRequestMessageId, tag(UNIVERSAL(10))))] pub message_id: DebugRequestMessageId,
}

impl DebugRequest {
}

#[asn(sequence)]

#[derive(Default, Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct ComponentStatus {
    #[asn(integer(min..max))] pub unix_timestamp_millis: u64,
    #[asn(bit_string(size(50)))] pub alive_sensors: BitVec,
    #[asn(bit_string(size(5)), const(ENVIRONMENT_AND_PREDICTION_MODULE(0), WARN_MODULE(1), COOPERATIVE_MODULE(2), PLACEHOLDER_4_MODULE(3), PLACEHOLDER_5_MODULE(4)))] pub alive_server_modules: BitVec,
    #[asn(bit_string(size(10)), const(VEHICLE_01_CAM(0), VEHICLE_01_CPM(1), VEHICLE_02_CAM(2), VEHICLE_02_CPM(3), VEHICLE_03_CAM(4), VEHICLE_03_CPM(5), VEHICLE_04_CAM(6), VEHICLE_04_CPM(7), VEHICLE_05_CAM(8), VEHICLE_05_CPM(9)))] pub alive_autonomous_vehicles: BitVec,
    #[asn(bit_string(size(10)), const(VEHICLE_09_CAM(0), VEHICLE_10_CAM(1), VEHICLE_11_CAM(2), VEHICLE_12_CAM(3), VEHICLE_13_CAM(4), VEHICLE_14_CAM(5), VEHICLE_15_CAM(6), VEHICLE_16_CAM(7), VEHICLE_17_CAM(8), VEHICLE_18_CAM(9)))] pub alive_connected_vehicles: BitVec,
    #[asn(bit_string(size(10)), const(NOMADIC_DEVICE_01_VAM(0), NOMADIC_DEVICE_02_VAM(1), NOMADIC_DEVICE_03_VAM(2), NOMADIC_DEVICE_04_VAM(3), NOMADIC_DEVICE_05_VAM(4), NOMADIC_DEVICE_06_VAM(5), NOMADIC_DEVICE_07_VAM(6), NOMADIC_DEVICE_08_VAM(7), NOMADIC_DEVICE_09_VAM(8), NOMADIC_DEVICE_10_VAM(9)))] pub alive_nomadic_devices: BitVec,
    #[asn(bit_string(size(3)), const(ALL_COMPONENTS_NOMINAL(0), PLACEHOLDER_0(1), PLACEHOLDER_1(2)))] pub other: BitVec,
}

impl ComponentStatus {

    pub const ALIVE_SERVER_MODULES_ENVIRONMENT_AND_PREDICTION_MODULE: u64 = 0;

    pub const ALIVE_SERVER_MODULES_WARN_MODULE: u64 = 1;

    pub const ALIVE_SERVER_MODULES_COOPERATIVE_MODULE: u64 = 2;

    pub const ALIVE_SERVER_MODULES_PLACEHOLDER_4_MODULE: u64 = 3;

    pub const ALIVE_SERVER_MODULES_PLACEHOLDER_5_MODULE: u64 = 4;

    pub const ALIVE_AUTONOMOUS_VEHICLES_VEHICLE_01_CAM: u64 = 0;

    pub const ALIVE_AUTONOMOUS_VEHICLES_VEHICLE_01_CPM: u64 = 1;

    pub const ALIVE_AUTONOMOUS_VEHICLES_VEHICLE_02_CAM: u64 = 2;

    pub const ALIVE_AUTONOMOUS_VEHICLES_VEHICLE_02_CPM: u64 = 3;

    pub const ALIVE_AUTONOMOUS_VEHICLES_VEHICLE_03_CAM: u64 = 4;

    pub const ALIVE_AUTONOMOUS_VEHICLES_VEHICLE_03_CPM: u64 = 5;

    pub const ALIVE_AUTONOMOUS_VEHICLES_VEHICLE_04_CAM: u64 = 6;

    pub const ALIVE_AUTONOMOUS_VEHICLES_VEHICLE_04_CPM: u64 = 7;

    pub const ALIVE_AUTONOMOUS_VEHICLES_VEHICLE_05_CAM: u64 = 8;

    pub const ALIVE_AUTONOMOUS_VEHICLES_VEHICLE_05_CPM: u64 = 9;

    pub const ALIVE_CONNECTED_VEHICLES_VEHICLE_09_CAM: u64 = 0;

    pub const ALIVE_CONNECTED_VEHICLES_VEHICLE_10_CAM: u64 = 1;

    pub const ALIVE_CONNECTED_VEHICLES_VEHICLE_11_CAM: u64 = 2;

    pub const ALIVE_CONNECTED_VEHICLES_VEHICLE_12_CAM: u64 = 3;

    pub const ALIVE_CONNECTED_VEHICLES_VEHICLE_13_CAM: u64 = 4;

    pub const ALIVE_CONNECTED_VEHICLES_VEHICLE_14_CAM: u64 = 5;

    pub const ALIVE_CONNECTED_VEHICLES_VEHICLE_15_CAM: u64 = 6;

    pub const ALIVE_CONNECTED_VEHICLES_VEHICLE_16_CAM: u64 = 7;

    pub const ALIVE_CONNECTED_VEHICLES_VEHICLE_17_CAM: u64 = 8;

    pub const ALIVE_CONNECTED_VEHICLES_VEHICLE_18_CAM: u64 = 9;

    pub const ALIVE_NOMADIC_DEVICES_NOMADIC_DEVICE_01_VAM: u64 = 0;

    pub const ALIVE_NOMADIC_DEVICES_NOMADIC_DEVICE_02_VAM: u64 = 1;

    pub const ALIVE_NOMADIC_DEVICES_NOMADIC_DEVICE_03_VAM: u64 = 2;

    pub const ALIVE_NOMADIC_DEVICES_NOMADIC_DEVICE_04_VAM: u64 = 3;

    pub const ALIVE_NOMADIC_DEVICES_NOMADIC_DEVICE_05_VAM: u64 = 4;

    pub const ALIVE_NOMADIC_DEVICES_NOMADIC_DEVICE_06_VAM: u64 = 5;

    pub const ALIVE_NOMADIC_DEVICES_NOMADIC_DEVICE_07_VAM: u64 = 6;

    pub const ALIVE_NOMADIC_DEVICES_NOMADIC_DEVICE_08_VAM: u64 = 7;

    pub const ALIVE_NOMADIC_DEVICES_NOMADIC_DEVICE_09_VAM: u64 = 8;

    pub const ALIVE_NOMADIC_DEVICES_NOMADIC_DEVICE_10_VAM: u64 = 9;

    pub const OTHER_ALL_COMPONENTS_NOMINAL: u64 = 0;

    pub const OTHER_PLACEHOLDER_0: u64 = 1;

    pub const OTHER_PLACEHOLDER_1: u64 = 2;

}

impl ComponentStatus {
    pub const fn unix_timestamp_millis_min() -> u64 {
        0
    }

    pub const fn unix_timestamp_millis_max() -> u64 {
        9_223_372_036_854_775_807
    }
}