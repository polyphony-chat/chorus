mod common;

use chorus::types;

#[tokio::test]
async fn test_registration() {
    let mut bundle = common::setup().await;
    let reg = types::RegisterSchema::new(
        "Hiiii".to_string(),
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
    bundle.instance.register_account(&reg).await.unwrap();
    common::teardown(bundle).await;
}

#[tokio::test]
async fn test_login() {
    let mut bundle = common::setup().await;
    let login_schema = types::LoginSchema::new(
        "integrationtestuser".to_string(),
        None,
        Some(false),
        None,
        None,
        None,
    );
    bundle
        .instance
        .login_account(&login_schema.unwrap())
        .await
        .unwrap();
    common::teardown(bundle).await;
}
