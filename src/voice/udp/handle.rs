use std::sync::Arc;

use crypto_secretbox::{
    aead::Aead, cipher::generic_array::GenericArray, KeyInit, XSalsa20Poly1305,
};
use discortp::Packet;

use log::*;

use tokio::{net::UdpSocket, sync::Mutex, sync::RwLock};

use crate::voice::{crypto, voice_data::VoiceData};

use super::{events::VoiceUDPEvents, RTP_HEADER_SIZE};

/// Handle to a voice udp connection
///
/// Can be safely cloned and will still correspond to the same connection.
#[derive(Debug, Clone)]
pub struct UdpHandle {
    pub events: Arc<Mutex<VoiceUDPEvents>>,
    pub(super) socket: Arc<UdpSocket>,
    pub data: Arc<RwLock<VoiceData>>,
}

impl UdpHandle {
    /// Constructs and sends encoded opus rtp data.
    ///
    /// Automatically makes an [RtpPacket](discorrtp::rtp::RtpPacket), encrypts it and sends it.
    pub async fn send_opus_data(&self, timestamp: u32, payload: Vec<u8>) {
        let ssrc = self.data.read().await.ready_data.clone().unwrap().ssrc;
        let sequence_number = self.data.read().await.last_sequence_number.wrapping_add(1);
        self.data.write().await.last_sequence_number = sequence_number;

        let payload_len = payload.len();

        let rtp_data = discortp::rtp::Rtp {
            // Always the same
            version: 2,
            padding: 0,
            extension: 0,
            csrc_count: 0,
            csrc_list: Vec::new(),
            marker: 0,
            payload_type: discortp::rtp::RtpType::Dynamic(120),
            // Actually variable
            sequence: sequence_number.into(),
            timestamp: timestamp.into(),
            ssrc,
            payload,
        };

        let buffer_size = payload_len + RTP_HEADER_SIZE as usize;

        let mut buffer = vec![0; buffer_size];

        let mut rtp_packet = discortp::rtp::MutableRtpPacket::new(&mut buffer).unwrap();
        rtp_packet.populate(&rtp_data);

        self.send_rtp_packet(rtp_packet).await;
    }

    /// Encrypts and sends and rtp packet.
    pub async fn send_rtp_packet(&self, packet: discortp::rtp::MutableRtpPacket<'_>) {
        let mut buffer = self.encrypt_rtp_packet_payload(&packet).await;
        let new_packet = discortp::rtp::MutableRtpPacket::new(&mut buffer).unwrap();
        self.send_encrypted_rtp_packet(new_packet.consume_to_immutable())
            .await;
    }

    /// Encrypts an unencrypted rtp packet, returning a copy of the packet's bytes with an
    /// encrypted payload
    pub async fn encrypt_rtp_packet_payload(
        &self,
        packet: &discortp::rtp::MutableRtpPacket<'_>,
    ) -> Vec<u8> {
        let payload = packet.payload();

        let session_description_result = self.data.read().await.session_description.clone();

        if session_description_result.is_none() {
            // FIXME: Make this function reutrn a result with a proper error type for these kinds
            // of functions
            panic!("Trying to encrypt packet but no key provided yet");
        }

        let session_description = session_description_result.unwrap();

        let nonce_bytes = crypto::get_xsalsa20_poly1305_nonce(packet.packet());
        let nonce = GenericArray::from_slice(&nonce_bytes);

        let key = GenericArray::from_slice(&session_description.secret_key);

        let encryptor = XSalsa20Poly1305::new(key);

        let encryption_result = encryptor.encrypt(nonce, payload);

        if encryption_result.is_err() {
            // FIXME: See above fixme
            panic!("Encryption error");
        }

        let mut encrypted_payload = encryption_result.unwrap();

        // We need to allocate a new buffer, since the old one is too small for our new encrypted
        // data
        let buffer_size = encrypted_payload.len() + RTP_HEADER_SIZE as usize;

        let mut new_buffer: Vec<u8> = Vec::with_capacity(buffer_size);

        let mut rtp_header = packet.packet().to_vec()[0..RTP_HEADER_SIZE as usize].to_vec();

        new_buffer.append(&mut rtp_header);
        new_buffer.append(&mut encrypted_payload);

        new_buffer
    }

    /// Sends an (already encrypted) rtp packet to the connection.
    pub async fn send_encrypted_rtp_packet(&self, packet: discortp::rtp::RtpPacket<'_>) {
        let raw_bytes = packet.packet();

        self.socket.send(raw_bytes).await.unwrap();

        debug!("VUDP: Sent rtp packet!");
    }
}
