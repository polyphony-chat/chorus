// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::str::FromStr;

use chorus::gateway::{Gateway, GatewayOptions};
use chorus::types::{DeleteDisableUserSchema, IntoShared, PermissionFlags, Snowflake};
use chorus::{
    instance::{ChorusUser, Instance},
    types::{
        Channel, ChannelCreateSchema, Guild, GuildCreateSchema, RegisterSchema,
        RoleCreateModifySchema, RoleObject, Shared,
    },
    UrlBundle,
};

use chrono::NaiveDate;

#[cfg(not(target_arch = "wasm32"))]
use httptest::{
    matchers::{all_of, contains, request},
    responders::{json_encoded, status_code},
    Expectation,
};

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
            mfa_token: None,
            limits: self.user.limits.clone(),
            settings: self.user.settings.clone(),
            object: self.user.object.clone(),
            gateway: Gateway::spawn(&self.instance.urls.wss, GatewayOptions::default())
                .await
                .unwrap(),
        }
    }
}

/// Set up a test by creating an [Instance] and a User for a real,
/// running server at localhost:3001. Reduces Test boilerplate.
#[allow(dead_code)]
pub(crate) async fn setup() -> TestBundle {
    // So we can get logs when tests fail
    let _ = simple_logger::SimpleLogger::with_level(
        simple_logger::SimpleLogger::new(),
        log::LevelFilter::Debug,
    )
    .init();

    let instance = Instance::new("http://localhost:3001/api", None)
        .await
        .unwrap();
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
        permissions: PermissionFlags::from_bits(8), // Administrator permissions
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
        "http://localhost:3001/api",
        "http://localhost:3001/api",
        "ws://localhost:3001/",
        "http://localhost:3001",
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

/// Set up a test by creating an [Instance] and a User for a mocked
/// server with httptest. Reduces Test boilerplate.
///
/// Note: httptest does not work on wasm!
///
/// This test server will always provide snowflake ids as 123456789101112131
/// and auth tokens as "faketoken"
#[allow(dead_code)]
#[cfg(not(target_arch = "wasm32"))]
pub(crate) async fn setup_with_mock_server(server: &httptest::Server) -> TestBundle {
    // So we can get logs when tests fail
    let _ = simple_logger::SimpleLogger::with_level(
        simple_logger::SimpleLogger::new(),
        log::LevelFilter::Debug,
    )
    .init();

    let instance = Instance::new(server.url_str("/api").as_str(), None)
        .await
        .unwrap();

    // Requires the existence of the below user.
    let reg = RegisterSchema {
        username: "integrationtestuser".into(),
        consent: true,
        date_of_birth: Some(NaiveDate::from_str("2000-01-01").unwrap()),
        ..Default::default()
    };
    let user = instance.clone().register_account(reg).await.unwrap();

    let guild = Guild {
        id: Snowflake(123456789101112131),
        name: Some("Test-Guild!".to_string()),
        ..Default::default()
    };

    let channel = Channel {
        id: Snowflake(123456789101112131),
        name: Some("testchannel".to_string()),
        channel_type: chorus::types::ChannelType::GuildText,
        nsfw: Some(false),
        flags: Some(0),
        default_thread_rate_limit_per_user: Some(0),
        ..Default::default()
    };

    let role = chorus::types::RoleObject {
        id: Snowflake(123456789101112131),
        name: "Bundle role".to_string(),
        permissions: PermissionFlags::from_bits(8).unwrap(),
        hoist: true,
        unicode_emoji: Some(String::new()),
        mentionable: true,
        ..Default::default()
    };

    let urls = instance.urls.clone();

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
    bundle
        .user
        .delete(DeleteDisableUserSchema { password: None })
        .await
        .unwrap()
}

/// Creates a mock http server at localhost:3001 with the basic routes
/// needed to run TestBundle setup and teardown
///
/// Note: httptest does not work on wasm!
///
/// This test server will always provide snowflake ids as 123456789101112131
/// and auth tokens as "faketoken"
#[allow(dead_code)]
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn create_mock_server() -> httptest::Server {
    let server = httptest::Server::run();

    let api_url = server.url("/api");
    let cdn_url = server.url("/cdn");

    // Just redirect it to the one we're running for spacebar tests
    // We're using this just for the api anyway, so it can break after identifying
    let gateway_url = "ws://localhost:3001";

    // Mock the instance/domains endpoint, so we can create from a single url
    server.expect(
        Expectation::matching(all_of![
            request::method("GET"),
            request::path("/api/policies/instance/domains")
        ])
        .respond_with(json_encoded(
            chorus::types::types::domains_configuration::Domains {
                api_endpoint: api_url.to_string(),
                cdn: cdn_url.to_string(),
                gateway: gateway_url.to_string(),
                default_api_version: "v9".to_string(),
            },
        )),
    );

    // The following routes are mocked so that login and register work:
    server.expect(
        Expectation::matching(all_of![
            request::method("POST"),
            request::path("/api/auth/register")
        ])
        .respond_with(json_encoded(chorus::instance::Token {
            token: "faketoken".to_string(),
        })),
    );

    server.expect(
        Expectation::matching(all_of![
            request::method("POST"),
            request::path("/api/auth/login")
        ])
        .respond_with(json_encoded(chorus::types::LoginResult {
            token: "faketoken".to_string(),
            settings: chorus::types::UserSettings {
                ..Default::default()
            }
            .into_shared(),
        })),
    );

    server.expect(
        Expectation::matching(all_of![
            request::method("GET"),
            request::path("/api/users/@me"),
            request::headers(contains(("Authorization", "faketoken")))
        ])
        .respond_with(json_encoded(chorus::types::User {
            id: chorus::types::Snowflake(123456789101112131),
            username: "integrationtestuser".to_string(),
            discriminator: "1234".to_string(),
            mfa_enabled: Some(true),
            locale: Some(String::from("en-us")),
            disabled: Some(false),
            ..Default::default()
        })),
    );

    server.expect(
        Expectation::matching(all_of![
            request::method("GET"),
            request::path("/api/users/@me/settings"),
            request::headers(contains(("Authorization", "faketoken")))
        ])
        .respond_with(json_encoded(chorus::types::UserSettings {
            status: chorus::types::UserStatus::Online.into_shared(),
            ..Default::default()
        })),
    );

    // The folowing routes are mocked so that teardown works:
    server.expect(
        Expectation::matching(all_of![
            request::method("POST"),
            // Can we have wildcards here?
            request::path("/api/guilds/123456789101112131/delete"),
            request::headers(contains(("Authorization", "faketoken")))
        ])
        .respond_with(status_code(200)),
    );

    server.expect(
        Expectation::matching(all_of![
            request::method("POST"),
            request::path("/api/users/@me/delete"),
            request::headers(contains(("Authorization", "faketoken")))
        ])
        .respond_with(status_code(200)),
    );

    server
}
