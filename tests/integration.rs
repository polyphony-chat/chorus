use chorus::{
    api::{AuthUsername, RegisterSchema, User},
    instance::Instance,
    URLBundle,
};

#[derive(Debug)]
struct TestBundle {
    urls: URLBundle,
    user: User,
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
    let user = instance.register_account(&reg).await.unwrap();

    TestBundle { urls, user }
}

// Teardown method to clean up after a test.
async fn teardown(bundle: TestBundle) {
    bundle.user.delete().await;
}

/*
Tests

Tests are grouped in modules which are to be named corresponding to their parent folders'
names in `./src/api/`. Tests from login or register would, for example, go into the
module `mod auth {...}`.
*/

mod guilds {
    use chorus::api::{schemas, types};

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
        crate::teardown(bundle).await;
    }
}

mod messages {
    use std::{
        fs::File,
        io::{BufReader, Read},
    };

    use chorus::api::{schemas, types};

    #[tokio::test]
    async fn send_message() {
        let mut bundle = crate::setup().await;
        let channel_id = "1106954414356168802".to_string();
        let mut message = schemas::MessageSendSchema::new(
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

        let mut message = schemas::MessageSendSchema::new(
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
