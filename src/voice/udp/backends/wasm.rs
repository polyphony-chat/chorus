use std::net::SocketAddr;

// TODO: Add wasm websockets
compile_error!("Udp voice support is not implemented yet for wasm.");

#[derive(Debug, Clone)]
pub struct WasmBackend;

pub type WasmSocket;

impl WasmBackend {
    pub async fn connect(url: SocketAddr) -> Result<WasmSocket, VoiceUdpError> {}
}
