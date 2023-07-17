use chorus::types::{
    self, Channel, GetChannelMessagesSchema, MessageSendSchema, PermissionFlags,
    PermissionOverwrite, PrivateChannelCreateSchema, RelationshipType, Snowflake,
};

mod common;

#[tokio::test]
async fn get_channel() {
    let mut bundle = common::setup().await;
    let bundle_channel = bundle.channel.clone();
    let bundle_user = &mut bundle.user;

    assert_eq!(
        bundle_channel,
        Channel::get(bundle_user, bundle_channel.id).await.unwrap()
    );
    common::teardown(bundle).await
}

#[tokio::test]
async fn delete_channel() {
    let mut bundle = common::setup().await;
    let result = Channel::delete(bundle.channel.clone(), &mut bundle.user).await;
    assert!(result.is_ok());
    common::teardown(bundle).await
}

#[tokio::test]
async fn modify_channel() {
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
    Channel::modify(channel, modify_data, channel.id, &mut bundle.user)
        .await
        .unwrap();
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
    .await
    .unwrap();

    Channel::delete_permission(&mut bundle.user, bundle.channel.id, permission_override.id)
        .await
        .unwrap();

    common::teardown(bundle).await
}

#[tokio::test]
async fn get_channel_messages() {
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
            .await
            .unwrap();
    }

    assert_eq!(
        Channel::messages(
            GetChannelMessagesSchema::before(Snowflake::generate()),
            bundle.channel.id,
            &mut bundle.user,
        )
        .await
        .unwrap()
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
    //     .await
    //     .unwrap()
    //     .len(),
    //     5
    // );

    assert!(Channel::messages(
        GetChannelMessagesSchema::after(Snowflake::generate()),
        bundle.channel.id,
        &mut bundle.user,
    )
    .await
    .unwrap()
    .is_empty());

    common::teardown(bundle).await
}

#[tokio::test]
async fn create_dm() {
    let mut bundle = common::setup().await;
    let other_user = bundle.create_user("integrationtestuser2").await;
    let user = &mut bundle.user;
    let private_channel_create_schema = PrivateChannelCreateSchema {
        recipients: Some(Vec::from([other_user.object.id])),
        access_tokens: None,
        nicks: None,
    };
    let dm_channel = user
        .create_private_channel(private_channel_create_schema)
        .await
        .unwrap();
    assert!(dm_channel.recipients.is_some());
    assert_eq!(
        dm_channel.recipients.as_ref().unwrap().get(0).unwrap().id,
        other_user.object.id
    );
    assert_eq!(
        dm_channel.recipients.as_ref().unwrap().get(1).unwrap().id,
        user.object.id
    );
    common::teardown(bundle).await;
}

// #[tokio::test]
// This test currently is broken due to an issue with the Spacebar Server.
#[allow(dead_code)]
async fn remove_add_person_from_to_dm() {
    let mut bundle = common::setup().await;
    let mut other_user = bundle.create_user("integrationtestuser2").await;
    let mut third_user = bundle.create_user("integrationtestuser3").await;
    let user = &mut bundle.user;
    let private_channel_create_schema = PrivateChannelCreateSchema {
        recipients: Some(Vec::from([other_user.object.id, third_user.object.id])),
        access_tokens: None,
        nicks: None,
    };
    let dm_channel = user
        .create_private_channel(private_channel_create_schema)
        .await
        .unwrap(); // Creates the Channel and stores the response Channel object
    dm_channel
        .remove_channel_recipient(other_user.object.id, user)
        .await
        .unwrap();
    assert!(dm_channel.recipients.as_ref().unwrap().get(1).is_none());
    other_user
        .modify_user_relationship(user.object.id, RelationshipType::Friends)
        .await
        .unwrap();
    user.modify_user_relationship(other_user.object.id, RelationshipType::Friends)
        .await
        .unwrap();
    third_user
        .modify_user_relationship(user.object.id, RelationshipType::Friends)
        .await
        .unwrap();
    user.modify_user_relationship(third_user.object.id, RelationshipType::Friends)
        .await
        .unwrap();
    // Users 1-2 and 1-3 are now friends
    dm_channel
        .add_channel_recipient(other_user.object.id, user, None)
        .await
        .unwrap();
    assert!(dm_channel.recipients.is_some());
    assert_eq!(
        dm_channel.recipients.as_ref().unwrap().get(0).unwrap().id,
        other_user.object.id
    );
    assert_eq!(
        dm_channel.recipients.as_ref().unwrap().get(1).unwrap().id,
        user.object.id
    );
}
