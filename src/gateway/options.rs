// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::instance::InstanceSoftware;

#[derive(Clone, PartialEq, Eq, Ord, PartialOrd, Debug, Default, Copy)]
/// Options passed when initializing the gateway connection.
///
/// E.g. compression
///
/// # Note
///
/// Discord allows specifying the api version (v10, v9, ...) as well, but chorus is built upon one
/// main version (v9).
///
/// Similarly, discord also supports etf encoding, while chorus does not (yet).
/// We are looking into supporting it as an option, since it is faster and more lightweight.
///
/// See <https://docs.discord.sex/topics/gateway#connections>
pub struct GatewayOptions {
    pub encoding: GatewayEncoding,
    pub transport_compression: GatewayTransportCompression,
}

impl GatewayOptions {
    /// Creates the ideal gateway options for an [InstanceSoftware],
    /// based off which features it supports.
    pub fn for_instance_software(software: InstanceSoftware) -> GatewayOptions {
        // TODO: Support ETF
        let encoding = GatewayEncoding::Json;

        let transport_compression = match software.supports_gateway_zlib() {
            true => GatewayTransportCompression::ZLibStream,
            false => GatewayTransportCompression::None,
        };

        GatewayOptions {
            encoding,
            transport_compression,
        }
    }

    /// Adds the options to an existing gateway url
    ///
    /// Returns the new url
    pub(crate) fn add_to_url(&self, url: &str) -> String {
        let mut url = url.to_string();

        let mut parameters = Vec::with_capacity(2);

        let encoding = self.encoding.to_url_parameter();
        parameters.push(encoding);

        let compression = self.transport_compression.to_url_parameter();
        if let Some(some_compression) = compression {
            parameters.push(some_compression);
        }

        let mut has_parameters = url.contains('?') && url.contains('=');

        if !has_parameters {
            // Insure it ends in a /, so we don't get a 400 error
            if !url.ends_with('/') {
                url.push('/');
            }

            // Lets hope that if it already has parameters the person knew to add '/'
        }

        for parameter in parameters {
            if !has_parameters {
                url = format!("{}?{}", url, parameter);
                has_parameters = true;
            } else {
                url = format!("{}&{}", url, parameter);
            }
        }

        url
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Debug, Default)]
/// Possible transport compression options for the gateway.
///
/// See <https://docs.discord.sex/topics/gateway#transport-compression>
pub enum GatewayTransportCompression {
    /// Do not transport compress packets
    None,
    /// Transport compress using zlib stream
    #[default]
    ZLibStream,
}

impl GatewayTransportCompression {
    /// Returns the option as a url parameter.
    ///
    /// If set to [GatewayTransportCompression::None] returns [None].
    ///
    /// If set to anything else, returns a string like "compress=zlib-stream"
    pub(crate) fn to_url_parameter(self) -> Option<String> {
        match self {
            Self::None => None,
            Self::ZLibStream => Some(String::from("compress=zlib-stream")),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Debug, Default)]
/// See <https://docs.discord.sex/topics/gateway#encoding-and-compression>
pub enum GatewayEncoding {
    /// Javascript object notation, a standard for websocket connections,
    /// but contains a lot of overhead
    #[default]
    Json,
    /// A binary format originating from Erlang
    ///
    /// Should be lighter and faster than json.
    ///
    /// !! Chorus does not implement ETF yet !!
    ETF,
}

impl GatewayEncoding {
    /// Returns the option as a url parameter.
    ///
    /// Returns a string like "encoding=json"
    pub(crate) fn to_url_parameter(self) -> String {
        match self {
            Self::Json => String::from("encoding=json"),
            Self::ETF => String::from("encoding=etf"),
        }
    }
}
