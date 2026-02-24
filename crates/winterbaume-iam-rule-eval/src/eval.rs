use crate::condition::evaluate_condition;
use crate::types::{
    Effect, EvalDecision, EvalOutcome, MatchedStatement, PolicyDocument, PolicySource,
    SimulationContext,
};
use crate::wildcard::{action_matches, resource_matches};

/// Evaluate a set of IAM policies against an action/resource pair.
///
/// Standard AWS evaluation order:
/// 1. Collect all statements where action, resource, and condition match.
/// 2. Any matching **Deny** → `ExplicitDenied` (wins unconditionally).
/// 3. Any matching **Allow** → `Allowed`.
/// 4. Otherwise → `ImplicitDenied`.
///
/// Principal matching is skipped — identity-based simulation does not
/// evaluate `Principal`/`NotPrincipal` fields.
pub fn evaluate(
    policies: &[(PolicyDocument, PolicySource)],
    action: &str,
    resource: &str,
    context: &SimulationContext,
) -> EvalOutcome {
    let mut matched_deny: Vec<MatchedStatement> = Vec::new();
    let mut matched_allow: Vec<MatchedStatement> = Vec::new();
    let mut all_missing_keys: Vec<String> = Vec::new();

    for (doc, source) in policies {
        for stmt in &doc.statement {
            // ── Action matching ────────────────────────────────────
            let action_matched = if let Some(ref actions) = stmt.action {
                actions.as_slice().iter().any(|p| action_matches(p, action))
            } else if let Some(ref not_actions) = stmt.not_action {
                // NotAction: matches if the action does NOT match any pattern.
                !not_actions
                    .as_slice()
                    .iter()
                    .any(|p| action_matches(p, action))
            } else {
                // No Action or NotAction specified — does not match.
                false
            };

            if !action_matched {
                continue;
            }

            // ── Resource matching ──────────────────────────────────
            let resource_matched = if let Some(ref resources) = stmt.resource {
                resources
                    .as_slice()
                    .iter()
                    .any(|p| resource_matches(p, resource))
            } else if let Some(ref not_resources) = stmt.not_resource {
                // NotResource: matches if the resource does NOT match any pattern.
                !not_resources
                    .as_slice()
                    .iter()
                    .any(|p| resource_matches(p, resource))
            } else {
                // No Resource or NotResource — treat as matching all resources.
                true
            };

            if !resource_matched {
                continue;
            }

            // ── Condition matching (Phase 1: stub) ─────────────────
            let (cond_matched, missing_keys) = if let Some(ref cond) = stmt.condition {
                evaluate_condition(cond, context)
            } else {
                (true, vec![])
            };

            all_missing_keys.extend(missing_keys);

            if !cond_matched {
                continue;
            }

            // ── Record match ───────────────────────────────────────
            let ms = MatchedStatement {
                source_policy_id: source.id.clone(),
                source_policy_type: source.policy_type.clone(),
                start_position: None,
                end_position: None,
            };

            match stmt.effect {
                Effect::Deny => matched_deny.push(ms),
                Effect::Allow => matched_allow.push(ms),
            }
        }
    }

    // De-duplicate missing context keys.
    all_missing_keys.sort();
    all_missing_keys.dedup();

    if !matched_deny.is_empty() {
        EvalOutcome {
            decision: EvalDecision::ExplicitDenied,
            matched_statements: matched_deny,
            missing_context_keys: all_missing_keys,
        }
    } else if !matched_allow.is_empty() {
        EvalOutcome {
            decision: EvalDecision::Allowed,
            matched_statements: matched_allow,
            missing_context_keys: all_missing_keys,
        }
    } else {
        EvalOutcome {
            decision: EvalDecision::ImplicitDenied,
            matched_statements: vec![],
            missing_context_keys: all_missing_keys,
        }
    }
}

// ── Tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::PolicyDocument;

    fn parse_policy(json: &str) -> PolicyDocument {
        serde_json::from_str(json).unwrap()
    }

    fn source(id: &str) -> PolicySource {
        PolicySource {
            id: id.to_owned(),
            policy_type: "IAMPolicy".to_owned(),
        }
    }

    fn default_ctx() -> SimulationContext {
        SimulationContext::default()
    }

    // ── Basic Allow ────────────────────────────────────────────────

    #[test]
    fn simple_allow() {
        let doc = parse_policy(
            r#"{
                "Statement": [{
                    "Effect": "Allow",
                    "Action": "s3:GetObject",
                    "Resource": "*"
                }]
            }"#,
        );
        let outcome = evaluate(
            &[(doc, source("p1"))],
            "s3:GetObject",
            "arn:aws:s3:::bucket/key",
            &default_ctx(),
        );
        assert_eq!(outcome.decision, EvalDecision::Allowed);
        assert_eq!(outcome.matched_statements.len(), 1);
        assert_eq!(outcome.matched_statements[0].source_policy_id, "p1");
    }

    // ── Deny overrides Allow ───────────────────────────────────────

    #[test]
    fn deny_overrides_allow() {
        let allow = parse_policy(
            r#"{
                "Statement": [{
                    "Effect": "Allow",
                    "Action": "s3:*",
                    "Resource": "*"
                }]
            }"#,
        );
        let deny = parse_policy(
            r#"{
                "Statement": [{
                    "Effect": "Deny",
                    "Action": "s3:DeleteObject",
                    "Resource": "*"
                }]
            }"#,
        );
        let outcome = evaluate(
            &[(allow, source("allow")), (deny, source("deny"))],
            "s3:DeleteObject",
            "arn:aws:s3:::bucket/key",
            &default_ctx(),
        );
        assert_eq!(outcome.decision, EvalDecision::ExplicitDenied);
        assert_eq!(outcome.matched_statements.len(), 1);
        assert_eq!(outcome.matched_statements[0].source_policy_id, "deny");
    }

    // ── No match → ImplicitDenied ──────────────────────────────────

    #[test]
    fn no_match_implicit_denied() {
        let doc = parse_policy(
            r#"{
                "Statement": [{
                    "Effect": "Allow",
                    "Action": "s3:GetObject",
                    "Resource": "arn:aws:s3:::other-bucket/*"
                }]
            }"#,
        );
        let outcome = evaluate(
            &[(doc, source("p1"))],
            "s3:GetObject",
            "arn:aws:s3:::my-bucket/key",
            &default_ctx(),
        );
        assert_eq!(outcome.decision, EvalDecision::ImplicitDenied);
        assert!(outcome.matched_statements.is_empty());
    }

    // ── NotAction inversion ────────────────────────────────────────

    #[test]
    fn not_action_inversion() {
        let doc = parse_policy(
            r#"{
                "Statement": [{
                    "Effect": "Deny",
                    "NotAction": "s3:GetObject",
                    "Resource": "*"
                }]
            }"#,
        );

        // s3:PutObject does NOT match NotAction pattern → statement applies → Deny
        let outcome = evaluate(
            &[(doc.clone(), source("p1"))],
            "s3:PutObject",
            "arn:aws:s3:::bucket/key",
            &default_ctx(),
        );
        assert_eq!(outcome.decision, EvalDecision::ExplicitDenied);

        // s3:GetObject matches NotAction pattern → statement does NOT apply → ImplicitDenied
        let outcome = evaluate(
            &[(doc, source("p1"))],
            "s3:GetObject",
            "arn:aws:s3:::bucket/key",
            &default_ctx(),
        );
        assert_eq!(outcome.decision, EvalDecision::ImplicitDenied);
    }

    // ── NotResource inversion ──────────────────────────────────────

    #[test]
    fn not_resource_inversion() {
        let doc = parse_policy(
            r#"{
                "Statement": [{
                    "Effect": "Allow",
                    "Action": "s3:*",
                    "NotResource": "arn:aws:s3:::secret-bucket/*"
                }]
            }"#,
        );

        // Resource does NOT match NotResource → statement applies → Allowed
        let outcome = evaluate(
            &[(doc.clone(), source("p1"))],
            "s3:GetObject",
            "arn:aws:s3:::public-bucket/key",
            &default_ctx(),
        );
        assert_eq!(outcome.decision, EvalDecision::Allowed);

        // Resource matches NotResource → statement does NOT apply → ImplicitDenied
        let outcome = evaluate(
            &[(doc, source("p1"))],
            "s3:GetObject",
            "arn:aws:s3:::secret-bucket/key",
            &default_ctx(),
        );
        assert_eq!(outcome.decision, EvalDecision::ImplicitDenied);
    }

    // ── Wildcard patterns ──────────────────────────────────────────

    #[test]
    fn wildcard_action_pattern() {
        let doc = parse_policy(
            r#"{
                "Statement": [{
                    "Effect": "Allow",
                    "Action": "s3:Get*",
                    "Resource": "*"
                }]
            }"#,
        );
        let outcome = evaluate(
            &[(doc.clone(), source("p1"))],
            "s3:GetObject",
            "arn:aws:s3:::bucket/key",
            &default_ctx(),
        );
        assert_eq!(outcome.decision, EvalDecision::Allowed);

        let outcome = evaluate(
            &[(doc, source("p1"))],
            "s3:PutObject",
            "arn:aws:s3:::bucket/key",
            &default_ctx(),
        );
        assert_eq!(outcome.decision, EvalDecision::ImplicitDenied);
    }

    #[test]
    fn wildcard_resource_pattern() {
        let doc = parse_policy(
            r#"{
                "Statement": [{
                    "Effect": "Allow",
                    "Action": "s3:GetObject",
                    "Resource": "arn:aws:s3:::my-bucket/prefix*"
                }]
            }"#,
        );
        let outcome = evaluate(
            &[(doc.clone(), source("p1"))],
            "s3:GetObject",
            "arn:aws:s3:::my-bucket/prefix/key.txt",
            &default_ctx(),
        );
        assert_eq!(outcome.decision, EvalDecision::Allowed);

        let outcome = evaluate(
            &[(doc, source("p1"))],
            "s3:GetObject",
            "arn:aws:s3:::my-bucket/other/key.txt",
            &default_ctx(),
        );
        assert_eq!(outcome.decision, EvalDecision::ImplicitDenied);
    }

    // ── Multiple policies ──────────────────────────────────────────

    #[test]
    fn multiple_policies_combined() {
        let p1 = parse_policy(
            r#"{
                "Statement": [{
                    "Effect": "Allow",
                    "Action": "s3:GetObject",
                    "Resource": "*"
                }]
            }"#,
        );
        let p2 = parse_policy(
            r#"{
                "Statement": [{
                    "Effect": "Allow",
                    "Action": "ec2:DescribeInstances",
                    "Resource": "*"
                }]
            }"#,
        );

        let outcome = evaluate(
            &[(p1.clone(), source("p1")), (p2.clone(), source("p2"))],
            "s3:GetObject",
            "arn:aws:s3:::bucket/key",
            &default_ctx(),
        );
        assert_eq!(outcome.decision, EvalDecision::Allowed);

        let outcome = evaluate(
            &[(p1, source("p1")), (p2, source("p2"))],
            "lambda:InvokeFunction",
            "arn:aws:lambda:us-east-1:123456789012:function:my-func",
            &default_ctx(),
        );
        assert_eq!(outcome.decision, EvalDecision::ImplicitDenied);
    }

    // ── Case-insensitive action matching ───────────────────────────

    #[test]
    fn action_case_insensitive() {
        let doc = parse_policy(
            r#"{
                "Statement": [{
                    "Effect": "Allow",
                    "Action": "S3:GetObject",
                    "Resource": "*"
                }]
            }"#,
        );
        let outcome = evaluate(
            &[(doc, source("p1"))],
            "s3:getobject",
            "arn:aws:s3:::bucket/key",
            &default_ctx(),
        );
        assert_eq!(outcome.decision, EvalDecision::Allowed);
    }

    // ── Empty policies ─────────────────────────────────────────────

    #[test]
    fn empty_policies() {
        let outcome = evaluate(
            &[],
            "s3:GetObject",
            "arn:aws:s3:::bucket/key",
            &default_ctx(),
        );
        assert_eq!(outcome.decision, EvalDecision::ImplicitDenied);
        assert!(outcome.matched_statements.is_empty());
    }

    // ── No resource specified in statement ─────────────────────────

    #[test]
    fn no_resource_in_statement_matches_all() {
        let doc = parse_policy(
            r#"{
                "Statement": [{
                    "Effect": "Allow",
                    "Action": "sts:GetCallerIdentity"
                }]
            }"#,
        );
        let outcome = evaluate(
            &[(doc, source("p1"))],
            "sts:GetCallerIdentity",
            "*",
            &default_ctx(),
        );
        assert_eq!(outcome.decision, EvalDecision::Allowed);
    }
}
