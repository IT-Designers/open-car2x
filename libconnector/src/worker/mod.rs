use crate::pods::connection::{CrConnectionInfo, CrConnectionStatus};
use crate::pods::identity::CrIdentity;
use crate::pods::message::{CrMessageId, CrMessageIdOverloaded};
use crate::worker::con::{Connection, Event};
use crate::worker::conf::CrConnectionConfig;
use common_async::tokio;
use common_edge::common_async;
use common_edge::common_async::tokio::sync::Mutex;
use common_edge::common_async::tokio::time::sleep;
use common_edge::common_message;
use common_edge::messages;
use common_edge::AmqpAuth;
use common_message::MessageId;
use log::{debug, error};
use messages::itd_data_protocol::ApplicationInfo;
use std::convert::TryFrom;
use std::sync::Arc;
use std::time::UNIX_EPOCH;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::oneshot::Sender as OneShotSender;
use tokio::time::Duration;

pub mod con;
pub mod conf;
pub mod rcv;

pub type ReceivedMessage = Box<common_edge::ReceivedMessage>;

#[derive(Debug)]
pub(crate) enum WorkerRequest {
    RetrieveReceivedMessage {
        filter: Vec<CrMessageId>,
        sender: OneShotSender<(CrMessageId, ReceivedMessage)>,
        retry: bool,
    },
    SendMessage {
        id: CrMessageId,
        body: Vec<u8>,
    },
}

pub(crate) struct Worker {
    pub(crate) config: CrConnectionConfig,
    pub(crate) info: ApplicationInfo,
    pub(crate) status: Arc<Mutex<CrConnectionInfo>>,
    pub(crate) receiver: UnboundedReceiver<WorkerRequest>,
    pub(crate) received_messages: Vec<(CrMessageId, ReceivedMessage)>,
    pub(crate) waiting_requests: Vec<WorkerRequest>,
}

impl Worker {
    const DEFAULT_ADDRESS: &'static str = "127.0.0.1:5672";
    const DEFAULT_RECONNECT_TIMEOUT: Duration = Duration::from_secs(1);
    const DEFAULT_SEND_TIMEOUT: Duration = Duration::from_secs(1);
    const DEFAULT_LOGIN_USER: &'static str = "prod1";
    const DEFAULT_LOGIN_PASSWORD: &'static str = "password1";
    const DEFAULT_TARGET_EXCHANGE: &'static str = "messages";
    const DEFAULT_SOURCE_EXCHANGE: &'static str = "messages";
    const DEFAULT_RECEIVE_OWN: bool = false;
    const MIN_CAPACITY_SIZE_WAITING_REQUESTS: usize = 32;

    pub async fn run(mut self) {
        let mut connection = Connection::new(
            self.config.address.as_deref().unwrap_or_else(|| {
                CrIdentity::opt_from_protocol_self(self.info.identity)
                    .map(CrIdentity::default_address)
                    .unwrap_or(Self::DEFAULT_ADDRESS)
            }),
            if Some(true) == self.config.anonymous
                || (self.config.login_user.is_none()
                    && self.config.login_password.is_none()
                    && self.config.anonymous.is_none())
            {
                AmqpAuth::Anonymous
            } else {
                AmqpAuth::Plain {
                    username: self
                        .config
                        .login_user
                        .as_deref()
                        .unwrap_or(Self::DEFAULT_LOGIN_USER)
                        .to_string(),
                    password: self
                        .config
                        .login_password
                        .as_deref()
                        .unwrap_or(Self::DEFAULT_LOGIN_PASSWORD)
                        .to_string(),
                }
            },
            self.config
                .reconnect_timeout
                .unwrap_or(Self::DEFAULT_RECONNECT_TIMEOUT),
            self.config
                .send_timeout
                .unwrap_or(Self::DEFAULT_SEND_TIMEOUT),
            self.config
                .receive_own_messages
                .unwrap_or(Self::DEFAULT_RECEIVE_OWN),
            self.config
                .filter_options
                .map(|ids| {
                    ids.iter_set()
                        .map(CrMessageId::to_common_id)
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default(),
            self.config
                .target_exchange
                .as_deref()
                .unwrap_or(Self::DEFAULT_TARGET_EXCHANGE),
            self.config
                .source_exchange
                .as_deref()
                .unwrap_or(Self::DEFAULT_SOURCE_EXCHANGE),
            self.config.station_id,
            self.config.station_id_receive_filter,
        );

        loop {
            self.update_status(&mut connection).await;
            tokio::select! {
                // run the loop at least once a second to update the status
                _ = sleep(Duration::from_secs(1)) => {},
                request = self.receiver.recv() => if let Some(request) = request {
                    self.process_next_worker_request(request, &mut connection).await;
                } else {
                    break;
                },
                update = connection.receive() => if let Some(update) = update {
                    self.process_next_connection_update(update, &mut connection).await;
                }
            }
        }

        if cfg!(debug_assertions) {
            println!("Worker: Receiver has been closed");
        }
    }

    async fn update_status(&self, connection: &mut Connection) {
        let message_sender_queue_size = connection
            .send_queue_len()
            .unwrap_or_default()
            .min(usize::from(u16::MAX)) as u16;

        {
            let mut status = self.status.lock().await;
            status.message_sender_queue_size = message_sender_queue_size;
        }
    }

    async fn process_next_connection_update(&mut self, update: Event, connection: &mut Connection) {
        match update {
            Event::ConnectionStatusChanged(status) => {
                if cfg!(debug_assertions) {
                    println!("Worker: Connection status changed to {:?}", status);
                }
                {
                    let mut lock = self.status.lock().await;
                    if CrConnectionStatus::Connected == status {
                        lock.times_connected_counter =
                            lock.times_connected_counter.saturating_add(1);
                        lock.connection_epoch_millis_timestamp =
                            UNIX_EPOCH.elapsed().unwrap_or_default().as_millis() as u64;
                    }
                    lock.status = status;
                }
                if CrConnectionStatus::Connected == status {
                    self.send_application_info(connection).await;
                }
            }
            Event::Received(id, message) => {
                if cfg!(debug_assertions) {
                    println!("Worker: Received: {:?}/{:?}", id, message);
                }
                {
                    let mut lock = self.status.lock().await;
                    lock.message_receiver_queue_types.add(id);
                    lock.message_receiver_queue_size =
                        lock.message_receiver_queue_size.saturating_add(1);
                }
                self.received_messages.push((id, message));
                self.process_waiting_worker_requests(connection).await;
            }
        }
    }

    async fn send_application_info(&self, connection: &mut Connection) {
        let bytes = {
            use asn1rs::prelude::*;
            use common_edge::asn1rs;
            let mut writer = UperWriter::default();
            if let Err(e) = self.info.write(&mut writer) {
                eprintln!("Worker: Failed to serialize ApplicationInfo: {e}");
                error!("Failed to serialize ApplicationInfo: {e}");
                return;
            } else {
                writer.into_bytes_vec()
            }
        };
        if let Err(e) = connection.send(MessageId::ApplicationInfo, bytes).await {
            eprintln!("Worker: Failed to send ApplicationInfo: {e}");
            error!("Failed to send ApplicationInfo: {e}");
        }
    }

    async fn process_next_worker_request(
        &mut self,
        request: WorkerRequest,
        connection: &mut Connection,
    ) {
        if self.waiting_requests.is_empty() {
            if let Some(request) = self.try_process(request, connection).await {
                self.waiting_requests.push(request);
            }
        } else {
            self.waiting_requests.push(request);
            self.process_waiting_worker_requests(connection).await
        }
    }

    async fn process_waiting_worker_requests(&mut self, connection: &mut Connection) {
        let empty_replacement = Vec::with_capacity(
            (self.waiting_requests.len() / 2 + 1).max(Self::MIN_CAPACITY_SIZE_WAITING_REQUESTS),
        );
        for request in std::mem::replace(&mut self.waiting_requests, empty_replacement) {
            if let Some(request) = self.try_process(request, connection).await {
                self.waiting_requests.push(request);
            }
        }
    }

    async fn try_process(
        &mut self,
        request: WorkerRequest,
        connection: &mut Connection,
    ) -> Option<WorkerRequest> {
        match request {
            WorkerRequest::RetrieveReceivedMessage { sender, .. } if sender.is_closed() => None,
            WorkerRequest::RetrieveReceivedMessage {
                filter,
                sender,
                retry,
            } => {
                if cfg!(debug_assertions) {
                    println!("Worker: Receiving a message with filter: {:?}", filter);
                }
                debug!("Receiving a message with filter: {:?}", filter);

                let index =
                    self.received_messages
                        .iter()
                        .enumerate()
                        .find_map(|(index, (mid, _bytes))| {
                            if filter.is_empty() || filter.contains(mid) {
                                Some(index)
                            } else {
                                None
                            }
                        });
                if let Some(index) = index {
                    if let Err(msg) = sender.send(self.received_messages.remove(index)) {
                        self.received_messages.insert(index, msg);
                    } else {
                        let types = self.received_messages.iter().fold(
                            CrMessageIdOverloaded::empty(),
                            |mut current, (id, _bytes)| {
                                current.add(*id);
                                current
                            },
                        );
                        {
                            let mut lock = self.status.lock().await;
                            lock.message_receiver_queue_types = types;
                            lock.message_receiver_queue_size =
                                u16::try_from(self.received_messages.len()).unwrap_or(u16::MAX);
                        }
                    }
                    None
                } else if retry {
                    Some(WorkerRequest::RetrieveReceivedMessage {
                        filter,
                        sender,
                        retry,
                    })
                } else {
                    None
                }
            }
            WorkerRequest::SendMessage { id, body } => {
                if cfg!(debug_assertions) {
                    println!(
                        "Worker: Sending a message of type {:?} and len {}",
                        id,
                        body.len()
                    );
                }
                debug!("Sending a message of type {:?} and len {}", id, body.len());

                if let Err(e) = connection.send(id.to_common_id(), body).await {
                    eprintln!("Worker: Failed to send message: {e:?}");
                    error!("Failed to send message: {e:?}");
                }

                None
            }
        }
    }
}
