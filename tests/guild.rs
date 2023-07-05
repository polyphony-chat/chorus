use chorus::{
    errors::ChorusResult,
    types::{Guild, GuildCreateSchema},
};

mod common;

#[tokio::test]
async fn guild_creation_deletion() -> ChorusResult<()> {
    let mut bundle = common::setup().await;

    let guild_create_schema = GuildCreateSchema {
        name: Some("test".to_string()),
        region: None,
        icon: None,
        channels: None,
        guild_template_code: None,
        system_channel_id: None,
        rules_channel_id: None,
    };

    let guild = Guild::create(&mut bundle.user, guild_create_schema).await?;

    Guild::delete(&mut bundle.user, guild.id).await?;
    common::teardown(bundle).await
}

#[tokio::test]
async fn get_channels() -> ChorusResult<()> {
    let mut bundle = common::setup().await;
    bundle.guild.channels(&mut bundle.user).await?;
    common::teardown(bundle).await
}
