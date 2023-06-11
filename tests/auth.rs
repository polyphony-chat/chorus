use chorus::types;

mod common;

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
