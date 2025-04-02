// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod common;
use chorus::instance::InstanceSoftware;
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

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn detect_instance_software() {
    let bundle = common::setup().await;

    let software = bundle.instance.detect_software().await;
    assert_eq!(software, InstanceSoftware::SpacebarTypescript);

    assert_eq!(bundle.instance.software(), InstanceSoftware::SpacebarTypescript);

    common::teardown(bundle).await;
}
