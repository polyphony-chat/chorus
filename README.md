<div align="center">

[![Discord]][Discord-invite]
[![Build][build-shield]][build-url]
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Issues][issues-shield]][issues-url]

</br>
<div align="center">
  <a href="https://github.com/polyphony-chat/chorus">
    <img src="https://raw.githubusercontent.com/polyphony-chat/design/main/branding/polyphony-chorus-round-8bit.png" alt="Logo" width="128" height="128">
  </a>

<h3 align="center">Chorus</h3>

  <p align="center">
    A rust library for interacting with (multiple) Spacebar-compatible APIs and Gateways (at the same time).
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

## Roadmap
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
- [ ] [Join and Leave Guilds](https://github.com/polyphony-chat/chorus/issues/45)
- [ ] [Start DMs](https://github.com/polyphony-chat/chorus/issues/45)
- [ ] [Group DM creation, deletion and member management](https://github.com/polyphony-chat/chorus/issues/89)
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
- [ ] [Friend requests](https://github.com/polyphony-chat/chorus/issues/92)
- [ ] [Blocking users](https://github.com/polyphony-chat/chorus/issues/92)
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
- [ ] [Role management](https://github.com/polyphony-chat/chorus/issues/46) (creation, deletion, modification)
- [ ] [Permission management](https://github.com/polyphony-chat/chorus/issues/46) (assigning and revoking permissions)
- [ ] [Channel-specific permissions](https://github.com/polyphony-chat/chorus/issues/88)
- [ ] Role-based access control

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

### Notifications and Push Notifications
- [ ] Notification settings management

### Webhooks
- [ ] Webhook creation and management
- [ ] Handling incoming webhook events

### Documentation and Examples
- [ ] Comprehensive documentation
- [ ] Example usage and code snippets
- [ ] Tutorials and guides

[Rust]: https://img.shields.io/badge/Rust-orange?style=plastic&logo=rust
[Rust-url]: https://www.rust-lang.org/
[build-shield]: https://img.shields.io/github/actions/workflow/status/polyphony-chat/chorus/rust.yml?style=flat
[build-url]: https://github.com/polyphony-chat/chorus/blob/main/.github/workflows/rust.yml
[contributors-shield]: https://img.shields.io/github/contributors/polyphony-chat/chorus.svg?style=flat
[contributors-url]: https://github.com/polyphony-chat/chorus/graphs/contributors
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
