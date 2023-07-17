mod common;

#[tokio::test]
async fn create_accept_invite() {
    let mut bundle = common::setup().await;
    let user = &mut bundle.user;
    let mut other_user = bundle.create_user("testuser1312").await;
}
