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
