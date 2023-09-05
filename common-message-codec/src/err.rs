use crate::ProtocolVersion;
use common_message::MessageId;

#[derive(thiserror::Error, Debug)]
pub enum CodecError {
    #[error("Failed to read (io-error): {0}")]
    IoError(#[from] std::io::Error),
    #[error("Failed to decode: {0}")]
    DecodeError(#[from] asn1rs::io::per::Error),
    #[error("Message with unexpected message-id received: {0:?}")]
    UnexpectedMessageId(MessageId),
    #[error("Expected {expected:?} but received {received:?}")]
    UnexpectedProtocolVersion {
        expected: ProtocolVersion,
        received: ProtocolVersion,
    },
    #[error("Failed to serialize BSON document: {0}")]
    BsonSerializeError(#[from] bson::ser::Error),
    #[error("Failed to deserialize BSON document: {0}")]
    BsonDeserializeError(#[from] bson::de::Error),
}

impl CodecError {
    pub const fn unexpected_message_id(mid: MessageId) -> Self {
        Self::UnexpectedMessageId(mid)
    }

    pub const fn check_protocol_version(
        expected: ProtocolVersion,
        received: ProtocolVersion,
    ) -> Result<(), CodecError> {
        if expected == received {
            Ok(())
        } else {
            Err(CodecError::UnexpectedProtocolVersion { expected, received })
        }
    }
}
