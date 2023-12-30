#[cfg(all(not(target_arch = "wasm32"), feature = "voice"))]
pub mod tokio;
#[cfg(all(not(target_arch = "wasm32"), feature = "voice"))]
pub use tokio::*;

#[cfg(all(not(target_arch = "wasm32"), feature = "voice"))]
pub type UdpSocket = tokio::TokioSocket;
#[cfg(all(not(target_arch = "wasm32"), feature = "voice"))]
pub type UdpBackend = tokio::TokioBackend;

#[cfg(target_arch = "wasm32")]
compile_error!("UDP Voice support is not (and will likely never be) supported for WASM. This is because UDP cannot be used in the browser. We are however looking into Webrtc for WASM voice support.");
