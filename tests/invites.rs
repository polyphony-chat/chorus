use chorus::types::CreateChannelInviteSchema;

mod common;

#[tokio::test]
async fn create_accept_invite() {
    let mut bundle = common::setup().await;
    let channel = bundle.channel.clone();
    let mut user = bundle.user.clone();
    let create_channel_invite_schema = CreateChannelInviteSchema::default();
    let mut other_user = bundle.create_user("testuser1312").await;
    assert!(chorus::types::Guild::get(bundle.guild.id, &mut other_user)
        .await
        .is_err());
    let invite = user
        .create_guild_invite(create_channel_invite_schema, channel.id)
        .await
        .unwrap();

    other_user.accept_invite(&invite.code, None).await.unwrap();
    assert!(chorus::types::Guild::get(bundle.guild.id, &mut other_user)
        .await
        .is_ok());
    common::teardown(bundle).await;
}
