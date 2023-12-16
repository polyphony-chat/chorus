mod common;
use chorus::types::CreateChannelInviteSchema;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;
#[cfg(target_arch = "wasm32")]
wasm_bindgen_test_configure!(run_in_browser);

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn create_accept_invite() {
    let mut bundle = common::setup().await;
    let channel = bundle.channel.read().unwrap().clone();
    let mut other_user = bundle.create_user("testuser1312").await;
    let user = &mut bundle.user;
    let create_channel_invite_schema = CreateChannelInviteSchema::default();
    let guild = bundle.guild.read().unwrap().clone();
    assert!(chorus::types::Guild::get(guild.id, &mut other_user)
        .await
        .is_err());
    let invite = user
        .create_channel_invite(create_channel_invite_schema, channel.id)
        .await
        .unwrap();

    other_user.accept_invite(&invite.code, None).await.unwrap();
    assert!(chorus::types::Guild::get(guild.id, &mut other_user)
        .await
        .is_ok());
    common::teardown(bundle).await;
}
