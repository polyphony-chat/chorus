//! Defines cryptography functions used within the voice implementation.
//!
//! All functions in this module return a 24 byte long [Vec<u8>].

/// Gets an xsalsa20poly1305 nonce from an rtppacket.
pub(crate) fn get_xsalsa20_poly1305_nonce(packet: &[u8]) -> Vec<u8> {
    let mut rtp_header = packet[0..12].to_vec();

    // The header is only 12 bytes, but the nonce has to be 24
    // This actually works mind you, and anything else doesn't
    for _i in 0..12 {
        rtp_header.push(0);
    }

    return rtp_header;
}
