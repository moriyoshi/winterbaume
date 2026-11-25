use chrono::{DateTime, Utc};

/// Represents an assumed role with temporary credentials.
#[derive(Debug, Clone)]
pub struct AssumedRole {
    pub account_id: String,
    pub role_arn: String,
    pub role_session_name: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub session_token: String,
    pub expiration: DateTime<Utc>,
    pub assumed_role_id: String,
}

impl AssumedRole {
    /// The ARN of the assumed role session.
    /// Format: arn:aws:sts::{account}:assumed-role/{role-name}/{session-name}
    pub fn arn(&self) -> String {
        // Extract role name from role ARN (arn:aws:iam::{account}:role/{name})
        let role_name = self.role_arn.rsplit('/').next().unwrap_or("unknown");
        format!(
            "arn:aws:sts::{}:assumed-role/{}/{}",
            self.account_id, role_name, self.role_session_name
        )
    }

    /// The user ID for the assumed role.
    /// Format: {assumed_role_id}:{session_name}
    pub fn user_id(&self) -> String {
        format!("{}:{}", self.assumed_role_id, self.role_session_name)
    }
}

/// Represents a session token (from GetSessionToken).
#[derive(Debug, Clone)]
pub struct SessionToken {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub session_token: String,
    pub expiration: DateTime<Utc>,
}

/// Caller identity information.
#[derive(Debug, Clone)]
pub struct CallerIdentity {
    pub user_id: String,
    pub account: String,
    pub arn: String,
}

/// Temporary credentials issued for the caller's own identity rather than for
/// a role: `AssumeRole` without `RoleArn`, accepted only under the
/// [`ASSUME_ROLE_WITHOUT_ROLE_ARN`](crate::ASSUME_ROLE_WITHOUT_ROLE_ARN)
/// relaxation. `GetCallerIdentity` with these credentials answers `identity`.
#[derive(Debug, Clone)]
pub struct CallerSession {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub session_token: String,
    pub expiration: DateTime<Utc>,
    /// The identity of the caller that requested the credentials.
    pub identity: CallerIdentity,
}
