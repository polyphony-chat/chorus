use chorus::{
    self,
    gateway::{Gateway, Observer},
    types::{GatewayIdentifyPayload, GatewayReady},
};
use std::sync::Arc;
use tokio::{self, sync::Mutex};

// This example creates a simple gateway connection and a basic observer struct

// Due to certain limitations all observers must impl debug
#[derive(Debug)]
pub struct ExampleObserver {}

// This struct can observe GatewayReady events when subscribed, because it implements the trait Observer<GatewayReady>.
// The Observer trait can be implemented for a struct for a given websocketevent to handle observing it
// One struct can be an observer of multiple websocketevents, if needed
impl Observer<GatewayReady> for ExampleObserver {
    // After we subscribe to an event this function is called every time we receive it
    fn update(&self, _data: &GatewayReady) {
        println!("Observed Ready!");
    }
}

#[tokio::main]
async fn main() {
    // Find the gateway websocket url of the server we want to connect to
    let websocket_url_spacebar = "wss://gateway.old.server.spacebar.chat/".to_string();

    // Initiate the gateway connection
    let gateway = Gateway::new(websocket_url_spacebar).await.unwrap();

    // Create an instance of our observer
    let observer = ExampleObserver {};

    // Because observers have to reside in between the main and gateway thread, (they have to be shared between both) we need to put them in an Arc<Mutex>
    let shared_observer = Arc::new(Mutex::new(observer));

    // Subscribe our observer to the Ready event on this gateway
    // From now on observer.update(data) will be called every time we receive the Ready event
    gateway
        .events
        .lock()
        .await
        .session
        .ready
        .subscribe(shared_observer)
        .unwrap();

    // Authenticate so we will receive any events
    let token = "SecretToken".to_string();
    let mut identify = GatewayIdentifyPayload::common();
    identify.token = token;
    gateway.send_identify(identify).await;

    // Do something on the main thread so we don't quit
    loop {}
}
