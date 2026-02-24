use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct PricingState {
    /// Number of calls made per operation, for observability.
    pub call_counts: HashMap<String, u64>,
}

impl PricingState {
    pub fn record_call(&mut self, op: &str) {
        *self.call_counts.entry(op.to_string()).or_default() += 1;
    }
}
