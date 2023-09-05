use crate::ReceiveError;
use common_amqp::dove::container::{Message, Value};
use common_amqp::dove::types::Timestamp;
use common_message::MessageId;
use std::time::UNIX_EPOCH;

#[derive(Debug)]
pub struct ReceivedMessage {
    id: Option<MessageId>,
    body: Vec<u8>,
    origin: Message,
    reception_time: Timestamp,
}

impl ReceivedMessage {
    pub fn id(&self) -> Option<MessageId> {
        self.id
    }

    pub fn body(&self) -> &[u8] {
        &self.body
    }

    pub fn creation_time_millis(&self) -> Option<u64> {
        self.origin
            .properties
            .as_ref()
            .and_then(|p| p.creation_time.as_ref().map(|t| t.0))
    }

    pub fn reception_time_millis(&self) -> u64 {
        self.reception_time.0
    }

    #[inline]
    pub fn into_body(self) -> Vec<u8> {
        self.into_body_and_id().0
    }

    #[inline]
    fn into_body_and_id(self) -> (Vec<u8>, Option<MessageId>) {
        self.into()
    }
}

impl TryFrom<Message> for ReceivedMessage {
    type Error = ReceiveError;

    fn try_from(mut message: Message) -> Result<Self, Self::Error> {
        use common_amqp::dove::message::MessageBody;
        let id = {
            let mid = common_amqp::dove_util::message_id_from_message_headers(&message);
            if Some(MessageId::Mcm) == mid {
                debug!("MCM properties: {:?}", message.properties);
                debug!(
                    "MCM application_properties: {:?}",
                    message.application_properties
                );
            }
            mid
        };

        let body = match &mut message.body {
            MessageBody::Data(data) | MessageBody::AmqpValue(Value::Binary(data)) => {
                Some(core::mem::take(data))
            }
            MessageBody::AmqpSequence(_) | MessageBody::AmqpValue(_) => None,
        }
        .ok_or(ReceiveError::MissingMessageBody)?;

        Ok(Self {
            id,
            body,
            origin: message,
            reception_time: Timestamp(UNIX_EPOCH.elapsed().unwrap_or_default().as_millis() as u64),
        })
    }
}

impl From<ReceivedMessage> for (Vec<u8>, Option<MessageId>) {
    fn from(message: ReceivedMessage) -> Self {
        (message.body, message.id)
    }
}
