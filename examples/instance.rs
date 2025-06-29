// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use chorus::{instance::Instance, types::IntoShared};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // This instance will later need to be shared across threads and users, so we'll
    // store it inside of the `Shared` type (note the `into_shared()` method call)
    let instance = Instance::new("https://example.com")
        .await
        .expect("Failed to connect to the Spacebar server")
        .into_shared();

    // You can create as many instances of `Instance` as you want, but each `Instance` should likely be unique.

    // Each time we want to access the underlying `Instance` we need to lock
    // its reference so other threads don't modify the data while we're reading or changing it
    let instance_lock = instance.read().unwrap();

    dbg!(&instance_lock.instance_info);
    dbg!(&instance_lock.limits_information);
}
