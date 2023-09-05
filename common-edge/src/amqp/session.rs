use crate::amqp::receiver::AmqpReceiver;
use crate::amqp::sender::AmqpSender;
use crate::AmqpAuth;
use common_amqp::dove::conn::ConnectionOptions;
use common_amqp::dove::container::{Connection, Container, Session};
use common_amqp::dove::error::AmqpError;
use common_amqp::dove::options::{DynamicLifetimePolicy, ReceiverOptions};
use common_amqp::dove_util::receiver_filter;
use common_message::MessageId;
use std::net::ToSocketAddrs;
use std::time::{Duration, UNIX_EPOCH};

pub struct AmqpSession {
    sender_id: u32,
    station_id: Option<u32>,
    _container: Container,
    _connection: Connection,
    session: Session,
}

impl AmqpSession {
    pub async fn create<S: ToSocketAddrs + Send + 'static>(
        host: S,
        auth: AmqpAuth,
        station_id: Option<u32>,
    ) -> Result<Self, AmqpError> {
        let sender_id = {
            let unix_epoch_time = UNIX_EPOCH.elapsed().unwrap_or_default();
            ((unix_epoch_time.as_millis() as u64) << 16) as u32 | unix_epoch_time.subsec_nanos()
        };

        let container = Container::new()?.start();
        let connection = container
            .connect(
                host,
                match &auth {
                    AmqpAuth::Anonymous => ConnectionOptions::anonymous(),
                    AmqpAuth::Plain { username, password } => {
                        ConnectionOptions::plain(username.clone(), password.clone())
                    }
                }
                .tcp_nodelay(true),
            )
            .await?;

        Ok(Self {
            sender_id,
            station_id,
            session: connection.new_session(None).await?,
            _container: container,
            _connection: connection,
        })
    }

    /// Returns a new [`AmqpReceiver`] that is only valid for the current session with its own
    /// unique address.
    pub async fn new_unique_session_bound_receiver(&self) -> Result<AmqpReceiver, AmqpError> {
        let receiver = self
            .session
            .new_receiver_with_options(
                "",
                ReceiverOptions::default().with_dynamic_flag(DynamicLifetimePolicy::DeleteOnClose),
            )
            .await?;

        Ok(AmqpReceiver { receiver })
    }

    pub async fn new_receiver_for(
        &self,
        exchange: &str,
        receive_own_messages: bool,
        relevant_message_ids: impl IntoIterator<Item = MessageId>,
        station_id_filter: Option<u32>,
    ) -> Result<AmqpReceiver, AmqpError> {
        let receiver = self
            .session
            .new_receiver_with_options(
                exchange,
                receiver_filter(
                    Some(self.sender_id).filter(|_| !receive_own_messages),
                    relevant_message_ids.into_iter().map(u16::from),
                    station_id_filter,
                ),
            )
            .await?;
        Ok(AmqpReceiver { receiver })
    }

    pub async fn new_sender(&self, target: &str) -> Result<AmqpSender, AmqpError> {
        Ok(AmqpSender {
            reply_to: None,
            retry_timeout: Duration::ZERO,
            sender_id: self.sender_id,
            station_id: self.station_id,
            sender: self.session.new_sender(target).await?,
        })
    }
}
