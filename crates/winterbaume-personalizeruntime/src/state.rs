use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct PersonalizeRuntimeState {
    /// Cumulative count of recommendation calls per campaign or recommender ARN.
    pub call_counts: HashMap<String, u64>,
}

impl PersonalizeRuntimeState {
    pub fn record_call(&mut self, arn: &str) {
        *self.call_counts.entry(arn.to_string()).or_default() += 1;
    }
}
