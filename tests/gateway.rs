mod common;

use std::sync::{Arc, RwLock};

use chorus::gateway::*;
use chorus::types::{
    self, ChannelModifySchema, Composite, PermissionFlags, RoleCreateModifySchema, RoleObject,
};

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
    let received_channel = bundle
        .user
        .gateway
        .observe_and_into_inner(bundle.channel.clone())
        .await;

    assert_eq!(received_channel, bundle.channel.read().unwrap().clone());

    let modify_schema = ChannelModifySchema {
        name: Some("selfupdating".to_string()),
        ..Default::default()
    };
    received_channel
        .modify(modify_schema, &mut bundle.user)
        .await
        .unwrap();
    assert_eq!(
        bundle
            .user
            .gateway
            .observe_and_into_inner(bundle.channel.clone())
            .await
            .name
            .unwrap(),
        "selfupdating".to_string()
    );

    common::teardown(bundle).await
}

#[tokio::test]
async fn test_recursive_self_updating_structs() {
    let mut bundle = common::setup().await;

    let guild = bundle
        .user
        .gateway
        .observe_and_into_inner(bundle.guild.clone())
        .await;
    assert!(guild.roles.is_none());
    let id = guild.id;
    let permissions = PermissionFlags::CONNECT | PermissionFlags::MANAGE_EVENTS;
    let permissions = Some(permissions.to_string());
    let mut role_create_schema = RoleCreateModifySchema {
        name: Some("among us".to_string()),
        permissions,
        hoist: Some(true),
        icon: None,
        unicode_emoji: Some("".to_string()),
        mentionable: Some(true),
        position: None,
        color: None,
    };
    let role = RoleObject::create(&mut bundle.user, id, role_create_schema.clone())
        .await
        .unwrap();
    let role_watch = bundle
        .user
        .gateway
        .observer_channel(Arc::new(RwLock::new(role.clone())))
        .await;
    let guild = bundle
        .user
        .gateway
        .observe_and_into_inner(bundle.guild.clone())
        .await;
    assert!(guild.roles.is_some());
    role_create_schema.name = Some("enbyenvy".to_string());
    RoleObject::modify(&mut bundle.user, id, role.id, role_create_schema)
        .await
        .unwrap();
    let newrole = role_watch.borrow().read().unwrap().clone();
    assert_eq!(newrole.name, "enbyenvy".to_string());
    let guild_role = role_watch.borrow().read().unwrap().clone();
    assert_eq!(guild_role.name, "enbyenvy".to_string());
    common::teardown(bundle).await;
}
