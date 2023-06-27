use chorus::{
    errors::ChorusResult,
    instance::{Instance, UserMeta},
    types::{
        Channel, ChannelCreateSchema, Guild, GuildCreateSchema, RegisterSchema,
        RoleCreateModifySchema, RoleObject,
    },
    UrlBundle,
};

#[derive(Debug)]
pub struct TestBundle {
    pub urls: UrlBundle,
    pub user: UserMeta,
    pub other_user: UserMeta,
    pub instance: Instance,
    pub guild: Guild,
    pub role: RoleObject,
    pub channel: Channel,
}

pub const API: &str = "http://localhost:3001/api";
pub const WSS: &str = "ws://localhost:3001";
pub const CDN: &str = "http://localhost:3001";

// Set up a test by creating an Instance and a User. Reduces Test boilerplate.
pub async fn setup() -> TestBundle {
    let urls = UrlBundle::new(API.to_string(), WSS.to_string(), CDN.to_string());
    let mut instance = Instance::new(urls.clone()).await.unwrap();
    // Requires the existance of the below user.
    let reg = RegisterSchema {
        username: "integrationtestuser".into(),
        consent: true,
        date_of_birth: Some("2000-01-01".to_string()),
        ..Default::default()
    };
    let reg_2 = RegisterSchema {
        username: "integrationtestuser2".to_string(),
        consent: true,
        date_of_birth: Some("2000-01-01".to_string()),
        ..Default::default()
    };
    let guild_create_schema = GuildCreateSchema {
        name: Some("Test-Guild!".to_string()),
        ..Default::default()
    };
    let channel_create_schema = ChannelCreateSchema {
        name: "testchannel".to_string(),
        channel_type: Some(0),
        topic: None,
        icon: None,
        bitrate: None,
        user_limit: None,
        rate_limit_per_user: None,
        position: None,
        permission_overwrites: None,
        parent_id: None,
        id: None,
        nsfw: Some(false),
        rtc_region: None,
        default_auto_archive_duration: None,
        default_reaction_emoji: None,
        flags: Some(0),
        default_thread_rate_limit_per_user: Some(0),
        video_quality_mode: None,
    };
    let mut user = instance.register_account(&reg).await.unwrap();
    let other_user = instance.register_account(&reg_2).await.unwrap();
    let guild = Guild::create(&mut user, guild_create_schema).await.unwrap();
    let channel = Channel::create(&mut user, guild.id, channel_create_schema)
        .await
        .unwrap();

    let role_create_schema = RoleCreateModifySchema {
        name: Some("Bundle role".to_string()),
        permissions: Some("8".to_string()), // Administrator permissions
        hoist: Some(true),
        icon: None,
        unicode_emoji: Some("".to_string()),
        mentionable: Some(true),
        position: None,
        color: None,
    };
    let role = RoleObject::create(&mut user, guild.id, role_create_schema)
        .await
        .unwrap();

    TestBundle {
        urls,
        user,
        other_user,
        instance,
        guild,
        role,
        channel,
    }
}

// Teardown method to clean up after a test.
#[allow(dead_code)]
pub async fn teardown(mut bundle: TestBundle) -> ChorusResult<()> {
    Guild::delete(&mut bundle.user, bundle.guild.id).await?;
    bundle.user.delete().await?;
    bundle.other_user.delete().await
}
