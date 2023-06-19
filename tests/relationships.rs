use chorus::types::{self, RegisterSchema, RegisterSchemaOptions};

mod common;

#[tokio::test]
async fn test_get_mutual_relationships() {
    let register_schema = RegisterSchemaOptions {
        date_of_birth: Some("2000-01-01".to_string()),
        ..RegisterSchema::builder("integrationtestuser2", true)
    }
    .build()
    .unwrap();

    let mut bundle = common::setup().await;
    let belongs_to = &mut bundle.instance;
    let user = &mut bundle.user;
    let mut other_user = belongs_to.register_account(&register_schema).await.unwrap();
    let friend_request_schema = types::FriendRequestSendSchema {
        username: user.object.username.clone(),
        discriminator: Some(user.object.discriminator.clone()),
    };
    other_user.send_friend_request(friend_request_schema).await;
    let relationships = user
        .get_mutual_relationships(&other_user.object.id.to_string())
        .await
        .unwrap();
    println!("{:?}", relationships);
    common::teardown(bundle).await
}

#[tokio::test]
async fn test_get_relationships() {
    let register_schema = RegisterSchemaOptions {
        date_of_birth: Some("2000-01-01".to_string()),
        ..RegisterSchema::builder("integrationtestuser2", true)
    }
    .build()
    .unwrap();

    let mut bundle = common::setup().await;
    let belongs_to = &mut bundle.instance;
    let user = &mut bundle.user;
    let mut other_user = belongs_to.register_account(&register_schema).await.unwrap();
    let friend_request_schema = types::FriendRequestSendSchema {
        username: user.object.username.clone(),
        discriminator: Some(user.object.discriminator.clone()),
    };
    other_user.send_friend_request(friend_request_schema).await;
    let relationships = user.get_relationships().await.unwrap();
    println!("{:?}", relationships);
    common::teardown(bundle).await
}
