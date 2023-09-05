use crate::pods::connection::CrConnectionStatus;
use crate::pods::message::CrMessageId;
use common_amqp::dove::error::AmqpError;
use common_async::tokio;
use common_edge::common_async;
use common_edge::common_async::tokio::task::JoinHandle;
use common_edge::common_message;
use common_edge::{common_amqp, AmqpSenderChannel};
use common_edge::{AmqpAuth, AmqpReceiver, AmqpSession};
use common_message::MessageId;
use log::{error, warn};
use std::convert::TryFrom;
use std::future::Future;
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;
use std::ops::Sub;
use std::pin::Pin;
use std::time::{Duration, SystemTime};

struct Connected {
    _session: AmqpSession,
    sender: AmqpSenderChannel,
    sender_handle: Option<JoinHandle<Result<(), AmqpError>>>,
    session_receiver: AmqpReceiver,
    message_receiver: AmqpReceiver,
}

#[allow(clippy::large_enum_variant)]
enum State {
    NotConnected,
    Connecting(Pin<Box<dyn Future<Output = Result<Connected, AmqpError>> + Send + Sync>>),
    Connected(Connected),
}

pub enum Event {
    ConnectionStatusChanged(CrConnectionStatus),
    Received(CrMessageId, super::ReceivedMessage),
}

pub struct Connection {
    host: String,
    auth: AmqpAuth,
    state: State,
    reconnect_timeout: Duration,
    send_timeout: Duration,
    last_connection_attempt: SystemTime,
    receive_own_messages: bool,
    accepted_messages: Vec<MessageId>,
    exchange_target: String,
    exchange_source: String,
    station_id: Option<u32>,
    station_id_receive_filter: Option<u32>,
}

impl Connection {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        host: impl Into<String>,
        auth: impl Into<AmqpAuth>,
        reconnect_timeout: Duration,
        send_timeout: Duration,
        receive_own_messages: bool,
        accepted_messages: Vec<MessageId>,
        exchange_target: impl Into<String>,
        exchange_source: impl Into<String>,
        station_id: Option<u32>,
        station_id_receive_filter: Option<u32>,
    ) -> Self {
        Self {
            host: host.into(),
            auth: auth.into(),
            state: State::NotConnected,
            reconnect_timeout,
            send_timeout,
            last_connection_attempt: SystemTime::now().sub(reconnect_timeout),
            receive_own_messages,
            accepted_messages,
            exchange_target: exchange_target.into(),
            exchange_source: exchange_source.into(),
            station_id,
            station_id_receive_filter,
        }
    }

    async fn update_state(&mut self) -> CrConnectionStatus {
        match self.state {
            State::NotConnected => {
                self.ensure_reconnect_timeout_reached().await;
                self.last_connection_attempt = SystemTime::now();

                let host = self.host.clone();
                let auth = self.auth.clone();
                let receive_own_messages = self.receive_own_messages;
                let accepted_messages = self.accepted_messages.clone();
                let exchange_source = self.exchange_source.clone();
                let exchange_target = self.exchange_target.clone();
                let send_timeout = self.send_timeout;
                let station_id = self.station_id;
                let station_id_receive_filter = self.station_id_receive_filter;

                self.state = State::Connecting(Box::pin(async move {
                    let session = AmqpSession::create(host, auth, station_id).await?;

                    let message_receiver = session
                        .new_receiver_for(
                            &exchange_source,
                            receive_own_messages,
                            accepted_messages,
                            station_id_receive_filter,
                        )
                        .await?;

                    let session_receiver = session.new_unique_session_bound_receiver().await?;
                    let (sender, future) = session
                        .new_sender(&exchange_target)
                        .await?
                        .with_reply_to(session_receiver.address())
                        .with_retry_timeout(send_timeout)
                        .into_unbound_channel();

                    Ok(Connected {
                        _session: session,
                        sender,
                        sender_handle: Some(common_async::spawn(future)),
                        session_receiver,
                        message_receiver,
                    })
                }));
                CrConnectionStatus::Connecting
            }
            State::Connecting(ref mut connection) => match connection.await {
                Err(e) => {
                    eprintln!("Worker: error while trying to connect. {e}: {e:?}");
                    error!("Error while trying to connect. {}: {:?}", e, e);
                    self.state = State::NotConnected;
                    CrConnectionStatus::Disconnected
                }
                Ok(connection) => {
                    self.state = State::Connected(connection);
                    CrConnectionStatus::Connected
                }
            },
            State::Connected(..) => CrConnectionStatus::Connected,
        }
    }

    pub async fn receive(&mut self) -> Option<Event> {
        if let State::Connected(connection) = &mut self.state {
            let message = loop {
                tokio::select! {
                    s = connection.session_receiver.receive_detailed_message() => break s,
                    m = connection.message_receiver.receive_detailed_message() => break m,
                }
            };

            match message {
                Ok(message) if message.id().is_some() => {
                    match CrMessageId::try_from(message.id().unwrap()) {
                        Ok(cid) => Some(Event::Received(cid, Box::new(message))),
                        Err(e) => {
                            eprintln!("Worker: Failed to convert MessageId to CrMessageId: {e:?}",);
                            error!("Failed to convert MessageId to CrMessageId: {e:?}",);
                            None
                        }
                    }
                }
                Ok(_message) => {
                    eprintln!(
                        "Worker: ignoring received message for which no message-id could be determined"
                    );
                    warn!("Ignoring received message for which no message-id could be determined");
                    None
                }
                Err(e) => {
                    eprintln!("Worker: error while trying to receive: {e:?}");
                    error!("Error while trying to receive: {e:?}");
                    self.state = State::NotConnected;
                    Some(Event::ConnectionStatusChanged(
                        CrConnectionStatus::Disconnected,
                    ))
                }
            }
        } else {
            Some(Event::ConnectionStatusChanged(self.update_state().await))
        }
    }

    async fn ensure_reconnect_timeout_reached(&self) {
        let elapsed_duration = self.last_connection_attempt.elapsed().unwrap_or_default();
        if elapsed_duration < self.reconnect_timeout {
            tokio::time::sleep(self.reconnect_timeout - elapsed_duration).await;
        }
    }

    pub async fn send(
        &mut self,
        mid: MessageId,
        body: impl Into<Vec<u8>>,
    ) -> Result<(), AmqpError> {
        if let State::Connected(connection) = &mut self.state {
            match connection.sender.send_parts(mid, body) {
                Ok(_) => Ok(()),
                Err(_) => {
                    // oh noes, where is the problem exactly?
                    let is_finished = connection
                        .sender_handle
                        .as_ref()
                        .map(|h| h.is_finished())
                        .unwrap_or(false);
                    if is_finished {
                        match connection.sender_handle.take().unwrap().await {
                            Ok(Ok(())) => Err(AmqpError::SendError),
                            Ok(Err(amqp)) => Err(amqp),
                            Err(e) => Err(AmqpError::Generic(format!("{e:?}"))),
                        }
                    } else {
                        Err(AmqpError::SendError)
                    }
                }
            }
        } else {
            Err(IoError::from(IoErrorKind::NotConnected).into())
        }
    }

    /// Tries to retrieve the number of enqueued messages for transmission. Returns `None` if
    /// there is no valid connection.
    pub fn send_queue_len(&self) -> Option<usize> {
        if let State::Connected(connection) = &self.state {
            Some(connection.sender.len())
        } else {
            None
        }
    }
}
