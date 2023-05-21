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

mod guild {
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
    }
}
