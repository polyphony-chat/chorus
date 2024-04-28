// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// This example showcases how to use the voice udp channel.
//
// To use this to properly communicate with voice, you will need to bring your own opus bindings
// along with potentially sending some other events, like Speaking
//
// To properly run this example, you will need to change some values below,
// like the token, guild and channel ids.

const TOKEN: &str = "";

const VOICE_GUILD_ID: Option<Snowflake> = None;
const VOICE_CHANNEL_ID: Option<Snowflake> = Some(Snowflake(0_u64));

const GATEWAY_URL: &str = "wss://gateway.discord.gg";

use async_trait::async_trait;
use simplelog::{TermLogger, Config, WriteLogger};
use std::{net::SocketAddrV4, sync::Arc, fs::File, time::Duration};

use chorus::{
    gateway::{Observer, Gateway},
    types::{
        GatewayReady, SelectProtocol, SelectProtocolData, SessionDescription, Snowflake, Speaking,
        SpeakingBitflags, SsrcDefinition, VoiceEncryptionMode, VoiceIdentify, VoiceProtocol,
        VoiceReady, VoiceServerUpdate, GatewayIdentifyPayload, UpdateVoiceState,
    },
    voice::{
        gateway::{VoiceGateway, VoiceGatewayHandle},
        udp::{UdpHandle, UdpHandler},
        voice_data::VoiceData,
    },
};
use log::{info, LevelFilter};
use tokio::sync::{Mutex, RwLock};

extern crate chorus;
extern crate tokio;

/// Handles in between connections between the gateway and UDP modules
#[derive(Debug, Clone)]
pub struct VoiceHandler {
    pub voice_gateway_connection: Arc<Mutex<Option<VoiceGatewayHandle>>>,
    pub voice_udp_connection: Arc<Mutex<Option<UdpHandle>>>,
    pub data: Arc<RwLock<VoiceData>>,
}

impl VoiceHandler {
    /// Creates a new [VoiceHandler], only initializing the data
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
// On [VoiceServerUpdate] we get our starting data and URL for the voice gateway server.
impl Observer<VoiceServerUpdate> for VoiceHandler {
    async fn update(&self, data: &VoiceServerUpdate) {
        let mut data_lock = self.data.write().await;

        data_lock.server_data = Some(data.clone());
        let user_id = data_lock.user_id;
        let session_id = data_lock.session_id.clone();

        drop(data_lock);

        // Create and connect to the voice gateway
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

        // Subscribe to voice gateway events
        voice_events.voice_ready.subscribe(self_reference.clone());
        voice_events
            .session_description
            .subscribe(self_reference.clone());
        voice_events.speaking.subscribe(self_reference.clone());
        voice_events
            .ssrc_definition
            .subscribe(self_reference.clone());

        *self.voice_gateway_connection.lock().await = Some(voice_gateway_handle);
    }
}

#[async_trait]
// On [VoiceReady] we get info for establishing a UDP connection, and we immediately need said UDP
// connection for ip discovery.
impl Observer<VoiceReady> for VoiceHandler {
    async fn update(&self, data: &VoiceReady) {
        let mut data_lock = self.data.write().await;

        data_lock.ready_data = Some(data.clone());

        drop(data_lock);

        // Create a udp connection and perform ip discovery
        let udp_handle = UdpHandler::spawn(
            self.data.clone(),
            std::net::SocketAddr::V4(SocketAddrV4::new(data.ip, data.port)),
            data.ssrc,
        )
        .await
        .unwrap();

        // Subscribe ourself to receiving rtp data
        udp_handle
            .events
            .lock()
            .await
            .rtp
            .subscribe(Arc::new(self.clone()));

        let ip_discovery = self.data.read().await.ip_discovery.clone().unwrap();

        *self.voice_udp_connection.lock().await = Some(udp_handle.clone());

        let string_ip_address =
            String::from_utf8(ip_discovery.address).expect("Ip discovery gave non string ip");

        // Send a select protocol, which tells the server where we'll be receiving data and what
        // mode to encrypt data in
        self.voice_gateway_connection
            .lock()
            .await
            .clone()
            .unwrap()
            .send_select_protocol(SelectProtocol {
                protocol: VoiceProtocol::Udp,
                data: SelectProtocolData {
                    address: string_ip_address,
                    port: ip_discovery.port,
                    // There are several other voice encryption modes available, though not all are
                    // implemented in chorus
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
// Ready is used just to obtain some info, like the user id and session id
impl Observer<GatewayReady> for VoiceHandler {
    async fn update(&self, data: &GatewayReady) {
        let mut lock = self.data.write().await;
        lock.user_id = data.user.id;
        lock.session_id = data.session_id.clone();
        drop(lock);
    }
}

#[async_trait]
// This is the received voice data
impl Observer<chorus::voice::discortp::rtp::Rtp> for VoiceHandler {
    async fn update(&self, data: &chorus::voice::discortp::rtp::Rtp) {
        info!(
            "Received decrypted voice data! {:?} (SSRC: {})",
            data.payload.clone(),
            data.ssrc,
        );
    }
}

#[async_trait]
// This event gives extra info about who is speaking
impl Observer<Speaking> for VoiceHandler {
    async fn update(&self, data: &Speaking) {
        println!(
            "Received Speaking! (SRRC: {}, flags: {:?})",
            data.ssrc,
            SpeakingBitflags::from_bits(data.speaking).unwrap()
        );
    }
}

#[async_trait]
// This event gives some info about which user has which ssrc
impl Observer<SsrcDefinition> for VoiceHandler {
    async fn update(&self, data: &SsrcDefinition) {
        println!(
            "Received SSRC Definition! (User {} has audio ssrc {})",
            data.user_id.unwrap(),
            data.audio_ssrc
        );
    }
}

#[tokio::main]
async fn main() {
    simplelog::CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Debug,
            Config::default(),
            simplelog::TerminalMode::Mixed,
            simplelog::ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Trace,
            Config::default(),
            File::create("latest.log").unwrap(),
        ),
    ])
    .unwrap();

    let gateway = Gateway::spawn(GATEWAY_URL.to_string())
        .await
        .unwrap();

    let mut identify = GatewayIdentifyPayload::common();
    identify.token = TOKEN.to_string();

    gateway.send_identify(identify).await;

    let voice_handler = Arc::new(VoiceHandler::new());

    // Voice handler needs voice server update
    gateway
        .events
        .lock()
        .await
        .voice
        .server_update
        .subscribe(voice_handler.clone());

    // It also needs a bit of the data in ready
    gateway
        .events
        .lock()
        .await
        .session
        .ready
        .subscribe(voice_handler.clone());
    
    // Data which channel to update the local user to be joined into.
    //
    // guild_id and channel_id can be some to join guild voice channels
    //
    // guild_id can be none and channel id some to join dm calls
    //
    // both can be none to leave all voice channels
    let voice_state_update = UpdateVoiceState {
        guild_id: VOICE_GUILD_ID,
        channel_id: VOICE_CHANNEL_ID,
        self_mute: false,
        self_deaf: false,
        ..Default::default()
    };

    gateway.send_update_voice_state(voice_state_update).await;

    loop {
        tokio::time::sleep(Duration::from_millis(1000)).await;

        // Potentially send some data here
        /*let voice_udp_option = voice_handler.voice_udp_connection.lock().await.clone();
        if voice_udp_option.is_some() {
            voice_udp_option.unwrap().send_opus_data(0, vec![1, 2, 3, 4, 5]).await.unwrap();
        }*/
    }
}
