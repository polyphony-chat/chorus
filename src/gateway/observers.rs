// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Includes pre-made observers to use with gateway events

use crate::types::WebSocketEvent;
use async_trait::async_trait;
use log::warn;
use pubserve::Subscriber;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Observes an event once and sends it via a [tokio::sync::oneshot] channel.
///
/// # Examples
/// ```no_run
/// # tokio_test::block_on(async {
/// # mod tests::common;
/// # let mut bundle = common::setup().await;
/// use chorus::gateway::{GatewayHandle, OneshotEventObserver};
/// use chorus::types::MessageCreate;
///
/// let handle: GatewayHandle; // Get this either by user.gateway or by manually opening a connection
/// # let handle = bundle.user.gateway;
///
/// // Let's say we want to wait until the next MessageCreate event
/// // Create the observer and receiver
/// let (observer, receiver) = OneshotEventObserver::<MessageCreate>::new();
///
/// // Subscribe the observer, so it receives events
/// // Note that we clone the reference so we can later unsubscribe the observer
/// handle.events.lock().await.message.create.subscribe(observer.clone());
///
/// // Await the event
/// let result = receiver.await;
///
/// match result {
///   Ok(event) => {
///      println!("Yay! we received the event!");
///   }
///   Err(e) => {
///      println!("We sadly encountered an error: {:?}", e);
///   }
/// }
///
/// // The observer has now served its purpose, unsubscribe it
/// handle.events.lock().await.message.create.unsubscribe(observer);
///
/// // Since we dropped all the references to the observer,
/// // it is now deleted
/// # tests::common::teardown(bundle).await;
/// # })
/// ```
///
/// We can also use [tokio::select] to await with a timeout:
///
/// ```no_run
/// # tokio_test::block_on(async {
/// # mod tests::common;
/// # let mut bundle = common::setup().await;
/// use chorus::gateway::{GatewayHandle, OneshotEventObserver};
/// use chorus::types::MessageCreate;
/// use std::time::Duration;
///
/// #[cfg(not(target_arch = "wasm32"))]
/// use tokio::time::sleep;
/// #[cfg(target_arch = "wasm32")]
/// use wasmtimer::tokio::sleep;
///
/// let handle: GatewayHandle; // Get this either by user.gateway or by manually opening a connection
/// # let handle = bundle.user.gateway;
///
/// // Let's say we want to wait until the next MessageCreate event, if it happens in the next 10 seconds
/// // Create the observer and receiver
/// let (observer, receiver) = OneshotEventObserver::<MessageCreate>::new();
///
/// // Subscribe the observer, so it receives events
/// // Note that we clone the reference so we can later unsubscribe the observer
/// handle.events.lock().await.message.create.subscribe(observer.clone());
///
/// tokio::select! {
///   () = sleep(Duration::from_secs(10)) => {
///      // No event happened in 10 seconds
///   }
///   result = receiver => {
///      match result {
///         Ok(event) => {
///            println!("Yay! we received the event!");
///         }
///         Err(e) => {
///            println!("We sadly encountered an error: {:?}", e);
///         }
///      }
///   }
/// }
///
/// // The observer has now served its purpose, unsubscribe it
/// handle.events.lock().await.message.create.unsubscribe(observer);
///
/// // Since we dropped all the references to the observer,
/// // it is now deleted
/// # tests::common::teardown(bundle).await;
/// # })
/// ```
#[derive(Debug)]
pub struct OneshotEventObserver<T>
where
    T: WebSocketEvent + Clone,
{
    pub sender: Mutex<Option<tokio::sync::oneshot::Sender<T>>>,
}

impl<T: WebSocketEvent + Clone> OneshotEventObserver<T> {
    /// Creates a new [OneshotEventObserver] for the given event
    ///
    /// You must subscribe it to your gateway event; after receiving on the channel,
    /// you should unsubscribe it if possible
    pub fn new() -> (
        Arc<OneshotEventObserver<T>>,
        tokio::sync::oneshot::Receiver<T>,
    ) {
        let (sender, receiver) = tokio::sync::oneshot::channel();

        let observer = Arc::new(OneshotEventObserver {
            sender: Mutex::new(Some(sender)),
        });

        (observer, receiver)
    }
}

#[async_trait]
impl<T> Subscriber<T> for OneshotEventObserver<T>
where
    T: WebSocketEvent + Clone,
{
    async fn update(&self, message: &T) {
        let mut lock = self.sender.lock().await;

        if lock.is_none() {
            warn!("OneshotEventObserver received event after closing channel!");
            return;
        }

        let sender = lock.take().unwrap();

        match sender.send(message.clone()) {
            Ok(_) => {}
            Err(e) => {
                warn!("OneshotEventObserver failed to send event: {:?}", e);
            }
        }
    }
}

/// Observes an event indefinitely and sends it via a [tokio::sync::broadcast] channel
///
/// # Examples
/// ```no_run
/// # tokio_test::block_on(async {
/// # mod tests::common;
/// # let mut bundle = common::setup().await;
/// use chorus::gateway::{GatewayHandle, BroadcastEventObserver};
/// use chorus::types::MessageCreate;
///
/// let handle: GatewayHandle; // Get this either by user.gateway or by manually opening a connection
/// # let handle = bundle.user.gateway;
///
/// // Let's say we want to wait for every MessageCreate event
/// // Create the observer and receiver
/// let (observer, receiver) = BroadcastEventObserver::<MessageCreate>::new();
///
/// // Subscribe the observer, so it receives events
/// // Note that we clone the reference so we can later unsubscribe the observer
/// handle.events.lock().await.message.create.subscribe(observer.clone());
///
/// loop {
///   let result = receiver.recv().await;
///
///   match result {
///      Ok(event) => {
///         println!("Yay! we received the event!");
///      }
///      Err(e) => {
///         println!("We sadly encountered an error: {:?}", e);
///         break;
///      }
///   }
/// }
///
/// // The observer has now served its purpose, unsubscribe it
/// handle.events.lock().await.message.create.unsubscribe(observer);
///
/// // Since we dropped all the references to the observer,
/// // it is now deleted
/// # tests::common::teardown(bundle).await;
/// # })
/// ```
///
/// We can also use [tokio::select] to await with a timeout:
///
/// ```no_run
/// # tokio_test::block_on(async {
/// # mod tests::common;
/// # let mut bundle = common::setup().await;
/// use chorus::gateway::{GatewayHandle, BroadcastEventObserver};
/// use chorus::types::MessageCreate;
/// use std::time::Duration;
///
/// #[cfg(not(target_arch = "wasm32"))]
/// use tokio::time::sleep;
/// #[cfg(target_arch = "wasm32")]
/// use wasmtimer::tokio::sleep;
///
/// let handle: GatewayHandle; // Get this either by user.gateway or by manually opening a connection
/// # let handle = bundle.user.gateway;
///
/// // Let's say we want to wait for every MessageCreate event, if it takes less than 10 seconds
/// // Create the observer and receiver
/// let (observer, receiver) = BroadcastEventObserver::<MessageCreate>::new();
///
/// // Subscribe the observer, so it receives events
/// // Note that we clone the reference so we can later unsubscribe the observer
/// handle.events.lock().await.message.create.subscribe(observer.clone());
///
/// loop {
///   tokio::select! {
///      () = sleep(Duration::from_secs(10)) => {
///         println!("Waited for 10 seconds with no message, stopping");
///         break;
///      }
///      result = receiver.recv() => {
///         match result {
///            Ok(event) => {
///               println!("Yay! we received the event!");
///            }
///            Err(e) => {
///               println!("We sadly encountered an error: {:?}", e);
///               break;
///            }
///         }
///      }
///   }
/// }
///
/// // The observer has now served its purpose, unsubscribe it
/// handle.events.lock().await.message.create.unsubscribe(observer);
///
/// // Since we dropped all the references to the observer,
/// // it is now deleted
/// # tests::common::teardown(bundle).await;
/// # })
/// ```
#[derive(Debug)]
pub struct BroadcastEventObserver<T>
where
    T: WebSocketEvent + Clone,
{
    pub sender: tokio::sync::broadcast::Sender<T>,
}

impl<T: WebSocketEvent + Clone> BroadcastEventObserver<T> {
    /// Creates a new [BroadcastEventObserver] for the given event
    ///
    /// You must subscribe it to your gateway event
    pub fn new(
        channel_size: usize,
    ) -> (
        Arc<BroadcastEventObserver<T>>,
        tokio::sync::broadcast::Receiver<T>,
    ) {
        let (sender, receiver) = tokio::sync::broadcast::channel(channel_size);

        let observer = Arc::new(BroadcastEventObserver { sender });

        (observer, receiver)
    }
}

#[async_trait]
impl<T> Subscriber<T> for BroadcastEventObserver<T>
where
    T: WebSocketEvent + Clone,
{
    async fn update(&self, message: &T) {
        match self.sender.send(message.clone()) {
            Ok(_) => {}
            Err(e) => {
                warn!("BroadcastEventObserver failed to send event: {:?}", e);
            }
        }
    }
}
