use std::ffi::OsString;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SentryConfiguration {
    pub enabled: bool,
    pub endpoint: String,
    pub trace_sample_rate: f64,
    pub environment: String,
}

impl SentryConfiguration {
    #[cfg(not(target_arch = "wasm32"))]
    fn get_hostname() -> std::io::Result<OsString> {
        hostname::get()
    }
    #[cfg(target_arch = "wasm32")]
    fn get_hostname() -> std::io::Result<OsString> {
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "Unsupported: wasm targets do not have a hostname",
        ))
    }
}

impl Default for SentryConfiguration {
    fn default() -> Self {
        Self {
            enabled: false,
            endpoint: String::from(
                "https://241c6fb08adb469da1bb82522b25c99f@sentry.quartzinc.space/3",
            ),
            trace_sample_rate: 1.0,
            environment: SentryConfiguration::get_hostname()
                .unwrap_or_else(|_| OsString::new())
                .to_string_lossy()
                .to_string(),
        }
    }
}
