use chorus::{
    api::schemas,
    api::{AuthUsername, Channel, Guild, GuildCreateSchema, RegisterSchema, User},
    instance::Instance,
    URLBundle,
};

#[derive(Debug)]
struct TestBundle {
    urls: URLBundle,
    user: User,
    instance: Instance,
    guild_id: String,
    channel: Channel,
}

// Set up a test by creating an Instance and a User. Reduces Test boilerplate.
async fn setup() -> TestBundle {
    let urls = URLBundle::new(
        "http://localhost:3001/api".to_string(),
        "ws://localhost:3001".to_string(),
        "http://localhost:3001".to_string(),
    );
    let mut instance = Instance::new(urls.clone()).await.unwrap();
    // Requires the existance of the below user.
    let reg = RegisterSchema::new(
        AuthUsername::new("integrationtestuser".to_string()).unwrap(),
        None,
        true,
        None,
        None,
        None,
        Some("2000-01-01".to_string()),
        None,
        None,
        None,
    )
    .unwrap();
    let guild_create_schema = GuildCreateSchema {
        name: Some("Test-Guild!".to_string()),
        region: None,
        icon: None,
        channels: None,
        guild_template_code: None,
        system_channel_id: None,
        rules_channel_id: None,
    };
    let channel_create_schema = schemas::ChannelCreateSchema {
        name: "testchannel".to_string(),
        channel_type: Some(0),
        topic: None,
        icon: None,
        bitrate: None,
        user_limit: None,
        rate_limit_per_user: None,
        position: None,
        permission_overwrites: None,
        parent_id: None,
        id: None,
        nsfw: Some(false),
        rtc_region: None,
        default_auto_archive_duration: None,
        default_reaction_emoji: None,
        flags: Some(0),
        default_thread_rate_limit_per_user: Some(0),
        video_quality_mode: None,
    };
    let mut user = instance.register_account(&reg).await.unwrap();
    let guild_id = Guild::create(&mut user, urls.get_api(), guild_create_schema)
        .await
        .unwrap();
    let channel = Channel::create(
        &user.token,
        urls.get_api(),
        guild_id.as_str(),
        channel_create_schema,
        &mut user.limits,
        &mut instance.limits,
    )
    .await
    .unwrap();

    TestBundle {
        urls,
        user,
        instance,
        guild_id,
        channel,
    }
}

// Teardown method to clean up after a test.
async fn teardown(mut bundle: TestBundle) {
    Guild::delete(
        &mut bundle.user,
        bundle.instance.urls.get_api(),
        bundle.guild_id,
    )
    .await;
    bundle.user.delete().await;
}

mod guild {
    use chorus::api::{schemas, types, Channel};

    #[tokio::test]
    async fn guild_creation_deletion() {
        let mut bundle = crate::setup().await;

        let guild_create_schema = schemas::GuildCreateSchema {
            name: Some("test".to_string()),
            region: None,
            icon: None,
            channels: None,
            guild_template_code: None,
            system_channel_id: None,
            rules_channel_id: None,
        };

        let guild =
            types::Guild::create(&mut bundle.user, bundle.urls.get_api(), guild_create_schema)
                .await
                .unwrap();

        println!("{}", guild);

        match types::Guild::delete(&mut bundle.user, bundle.urls.get_api(), guild).await {
            None => assert!(true),
            Some(_) => assert!(false),
        }
        crate::teardown(bundle).await
    }

    #[tokio::test]
    async fn get_channel() {
        let mut bundle = crate::setup().await;
        let bundle_channel = bundle.channel.clone();
        let bundle_user = &mut bundle.user;

        assert_eq!(
            bundle_channel,
            Channel::get(
                bundle_user.token.as_str(),
                bundle.instance.urls.get_api(),
                &bundle_channel.id,
                &mut bundle_user.limits,
                &mut bundle.instance.limits
            )
            .await
            .unwrap()
        );
        crate::teardown(bundle).await
    }
}
