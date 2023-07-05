use chorus::{
    errors::ChorusResult,
    types::{PermissionFlags, RoleCreateModifySchema, RoleObject},
};

mod common;

#[tokio::test]
async fn create_and_get_roles() -> ChorusResult<()> {
    let mut bundle = common::setup().await;
    let permissions = PermissionFlags::CONNECT | PermissionFlags::MANAGE_EVENTS;
    let role_create_schema = RoleCreateModifySchema {
        name: Some("cool person".to_string()),
        permissions: Some(permissions.to_string()),
        hoist: Some(true),
        icon: None,
        unicode_emoji: Some("".to_string()),
        mentionable: Some(true),
        position: None,
        color: None,
    };
    let guild = bundle.guild.id;
    let role = RoleObject::create(&mut bundle.user, guild, role_create_schema).await?;

    let expected = RoleObject::get_all(&mut bundle.user, guild).await?[2].clone();

    assert_eq!(role, expected);
    common::teardown(bundle).await
}

#[tokio::test]
async fn get_singular_role() -> ChorusResult<()> {
    let mut bundle = common::setup().await;
    let role = bundle.role.clone();
    let same_role = RoleObject::get(&mut bundle.user, bundle.guild.id, bundle.role.id).await?;
    assert_eq!(role, same_role);
    common::teardown(bundle).await
}
