use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct AccountSettings {
    /// One of `SUBSCRIBED` or `NOT_SUBSCRIBED`. Defaults to `NOT_SUBSCRIBED`.
    pub notification_subscription_status: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Report {
    pub id: String,
    pub version: i64,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub series: Option<String>,
    pub state: String,
    pub arn: String,
    /// Document URL returned to GetReport callers.
    pub document_url: String,
}

#[derive(Debug, Clone)]
pub struct CustomerAgreement {
    pub name: String,
    pub arn: String,
    pub id: String,
    pub agreement_arn: String,
    pub aws_account_id: String,
    pub organization_arn: Option<String>,
    pub effective_start: Option<i64>,
    pub effective_end: Option<i64>,
    pub state: String,
    /// Free-form key/value tags ( the Smithy model does not declare tag
    /// operations; this is here only for state-view round-trips ).
    pub tags: HashMap<String, String>,
}
