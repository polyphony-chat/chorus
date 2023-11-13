<div align="center">

[![Discord]][Discord-invite]
[![Build][build-shield]][build-url]
[![Coverage][coverage-shield]][coverage-url]
[![Contributors][contributors-shield]][contributors-url]
<img src="https://img.shields.io/static/v1?label=Status&message=Alpha&color=blue">

</br>
<div align="center">
  <a href="https://github.com/polyphony-chat/chorus">
    <img src="https://raw.githubusercontent.com/polyphony-chat/design/main/branding/polyphony-chorus-round-8bit.png" alt="Logo" width="128" height="128">
  </a>

<h3 align="center">Chorus</h3>

  <p align="center">
    <br />
    <a href="https://github.com/polyphony-chat/chorus"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/polyphony-chat/chorus/issues">Report Bug</a>
    ·
    <a href="https://github.com/polyphony-chat/chorus/issues">Request Feature</a>
    ·
    <a href="https://discord.gg/8tKSC8wzDq">Join Discord</a>
  </p>
</div>

</div>

Chorus is a Rust library that allows developers to interact with multiple Spacebar-compatible APIs and Gateways (Including
Discord.com) simultaneously. The library provides a simple and efficient way to communicate with these services, making it easier for developers to build applications that rely on them. Chorus is open-source and welcomes contributions from the community. 

## A Tour of Chorus

Chorus combines all the required functionalities of a user-centric Spacebar library into one package. The library
handles a lot of things for you, such as rate limiting, authentication, and more. This means that you can focus on
building your application, instead of worrying about the underlying implementation details.

To get started with Chorus, import it into your project by adding the following to your `Cargo.toml` file:

```toml
[dependencies]
chorus = "0"
```

### Establishing a Connection

To connect to a Spacebar compatible server, you need to create an [`Instance`](https://docs.rs/chorus/latest/chorus/instance/struct.Instance.html) like this:

```rs
use chorus::instance::Instance;
use chorus::UrlBundle;

#[tokio::main]
async fn main() {
    let bundle = UrlBundle::new(
        "https://example.com/api".to_string(),
        "wss://example.com/".to_string(),
        "https://example.com/cdn".to_string(),
    );
    let instance = Instance::new(bundle, true)
        .await
        .expect("Failed to connect to the Spacebar server");
    // You can create as many instances of `Instance` as you want, but each `Instance` should likely be unique.
    dbg!(instance.instance_info);
    dbg!(instance.limits_information);
}
```

This Instance can now be used to log in, register and from there on, interact with the server in all sorts of ways.

### Logging In

Logging in correctly provides you with an instance of `ChorusUser`, with which you can interact with the server and
manipulate the account. Assuming you already have an account on the server, you can log in like this:

```rs
use chorus::types::LoginSchema;
// Assume, you already have an account created on this instance. Registering an account works
// the same way, but you'd use the Register-specific Structs and methods instead.
let login_schema = LoginSchema {
    login: "user@example.com".to_string(),
    password: "Correct-Horse-Battery-Staple".to_string(),
    ..Default::default()
};
// Each user connects to the Gateway. The Gateway connection lives on a seperate thread. Depending on
// the runtime feature you choose, this can potentially take advantage of all of your computers' threads.
let user = instance
    .login_account(login_schema)
    .await
    .expect("An error occurred during the login process");
dbg!(user.belongs_to);
dbg!(&user.object.read().unwrap().username);
```

## Supported Platforms

All major desktop operating systems (Windows, macOS (aarch64/x86_64), Linux (aarch64/x86_64)) are supported.
We are currently working on adding full support for `wasm32-unknown-unknown`. This will allow you to use Chorus in
your browser, or in any other environment that supports WebAssembly.

We recommend checking out the examples directory, as well as the documentation for more information.

## MSRV (Minimum Supported Rust Version)

Rust **1.67.1**. This number might change at any point while Chorus is not yet at version 1.0.0.

## Versioning

This crate uses Semantic Versioning 2.0.0 as its versioning scheme. You can read the specification [here](https://semver.org/spec/v2.0.0.html).

## Contributing

Chorus is currently missing voice support and a lot of API endpoints, many of which should be trivial to implement,
ever since [we streamlined the process of doing so](https://github.com/polyphony-chat/chorus/discussions/401).

If you'd like to contribute new functionality, check out [The 'Meta'-issues.](https://github.com/polyphony-chat/chorus/issues?q=is%3Aissue+label%3A%22Type%3A+Meta%22+) They contain a comprehensive list of all features which are yet missing for full Discord.com compatibility.
Please feel free to open an Issue with the idea you have, or a Pull Request. Please keep our [contribution guidelines](https://github.com/polyphony-chat/.github/blob/main/CONTRIBUTION_GUIDELINES.md) in mind. Your contribution might not be accepted if it violates these guidelines or [our Code of Conduct](https://github.com/polyphony-chat/.github/blob/main/CODE_OF_CONDUCT.md).

<details>
  <summary>Progress Tracker/Roadmap</summary>

  ### Core Functionality
  - [x] Rate Limiter (hint: couldn't be fully tested due to [an Issue with the Spacebar Server](https://github.com/spacebarchat/server/issues/1022))
  - [x] [Login (the conventional way)](https://github.com/polyphony-chat/chorus/issues/1)
  - [ ] [2FA](https://github.com/polyphony-chat/chorus/issues/40)
  - [x] [Registration](https://github.com/polyphony-chat/chorus/issues/1)

  ### Messaging
  - [x] [Sending messages](https://github.com/polyphony-chat/chorus/issues/23)
  - [x] [Events (Message, User, Channel, etc.)](https://github.com/polyphony-chat/chorus/issues/51)
  - [x] Channel creation
  - [x] Channel deletion
  - [x] [Channel management (name, description, icon, etc.)](https://github.com/polyphony-chat/chorus/issues/48)
  - [x] [Join and Leave Guilds](https://github.com/polyphony-chat/chorus/issues/45)
  - [x] [Start DMs](https://github.com/polyphony-chat/chorus/issues/45)
  - [x] [Group DM creation, deletion and member management](https://github.com/polyphony-chat/chorus/issues/89)
  - [ ] [Deleting messages](https://github.com/polyphony-chat/chorus/issues/91)
  - [ ] [Message threads](https://github.com/polyphony-chat/chorus/issues/90)
  - [x] [Reactions](https://github.com/polyphony-chat/chorus/issues/85)
  - [ ] Message Search
  - [ ] Message history
  - [ ] Emoji
  - [ ] Stickers
  - [ ] [Forum channels](https://github.com/polyphony-chat/chorus/issues/90)

  ### User Management
  - [ ] [User profile customization](https://github.com/polyphony-chat/chorus/issues/41)
  - [x] Gettings users and user profiles
  - [x] [Friend requests](https://github.com/polyphony-chat/chorus/issues/92)
  - [x] [Blocking users](https://github.com/polyphony-chat/chorus/issues/92)
  - [ ] User presence (online, offline, idle, etc.)
  - [ ] User status (custom status, etc.)
  - [x] Account deletion

  ### Additional Features
  - [ ] Server discovery
  - [ ] Server templates

  ### Voice and Video
  - [ ] [Voice chat support](https://github.com/polyphony-chat/chorus/issues/49)
  - [ ] [Video chat support](https://github.com/polyphony-chat/chorus/issues/49)

  ### Permissions and Roles
  - [x] [Role management](https://github.com/polyphony-chat/chorus/issues/46) (creation, deletion, modification)
  - [x] [Permission management](https://github.com/polyphony-chat/chorus/issues/46) (assigning and revoking permissions)
  - [x] [Channel-specific permissions](https://github.com/polyphony-chat/chorus/issues/88)
  - [x] Role-based access control

  ### Guild Management
  - [x] Guild creation
  - [x] Guild deletion
  - [ ] [Guild settings (name, description, icon, etc.)](https://github.com/polyphony-chat/chorus/issues/43)
  - [ ] Guild invites

  ### Moderation
  - [ ] Channel moderation (slow mode, etc.)
  - [ ] User sanctions (mute, kick, ban)
  - [ ] Audit logs

  ### Embeds and Rich Content
  - [x] Sending rich content in messages (links, images, videos)
  - [ ] Customizing embed appearance (title, description, color, fields)

  ### Webhooks
  - [ ] Webhook creation and management
  - [ ] Handling incoming webhook events

  ### Documentation and Examples
  - [ ] Comprehensive documentation
  - [ ] Example usage and code snippets
  - [ ] Tutorials and guides

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
</details>
