mod common;

#[tokio::test]
async fn add_remove_role() {
    let mut bundle = common::setup().await;
    let guild_id = &bundle.guild.id.to_string();
    let role_id = &bundle.role.id.to_string();
    let user_id = &bundle.user.object.id.to_string();
    chorus::types::GuildMember::add_role(&mut bundle.user, guild_id, user_id, role_id).await;
    let member = chorus::types::GuildMember::get(&mut bundle.user, guild_id, user_id)
        .await
        .unwrap();
    let mut role_found = false;
    for role in member.roles.iter() {
        if role == role_id {
            println!("Role found: {:?}", role);
            role_found = true;
        }
    }
    if !role_found {
        panic!()
    }
    chorus::types::GuildMember::remove_role(&mut bundle.user, guild_id, user_id, role_id).await;
    let member = chorus::types::GuildMember::get(&mut bundle.user, guild_id, user_id)
        .await
        .unwrap();
    for role in member.roles.iter() {
        if role != role_id {
            role_found = false;
        } else {
            panic!();
        }
    }
    if role_found {
        panic!()
    }
    common::teardown(bundle).await
}
