// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use pubserve::Publisher;

use crate::{
    errors::VoiceGatewayError,
    types::{
        SessionDescription, SessionUpdate, Speaking, SsrcDefinition, VoiceBackendVersion,
        VoiceClientConnectFlags, VoiceClientConnectPlatform, VoiceClientDisconnection,
        VoiceMediaSinkWants, VoiceReady,
    },
};

#[derive(Default, Debug)]
pub struct VoiceEvents {
    pub voice_ready: Publisher<VoiceReady>,
    pub backend_version: Publisher<VoiceBackendVersion>,
    pub session_description: Publisher<SessionDescription>,
    pub session_update: Publisher<SessionUpdate>,
    pub speaking: Publisher<Speaking>,
    pub ssrc_definition: Publisher<SsrcDefinition>,
    pub client_disconnect: Publisher<VoiceClientDisconnection>,
    pub client_connect_flags: Publisher<VoiceClientConnectFlags>,
    pub client_connect_platform: Publisher<VoiceClientConnectPlatform>,
    pub media_sink_wants: Publisher<VoiceMediaSinkWants>,
    pub error: Publisher<VoiceGatewayError>,
}
