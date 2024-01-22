mod config {
    mod subconfigs {
        mod client {
            use chorus::types::types::subconfigs::client::ClientReleaseConfiguration;

            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
            #[cfg_attr(not(target_arch = "wasm32"), test)]
            fn client_release_configuration() {
                let _client_release_configuration = ClientReleaseConfiguration::default();
            }
        }

        mod limits {
            use chorus::types::types::subconfigs::limits::rates::RateLimits;

            #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
            #[cfg_attr(not(target_arch = "wasm32"), test)]
            fn rates() {
                let rate_limits = RateLimits::default();
                let hash_map = rate_limits.to_hash_map();
                assert!(hash_map.contains_key(&chorus::types::LimitType::ChannelBaseline));
                assert!(hash_map.contains_key(&chorus::types::LimitType::GuildBaseline));
                assert!(hash_map.contains_key(&chorus::types::LimitType::AuthLogin));
                assert!(hash_map.contains_key(&chorus::types::LimitType::AuthRegister));
                assert!(hash_map.contains_key(&chorus::types::LimitType::Error));
                assert!(hash_map.contains_key(&chorus::types::LimitType::Global));
                assert!(hash_map.contains_key(&chorus::types::LimitType::Ip));
                assert!(hash_map.contains_key(&chorus::types::LimitType::WebhookBaseline));
                assert!(hash_map.len() == 8)
            }
        }
    }

    mod guild_configuration {
        use std::ops::Deref;

        use chorus::types::types::guild_configuration::GuildFeaturesList;

        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
        #[cfg_attr(not(target_arch = "wasm32"), test)]
        fn deref_guild_features_list() {
            let guild_features_list = &GuildFeaturesList::default();
            let _guild_features_list_deref = guild_features_list.deref().clone();
        }
    }

    mod domains_configuration {
        use chorus::types::types::domains_configuration::Domains;

        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
        #[cfg_attr(not(target_arch = "wasm32"), test)]
        fn display_domains() {
            let domains = Domains {
                cdn: "http://localhost:3020/cdn/".to_string(),
                gateway: "ws://localhost:3020/".to_string(),
                api_endpoint: "http://localhost:3020/".to_string(),
                default_api_version: "9".to_string(),
            };
            let fmt_domains = domains.to_string();
            assert!(fmt_domains.contains("CDN URL: http://localhost:3020/cdn/"));
            assert!(fmt_domains.contains("Gateway URL: ws://localhost:3020/"));
            assert!(fmt_domains.contains("API Endpoint: http://localhost:3020/"));
            assert!(fmt_domains.contains("Default API Version: 9"));
        }
    }
}

mod entities {
    use std::sync::{Arc, RwLock};

    use chorus::types::{ApplicationFlags, ConfigEntity, Emoji, User};
    use serde_json::json;

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    #[cfg_attr(not(target_arch = "wasm32"), test)]
    fn application() {
        let application = chorus::types::Application::default();
        assert!(application.name == *"");
        assert!(application.verify_key == *"");
        assert_ne!(
            application.owner.read().unwrap().clone(),
            Arc::new(RwLock::new(User::default()))
                .read()
                .unwrap()
                .clone()
        );
        let flags = ApplicationFlags::from_bits(0).unwrap();
        assert!(application.flags() == flags);
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    #[cfg_attr(not(target_arch = "wasm32"), test)]
    fn config() {
        let mut config_entity = ConfigEntity {
            key: "ExampleKey".to_string(),
            value: Some(json!(1)),
        };
        config_entity.as_int().unwrap();
        assert!(config_entity.as_bool().is_none());
        assert!(config_entity.as_string().is_none());
        config_entity.value = Some(json!(true));
        config_entity.as_bool().unwrap();
        assert!(config_entity.as_int().is_none());
        assert!(config_entity.as_string().is_none());
        config_entity.value = Some(json!("Hello"));
        config_entity.as_string().unwrap();
        assert!(config_entity.as_bool().is_none());
        assert!(config_entity.as_int().is_none());
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    #[cfg_attr(not(target_arch = "wasm32"), test)]
    fn emoji() {
        let emoji = Emoji::default();
        let another_emoji = Emoji::default();
        assert_ne!(emoji.id, another_emoji.id);
        assert_ne!(emoji, another_emoji);
    }
}
