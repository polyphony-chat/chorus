// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::sync::Arc;

use crypto_secretbox::{
    aead::Aead, cipher::generic_array::GenericArray, KeyInit, XSalsa20Poly1305,
};
use discortp::Packet;

use getrandom::getrandom;
use log::*;

use tokio::{sync::Mutex, sync::RwLock};

use super::UdpSocket;

use crate::{
    errors::VoiceUdpError,
    types::VoiceEncryptionMode,
    voice::{crypto::get_xsalsa20_poly1305_nonce, voice_data::VoiceData},
};

use super::{events::VoiceUDPEvents, RTP_HEADER_SIZE};

/// Handle to a voice UDP connection
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
    /// Automatically makes an [RtpPacket](discortp::rtp::RtpPacket), encrypts it and sends it.
    ///
    /// # Errors
    /// If we do not have VoiceReady data, which contains our ssrc, this returns a
    /// [VoiceUdpError::NoData] error.
    ///
    /// If we have not received an encryption key, this returns a [VoiceUdpError::NoKey] error.
    ///
    /// If the UDP socket is broken, this returns a [VoiceUdpError::BrokenSocket] error.
    pub async fn send_opus_data(
        &self,
        timestamp: u32,
        payload: Vec<u8>,
    ) -> Result<(), VoiceUdpError> {
        let voice_ready_data_result = self.data.read().await.ready_data.clone();
        if voice_ready_data_result.is_none() {
            return Err(VoiceUdpError::NoData);
        }

        let ssrc = voice_ready_data_result.unwrap().ssrc;
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

        let mut rtp_packet = discortp::rtp::MutableRtpPacket::new(&mut buffer).expect("Mangled rtp packet creation buffer, something is very wrong. Please open an issue on the chorus github: https://github.com/polyphony-chat/chorus/issues/new");
        rtp_packet.populate(&rtp_data);

        self.send_rtp_packet(rtp_packet).await
    }

    /// Encrypts and sends and rtp packet.
    ///
    /// # Errors
    /// If we have not received an encryption key, this returns a [VoiceUdpError::NoKey] error.
    ///
    /// If the Udp socket is broken, this returns a [VoiceUdpError::BrokenSocket] error.
    pub async fn send_rtp_packet(
        &self,
        packet: discortp::rtp::MutableRtpPacket<'_>,
    ) -> Result<(), VoiceUdpError> {
        let mut buffer = self.encrypt_rtp_packet_payload(&packet).await?;
        let new_packet = discortp::rtp::MutableRtpPacket::new(&mut buffer).unwrap();
        self.send_encrypted_rtp_packet(new_packet.consume_to_immutable())
            .await?;
        Ok(())
    }

    /// Encrypts an unencrypted rtp packet, returning a copy of the packet's bytes with an
    /// encrypted payload
    ///
    /// # Errors
    /// If we have not received an encryption key, this returns a [VoiceUdpError::NoKey] error.
    ///
    /// When using voice encryption modes which require special nonce generation, and said generation fails, this returns a [VoiceUdpError::FailedNonceGeneration] error.
    pub async fn encrypt_rtp_packet_payload(
        &self,
        packet: &discortp::rtp::MutableRtpPacket<'_>,
    ) -> Result<Vec<u8>, VoiceUdpError> {
        let payload = packet.payload();

        let session_description_result = self.data.read().await.session_description.clone();

        // We are trying to encrypt, but have not received SessionDescription yet,
        // which contains the secret key.
        if session_description_result.is_none() {
            return Err(VoiceUdpError::NoKey);
        }

        let session_description = session_description_result.unwrap();

        let mut nonce_bytes = match session_description.encryption_mode {
            VoiceEncryptionMode::Xsalsa20Poly1305 => get_xsalsa20_poly1305_nonce(packet.packet()),
            VoiceEncryptionMode::Xsalsa20Poly1305Suffix => {
                // Generate 24 random bytes
                let mut random_destinaton: Vec<u8> = vec![0; 24];
                let random_result = getrandom(&mut random_destinaton);
                if let Err(e) = random_result {
                    return Err(VoiceUdpError::FailedNonceGeneration {
                        error: format!("{:?}", e),
                    });
                }
                random_destinaton
            }
            VoiceEncryptionMode::Xsalsa20Poly1305Lite => {
                // "Incremental 4 bytes (32bit) int value"
                let mut data_lock = self.data.write().await;
                let nonce = data_lock
                    .last_udp_encryption_nonce
                    .unwrap_or_default()
                    .wrapping_add(1);

                data_lock.last_udp_encryption_nonce = Some(nonce);
                drop(data_lock);
                // TODO: Is big endian correct? This is not documented anywhere
                let mut bytes = nonce.to_be_bytes().to_vec();

                // This is 4 bytes, it has to be a different size, appends 0s
                while bytes.len() < 24 {
                    bytes.push(0);
                }
                bytes
            }
            _ => {
                error!(
                    "This voice encryption mode ({:?}) is not yet implemented.",
                    session_description.encryption_mode
                );
                return Err(VoiceUdpError::EncryptionModeNotImplemented {
                    encryption_mode: format!("{:?}", session_description.encryption_mode),
                });
            }
        };

        let key = GenericArray::from_slice(&session_description.secret_key);

        let encryption_result;

        if session_description.encryption_mode.is_xsalsa20_poly1305() {
            let nonce = GenericArray::from_slice(&nonce_bytes);

            let encryptor = XSalsa20Poly1305::new(key);

            encryption_result = encryptor.encrypt(nonce, payload);
        }
        // Note: currently unused because I have no idea what the AeadAes256Gcm nonce is
        /*else if session_description.encryption_mode.is_aead_aes256_gcm() {
            let nonce = GenericArray::from_slice(&nonce_bytes);

            let encryptor = Aes256Gcm::new(key);

            encryption_result = encryptor.encrypt(nonce, payload);

        }*/
        else {
            error!(
                "This voice encryption mode ({:?}) is not yet implemented.",
                session_description.encryption_mode
            );
            return Err(VoiceUdpError::EncryptionModeNotImplemented {
                encryption_mode: format!("{:?}", session_description.encryption_mode),
            });
        }

        if encryption_result.is_err() {
            // Safety: If encryption fails here, it's chorus' fault, and it makes no sense to
            // return the error to the user.
            //
            // This is not an error the user should account for, which is why we throw it here.
            panic!("{}", VoiceUdpError::FailedEncryption);
        }

        let mut encrypted_payload = encryption_result.unwrap();

        // Append the nonce bytes, if needed
        // All other encryption modes have an explicit nonce, whereas Xsalsa20Poly1305
        // has the nonce as the rtp header.
        if session_description.encryption_mode != VoiceEncryptionMode::Xsalsa20Poly1305 {
            encrypted_payload.append(&mut nonce_bytes);
        }

        // We need to allocate a new buffer, since the old one is too small for our new encrypted
        // data
        let buffer_size = encrypted_payload.len() + RTP_HEADER_SIZE as usize;

        let mut new_buffer: Vec<u8> = Vec::with_capacity(buffer_size);

        let mut rtp_header = packet.packet().to_vec()[0..RTP_HEADER_SIZE as usize].to_vec();

        new_buffer.append(&mut rtp_header);
        new_buffer.append(&mut encrypted_payload);

        Ok(new_buffer)
    }

    /// Sends an (already encrypted) rtp packet to the connection.
    ///
    /// # Errors
    /// If the Udp socket is broken, this returns a [VoiceUdpError::BrokenSocket] error.
    pub async fn send_encrypted_rtp_packet(
        &self,
        packet: discortp::rtp::RtpPacket<'_>,
    ) -> Result<(), VoiceUdpError> {
        let raw_bytes = packet.packet();

        let send_res = self.socket.send(raw_bytes).await;
        if let Err(e) = send_res {
            return Err(VoiceUdpError::BrokenSocket {
                error: format!("{:?}", e),
            });
        }

        trace!("VUDP: Sent rtp packet!");

        Ok(())
    }
}
