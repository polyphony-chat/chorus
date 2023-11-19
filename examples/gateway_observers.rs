use async_trait::async_trait;
use chorus::gateway::Gateway;
use chorus::{
    self,
    gateway::Observer,
    types::{GatewayIdentifyPayload, GatewayReady},
};
use std::{sync::Arc, time::Duration};
use tokio::{self, time::sleep};

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
    // Find the gateway websocket url of the server we want to connect to
    let websocket_url_spacebar = "wss://gateway.old.server.spacebar.chat/".to_string();

    // Initiate the gateway connection
    let gateway = Gateway::spawn(websocket_url_spacebar).await.unwrap();

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
    let token = "SecretToken".to_string();
    let mut identify = GatewayIdentifyPayload::common();
    identify.token = token;
    gateway.send_identify(identify).await;

    // Do something on the main thread so we don't quit
    loop {
        sleep(Duration::MAX).await;
    }
}
