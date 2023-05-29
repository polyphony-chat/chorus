mod common;
use chorus::types::{Guild, GuildCreateSchema};

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

    let guild = Guild::create(&mut bundle.user, bundle.urls.get_api(), guild_create_schema)
        .await
        .unwrap();

    match Guild::delete(
        &mut bundle.user,
        bundle.urls.get_api(),
        &guild.id.to_string(),
    )
    .await
    {
        None => assert!(true),
        Some(_) => assert!(false),
    }
    common::teardown(bundle).await
}

#[tokio::test]
async fn get_channels() {
    let mut bundle = common::setup().await;
    println!(
        "{:?}",
        bundle
            .guild
            .channels(
                bundle.instance.urls.get_api(),
                &bundle.user.token,
                &mut bundle.user.limits,
                &mut bundle.instance.limits,
            )
            .await
            .unwrap()
    );
    common::teardown(bundle).await;
}
