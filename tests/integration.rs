use std::sync::Mutex;

use chorus::{
    api::{AuthUsername, LoginSchema, User},
    instance::Instance,
    limit::LimitedRequester,
    URLBundle,
};

struct TestBundle {
    urls: URLBundle,
    user: User,
}

async fn setup() -> TestBundle {
    let urls = URLBundle::new(
        "http://localhost:3001/api".to_string(),
        "ws://localhost:3001".to_string(),
        "http://localhost:3001".to_string(),
    );
    let mut instance = Instance::new(urls.clone()).await.unwrap();
    // Requires the existance of the below user.
    let login_schema: LoginSchema = LoginSchema::new(
        AuthUsername::new("user@test.xyz".to_string()).unwrap(),
        "transrights".to_string(),
        None,
        None,
        None,
        None,
    )
    .unwrap();
    let user = instance.login_account(&login_schema).await.unwrap();

    TestBundle {
        urls: urls,
        user: user,
    }
}

async fn teardown() {}
