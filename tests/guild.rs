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

    println!("{}", guild);

    match Guild::delete(&mut bundle.user, bundle.urls.get_api(), guild).await {
        None => assert!(true),
        Some(_) => assert!(false),
    }
    common::teardown(bundle).await
}
