// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use chorus::types::{LoginSchema, RegisterSchema};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;
#[cfg(target_arch = "wasm32")]
wasm_bindgen_test_configure!(run_in_browser);

mod common;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn test_registration() {
    let mut bundle = common::setup().await;
    let reg = RegisterSchema {
        username: "Hiiii".into(),
        date_of_birth: Some("2000-01-01".to_string()),
        consent: true,
        ..Default::default()
    };
    bundle.instance.register_account(reg).await.unwrap();
    common::teardown(bundle).await;
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn test_login() {
    let mut bundle = common::setup().await;
    let reg = RegisterSchema {
        username: "Hiiii".into(),
        email: Some("testuser1@integrationtesting.xyz".into()),
        password: Some("Correct-Horse-Battery-Staple1".into()),
        date_of_birth: Some("2000-01-01".to_string()),
        consent: true,
        ..Default::default()
    };
    bundle.instance.register_account(reg).await.unwrap();
    let login = LoginSchema {
        login: "testuser1@integrationtesting.xyz".into(),
        password: "Correct-Horse-Battery-Staple1".into(),
        ..Default::default()
    };
    bundle.instance.login_account(login).await.unwrap();
    common::teardown(bundle).await;
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn test_wrong_login() {
    let mut bundle = common::setup().await;
    let reg = RegisterSchema {
        username: "Hiiii".into(),
        email: Some("testuser2@integrationtesting.xyz".into()),
        password: Some("Correct-Horse-Battery-Staple1".into()),
        date_of_birth: Some("2000-01-01".to_string()),
        consent: true,
        ..Default::default()
    };
    bundle.instance.register_account(reg).await.unwrap();
    let login = LoginSchema {
        login: "testuser2@integrationtesting.xyz".into(),
        password: "Correct-Horse-Battery-Staple2".into(),
        ..Default::default()
    };
    let res = bundle.instance.login_account(login).await;
    assert!(res.is_err());
    common::teardown(bundle).await;
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn test_login_with_token() {
    let mut bundle = common::setup().await;

    let token = &bundle.user.token;
    let other_user = bundle
        .instance
        .login_with_token(token.clone())
        .await
        .unwrap();
    assert_eq!(
        bundle.user.object.read().unwrap().id,
        other_user.object.read().unwrap().id
    );
    assert_eq!(bundle.user.token, other_user.token);

    common::teardown(bundle).await;
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn test_login_with_invalid_token() {
    let mut bundle = common::setup().await;

    let token = "invalid token lalalalala".to_string();
    let other_user = bundle.instance.login_with_token(token.clone()).await;

    assert!(other_user.is_err());

    common::teardown(bundle).await;
}
