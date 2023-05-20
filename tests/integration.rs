use std::sync::Mutex;

use chorus::{
    api::{AuthUsername, LoginSchema, User},
    instance::Instance,
    limit::LimitedRequester,
    URLBundle,
};

use lazy_static::lazy_static;

lazy_static! {
    static ref INSTANCE: Mutex<Instance> =
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            Mutex::new(
                Instance::new(
                    URLBundle::new(
                        "http://localhost:3001/api".to_string(),
                        "ws://localhost:3001".to_string(),
                        "http://localhost:3001".to_string(),
                    ),
                    LimitedRequester::new().await,
                )
                .await
                .unwrap(),
            )
        });
}

struct TestBundle<'a> {
    urls: URLBundle,
    user: User<'a>,
}

async fn setup<'a>() -> TestBundle<'a> {
    let urls = URLBundle::new(
        "http://localhost:3001/api".to_string(),
        "ws://localhost:3001".to_string(),
        "http://localhost:3001".to_string(),
    );
    let requester = LimitedRequester::new().await;
    let mut instance = Instance::new(urls.clone(), requester).await.unwrap();
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
