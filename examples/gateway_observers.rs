// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// This example showcase how to properly use gateway observers.
//
// To properly run it, you will need to change the token below.

const TOKEN: &str = "";

/// Find the gateway websocket url of the server we want to connect to
const GATEWAY_URL: &str = "wss://gateway.old.server.spacebar.chat/";

use async_trait::async_trait;
use chorus::gateway::Gateway;
use chorus::{
    self,
    gateway::Observer,
    types::{GatewayIdentifyPayload, GatewayReady},
};
use std::{sync::Arc, time::Duration};
use tokio::{self};

#[cfg(not(target_arch = "wasm32"))]
use tokio::time::sleep;
#[cfg(target_arch = "wasm32")]
use wasmtimer::tokio::sleep;

// This example creates a simple gateway connection and a basic observer struct

// Due to certain limitations all observers must impl debug
#[derive(Debug)]
pub struct ExampleObserver {}

// This struct can observe GatewayReady events when subscribed, because it implements the trait Observer<GatewayReady>.
// The Observer trait can be implemented for a struct for a given websocketevent to handle observing it
// One struct can be an observer of multiple websocketevents, if needed
#[async_trait]
impl Observer<GatewayReady> for ExampleObserver {
    // After we subscribe to an event this function is called every time we receive it
    async fn update(&self, _data: &GatewayReady) {
        println!("Observed Ready!");
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let gateway_websocket_url = GATEWAY_URL.to_string();

    // Initiate the gateway connection
    let gateway = Gateway::spawn(gateway_websocket_url).await.unwrap();

    // Create an instance of our observer
    let observer = ExampleObserver {};

    // Share ownership of the observer with the gateway
    let shared_observer = Arc::new(observer);

    // Subscribe our observer to the Ready event on this gateway
    // From now on observer.update(data) will be called every time we receive the Ready event
    gateway
        .events
        .lock()
        .await
        .session
        .ready
        .subscribe(shared_observer);

    // Authenticate so we will receive any events
    let token = TOKEN.to_string();
    let mut identify = GatewayIdentifyPayload::common();
    identify.token = token;
    gateway.send_identify(identify).await;

    // Do something on the main thread so we don't quit
    loop {
        sleep(Duration::from_secs(3600)).await;
    }
}
