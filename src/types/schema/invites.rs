use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
/// Query parameters for the `Get Invite` route.
///
/// # Reference:
/// Read: <https://docs.discord.sex/resources/invite#query-string-params>
pub struct GetInvitesSchema {
    pub with_counts: Option<bool>,
}