// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{os::unix::fs::chroot, str::FromStr};

use chorus::{instance::ChorusUser, types::{ReadyAuthenticatorType, LoginSchema, MfaVerifySchema, RegisterSchema, SendMfaSmsSchema}};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;
#[cfg(target_arch = "wasm32")]
wasm_bindgen_test_configure!(run_in_browser);

use chrono::NaiveDate;

mod common;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn test_registration() {
    let mut bundle = common::setup().await;
    let reg = RegisterSchema {
        username: "Hiiii".into(),
        date_of_birth: Some(NaiveDate::from_str("2000-01-01").unwrap()),
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
        date_of_birth: Some(NaiveDate::from_str("2000-01-01").unwrap()),
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
        date_of_birth: Some(NaiveDate::from_str("2000-01-01").unwrap()),
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
    let other_user = bundle.instance.login_with_token(token).await.unwrap();
    assert_eq!(
        bundle.user.object.as_ref().unwrap().read().unwrap().id,
        other_user.object.unwrap().read().unwrap().id
    );
    assert_eq!(bundle.user.token, other_user.token);

    common::teardown(bundle).await;
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn test_login_with_invalid_token() {
    let mut bundle = common::setup().await;

    let token = "invalid token lalalalala";
    let other_user = bundle.instance.login_with_token(token).await;

    assert!(other_user.is_err());

    common::teardown(bundle).await;
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn test_complete_mfa_challenge_totp() {
    let mut bundle = common::setup().await;

    let token = "".to_string();
    let mut chorus_user = bundle.instance.login_with_token(token).await
        .unwrap();

    let schema = MfaVerifySchema {
        ticket: "".to_string(),
        mfa_type: ReadyAuthenticatorType::TOTP,
        data: "".to_string(),
    };

    let result = chorus_user.complete_mfa_challenge(schema)
        .await;

    assert!(result.is_ok())
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn test_complete_mfa_challenge_sms() {
    let mut bundle = common::setup().await;

    let token = "".to_string();
    let mut chorus_user = bundle.instance.login_with_token(token).await
        .unwrap();

    let schema = MfaVerifySchema {
        ticket: "".to_string(),
        mfa_type: ReadyAuthenticatorType::SMS,
        data: "".to_string(),
    };

    let result = chorus_user.complete_mfa_challenge(schema)
        .await;

    assert!(result.is_ok())
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn test_verify_mfa_login_webauthn() {
    let mut bundle = common::setup().await;

    let token = "".to_string();
    let mut chorus_user = bundle.instance.login_with_token(token).await
        .unwrap();

    let schema = MfaVerifySchema {
        ticket: "".to_string(),
        mfa_type: ReadyAuthenticatorType::SMS,
        data: "".to_string(),
    };

    let result = chorus_user.complete_mfa_challenge(schema)
        .await;

    assert!(result.is_ok())
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn test_complete_mfa_challenge_backup() {
    let mut bundle = common::setup().await;

    let token = "".to_string();
    let mut chorus_user = bundle.instance.login_with_token(token).await
        .unwrap();

    let schema = MfaVerifySchema {
        ticket: "".to_string(),
        mfa_type: ReadyAuthenticatorType::Backup,
        data: "".to_string(),
    };

    let result = chorus_user.complete_mfa_challenge(schema)
        .await;

    assert!(result.is_ok())
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn test_complete_mfa_challenge_password() {
    let mut bundle = common::setup().await;

    let token = "".to_string();
    let mut chorus_user = bundle.instance.login_with_token(token).await
        .unwrap();

    let schema = MfaVerifySchema {
        ticket: "".to_string(),
        mfa_type: ReadyAuthenticatorType::Password,
        data: "".to_string(),
    };

    let result = chorus_user.complete_mfa_challenge(schema)
        .await;

    assert!(result.is_ok())
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn test_send_mfa_sms() {
    let mut bundle = common::setup().await;

    let schema = SendMfaSmsSchema { ticket: "".to_string() };

    let result = bundle.instance.send_mfa_sms(schema).await;

    assert!(result.is_ok())
}
