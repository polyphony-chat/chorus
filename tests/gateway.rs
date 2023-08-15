mod common;

use chorus::gateway::*;
use chorus::types::{self, Channel, ChannelModifySchema, Guild};

#[tokio::test]
/// Tests establishing a connection (hello and heartbeats) on the local gateway;
async fn test_gateway_establish() {
    let bundle = common::setup().await;

    Gateway::new(bundle.urls.wss.clone()).await.unwrap();
    common::teardown(bundle).await
}

#[tokio::test]
/// Tests establishing a connection and authenticating
async fn test_gateway_authenticate() {
    let bundle = common::setup().await;

    let gateway = Gateway::new(bundle.urls.wss.clone()).await.unwrap();

    let mut identify = types::GatewayIdentifyPayload::common();
    identify.token = bundle.user.token.clone();

    gateway.send_identify(identify).await;
    common::teardown(bundle).await
}

#[tokio::test]
async fn test_self_updating_structs() {
    let mut bundle = common::setup().await;
    let channel_updater = bundle.user.gateway.observe(bundle.channel.clone()).await;
    let received_channel = channel_updater.borrow().clone().read().unwrap().clone();

    assert_eq!(received_channel, bundle.channel.read().unwrap().clone());

    let updater = bundle.user.gateway.observe(bundle.channel.clone()).await;
    assert_eq!(
        updater
            .borrow()
            .clone()
            .read()
            .unwrap()
            .clone()
            .name
            .unwrap(),
        bundle.channel.read().unwrap().clone().name.unwrap()
    );

    let channel = bundle.channel.read().unwrap().clone();
    let modify_schema = ChannelModifySchema {
        name: Some("selfupdating".to_string()),
        ..Default::default()
    };
    Channel::modify(&channel, modify_schema, &mut bundle.user)
        .await
        .unwrap();

    assert_eq!(
        updater.borrow().read().unwrap().clone().name.unwrap(),
        "selfupdating".to_string()
    );

    common::teardown(bundle).await
}

#[tokio::test]
async fn test_recursive_self_updating_structs() {}
