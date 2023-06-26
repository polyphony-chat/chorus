pub mod limits {
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

    pub struct Ratelimits;

    pub struct Limit {}
}
