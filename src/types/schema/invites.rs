use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
/// Query parameters for routes that list invites.
/// # Reference:
/// Read: <https://discord-userdoccers.vercel.app/resources/invite#invite-structure>
pub struct GetInvitesSchema {
    pub with_counts: Option<bool>,
}