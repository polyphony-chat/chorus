<div align="center">

[![Discord]][Discord-invite]
[![Build][build-shield]][build-url]
[![Coverage][coverage-shield]][coverage-url]
<img src="https://img.shields.io/static/v1?label=Status&message=Alpha&color=blue">

</br>
<div align="center">
  <a href="https://github.com/polyphony-chat/chorus">
    <img src="https://github.com/polyphony-chat/branding/blob/main/logos/polyphony-chorus-round-8bit.png?raw=true" alt="The chorus logo. a dark, square background with rounded edges. on this background, there are three vertically stacked, purple lines. The lines each resemble a sine curve." width="128" height="128">
  </a>

<h3 align="center">Chorus</h3>

  <p align="center">
    <br />
    <a href="https://docs.rs/chorus/latest/chorus/"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/polyphony-chat/chorus/issues">Report Bug</a>
    ·
    <a href="https://crates.io/crates/chorus">crates.io</a>
    ·
    <a href="https://discord.gg/8tKSC8wzDq">Join Discord</a>
  </p>
</div>

</div>

Chorus is a Rust library which poses as an API wrapper for [Spacebar Chat](https://github.com/spacebarchat/),
Discord and our own Polyphony. Its high-level API is designed to be easy to use, while still providing the
flexibility one would expect from a library like this.

You can establish as many connections to as many servers as you want, and you can use them all at the same time.

## A Tour of Chorus

Chorus combines all the required functionalities of an API wrapper for chat services into one modular library.
The library handles various aspects on your behalf, such as rate limiting, authentication and maintaining
a WebSocket connection to the Gateway. This means that you can focus on building your application,
instead of worrying about the underlying implementation details.

To get started with Chorus, import it into your project by adding the following to your `Cargo.toml` file:

```toml
[dependencies]
chorus = "0.20.0"
```

### Establishing a Connection

To connect to a Polyphony/Spacebar compatible server, you'll need to create an [`Instance`](https://docs.rs/chorus/latest/chorus/instance/struct.Instance.html) like this:

```rust
use chorus::{instance::Instance, types::IntoShared};

#[tokio::main]
async fn main() {

    // This instance will later need to be shared across threads and users, so we'll
    // store it inside of the `Shared` type (note the `into_shared()` method call)
    let instance = Instance::new("https://example.com", None)
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
```

This Instance can now be used to log in, register and from there on, interact with the server in all sorts of ways.

### Logging In

Logging in correctly provides you with an instance of `ChorusUser`, with which you can interact with the server and
manipulate the account. Assuming you already have an account on the server, you can log in like this:

```rust
use chorus::types::LoginSchema;
// Assume, you already have an account created on this instance. Registering an account works
// the same way, but you'd use the Register-specific Structs and methods instead.
let login_schema = LoginSchema {
    login: "user@example.com".to_string(),
    password: "Correct-Horse-Battery-Staple".to_string(),
    ..Default::default()
};
// Each user connects to the Gateway. Each users' Gateway connection lives on a separate thread. Depending on
// the runtime feature you choose, this can potentially take advantage of all of your computers' threads.
//
// Note that we clone the reference to the instance here, not the instance itself
// (we do this because each user needs its own access to the instance's data)
let user = Instance::login_account(instance.clone(), login_schema)
    .await
    .expect("An error occurred during the login process");
dbg!(user.belongs_to);
dbg!(&user.object.read().unwrap().username);
```

## Supported Platforms

All major desktop operating systems (Windows, macOS (aarch64/x86_64), Linux (aarch64/x86_64)) are supported.
`wasm32-unknown-unknown` is a supported compilation target on versions `0.12.0` and up. This allows you to use
Chorus in your browser, or in any other environment that supports WebAssembly.

To compile for `wasm32-unknown-unknown`, execute the following command:

```sh
cargo build --target=wasm32-unknown-unknown --no-default-features
```

The following features are supported on `wasm32-unknown-unknown`:

| Feature           | WASM Support |
| ----------------- | ------------ |
| `client`          | ✅            |
| `rt`              | ✅            |
| `rt-multi-thread` | ❌            |
| `backend`         | ❌            |
| `voice`           | ❌            |
| `voice_udp`       | ❌            |
| `voice_gateway`   | ✅            |

We recommend checking out the "examples" directory, as well as the documentation for more information.

## MSRV (Minimum Supported Rust Version)

Rust **1.81.0**. This number might change at any point while Chorus is not yet at version 1.0.0.

## Development Setup

Make sure that you have at least Rust 1.81.0 installed. You can check your Rust version by running `cargo --version`
in your terminal. To compile for `wasm32-unknown-unknown`, you need to install the `wasm32-unknown-unknown` target.
You can do this by running `rustup target add wasm32-unknown-unknown`.

### Testing

In general, the tests will require you to run a local instance of the Spacebar server. You can find instructions on how
to do that [here](https://docs.spacebar.chat/setup/server/). You can find a pre-configured version of the server
[here](https://github.com/bitfl0wer/server). It is recommended to use the pre-configured version, as certain things
like "proxy connection checking" are already disabled on this version, which otherwise might break tests.

### wasm

To test for wasm, you will need to `cargo install wasm-pack`. You can then run
`wasm-pack test --<chrome/firefox/safari> --headless -- --target wasm32-unknown-unknown --features="rt, client, voice_gateway" --no-default-features`
to run the tests for wasm.

## Versioning

Like other cargo crates, this crate uses Semantic Versioning 2.0.0 as its versioning scheme.
You can read the specification [here](https://semver.org/spec/v2.0.0.html).

Code gated behind the `backend` feature is not considered part of the public API and can change without
affecting semver compatibility. The `backend` feature is explicitly meant for use in [`symfonia`](https://github.com/polyphony-chat/symfonia)

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md).

[Rust]: https://img.shields.io/badge/Rust-orange?style=plastic&logo=rust
[Rust-url]: https://www.rust-lang.org/
[build-shield]: https://img.shields.io/github/actions/workflow/status/polyphony-chat/chorus/build_and_test.yml?style=flat
[build-url]: https://github.com/polyphony-chat/chorus/blob/main/.github/workflows/build_and_test.yml
[clippy-shield]: https://img.shields.io/github/actions/workflow/status/polyphony-chat/chorus/clippy.yml?style=flat
[clippy-url]: https://github.com/polyphony-chat/chorus/blob/main/.github/workflows/clippy.yml
[contributors-shield]: https://img.shields.io/github/contributors/polyphony-chat/chorus.svg?style=flat
[contributors-url]: https://github.com/polyphony-chat/chorus/graphs/contributors
[coverage-shield]: https://coveralls.io/repos/github/polyphony-chat/chorus/badge.svg?branch=main
[coverage-url]: https://coveralls.io/github/polyphony-chat/chorus?branch=main
[forks-shield]: https://img.shields.io/github/forks/polyphony-chat/chorus.svg?style=flat
[forks-url]: https://github.com/polyphony-chat/chorus/network/members
[stars-shield]: https://img.shields.io/github/stars/polyphony-chat/chorus.svg?style=flat
[stars-url]: https://github.com/polyphony-chat/chorus/stargazers
[issues-shield]: https://img.shields.io/github/issues/polyphony-chat/chorus.svg?style=flat
[issues-url]: https://github.com/polyphony-chat/chorus/issues
[license-shield]: https://img.shields.io/github/license/polyphony-chat/chorus.svg?style=f;at
[license-url]: https://github.com/polyphony-chat/chorus/blob/master/LICENSE
[Discord]: https://dcbadge.vercel.app/api/server/m3FpcapGDD?style=flat
[Discord-invite]: https://discord.com/invite/m3FpcapGDD
