// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::sync::Arc;

use log::*;

use futures_util::SinkExt;

use serde_json::json;
use tokio::sync::Mutex;

use crate::types::{
    SelectProtocol, Speaking, SsrcDefinition, VoiceGatewaySendPayload, VoiceIdentify,
    VOICE_BACKEND_VERSION, VOICE_IDENTIFY, VOICE_SELECT_PROTOCOL, VOICE_SPEAKING,
    VOICE_SSRC_DEFINITION,
};

use super::{events::VoiceEvents, Sink, VoiceGatewayMessage};

/// Represents a handle to a Voice Gateway connection.
/// Using this handle you can send Gateway Events directly.
#[derive(Debug, Clone)]
pub struct VoiceGatewayHandle {
    pub url: String,
    pub events: Arc<Mutex<VoiceEvents>>,
    pub websocket_send: Arc<Mutex<Sink>>,
    /// Tells gateway tasks to close
    pub(super) kill_send: tokio::sync::broadcast::Sender<()>,
}

impl VoiceGatewayHandle {
    /// Sends json to the gateway with an opcode
    async fn send_json(&self, op_code: u8, to_send: serde_json::Value) {
        let gateway_payload = VoiceGatewaySendPayload {
            op_code,
            data: to_send,
        };

        let payload_json = serde_json::to_string(&gateway_payload).unwrap();
        let message = VoiceGatewayMessage(payload_json);

        self.websocket_send
            .lock()
            .await
            .send(message.into())
            .await
            .unwrap();
    }

    /// Sends a voice identify event to the gateway
    pub async fn send_identify(&self, to_send: VoiceIdentify) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        trace!("VGW: Sending Identify..");

        self.send_json(VOICE_IDENTIFY, to_send_value).await;
    }

    /// Sends a select protocol event to the gateway
    pub async fn send_select_protocol(&self, to_send: SelectProtocol) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        trace!("VGW: Sending Select Protocol");

        self.send_json(VOICE_SELECT_PROTOCOL, to_send_value).await;
    }

    /// Sends a speaking event to the gateway
    pub async fn send_speaking(&self, to_send: Speaking) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        trace!("VGW: Sending Speaking");

        self.send_json(VOICE_SPEAKING, to_send_value).await;
    }

    /// Sends an ssrc definition event
    pub async fn send_ssrc_definition(&self, to_send: SsrcDefinition) {
        let to_send_value = serde_json::to_value(&to_send).unwrap();

        trace!("VGW: Sending SsrcDefinition");

        self.send_json(VOICE_SSRC_DEFINITION, to_send_value).await;
    }

    /// Sends a voice backend version request to the gateway
    pub async fn send_voice_backend_version_request(&self) {
        let data_empty_object = json!("{}");

        trace!("VGW: Requesting voice backend version");

        self.send_json(VOICE_BACKEND_VERSION, data_empty_object)
            .await;
    }

    /// Closes the websocket connection and stops all gateway tasks;
    ///
    /// Essentially pulls the plug on the voice gateway, leaving it possible to resume;
    pub async fn close(&self) {
        self.kill_send.send(()).unwrap();
        self.websocket_send.lock().await.close().await.unwrap();
    }
}
