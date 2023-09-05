use crate::amqp::message::{ ReceivedMessage};
use crate::ReceiveError;
use common_amqp::dove::container::{Delivery, Message, Receiver};

pub struct AmqpReceiver {
    pub(super) receiver: Receiver,
}

impl AmqpReceiver {
    /// The address of this receiver instance. This address is for example required by an
    /// [`crate::amqp::AmqpSender`] to be able to send messages to this instance.
    #[inline]
    pub fn address(&self) -> &str {
        self.receiver.address()
    }

    #[inline]
    async fn receive(&self) -> Result<Delivery, ReceiveError> {
        Ok(self.receiver.receive().await?)
    }

    async fn receive_message(&self) -> Result<Message, ReceiveError> {
        self.receive()
            .await?
            .take_message()
            .ok_or(ReceiveError::MissingMessage)
    }

    #[inline]
    pub async fn receive_detailed_message(&mut self) -> Result<ReceivedMessage, ReceiveError> {
        ReceivedMessage::try_from(self.receive_message().await?)
    }
}
