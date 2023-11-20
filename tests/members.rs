use chorus::{errors::ChorusResult, types::GuildMember};
// PRETTYFYME: Move common wasm setup to common.rs
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;
#[cfg(target_arch = "wasm32")]
wasm_bindgen_test_configure!(run_in_browser);

mod common;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test]
async fn add_remove_role_wasm() {
    add_remove_role().unwrap()
}

#[tokio::test]
async fn add_remove_role() -> ChorusResult<()> {
    let mut bundle = common::setup().await;
    let guild = bundle.guild.read().unwrap().id;
    let role = bundle.role.read().unwrap().id;
    let member_id = bundle.user.object.read().unwrap().id;
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
