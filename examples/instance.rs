// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use chorus::instance::Instance;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let instance = Instance::new("https://example.com/")
        .await
        .expect("Failed to connect to the Spacebar server");
    dbg!(instance.instance_info);
    dbg!(instance.limits_information);
}
