pub mod ffi;

use crate::pods::connection::{CrConnectionInfo, CrConnectionStatus};
use crate::pods::message::{CrMessageId, CrMessageIdOverloaded};
use crate::worker::conf::CrConnectionConfig;
use crate::worker::{Worker, WorkerRequest};
use common_async::tokio;
use common_edge::common_async::tokio::sync::Mutex;
use common_edge::messages;
use common_edge::{common_async, ReceivedMessage};
use log::error;
use messages::itd_data_protocol::ApplicationInfo;
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Builder;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::UnboundedSender;
use tokio::task::JoinHandle;

#[derive(Debug)]
pub struct CrConnection {
    sender: UnboundedSender<WorkerRequest>,
    status: Arc<Mutex<CrConnectionInfo>>,
    worker: JoinHandle<()>,
    runtime: Runtime,
}

impl CrConnection
where
    Self: Sync + Send,
{
    fn new(config: CrConnectionConfig, info: ApplicationInfo) -> Self {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        let status = Arc::new(Mutex::new(CrConnectionInfo {
            status: CrConnectionStatus::Initializing,
            times_connected_counter: 0,
            connection_epoch_millis_timestamp: 0,
            message_receiver_queue_size: 0,
            message_receiver_queue_types: CrMessageIdOverloaded::empty(),
            message_sender_queue_size: 0,
        }));

        let worker = Worker {
            config,
            info,
            status: status.clone(),
            receiver,
            received_messages: Vec::default(),
            waiting_requests: Vec::default(),
        };

        let runtime = Builder::new_multi_thread().enable_all().build().unwrap();
        let worker = runtime.spawn(async {
            worker.run().await;
            if cfg!(debug_assertions) {
                println!("Worker: Execution has stopped");
            }
        });

        Self {
            sender,
            status,
            worker,
            runtime,
        }
    }

    #[inline]
    fn send_message(
        &self,
        message_id: CrMessageId,
        body: Vec<u8>,
    ) -> Result<(), SendError<(CrMessageId, Vec<u8>)>> {
        self.sender
            .send(WorkerRequest::SendMessage {
                id: message_id,
                body,
            })
            .map_err(|SendError(req)| match req {
                WorkerRequest::SendMessage { id, body } => SendError((id, body)),
                _ => unreachable!(),
            })
    }

    fn receive_message(
        &self,
        accepted_ids: Vec<CrMessageId>,
        timeout: Duration,
    ) -> Result<(CrMessageId, Box<ReceivedMessage>), bool> {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        self.sender
            .send(WorkerRequest::RetrieveReceivedMessage {
                filter: accepted_ids,
                sender,
                retry: timeout > Duration::from_millis(0),
            })
            .map_err(|_| false)?;

        let receiver = async move {
            if timeout > Duration::from_millis(0) {
                tokio::time::timeout(timeout, receiver).await
            } else {
                Ok(receiver.await)
            }
        };

        match self.runtime.block_on(receiver) {
            Ok(Ok((mid, message))) => Ok((mid, message)),
            Ok(Err(e)) if timeout > Duration::from_millis(0) => {
                eprintln!("{e}");
                error!("{e}");
                Err(false)
            }
            /*Err(_elapsed) |*/ _ => Err(true),
        }
    }

    fn receive_message_into(
        &self,
        accepted_ids: Vec<CrMessageId>,
        timout: Duration,
        target_buffer: &mut [u8],
    ) -> Result<(CrMessageId, usize), bool> {
        self.receive_message(accepted_ids, timout)
            .map(|(id, message)| {
                let bytes = message.into_body();
                let len = target_buffer.len().min(bytes.len());
                target_buffer[..len].copy_from_slice(&bytes[..len]);
                (id, len)
            })
    }

    /// Tries to clone and return the current [`CrConnectionInfo`]
    #[inline]
    pub fn load_status_info(&self) -> Option<CrConnectionInfo> {
        Some(self.status.blocking_lock().clone())
    }

    /// Requests the internal worker to stop and returns the [`JoinHandle`] to be able to wait
    /// until it has actually stopped.
    #[allow(clippy::result_unit_err)]
    fn stop(self) -> Result<(), ()> {
        // drop the sender to let the worker know to stop
        drop(self.sender);
        drop(self.status);

        // move out of self to be able to move into async-block
        let worker = self.worker;
        let runtime = self.runtime;

        let result = runtime
            .block_on(async { tokio::time::timeout(Duration::from_secs(2), worker).await })
            .map(drop)
            .map_err(drop);
        runtime.shutdown_timeout(Duration::from_secs(3));
        result
    }
}
