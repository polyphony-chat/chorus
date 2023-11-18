pub use super::*;
use ws_stream_wasm::*;
#[derive(Debug)]
pub struct WasmGateway {
    events: Arc<Mutex<Events>>,
}
