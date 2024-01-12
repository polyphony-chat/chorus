//! Defines cryptography functions used within the voice implementation.
//!
//! All functions in this module return a 24 byte long [Vec<u8>].

/// Gets an xsalsa20_poly1305 nonce from an rtppacket.
///
/// See <https://discord-userdoccers.vercel.app/topics/voice-connections#encryption-mode>
pub(crate) fn get_xsalsa20_poly1305_nonce(packet: &[u8]) -> Vec<u8> {
    let mut rtp_header = Vec::with_capacity(24);
    rtp_header.append(&mut packet[0..12].to_vec());

    // The header is only 12 bytes, but the nonce has to be 24
    while rtp_header.len() < 24 {
        rtp_header.push(0);
    }

    rtp_header
}

/// Gets an xsalsa20_poly1305_suffix nonce from an rtppacket.
///
/// See <https://discord-userdoccers.vercel.app/topics/voice-connections#encryption-mode>
pub(crate) fn get_xsalsa20_poly1305_suffix_nonce(packet: &[u8]) -> Vec<u8> {
    let mut nonce = Vec::with_capacity(24);

    nonce.append(&mut packet[(packet.len() - 24)..packet.len()].to_vec());

    nonce
}

/// Gets an xsalsa20_poly1305_lite nonce from an rtppacket.
///
/// See <https://discord-userdoccers.vercel.app/topics/voice-connections#encryption-mode>
pub(crate) fn get_xsalsa20_poly1305_lite_nonce(packet: &[u8]) -> Vec<u8> {
    let mut nonce = Vec::with_capacity(24);

    nonce.append(&mut packet[(packet.len() - 4)..packet.len()].to_vec());

    // The suffix is only 4 bytes, but the nonce has to be 24
    while nonce.len() < 24 {
        nonce.push(0);
    }

    nonce
}
