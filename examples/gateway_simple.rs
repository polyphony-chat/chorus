use std::time::Duration;

use chorus::gateway::Gateway;
use chorus::{self, types::GatewayIdentifyPayload};

#[cfg(not(target_arch = "wasm32"))]
use tokio::time::sleep;
#[cfg(target_arch = "wasm32")]
use wasmtimer::tokio::sleep;

/// This example creates a simple gateway connection and a session with an Identify event
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Find the gateway websocket url of the server we want to connect to
    let websocket_url_spacebar = "wss://gateway.old.server.spacebar.chat/".to_string();

    // Initiate the gateway connection, starting a listener in one thread and a heartbeat handler in another
    let gateway = Gateway::spawn(websocket_url_spacebar).await.unwrap();

    // At this point, we are connected to the server and are sending heartbeats, however we still haven't authenticated

    // Get a token for an account on the server
    let token = "SecretToken".to_string();

    // Create an identify event
    // An Identify event is how the server authenticates us and gets info about our os and browser, along with our intents / capabilities
    // (Chorus never sends real telemetry data about your system to servers, always just using the most common option or a custom set one)
    // By default the capabilities requests all the data of a regular client
    let mut identify = GatewayIdentifyPayload::common();

    identify.token = token;

    // Send off the event
    gateway.send_identify(identify).await; 
    
    // Do something on the main thread so we don't quit
    loop {
        sleep(Duration::from_secs(3600)).await;
    }
}
