use chrono::{DateTime, Utc};

/// An AWS account visible to the SSO user.
#[derive(Debug, Clone)]
pub struct SsoAccount {
    pub account_id: String,
    pub account_name: String,
    pub email_address: String,
}

/// A role available in an SSO account.
#[derive(Debug, Clone)]
pub struct SsoAccountRole {
    pub account_id: String,
    pub role_name: String,
}

/// Temporary credentials returned by GetRoleCredentials.
#[derive(Debug, Clone)]
pub struct SsoRoleCredentials {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub session_token: String,
    pub expiration: i64,
}

/// An SSO access token session.
#[derive(Debug, Clone)]
pub struct SsoSession {
    pub access_token: String,
    pub logged_out: bool,
    pub created_at: DateTime<Utc>,
}
