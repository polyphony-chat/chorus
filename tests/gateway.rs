mod common;

use chorus::gateway::*;
use chorus::types::{self, Channel};

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
    let channel_updater = bundle.user.gateway.observe(bundle.channel).await;
    let received_channel = channel_updater.borrow().clone();
    assert_eq!(received_channel, bundle.channel);
    let channel = &mut bundle.channel;
    let modify_data = types::ChannelModifySchema {
        name: Some("beepboop".to_string()),
        ..Default::default()
    };
    Channel::modify(channel, modify_data, channel.id, &mut bundle.user)
        .await
        .unwrap();
    let received_channel = channel_updater.borrow();
    assert_eq!(received_channel.name.as_ref().unwrap(), "beepboop");
    common::teardown(bundle).await
}
