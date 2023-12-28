use crate::{errors::VoiceGatewayError, types::VoiceGatewayReceivePayload};

/// Represents a messsage received from the webrtc socket. This will be either a [GatewayReceivePayload], containing webrtc events, or a [WebrtcError].
/// This struct is used internally when handling messages.
#[derive(Clone, Debug)]
pub struct VoiceGatewayMessage(pub String);

impl VoiceGatewayMessage {
    /// Parses the message as an error;
    /// Returns the error if succesfully parsed, None if the message isn't an error
    pub fn error(&self) -> Option<VoiceGatewayError> {
        // Some error strings have dots on the end, which we don't care about
        let processed_content = self.0.to_lowercase().replace('.', "");

        match processed_content.as_str() {
            "unknown opcode" | "4001" => Some(VoiceGatewayError::UnknownOpcode),
            "decode error" | "failed to decode payload" | "4002" => {
                Some(VoiceGatewayError::FailedToDecodePayload)
            }
            "not authenticated" | "4003" => Some(VoiceGatewayError::NotAuthenticated),
            "authentication failed" | "4004" => Some(VoiceGatewayError::AuthenticationFailed),
            "already authenticated" | "4005" => Some(VoiceGatewayError::AlreadyAuthenticated),
            "session is no longer valid" | "4006" => Some(VoiceGatewayError::SessionNoLongerValid),
            "session timeout" | "4009" => Some(VoiceGatewayError::SessionTimeout),
            "server not found" | "4011" => Some(VoiceGatewayError::ServerNotFound),
            "unknown protocol" | "4012" => Some(VoiceGatewayError::UnknownProtocol),
            "disconnected" | "4014" => Some(VoiceGatewayError::Disconnected),
            "voice server crashed" | "4015" => Some(VoiceGatewayError::VoiceServerCrashed),
            "unknown encryption mode" | "4016" => Some(VoiceGatewayError::UnknownEncryptionMode),
            _ => None,
        }
    }

    /// Parses the message as a payload;
    /// Returns a result of deserializing
    pub fn payload(&self) -> Result<VoiceGatewayReceivePayload, serde_json::Error> {
        serde_json::from_str(&self.0)
    }
}
