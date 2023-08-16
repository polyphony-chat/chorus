mod common;

use std::sync::{Arc, RwLock};

use chorus::gateway::*;
use chorus::types::{self, ChannelModifySchema, RoleCreateModifySchema, RoleObject};

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
    // Setup
    let mut bundle = common::setup().await;
    let guild = bundle.guild.clone();
    // Observe Guild, make sure it has no channels
    let guild = bundle.user.gateway.observe(guild.clone()).await;
    let inner_guild = guild.read().unwrap().clone();
    assert!(inner_guild.roles.is_none());
    // Create Role
    let permissions = types::PermissionFlags::CONNECT | types::PermissionFlags::MANAGE_EVENTS;
    let permissions = Some(permissions.to_string());
    let mut role_create_schema: types::RoleCreateModifySchema = RoleCreateModifySchema {
        name: Some("cool person".to_string()),
        permissions,
        hoist: Some(true),
        icon: None,
        unicode_emoji: Some("".to_string()),
        mentionable: Some(true),
        position: None,
        color: None,
    };
    let guild_id = inner_guild.id;
    let role = RoleObject::create(&mut bundle.user, guild_id, role_create_schema.clone())
        .await
        .unwrap();
    // Watch role;
    bundle
        .user
        .gateway
        .observe(Arc::new(RwLock::new(role.clone())))
        .await;
    // Update Guild and check for Guild
    let guild = bundle.user.gateway.observe(guild.clone()).await;
    let inner_guild = guild.read().unwrap().clone();
    assert!(inner_guild.roles.is_some());
    // Update the Role
    role_create_schema.name = Some("yippieee".to_string());
    RoleObject::modify(&mut bundle.user, guild_id, role.id, role_create_schema)
        .await
        .unwrap();
    // Check if the change propagated
    let guild_roles = guild.read().unwrap().clone().roles;
    let guild_role = guild_roles.unwrap();
    let guild_role_inner = guild_role.get(0).unwrap().read().unwrap().clone();
    assert_eq!(guild_role_inner.name, "yippieee".to_string());
    common::teardown(bundle).await;
}
