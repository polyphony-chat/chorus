use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct MfaToken {
    pub token: String,
    pub expires_at: DateTime<Utc>,
}
