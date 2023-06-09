mod common;

#[tokio::test]
async fn add_remove_role() {
    let mut bundle = common::setup().await;
    let guild_id = &bundle.guild.id.to_string();
    let role_id = &bundle.role.id.to_string();
    let user_id = &bundle.user.object.id.to_string();
    chorus::types::GuildMember::add_role(&mut bundle.user, guild_id, user_id, role_id).await;
    chorus::types::GuildMember::remove_role(&mut bundle.user, guild_id, user_id, role_id).await;
    // TODO: Implement /guilds/{guild_id}/members/{member_id}/ GET route.
    common::teardown(bundle).await
}
