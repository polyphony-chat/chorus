// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Includes pre-made observers to use with gateway events

use crate::types::WebSocketEvent;
use async_trait::async_trait;
use log::warn;
use pubserve::Subscriber;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Observes an event once and sends it via a [tokio::sync::oneshot] channel
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
