use common_amqp::dove::container::{Message, Sender};
use common_amqp::dove::error::AmqpError;
use common_amqp::dove_util::wrap_message;
use common_message::MessageId;
use std::ops::Div;
use std::time::{Duration, Instant};

pub struct AmqpSender {
    pub(super) reply_to: Option<String>,
    pub(super) retry_timeout: Duration,
    pub(super) sender_id: u32,
    pub(super) station_id: Option<u32>,
    pub(super) sender: Sender,
}

impl AmqpSender {
    /// The address appended to all sent messages for sending responses back
    pub fn reply_to(&self) -> Option<&str> {
        self.reply_to.as_deref()
    }

    /// Sets the address appended to all sent messages for sending responses back
    pub fn set_reply_to(&mut self, reply_to: impl Into<String>) {
        self.reply_to = Some(reply_to.into());
    }

    /// Returns itself with [`AmqpSender::set_reply_to`] applied
    pub fn with_reply_to(mut self, reply_to: impl Into<String>) -> Self {
        self.set_reply_to(reply_to);
        self
    }

    /// If non zero, the time until a retry-ing to send a message is considered failed.
    /// On [`Duration::ZERO`], a failed send attempt will fail immediately without retry-ing.
    pub fn retry_timeout(&self) -> Duration {
        self.retry_timeout
    }

    /// If non zero, the time until a retry-ing to send a message is considered failed.
    /// On [`Duration::ZERO`], a failed send attempt will fail immediately without retry-ing.
    pub fn set_retry_timeout(&mut self, duration: Duration) {
        self.retry_timeout = duration;
    }

    /// Returns itself with [`AmqpSender::set_retry_timeout`] applied
    pub fn with_retry_timeout(mut self, duration: Duration) -> Self {
        self.set_retry_timeout(duration);
        self
    }

    pub(crate) async fn send_parts(
        &self,
        mid: MessageId,
        body: impl Into<Vec<u8>>,
    ) -> Result<(), AmqpError> {
        self.send_message(wrap_message(
            self.sender_id,
            self.reply_to.clone(),
            mid,
            body,
            self.station_id,
        ))
        .await
    }

    async fn send_message(&self, mut message: Message) -> Result<(), AmqpError> {
        let start = Instant::now();
        loop {
            match self.sender.send(message).await {
                Ok(_disposition) => break Ok(()),
                Err(common_amqp::dove::error::AmqpError::NotEnoughCreditsToSend(msg))
                    if start.elapsed() < self.retry_timeout =>
                {
                    warn!("Not enough credits to send message...");

                    // restore to be able to try again
                    message = *msg;

                    // wait a bit to reduce pressure
                    common_async::tokio::time::sleep(
                        Duration::from_millis(10)
                            .min(self.retry_timeout.div(10).max(Duration::from_millis(1))),
                    )
                    .await;
                }
                Err(e) => break Err(e),
            }
        }
    }
}
