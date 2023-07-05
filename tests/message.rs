use chorus::errors::ChorusResult;
use chorus::types::{MessageSendSchema, PartialDiscordFileAttachment};

mod common;

#[tokio::test]
async fn send_message() -> ChorusResult<()> {
    let mut bundle = common::setup().await;
    let mut message = MessageSendSchema {
        content: Some("A Message!".to_string()),
        ..Default::default()
    };
    let _ = bundle
        .user
        .send_message(&mut message, bundle.channel.id, None)
        .await?;
    common::teardown(bundle).await
}

#[tokio::test]
async fn send_message_attachment() -> ChorusResult<()> {
    let mut bundle = common::setup().await;

    let attachment = PartialDiscordFileAttachment {
        id: None,
        filename: "README.md".to_string(),
        description: None,
        content_type: None,
        size: None,
        url: None,
        proxy_url: None,
        width: None,
        height: None,
        ephemeral: None,
        duration_secs: None,
        waveform: None,
        content: include_bytes!("../README.md").to_vec(),
    };

    let mut message = MessageSendSchema {
        content: Some("trans rights now".to_string()),
        attachments: Some(vec![attachment.clone()]),
        ..Default::default()
    };

    bundle
        .user
        .send_message(&mut message, bundle.channel.id, Some(vec![attachment]))
        .await?;
    common::teardown(bundle).await
}
