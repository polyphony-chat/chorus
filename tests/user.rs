// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use chorus::types::{
        DeleteDisableUserSchema, PublicUser, Snowflake, User,
        UserModifyProfileSchema,
    };
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;
#[cfg(target_arch = "wasm32")]
wasm_bindgen_test_configure!(run_in_browser);

mod common;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn to_public_user() {
    let mut user = User::default();
    let mut public_user = PublicUser {
        username: Some("".to_string()),
        discriminator: Some("".to_string()),
        ..Default::default()
    };
    let id: Snowflake = 1_u64.into();
    user.id = id;
    public_user.id = id;

    let from_user = user.into_public_user();
    assert_eq!(public_user, from_user);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn test_get_user_profile() {
    let mut bundle = common::setup().await;

    let user_id = bundle.user.object.read().unwrap().id;

    let user_profile = bundle
        .user
        .get_user_profile(user_id, chorus::types::GetUserProfileSchema::default())
        .await;

    assert!(user_profile.is_ok());

    common::teardown(bundle).await;
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn test_modify_user_profile() {
    let mut bundle = common::setup().await;

    let bio = Some(String::from("A user."));
    let pronouns = Some(String::from("they/them"));

    let modify = UserModifyProfileSchema {
        bio: bio.clone(),
        pronouns: pronouns.clone(),
        ..Default::default()
    };

    bundle.user.modify_profile(modify).await.unwrap();

    let user_id = bundle.user.object.read().unwrap().id;

    let user_profile = bundle
        .user
        .get_user_profile(user_id, chorus::types::GetUserProfileSchema::default())
        .await
        .unwrap();

    assert_eq!(user_profile.profile_metadata.bio, bio);
    assert_eq!(user_profile.profile_metadata.pronouns, pronouns.unwrap());

    common::teardown(bundle).await;
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn test_disable_user() {
    let mut bundle = common::setup().await;

    let mut other_user = bundle.create_user("integrationtestuser4").await;

    other_user
        .disable(DeleteDisableUserSchema { password: None })
        .await
        .unwrap();

    common::teardown(bundle).await;
}

// Note: these two tests are currently broken.
// FIXME: readd them once bitfl0wer/server#2 is merged
/*
#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn test_get_user_note() {
    let mut bundle = common::setup().await;

    let mut other_user = bundle.create_user("integrationtestuser3").await;

    let user_id = bundle.user.object.read().unwrap().id;
    let other_user_id = other_user.object.read().unwrap().id;

    let result = bundle.user.get_user_note(other_user_id).await;
    assert!(matches!(
        result.err().unwrap(),
        ChorusError::NotFound { .. }
    ));

    bundle
        .user
        .set_user_note(other_user_id, Some(String::from("A note.")))
        .await
        .unwrap();

     assert!(false);

    let result = bundle.user.get_user_note(other_user_id).await;
    assert_eq!(
        result,
        Ok(UserNote {
            user_id,
            note_user_id: other_user_id,
            note: String::from("A note.")
        })
    );

    other_user
        .delete(DeleteDisableUserSchema { password: None })
        .await
        .unwrap();
    common::teardown(bundle).await;
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn test_set_user_note() {
    let mut bundle = common::setup().await;

    let mut other_user = bundle.create_user("integrationtestuser3").await;

    let user_id = bundle.user.object.read().unwrap().id;
    let other_user_id = other_user.object.read().unwrap().id;

    bundle
        .user
        .set_user_note(other_user_id, Some(String::from("A note.")))
        .await
        .unwrap();

    let result = bundle.user.get_user_note(other_user_id).await;
    assert_eq!(
        result,
        Ok(UserNote {
            user_id,
            note_user_id: other_user_id,
            note: String::from("A note.")
        })
    );

    other_user
        .delete(DeleteDisableUserSchema { password: None })
        .await
        .unwrap();
    common::teardown(bundle).await;
}*/

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn test_get_user_affinities() {
    let mut bundle = common::setup().await;

    let result = bundle.user.get_user_affinities().await.unwrap();

    assert!(result.user_affinities.is_empty());
    assert!(result.inverse_user_affinities.is_empty());

    common::teardown(bundle).await;
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn test_get_guild_affinities() {
    let mut bundle = common::setup().await;

    let result = bundle.user.get_guild_affinities().await.unwrap();

    assert!(result.guild_affinities.is_empty());

    common::teardown(bundle).await;
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn test_get_connections() {
    let mut bundle = common::setup().await;

    let result = bundle.user.get_connections().await.unwrap();

    // We can't *really* test creating or getting connections...
    // TODO: Find a way?
    assert!(result.is_empty());

    common::teardown(bundle).await;
}
