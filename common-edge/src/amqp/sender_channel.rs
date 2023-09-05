use crate::AmqpSender;
use common_amqp::dove::error::AmqpError;
use common_async::tokio::sync::mpsc;
use common_async::tokio::sync::mpsc::UnboundedSender;
use common_message::MessageId;
use std::future::Future;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub struct AmqpSenderChannel {
    sender: UnboundedSender<SendRequest>,
    counter: Arc<AtomicUsize>,
}

impl AmqpSenderChannel {
    #[inline]
    #[allow(clippy::result_unit_err)]
    pub fn send_parts(&self, mid: MessageId, body: impl Into<Vec<u8>>) -> Result<(), ()> {
        self.send(SendRequest::Parts(mid, body.into()))
    }

    #[inline]
    #[allow(clippy::result_unit_err)]
    fn send(&self, request: SendRequest) -> Result<(), ()> {
        self.counter.fetch_add(1, Ordering::Relaxed);
        self.sender.send(request).map_err(drop)
    }

    /// The number of messages being in-flight for transmission
    #[inline]
    pub fn len(&self) -> usize {
        self.counter.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl AmqpSender {
    pub fn into_unbound_channel(
        self,
    ) -> (
        AmqpSenderChannel,
        impl Future<Output = Result<(), AmqpError>>,
    ) {
        let counter = Arc::new(AtomicUsize::new(0));
        let (sender, mut receiver) = mpsc::unbounded_channel();
        (
            AmqpSenderChannel {
                sender,
                counter: Arc::clone(&counter),
            },
            async move {
                while let Some(request) = receiver.recv().await {
                    counter.fetch_sub(1, Ordering::Relaxed);
                    match request {
                        SendRequest::Parts(mid, body) => self.send_parts(mid, body).await?,
                    }
                }
                Ok(())
            },
        )
    }
}

enum SendRequest {
    Parts(MessageId, Vec<u8>),
}
