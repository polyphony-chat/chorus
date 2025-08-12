// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use chorus::{
    gateway::{GatewayEncoding, GatewayOptions, GatewayTransportCompression},
    instance::{Instance, InstanceBuilder, InstanceSoftware},
    types::IntoShared,
};

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

    // The above way is the easiest to create an instance, but you may want more options
    //
    // To do so, you can use InstanceBuilder:
    let instance = InstanceBuilder::new("https://other-example.com".to_string())
        // Customize how our gateway connections will be made
        .with_gateway_options(GatewayOptions {
            encoding: GatewayEncoding::Json,

            // Explicitly disables Gateway compression, if we want to
            transport_compression: GatewayTransportCompression::None,
        })
        // Skip fetching ratelimits and instance info, we know we our sever doesn't support that
        .skip_optional_requests(true)
        // Skip automatically detecting the software, we know which it is
        .with_software(InstanceSoftware::Other)
        // Once we're ready we call build
        .build()
        .await
        .expect("Failed to connect to the Spacebar server")
        .into_shared();

    let instance_lock = instance.read().unwrap();
    dbg!(&instance_lock.instance_info);
    dbg!(&instance_lock.limits_information);
}
