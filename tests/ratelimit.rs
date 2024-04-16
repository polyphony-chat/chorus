// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use chorus::ratelimiter::ChorusRequest;

mod common;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
async fn get_limit_config() {
    let conf = ChorusRequest::get_limits_config("http://localhost:3001/api")
        .await
        .unwrap();
    assert!(conf.channel.max_pins > 0);
    assert!(conf.channel.max_topic > 0);
    assert!(conf.channel.max_webhooks > 0);
    assert!(conf.guild.max_roles > 0);
    assert!(conf.guild.max_channels > 0);
    assert!(conf.guild.max_emojis > 0);
    assert!(conf.guild.max_channels_in_category > 0);
    assert!(conf.guild.max_members > 0);
    assert!(conf.message.max_attachment_size > 0);
    assert!(conf.message.max_bulk_delete > 0);
    assert!(conf.message.max_reactions > 0);
    assert!(conf.message.max_characters > 0);
    assert!(conf.message.max_tts_characters == 0);
    assert!(conf.user.max_guilds > 0);
    assert!(conf.user.max_friends > 0);
}
