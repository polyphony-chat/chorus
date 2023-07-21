use chorus::{errors::ChorusResult, types::GuildMember};

mod common;

#[tokio::test]
async fn add_remove_role() -> ChorusResult<()> {
    let mut bundle = common::setup().await;
    let guild = bundle.guild.id;
    let role = bundle.role.id;
    let member_id = bundle.user.object.id;
    GuildMember::add_role(&mut bundle.user, guild, member_id, role).await?;
    let member = GuildMember::get(&mut bundle.user, guild, member_id)
        .await
        .unwrap();
    assert!(member.roles.contains(&role));

    GuildMember::remove_role(&mut bundle.user, guild, member_id, role).await?;
    let member = GuildMember::get(&mut bundle.user, guild, member_id)
        .await
        .unwrap();
    assert!(!member.roles.contains(&role));

    common::teardown(bundle).await;
    Ok(())
}
