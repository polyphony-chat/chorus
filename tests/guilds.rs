use chorus::types::{
    CreateChannelInviteSchema, Guild, GuildBanCreateSchema, GuildCreateSchema, GuildModifySchema,
};

mod common;

#[tokio::test]
async fn guild_creation_deletion() {
    let mut bundle = common::setup().await;

    let guild_create_schema = GuildCreateSchema {
        name: Some("test".to_string()),
        region: None,
        icon: None,
        channels: None,
        guild_template_code: None,
        system_channel_id: None,
        rules_channel_id: None,
    };

    let guild = Guild::create(&mut bundle.user, guild_create_schema)
        .await
        .unwrap();

    assert!(Guild::delete(&mut bundle.user, guild.id).await.is_ok());
    common::teardown(bundle).await
}

#[tokio::test]
async fn get_channels() {
    let mut bundle = common::setup().await;
    let guild = bundle.guild.read().unwrap().clone();
    println!("{:?}", guild.channels(&mut bundle.user).await.unwrap());
    common::teardown(bundle).await;
}

#[tokio::test]
async fn guild_create_ban() {
    // TODO: When routes exist to check if user x is on guild y, add this as an assertion to check
    // if Spacebar actually bans the user.
    let mut bundle = common::setup().await;
    let channel = bundle.channel.read().unwrap().clone();
    let mut other_user = bundle.create_user("testuser1312").await;
    let user = &mut bundle.user;
    let create_channel_invite_schema = CreateChannelInviteSchema::default();
    let guild = bundle.guild.read().unwrap().clone();
    let invite = user
        .create_channel_invite(create_channel_invite_schema, channel.id)
        .await
        .unwrap();
    other_user.accept_invite(&invite.code, None).await.unwrap();
    let other_user_id = other_user.object.read().unwrap().id;
    Guild::create_ban(
        guild.id,
        other_user_id,
        None,
        GuildBanCreateSchema::default(),
        &mut bundle.user,
    )
    .await
    .unwrap();
    assert!(Guild::create_ban(
        guild.id,
        other_user_id,
        None,
        GuildBanCreateSchema::default(),
        &mut bundle.user,
    )
    .await
    .is_err());
    common::teardown(bundle).await
}

#[tokio::test]
async fn modify_guild() {
    let mut bundle = common::setup().await;
    let schema = GuildModifySchema {
        name: Some("Mycoolguild".to_string()),
        ..Default::default()
    };
    let guild_id = bundle.guild.read().unwrap().id;
    let result = Guild::modify(guild_id, schema, &mut bundle.user)
        .await
        .unwrap();
    assert_eq!(result.name.unwrap(), "Mycoolguild".to_string());
    common::teardown(bundle).await
}

#[tokio::test]
async fn guild_remove_member() {
    let mut bundle = common::setup().await;
    let channel = bundle.channel.read().unwrap().clone();
    let mut other_user = bundle.create_user("testuser1312").await;
    let user = &mut bundle.user;
    let create_channel_invite_schema = CreateChannelInviteSchema::default();
    let guild = bundle.guild.read().unwrap().clone();
    let invite = user
        .create_channel_invite(create_channel_invite_schema, channel.id)
        .await
        .unwrap();
    other_user.accept_invite(&invite.code, None).await.unwrap();
    let other_user_id = other_user.object.read().unwrap().id;
    Guild::remove_member(guild.id, other_user_id, None, &mut bundle.user)
        .await
        .unwrap();
    assert!(
        Guild::remove_member(guild.id, other_user_id, None, &mut bundle.user,)
            .await
            .is_err()
    );
    common::teardown(bundle).await
}
