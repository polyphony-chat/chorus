use chorus::{
    instance::{Instance, UserMeta},
    types::{Channel, ChannelCreateSchema, Guild, GuildCreateSchema, RegisterSchema},
    URLBundle,
};

#[derive(Debug)]
struct TestBundle {
    urls: URLBundle,
    user: UserMeta,
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
        "integrationtestuser".to_string(),
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
    let channel_create_schema = ChannelCreateSchema {
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
    use chorus::types::{Channel, Guild, GuildCreateSchema};

    #[tokio::test]
    async fn guild_creation_deletion() {
        let mut bundle = crate::setup().await;

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
                &bundle_channel.id.to_string(),
                &mut bundle_user.limits,
                &mut bundle.instance.limits
            )
            .await
            .unwrap()
        );
        crate::teardown(bundle).await;
    }
}

mod messages {
    use std::{
        fs::File,
        io::{BufReader, Read},
    };

    use chorus::types;

    #[tokio::test]
    async fn send_message() {
        let mut bundle = crate::setup().await;
        let channel_id = "1106954414356168802".to_string();
        let mut message = types::MessageSendSchema::new(
            None,
            Some("A Message!".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );
        let token = bundle.user.token.clone();
        println!("TOKEN: {}", token);
        let _ = bundle
            .user
            .send_message(&mut message, channel_id, None)
            .await
            .unwrap();
        crate::teardown(bundle).await;
    }

    #[tokio::test]
    async fn send_message_attachment() {
        let mut bundle = crate::setup().await;

        let channel_id = "1106954414356168802".to_string();
        let f = File::open("./README.md").unwrap();
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();

        reader.read_to_end(&mut buffer).unwrap();

        let attachment = types::PartialDiscordFileAttachment {
            id: None,
            filename: "README.md".to_string(),
            description: None,
            content_type: None,
            size: None,
            url: None,
            proxy_url: None,
            width: None,
            height: None,
            ephemeral: None,
            duration_secs: None,
            waveform: None,
            content: buffer,
        };

        let mut message = types::MessageSendSchema::new(
            None,
            Some("trans rights now".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(vec![attachment.clone()]),
        );
        let vec_attach = vec![attachment.clone()];
        let _arg = Some(&vec_attach);
        let response = bundle
            .user
            .send_message(&mut message, channel_id, Some(vec![attachment.clone()]))
            .await
            .unwrap();
        println!("[Response:] {}", response.text().await.unwrap());
    }
}
