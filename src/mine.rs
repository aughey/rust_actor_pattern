use anyhow::Result;
use std::{
    future::Future,
    sync::{Arc, Mutex},
};

#[allow(async_fn_in_trait)]
pub trait Generator {
    async fn get_unique_id(&self) -> Result<u32>;
}

impl Generator for u32 {
    async fn get_unique_id(&self) -> Result<u32> {
        Ok(*self)
    }
}

impl Generator for Arc<Mutex<u32>> {
    async fn get_unique_id(&self) -> Result<u32> {
        let mut id = self.lock().unwrap();
        *id += 1;
        Ok(*id)
    }
}

pub fn create_const_id_generator() -> impl Generator {
    5
}

pub fn create_locked_id_generator() -> impl Generator {
    Arc::new(Mutex::new(0))
}

type Request = tokio::sync::oneshot::Sender<u32>;
type RequestSender = tokio::sync::mpsc::Sender<Request>;
type RequestReceiver = tokio::sync::mpsc::Receiver<Request>;

impl Generator for RequestSender {
    async fn get_unique_id(&self) -> Result<u32> {
        let (send, recv) = tokio::sync::oneshot::channel();
        let _ = self.send(send).await;
        Ok(recv.await?)
    }
}

async fn actor(mut receiver: RequestReceiver) {
    let mut next_id = 0;
    while let Some(request) = receiver.recv().await {
        next_id += 1;
        let _ = request.send(next_id);
    }
}

pub fn create_actor_id_generator() -> (
    impl Future<Output = ()> + Send + Sync,
    impl Generator + Clone,
) {
    let (sender, receiver) = tokio::sync::mpsc::channel::<Request>(8);

    (actor(receiver), sender)
}
