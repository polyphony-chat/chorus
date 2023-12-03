use chorus::UrlBundle;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;
#[cfg(target_arch = "wasm32")]
wasm_bindgen_test_configure!(run_in_browser);

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn test_parse_url() {
    let url = url::Url::parse("http://localhost:3001/").unwrap();
    UrlBundle::from_root_domain(url.as_str()).await.unwrap();
}
