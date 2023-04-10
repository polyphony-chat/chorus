pub mod limits {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Deserialize, Serialize)]
    #[allow(non_snake_case)]
    pub struct User {
        pub maxGuilds: u64,
        pub maxUsername: u64,
        pub maxFriends: u64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[allow(non_snake_case)]

    pub struct Guild {
        pub maxRoles: u64,
        pub maxEmojis: u64,
        pub maxMembers: u64,
        pub maxChannels: u64,
        pub maxChannelsInCategory: u64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[allow(non_snake_case)]

    pub struct Message {
        pub maxCharacters: u64,
        pub maxTTSCharacters: u64,
        pub maxReactions: u64,
        pub maxAttachmentSize: u64,
        pub maxBulkDelete: u64,
        pub maxEmbedDownloadSize: u64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[allow(non_snake_case)]

    pub struct Channel {
        pub maxPins: u64,
        pub maxTopic: u64,
        pub maxWebhooks: u64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Rate {
        pub enabled: bool,
        pub ip: Window,
        pub global: Window,
        pub error: Window,
        pub routes: Routes,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Window {
        pub count: u64,
        pub window: u64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Routes {
        pub guild: Window,
        pub webhook: Window,
        pub channel: Window,
        pub auth: AuthRoutes,
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[allow(non_snake_case)]

    pub struct AuthRoutes {
        pub login: Window,
        pub register: Window,
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[allow(non_snake_case)]

    pub struct AbsoluteRate {
        pub register: AbsoluteWindow,
        pub sendMessage: AbsoluteWindow,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct AbsoluteWindow {
        pub limit: u64,
        pub window: u64,
        pub enabled: bool,
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[allow(non_snake_case)]

    pub struct Config {
        pub user: User,
        pub guild: Guild,
        pub message: Message,
        pub channel: Channel,
        pub rate: Rate,
        pub absoluteRate: AbsoluteRate,
    }
}
