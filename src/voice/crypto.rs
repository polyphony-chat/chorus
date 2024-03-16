// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Defines cryptography functions used within the voice implementation.
//!
//! All functions in this module return a 24 byte long `Vec<u8>`.

/// Gets an `xsalsa20_poly1305` nonce from an rtppacket.
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

/// Gets an `xsalsa20_poly1305_suffix` nonce from an rtppacket.
///
/// See <https://discord-userdoccers.vercel.app/topics/voice-connections#encryption-mode>
pub(crate) fn get_xsalsa20_poly1305_suffix_nonce(packet: &[u8]) -> Vec<u8> {
    let mut nonce = Vec::with_capacity(24);

    nonce.append(&mut packet[(packet.len() - 24)..packet.len()].to_vec());

    nonce
}

/// Gets an `xsalsa20_poly1305_lite` nonce from an rtppacket.
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

#[test]
// Asserts all functions that retrieve a nonce from packet bytes
fn test_packet_nonce_derives() {
    let test_packet_bytes = vec![
        144, 120, 98, 5, 71, 174, 52, 64, 0, 4, 85, 36, 178, 8, 37, 146, 35, 154, 141, 36, 125, 15,
        65, 179, 227, 108, 165, 56, 68, 68, 3, 62, 87, 233, 7, 81, 147, 93, 22, 95, 115, 202, 48,
        66, 190, 229, 69, 146, 66, 108, 60, 114, 2, 228, 111, 40, 108, 5, 68, 226, 76, 240, 20,
        231, 210, 214, 123, 175, 188, 161, 10, 125, 13, 196, 114, 248, 50, 84, 103, 139, 86, 223,
        82, 173, 8, 209, 78, 188, 169, 151, 157, 42, 189, 153, 228, 105, 199, 19, 185, 16, 33, 133,
        113, 253, 145, 36, 106, 14, 222, 128, 226, 239, 10, 39, 72, 113, 33, 113,
    ];

    let nonce_1 = get_xsalsa20_poly1305_nonce(&test_packet_bytes);
    let nonce_1_expected = vec![
        144, 120, 98, 5, 71, 174, 52, 64, 0, 4, 85, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    let nonce_2 = get_xsalsa20_poly1305_suffix_nonce(&test_packet_bytes);
    let nonce_2_expected = vec![
        228, 105, 199, 19, 185, 16, 33, 133, 113, 253, 145, 36, 106, 14, 222, 128, 226, 239, 10,
        39, 72, 113, 33, 113,
    ];

    let nonce_3 = get_xsalsa20_poly1305_lite_nonce(&test_packet_bytes);
    let nonce_3_expected = vec![
        72, 113, 33, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    println!("nonce 1: {:?}", nonce_1);
    println!("nonce 2: {:?}", nonce_2);
    println!("nonce 3: {:?}", nonce_3);

    assert_eq!(nonce_1.len(), 24);
    assert_eq!(nonce_2.len(), 24);
    assert_eq!(nonce_3.len(), 24);

    assert_eq!(nonce_1, nonce_1_expected);
    assert_eq!(nonce_2, nonce_2_expected);
    assert_eq!(nonce_3, nonce_3_expected);
}
