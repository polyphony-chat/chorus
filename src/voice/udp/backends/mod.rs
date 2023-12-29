#[cfg(all(not(target_arch = "wasm32"), feature = "client"))]
pub mod tokio;
#[cfg(all(not(target_arch = "wasm32"), feature = "client"))]
pub use tokio::*;

#[cfg(all(target_arch = "wasm32", feature = "client"))]
pub mod wasm;
#[cfg(all(target_arch = "wasm32", feature = "client"))]
pub use wasm::*;

#[cfg(all(not(target_arch = "wasm32"), feature = "client"))]
pub type UdpSocket = tokio::TokioSocket;
#[cfg(all(not(target_arch = "wasm32"), feature = "client"))]
pub type UdpBackend = tokio::TokioBackend;

#[cfg(all(target_arch = "wasm32", feature = "client"))]
pub type UdpSocket = wasm::WasmSocket;
#[cfg(all(target_arch = "wasm32", feature = "client"))]
pub type UdpBackend = wasm::WasmBackend;
