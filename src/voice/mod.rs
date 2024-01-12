//! Module for all voice functionality within chorus.

mod crypto;
#[cfg(feature = "voice_gateway")]
pub mod gateway;
#[cfg(all(feature = "voice_udp", feature = "voice_gateway"))]
pub mod handler;
#[cfg(feature = "voice_udp")]
pub mod udp;
#[cfg(feature = "voice_udp")]
pub mod voice_data;

// Pub use this so users can interact with packet types if they want
#[cfg(feature = "voice_udp")]
pub use discortp;
