mod common;
use chorus::types::{self, Channel};

#[tokio::test]
async fn get_channel() {
    let mut bundle = common::setup().await;
    let bundle_channel = bundle.channel.clone();
    let mut bundle_user = &mut bundle.user;

    assert_eq!(
        bundle_channel,
        Channel::get(&mut bundle_user, &bundle_channel.id.to_string(),)
            .await
            .unwrap()
    );
    common::teardown(bundle).await
}

#[tokio::test]
async fn delete_channel() {
    let mut bundle = common::setup().await;
    let result = bundle.channel.clone().delete(&mut bundle.user).await;
    assert!(result.is_none());
    common::teardown(bundle).await
}

#[tokio::test]
async fn modify_channel() {
    let mut bundle = common::setup().await;
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
    let result = Channel::modify(
        modify_data,
        &bundle.channel.id.to_string(),
        &mut bundle.user,
    )
    .await
    .unwrap();
    assert_eq!(result.name, Some("beepboop".to_string()));
    common::teardown(bundle).await
}
