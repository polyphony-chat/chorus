use chorus::{
    errors::ChorusResult,
    types::{FriendRequestSendSchema, RelationshipType},
};

mod common;

#[tokio::test]
async fn test_get_mutual_relationships() -> ChorusResult<()> {
    let mut bundle = common::setup().await;
    let user = &mut bundle.user;
    let friend_request_schema = FriendRequestSendSchema {
        username: user.object.username.clone(),
        discriminator: Some(user.object.discriminator.clone()),
    };
    bundle
        .other_user
        .send_friend_request(friend_request_schema)
        .await?;
    user.get_mutual_relationships(bundle.other_user.object.id)
        .await?;
    common::teardown(bundle).await
}

#[tokio::test]
async fn test_get_relationships() -> ChorusResult<()> {
    let mut bundle = common::setup().await;
    let user = &mut bundle.user;
    let friend_request_schema = FriendRequestSendSchema {
        username: user.object.username.clone(),
        discriminator: Some(user.object.discriminator.clone()),
    };
    bundle
        .other_user
        .send_friend_request(friend_request_schema)
        .await?;
    let relationships = user.get_relationships().await?;
    assert_eq!(relationships[0].id, bundle.other_user.object.id);
    common::teardown(bundle).await
}

#[tokio::test]
async fn test_modify_relationship_friends() -> ChorusResult<()> {
    let mut bundle = common::setup().await;
    let user = &mut bundle.user;
    bundle
        .other_user
        .modify_user_relationship(user.object.id, RelationshipType::Friends)
        .await?;

    let relationships = user.get_relationships().await?;
    assert_eq!(relationships[0].id, bundle.other_user.object.id);
    assert_eq!(
        relationships[0].relationship_type,
        RelationshipType::Incoming
    );

    let relationships = bundle.other_user.get_relationships().await?;
    assert_eq!(relationships[0].id, user.object.id);
    assert_eq!(
        relationships[0].relationship_type,
        RelationshipType::Outgoing
    );

    user.modify_user_relationship(bundle.other_user.object.id, RelationshipType::Friends)
        .await?;
    assert_eq!(
        bundle.other_user.get_relationships().await?[0].relationship_type,
        RelationshipType::Friends
    );

    user.remove_relationship(bundle.other_user.object.id)
        .await?;
    assert_eq!(bundle.other_user.get_relationships().await?, Vec::new());

    common::teardown(bundle).await
}

#[tokio::test]
async fn test_modify_relationship_block() -> ChorusResult<()> {
    let mut bundle = common::setup().await;
    let user = &mut bundle.user;
    bundle
        .other_user
        .modify_user_relationship(user.object.id, RelationshipType::Blocked)
        .await?;
    let relationships = user.get_relationships().await?;
    assert_eq!(relationships, Vec::new());

    let relationships = bundle.other_user.get_relationships().await?;
    assert_eq!(relationships[0].id, user.object.id);
    assert_eq!(
        relationships[0].relationship_type,
        RelationshipType::Blocked
    );

    bundle
        .other_user
        .remove_relationship(user.object.id)
        .await?;
    assert_eq!(bundle.other_user.get_relationships().await?, Vec::new());
    common::teardown(bundle).await
}
