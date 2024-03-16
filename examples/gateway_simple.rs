// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// This example showcases how to initiate a gateway connection manually
// (e. g. not through ChorusUser)
//
// To properly run it, you will need to modify the token below.

const TOKEN: &str = "";

/// Find the gateway websocket url of the server we want to connect to
const GATEWAY_URL: &str = "wss://gateway.old.server.spacebar.chat/";

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
    let gateway_websocket_url = GATEWAY_URL.to_string();

    // Initiate the gateway connection, starting a listener in one thread and a heartbeat handler in another
    let gateway = Gateway::spawn(gateway_websocket_url).await.unwrap();

    // At this point, we are connected to the server and are sending heartbeats, however we still haven't authenticated

    // Get a token for an account on the server
    let token = TOKEN.to_string();

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
