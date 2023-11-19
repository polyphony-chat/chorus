#[cfg(all(not(target_arch = "wasm32"), feature = "client"))]
pub mod tungstenite;
#[cfg(all(not(target_arch = "wasm32"), feature = "client"))]
pub use tungstenite::*;
#[cfg(all(target_arch = "wasm32", feature = "client"))]
pub mod wasm;
#[cfg(all(target_arch = "wasm32", feature = "client"))]
pub use wasm::*;

#[cfg(all(not(target_arch = "wasm32"), feature = "client"))]
pub type Sink = tungstenite::TungsteniteSink;
#[cfg(all(not(target_arch = "wasm32"), feature = "client"))]
pub type Stream = tungstenite::TungsteniteStream;
#[cfg(all(not(target_arch = "wasm32"), feature = "client"))]
pub type WebSocketBackend = tungstenite::TungsteniteBackend;

#[cfg(all(target_arch = "wasm32", feature = "client"))]
pub type Sink = wasm::WasmSink;
#[cfg(all(target_arch = "wasm32", feature = "client"))]
pub type Stream = wasm::WasmStream;
#[cfg(all(target_arch = "wasm32", feature = "client"))]
pub type WebSocketBackend = wasm::WasmBackend;
