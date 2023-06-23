pub mod limits {
    use std::collections::HashMap;

    use reqwest::Client;
    use serde::{Deserialize, Serialize};
    use serde_json::from_str;

    #[derive(Clone, Copy, Eq, Hash, PartialEq, Debug, Default)]
    pub enum LimitType {
        AuthRegister,
        AuthLogin,
        AbsoluteMessage,
        AbsoluteRegister,
        #[default]
        Global,
        Ip,
        Channel,
        Error,
        Guild,
        Webhook,
    }
}
