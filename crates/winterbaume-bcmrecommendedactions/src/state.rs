use thiserror::Error;

#[derive(Debug, Default)]
pub struct BcmRecommendedActionsState {
    /// Mock-side recommended actions; empty by default. Tests can populate via the
    /// state view if a non-empty response is desired.
    pub recommended_actions: Vec<RecommendedAction>,
}

#[derive(Debug, Clone)]
pub struct RecommendedAction {
    pub id: Option<String>,
    pub action_type: Option<String>,
    pub account_id: Option<String>,
    pub severity: Option<String>,
    pub feature: Option<String>,
    pub context: std::collections::HashMap<String, String>,
    pub next_steps: Vec<String>,
    pub last_updated_timestamp: Option<String>,
}

#[derive(Debug, Error)]
pub enum BcmRecommendedActionsError {
    #[error("{message}")]
    Validation { message: String },
}
