use crate::MessageBody;
use crate::ProtocolVersion;
use common_message::MessageId;

#[derive(PartialEq, PartialOrd, Clone, Debug)]
pub struct RawParts {
    pub protocol: ProtocolVersion,
    pub message_id: MessageId,
    pub body: MessageBody,
}

impl RawParts {
    pub const fn empty() -> Self {
        Self {
            protocol: 0,
            message_id: MessageId::Unknown(0),
            body: MessageBody::Vec(Vec::new()),
        }
    }

    pub const fn protocol_version(&self) -> ProtocolVersion {
        self.protocol
    }

    pub const fn id(&self) -> MessageId {
        self.message_id
    }
}
