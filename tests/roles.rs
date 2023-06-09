mod common;

use chorus::types::{self, RoleCreateModifySchema};

#[tokio::test]
async fn create_and_get_roles() {
    let mut bundle = common::setup().await;
    let role_create_schema: types::RoleCreateModifySchema = RoleCreateModifySchema {
        name: Some("cool person".to_string()),
        permissions: Some("2251804225".to_string()),
        hoist: Some(true),
        icon: None,
        unicode_emoji: Some("".to_string()),
        mentionable: Some(true),
        position: None,
        color: None,
    };
    let guild_id = bundle.guild.id.clone().to_string();
    let role = types::RoleObject::create(&mut bundle.user, &guild_id, role_create_schema)
        .await
        .unwrap();

    let expected = types::RoleObject::get_all(&mut bundle.user, &guild_id)
        .await
        .unwrap()
        .unwrap()
        .iter()
        .nth(1)
        .unwrap()
        .clone();

    assert_eq!(role, expected);
    common::teardown(bundle).await
}
