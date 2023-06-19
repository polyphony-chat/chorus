use std::fs::File;
use std::io::{BufReader, Read};

use chorus::types;

mod common;

#[tokio::test]
async fn send_message() {
    let mut bundle = common::setup().await;
    let mut message = types::MessageSendSchema {
        content: Some("A Message!".to_string()),
        ..Default::default()
    };
    let _ = bundle
        .user
        .send_message(&mut message, bundle.channel.id.to_string(), None)
        .await
        .unwrap();
    common::teardown(bundle).await
}

#[tokio::test]
async fn send_message_attachment() {
    let f = File::open("./README.md").unwrap();
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    let mut bundle = common::setup().await;

    reader.read_to_end(&mut buffer).unwrap();

    let attachment = types::PartialDiscordFileAttachment {
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
        content: buffer,
    };

    let mut message = types::MessageSendSchema {
        content: Some("trans rights now".to_string()),
        attachments: Some(vec![attachment.clone()]),
        ..Default::default()
    };

    let vec_attach = vec![attachment.clone()];
    let _arg = Some(&vec_attach);
    bundle
        .user
        .send_message(
            &mut message,
            bundle.channel.id.to_string(),
            Some(vec![attachment.clone()]),
        )
        .await
        .unwrap();
    common::teardown(bundle).await
}
