use chorus::types::{self, Relationship, RelationshipType};

mod common;

#[tokio::test]
async fn test_get_mutual_relationships() {
    let mut bundle = common::setup().await;
    let mut other_user = bundle.create_user("integrationtestuser2").await;
    let user = &mut bundle.user;
    let username = user.object.read().unwrap().username.clone();
    let discriminator = user.object.read().unwrap().discriminator.clone();
    let other_user_id: types::Snowflake = other_user.object.read().unwrap().id;
    let friend_request_schema = types::FriendRequestSendSchema {
        username,
        discriminator: Some(discriminator),
    };
    other_user
        .send_friend_request(friend_request_schema)
        .await
        .unwrap();
    let relationships = user.get_mutual_relationships(other_user_id).await.unwrap();
    println!("{:?}", relationships);
    common::teardown(bundle).await
}

#[tokio::test]
async fn test_get_relationships() {
    let mut bundle = common::setup().await;
    let mut other_user = bundle.create_user("integrationtestuser2").await;
    let user = &mut bundle.user;
    let username = user.object.read().unwrap().username.clone();
    let discriminator = user.object.read().unwrap().discriminator.clone();
    let friend_request_schema = types::FriendRequestSendSchema {
        username,
        discriminator: Some(discriminator),
    };
    other_user
        .send_friend_request(friend_request_schema)
        .await
        .unwrap();
    let relationships = user.get_relationships().await.unwrap();
    assert_eq!(
        relationships.get(0).unwrap().id,
        other_user.object.read().unwrap().id
    );
    common::teardown(bundle).await
}

#[tokio::test]
async fn test_modify_relationship_friends() {
    let mut bundle = common::setup().await;
    let mut other_user = bundle.create_user("integrationtestuser2").await;
    let user = &mut bundle.user;
    let user_id: types::Snowflake = user.object.read().unwrap().id;
    let other_user_id: types::Snowflake = other_user.object.read().unwrap().id;

    other_user
        .modify_user_relationship(user_id, types::RelationshipType::Friends)
        .await
        .unwrap();
    let relationships = user.get_relationships().await.unwrap();
    assert_eq!(
        relationships.get(0).unwrap().id,
        other_user.object.read().unwrap().id
    );
    assert_eq!(
        relationships.get(0).unwrap().relationship_type,
        RelationshipType::Incoming
    );
    let relationships = other_user.get_relationships().await.unwrap();
    assert_eq!(
        relationships.get(0).unwrap().id,
        user.object.read().unwrap().id
    );
    assert_eq!(
        relationships.get(0).unwrap().relationship_type,
        RelationshipType::Outgoing
    );
    let _ = user
        .modify_user_relationship(other_user_id, RelationshipType::Friends)
        .await;
    assert_eq!(
        other_user
            .get_relationships()
            .await
            .unwrap()
            .get(0)
            .unwrap()
            .relationship_type,
        RelationshipType::Friends
    );
    let _ = user.remove_relationship(other_user_id).await;
    assert_eq!(
        other_user.get_relationships().await.unwrap(),
        Vec::<Relationship>::new()
    );
    common::teardown(bundle).await
}

#[tokio::test]
async fn test_modify_relationship_block() {
    let mut bundle = common::setup().await;
    let mut other_user = bundle.create_user("integrationtestuser2").await;
    let user = &mut bundle.user;
    let user_id: types::Snowflake = user.object.read().unwrap().id;

    other_user
        .modify_user_relationship(user_id, types::RelationshipType::Blocked)
        .await
        .unwrap();
    let relationships = user.get_relationships().await.unwrap();
    assert_eq!(relationships, Vec::<Relationship>::new());
    let relationships = other_user.get_relationships().await.unwrap();
    assert_eq!(
        relationships.get(0).unwrap().id,
        user.object.read().unwrap().id
    );
    assert_eq!(
        relationships.get(0).unwrap().relationship_type,
        RelationshipType::Blocked
    );
    other_user.remove_relationship(user_id).await.unwrap();
    assert_eq!(
        other_user.get_relationships().await.unwrap(),
        Vec::<Relationship>::new()
    );
    common::teardown(bundle).await
}
