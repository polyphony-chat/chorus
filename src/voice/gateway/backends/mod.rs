// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[cfg(all(not(target_arch = "wasm32"), feature = "voice_gateway"))]
pub mod tungstenite;

#[cfg(all(target_arch = "wasm32", feature = "voice_gateway"))]
pub mod wasm;
