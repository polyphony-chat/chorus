#[cfg(not(target_arch = "wasm32"))]
pub mod tungstenite;

#[cfg(not(target_arch = "wasm32"))]
pub type Sink = tungstenite::TungsteniteSink;
#[cfg(not(target_arch = "wasm32"))]
pub type Stream = tungstenite::TungsteniteStream;
#[cfg(not(target_arch = "wasm32"))]
pub type WebSocketBackend = tungstenite::TungsteniteBackend;
