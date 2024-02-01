// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use chorus::types::types::domains_configuration::WellKnownResponse;
use chorus::UrlBundle;
use serde_json::json;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;
#[cfg(target_arch = "wasm32")]
wasm_bindgen_test_configure!(run_in_browser);

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn test_parse_url() {
    // TODO: Currently only tests two of the three branches in UrlBundle::from_root_domain.
    let url = url::Url::parse("http://localhost:3001/").unwrap();
    UrlBundle::from_root_url(url.as_str()).await.unwrap();
    let url = url::Url::parse("http://localhost:3001/api/").unwrap();
    UrlBundle::from_root_url(url.as_str()).await.unwrap();
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn test_parse_wellknown() {
    let json = json!({
        "api": "http://localhost:3001/api/v9"
    });
    let _well_known: WellKnownResponse = serde_json::from_value(json).unwrap();
}
