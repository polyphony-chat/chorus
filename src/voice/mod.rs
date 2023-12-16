//! Module for all voice functionality within chorus.

mod crypto;
pub mod gateway;
pub mod udp;
pub mod voice_data;

// Pub use this so users can interact with packet types if they want
pub use discortp;
