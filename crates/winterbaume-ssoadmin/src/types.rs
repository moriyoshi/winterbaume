//! Domain types for the SSO Admin service.

use std::collections::HashMap;

pub const INSTANCE_ARN: &str = "arn:aws:sso:::instance/ssoins-0123456789abcdef";
pub const IDENTITY_STORE_ID: &str = "d-0123456789";

#[derive(Debug, Clone)]
pub struct PermissionSetData {
    pub permission_set_arn: String,
    pub name: String,
    pub description: Option<String>,
    pub session_duration: Option<String>,
    pub relay_state: Option<String>,
    pub inline_policy: Option<String>,
    /// AWS-managed policy ARNs
    pub managed_policies: Vec<String>,
    /// Customer-managed policy references: (name, path)
    pub customer_managed_policies: Vec<(String, Option<String>)>,
    pub tags: HashMap<String, String>,
    pub created_date: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AccountAssignmentKey {
    pub account_id: String,
    pub permission_set_arn: String,
    pub principal_type: String,
    pub principal_id: String,
}
