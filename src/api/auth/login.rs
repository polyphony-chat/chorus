pub mod login {
    use crate::api::schemas::schemas::LoginSchema;
    use crate::instance::Instance;

    impl Instance {
        pub async fn login_account(&mut self, login_schema: &LoginSchema) {}
    }
}
