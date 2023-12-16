mod common;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;
#[cfg(target_arch = "wasm32")]
wasm_bindgen_test_configure!(run_in_browser);

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn generate_general_configuration_schema() {
    let bundle = common::setup().await;
    bundle
        .instance
        .general_configuration_schema()
        .await
        .unwrap();
    common::teardown(bundle).await;
}
