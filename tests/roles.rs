use chorus::types::{self, RoleCreateModifySchema, RoleObject};
// PRETTYFYME: Move common wasm setup to common.rs
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;
#[cfg(target_arch = "wasm32")]
wasm_bindgen_test_configure!(run_in_browser);

mod common;

#[tokio::test]
async fn create_and_get_roles() {
    let mut bundle = common::setup().await;
    let permissions = types::PermissionFlags::CONNECT | types::PermissionFlags::MANAGE_EVENTS;
    let permissions = Some(permissions.to_string());
    let role_create_schema: types::RoleCreateModifySchema = RoleCreateModifySchema {
        name: Some("cool person".to_string()),
        permissions,
        hoist: Some(true),
        icon: None,
        unicode_emoji: Some("".to_string()),
        mentionable: Some(true),
        position: None,
        color: None,
    };
    let guild_id = bundle.guild.read().unwrap().id;
    let role = types::RoleObject::create(&mut bundle.user, guild_id, role_create_schema)
        .await
        .unwrap();

    let expected = types::RoleObject::get_all(&mut bundle.user, guild_id)
        .await
        .unwrap()[2]
        .clone();

    assert_eq!(role, expected);
    common::teardown(bundle).await
}

#[tokio::test]
async fn get_and_delete_role() {
    let mut bundle = common::setup().await;
    let guild_id = bundle.guild.read().unwrap().id;
    let role_id = bundle.role.read().unwrap().id;
    let role = bundle.role.read().unwrap().clone();
    let same_role = chorus::types::RoleObject::get(&mut bundle.user, guild_id, role_id)
        .await
        .unwrap();
    assert_eq!(role, same_role);
    assert_eq!(
        chorus::types::RoleObject::get_all(&mut bundle.user, guild_id)
            .await
            .unwrap()
            .len(),
        2
    );
    RoleObject::delete_role(&mut bundle.user, guild_id, role_id, None)
        .await
        .unwrap();
    assert_eq!(
        chorus::types::RoleObject::get_all(&mut bundle.user, guild_id)
            .await
            .unwrap()
            .len(),
        1
    );
    common::teardown(bundle).await
}
