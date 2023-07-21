mod common;
use std::default;

use chorus::gateway::*;
use chorus::types::{self, Channel};

#[tokio::test]
/// Tests establishing a connection (hello and heartbeats) on the local gateway;
async fn test_gateway_establish() {
    let bundle = common::setup().await;

    Gateway::new(bundle.urls.wss).await.unwrap();
}

#[tokio::test]
/// Tests establishing a connection and authenticating
async fn test_gateway_authenticate() {
    let bundle = common::setup().await;

    let gateway = Gateway::new(bundle.urls.wss).await.unwrap();

    let mut identify = types::GatewayIdentifyPayload::common();
    identify.token = bundle.user.token;

    gateway.send_identify(identify).await;
}

#[tokio::test]
async fn test_self_updating_structs() {
    let mut bundle = common::setup().await;
    let gateway = Gateway::new(bundle.urls.wss).await.unwrap();
    let mut identify = types::GatewayIdentifyPayload::common();
    identify.token = bundle.user.token.clone();
    gateway.send_identify(identify).await;
    let channel_receiver = gateway.observe(bundle.channel.clone()).await;
    let received_channel = channel_receiver.borrow();
    assert_eq!(*received_channel, bundle.channel);
    drop(received_channel);
    let channel = &mut bundle.channel;
    let modify_data = types::ChannelModifySchema {
        name: Some("beepboop".to_string()),
        ..Default::default()
    };
    Channel::modify(
        &mut channel.clone(),
        modify_data,
        channel.id,
        &mut bundle.user,
    )
    .await
    .unwrap();
    // assert_eq!(channel.name, Some("beepboop".to_string()));
}
