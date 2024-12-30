// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use futures_util::SinkExt;
use log::*;

use std::fmt::Debug;

use super::{events::Events, *};
use crate::types::{self, Composite, GuildMembersChunk, Opcode, Shared, VoiceStateUpdate};

#[cfg(not(target_arch = "wasm32"))]
use tokio::time::sleep;
#[cfg(target_arch = "wasm32")]
use wasmtimer::tokio::sleep;

/// Represents a handle to a Gateway connection.
///
/// A Gateway connection will create observable [`Events`], which you can subscribe to.
///
/// Using this handle you can also send Gateway Events directly.
#[derive(Debug, Clone)]
pub struct GatewayHandle {
    pub url: String,
    pub events: Arc<Mutex<Events>>,
    pub websocket_send: Arc<Mutex<Sink>>,
    /// Tells gateway tasks to close
    pub(super) kill_send: tokio::sync::broadcast::Sender<()>,
    pub(crate) store: Arc<Mutex<HashMap<Snowflake, Arc<RwLock<ObservableObject>>>>>,
}

impl GatewayHandle {
    /// Sends json to the gateway with an opcode
    async fn send_json_event(&self, op_code: u8, to_send: serde_json::Value) {
        let gateway_payload = types::GatewaySendPayload {
            op_code,
            event_data: Some(to_send),
            sequence_number: None,
        };

        let payload_json = serde_json::to_string(&gateway_payload).unwrap();
        let message = GatewayMessage(payload_json);

        self.websocket_send
            .lock()
            .await
            .send(message.into())
            .await
            .unwrap();
    }

    /// Recursively observes a [`Shared`] object, by making sure all [`Composite `] fields within
    /// that object and its children are being watched.
    ///
    /// Observing means, that if new information arrives about the observed object or its children,
    /// the object automatically gets updated, without you needing to request new information about
    /// the object in question from the API, which is expensive and can lead to rate limiting.
    ///
    /// The [`Shared`] object returned by this method points to a different object than the one
    /// being supplied as a &self function argument.
    pub async fn observe<T: Updateable + Clone + Debug + Composite<T>>(
        &self,
        object: Shared<T>,
    ) -> Shared<T> {
        let mut store = self.store.lock().await;
        let id = object.read().unwrap().id();
        if let Some(channel) = store.get(&id) {
            let object = channel.clone();
            drop(store);
            object
                .read()
                .unwrap()
                .downcast_ref::<T>()
                .unwrap_or_else(|| {
                    panic!(
                        "Snowflake {} already exists in the store, but it is not of type T.",
                        id
                    )
                });
            let ptr = Arc::into_raw(object.clone());
            // SAFETY:
            // - We have just checked that the typeid of the `dyn Any ...` matches that of `T`.
            // - This operation doesn't read or write any shared data, and thus cannot cause a data race
            // - The reference count is not being modified
            let downcasted = unsafe { Arc::from_raw(ptr as *const RwLock<T>).clone() };
            let object = downcasted.read().unwrap().clone();

            let watched_object = object.watch_whole(self).await;
            *downcasted.write().unwrap() = watched_object;
            downcasted
        } else {
            let id = object.read().unwrap().id();
            let object = object.read().unwrap().clone();
            let object = object.clone().watch_whole(self).await;
            let wrapped = Arc::new(RwLock::new(object));
            store.insert(id, wrapped.clone());
            wrapped
        }
    }

    /// Recursively observes and updates all updateable fields on the struct T. Returns an object `T`
    /// with all of its observable fields being observed.
    pub async fn observe_and_into_inner<T: Updateable + Clone + Debug + Composite<T>>(
        &self,
        object: Shared<T>,
    ) -> T {
        let channel = self.observe(object.clone()).await;
        let object = channel.read().unwrap().clone();
        object
    }

    /// Sends an identify event ([types::GatewayIdentifyPayload]) to the gateway
    ///
    /// Fires off a [types::GatewayReady] event
    pub async fn send_identify(&self, to_send: types::GatewayIdentifyPayload) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        trace!("GW: Sending Identify..");

        self.send_json_event(Opcode::Identify as u8, to_send_value)
            .await;
    }

    /// Sends an identify event ([types::GatewayIdentifyPayload]) to the gateway and
    /// waits to receive a [types::GatewayReady] event.
    ///
    /// Returns [GatewayError::NoResponse] if the server sends no response after 5 seconds of
    /// waiting
    pub async fn identify(
        &self,
        to_send: types::GatewayIdentifyPayload,
    ) -> Result<types::GatewayReady, GatewayError> {
        self.send_identify(to_send).await;

        let (observer, receiver) = OneshotEventObserver::<types::GatewayReady>::new();

        self.events
            .lock()
            .await
            .session
            .ready
            .subscribe(observer.clone());

        loop {
            tokio::select! {
                  () = sleep(std::time::Duration::from_secs(5)) => {
                      // Timeout
                      self.events.lock().await.session.ready.unsubscribe(observer);
                      return Err(GatewayError::NoResponse);
                  }
                  result = receiver => {
                      match result {
                          Ok(event) => {
                              self.events.lock().await.session.ready.unsubscribe(observer);
                              return Ok(event);
                          }
                          Err(e) => {
                              warn!("Gateway in-place-events receive error: {:?}", e);
                              self.events.lock().await.session.ready.unsubscribe(observer);
                              return Err(GatewayError::Unknown);
                          }
                      }
                  }
            }
        }
    }

    /// Sends a resume event ([types::GatewayResume]) to the gateway
    ///
    /// Fires off a [types::GatewayResumed] event after replaying missed events
    pub async fn send_resume(&self, to_send: types::GatewayResume) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        trace!("GW: Sending Resume..");

        self.send_json_event(Opcode::Resume as u8, to_send_value)
            .await;
    }

    /// Sends a resume event ([types::GatewayResume]) to the gateway and
    /// waits to receive a [types::GatewayResumed] event.
    ///
    /// Returns [GatewayError::NoResponse] if the server sends no response after 5 seconds of
    /// waiting
    pub async fn resume(
        &self,
        to_send: types::GatewayResume,
    ) -> Result<types::GatewayResumed, GatewayError> {
        self.send_resume(to_send).await;

        let (observer, receiver) = OneshotEventObserver::<types::GatewayResumed>::new();

        self.events
            .lock()
            .await
            .session
            .resumed
            .subscribe(observer.clone());

        loop {
            tokio::select! {
                  () = sleep(std::time::Duration::from_secs(5)) => {
                      // Timeout
                      self.events.lock().await.session.resumed.unsubscribe(observer);
                      return Err(GatewayError::NoResponse);
                  }
                  result = receiver => {
                      match result {
                          Ok(event) => {
                              self.events.lock().await.session.resumed.unsubscribe(observer);
                              return Ok(event);
                          }
                          Err(e) => {
                              warn!("Gateway in-place-events receive error: {:?}", e);
                              self.events.lock().await.session.resumed.unsubscribe(observer);
                              return Err(GatewayError::Unknown);
                          }
                      }
                  }
            }
        }
    }

    /// Sends an update presence event ([types::UpdatePresence]) to the gateway
    pub async fn send_update_presence(&self, to_send: types::UpdatePresence) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        trace!("GW: Sending Update Presence..");

        self.send_json_event(Opcode::PresenceUpdate as u8, to_send_value)
            .await;
    }

    /// Sends a request guild members ([types::GatewayRequestGuildMembers]) event to the server
    ///
    /// Fires off one or more [types::GuildMembersChunk]
    pub async fn send_request_guild_members(&self, to_send: types::GatewayRequestGuildMembers) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        trace!("GW: Sending Request Guild Members..");

        self.send_json_event(Opcode::RequestGuildMembers as u8, to_send_value)
            .await;
    }

    /// Sends a request guild members ([types::GatewayRequestGuildMembers]) event to the server and
    /// waits to receive all [types::GuildMembersChunk]s
    ///
    /// Returns [GatewayError::NoResponse] if the server sends no response after 5 seconds of
    /// waiting
    pub async fn request_guild_members(
        &self,
        to_send: types::GatewayRequestGuildMembers,
    ) -> Result<Vec<GuildMembersChunk>, GatewayError> {
        self.send_request_guild_members(to_send).await;

        let (observer, mut receiver) = BroadcastEventObserver::<GuildMembersChunk>::new(32);

        self.events
            .lock()
            .await
            .guild
            .members_chunk
            .subscribe(observer.clone());

        let mut chunks = Vec::new();

        loop {
            tokio::select! {
                  () = sleep(std::time::Duration::from_secs(5)) => {
                      // Timeout
                      self.events.lock().await.guild.members_chunk.unsubscribe(observer);
                      return Err(GatewayError::NoResponse);
                  }
                  result = receiver.recv() => {
                      match result {
                          Ok(event) => {
                              let remaining = event.chunk_count - (event.chunk_index + 1);

                              chunks.push(event);

                              if remaining < 1 {
                                  self.events.lock().await.guild.members_chunk.unsubscribe(observer);
                                  return Ok(chunks);
                              }
                          }
                          Err(e) => {
                              warn!("Gateway in-place-events receive error: {:?}", e);
                              self.events.lock().await.guild.members_chunk.unsubscribe(observer);
                              return Err(GatewayError::Unknown);
                          }
                      }
                  }
            }
        }
    }

    /// Sends an update voice state ([types::UpdateVoiceState]) event to the server
    ///
    /// Fires a [types::VoiceStateUpdate] event if the user left or joined a different channel
    pub async fn send_update_voice_state(&self, to_send: types::UpdateVoiceState) {
        let to_send_value = serde_json::to_value(to_send).unwrap();

        trace!("GW: Sending Update Voice State..");

        self.send_json_event(Opcode::VoiceStateUpdate as u8, to_send_value)
            .await;
    }

    /// Sends an update voice state ([types::UpdateVoiceState]) event to the server and
    /// waits to receive a [types::VoiceStateUpdate] event
    ///
    /// Returns [None] if the server sends no response after a second of
    /// waiting
    ///
    /// Note that not receiving a response is normal behaviour if the user didn't leave or join a
    /// new voice channel
    pub async fn update_voice_state(
        &self,
        to_send: types::UpdateVoiceState,
    ) -> Option<VoiceStateUpdate> {
        self.send_update_voice_state(to_send).await;

        let (observer, receiver) = OneshotEventObserver::<VoiceStateUpdate>::new();

        self.events
            .lock()
            .await
            .voice
            .state_update
            .subscribe(observer.clone());

        loop {
            tokio::select! {
                  () = sleep(std::time::Duration::from_secs(1)) => {
                      // Timeout
                      self.events.lock().await.voice.state_update.unsubscribe(observer);
                      return None;
                  }
                  result = receiver => {
                      match result {
                          Ok(event) => {
                              self.events.lock().await.voice.state_update.unsubscribe(observer);
                              return Some(event);
                          }
                          Err(e) => {
                              warn!("Gateway in-place-events receive error: {:?}", e);
                              self.events.lock().await.voice.state_update.unsubscribe(observer);
                              return None;
                          }
                      }
                  }
            }
        }
    }

    /// Sends a call sync ([types::CallSync]) to the server
    pub async fn send_call_sync(&self, to_send: types::CallSync) {
        let to_send_value = serde_json::to_value(to_send).unwrap();

        trace!("GW: Sending Call Sync..");

        self.send_json_event(Opcode::CallConnect as u8, to_send_value)
            .await;
    }

    /// Sends a request call connect event (aka [types::CallSync]) to the server
    ///
    /// # Notes
    /// Alias of [Self::send_call_sync]
    pub async fn send_request_call_connect(&self, to_send: types::CallSync) {
        self.send_call_sync(to_send).await
    }

    /// Sends a Lazy Request ([types::LazyRequest]) to the server
    pub async fn send_lazy_request(&self, to_send: types::LazyRequest) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        trace!("GW: Sending Lazy Request..");

        self.send_json_event(Opcode::GuildSubscriptions as u8, to_send_value)
            .await;
    }

    /// Sends a Request Last Messages ([types::RequestLastMessages]) to the server
    ///
    /// Fires off a [types::LastMessages] event
    pub async fn send_request_last_messages(&self, to_send: types::RequestLastMessages) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        trace!("GW: Sending Request Last Messages..");

        self.send_json_event(Opcode::RequestLastMessages as u8, to_send_value)
            .await;
    }

    /// Sends a Request Last Messages ([types::RequestLastMessages]) event to the server and
    /// waits to receive a [types::LastMessages] event
    ///
    /// Returns [None] if the server sends no response after 5 seconds of
    /// waiting
    pub async fn request_last_messages(
        &self,
        to_send: types::RequestLastMessages,
    ) -> Result<types::LastMessages, GatewayError> {
        self.send_request_last_messages(to_send).await;

        let (observer, receiver) = OneshotEventObserver::<types::LastMessages>::new();

        self.events
            .lock()
            .await
            .message
            .last_messages
            .subscribe(observer.clone());

        loop {
            tokio::select! {
                  () = sleep(std::time::Duration::from_secs(5)) => {
                      // Timeout
                      self.events.lock().await.message.last_messages.unsubscribe(observer);
                      return Err(GatewayError::NoResponse);
                  }
                  result = receiver => {
                      match result {
                          Ok(event) => {
                              self.events.lock().await.message.last_messages.unsubscribe(observer);
                              return Ok(event);
                          }
                          Err(e) => {
                              warn!("Gateway in-place-events receive error: {:?}", e);
                              self.events.lock().await.message.last_messages.unsubscribe(observer);
                              return Err(GatewayError::Unknown);
                          }
                      }
                  }
            }
        }
    }

    /// Closes the websocket connection and stops all gateway tasks.
    ///
    /// Essentially pulls the plug on the gateway, leaving it possible to resume
    pub async fn close(&self) {
        self.kill_send.send(()).unwrap();
        self.websocket_send.lock().await.close().await.unwrap();
    }
}
