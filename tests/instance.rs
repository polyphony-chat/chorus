mod common;
// PRETTYFYME: Move common wasm setup to common.rs
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;
#[cfg(target_arch = "wasm32")]
wasm_bindgen_test_configure!(run_in_browser);

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
async fn guild_creation_deletion_wasm() {
    guild_creation_deletion().await
}

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
