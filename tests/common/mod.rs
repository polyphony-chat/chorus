// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::str::FromStr;

use chorus::gateway::{Gateway, GatewayOptions};
use chorus::types::IntoShared;
use chorus::{
    instance::{ChorusUser, Instance},
    types::{
        Channel, ChannelCreateSchema, Guild, GuildCreateSchema, RegisterSchema,
        RoleCreateModifySchema, RoleObject, Shared
    },
    UrlBundle,
};

use chrono::NaiveDate;

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct TestBundle {
    pub urls: UrlBundle,
    pub user: ChorusUser,
    pub instance: Instance,
    pub guild: Shared<Guild>,
    pub role: Shared<RoleObject>,
    pub channel: Shared<Channel>,
}

#[allow(unused)]
impl TestBundle {
    pub(crate) async fn create_user(&mut self, username: &str) -> ChorusUser {
        let register_schema = RegisterSchema {
            username: username.to_string(),
            consent: true,
            date_of_birth: Some(NaiveDate::from_str("2000-01-01").unwrap()),
            ..Default::default()
        };
        self.instance
            .clone()
            .register_account(register_schema)
            .await
            .unwrap()
    }
    pub(crate) async fn clone_user_without_gateway(&self) -> ChorusUser {
        ChorusUser {
            belongs_to: self.user.belongs_to.clone(),
            token: self.user.token.clone(),
            limits: self.user.limits.clone(),
            settings: self.user.settings.clone(),
            object: self.user.object.clone(),
            gateway: Gateway::spawn(self.instance.urls.wss.clone(), GatewayOptions::default())
                .await
                .unwrap(),
        }
    }
}

// Set up a test by creating an Instance and a User. Reduces Test boilerplate.
pub(crate) async fn setup() -> TestBundle {

    // So we can get logs when tests fail
    let _ = simple_logger::SimpleLogger::with_level(simple_logger::SimpleLogger::new(), log::LevelFilter::Debug).init();

    let instance = Instance::new("http://localhost:3001/api").await.unwrap();
    // Requires the existence of the below user.
    let reg = RegisterSchema {
        username: "integrationtestuser".into(),
        consent: true,
        date_of_birth: Some(NaiveDate::from_str("2000-01-01").unwrap()),
        ..Default::default()
    };
    let guild_create_schema = GuildCreateSchema {
        name: Some("Test-Guild!".to_string()),
        region: None,
        icon: None,
        channels: None,
        guild_template_code: None,
        system_channel_id: None,
        rules_channel_id: None,
    };
    let channel_create_schema = ChannelCreateSchema {
        name: "testchannel".to_string(),
        channel_type: Some(chorus::types::ChannelType::GuildText),
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
    let mut user = instance.clone().register_account(reg).await.unwrap();
    let guild = Guild::create(&mut user, guild_create_schema).await.unwrap();
    let channel = Channel::create(&mut user, guild.id, None, channel_create_schema)
        .await
        .unwrap();

    let role_create_schema: chorus::types::RoleCreateModifySchema = RoleCreateModifySchema {
        name: Some("Bundle role".to_string()),
        permissions: Some("8".to_string()), // Administrator permissions
        hoist: Some(true),
        icon: None,
        unicode_emoji: Some("".to_string()),
        mentionable: Some(true),
        position: None,
        color: None,
    };
    let role = chorus::types::RoleObject::create(&mut user, guild.id, role_create_schema)
        .await
        .unwrap();

    let urls = UrlBundle::new(
        "http://localhost:3001/api".to_string(),
        "http://localhost:3001/api".to_string(),
        "ws://localhost:3001/".to_string(),
        "http://localhost:3001".to_string(),
    );
    TestBundle {
        urls,
        user,
        instance,
        guild: guild.into_shared(),
        role: role.into_shared(),
        channel: channel.into_shared(),
    }
}

// Teardown method to clean up after a test.
#[allow(dead_code)]
pub(crate) async fn teardown(mut bundle: TestBundle) {
    let id = bundle.guild.read().unwrap().id;
    Guild::delete(&mut bundle.user, id).await.unwrap();
    bundle.user.delete().await.unwrap()
}
