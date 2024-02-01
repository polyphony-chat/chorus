// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{
    errors::VoiceGatewayError,
    gateway::GatewayEvent,
    types::{
        SessionDescription, SessionUpdate, Speaking, SsrcDefinition, VoiceBackendVersion,
        VoiceClientConnectFlags, VoiceClientConnectPlatform, VoiceClientDisconnection,
        VoiceMediaSinkWants, VoiceReady,
    },
};

#[derive(Default, Debug)]
pub struct VoiceEvents {
    pub voice_ready: GatewayEvent<VoiceReady>,
    pub backend_version: GatewayEvent<VoiceBackendVersion>,
    pub session_description: GatewayEvent<SessionDescription>,
    pub session_update: GatewayEvent<SessionUpdate>,
    pub speaking: GatewayEvent<Speaking>,
    pub ssrc_definition: GatewayEvent<SsrcDefinition>,
    pub client_disconnect: GatewayEvent<VoiceClientDisconnection>,
    pub client_connect_flags: GatewayEvent<VoiceClientConnectFlags>,
    pub client_connect_platform: GatewayEvent<VoiceClientConnectPlatform>,
    pub media_sink_wants: GatewayEvent<VoiceMediaSinkWants>,
    pub error: GatewayEvent<VoiceGatewayError>,
}
