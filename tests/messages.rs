// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::fs::File;
use std::io::{BufReader, Read};

use chorus::types::{self, Guild, Message, MessageSearchQuery};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::*;
#[cfg(target_arch = "wasm32")]
wasm_bindgen_test_configure!(run_in_browser);

mod common;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn send_message() {
    let mut bundle = common::setup().await;
    let message = types::MessageSendSchema {
        content: Some("A Message!".to_string()),
        ..Default::default()
    };
    let channel = bundle.channel.read().unwrap().clone();
    let _ = bundle.user.send_message(message, channel.id).await.unwrap();
    common::teardown(bundle).await
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn send_message_attachment() {
    /// FIXME: Fix this test for wasm32. wasm32-unknown-unknown does not have a filesystem and therefore cannot read files.
    #[cfg(target_arch = "wasm32")]
    return;
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

    let message = types::MessageSendSchema {
        content: Some("trans rights now".to_string()),
        attachments: Some(vec![attachment.clone()]),
        ..Default::default()
    };
    let channel = bundle.channel.read().unwrap().clone();
    let vec_attach = vec![attachment.clone()];
    let _arg = Some(&vec_attach);
    bundle.user.send_message(message, channel.id).await.unwrap();
    common::teardown(bundle).await
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn search_messages() {
    /// FIXME: Fix this test for wasm32. wasm32-unknown-unknown does not have a filesystem and therefore cannot read files.
    #[cfg(target_arch = "wasm32")]
    return;
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

    let message = types::MessageSendSchema {
        content: Some("trans rights now".to_string()),
        attachments: Some(vec![attachment.clone()]),
        ..Default::default()
    };
    let channel = bundle.channel.read().unwrap().clone();
    let vec_attach = vec![attachment.clone()];
    let _arg = Some(&vec_attach);
    let message = bundle.user.send_message(message, channel.id).await.unwrap();
    let query = MessageSearchQuery {
        author_id: Some(Vec::from([bundle.user.object.read().unwrap().id])),
        ..Default::default()
    };
    let guild_id = bundle.guild.read().unwrap().id;
    let query_result = Guild::search_messages(guild_id, query, &mut bundle.user)
        .await
        .unwrap();
    assert!(!query_result.is_empty());
    assert_eq!(query_result.get(0).unwrap().id, message.id);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn test_stickies() {
    let mut bundle = common::setup().await;
    let message = types::MessageSendSchema {
        content: Some("A Message!".to_string()),
        ..Default::default()
    };
    let channel = bundle.channel.read().unwrap().clone();
    let message = bundle.user.send_message(message, channel.id).await.unwrap();
    assert_eq!(
        Message::get_sticky(channel.id, &mut bundle.user)
            .await
            .unwrap(),
        Vec::<Message>::new()
    );
    Message::sticky(channel.id, message.id, None, &mut bundle.user)
        .await
        .unwrap();
    assert_eq!(
        Message::get_sticky(channel.id, &mut bundle.user)
            .await
            .unwrap()
            .get(0)
            .unwrap()
            .id,
        message.id
    );
    Message::unsticky(channel.id, message.id, None, &mut bundle.user)
        .await
        .unwrap();
    assert_eq!(
        Message::get_sticky(channel.id, &mut bundle.user)
            .await
            .unwrap(),
        Vec::<Message>::new()
    );
    common::teardown(bundle).await
}
