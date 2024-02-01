// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use chorus::types::{PublicUser, Snowflake, User};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn to_public_user() {
    let mut user = User::default();
    let mut public_user = PublicUser {
        username: Some("".to_string()),
        discriminator: Some("".to_string()),
        ..Default::default()
    };
    let id: Snowflake = 1_u64.into();
    user.id = id;
    public_user.id = id;

    let from_user = user.into_public_user();
    assert_eq!(public_user, from_user);
}
