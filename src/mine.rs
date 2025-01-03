use std::{
    future::Future,
    sync::{Arc, Mutex},
};

use anyhow::Result;

#[allow(async_fn_in_trait)]
pub trait Generator {
    async fn get_unique_id(&mut self) -> Result<u32>;
}

// Traits are ways of teaching the compiler to have a type behave in a certain way.

impl Generator for u32 {
    async fn get_unique_id(&mut self) -> Result<u32> {
        *self = self
            .checked_add(1)
            .ok_or_else(|| anyhow::anyhow!("No more IDs"))?;
        Ok(*self)
    }
}

pub fn create_u32_id_generator() -> impl Generator {
    42
}

impl Generator for Option<u32> {
    async fn get_unique_id(&mut self) -> Result<u32> {
        self.take().ok_or_else(|| anyhow::anyhow!("No more IDs"))
    }
}

impl Generator for Arc<Mutex<u32>> {
    async fn get_unique_id(&mut self) -> Result<u32> {
        let mut id = self.lock().unwrap();
        *id += 1;
        Ok(*id)
    }
}

struct IteratorGenerator<I> {
    iter: I,
}

impl<I> Generator for IteratorGenerator<I>
where
    I: Iterator<Item = u32>,
{
    async fn get_unique_id(&mut self) -> Result<u32> {
        self.iter
            .next()
            .ok_or_else(|| anyhow::anyhow!("No more IDs"))
    }
}

pub fn create_iterator_id_generator(i: impl Iterator<Item = u32>) -> impl Generator {
    IteratorGenerator { iter: i }
}

pub fn create_locked_id_generator() -> impl Generator + Sync + Send {
    Arc::new(Mutex::new(0))
}

type Request = tokio::sync::oneshot::Sender<u32>;
type RequestSender = tokio::sync::mpsc::Sender<Request>;
type RequestReceiver = tokio::sync::mpsc::Receiver<Request>;

async fn actor(mut receiver: RequestReceiver) {
    let mut next_id = 0;
    while let Some(request) = receiver.recv().await {
        next_id += 1;
        let _ = request.send(next_id);
    }
}

impl Generator for RequestSender {
    async fn get_unique_id(&mut self) -> Result<u32> {
        let (send, recv) = tokio::sync::oneshot::channel();
        self.send(send).await?;
        Ok(recv.await?)
    }
}

/// Factory method to create an id generator with the actor pattern.
/// Returns a tuple:
/// - A future that will keep the actor running until dropped.
/// - A generator that can be cloned and used to get unique IDs.
/// The future must be spawned or somehow awaited on in order to
/// keep the actor running.
pub fn create_actor_id_generator() -> (
    impl Future<Output = ()> + Send + Sync,
    impl Generator + Clone,
) {
    let (sender, receiver) = tokio::sync::mpsc::channel::<Request>(8);

    (actor(receiver), sender)
}
