use winterbaume_core::default_account_id;

use crate::types::{AssumedRole, CallerIdentity};

/// Domain errors produced by STS request validation.
#[derive(Debug, thiserror::Error)]
pub enum StsError {
    #[error("Missing 'Action' parameter")]
    MissingAction,
    #[error("Could not find operation {action} for STS")]
    InvalidAction { action: String },
    #[error("{0}")]
    InvalidParameterValue(String),
    #[error("Missing '{0}'")]
    MissingParameter(&'static str),
}

/// In-memory state for the STS service.
#[derive(Debug, Default)]
pub struct StsState {
    pub assumed_roles: Vec<AssumedRole>,
}

impl StsState {
    /// Look up an assumed role by its access key ID.
    pub fn find_assumed_role_by_access_key(&self, access_key_id: &str) -> Option<&AssumedRole> {
        self.assumed_roles
            .iter()
            .find(|r| r.access_key_id == access_key_id)
    }

    /// Get the caller identity for a given access key.
    ///
    /// If the access key belongs to an assumed role, returns that role's identity.
    /// Otherwise returns a default mock identity.
    pub fn get_caller_identity(&self, _access_key_id: &str) -> CallerIdentity {
        // Check if this is an assumed role access key
        if let Some(role) = self.find_assumed_role_by_access_key(_access_key_id) {
            return CallerIdentity {
                user_id: role.user_id(),
                account: role.account_id.clone(),
                arn: role.arn(),
            };
        }

        // Default fallback identity
        CallerIdentity {
            user_id: "AKIAIOSFODNN7EXAMPLE".to_string(),
            account: default_account_id().to_string(),
            arn: format!(
                "arn:aws:sts::{account_id}:user/moto",
                account_id = default_account_id()
            ),
        }
    }
}
