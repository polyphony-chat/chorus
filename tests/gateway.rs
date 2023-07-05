mod common;
use chorus::gateway::*;
use chorus::types;

#[tokio::test]
/// Tests establishing a connection (hello and heartbeats) on the local gateway;
async fn test_gateway_establish() {
    Gateway::new(common::WSS.into()).await.unwrap();
}

#[tokio::test]
/// Tests establishing a connection and authenticating
async fn test_gateway_authenticate() {
    let bundle = common::setup().await;

    let gateway = Gateway::new(common::WSS.into()).await.unwrap();

    let mut identify = types::GatewayIdentifyPayload::common();
    identify.token = bundle.user.token.clone();

    gateway.send_identify(identify).await;

    // TODO: Trying to teardown the bundle hangs?
    // common::teardown(bundle).await.unwrap();
}
