use chorus::types;

mod common;

#[tokio::test]
async fn test_get_mutual_relationships() {
    let register_schema = types::RegisterSchema::new(
        "integrationtestuser2".to_string(),
        None,
        true,
        None,
        None,
        None,
        Some("2000-01-01".to_string()),
        None,
        None,
        None,
    )
    .unwrap();

    let bundle = common::setup().await;
    let mut belongs_to = bundle.instance;
    let mut user = bundle.user;
    let other_user = belongs_to.register_account(&register_schema).await.unwrap();
    let relationships = user
        .get_mutual_relationships(&other_user.object.id.to_string())
        .await
        .unwrap();
    println!("{:?}", relationships.unwrap());
}
