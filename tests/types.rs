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
        use std::str::FromStr;

        use chorus::types::types::guild_configuration::{GuildFeatures, GuildFeaturesList};
        use chorus::types::{Error, GuildError};

        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
        #[cfg_attr(not(target_arch = "wasm32"), test)]
        fn deref_guild_features_list() {
            let guild_features_list = &GuildFeaturesList::default();
            let _guild_features_list_deref = guild_features_list.deref().clone();
        }

        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
        #[cfg_attr(not(target_arch = "wasm32"), test)]
        fn test_deref_mut() {
            let mut guild_features_list = GuildFeaturesList::default();
            guild_features_list.clear();
            let mut list = GuildFeaturesList::default().to_vec();
            list.push(GuildFeatures::ActivitiesAlpha);
            *guild_features_list = list.to_vec();
            assert_eq!(guild_features_list.len(), 1);
        }

        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
        #[cfg_attr(not(target_arch = "wasm32"), test)]
        fn test_display() {
            let mut guild_features_list = GuildFeaturesList::default();
            guild_features_list.push(GuildFeatures::ActivitiesAlpha);
            guild_features_list.push(GuildFeatures::AnimatedBanner);
            assert_eq!(
                format!("{}", guild_features_list),
                "ACTIVITIES_ALPHA,ANIMATED_BANNER"
            );
        }

        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
        #[cfg_attr(not(target_arch = "wasm32"), test)]
        fn test_from_str() {
            // GPT moment
            assert_eq!(
                GuildFeatures::from_str("ACTIVITIES_ALPHA").unwrap(),
                GuildFeatures::ActivitiesAlpha
            );
            assert_eq!(
                GuildFeatures::from_str("ACTIVITIES_EMPLOYEE").unwrap(),
                GuildFeatures::ActivitiesEmployee
            );
            assert_eq!(
                GuildFeatures::from_str("ACTIVITIES_INTERNAL_DEV").unwrap(),
                GuildFeatures::ActivitiesInternalDev
            );
            assert_eq!(
                GuildFeatures::from_str("ANIMATED_BANNER").unwrap(),
                GuildFeatures::AnimatedBanner
            );
            assert_eq!(
                GuildFeatures::from_str("ANIMATED_ICON").unwrap(),
                GuildFeatures::AnimatedIcon
            );
            assert_eq!(
                GuildFeatures::from_str("APPLICATION_COMMAND_PERMISSIONS_V2").unwrap(),
                GuildFeatures::ApplicationCommandPermissionsV2
            );
            assert_eq!(
                GuildFeatures::from_str("AUTO_MODERATION").unwrap(),
                GuildFeatures::AutoModeration
            );
            assert_eq!(
                GuildFeatures::from_str("AUTO_MOD_TRIGGER_KEYWORD_FILTER").unwrap(),
                GuildFeatures::AutoModTriggerKeywordFilter
            );
            assert_eq!(
                GuildFeatures::from_str("AUTO_MOD_TRIGGER_ML_SPAM_FILTER").unwrap(),
                GuildFeatures::AutoModTriggerMLSpamFilter
            );
            assert_eq!(
                GuildFeatures::from_str("AUTO_MOD_TRIGGER_SPAM_LINK_FILTER").unwrap(),
                GuildFeatures::AutoModTriggerSpamLinkFilter
            );
            assert_eq!(
                GuildFeatures::from_str("AUTO_MOD_TRIGGER_USER_PROFILE").unwrap(),
                GuildFeatures::AutoModTriggerUserProfile
            );
            assert_eq!(
                GuildFeatures::from_str("BANNER").unwrap(),
                GuildFeatures::Banner
            );
            assert_eq!(GuildFeatures::from_str("BFG").unwrap(), GuildFeatures::Bfg);
            assert_eq!(
                GuildFeatures::from_str("BOOSTING_TIERS_EXPERIMENT_MEDIUM_GUILD").unwrap(),
                GuildFeatures::BoostingTiersExperimentMediumGuild
            );
            assert_eq!(
                GuildFeatures::from_str("BOOSTING_TIERS_EXPERIMENT_SMALL_GUILD").unwrap(),
                GuildFeatures::BoostingTiersExperimentSmallGuild
            );
            assert_eq!(
                GuildFeatures::from_str("BOT_DEVELOPER_EARLY_ACCESS").unwrap(),
                GuildFeatures::BotDeveloperEarlyAccess
            );
            assert_eq!(
                GuildFeatures::from_str("BURST_REACTIONS").unwrap(),
                GuildFeatures::BurstReactions
            );
            assert_eq!(
                GuildFeatures::from_str("COMMUNITY_CANARY").unwrap(),
                GuildFeatures::CommunityCanary
            );
            assert_eq!(
                GuildFeatures::from_str("COMMUNITY_EXP_LARGE_GATED").unwrap(),
                GuildFeatures::CommunityExpLargeGated
            );
            assert_eq!(
                GuildFeatures::from_str("COMMUNITY_EXP_LARGE_UNGATED").unwrap(),
                GuildFeatures::CommunityExpLargeUngated
            );
            assert_eq!(
                GuildFeatures::from_str("COMMUNITY_EXP_MEDIUM").unwrap(),
                GuildFeatures::CommunityExpMedium
            );
            assert_eq!(
                GuildFeatures::from_str("CHANNEL_EMOJIS_GENERATED").unwrap(),
                GuildFeatures::ChannelEmojisGenerated
            );
            assert_eq!(
                GuildFeatures::from_str("CHANNEL_HIGHLIGHTS").unwrap(),
                GuildFeatures::ChannelHighlights
            );
            assert_eq!(
                GuildFeatures::from_str("CHANNEL_HIGHLIGHTS_DISABLED").unwrap(),
                GuildFeatures::ChannelHighlightsDisabled
            );
            assert_eq!(
                GuildFeatures::from_str("CLYDE_ENABLED").unwrap(),
                GuildFeatures::ClydeEnabled
            );
            assert_eq!(
                GuildFeatures::from_str("CLYDE_EXPERIMENT_ENABLED").unwrap(),
                GuildFeatures::ClydeExperimentEnabled
            );
            assert_eq!(
                GuildFeatures::from_str("CLYDE_DISABLED").unwrap(),
                GuildFeatures::ClydeDisabled
            );
            assert_eq!(
                GuildFeatures::from_str("COMMUNITY").unwrap(),
                GuildFeatures::Community
            );
            assert_eq!(
                GuildFeatures::from_str("CREATOR_ACCEPTED_NEW_TERMS").unwrap(),
                GuildFeatures::CreatorAcceptedNewTerms
            );
            assert_eq!(
                GuildFeatures::from_str("CREATOR_MONETIZABLE").unwrap(),
                GuildFeatures::CreatorMonetizable
            );
            assert_eq!(
                GuildFeatures::from_str("CREATOR_MONETIZABLE_DISABLED").unwrap(),
                GuildFeatures::CreatorMonetizableDisabled
            );
            assert_eq!(
                GuildFeatures::from_str("CREATOR_MONETIZABLE_PENDING_NEW_OWNER_ONBOARDING")
                    .unwrap(),
                GuildFeatures::CreatorMonetizablePendingNewOwnerOnboarding
            );
            assert_eq!(
                GuildFeatures::from_str("CREATOR_MONETIZABLE_PROVISIONAL").unwrap(),
                GuildFeatures::CreatorMonetizableProvisional
            );
            assert_eq!(
                GuildFeatures::from_str("CREATOR_MONETIZABLE_RESTRICTED").unwrap(),
                GuildFeatures::CreatorMonetizableRestricted
            );
            assert_eq!(
                GuildFeatures::from_str("CREATOR_MONETIZABLE_WHITEGLOVE").unwrap(),
                GuildFeatures::CreatorMonetizableWhiteglove
            );
            assert_eq!(
                GuildFeatures::from_str("CREATOR_MONETIZABLE_APPLICATION_ALLOWLIST").unwrap(),
                GuildFeatures::CreatorMonetizableApplicationAllowlist
            );
            assert_eq!(
                GuildFeatures::from_str("CREATE_STORE_PAGE").unwrap(),
                GuildFeatures::CreateStorePage
            );
            assert_eq!(
                GuildFeatures::from_str("DEVELOPER_SUPPORT_SERVER").unwrap(),
                GuildFeatures::DeveloperSupportServer
            );
            assert_eq!(
                GuildFeatures::from_str("DISCOVERABLE_DISABLED").unwrap(),
                GuildFeatures::DiscoverableDisabled
            );
            assert_eq!(
                GuildFeatures::from_str("DISCOVERABLE").unwrap(),
                GuildFeatures::Discoverable
            );
            assert_eq!(
                GuildFeatures::from_str("ENABLED_DISCOVERABLE_BEFORE").unwrap(),
                GuildFeatures::EnabledDiscoverableBefore
            );
            assert_eq!(
                GuildFeatures::from_str("EXPOSED_TO_ACTIVITIES_WTP_EXPERIMENT").unwrap(),
                GuildFeatures::ExposedToActivitiesWTPExperiment
            );
            assert_eq!(
                GuildFeatures::from_str("GUESTS_ENABLED").unwrap(),
                GuildFeatures::GuestsEnabled
            );
            assert_eq!(
                GuildFeatures::from_str("GUILD_AUTOMOD_DEFAULT_LIST").unwrap(),
                GuildFeatures::GuildAutomodDefaultList
            );
            assert_eq!(
                GuildFeatures::from_str("GUILD_COMMUNICATION_DISABLED_GUILDS").unwrap(),
                GuildFeatures::GuildCommunicationDisabledGuilds
            );
            assert_eq!(
                GuildFeatures::from_str("GUILD_HOME_DEPRECATION_OVERRIDE").unwrap(),
                GuildFeatures::GuildHomeDeprecationOverride
            );
            assert_eq!(
                GuildFeatures::from_str("GUILD_HOME_OVERRIDE").unwrap(),
                GuildFeatures::GuildHomeOverride
            );
            assert_eq!(
                GuildFeatures::from_str("GUILD_HOME_TEST").unwrap(),
                GuildFeatures::GuildHomeTest
            );
            assert_eq!(
                GuildFeatures::from_str("GUILD_MEMBER_VERIFICATION_EXPERIMENT").unwrap(),
                GuildFeatures::GuildMemberVerificationExperiment
            );
            assert_eq!(
                GuildFeatures::from_str("GUILD_ONBOARDING").unwrap(),
                GuildFeatures::GuildOnboarding
            );
            assert_eq!(
                GuildFeatures::from_str("GUILD_ONBOARDING_ADMIN_ONLY").unwrap(),
                GuildFeatures::GuildOnboardingAdminOnly
            );
            assert_eq!(
                GuildFeatures::from_str("GUILD_ONBOARDING_EVER_ENABLED").unwrap(),
                GuildFeatures::GuildOnboardingEverEnabled
            );
            assert_eq!(
                GuildFeatures::from_str("GUILD_ONBOARDING_HAS_PROMPTS").unwrap(),
                GuildFeatures::GuildOnboardingHasPrompts
            );
            assert_eq!(
                GuildFeatures::from_str("GUILD_ROLE_SUBSCRIPTION").unwrap(),
                GuildFeatures::GuildRoleSubscription
            );
            assert_eq!(
                GuildFeatures::from_str("GUILD_ROLE_SUBSCRIPTION_PURCHASE_FEEDBACK_LOOP").unwrap(),
                GuildFeatures::GuildRoleSubscriptionPurchaseFeedbackLoop
            );
            assert_eq!(
                GuildFeatures::from_str("GUILD_ROLE_SUBSCRIPTION_TRIALS").unwrap(),
                GuildFeatures::GuildRoleSubscriptionTrials
            );
            assert_eq!(
                GuildFeatures::from_str("GUILD_SERVER_GUIDE").unwrap(),
                GuildFeatures::GuildServerGuide
            );
            assert_eq!(
                GuildFeatures::from_str("GUILD_WEB_PAGE_VANITY_URL").unwrap(),
                GuildFeatures::GuildWebPageVanityURL
            );
            assert_eq!(
                GuildFeatures::from_str("HAD_EARLY_ACTIVITIES_ACCESS").unwrap(),
                GuildFeatures::HadEarlyActivitiesAccess
            );
            assert_eq!(
                GuildFeatures::from_str("HAS_DIRECTORY_ENTRY").unwrap(),
                GuildFeatures::HasDirectoryEntry
            );
            assert_eq!(
                GuildFeatures::from_str("HIDE_FROM_EXPERIMENT_UI").unwrap(),
                GuildFeatures::HideFromExperimentUi
            );
            assert_eq!(GuildFeatures::from_str("HUB").unwrap(), GuildFeatures::Hub);
            assert_eq!(
                GuildFeatures::from_str("INCREASED_THREAD_LIMIT").unwrap(),
                GuildFeatures::IncreasedThreadLimit
            );
            assert_eq!(
                GuildFeatures::from_str("INTERNAL_EMPLOYEE_ONLY").unwrap(),
                GuildFeatures::InternalEmployeeOnly
            );
            assert_eq!(
                GuildFeatures::from_str("INVITE_SPLASH").unwrap(),
                GuildFeatures::InviteSplash
            );
            assert_eq!(
                GuildFeatures::from_str("INVITES_DISABLED").unwrap(),
                GuildFeatures::InvitesDisabled
            );
            assert_eq!(
                GuildFeatures::from_str("LINKED_TO_HUB").unwrap(),
                GuildFeatures::LinkedToHub
            );
            assert_eq!(
                GuildFeatures::from_str("MARKETPLACES_CONNECTION_ROLES").unwrap(),
                GuildFeatures::MarketplacesConnectionRoles
            );
            assert_eq!(
                GuildFeatures::from_str("MEMBER_PROFILES").unwrap(),
                GuildFeatures::MemberProfiles
            );
            assert_eq!(
                GuildFeatures::from_str("MEMBER_VERIFICATION_GATE_ENABLED").unwrap(),
                GuildFeatures::MemberVerificationGateEnabled
            );
            assert_eq!(
                GuildFeatures::from_str("MEMBER_VERIFICATION_MANUAL_APPROVAL").unwrap(),
                GuildFeatures::MemberVerificationManualApproval
            );
            assert_eq!(
                GuildFeatures::from_str("MOBILE_WEB_ROLE_SUBSCRIPTION_PURCHASE_PAGE").unwrap(),
                GuildFeatures::MobileWebRoleSubscriptionPurchasePage
            );
            assert_eq!(
                GuildFeatures::from_str("MONETIZATION_ENABLED").unwrap(),
                GuildFeatures::MonetizationEnabled
            );
            assert_eq!(
                GuildFeatures::from_str("MORE_EMOJI").unwrap(),
                GuildFeatures::MoreEmoji
            );
            assert_eq!(
                GuildFeatures::from_str("MORE_STICKERS").unwrap(),
                GuildFeatures::MoreStickers
            );
            assert_eq!(
                GuildFeatures::from_str("NEWS").unwrap(),
                GuildFeatures::News
            );
            assert_eq!(
                GuildFeatures::from_str("NEW_THREAD_PERMISSIONS").unwrap(),
                GuildFeatures::NewThreadPermissions
            );
            assert_eq!(
                GuildFeatures::from_str("PARTNERED").unwrap(),
                GuildFeatures::Partnered
            );
            assert_eq!(
                GuildFeatures::from_str("PREMIUM_TIER_3_OVERRIDE").unwrap(),
                GuildFeatures::PremiumTier3Override
            );
            assert_eq!(
                GuildFeatures::from_str("PREVIEW_ENABLED").unwrap(),
                GuildFeatures::PreviewEnabled
            );
            assert_eq!(
                GuildFeatures::from_str("RAID_ALERTS_DISABLED").unwrap(),
                GuildFeatures::RaidAlertsDisabled
            );
            assert_eq!(
                GuildFeatures::from_str("RELAY_ENABLED").unwrap(),
                GuildFeatures::RelayEnabled
            );
            assert_eq!(
                GuildFeatures::from_str("RESTRICT_SPAM_RISK_GUILD").unwrap(),
                GuildFeatures::RestrictSpamRiskGuild
            );
            assert_eq!(
                GuildFeatures::from_str("ROLE_ICONS").unwrap(),
                GuildFeatures::RoleIcons
            );
            assert_eq!(
                GuildFeatures::from_str("ROLE_SUBSCRIPTIONS_AVAILABLE_FOR_PURCHASE").unwrap(),
                GuildFeatures::RoleSubscriptionsAvailableForPurchase
            );
            assert_eq!(
                GuildFeatures::from_str("ROLE_SUBSCRIPTIONS_ENABLED").unwrap(),
                GuildFeatures::RoleSubscriptionsEnabled
            );
            assert_eq!(
                GuildFeatures::from_str("ROLE_SUBSCRIPTIONS_ENABLED_FOR_PURCHASE").unwrap(),
                GuildFeatures::RoleSubscriptionsEnabledForPurchase
            );
            assert_eq!(
                GuildFeatures::from_str("SHARD").unwrap(),
                GuildFeatures::Shard
            );
            assert_eq!(
                GuildFeatures::from_str("SHARED_CANVAS_FRIENDS_AND_FAMILY_TEST").unwrap(),
                GuildFeatures::SharedCanvasFriendsAndFamilyTest
            );
            assert_eq!(
                GuildFeatures::from_str("SOUNDBOARD").unwrap(),
                GuildFeatures::Soundboard
            );
            assert_eq!(
                GuildFeatures::from_str("SUMMARIES_ENABLED").unwrap(),
                GuildFeatures::SummariesEnabled
            );
            assert_eq!(
                GuildFeatures::from_str("SUMMARIES_ENABLED_GA").unwrap(),
                GuildFeatures::SummariesEnabledGa
            );
            assert_eq!(
                GuildFeatures::from_str("SUMMARIES_DISABLED_BY_USER").unwrap(),
                GuildFeatures::SummariesDisabledByUser
            );
            assert_eq!(
                GuildFeatures::from_str("SUMMARIES_ENABLED_BY_USER").unwrap(),
                GuildFeatures::SummariesEnabledByUser
            );
            assert_eq!(
                GuildFeatures::from_str("TEXT_IN_STAGE_ENABLED").unwrap(),
                GuildFeatures::TextInStageEnabled
            );
            assert_eq!(
                GuildFeatures::from_str("TEXT_IN_VOICE_ENABLED").unwrap(),
                GuildFeatures::TextInVoiceEnabled
            );
            assert_eq!(
                GuildFeatures::from_str("THREADS_ENABLED_TESTING").unwrap(),
                GuildFeatures::ThreadsEnabledTesting
            );
            assert_eq!(
                GuildFeatures::from_str("THREADS_ENABLED").unwrap(),
                GuildFeatures::ThreadsEnabled
            );
            assert_eq!(
                GuildFeatures::from_str("THREAD_DEFAULT_AUTO_ARCHIVE_DURATION").unwrap(),
                GuildFeatures::ThreadDefaultAutoArchiveDuration
            );
            assert_eq!(
                GuildFeatures::from_str("THREADS_ONLY_CHANNEL").unwrap(),
                GuildFeatures::ThreadsOnlyChannel
            );
            assert_eq!(
                GuildFeatures::from_str("TICKETED_EVENTS_ENABLED").unwrap(),
                GuildFeatures::TicketedEventsEnabled
            );
            assert_eq!(
                GuildFeatures::from_str("TICKETING_ENABLED").unwrap(),
                GuildFeatures::TicketingEnabled
            );
            assert_eq!(
                GuildFeatures::from_str("VANITY_URL").unwrap(),
                GuildFeatures::VanityUrl
            );
            assert_eq!(
                GuildFeatures::from_str("VERIFIED").unwrap(),
                GuildFeatures::Verified
            );
            assert_eq!(
                GuildFeatures::from_str("VIP_REGIONS").unwrap(),
                GuildFeatures::VipRegions
            );
            assert_eq!(
                GuildFeatures::from_str("VOICE_CHANNEL_EFFECTS").unwrap(),
                GuildFeatures::VoiceChannelEffects
            );
            assert_eq!(
                GuildFeatures::from_str("WELCOME_SCREEN_ENABLED").unwrap(),
                GuildFeatures::WelcomeScreenEnabled
            );
            assert_eq!(
                GuildFeatures::from_str("ALIASABLE_NAMES").unwrap(),
                GuildFeatures::AliasableNames
            );
            assert_eq!(
                GuildFeatures::from_str("ALLOW_INVALID_CHANNEL_NAME").unwrap(),
                GuildFeatures::AllowInvalidChannelName
            );
            assert_eq!(
                GuildFeatures::from_str("ALLOW_UNNAMED_CHANNELS").unwrap(),
                GuildFeatures::AllowUnnamedChannels
            );
            assert_eq!(
                GuildFeatures::from_str("CROSS_CHANNEL_REPLIES").unwrap(),
                GuildFeatures::CrossChannelReplies
            );
            assert_eq!(
                GuildFeatures::from_str("IRC_LIKE_CATEGORY_NAMES").unwrap(),
                GuildFeatures::IrcLikeCategoryNames
            );
            assert_eq!(
                GuildFeatures::from_str("INVITES_CLOSED").unwrap(),
                GuildFeatures::InvitesClosed
            );
            assert_eq!(
                GuildFeatures::from_str("INVALID").unwrap_err().to_string(),
                Error::Guild(GuildError::InvalidGuildFeature).to_string()
            );
        }

        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
        #[cfg_attr(not(target_arch = "wasm32"), test)]
        fn test_to_str() {
            assert_eq!(GuildFeatures::ActivitiesAlpha.to_str(), "ACTIVITIES_ALPHA");
            assert_eq!(
                GuildFeatures::ActivitiesEmployee.to_str(),
                "ACTIVITIES_EMPLOYEE"
            );
            assert_eq!(
                GuildFeatures::ActivitiesInternalDev.to_str(),
                "ACTIVITIES_INTERNAL_DEV"
            );
            assert_eq!(GuildFeatures::AnimatedBanner.to_str(), "ANIMATED_BANNER");
            assert_eq!(GuildFeatures::AnimatedIcon.to_str(), "ANIMATED_ICON");
            assert_eq!(
                GuildFeatures::ApplicationCommandPermissionsV2.to_str(),
                "APPLICATION_COMMAND_PERMISSIONS_V2"
            );
            assert_eq!(GuildFeatures::AutoModeration.to_str(), "AUTO_MODERATION");
            assert_eq!(
                GuildFeatures::AutoModTriggerKeywordFilter.to_str(),
                "AUTO_MOD_TRIGGER_KEYWORD_FILTER"
            );
            assert_eq!(
                GuildFeatures::AutoModTriggerMLSpamFilter.to_str(),
                "AUTO_MOD_TRIGGER_ML_SPAM_FILTER"
            );
            assert_eq!(
                GuildFeatures::AutoModTriggerSpamLinkFilter.to_str(),
                "AUTO_MOD_TRIGGER_SPAM_LINK_FILTER"
            );
            assert_eq!(
                GuildFeatures::AutoModTriggerUserProfile.to_str(),
                "AUTO_MOD_TRIGGER_USER_PROFILE"
            );
            assert_eq!(GuildFeatures::Banner.to_str(), "BANNER");
            assert_eq!(GuildFeatures::Bfg.to_str(), "BFG");
            assert_eq!(
                GuildFeatures::BoostingTiersExperimentMediumGuild.to_str(),
                "BOOSTING_TIERS_EXPERIMENT_MEDIUM_GUILD"
            );
            assert_eq!(
                GuildFeatures::BoostingTiersExperimentSmallGuild.to_str(),
                "BOOSTING_TIERS_EXPERIMENT_SMALL_GUILD"
            );
            assert_eq!(
                GuildFeatures::BotDeveloperEarlyAccess.to_str(),
                "BOT_DEVELOPER_EARLY_ACCESS"
            );
            assert_eq!(GuildFeatures::BurstReactions.to_str(), "BURST_REACTIONS");
            assert_eq!(GuildFeatures::CommunityCanary.to_str(), "COMMUNITY_CANARY");
            assert_eq!(
                GuildFeatures::CommunityExpLargeGated.to_str(),
                "COMMUNITY_EXP_LARGE_GATED"
            );
            assert_eq!(
                GuildFeatures::CommunityExpLargeUngated.to_str(),
                "COMMUNITY_EXP_LARGE_UNGATED"
            );
            assert_eq!(
                GuildFeatures::CommunityExpMedium.to_str(),
                "COMMUNITY_EXP_MEDIUM"
            );
            assert_eq!(
                GuildFeatures::ChannelEmojisGenerated.to_str(),
                "CHANNEL_EMOJIS_GENERATED"
            );
            assert_eq!(
                GuildFeatures::ChannelHighlights.to_str(),
                "CHANNEL_HIGHLIGHTS"
            );
            assert_eq!(
                GuildFeatures::ChannelHighlightsDisabled.to_str(),
                "CHANNEL_HIGHLIGHTS_DISABLED"
            );
            assert_eq!(GuildFeatures::ClydeEnabled.to_str(), "CLYDE_ENABLED");
            assert_eq!(
                GuildFeatures::ClydeExperimentEnabled.to_str(),
                "CLYDE_EXPERIMENT_ENABLED"
            );
            assert_eq!(GuildFeatures::ClydeDisabled.to_str(), "CLYDE_DISABLED");
            assert_eq!(GuildFeatures::Community.to_str(), "COMMUNITY");
            assert_eq!(
                GuildFeatures::CreatorAcceptedNewTerms.to_str(),
                "CREATOR_ACCEPTED_NEW_TERMS"
            );
            assert_eq!(
                GuildFeatures::CreatorMonetizable.to_str(),
                "CREATOR_MONETIZABLE"
            );
            assert_eq!(
                GuildFeatures::CreatorMonetizableDisabled.to_str(),
                "CREATOR_MONETIZABLE_DISABLED"
            );
            assert_eq!(
                GuildFeatures::CreatorMonetizablePendingNewOwnerOnboarding.to_str(),
                "CREATOR_MONETIZABLE_PENDING_NEW_OWNER_ONBOARDING"
            );
            assert_eq!(
                GuildFeatures::CreatorMonetizableProvisional.to_str(),
                "CREATOR_MONETIZABLE_PROVISIONAL"
            );
            assert_eq!(
                GuildFeatures::CreatorMonetizableRestricted.to_str(),
                "CREATOR_MONETIZABLE_RESTRICTED"
            );
            assert_eq!(
                GuildFeatures::CreatorMonetizableWhiteglove.to_str(),
                "CREATOR_MONETIZABLE_WHITEGLOVE"
            );
            assert_eq!(
                GuildFeatures::CreatorMonetizableApplicationAllowlist.to_str(),
                "CREATOR_MONETIZABLE_APPLICATION_ALLOWLIST"
            );
            assert_eq!(GuildFeatures::CreateStorePage.to_str(), "CREATE_STORE_PAGE");
            assert_eq!(
                GuildFeatures::DeveloperSupportServer.to_str(),
                "DEVELOPER_SUPPORT_SERVER"
            );
            assert_eq!(
                GuildFeatures::DiscoverableDisabled.to_str(),
                "DISCOVERABLE_DISABLED"
            );
            assert_eq!(GuildFeatures::Discoverable.to_str(), "DISCOVERABLE");
            assert_eq!(
                GuildFeatures::EnabledDiscoverableBefore.to_str(),
                "ENABLED_DISCOVERABLE_BEFORE"
            );
            assert_eq!(
                GuildFeatures::ExposedToActivitiesWTPExperiment.to_str(),
                "EXPOSED_TO_ACTIVITIES_WTP_EXPERIMENT"
            );
            assert_eq!(GuildFeatures::GuestsEnabled.to_str(), "GUESTS_ENABLED");
            assert_eq!(
                GuildFeatures::GuildAutomodDefaultList.to_str(),
                "GUILD_AUTOMOD_DEFAULT_LIST"
            );
            assert_eq!(
                GuildFeatures::GuildCommunicationDisabledGuilds.to_str(),
                "GUILD_COMMUNICATION_DISABLED_GUILDS"
            );
            assert_eq!(
                GuildFeatures::GuildHomeDeprecationOverride.to_str(),
                "GUILD_HOME_DEPRECATION_OVERRIDE"
            );
            assert_eq!(
                GuildFeatures::GuildHomeOverride.to_str(),
                "GUILD_HOME_OVERRIDE"
            );
            assert_eq!(GuildFeatures::GuildHomeTest.to_str(), "GUILD_HOME_TEST");
            assert_eq!(
                GuildFeatures::GuildMemberVerificationExperiment.to_str(),
                "GUILD_MEMBER_VERIFICATION_EXPERIMENT"
            );
            assert_eq!(GuildFeatures::GuildOnboarding.to_str(), "GUILD_ONBOARDING");
            assert_eq!(
                GuildFeatures::GuildOnboardingAdminOnly.to_str(),
                "GUILD_ONBOARDING_ADMIN_ONLY"
            );
            assert_eq!(
                GuildFeatures::GuildOnboardingEverEnabled.to_str(),
                "GUILD_ONBOARDING_EVER_ENABLED"
            );
            assert_eq!(
                GuildFeatures::GuildOnboardingHasPrompts.to_str(),
                "GUILD_ONBOARDING_HAS_PROMPTS"
            );
            assert_eq!(
                GuildFeatures::GuildRoleSubscription.to_str(),
                "GUILD_ROLE_SUBSCRIPTION"
            );
            assert_eq!(
                GuildFeatures::GuildRoleSubscriptionPurchaseFeedbackLoop.to_str(),
                "GUILD_ROLE_SUBSCRIPTION_PURCHASE_FEEDBACK_LOOP"
            );
            assert_eq!(
                GuildFeatures::GuildRoleSubscriptionTrials.to_str(),
                "GUILD_ROLE_SUBSCRIPTION_TRIALS"
            );
            assert_eq!(
                GuildFeatures::GuildServerGuide.to_str(),
                "GUILD_SERVER_GUIDE"
            );
            assert_eq!(
                GuildFeatures::GuildWebPageVanityURL.to_str(),
                "GUILD_WEB_PAGE_VANITY_URL"
            );
            assert_eq!(
                GuildFeatures::HadEarlyActivitiesAccess.to_str(),
                "HAD_EARLY_ACTIVITIES_ACCESS"
            );
            assert_eq!(
                GuildFeatures::HasDirectoryEntry.to_str(),
                "HAS_DIRECTORY_ENTRY"
            );
            assert_eq!(
                GuildFeatures::HideFromExperimentUi.to_str(),
                "HIDE_FROM_EXPERIMENT_UI"
            );
            assert_eq!(GuildFeatures::Hub.to_str(), "HUB");
            assert_eq!(
                GuildFeatures::IncreasedThreadLimit.to_str(),
                "INCREASED_THREAD_LIMIT"
            );
            assert_eq!(
                GuildFeatures::InternalEmployeeOnly.to_str(),
                "INTERNAL_EMPLOYEE_ONLY"
            );
            assert_eq!(GuildFeatures::InviteSplash.to_str(), "INVITE_SPLASH");
            assert_eq!(GuildFeatures::InvitesDisabled.to_str(), "INVITES_DISABLED");
            assert_eq!(GuildFeatures::LinkedToHub.to_str(), "LINKED_TO_HUB");
            assert_eq!(
                GuildFeatures::MarketplacesConnectionRoles.to_str(),
                "MARKETPLACES_CONNECTION_ROLES"
            );
            assert_eq!(GuildFeatures::MemberProfiles.to_str(), "MEMBER_PROFILES");
            assert_eq!(
                GuildFeatures::MemberVerificationGateEnabled.to_str(),
                "MEMBER_VERIFICATION_GATE_ENABLED"
            );
            assert_eq!(
                GuildFeatures::MemberVerificationManualApproval.to_str(),
                "MEMBER_VERIFICATION_MANUAL_APPROVAL"
            );
            assert_eq!(
                GuildFeatures::MobileWebRoleSubscriptionPurchasePage.to_str(),
                "MOBILE_WEB_ROLE_SUBSCRIPTION_PURCHASE_PAGE"
            );
            assert_eq!(
                GuildFeatures::MonetizationEnabled.to_str(),
                "MONETIZATION_ENABLED"
            );
            assert_eq!(GuildFeatures::MoreEmoji.to_str(), "MORE_EMOJI");
            assert_eq!(GuildFeatures::MoreStickers.to_str(), "MORE_STICKERS");
            assert_eq!(GuildFeatures::News.to_str(), "NEWS");
            assert_eq!(
                GuildFeatures::NewThreadPermissions.to_str(),
                "NEW_THREAD_PERMISSIONS"
            );
            assert_eq!(GuildFeatures::Partnered.to_str(), "PARTNERED");
            assert_eq!(
                GuildFeatures::PremiumTier3Override.to_str(),
                "PREMIUM_TIER_3_OVERRIDE"
            );
            assert_eq!(GuildFeatures::PreviewEnabled.to_str(), "PREVIEW_ENABLED");
            assert_eq!(
                GuildFeatures::RaidAlertsDisabled.to_str(),
                "RAID_ALERTS_DISABLED"
            );
            assert_eq!(GuildFeatures::RelayEnabled.to_str(), "RELAY_ENABLED");
            assert_eq!(
                GuildFeatures::RestrictSpamRiskGuild.to_str(),
                "RESTRICT_SPAM_RISK_GUILD"
            );
            assert_eq!(GuildFeatures::RoleIcons.to_str(), "ROLE_ICONS");
            assert_eq!(
                GuildFeatures::RoleSubscriptionsAvailableForPurchase.to_str(),
                "ROLE_SUBSCRIPTIONS_AVAILABLE_FOR_PURCHASE"
            );
            assert_eq!(
                GuildFeatures::RoleSubscriptionsEnabled.to_str(),
                "ROLE_SUBSCRIPTIONS_ENABLED"
            );
            assert_eq!(
                GuildFeatures::RoleSubscriptionsEnabledForPurchase.to_str(),
                "ROLE_SUBSCRIPTIONS_ENABLED_FOR_PURCHASE"
            );
            assert_eq!(GuildFeatures::Shard.to_str(), "SHARD");
            assert_eq!(
                GuildFeatures::SharedCanvasFriendsAndFamilyTest.to_str(),
                "SHARED_CANVAS_FRIENDS_AND_FAMILY_TEST"
            );
            assert_eq!(GuildFeatures::Soundboard.to_str(), "SOUNDBOARD");
            assert_eq!(
                GuildFeatures::SummariesEnabled.to_str(),
                "SUMMARIES_ENABLED"
            );
            assert_eq!(
                GuildFeatures::SummariesEnabledGa.to_str(),
                "SUMMARIES_ENABLED_GA"
            );
            assert_eq!(
                GuildFeatures::SummariesDisabledByUser.to_str(),
                "SUMMARIES_DISABLED_BY_USER"
            );
            assert_eq!(
                GuildFeatures::SummariesEnabledByUser.to_str(),
                "SUMMARIES_ENABLED_BY_USER"
            );
            assert_eq!(
                GuildFeatures::TextInStageEnabled.to_str(),
                "TEXT_IN_STAGE_ENABLED"
            );
            assert_eq!(
                GuildFeatures::TextInVoiceEnabled.to_str(),
                "TEXT_IN_VOICE_ENABLED"
            );
            assert_eq!(
                GuildFeatures::ThreadsEnabledTesting.to_str(),
                "THREADS_ENABLED_TESTING"
            );
            assert_eq!(GuildFeatures::ThreadsEnabled.to_str(), "THREADS_ENABLED");
            assert_eq!(
                GuildFeatures::ThreadDefaultAutoArchiveDuration.to_str(),
                "THREAD_DEFAULT_AUTO_ARCHIVE_DURATION"
            );
            assert_eq!(
                GuildFeatures::ThreadsOnlyChannel.to_str(),
                "THREADS_ONLY_CHANNEL"
            );
            assert_eq!(
                GuildFeatures::TicketedEventsEnabled.to_str(),
                "TICKETED_EVENTS_ENABLED"
            );
            assert_eq!(
                GuildFeatures::TicketingEnabled.to_str(),
                "TICKETING_ENABLED"
            );
            assert_eq!(GuildFeatures::VanityUrl.to_str(), "VANITY_URL");
            assert_eq!(GuildFeatures::Verified.to_str(), "VERIFIED");
            assert_eq!(GuildFeatures::VipRegions.to_str(), "VIP_REGIONS");
            assert_eq!(
                GuildFeatures::VoiceChannelEffects.to_str(),
                "VOICE_CHANNEL_EFFECTS"
            );
            assert_eq!(
                GuildFeatures::WelcomeScreenEnabled.to_str(),
                "WELCOME_SCREEN_ENABLED"
            );
            assert_eq!(GuildFeatures::AliasableNames.to_str(), "ALIASABLE_NAMES");
            assert_eq!(
                GuildFeatures::AllowInvalidChannelName.to_str(),
                "ALLOW_INVALID_CHANNEL_NAME"
            );
            assert_eq!(
                GuildFeatures::AllowUnnamedChannels.to_str(),
                "ALLOW_UNNAMED_CHANNELS"
            );
            assert_eq!(
                GuildFeatures::CrossChannelReplies.to_str(),
                "CROSS_CHANNEL_REPLIES"
            );
            assert_eq!(
                GuildFeatures::IrcLikeCategoryNames.to_str(),
                "IRC_LIKE_CATEGORY_NAMES"
            );
            assert_eq!(GuildFeatures::InvitesClosed.to_str(), "INVITES_CLOSED");
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

    mod guild {
        use std::hash::{Hash, Hasher};

        use chorus::types::{Guild, GuildInvite};

        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
        #[cfg_attr(not(target_arch = "wasm32"), test)]
        fn guild_hash() {
            let id: u64 = 1;
            let mut guild1 = Guild::default();
            let mut guild2 = Guild::default();
            guild1.id = id.into();
            guild2.id = id.into();
            let mut hasher1 = std::collections::hash_map::DefaultHasher::new();
            guild1.hash(&mut hasher1);

            let mut hasher2 = std::collections::hash_map::DefaultHasher::new();
            guild2.hash(&mut hasher2);

            assert_eq!(hasher1.finish(), hasher2.finish());
        }

        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
        #[cfg_attr(not(target_arch = "wasm32"), test)]
        fn guild_invite_hash() {
            let id: u64 = 1;
            let mut invite1 = GuildInvite::default();
            let mut invite2 = GuildInvite::default();
            invite1.channel_id = id.into();
            invite2.channel_id = id.into();
            invite1.guild_id = id.into();
            invite2.guild_id = id.into();
            let mut hasher1 = std::collections::hash_map::DefaultHasher::new();
            invite1.hash(&mut hasher1);

            let mut hasher2 = std::collections::hash_map::DefaultHasher::new();
            invite2.hash(&mut hasher2);

            assert_eq!(hasher1.finish(), hasher2.finish());
        }

        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
        #[cfg_attr(not(target_arch = "wasm32"), test)]
        fn guild_partial_eq() {
            let id: u64 = 1;
            let mut guild1 = Guild::default();
            let mut guild2 = Guild::default();
            guild1.id = id.into();
            guild2.id = id.into();

            assert_eq!(guild1, guild2);
        }
    }

    mod relationship {
        use chorus::types::{IntoShared, Relationship, User};

        #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
        #[cfg_attr(not(target_arch = "wasm32"), test)]
        fn relationship_partial_eq() {
            let user = User::default();
            // These 2 users are different, because they do not have the same Snowflake "id".
            let user_2 = User::default();
            let relationship_1 = Relationship {
                id: 32_u64.into(),
                relationship_type: chorus::types::RelationshipType::Friends,
                nickname: Some("Xenia".to_string()),
                user: user.into_public_user().into_shared(),
                since: None,
            };

            let relationship_2 = Relationship {
                id: 32_u64.into(),
                relationship_type: chorus::types::RelationshipType::Friends,
                nickname: Some("Xenia".to_string()),
                user: user_2.into_public_user().into_shared(),
                since: None,
            };

            // This should succeed, even though the two users' IDs are different. This is because
            // `User` is only `PartialEq`, and the actual user object is not checked, since the
            // `RwLock` would have to be locked.
            assert_eq!(relationship_1, relationship_2);
        }
    }
}
