use serde::{Deserialize, Serialize};
#[cfg(feature = "sqlx")]
use sqlx::{
    database::{HasArguments, HasValueRef},
    encode::IsNull,
    error::BoxDynError,
    Decode, Encode, MySql,
};
use std::fmt::{Display, Formatter};
use std::io::Write;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use crate::types::config::types::subconfigs::guild::{
    autojoin::AutoJoinConfiguration, discovery::DiscoverConfiguration,
};
use crate::types::{Error, GuildError};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GuildFeatures {
    ActivitiesAlpha,
    ActivitiesEmployee,
    ActivitiesInternalDev,
    AnimatedBanner,
    AnimatedIcon,
    ApplicationCommandPermissionsV2,
    AutoModeration,
    AutoModTriggerKeywordFilter,
    AutoModTriggerMLSpamFilter,
    AutoModTriggerSpamLinkFilter,
    AutoModTriggerUserProfile,
    Banner,
    Bfg,
    BoostingTiersExperimentMediumGuild,
    BoostingTiersExperimentSmallGuild,
    BotDeveloperEarlyAccess,
    BurstReactions,
    CommunityCanary,
    CommunityExpLargeGated,
    CommunityExpLargeUngated,
    CommunityExpMedium,
    ChannelEmojisGenerated,
    ChannelHighlights,
    ChannelHighlightsDisabled,
    ClydeEnabled,
    ClydeExperimentEnabled,
    ClydeDisabled,
    Community,
    CreatorAcceptedNewTerms,
    CreatorMonetizable,
    CreatorMonetizableDisabled,
    CreatorMonetizablePendingNewOwnerOnboarding,
    CreatorMonetizableProvisional,
    CreatorMonetizableRestricted,
    CreatorMonetizableWhiteglove,
    CreatorMonetizableApplicationAllowlist,
    CreateStorePage,
    DeveloperSupportServer,
    DiscoverableDisabled,
    Discoverable,
    EnabledDiscoverableBefore,
    ExposedToActivitiesWTPExperiment,
    GuestsEnabled,
    GuildAutomodDefaultList,
    GuildCommunicationDisabledGuilds,
    GuildHomeDeprecationOverride,
    GuildHomeOverride,
    GuildHomeTest,
    GuildMemberVerificationExperiment,
    GuildOnboarding,
    GuildOnboardingAdminOnly,
    GuildOnboardingEverEnabled,
    GuildOnboardingHasPrompts,
    GuildRoleSubscription,
    GuildRoleSubscriptionPurchaseFeedbackLoop,
    GuildRoleSubscriptionTrials,
    GuildServerGuide,
    GuildWebPageVanityURL,
    HadEarlyActivitiesAccess,
    HasDirectoryEntry,
    HideFromExperimentUi,
    Hub,
    IncreasedThreadLimit,
    InternalEmployeeOnly,
    InviteSplash,
    InvitesDisabled,
    LinkedToHub,
    MarketplacesConnectionRoles,
    MemberProfiles,
    MemberVerificationGateEnabled,
    MemberVerificationManualApproval,
    MobileWebRoleSubscriptionPurchasePage,
    MonetizationEnabled,
    MoreEmoji,
    MoreStickers,
    News,
    NewThreadPermissions,
    Partnered,
    #[serde(rename = "PREMIUM_TIER_3_OVERRIDE")]
    PremiumTier3Override,
    PreviewEnabled,
    RaidAlertsDisabled,
    RelayEnabled,
    RestrictSpamRiskGuild,
    RoleIcons,
    RoleSubscriptionsAvailableForPurchase,
    RoleSubscriptionsEnabled,
    RoleSubscriptionsEnabledForPurchase,
    Shard,
    SharedCanvasFriendsAndFamilyTest,
    Soundboard,
    SummariesEnabled,
    SummariesEnabledGa,
    SummariesDisabledByUser,
    SummariesEnabledByUser,
    TextInStageEnabled,
    TextInVoiceEnabled,
    ThreadsEnabledTesting,
    ThreadsEnabled,
    ThreadDefaultAutoArchiveDuration,
    ThreadsOnlyChannel,
    TicketedEventsEnabled,
    TicketingEnabled,
    VanityUrl,
    Verified,
    VipRegions,
    VoiceChannelEffects,
    WelcomeScreenEnabled,

    /// Spacebar Specific
    AliasableNames,
    AllowInvalidChannelName,
    AllowUnnamedChannels,
    CrossChannelReplies,
    IrcLikeCategoryNames,
    InvitesClosed,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GuildFeaturesList(Vec<GuildFeatures>);

impl Deref for GuildFeaturesList {
    type Target = Vec<GuildFeatures>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GuildFeaturesList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for GuildFeaturesList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let features = self
            .iter()
            .map(|x| serde_json::to_string(x).unwrap().replace('"', ""))
            .collect::<Vec<_>>()
            .join(",");
        write!(f, "{features}")
    }
}

#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::MySql> for GuildFeaturesList {
    fn decode(value: <MySql as HasValueRef<'r>>::ValueRef) -> Result<Self, BoxDynError> {
        let v = <&str as Decode<sqlx::MySql>>::decode(value)?;
        Ok(Self(
            v.split(',')
                .filter(|f| !f.is_empty())
                .flat_map(GuildFeatures::from_str)
                .collect(),
        ))
    }
}

#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::MySql> for GuildFeaturesList {
    fn encode_by_ref(&self, buf: &mut <MySql as HasArguments<'q>>::ArgumentBuffer) -> IsNull {
        if self.is_empty() {
            return IsNull::Yes;
        }
        let features = self
            .iter()
            .map(|x| x.to_str())
            .collect::<Vec<_>>()
            .join(",");

        let _ = buf.write(features.as_bytes());
        IsNull::No
    }
}

#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::MySql> for GuildFeaturesList {
    fn type_info() -> sqlx::mysql::MySqlTypeInfo {
        <&str as sqlx::Type<sqlx::MySql>>::type_info()
    }

    fn compatible(ty: &sqlx::mysql::MySqlTypeInfo) -> bool {
        <&str as sqlx::Type<sqlx::MySql>>::compatible(ty)
    }
}

#[cfg(feature = "sqlx")]
impl sqlx::TypeInfo for GuildFeaturesList {
    fn is_null(&self) -> bool {
        false
    }

    fn name(&self) -> &str {
        "TEXT"
    }
}

impl FromStr for GuildFeatures {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_ascii_uppercase();
        match s.as_str() {
            "ACTIVITIES_ALPHA" => Ok(GuildFeatures::ActivitiesAlpha),
            "ACTIVITIES_EMPLOYEE" => Ok(GuildFeatures::ActivitiesEmployee),
            "ACTIVITIES_INTERNAL_DEV" => Ok(GuildFeatures::ActivitiesInternalDev),
            "ANIMATED_BANNER" => Ok(GuildFeatures::AnimatedBanner),
            "ANIMATED_ICON" => Ok(GuildFeatures::AnimatedIcon),
            "APPLICATION_COMMAND_PERMISSIONS_V2" => {
                Ok(GuildFeatures::ApplicationCommandPermissionsV2)
            }
            "AUTO_MODERATION" => Ok(GuildFeatures::AutoModeration),
            "AUTO_MOD_TRIGGER_KEYWORD_FILTER" => Ok(GuildFeatures::AutoModTriggerKeywordFilter),
            "AUTO_MOD_TRIGGER_ML_SPAM_FILTER" => Ok(GuildFeatures::AutoModTriggerMLSpamFilter),
            "AUTO_MOD_TRIGGER_SPAM_LINK_FILTER" => Ok(GuildFeatures::AutoModTriggerSpamLinkFilter),
            "AUTO_MOD_TRIGGER_USER_PROFILE" => Ok(GuildFeatures::AutoModTriggerUserProfile),
            "BANNER" => Ok(GuildFeatures::Banner),
            "BFG" => Ok(GuildFeatures::Bfg),
            "BOOSTING_TIERS_EXPERIMENT_MEDIUM_GUILD" => {
                Ok(GuildFeatures::BoostingTiersExperimentMediumGuild)
            }
            "BOOSTING_TIERS_EXPERIMENT_SMALL_GUILD" => {
                Ok(GuildFeatures::BoostingTiersExperimentSmallGuild)
            }
            "BOT_DEVELOPER_EARLY_ACCESS" => Ok(GuildFeatures::BotDeveloperEarlyAccess),
            "BURST_REACTIONS" => Ok(GuildFeatures::BurstReactions),
            "COMMUNITY_CANARY" => Ok(GuildFeatures::CommunityCanary),
            "COMMUNITY_EXP_LARGE_GATED" => Ok(GuildFeatures::CommunityExpLargeGated),
            "COMMUNITY_EXP_LARGE_UNGATED" => Ok(GuildFeatures::CommunityExpLargeUngated),
            "COMMUNITY_EXP_MEDIUM" => Ok(GuildFeatures::CommunityExpMedium),
            "CHANNEL_EMOJIS_GENERATED" => Ok(GuildFeatures::ChannelEmojisGenerated),
            "CHANNEL_HIGHLIGHTS" => Ok(GuildFeatures::ChannelHighlights),
            "CHANNEL_HIGHLIGHTS_DISABLED" => Ok(GuildFeatures::ChannelHighlightsDisabled),
            "CLYDE_ENABLED" => Ok(GuildFeatures::ClydeEnabled),
            "CLYDE_EXPERIMENT_ENABLED" => Ok(GuildFeatures::ClydeExperimentEnabled),
            "CLYDE_DISABLED" => Ok(GuildFeatures::ClydeDisabled),
            "COMMUNITY" => Ok(GuildFeatures::Community),
            "CREATOR_ACCEPTED_NEW_TERMS" => Ok(GuildFeatures::CreatorAcceptedNewTerms),
            "CREATOR_MONETIZABLE" => Ok(GuildFeatures::CreatorMonetizable),
            "CREATOR_MONETIZABLE_DISABLED" => Ok(GuildFeatures::CreatorMonetizableDisabled),
            "CREATOR_MONETIZABLE_PENDING_NEW_OWNER_ONBOARDING" => {
                Ok(GuildFeatures::CreatorMonetizablePendingNewOwnerOnboarding)
            }
            "CREATOR_MONETIZABLE_PROVISIONAL" => Ok(GuildFeatures::CreatorMonetizableProvisional),
            "CREATOR_MONETIZABLE_RESTRICTED" => Ok(GuildFeatures::CreatorMonetizableRestricted),
            "CREATOR_MONETIZABLE_WHITEGLOVE" => Ok(GuildFeatures::CreatorMonetizableWhiteglove),
            "CREATOR_MONETIZABLE_APPLICATION_ALLOWLIST" => {
                Ok(GuildFeatures::CreatorMonetizableApplicationAllowlist)
            }
            "CREATE_STORE_PAGE" => Ok(GuildFeatures::CreateStorePage),
            "DEVELOPER_SUPPORT_SERVER" => Ok(GuildFeatures::DeveloperSupportServer),
            "DISCOVERABLE_DISABLED" => Ok(GuildFeatures::DiscoverableDisabled),
            "DISCOVERABLE" => Ok(GuildFeatures::Discoverable),
            "ENABLED_DISCOVERABLE_BEFORE" => Ok(GuildFeatures::EnabledDiscoverableBefore),
            "EXPOSED_TO_ACTIVITIES_WTP_EXPERIMENT" => {
                Ok(GuildFeatures::ExposedToActivitiesWTPExperiment)
            }
            "GUESTS_ENABLED" => Ok(GuildFeatures::GuestsEnabled),
            "GUILD_AUTOMOD_DEFAULT_LIST" => Ok(GuildFeatures::GuildAutomodDefaultList),
            "GUILD_COMMUNICATION_DISABLED_GUILDS" => {
                Ok(GuildFeatures::GuildCommunicationDisabledGuilds)
            }
            "GUILD_HOME_DEPRECATION_OVERRIDE" => Ok(GuildFeatures::GuildHomeDeprecationOverride),
            "GUILD_HOME_OVERRIDE" => Ok(GuildFeatures::GuildHomeOverride),
            "GUILD_HOME_TEST" => Ok(GuildFeatures::GuildHomeTest),
            "GUILD_MEMBER_VERIFICATION_EXPERIMENT" => {
                Ok(GuildFeatures::GuildMemberVerificationExperiment)
            }
            "GUILD_ONBOARDING" => Ok(GuildFeatures::GuildOnboarding),
            "GUILD_ONBOARDING_ADMIN_ONLY" => Ok(GuildFeatures::GuildOnboardingAdminOnly),
            "GUILD_ONBOARDING_EVER_ENABLED" => Ok(GuildFeatures::GuildOnboardingEverEnabled),
            "GUILD_ONBOARDING_HAS_PROMPTS" => Ok(GuildFeatures::GuildOnboardingHasPrompts),
            "GUILD_ROLE_SUBSCRIPTION" => Ok(GuildFeatures::GuildRoleSubscription),
            "GUILD_ROLE_SUBSCRIPTION_PURCHASE_FEEDBACK_LOOP" => {
                Ok(GuildFeatures::GuildRoleSubscriptionPurchaseFeedbackLoop)
            }
            "GUILD_ROLE_SUBSCRIPTION_TRIALS" => Ok(GuildFeatures::GuildRoleSubscriptionTrials),
            "GUILD_SERVER_GUIDE" => Ok(GuildFeatures::GuildServerGuide),
            "GUILD_WEB_PAGE_VANITY_URL" => Ok(GuildFeatures::GuildWebPageVanityURL),
            "HAD_EARLY_ACTIVITIES_ACCESS" => Ok(GuildFeatures::HadEarlyActivitiesAccess),
            "HAS_DIRECTORY_ENTRY" => Ok(GuildFeatures::HasDirectoryEntry),
            "HIDE_FROM_EXPERIMENT_UI" => Ok(GuildFeatures::HideFromExperimentUi),
            "HUB" => Ok(GuildFeatures::Hub),
            "INCREASED_THREAD_LIMIT" => Ok(GuildFeatures::IncreasedThreadLimit),
            "INTERNAL_EMPLOYEE_ONLY" => Ok(GuildFeatures::InternalEmployeeOnly),
            "INVITE_SPLASH" => Ok(GuildFeatures::InviteSplash),
            "INVITES_DISABLED" => Ok(GuildFeatures::InvitesDisabled),
            "LINKED_TO_HUB" => Ok(GuildFeatures::LinkedToHub),
            "MARKETPLACES_CONNECTION_ROLES" => Ok(GuildFeatures::MarketplacesConnectionRoles),
            "MEMBER_PROFILES" => Ok(GuildFeatures::MemberProfiles),
            "MEMBER_VERIFICATION_GATE_ENABLED" => Ok(GuildFeatures::MemberVerificationGateEnabled),
            "MEMBER_VERIFICATION_MANUAL_APPROVAL" => {
                Ok(GuildFeatures::MemberVerificationManualApproval)
            }
            "MOBILE_WEB_ROLE_SUBSCRIPTION_PURCHASE_PAGE" => {
                Ok(GuildFeatures::MobileWebRoleSubscriptionPurchasePage)
            }
            "MONETIZATION_ENABLED" => Ok(GuildFeatures::MonetizationEnabled),
            "MORE_EMOJI" => Ok(GuildFeatures::MoreEmoji),
            "MORE_STICKERS" => Ok(GuildFeatures::MoreStickers),
            "NEWS" => Ok(GuildFeatures::News),
            "NEW_THREAD_PERMISSIONS" => Ok(GuildFeatures::NewThreadPermissions),
            "PARTNERED" => Ok(GuildFeatures::Partnered),
            "PREMIUM_TIER_3_OVERRIDE" => Ok(GuildFeatures::PremiumTier3Override),
            "PREVIEW_ENABLED" => Ok(GuildFeatures::PreviewEnabled),
            "RAID_ALERTS_DISABLED" => Ok(GuildFeatures::RaidAlertsDisabled),
            "RELAY_ENABLED" => Ok(GuildFeatures::RelayEnabled),
            "RESTRICT_SPAM_RISK_GUILD" => Ok(GuildFeatures::RestrictSpamRiskGuild),
            "ROLE_ICONS" => Ok(GuildFeatures::RoleIcons),
            "ROLE_SUBSCRIPTIONS_AVAILABLE_FOR_PURCHASE" => {
                Ok(GuildFeatures::RoleSubscriptionsAvailableForPurchase)
            }
            "ROLE_SUBSCRIPTIONS_ENABLED" => Ok(GuildFeatures::RoleSubscriptionsEnabled),
            "ROLE_SUBSCRIPTIONS_ENABLED_FOR_PURCHASE" => {
                Ok(GuildFeatures::RoleSubscriptionsEnabledForPurchase)
            }
            "SHARD" => Ok(GuildFeatures::Shard),
            "SHARED_CANVAS_FRIENDS_AND_FAMILY_TEST" => {
                Ok(GuildFeatures::SharedCanvasFriendsAndFamilyTest)
            }
            "SOUNDBOARD" => Ok(GuildFeatures::Soundboard),
            "SUMMARIES_ENABLED" => Ok(GuildFeatures::SummariesEnabled),
            "SUMMARIES_ENABLED_GA" => Ok(GuildFeatures::SummariesEnabledGa),
            "SUMMARIES_DISABLED_BY_USER" => Ok(GuildFeatures::SummariesDisabledByUser),
            "SUMMARIES_ENABLED_BY_USER" => Ok(GuildFeatures::SummariesEnabledByUser),
            "TEXT_IN_STAGE_ENABLED" => Ok(GuildFeatures::TextInStageEnabled),
            "TEXT_IN_VOICE_ENABLED" => Ok(GuildFeatures::TextInVoiceEnabled),
            "THREADS_ENABLED_TESTING" => Ok(GuildFeatures::ThreadsEnabledTesting),
            "THREADS_ENABLED" => Ok(GuildFeatures::ThreadsEnabled),
            "THREAD_DEFAULT_AUTO_ARCHIVE_DURATION" => {
                Ok(GuildFeatures::ThreadDefaultAutoArchiveDuration)
            }
            "THREADS_ONLY_CHANNEL" => Ok(GuildFeatures::ThreadsOnlyChannel),
            "TICKETED_EVENTS_ENABLED" => Ok(GuildFeatures::TicketedEventsEnabled),
            "TICKETING_ENABLED" => Ok(GuildFeatures::TicketingEnabled),
            "VANITY_URL" => Ok(GuildFeatures::VanityUrl),
            "VERIFIED" => Ok(GuildFeatures::Verified),
            "VIP_REGIONS" => Ok(GuildFeatures::VipRegions),
            "VOICE_CHANNEL_EFFECTS" => Ok(GuildFeatures::VoiceChannelEffects),
            "WELCOME_SCREEN_ENABLED" => Ok(GuildFeatures::WelcomeScreenEnabled),
            "ALIASABLE_NAMES" => Ok(GuildFeatures::AliasableNames),
            "ALLOW_INVALID_CHANNEL_NAME" => Ok(GuildFeatures::AllowInvalidChannelName),
            "ALLOW_UNNAMED_CHANNELS" => Ok(GuildFeatures::AllowUnnamedChannels),
            "CROSS_CHANNEL_REPLIES" => Ok(GuildFeatures::CrossChannelReplies),
            "IRC_LIKE_CATEGORY_NAMES" => Ok(GuildFeatures::IrcLikeCategoryNames),
            "INVITES_CLOSED" => Ok(GuildFeatures::InvitesClosed),
            _ => Err(Error::Guild(GuildError::InvalidGuildFeature)),
        }
    }
}

impl GuildFeatures {
    pub fn to_str(&self) -> &'static str {
        match *self {
            GuildFeatures::ActivitiesAlpha => "ACTIVITIES_ALPHA",
            GuildFeatures::ActivitiesEmployee => "ACTIVITIES_EMPLOYEE",
            GuildFeatures::ActivitiesInternalDev => "ACTIVITIES_INTERNAL_DEV",
            GuildFeatures::AnimatedBanner => "ANIMATED_BANNER",
            GuildFeatures::AnimatedIcon => "ANIMATED_ICON",
            GuildFeatures::ApplicationCommandPermissionsV2 => "APPLICATION_COMMAND_PERMISSIONS_V2",
            GuildFeatures::AutoModeration => "AUTO_MODERATION",
            GuildFeatures::AutoModTriggerKeywordFilter => "AUTO_MOD_TRIGGER_KEYWORD_FILTER",
            GuildFeatures::AutoModTriggerMLSpamFilter => "AUTO_MOD_TRIGGER_ML_SPAM_FILTER",
            GuildFeatures::AutoModTriggerSpamLinkFilter => "AUTO_MOD_TRIGGER_SPAM_LINK_FILTER",
            GuildFeatures::AutoModTriggerUserProfile => "AUTO_MOD_TRIGGER_USER_PROFILE",
            GuildFeatures::Banner => "BANNER",
            GuildFeatures::Bfg => "BFG",
            GuildFeatures::BoostingTiersExperimentMediumGuild => {
                "BOOSTING_TIERS_EXPERIMENT_MEDIUM_GUILD"
            }
            GuildFeatures::BoostingTiersExperimentSmallGuild => {
                "BOOSTING_TIERS_EXPERIMENT_SMALL_GUILD"
            }
            GuildFeatures::BotDeveloperEarlyAccess => "BOT_DEVELOPER_EARLY_ACCESS",
            GuildFeatures::BurstReactions => "BURST_REACTIONS",
            GuildFeatures::CommunityCanary => "COMMUNITY_CANARY",
            GuildFeatures::CommunityExpLargeGated => "COMMUNITY_EXP_LARGE_GATED",
            GuildFeatures::CommunityExpLargeUngated => "COMMUNITY_EXP_LARGE_UNGATED",
            GuildFeatures::CommunityExpMedium => "COMMUNITY_EXP_MEDIUM",
            GuildFeatures::ChannelEmojisGenerated => "CHANNEL_EMOJIS_GENERATED",
            GuildFeatures::ChannelHighlights => "CHANNEL_HIGHLIGHTS",
            GuildFeatures::ChannelHighlightsDisabled => "CHANNEL_HIGHLIGHTS_DISABLED",
            GuildFeatures::ClydeEnabled => "CLYDE_ENABLED",
            GuildFeatures::ClydeExperimentEnabled => "CLYDE_EXPERIMENT_ENABLED",
            GuildFeatures::ClydeDisabled => "CLYDE_DISABLED",
            GuildFeatures::Community => "COMMUNITY",
            GuildFeatures::CreatorAcceptedNewTerms => "CREATOR_ACCEPTED_NEW_TERMS",
            GuildFeatures::CreatorMonetizable => "CREATOR_MONETIZABLE",
            GuildFeatures::CreatorMonetizableDisabled => "CREATOR_MONETIZABLE_DISABLED",
            GuildFeatures::CreatorMonetizablePendingNewOwnerOnboarding => {
                "CREATOR_MONETIZABLE_PENDING_NEW_OWNER_ONBOARDING"
            }
            GuildFeatures::CreatorMonetizableProvisional => "CREATOR_MONETIZABLE_PROVISIONAL",
            GuildFeatures::CreatorMonetizableRestricted => "CREATOR_MONETIZABLE_RESTRICTED",
            GuildFeatures::CreatorMonetizableWhiteglove => "CREATOR_MONETIZABLE_WHITEGLOVE",
            GuildFeatures::CreatorMonetizableApplicationAllowlist => {
                "CREATOR_MONETIZABLE_APPLICATION_ALLOWLIST"
            }
            GuildFeatures::CreateStorePage => "CREATE_STORE_PAGE",
            GuildFeatures::DeveloperSupportServer => "DEVELOPER_SUPPORT_SERVER",
            GuildFeatures::DiscoverableDisabled => "DISCOVERABLE_DISABLED",
            GuildFeatures::Discoverable => "DISCOVERABLE",
            GuildFeatures::EnabledDiscoverableBefore => "ENABLED_DISCOVERABLE_BEFORE",
            GuildFeatures::ExposedToActivitiesWTPExperiment => {
                "EXPOSED_TO_ACTIVITIES_WTP_EXPERIMENT"
            }
            GuildFeatures::GuestsEnabled => "GUESTS_ENABLED",
            GuildFeatures::GuildAutomodDefaultList => "GUILD_AUTOMOD_DEFAULT_LIST",
            GuildFeatures::GuildCommunicationDisabledGuilds => {
                "GUILD_COMMUNICATION_DISABLED_GUILDS"
            }
            GuildFeatures::GuildHomeDeprecationOverride => "GUILD_HOME_DEPRECATION_OVERRIDE",
            GuildFeatures::GuildHomeOverride => "GUILD_HOME_OVERRIDE",
            GuildFeatures::GuildHomeTest => "GUILD_HOME_TEST",
            GuildFeatures::GuildMemberVerificationExperiment => {
                "GUILD_MEMBER_VERIFICATION_EXPERIMENT"
            }
            GuildFeatures::GuildOnboarding => "GUILD_ONBOARDING",
            GuildFeatures::GuildOnboardingAdminOnly => "GUILD_ONBOARDING_ADMIN_ONLY",
            GuildFeatures::GuildOnboardingEverEnabled => "GUILD_ONBOARDING_EVER_ENABLED",
            GuildFeatures::GuildOnboardingHasPrompts => "GUILD_ONBOARDING_HAS_PROMPTS",
            GuildFeatures::GuildRoleSubscription => "GUILD_ROLE_SUBSCRIPTION",
            GuildFeatures::GuildRoleSubscriptionPurchaseFeedbackLoop => {
                "GUILD_ROLE_SUBSCRIPTION_PURCHASE_FEEDBACK_LOOP"
            }
            GuildFeatures::GuildRoleSubscriptionTrials => "GUILD_ROLE_SUBSCRIPTION_TRIALS",
            GuildFeatures::GuildServerGuide => "GUILD_SERVER_GUIDE",
            GuildFeatures::GuildWebPageVanityURL => "GUILD_WEB_PAGE_VANITY_URL",
            GuildFeatures::HadEarlyActivitiesAccess => "HAD_EARLY_ACTIVITIES_ACCESS",
            GuildFeatures::HasDirectoryEntry => "HAS_DIRECTORY_ENTRY",
            GuildFeatures::HideFromExperimentUi => "HIDE_FROM_EXPERIMENT_UI",
            GuildFeatures::Hub => "HUB",
            GuildFeatures::IncreasedThreadLimit => "INCREASED_THREAD_LIMIT",
            GuildFeatures::InternalEmployeeOnly => "INTERNAL_EMPLOYEE_ONLY",
            GuildFeatures::InviteSplash => "INVITE_SPLASH",
            GuildFeatures::InvitesDisabled => "INVITES_DISABLED",
            GuildFeatures::LinkedToHub => "LINKED_TO_HUB",
            GuildFeatures::MarketplacesConnectionRoles => "MARKETPLACES_CONNECTION_ROLES",
            GuildFeatures::MemberProfiles => "MEMBER_PROFILES",
            GuildFeatures::MemberVerificationGateEnabled => "MEMBER_VERIFICATION_GATE_ENABLED",
            GuildFeatures::MemberVerificationManualApproval => {
                "MEMBER_VERIFICATION_MANUAL_APPROVAL"
            }
            GuildFeatures::MobileWebRoleSubscriptionPurchasePage => {
                "MOBILE_WEB_ROLE_SUBSCRIPTION_PURCHASE_PAGE"
            }
            GuildFeatures::MonetizationEnabled => "MONETIZATION_ENABLED",
            GuildFeatures::MoreEmoji => "MORE_EMOJI",
            GuildFeatures::MoreStickers => "MORE_STICKERS",
            GuildFeatures::News => "NEWS",
            GuildFeatures::NewThreadPermissions => "NEW_THREAD_PERMISSIONS",
            GuildFeatures::Partnered => "PARTNERED",
            GuildFeatures::PremiumTier3Override => "PREMIUM_TIER_3_OVERRIDE",
            GuildFeatures::PreviewEnabled => "PREVIEW_ENABLED",
            GuildFeatures::RaidAlertsDisabled => "RAID_ALERTS_DISABLED",
            GuildFeatures::RelayEnabled => "RELAY_ENABLED",
            GuildFeatures::RestrictSpamRiskGuild => "RESTRICT_SPAM_RISK_GUILD",
            GuildFeatures::RoleIcons => "ROLE_ICONS",
            GuildFeatures::RoleSubscriptionsAvailableForPurchase => {
                "ROLE_SUBSCRIPTIONS_AVAILABLE_FOR_PURCHASE"
            }
            GuildFeatures::RoleSubscriptionsEnabled => "ROLE_SUBSCRIPTIONS_ENABLED",
            GuildFeatures::RoleSubscriptionsEnabledForPurchase => {
                "ROLE_SUBSCRIPTIONS_ENABLED_FOR_PURCHASE"
            }
            GuildFeatures::Shard => "SHARD",
            GuildFeatures::SharedCanvasFriendsAndFamilyTest => {
                "SHARED_CANVAS_FRIENDS_AND_FAMILY_TEST"
            }
            GuildFeatures::Soundboard => "SOUNDBOARD",
            GuildFeatures::SummariesEnabled => "SUMMARIES_ENABLED",
            GuildFeatures::SummariesEnabledGa => "SUMMARIES_ENABLED_GA",
            GuildFeatures::SummariesDisabledByUser => "SUMMARIES_DISABLED_BY_USER",
            GuildFeatures::SummariesEnabledByUser => "SUMMARIES_ENABLED_BY_USER",
            GuildFeatures::TextInStageEnabled => "TEXT_IN_STAGE_ENABLED",
            GuildFeatures::TextInVoiceEnabled => "TEXT_IN_VOICE_ENABLED",
            GuildFeatures::ThreadsEnabledTesting => "THREADS_ENABLED_TESTING",
            GuildFeatures::ThreadsEnabled => "THREADS_ENABLED",
            GuildFeatures::ThreadDefaultAutoArchiveDuration => {
                "THREAD_DEFAULT_AUTO_ARCHIVE_DURATION"
            }
            GuildFeatures::ThreadsOnlyChannel => "THREADS_ONLY_CHANNEL",
            GuildFeatures::TicketedEventsEnabled => "TICKETED_EVENTS_ENABLED",
            GuildFeatures::TicketingEnabled => "TICKETING_ENABLED",
            GuildFeatures::VanityUrl => "VANITY_URL",
            GuildFeatures::Verified => "VERIFIED",
            GuildFeatures::VipRegions => "VIP_REGIONS",
            GuildFeatures::VoiceChannelEffects => "VOICE_CHANNEL_EFFECTS",
            GuildFeatures::WelcomeScreenEnabled => "WELCOME_SCREEN_ENABLED",
            GuildFeatures::AliasableNames => "ALIASABLE_NAMES",
            GuildFeatures::AllowInvalidChannelName => "ALLOW_INVALID_CHANNEL_NAME",
            GuildFeatures::AllowUnnamedChannels => "ALLOW_UNNAMED_CHANNELS",
            GuildFeatures::CrossChannelReplies => "CROSS_CHANNEL_REPLIES",
            GuildFeatures::IrcLikeCategoryNames => "IRC_LIKE_CATEGORY_NAMES",
            GuildFeatures::InvitesClosed => "INVITES_CLOSED",
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GuildConfiguration {
    pub discovery: DiscoverConfiguration,
    pub auto_join: AutoJoinConfiguration,
    #[serde(default)]
    pub default_features: Vec<GuildFeatures>,
}
