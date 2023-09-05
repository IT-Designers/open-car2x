use crate::id;

#[derive(
    Debug, Clone, Copy, Hash, Eq, Ord, PartialEq, PartialOrd, strum::AsRefStr, strum::EnumIter,
)]
pub enum MessageId {
    Unknown(u16),

    // public - mecview
    ClientRegistration,
    SensorFrame,
    EnvironmentFrame,
    UpdateSubscription,
    InitMessage,
    RoadClearanceFrame,
    SensorIdleFrame,

    // public - lukas
    Denm,
    Cpm,
    Cam,
    Vam,
    Mcm,

    // public - lukas debug
    DebugRequest,
    ComponentStatus,

    // ITD
    DataMessage,
    DataSupplement,
    Record,
    MalformedMessage,
    TrackingInformation,
    TrackingEvent,
    TrackedRecord,
    ApplicationInfo,
    LogRecord,
    UtilizationReport,
    Ping,
    Pong,
    ShutdownRequest,

    // ITD -- web
    SystemStatus,
    ClientMessage,

    // ITD -- fusion
    SensorDeregistration,
    FusionInformation,
    UulmSpuBson,
}

impl From<u16> for MessageId {
    fn from(given: u16) -> Self {
        #[allow(deprecated, clippy::match_same_arms)]
        match given {
            id::public::CLIENT_REGISTRATION => MessageId::ClientRegistration,
            id::public::SENSOR_FRAME => MessageId::SensorFrame,
            id::public::ENVIRONMENT_FRAME => MessageId::EnvironmentFrame,
            id::public::UPDATE_SUBSCRIPTION => MessageId::UpdateSubscription,
            id::public::INIT_MESSAGE => MessageId::InitMessage,
            id::public::ROAD_CLEARANCE_FRAME => MessageId::RoadClearanceFrame,
            id::public::SENSOR_IDLE_FRAME => MessageId::SensorIdleFrame,

            id::lukas::DENM => MessageId::Denm,
            id::lukas::CPM => MessageId::Cpm,
            id::lukas::CAM => MessageId::Cam,
            id::lukas::VAM => MessageId::Vam,
            id::lukas::MCM => MessageId::Mcm,

            id::lukas_debug::DEBUG_REQUEST => MessageId::DebugRequest,
            id::lukas_debug::COMPONENT_STATUS => MessageId::ComponentStatus,

            id::itd::com::DATA_MESSAGE => MessageId::DataMessage,
            id::itd::com::RECORD => MessageId::Record,
            id::itd::com::DATA_SUPPLEMENT => MessageId::DataSupplement,
            id::itd::com::MALFORMED_MESSAGE => MessageId::MalformedMessage,
            id::itd::com::TRACKING_INFORMATION => MessageId::TrackingInformation,
            id::itd::com::TRACKING_EVENT => MessageId::TrackingEvent,
            id::itd::com::TRACKED_RECORD => MessageId::TrackedRecord,
            id::itd::com::APPLICATION_INFO => MessageId::ApplicationInfo,
            id::itd::com::LOG_RECORD => MessageId::LogRecord,
            id::itd::com::UTILIZATION_REPORT => MessageId::UtilizationReport,
            id::itd::com::PING => MessageId::Ping,
            id::itd::com::PONG => MessageId::Pong,
            id::itd::com::SHUTDOWN_REQUEST => MessageId::ShutdownRequest,

            id::itd::web::STATUS => MessageId::SystemStatus,
            id::itd::web::CLIENT_MESSAGE => MessageId::ClientMessage,

            id::itd::fusion::SENSOR_DEREGISTRATION => MessageId::SensorDeregistration,
            id::itd::fusion::FUSION_INFORMATION => MessageId::FusionInformation,
            id::itd::fusion::UULM_SPU_BSON => MessageId::UulmSpuBson,

            id::legacy::DATA_MESSAGE => MessageId::DataMessage,
            id::legacy::STATUS => MessageId::SystemStatus,
            id::legacy::CLIENT_MESSAGE => MessageId::ClientMessage,
            id::legacy::RECORDING => MessageId::Record,

            _ => MessageId::Unknown(given),
        }
    }
}

impl From<MessageId> for u16 {
    fn from(mid: MessageId) -> u16 {
        #[allow(deprecated)]
        match mid {
            MessageId::ClientRegistration => id::public::CLIENT_REGISTRATION,
            MessageId::SensorFrame => id::public::SENSOR_FRAME,
            MessageId::EnvironmentFrame => id::public::ENVIRONMENT_FRAME,
            MessageId::UpdateSubscription => id::public::UPDATE_SUBSCRIPTION,
            MessageId::InitMessage => id::public::INIT_MESSAGE,
            MessageId::RoadClearanceFrame => id::public::ROAD_CLEARANCE_FRAME,
            MessageId::SensorIdleFrame => id::public::SENSOR_IDLE_FRAME,

            MessageId::Denm => id::lukas::DENM,
            MessageId::Cpm => id::lukas::CPM,
            MessageId::Cam => id::lukas::CAM,
            MessageId::Vam => id::lukas::VAM,
            MessageId::Mcm => id::lukas::MCM,

            MessageId::DebugRequest => id::lukas_debug::DEBUG_REQUEST,
            MessageId::ComponentStatus => id::lukas_debug::COMPONENT_STATUS,

            MessageId::DataMessage => id::itd::com::DATA_MESSAGE,
            MessageId::Record => id::itd::com::RECORD,
            MessageId::DataSupplement => id::itd::com::DATA_SUPPLEMENT,
            MessageId::MalformedMessage => id::itd::com::MALFORMED_MESSAGE,
            MessageId::TrackingInformation => id::itd::com::TRACKING_INFORMATION,
            MessageId::TrackingEvent => id::itd::com::TRACKING_EVENT,
            MessageId::TrackedRecord => id::itd::com::TRACKED_RECORD,
            MessageId::ApplicationInfo => id::itd::com::APPLICATION_INFO,
            MessageId::LogRecord => id::itd::com::LOG_RECORD,
            MessageId::UtilizationReport => id::itd::com::UTILIZATION_REPORT,
            MessageId::Ping => id::itd::com::PING,
            MessageId::Pong => id::itd::com::PONG,
            MessageId::ShutdownRequest => id::itd::com::SHUTDOWN_REQUEST,

            MessageId::SystemStatus => id::itd::web::STATUS,
            MessageId::ClientMessage => id::itd::web::CLIENT_MESSAGE,

            MessageId::SensorDeregistration => id::itd::fusion::SENSOR_DEREGISTRATION,
            MessageId::FusionInformation => id::itd::fusion::FUSION_INFORMATION,
            MessageId::UulmSpuBson => id::itd::fusion::UULM_SPU_BSON,

            MessageId::Unknown(id) => id,
        }
    }
}
