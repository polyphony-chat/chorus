// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::str::FromStr;

use chorus::types::{
    LoginSchema, MfaAuthenticationType, MfaVerifySchema, RegisterSchema, SendMfaSmsSchema,
};

#[cfg(not(target_arch = "wasm32"))]
use httptest::{
    matchers::{all_of, contains, eq, json_decoded, request},
    responders::json_encoded,
    Expectation,
};

use serde_json::json;
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

    let token = "invalid token lalalalala";
    let other_user = bundle.instance.login_with_token(token).await;

    assert!(other_user.is_err());

    common::teardown(bundle).await;
}

#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
#[cfg(not(target_arch = "wasm32"))]
async fn test_complete_mfa_challenge_totp() {
    let server = common::create_mock_server();
    let mut bundle = common::setup_with_mock_server(&server).await;

    server.expect(
        Expectation::matching(all_of![
            request::method("POST"),
            request::path("/api/mfa/finish"),
            request::body(json_decoded(eq(
                json!({"ticket": "testticket", "mfa_type": "totp", "data": "testdata"})
            ))),
            request::headers(contains(("authorization", "faketoken")))
        ])
        .respond_with(json_encoded(json!({"token": "testtoken"}))),
    );

    let schema = MfaVerifySchema {
        ticket: "testticket".to_string(),
        mfa_type: MfaAuthenticationType::TOTP,
        data: "testdata".to_string(),
    };

    let result = bundle.user.complete_mfa_challenge(schema).await;

    assert!(result.is_ok());
    assert_eq!(
        bundle.user.mfa_token.unwrap().token,
        "testtoken".to_string()
    );
}

#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
#[cfg(not(target_arch = "wasm32"))]
async fn test_complete_mfa_challenge_sms() {
    let server = common::create_mock_server();
    let mut bundle = common::setup_with_mock_server(&server).await;

    server.expect(
        Expectation::matching(all_of![
            request::method("POST"),
            request::path("/api/mfa/finish"),
            request::body(json_decoded(eq(
                json!({"ticket": "testticket", "mfa_type": "sms", "data": "testdata"})
            ))),
            request::headers(contains(("authorization", "faketoken")))
        ])
        .respond_with(json_encoded(json!({"token": "testtoken"}))),
    );

    let schema = MfaVerifySchema {
        ticket: "testticket".to_string(),
        mfa_type: MfaAuthenticationType::SMS,
        data: "testdata".to_string(),
    };

    let result = bundle.user.complete_mfa_challenge(schema).await;

    assert!(result.is_ok());
    assert_eq!(
        bundle.user.mfa_token.unwrap().token,
        "testtoken".to_string()
    );
}

#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
#[cfg(not(target_arch = "wasm32"))]
async fn test_verify_mfa_login_webauthn() {
    let server = common::create_mock_server();
    let mut bundle = common::setup_with_mock_server(&server).await;

    server.expect(
        Expectation::matching(all_of![
            request::method("POST"),
            request::path("/api/mfa/finish"),
            request::body(json_decoded(eq(
                json!({"ticket": "testticket", "mfa_type": "webauthn", "data": "testdata"})
            ))),
            request::headers(contains(("authorization", "faketoken")))
        ])
        .respond_with(json_encoded(json!({"token": "testtoken"}))),
    );

    let schema = MfaVerifySchema {
        ticket: "testticket".to_string(),
        mfa_type: MfaAuthenticationType::WebAuthn,
        data: "testdata".to_string(),
    };

    let result = bundle.user.complete_mfa_challenge(schema).await;

    assert!(result.is_ok());
    assert_eq!(
        bundle.user.mfa_token.unwrap().token,
        "testtoken".to_string()
    );
}

#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
#[cfg(not(target_arch = "wasm32"))]
async fn test_complete_mfa_challenge_backup() {
    let server = common::create_mock_server();
    let mut bundle = common::setup_with_mock_server(&server).await;

    server.expect(
        Expectation::matching(all_of![
            request::method("POST"),
            request::path("/api/mfa/finish"),
            request::body(json_decoded(eq(
                json!({"ticket": "testticket", "mfa_type": "backup", "data": "testdata"})
            ))),
            request::headers(contains(("authorization", "faketoken")))
        ])
        .respond_with(json_encoded(json!({"token": "testtoken"}))),
    );

    let schema = MfaVerifySchema {
        ticket: "testticket".to_string(),
        mfa_type: MfaAuthenticationType::Backup,
        data: "testdata".to_string(),
    };

    let result = bundle.user.complete_mfa_challenge(schema).await;

    assert!(result.is_ok());
    assert_eq!(
        bundle.user.mfa_token.unwrap().token,
        "testtoken".to_string()
    );
}

#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
#[cfg(not(target_arch = "wasm32"))]
async fn test_complete_mfa_challenge_password() {
    let server = common::create_mock_server();
    let mut bundle = common::setup_with_mock_server(&server).await;

    server.expect(
        Expectation::matching(all_of![
            request::method("POST"),
            request::path("/api/mfa/finish"),
            request::body(json_decoded(eq(
                json!({"ticket": "testticket", "mfa_type": "password", "data": "testdata"})
            ))),
            request::headers(contains(("authorization", "faketoken")))
        ])
        .respond_with(json_encoded(json!({"token": "testtoken"}))),
    );

    let schema = MfaVerifySchema {
        ticket: "testticket".to_string(),
        mfa_type: MfaAuthenticationType::Password,
        data: "testdata".to_string(),
    };

    let result = bundle.user.complete_mfa_challenge(schema).await;

    assert!(result.is_ok())
}

#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
#[cfg(not(target_arch = "wasm32"))]
async fn test_send_mfa_sms() {
    let server = common::create_mock_server();
    let mut bundle = common::setup_with_mock_server(&server).await;

    server.expect(
        Expectation::matching(all_of![
            request::method("POST"),
            request::path("/api/auth/mfa/sms/send"),
            request::body(json_decoded(eq(json!({"ticket": "testticket"})))),
        ])
        .respond_with(json_encoded(json!({"phone": "+*******0085"}))),
    );

    let schema = SendMfaSmsSchema {
        ticket: "testticket".to_string(),
    };

    let result = bundle.instance.send_mfa_sms(schema).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap().phone, "+*******0085".to_string());
}

// Note: user mfa routes are also here, because the other mfa routes were already here
// TODO: Test also not having an mfa token and trying to make a request that needs mfa
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
#[cfg(not(target_arch = "wasm32"))]
async fn test_enable_totp_mfa() {
    use chorus::types::{EnableTotpMfaSchema, MfaBackupCode, Snowflake};

    let server = common::create_mock_server();
    let mut bundle = common::setup_with_mock_server(&server).await;

    // TODO: Once json response codes are implemented, add the case where we can validate a user's
    // password

    server.expect(
        Expectation::matching(all_of![
            request::method("POST"),
            request::path("/api/users/@me/mfa/totp/enable"),
            request::body(json_decoded(eq(json!({"password": "test_password", "secret":"testsecret", "code":"testcode"})))),
				request::headers(contains(("authorization", "faketoken"))),
        ])
        .respond_with(json_encoded(json!({"token": "testtoken", "backup_codes": [{"user_id": "852892297661906993", "code": "zqs8oqxk", "consumed": false}]}))),
    );

    let schema = EnableTotpMfaSchema {
        code: Some("testcode".to_string()),
        password: "test_password".to_string(),
        secret: Some("testsecret".to_string()),
    };

    let result = bundle.user.enable_totp_mfa(schema).await;

    assert!(result.is_ok());
    assert_eq!(
        result.unwrap().backup_codes,
        vec![MfaBackupCode {
            user_id: Snowflake(852892297661906993),
            code: "zqs8oqxk".to_string(),
            consumed: false
        }]
    );
}

#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
#[cfg(not(target_arch = "wasm32"))]
async fn test_disable_totp_mfa() {
    use chrono::{TimeDelta, Utc};

    let server = common::create_mock_server();
    let mut bundle = common::setup_with_mock_server(&server).await;

    bundle.user.mfa_token = Some(chorus::types::MfaToken {
        token: "fakemfatoken".to_string(),
        expires_at: Utc::now() + TimeDelta::minutes(5),
    });

    server.expect(
        Expectation::matching(all_of![
            request::method("POST"),
            request::path("/api/users/@me/mfa/totp/disable"),
            request::headers(contains(("x-discord-mfa-authorization", "fakemfatoken"))),
            request::headers(contains(("authorization", "faketoken"))),
        ])
        .respond_with(json_encoded(json!({"token": "testmfatoken"}))),
    );

    let result = bundle.user.disable_totp_mfa().await;

    assert!(result.is_ok());
}

#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
#[cfg(not(target_arch = "wasm32"))]
async fn test_enable_sms_mfa() {
    use chrono::{TimeDelta, Utc};
    use httptest::responders::status_code;

    let server = common::create_mock_server();
    let mut bundle = common::setup_with_mock_server(&server).await;

    bundle.user.mfa_token = Some(chorus::types::MfaToken {
        token: "fakemfatoken".to_string(),
        expires_at: Utc::now() + TimeDelta::minutes(5),
    });

    server.expect(
        Expectation::matching(all_of![
            request::method("POST"),
            request::path("/api/users/@me/mfa/sms/enable"),
            request::headers(contains(("x-discord-mfa-authorization", "fakemfatoken"))),
            request::headers(contains(("authorization", "faketoken"))),
            request::body(json_decoded(eq(json!({"password": "test_password"})))),
        ])
        .respond_with(status_code(204)),
    );

    let schema = chorus::types::SmsMfaRouteSchema {
        password: "test_password".to_string(),
    };

    let result = bundle.user.enable_sms_mfa(schema).await;

    assert!(result.is_ok());
}

#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
#[cfg(not(target_arch = "wasm32"))]
async fn test_disable_sms_mfa() {
    use chrono::{TimeDelta, Utc};
    use httptest::responders::status_code;

    let server = common::create_mock_server();
    let mut bundle = common::setup_with_mock_server(&server).await;

    bundle.user.mfa_token = Some(chorus::types::MfaToken {
        token: "fakemfatoken".to_string(),
        expires_at: Utc::now() + TimeDelta::minutes(5),
    });

    server.expect(
        Expectation::matching(all_of![
            request::method("POST"),
            request::path("/api/users/@me/mfa/sms/disable"),
            request::headers(contains(("x-discord-mfa-authorization", "fakemfatoken"))),
            request::headers(contains(("authorization", "faketoken"))),
            request::body(json_decoded(eq(json!({"password": "test_password"})))),
        ])
        .respond_with(status_code(204)),
    );

    let schema = chorus::types::SmsMfaRouteSchema {
        password: "test_password".to_string(),
    };

    let result = bundle.user.disable_sms_mfa(schema).await;

    assert!(result.is_ok());
}

#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
#[cfg(not(target_arch = "wasm32"))]
async fn test_get_mfa_webauthn_authenticators() {
    use chorus::types::{MfaAuthenticator, Snowflake};
    use chrono::{TimeDelta, Utc};

    let server = common::create_mock_server();
    let mut bundle = common::setup_with_mock_server(&server).await;

    bundle.user.mfa_token = Some(chorus::types::MfaToken {
        token: "fakemfatoken".to_string(),
        expires_at: Utc::now() + TimeDelta::minutes(5),
    });

    server.expect(
        Expectation::matching(all_of![
            request::method("GET"),
            request::path("/api/users/@me/mfa/webauthn/credentials"),
            request::headers(contains(("authorization", "faketoken"))),
        ])
        .respond_with(json_encoded(
            json!([{"id": "1219430671865610261", "type": 1, "name": "Alienkey"}]),
        )),
    );

    let result = bundle.user.get_webauthn_authenticators().await;

    assert_eq!(
        result.unwrap(),
        vec![MfaAuthenticator {
            id: Snowflake(1219430671865610261),
            name: "Alienkey".to_string(),
            authenticator_type: chorus::types::MfaAuthenticatorType::WebAuthn
        }]
    );
}

#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
#[cfg(not(target_arch = "wasm32"))]
async fn test_create_mfa_webauthn_authenticator() {
    use chorus::types::{
        FinishWebAuthnAuthenticatorCreationReturn, FinishWebAuthnAuthenticatorCreationSchema,
        MfaAuthenticator, MfaBackupCode, Snowflake,
    };
    use chrono::{TimeDelta, Utc};

    let server = common::create_mock_server();
    let mut bundle = common::setup_with_mock_server(&server).await;

    bundle.user.mfa_token = Some(chorus::types::MfaToken {
        token: "fakemfatoken".to_string(),
        expires_at: Utc::now() + TimeDelta::minutes(5),
    });

    // Begin creation
    server.expect(
        Expectation::matching(all_of![
            request::method("POST"),
            request::path("/api/users/@me/mfa/webauthn/credentials"),
				request::headers(contains(("authorization", "faketoken"))),
            request::headers(contains(("x-discord-mfa-authorization", "fakemfatoken"))),
        ])
        .respond_with(json_encoded(json!({"ticket": "ODUyODkyMjk3NjYxOTA2OTkz.WrhGhYEhM3lHUPN61xF6JcQKwVutk8fBvcoHjo", "challenge": "{\"publicKey\":{\"challenge\":\"a8a1cHP7_zYheggFG68zKUkl8DwnEqfKvPE-GOMvhss\",\"timeout\":60000,\"rpId\":\"discord.com\",\"allowCredentials\":[{\"type\":\"public-key\",\"id\":\"izrvF80ogrfg9dC3RmWWwW1VxBVBG0TzJVXKOJl__6FvMa555dH4Trt2Ub8AdHxNLkQsc0unAGcn4-hrJHDKSO\"}],\"userVerification\":\"preferred\"}}"}))),
    );

    // Finish creation
    server.expect(
        Expectation::matching(all_of![
            request::method("POST"),
            request::path("/api/users/@me/mfa/webauthn/credentials"),
				request::headers(contains(("authorization", "faketoken"))),
            request::headers(contains(("x-discord-mfa-authorization", "fakemfatoken"))),
            request::body(json_decoded(eq(json!({"name": "AlienKey", "ticket": "ODUyODkyMjk3NjYxOTA2OTkz.WrhGhYEhM3lHUPN61xF6JcQKwVutk8fBvcoHjo", "credential": "{\"test\": \"lest\"}"})))),
        ])
        .respond_with(json_encoded(json!({  "id": "1219430671865610261",
  "type": 1,
  "name": "AlienKey",
  "backup_codes": [
    {
      "user_id": "852892297661906993",
      "code": "zqs8oqxk",
      "consumed": false
    }
  ]}))),
    );

    let result = bundle
        .user
        .begin_webauthn_authenticator_creation()
        .await
        .unwrap();

    let schema = FinishWebAuthnAuthenticatorCreationSchema {
        name: "AlienKey".to_string(),
        ticket: result.ticket,
        credential: "{\"test\": \"lest\"}".to_string(),
    };

    let result = bundle
        .user
        .finish_webauthn_authenticator_creation(schema)
        .await;

    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        FinishWebAuthnAuthenticatorCreationReturn {
            backup_codes: vec![MfaBackupCode {
                user_id: Snowflake(852892297661906993),
                code: "zqs8oqxk".to_string(),
                consumed: false
            }],
            authenticator: MfaAuthenticator {
                name: "AlienKey".to_string(),
                id: Snowflake(1219430671865610261),
                authenticator_type: chorus::types::MfaAuthenticatorType::WebAuthn
            }
        }
    );
}

#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
#[cfg(not(target_arch = "wasm32"))]
async fn test_modify_mfa_webauthn_authenticator() {
    use chorus::types::{MfaAuthenticator, ModifyWebAuthnAuthenticatorSchema, Snowflake};
    use chrono::{TimeDelta, Utc};

    let server = common::create_mock_server();
    let mut bundle = common::setup_with_mock_server(&server).await;

    bundle.user.mfa_token = Some(chorus::types::MfaToken {
        token: "fakemfatoken".to_string(),
        expires_at: Utc::now() + TimeDelta::minutes(5),
    });

    server.expect(
        Expectation::matching(all_of![
            request::method("PATCH"),
            request::path("/api/users/@me/mfa/webauthn/credentials/1219430671865610261"),
            request::headers(contains(("authorization", "faketoken"))),
            request::headers(contains(("x-discord-mfa-authorization", "fakemfatoken"))),
            request::body(json_decoded(eq(json!({"name": "Alienkey Pro Ultra SE+"})))),
        ])
        .respond_with(json_encoded(
            json!({  "id": "1219430671865610261", "type": 1, "name": "Alienkey Pro Ultra SE+" }),
        )),
    );

    let schema = ModifyWebAuthnAuthenticatorSchema {
        name: Some("Alienkey Pro Ultra SE+".to_string()),
    };

    let result = bundle
        .user
        .modify_webauthn_authenticator(Snowflake(1219430671865610261), schema)
        .await;

    assert!(result.is_ok());
    assert_eq!(
        result.unwrap(),
        MfaAuthenticator {
            name: "Alienkey Pro Ultra SE+".to_string(),
            id: Snowflake(1219430671865610261),
            authenticator_type: chorus::types::MfaAuthenticatorType::WebAuthn
        }
    );
}

#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
#[cfg(not(target_arch = "wasm32"))]
async fn test_delete_mfa_webauthn_authenticator() {
    use chorus::types::Snowflake;
    use chrono::{TimeDelta, Utc};
    use httptest::responders::status_code;

    let server = common::create_mock_server();
    let mut bundle = common::setup_with_mock_server(&server).await;

    bundle.user.mfa_token = Some(chorus::types::MfaToken {
        token: "fakemfatoken".to_string(),
        expires_at: Utc::now() + TimeDelta::minutes(5),
    });

    server.expect(
        Expectation::matching(all_of![
            request::method("DELETE"),
            request::path("/api/users/@me/mfa/webauthn/credentials/1219430671865610261"),
            request::headers(contains(("authorization", "faketoken"))),
            request::headers(contains(("x-discord-mfa-authorization", "fakemfatoken"))),
        ])
        .respond_with(status_code(204)),
    );

    let result = bundle
        .user
        .delete_webauthn_authenticator(Snowflake(1219430671865610261))
        .await;

    assert!(result.is_ok());
}

#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
#[cfg(not(target_arch = "wasm32"))]
// Tests the send backup codes challenge and get backup codes endpoints
async fn test_send_mfa_backup_codes() {
    use chorus::types::{MfaBackupCode, SendBackupCodesChallengeReturn, Snowflake};

    let server = common::create_mock_server();
    let mut bundle = common::setup_with_mock_server(&server).await;

    server.expect(
        Expectation::matching(all_of![
            request::method("POST"),
            request::path("/api/auth/verify/view-backup-codes-challenge"),
            request::headers(contains(("authorization", "faketoken"))),
            request::body(json_decoded(eq(json!({"password": "test_password"})))),
        ])
		  .times(1)
        .respond_with(json_encoded(json!({"nonce": "test_view_nonce", "regenerate_nonce": "test_regenerate_nonce"}))),
    );

	 let schema = chorus::types::SendBackupCodesChallengeSchema { password: "test_password".to_string() };

    let result = bundle
        .user
        .send_backup_codes_challenge(schema)
		  .await.unwrap();

	 assert_eq!(result, SendBackupCodesChallengeReturn {view_nonce: "test_view_nonce".to_string(), regenerate_nonce: "test_regenerate_nonce".to_string() });

	 // View routes, assume we got an email key of "test_key"
	 // View nonce, regenerate = false
	 server.expect(
        Expectation::matching(all_of![
            request::method("POST"),
            request::path("/api/users/@me/mfa/codes-verification"),
            request::headers(contains(("authorization", "faketoken"))),
            request::body(json_decoded(eq(json!({"key": "test_key", "nonce": "test_view_nonce", "regenerate": false})))),
        ])
		  .times(1)
        .respond_with(json_encoded(json!([{"user_id": "852892297661906993", "code": "zqs8oqxk", "consumed": false}]))),
    );

	 // Regenerate nonce, regenerate = true
	 server.expect(
        Expectation::matching(all_of![
            request::method("POST"),
            request::path("/api/users/@me/mfa/codes-verification"),
            request::headers(contains(("authorization", "faketoken"))),
            request::body(json_decoded(eq(json!({"key": "test_key", "nonce": "test_regenerate_nonce", "regenerate": true})))),
        ])
		  .times(1)
        .respond_with(json_encoded(json!([{"user_id": "852892297661906993", "code": "oqxk8zqs", "consumed": false}]))),
    );

	 let schema_view = chorus::types::GetBackupCodesSchema { nonce: result.view_nonce, key: "test_key".to_string(), regenerate: false };

	 let schema_regenerate = chorus::types::GetBackupCodesSchema { nonce: result.regenerate_nonce, key: "test_key".to_string(), regenerate: true };

	 let result_view = bundle.user.get_backup_codes(schema_view).await.unwrap();

	 assert_eq!(result_view, vec![MfaBackupCode {user_id: Snowflake(852892297661906993), code: "zqs8oqxk".to_string(), consumed: false}]);

	 let result_regenerate = bundle.user.get_backup_codes(schema_regenerate).await.unwrap();

	 assert_ne!(result_view, result_regenerate);
	 assert_eq!(result_regenerate, vec![MfaBackupCode {user_id: Snowflake(852892297661906993), code: "oqxk8zqs".to_string(), consumed: false}]);
}
