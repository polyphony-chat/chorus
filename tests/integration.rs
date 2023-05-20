use chorus::{instance::Instance, limit::LimitedRequester, URLBundle};

struct TestBundle {
    urls: URLBundle,
    instance: Instance,
}

async fn setup() -> TestBundle {
    let urls = URLBundle::new(
        "http://localhost:3001/api".to_string(),
        "ws://localhost:3001".to_string(),
        "http://localhost:3001".to_string(),
    );
    let requester = LimitedRequester::new().await;
    let instance = Instance::new(urls.clone(), requester).await.unwrap();

    TestBundle { urls, instance }
}
