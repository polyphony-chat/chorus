#[cfg(all(not(target_arch = "wasm32"), feature = "voice_udp"))]
pub mod tokio;
#[cfg(all(not(target_arch = "wasm32"), feature = "voice_udp"))]
pub use tokio::*;

#[cfg(all(not(target_arch = "wasm32"), feature = "voice_udp"))]
pub type UdpSocket = tokio::TokioSocket;
#[cfg(all(not(target_arch = "wasm32"), feature = "voice_udp"))]
pub type UdpBackend = tokio::TokioBackend;

#[cfg(all(target_arch = "wasm32", feature = "voice_udp"))]
compile_error!("UDP Voice support is not (and will likely never be) supported for WASM. This is because UDP cannot be used in the browser. We are however looking into Webrtc for WASM voice support.");
