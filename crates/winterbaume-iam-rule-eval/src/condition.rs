use crate::types::{ConditionBlock, SimulationContext};

/// Evaluate a condition block against the simulation context.
///
/// **Phase 1 stub**: always returns `(true, vec![])` — i.e. conditions are
/// treated as satisfied with no missing context keys.
///
/// Phase 2+ will implement the full operator dispatch.
pub fn evaluate_condition(
    _condition: &ConditionBlock,
    _context: &SimulationContext,
) -> (bool, Vec<String>) {
    (true, vec![])
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn stub_always_true() {
        let cond: ConditionBlock = HashMap::new();
        let ctx = SimulationContext::default();
        let (matched, missing) = evaluate_condition(&cond, &ctx);
        assert!(matched);
        assert!(missing.is_empty());
    }
}
