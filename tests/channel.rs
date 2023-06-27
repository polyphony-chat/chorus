use chorus::{
    errors::ChorusResult,
    types::{
        self, Channel, GetChannelMessagesSchema, MessageSendSchema, PermissionFlags,
        PermissionOverwrite, Snowflake,
    },
};

mod common;

#[tokio::test]
async fn get_channel() -> ChorusResult<()> {
    let mut bundle = common::setup().await;
    assert_eq!(
        bundle.channel,
        Channel::get(&mut bundle.user, bundle.channel.id).await?
    );
    common::teardown(bundle).await
}

#[tokio::test]
async fn delete_channel() -> ChorusResult<()> {
    let mut bundle = common::setup().await;
    Channel::delete(bundle.channel.clone(), &mut bundle.user).await?;
    common::teardown(bundle).await
}

#[tokio::test]
async fn modify_channel() -> ChorusResult<()> {
    let mut bundle = common::setup().await;
    let channel = &mut bundle.channel;
    let modify_data: types::ChannelModifySchema = types::ChannelModifySchema {
        name: Some("beepboop".to_string()),
        channel_type: None,
        topic: None,
        icon: None,
        bitrate: None,
        user_limit: None,
        rate_limit_per_user: None,
        position: None,
        permission_overwrites: None,
        parent_id: None,
        nsfw: None,
        rtc_region: None,
        default_auto_archive_duration: None,
        default_reaction_emoji: None,
        flags: None,
        default_thread_rate_limit_per_user: None,
        video_quality_mode: None,
    };
    Channel::modify(channel, modify_data, channel.id, &mut bundle.user).await?;
    assert_eq!(channel.name, Some("beepboop".to_string()));

    let permission_override = PermissionFlags::from_vec(Vec::from([
        PermissionFlags::MANAGE_CHANNELS,
        PermissionFlags::MANAGE_MESSAGES,
    ]));
    let permission_override = PermissionOverwrite {
        id: bundle.user.object.id,
        overwrite_type: "1".to_string(),
        allow: permission_override,
        deny: "0".to_string(),
    };

    Channel::edit_permissions(
        &mut bundle.user,
        bundle.channel.id,
        permission_override.clone(),
    )
    .await?;

    Channel::delete_permission(&mut bundle.user, bundle.channel.id, permission_override.id).await?;

    common::teardown(bundle).await
}

#[tokio::test]
async fn get_channel_messages() -> ChorusResult<()> {
    let mut bundle = common::setup().await;

    // First create some messages to read
    for _ in 0..10 {
        let _ = bundle
            .user
            .send_message(
                &mut MessageSendSchema {
                    content: Some("A Message!".to_string()),
                    ..Default::default()
                },
                bundle.channel.id,
                None,
            )
            .await?;
    }

    assert_eq!(
        Channel::messages(
            GetChannelMessagesSchema::before(Snowflake::generate()),
            bundle.channel.id,
            &mut bundle.user,
        )
        .await?
        .len(),
        10
    );

    // around is currently bugged in spacebar: https://github.com/spacebarchat/server/issues/1072
    // assert_eq!(
    //     Channel::messages(
    //         GetChannelMessagesSchema::around(Snowflake::generate()).limit(10),
    //         bundle.channel.id,
    //         &mut bundle.user,
    //     )
    //     .await?
    //     .len(),
    //     5
    // );

    assert!(Channel::messages(
        GetChannelMessagesSchema::after(Snowflake::generate()),
        bundle.channel.id,
        &mut bundle.user,
    )
    .await?
    .is_empty());

    common::teardown(bundle).await
}
