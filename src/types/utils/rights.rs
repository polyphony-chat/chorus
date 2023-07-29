use bitflags::bitflags;

bitflags! {
    /// Rights are instance-wide, per-user permissions for everything you may perform on the instance,
    /// such as sending messages, editing messages, or shutting down the server.
    /// They are separate from guild member permissions, which only apply to a given guild.
    ///
    /// # Notes
    /// The default rights on Discord.com are 648540060672 ([source](https://github.com/spacebarchat/server/issues/878#issuecomment-1234669715))
    ///
    /// # Reference
    /// See <https://docs.spacebar.chat/setup/server/security/rights/>
    pub struct Rights: u64 {
        /// All rights
        const OPERATOR = 1 << 0;
        /// Ability to alter or remove others' applications
        const MANAGE_APPLICATIONS = 1 << 1;
        /// Same as the per-guild [MANAGE_GUILD] permission, but applies to all guilds and DM channels, can join any guild without invite
        const MANAGE_GUILDS = 1 << 2;
        /// Can delete or edit any message they can read
        const MANAGE_MESSAGES = 1 << 3;
        /// Can add, change, define rate limits of other users,
        /// can also grant others [BYPASS_RATE_LIMITS] when combined
        /// with [BYPASS_RATE_LIMITS] and [MANAGE_USERS].
        const MANAGE_RATE_LIMITS = 1 << 4;
        /// Can create, alter, enable and disable custom message routing rules in any channel/guild
        const MANAGE_ROUTING = 1 << 5;
        /// Respond to or resolve other users' support tickets
        const MANAGE_TICKETS = 1 << 6;
        /// Can create, alter, remove and ban users; can also create, modify and remove user groups
        const MANAGE_USERS = 1 << 7;
        /// Can manually add members into their guilds and group DMs
        const ADD_MEMBERS = 1 << 8;
        /// Makes the user exempt from all rate limits
        const BYPASS_RATE_LIMITS = 1 << 9;
        /// Can create, edit and remove own applications
        const CREATE_APPLICATIONS = 1 << 10;
        /// Can create guild channels and custom channels
        const CREATE_CHANNELS = 1 << 11;
        /// Can create 1:1 DMs
        ///
        /// # Notes
        /// A user without [SEND_MESSAGES] cannot be added to a DM
        const CREATE_DMS = 1 << 12;
        /// Can create group DMs
        ///
        /// # Notes
        /// A user without [SEND_MESSAGES] cannot be added to a DM
        const CREATE_DM_GROUPS = 1 << 13;
        /// Can create guilds
        const CREATE_GUILDS = 1 << 14;
        /// Can create mass invites in guilds where they have [CREATE_INSTANT_INVITE]
        const CREATE_INVITES = 1 << 15;
        /// Can create roles and per-guild or per-channel permission
        /// overrides in the guilds that they have permissions
        const CREATE_ROLES = 1 << 16;
        /// Can create templates for guilds, custom channels and channels with custom routing
        const CREATE_TEMPLATES = 1 << 17;
        /// Can create webhooks in the guilds that they have permissions
        const CREATE_WEBHOOKS = 1 << 18;
        /// Can join guilds by using invites or vanity names
        const JOIN_GUILDS = 1 << 19;
        /// Can modify the pinned messages in the guilds that they have permission
        const PIN_MESSAGES = 1 << 20;
        /// Can react to messages, subject to permissions
        const SELF_ADD_REACTIONS = 1 << 21;
        /// Can delete own messages
        const SELF_DELETE_MESSAGES = 1 << 22;
        /// Can edit own messages
        const SELF_EDIT_MESSAGES = 1 << 23;
        /// Can edit own username, nickname and avatar
        const SELF_EDIT_NAME = 1 << 24;
        /// Can send messages in the channels that they have permissions
        const SEND_MESSAGES = 1 << 25;
        /// Can use voice activities, such as watch together or whiteboard
        const USE_ACTIVITIES = 1 << 26;
        /// Can use video and screenshare in guilds/channels that they have permissions
        const USE_VIDEO = 1 << 27;
        /// Can use voice in guilds/channels that they have permissions
        const USE_VOICE = 1 << 28;
        /// Can create user-specific invites in guilds that they have INVITE_USERS
        const INVITE_USERS = 1 << 29;
        /// Can delete/disable own account
        const SELF_DELETE_DISABLE = 1 << 30;
        /// Can use pay-to-use features once paid
        const DEBTABLE = 1 << 31;
        /// Can earn money using monetization features in guilds that have MONETIZATION_ENABLED
        const CREDITABLE = 1 << 32;
        /// Can kick or ban guild or group DM members in the guilds/groups where they have KICK_MEMBERS or BAN_MEMBERS
        const KICK_BAN_MEMBERS = 1 << 33;
        /// Can leave the guilds or group DMs that they joined on their own (one can always leave a guild or group DMs where they have been force-added)
        const SELF_LEAVE_GROUPS = 1 << 34;
        /// Inverts the presence confidentiality default (OPERATOR's presence is not routed by default, others' are) for a given user
        const PRESENCE = 1 << 35;
        /// Can mark discoverable guilds where they have permissions to mark as discoverable
        const SELF_ADD_DISCOVERABLE = 1 << 36;
        /// Can change anything in the primary guild directory
        const MANAGE_GUILD_DIRECTORY = 1 << 37;
        /// Can send confetti, screenshake and use the random user mention (@someone)
        const POGGERS = 1 << 38;
        /// Can use achievements and cheers
        const USE_ACHIEVEMENTS = 1 << 39;
        /// Can initiate interactions
        const INITIATE_INTERACTIONS = 1 << 40;
        /// Can respond to interactions
        const RESPOND_TO_INTERACTIONS = 1 << 41;
        /// Can send backdated events
        const SEND_BACKDATED_EVENTS = 1 << 42;
        /// Can accept mass (guild) invites
        const USE_MASS_INVITES = 1 << 43;
        /// Can accept user-specific invites and DM requests
        const ACCEPT_INVITES = 1 << 44;
        /// Can modify own flags
        const SELF_EDIT_FLAGS = 1 << 45;
        /// Can modify other's flags
        const EDIT_FLAGS = 1 << 46;
        /// Can manage other's groups
        const MANAGE_GROUPS = 1 << 47;
        /// Can view server stats at /api/policies/stats
        const VIEW_SERVER_STATS = 1 << 48;
        /// Can resend verification emails using /auth/verify/resend
        const RESEND_VERIFICATION_EMAIL = 1 << 49;
    }
}

impl Rights {
    pub fn any(&self, permission: Rights, check_operator: bool) -> bool {
        (check_operator && self.contains(Rights::OPERATOR)) || self.contains(permission)
    }

    /// Returns whether or not the Rights object has specific rights
    pub fn has(&self, permission: Rights, check_operator: bool) -> bool {
        (check_operator && self.contains(Rights::OPERATOR)) || self.contains(permission)
    }

    /// Returns whether or not the Rights object has specific rights.
    ///
    /// # Notes
    /// Unlike has, this returns an Error if we are missing rights
    /// and Ok(true) otherwise
    pub fn has_throw(&self, permission: Rights) -> Result<bool, &'static str> {
        if self.has(permission, true) {
            Ok(true)
        } else {
            Err("You are missing the following rights")
        }
    }
}

#[allow(dead_code)] // FIXME: Remove this when we  use this
fn all_rights() -> Rights {
    Rights::OPERATOR
        | Rights::MANAGE_APPLICATIONS
        | Rights::MANAGE_GUILDS
        | Rights::MANAGE_MESSAGES
        | Rights::MANAGE_RATE_LIMITS
        | Rights::MANAGE_ROUTING
        | Rights::MANAGE_TICKETS
        | Rights::MANAGE_USERS
        | Rights::ADD_MEMBERS
        | Rights::BYPASS_RATE_LIMITS
        | Rights::CREATE_APPLICATIONS
        | Rights::CREATE_CHANNELS
        | Rights::CREATE_DMS
        | Rights::CREATE_DM_GROUPS
        | Rights::CREATE_GUILDS
        | Rights::CREATE_INVITES
        | Rights::CREATE_ROLES
        | Rights::CREATE_TEMPLATES
        | Rights::CREATE_WEBHOOKS
        | Rights::JOIN_GUILDS
        | Rights::PIN_MESSAGES
        | Rights::SELF_ADD_REACTIONS
        | Rights::SELF_DELETE_MESSAGES
        | Rights::SELF_EDIT_MESSAGES
        | Rights::SELF_EDIT_NAME
        | Rights::SEND_MESSAGES
        | Rights::USE_ACTIVITIES
        | Rights::USE_VIDEO
        | Rights::USE_VOICE
        | Rights::INVITE_USERS
        | Rights::SELF_DELETE_DISABLE
        | Rights::DEBTABLE
        | Rights::CREDITABLE
        | Rights::KICK_BAN_MEMBERS
        | Rights::SELF_LEAVE_GROUPS
        | Rights::PRESENCE
        | Rights::SELF_ADD_DISCOVERABLE
        | Rights::MANAGE_GUILD_DIRECTORY
        | Rights::POGGERS
        | Rights::USE_ACHIEVEMENTS
        | Rights::INITIATE_INTERACTIONS
        | Rights::RESPOND_TO_INTERACTIONS
        | Rights::SEND_BACKDATED_EVENTS
        | Rights::USE_MASS_INVITES
        | Rights::ACCEPT_INVITES
        | Rights::SELF_EDIT_FLAGS
        | Rights::EDIT_FLAGS
        | Rights::MANAGE_GROUPS
        | Rights::VIEW_SERVER_STATS
        | Rights::RESEND_VERIFICATION_EMAIL
}
