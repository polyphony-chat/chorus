use chorus::types::{RegisterSchema, RegisterSchemaOptions};

mod common;

#[tokio::test]
async fn test_registration() {
    let mut bundle = common::setup().await;
    let reg = RegisterSchemaOptions {
        date_of_birth: Some("2000-01-01".to_string()),
        ..RegisterSchema::builder("Hiiii", true)
    }
    .build()
    .unwrap();
    bundle.instance.register_account(&reg).await.unwrap();
    common::teardown(bundle).await;
}
