use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
/// Query parameters for the `Get Invite` route.
///
/// # Reference:
/// Read: <https://docs.discord.sex/resources/invite#query-string-params>
pub struct GetInvitesSchema {
    pub with_counts: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd, Eq, Ord)]
/// # Reference:
/// See <https://docs.discord.sex/resources/guild#get-guild-vanity-invite>
pub struct GuildVanityInviteResponse {
    pub code: String,
    #[serde(default)]
    pub uses: Option<u32>
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd, Eq, Ord)]
/// # Reference:
/// See <https://docs.discord.sex/resources/guild#modify-guild-vanity-invite>
pub struct GuildCreateVanitySchema {
    pub code: String,
}