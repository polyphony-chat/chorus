use chorus::{errors::ChorusResult, types::RegisterSchema};

mod common;

#[tokio::test]
async fn test_registration() -> ChorusResult<()> {
    let mut bundle = common::setup().await;
    let reg = RegisterSchema {
        username: "Hiiii".into(),
        date_of_birth: Some("2000-01-01".to_string()),
        consent: true,
        ..Default::default()
    };
    bundle.instance.register_account(&reg).await?;
    common::teardown(bundle).await
}
