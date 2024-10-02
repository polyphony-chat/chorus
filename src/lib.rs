// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/*!
Chorus is a Rust library which poses as an API wrapper for [Spacebar Chat](https://github.com/spacebarchat/),
Discord and our own Polyphony. Its high-level API is designed to be easy to use, while still providing the
flexibility one would expect from a library like this.

You can establish as many connections to as many servers as you want, and you can use them all at the same time.

## A Tour of Chorus

Chorus combines all the required functionalities of an API wrapper for chat services into one modular library.
The library handles various aspects on your behalf, such as rate limiting, authentication and maintaining
a WebSocket connection to the Gateway. This means that you can focus on building your application,
instead of worrying about the underlying implementation details.

### Establishing a Connection

To connect to a Polyphony/Spacebar compatible server, you'll need to create an [`Instance`](https://docs.rs/chorus/latest/chorus/instance/struct.Instance.html) like this:

```rs
use chorus::instance::Instance;

#[tokio::main]
async fn main() {
    let instance = Instance::new("https://example.com")
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
// Each user connects to the Gateway. Each users' Gateway connection lives on a separate thread. Depending on
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

Rust **1.70.0**. This number might change at any point while Chorus is not yet at version 1.0.0.

## Development Setup

Make sure that you have at least Rust 1.70.0 installed. You can check your Rust version by running `cargo --version`
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

This crate uses Semantic Versioning 2.0.0 as its versioning scheme. You can read the specification [here](https://semver.org/spec/v2.0.0.html).

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md).
!*/
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/polyphony-chat/design/main/branding/polyphony-chorus-round-8bit.png"
)]
#![allow(clippy::module_inception)]
#![deny(
    clippy::extra_unused_lifetimes,
    clippy::from_over_into,
    clippy::needless_borrow,
    clippy::new_without_default
)]
#![warn(
    clippy::todo,
    clippy::unimplemented,
    clippy::dbg_macro,
    clippy::print_stdout,
    clippy::print_stderr,
    missing_debug_implementations,
    missing_copy_implementations,
    clippy::useless_conversion
)]
#[cfg(all(feature = "rt", feature = "rt_multi_thread"))]
compile_error!("feature \"rt\" and feature \"rt_multi_thread\" cannot be enabled at the same time");

use errors::ChorusResult;
use serde::{Deserialize, Serialize};
use types::types::domains_configuration::WellKnownResponse;
use url::{ParseError, Url};

use crate::errors::ChorusError;

#[cfg(feature = "client")]
pub mod api;
pub mod errors;
#[cfg(feature = "client")]
pub mod gateway;
#[cfg(feature = "client")]
pub mod instance;
#[cfg(feature = "client")]
pub mod ratelimiter;
pub mod types;
#[cfg(all(
    feature = "client",
    any(feature = "voice_udp", feature = "voice_gateway")
))]
pub mod voice;

#[cfg(not(feature = "sqlx"))]
pub type UInt128 = u128;
#[cfg(feature = "sqlx")]
pub type UInt128 = sqlx_pg_uint::PgU128;
#[cfg(not(feature = "sqlx"))]
pub type UInt64 = u64;
#[cfg(feature = "sqlx")]
pub type UInt64 = sqlx_pg_uint::PgU64;
#[cfg(not(feature = "sqlx"))]
pub type UInt32 = u32;
#[cfg(feature = "sqlx")]
pub type UInt32 = sqlx_pg_uint::PgU32;
#[cfg(not(feature = "sqlx"))]
pub type UInt16 = u16;
#[cfg(feature = "sqlx")]
pub type UInt16 = sqlx_pg_uint::PgU16;
#[cfg(not(feature = "sqlx"))]
pub type UInt8 = u8;
#[cfg(feature = "sqlx")]
pub type UInt8 = sqlx_pg_uint::PgU8;

#[derive(Clone, Default, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// A URLBundle bundles together the API-, Gateway- and CDN-URLs of a Spacebar instance.
///
/// # Notes
/// All the urls can be found on the /api/policies/instance/domains endpoint of a spacebar server
pub struct UrlBundle {
    /// The root url of an Instance. Usually, this would be the url where `.well-known/spacebar` can
    /// be located under. If the instance you are connecting to for some reason does not have a
    /// `.well-known` set up (for example, if it is a local/testing instance), you can use the api
    /// url as a substitute.
    /// Ex: `https://spacebar.chat`
    pub root: String,
    /// The api's url.
    /// Ex: `https://old.server.spacebar.chat/api`
    pub api: String,
    /// The gateway websocket url.
    /// Note that because this is a websocket url, it will always start with `wss://` or `ws://`
    /// Ex: `wss://gateway.old.server.spacebar.chat`
    pub wss: String,
    /// The CDN's url.
    /// Ex: `https://cdn.old.server.spacebar.chat`
    pub cdn: String,
}

impl UrlBundle {
    /// Creates a new UrlBundle from the relevant urls.
    pub fn new(root: &str, api: &str, wss: &str, cdn: &str) -> Self {
        Self {
            root: UrlBundle::parse_url(root),
            api: UrlBundle::parse_url(api),
            wss: UrlBundle::parse_url(wss),
            cdn: UrlBundle::parse_url(cdn),
        }
    }

    /// Parses a URL using the Url library and formats it in a standardized way.
    /// If no protocol is given, HTTP (not HTTPS) is assumed.
    ///
    /// # Examples:
    /// ```rs
    /// let url = parse_url("localhost:3000");
    /// ```
    /// `-> Outputs "http://localhost:3000".`
    pub fn parse_url(url: &str) -> String {
        let url = match Url::parse(url) {
            Ok(url) => {
                if url.scheme() == "localhost" {
                    return UrlBundle::parse_url(&format!("http://{}", url));
                }
                url
            }
            Err(ParseError::RelativeUrlWithoutBase) => {
                let url_fmt = format!("http://{}", url);
                return UrlBundle::parse_url(&url_fmt);
            }
            Err(_) => panic!("Invalid URL"), // TODO: should not panic here
        };
        // if the last character of the string is a slash, remove it.
        let mut url_string = url.to_string();
        if url_string.ends_with('/') {
            url_string.pop();
        }
        url_string
    }

    /// Performs a few HTTP requests to try and retrieve a `UrlBundle` from an instances' root url.
    /// The method tries to retrieve the `UrlBundle` via these three strategies, in order:
    /// - GET: `$url/.well-known/spacebar` -> Retrieve UrlBundle via `$wellknownurl/api/policies/instance/domains`
    /// - GET: `$url/api/policies/instance/domains`
    /// - GET: `$url/policies/instance/domains`
    ///
    /// The URL stored at `.well-known/spacebar` is the instances' API endpoint. The API
    /// stores the CDN and WSS URLs under the `$api/policies/instance/domains` endpoint. If all three
    /// of the above approaches fail, it is very likely that the instance is misconfigured, unreachable, or that
    /// a wrong URL was provided.
    pub async fn from_root_url(url: &str) -> ChorusResult<UrlBundle> {
        let parsed = UrlBundle::parse_url(url);
        let client = reqwest::Client::new();
        let request_wellknown = client
            .get(format!("{}/.well-known/spacebar", &parsed))
            .header(http::header::ACCEPT, "application/json")
            .build()?;
        let response_wellknown = client.execute(request_wellknown).await?;
        if response_wellknown.status().is_success() {
            let body = response_wellknown.json::<WellKnownResponse>().await?.api;
            UrlBundle::from_api_url(&body).await
        } else {
            if let Ok(response_slash_api) =
                UrlBundle::from_api_url(&format!("{}/api/policies/instance/domains", parsed)).await
            {
                return Ok(response_slash_api);
            }
            if let Ok(response_api) =
                UrlBundle::from_api_url(&format!("{}/policies/instance/domains", parsed)).await
            {
                Ok(response_api)
            } else {
                Err(ChorusError::RequestFailed { url: parsed.to_string(), error: "Could not retrieve UrlBundle from url after trying 3 different approaches. Check the provided Url and make sure the instance is reachable.".to_string() } )
            }
        }
    }

    async fn from_api_url(url: &str) -> ChorusResult<UrlBundle> {
        let client = reqwest::Client::new();
        let request = client
            .get(url)
            .header(http::header::ACCEPT, "application/json")
            .build()?;
        let response = client.execute(request).await?;
        if let Ok(body) = response
            .json::<types::types::domains_configuration::Domains>()
            .await
        {
            Ok(UrlBundle::new(
                url,
                &body.api_endpoint,
                &body.gateway,
                &body.cdn,
            ))
        } else {
            Err(ChorusError::RequestFailed {
                url: url.to_string(),
                error: "Could not retrieve a UrlBundle from the given url. Check the provided url and make sure the instance is reachable.".to_string(),
            })
        }
    }
}

#[cfg(test)]
mod lib {
    use super::*;

    #[test]
    fn test_parse_url() {
        let mut result = UrlBundle::parse_url("localhost:3000/");
        assert_eq!(result, "http://localhost:3000");
        result = UrlBundle::parse_url("https://some.url.com/");
        assert_eq!(result, String::from("https://some.url.com"));
        result = UrlBundle::parse_url("https://some.url.com/");
        assert_eq!(result, "https://some.url.com");
        result = UrlBundle::parse_url("https://some.url.com");
        assert_eq!(result, "https://some.url.com");
    }
}
