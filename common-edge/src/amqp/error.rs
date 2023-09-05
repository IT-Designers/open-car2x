use common_amqp::dove::error::AmqpError;

#[derive(Debug, thiserror::Error)]
pub enum ReceiveError {
    #[error("AMQP-Transport error: {0:?}")]
    Amqp(#[from] AmqpError),
    #[error("The Delivery contained no Message")]
    MissingMessage,
    #[error("The MessageId of the received message cannot be determined")]
    MissingMessageId,
    #[error("The message is missing a body")]
    MissingMessageBody,
}
