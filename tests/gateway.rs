// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod common;

use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use chorus::errors::GatewayError;
use chorus::gateway::*;
use chorus::types::{
    self, Channel, ChannelCreateSchema, ChannelModifySchema, GatewayReady, IntoShared,
    RoleCreateModifySchema, RoleObject,
};
use pubserve::Subscriber;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;
#[cfg(target_arch = "wasm32")]
wasm_bindgen_test_configure!(run_in_browser);

#[cfg(not(target_arch = "wasm32"))]
use tokio::time::sleep;
#[cfg(target_arch = "wasm32")]
use wasmtimer::tokio::sleep;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
/// Tests establishing a connection (hello and heartbeats) on the local gateway;
async fn test_gateway_establish() {
    let bundle = common::setup().await;

    let _: GatewayHandle = Gateway::spawn(&bundle.urls.wss, GatewayOptions::default())
        .await
        .unwrap();
    common::teardown(bundle).await
}

#[derive(Debug)]
struct GatewayReadyObserver {
    channel: tokio::sync::mpsc::Sender<()>,
}

#[async_trait]
impl Subscriber<GatewayReady> for GatewayReadyObserver {
    async fn update(&self, _data: &GatewayReady) {
        self.channel.send(()).await.unwrap();
    }
}

#[derive(Debug)]
struct GatewayErrorObserver {
    channel: tokio::sync::mpsc::Sender<GatewayError>,
}

#[async_trait]
impl Subscriber<GatewayError> for GatewayErrorObserver {
    async fn update(&self, data: &GatewayError) {
        self.channel.send(data.clone()).await.unwrap();
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
/// Tests establishing a connection and authenticating
async fn test_gateway_authenticate() {
    let bundle = common::setup().await;

    let gateway: GatewayHandle = Gateway::spawn(&bundle.urls.wss, GatewayOptions::default())
        .await
        .unwrap();

    let (ready_send, mut ready_receive) = tokio::sync::mpsc::channel(1);

    let observer = Arc::new(GatewayReadyObserver {
        channel: ready_send,
    });

    gateway
        .events
        .lock()
        .await
        .session
        .ready
        .subscribe(observer);

    let mut identify = types::GatewayIdentifyPayload::common();
    identify.token = bundle.user.token.clone();

    gateway.send_identify(identify).await;

    tokio::select! {
        // Fail, we timed out waiting for it
        () = sleep(Duration::from_secs(20)) => {
            println!("Timed out waiting for event, failing..");
            assert!(false);
        }
        // Success, we have received it
        Some(_) = ready_receive.recv() => {}
    }

    common::teardown(bundle).await
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
/// Tests establishing a connection and receiving errors
async fn test_gateway_errors() {
    let bundle = common::setup().await;

    // FIXME: Without this, this test does not work
    //
    // Can WASM handle multiple gateway existing simulatinously?
    // This reminds of when my very old laptop couldn't handle multiple connections
    // with the ammount of tasks
    //
    // Anyway, if you have a free weekend to spend debugging wasm, you're welcome to have a crack
    // at this
    bundle.user.gateway.close().await;

    let gateway: GatewayHandle = Gateway::spawn(&bundle.urls.wss, GatewayOptions::default())
        .await
        .unwrap();

    // First we'll authenticate, wait for ready, and then authenticate again to get AlreadyAuthenticated
    let (ready_send, mut ready_receive) = tokio::sync::mpsc::channel(1);

    let observer = Arc::new(GatewayReadyObserver {
        channel: ready_send,
    });

    gateway
        .events
        .lock()
        .await
        .session
        .ready
        .subscribe(observer);

    let (error_send, mut error_receive) = tokio::sync::mpsc::channel(1);

    let observer = Arc::new(GatewayErrorObserver {
        channel: error_send,
    });

    gateway.events.lock().await.error.subscribe(observer);

    let mut identify = types::GatewayIdentifyPayload::common();
    identify.token = bundle.user.token.clone();

    // Identify and wait to receive ready
    gateway.send_identify(identify.clone()).await;

    tokio::select! {
        // Fail, we timed out waiting for it
        () = sleep(Duration::from_secs(20)) => {
            println!("Timed out waiting for ready, failing..");
            assert!(false);
        }
        // Success, we have received it
        Some(_) = ready_receive.recv() => {}
    }

    // Identify again, so we should receive already authenticated
    gateway.send_identify(identify).await;

    tokio::select! {
        // Fail, we timed out waiting for it
        () = sleep(Duration::from_secs(20)) => {
            assert!(false);
        }
        // Success, we have received it
        Some(error) = error_receive.recv() => {
                assert_eq!(error, GatewayError::AlreadyAuthenticated);
          }
    }

    common::teardown(bundle).await;
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn test_self_updating_structs() {
    // PRETTYFYME: This test is a bit of a mess, but it works. Ideally, each self-updating struct
    // would have its own test.
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
        .modify(modify_schema, None, &mut bundle.user)
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

    let guild = bundle
        .user
        .gateway
        .observe_and_into_inner(bundle.guild.clone())
        .await;
    assert!(guild.channels.is_empty());

    Channel::create(
        &mut bundle.user,
        guild.id,
        None,
        ChannelCreateSchema {
            name: "selfupdating2".to_string(),
            channel_type: Some(types::ChannelType::GuildText),
            ..Default::default()
        },
    )
    .await
    .unwrap();

    let guild = bundle
        .user
        .gateway
        .observe_and_into_inner(guild.into_shared())
        .await;
    assert!(!guild.channels.is_empty());
    assert_eq!(guild.channels.len(), 1);

    common::teardown(bundle).await
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn test_recursive_self_updating_structs() {
    // Setup
    let mut bundle = common::setup().await;
    let guild = bundle.guild.clone();
    // Observe Guild, make sure it has no channels
    let guild = bundle.user.gateway.observe(guild.clone()).await;
    let inner_guild = guild.read().unwrap().clone();
    assert!(inner_guild.roles.is_empty());
    // Create Role
    let permissions = types::PermissionFlags::CONNECT | types::PermissionFlags::MANAGE_EVENTS;
    let mut role_create_schema: types::RoleCreateModifySchema = RoleCreateModifySchema {
        name: Some("cool person".to_string()),
        permissions: Some(permissions),
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
        .observe(role.clone().into_shared())
        .await;
    // Update Guild and check for Guild
    let inner_guild = guild.read().unwrap().clone();
    assert!(!inner_guild.roles.is_empty());
    // Update the Role
    role_create_schema.name = Some("yippieee".to_string());
    RoleObject::modify(&mut bundle.user, guild_id, role.id, role_create_schema)
        .await
        .unwrap();
    let role_inner = bundle
        .user
        .gateway
        .observe_and_into_inner(role.clone().into_shared())
        .await;
    assert_eq!(role_inner.name, "yippieee");
    // Check if the change propagated
    let guild = bundle.user.gateway.observe(bundle.guild.clone()).await;
    let inner_guild = guild.read().unwrap().clone();
    let guild_roles = inner_guild.roles;
    let guild_role_inner = guild_roles.first().unwrap().read().unwrap().clone();
    assert_eq!(guild_role_inner.name, "yippieee".to_string());
    common::teardown(bundle).await;
}
