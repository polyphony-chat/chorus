use std::{net::SocketAddrV4, sync::Arc};

use async_trait::async_trait;
use tokio::sync::{Mutex, RwLock};

use crate::{
    gateway::Observer,
    types::{
        GatewayReady, SelectProtocol, SelectProtocolData, SessionDescription, Snowflake,
        VoiceEncryptionMode, VoiceIdentify, VoiceProtocol, VoiceReady, VoiceServerUpdate,
    },
};

use super::{
    gateway::{VoiceGateway, VoiceGatewayHandle},
    udp::UdpHandle,
    udp::UdpHandler,
    voice_data::VoiceData,
};

/// Handles inbetween connections between the gateway and udp modules
#[derive(Debug, Clone)]
pub struct VoiceHandler {
    pub voice_gateway_connection: Arc<Mutex<Option<VoiceGatewayHandle>>>,
    pub voice_udp_connection: Arc<Mutex<Option<UdpHandle>>>,
    pub data: Arc<RwLock<VoiceData>>,
}

impl VoiceHandler {
    /// Creates a new voicehandler, only initializing the data
    pub fn new() -> VoiceHandler {
        Self {
            data: Arc::new(RwLock::new(VoiceData::default())),
            voice_gateway_connection: Arc::new(Mutex::new(None)),
            voice_udp_connection: Arc::new(Mutex::new(None)),
        }
    }
}

impl Default for VoiceHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
// On [VoiceServerUpdate] we get our starting data and url for the voice gateway server.
impl Observer<VoiceServerUpdate> for VoiceHandler {
    async fn update(&self, data: &VoiceServerUpdate) {
        let mut data_lock = self.data.write().await;
        data_lock.server_data = Some(data.clone());
        let user_id = data_lock.user_id;
        let session_id = data_lock.session_id.clone();
        drop(data_lock);

        let voice_gateway_handle = VoiceGateway::spawn(data.endpoint.clone().unwrap())
            .await
            .unwrap();

        let server_id: Snowflake;

        if data.guild_id.is_some() {
            server_id = data.guild_id.unwrap();
        } else {
            server_id = data.channel_id.unwrap();
        }

        let voice_identify = VoiceIdentify {
            server_id,
            user_id,
            session_id,
            token: data.token.clone(),
            video: Some(false),
        };

        voice_gateway_handle.send_identify(voice_identify).await;

        let cloned_gateway_handle = voice_gateway_handle.clone();

        let mut voice_events = cloned_gateway_handle.events.lock().await;

        let self_reference = Arc::new(self.clone());

        voice_events.voice_ready.subscribe(self_reference.clone());
        voice_events
            .session_description
            .subscribe(self_reference.clone());

        *self.voice_gateway_connection.lock().await = Some(voice_gateway_handle);
    }
}

#[async_trait]
// On [VoiceReady] we get info for establishing a UDP connection, and we immedietly need said UDP
// connection for ip discovery.
impl Observer<VoiceReady> for VoiceHandler {
    async fn update(&self, data: &VoiceReady) {
        let mut data_lock = self.data.write().await;
        data_lock.ready_data = Some(data.clone());
        drop(data_lock);

        let udp_handle = UdpHandler::spawn(
            self.data.clone(),
            std::net::SocketAddr::V4(SocketAddrV4::new(data.ip, data.port)),
            data.ssrc,
        )
        .await;

        let ip_discovery = self.data.read().await.ip_discovery.clone().unwrap();

        *self.voice_udp_connection.lock().await = Some(udp_handle.clone());

        self.voice_gateway_connection
            .lock()
            .await
            .clone()
            .unwrap()
            .send_select_protocol(SelectProtocol {
                protocol: VoiceProtocol::Udp,
                data: SelectProtocolData {
                    address: ip_discovery.address,
                    port: ip_discovery.port,
                    mode: VoiceEncryptionMode::Xsalsa20Poly1305,
                },
                ..Default::default()
            })
            .await;
    }
}

#[async_trait]
// Session descryption gives us final info regarding codecs and our encryption key
impl Observer<SessionDescription> for VoiceHandler {
    async fn update(&self, data: &SessionDescription) {
        let mut data_write = self.data.write().await;

        data_write.session_description = Some(data.clone());

        drop(data_write);
    }
}

#[async_trait]
impl Observer<GatewayReady> for VoiceHandler {
    async fn update(&self, data: &GatewayReady) {
        let mut lock = self.data.write().await;
        lock.user_id = data.user.id;
        lock.session_id = data.session_id.clone();
        drop(lock);
    }
}
