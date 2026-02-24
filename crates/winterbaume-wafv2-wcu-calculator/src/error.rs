use thiserror::Error;

#[derive(Debug, Error)]
pub enum WcuError {
    #[error("unknown managed rule group: vendor={vendor}, name={name}")]
    UnknownManagedRuleGroup { vendor: String, name: String },

    #[error("unresolved rule group reference ARN: {0}")]
    UnresolvedRuleGroup(String),
}
