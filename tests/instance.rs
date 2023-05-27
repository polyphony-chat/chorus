mod common;

#[tokio::test]
async fn generate_general_configuration_schema() {
    let bundle = common::setup().await;
    bundle
        .instance
        .general_configuration_schema()
        .await
        .unwrap();
    common::teardown(bundle).await;
}
