use chorus::errors::ChorusResult;

mod common;

#[tokio::test]
async fn generate_general_configuration_schema() -> ChorusResult<()> {
    let bundle = common::setup().await;
    bundle.instance.general_configuration_schema().await?;
    common::teardown(bundle).await
}
