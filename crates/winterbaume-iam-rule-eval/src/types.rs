use std::collections::HashMap;
use std::fmt;

use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::{Deserialize, Serialize};

// ── StringOrVec ────────────────────────────────────────────────────────

/// A value that can be deserialised from either a single JSON string or an
/// array of strings.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct StringOrVec(pub Vec<String>);

impl StringOrVec {
    pub fn as_slice(&self) -> &[String] {
        &self.0
    }
}

impl<'de> Deserialize<'de> for StringOrVec {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct SoVVisitor;

        impl<'de> Visitor<'de> for SoVVisitor {
            type Value = StringOrVec;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a string or array of strings")
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(StringOrVec(vec![v.to_owned()]))
            }

            fn visit_seq<A: de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut v = Vec::new();
                while let Some(s) = seq.next_element::<String>()? {
                    v.push(s);
                }
                Ok(StringOrVec(v))
            }
        }

        deserializer.deserialize_any(SoVVisitor)
    }
}

// ── Effect ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Effect {
    Allow,
    Deny,
}

// ── PrincipalSpec ──────────────────────────────────────────────────────

/// IAM principal specification: either `"*"` or a map like
/// `{ "AWS": ["arn:..."], "Service": ["..."] }`.
#[derive(Debug, Clone, PartialEq)]
pub enum PrincipalSpec {
    Wildcard,
    Map(HashMap<String, Vec<String>>),
}

impl<'de> Deserialize<'de> for PrincipalSpec {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct PrincipalVisitor;

        impl<'de> Visitor<'de> for PrincipalVisitor {
            type Value = PrincipalSpec;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str(r#""*" or a map of principal types"#)
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                if v == "*" {
                    Ok(PrincipalSpec::Wildcard)
                } else {
                    Err(de::Error::custom(format!(
                        "expected \"*\" for wildcard principal, got \"{v}\""
                    )))
                }
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut m = HashMap::new();
                while let Some((key, val)) = map.next_entry::<String, StringOrVec>()? {
                    m.insert(key, val.0);
                }
                Ok(PrincipalSpec::Map(m))
            }
        }

        deserializer.deserialize_any(PrincipalVisitor)
    }
}

// ── ConditionBlock ─────────────────────────────────────────────────────

/// Outer key: operator name (e.g. `"StringEquals"`).
/// Inner key: condition context key (e.g. `"aws:RequestedRegion"`).
/// Value: one or more string values.
pub type ConditionBlock = HashMap<String, HashMap<String, StringOrVec>>;

// ── Statement ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Statement {
    #[serde(default)]
    pub sid: Option<String>,

    pub effect: Effect,

    #[serde(default)]
    pub principal: Option<PrincipalSpec>,

    #[serde(default)]
    pub not_principal: Option<PrincipalSpec>,

    #[serde(default)]
    pub action: Option<StringOrVec>,

    #[serde(default)]
    pub not_action: Option<StringOrVec>,

    #[serde(default)]
    pub resource: Option<StringOrVec>,

    #[serde(default)]
    pub not_resource: Option<StringOrVec>,

    #[serde(default)]
    pub condition: Option<ConditionBlock>,
}

// ── PolicyDocument ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PolicyDocument {
    #[serde(default)]
    pub version: Option<String>,

    pub statement: Vec<Statement>,
}

// ── PolicySource ───────────────────────────────────────────────────────

/// Metadata carried alongside each `PolicyDocument` so matched statements
/// in the response can be annotated with `source_policy_id` / `source_policy_type`.
#[derive(Debug, Clone)]
pub struct PolicySource {
    pub id: String,
    pub policy_type: String,
}

// ── SimulationContext ──────────────────────────────────────────────────

/// Request context supplied to the evaluation engine.
#[derive(Debug, Clone, Default)]
pub struct SimulationContext {
    pub caller_arn: Option<String>,
    /// Keys normalised to lowercase.
    pub context_keys: HashMap<String, Vec<String>>,
    /// Key → type name for type-aware condition comparison.
    pub context_key_types: HashMap<String, String>,
}

// ── EvalDecision / EvalOutcome / MatchedStatement ──────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvalDecision {
    Allowed,
    ExplicitDenied,
    ImplicitDenied,
}

impl EvalDecision {
    /// AWS string representation used in `SimulateCustomPolicy` / `SimulatePrincipalPolicy` responses.
    pub fn as_aws_str(&self) -> &'static str {
        match self {
            EvalDecision::Allowed => "allowed",
            EvalDecision::ExplicitDenied => "explicitDeny",
            EvalDecision::ImplicitDenied => "implicitDeny",
        }
    }
}

#[derive(Debug, Clone)]
pub struct MatchedStatement {
    pub source_policy_id: String,
    pub source_policy_type: String,
    pub start_position: Option<(i64, i64)>,
    pub end_position: Option<(i64, i64)>,
}

#[derive(Debug, Clone)]
pub struct EvalOutcome {
    pub decision: EvalDecision,
    pub matched_statements: Vec<MatchedStatement>,
    pub missing_context_keys: Vec<String>,
}

// ── Tests ──────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_string_or_vec_string() {
        let s: StringOrVec = serde_json::from_str(r#""s3:GetObject""#).unwrap();
        assert_eq!(s.as_slice(), &["s3:GetObject"]);
    }

    #[test]
    fn deserialize_string_or_vec_array() {
        let s: StringOrVec = serde_json::from_str(r#"["s3:GetObject", "s3:PutObject"]"#).unwrap();
        assert_eq!(s.as_slice(), &["s3:GetObject", "s3:PutObject"]);
    }

    #[test]
    fn deserialize_effect() {
        let e: Effect = serde_json::from_str(r#""Allow""#).unwrap();
        assert_eq!(e, Effect::Allow);
        let e: Effect = serde_json::from_str(r#""Deny""#).unwrap();
        assert_eq!(e, Effect::Deny);
    }

    #[test]
    fn deserialize_principal_wildcard() {
        let p: PrincipalSpec = serde_json::from_str(r#""*""#).unwrap();
        assert_eq!(p, PrincipalSpec::Wildcard);
    }

    #[test]
    fn deserialize_principal_map() {
        let p: PrincipalSpec =
            serde_json::from_str(r#"{"AWS": "arn:aws:iam::123456789012:root"}"#).unwrap();
        match p {
            PrincipalSpec::Map(m) => {
                assert_eq!(m.get("AWS").unwrap(), &["arn:aws:iam::123456789012:root"]);
            }
            _ => panic!("expected Map"),
        }
    }

    #[test]
    fn deserialize_principal_map_array() {
        let p: PrincipalSpec =
            serde_json::from_str(r#"{"Service": ["lambda.amazonaws.com", "ec2.amazonaws.com"]}"#)
                .unwrap();
        match p {
            PrincipalSpec::Map(m) => {
                assert_eq!(m.get("Service").unwrap().len(), 2);
            }
            _ => panic!("expected Map"),
        }
    }

    #[test]
    fn deserialize_simple_allow_policy() {
        let json = r#"{
            "Version": "2012-10-17",
            "Statement": [
                {
                    "Effect": "Allow",
                    "Action": "s3:GetObject",
                    "Resource": "arn:aws:s3:::my-bucket/*"
                }
            ]
        }"#;
        let doc: PolicyDocument = serde_json::from_str(json).unwrap();
        assert_eq!(doc.version.as_deref(), Some("2012-10-17"));
        assert_eq!(doc.statement.len(), 1);
        assert_eq!(doc.statement[0].effect, Effect::Allow);
        assert_eq!(
            doc.statement[0].action.as_ref().unwrap().as_slice(),
            &["s3:GetObject"]
        );
        assert_eq!(
            doc.statement[0].resource.as_ref().unwrap().as_slice(),
            &["arn:aws:s3:::my-bucket/*"]
        );
    }

    #[test]
    fn deserialize_deny_with_condition() {
        let json = r#"{
            "Version": "2012-10-17",
            "Statement": [
                {
                    "Sid": "DenyNonUsEast",
                    "Effect": "Deny",
                    "Action": "*",
                    "Resource": "*",
                    "Condition": {
                        "StringNotEquals": {
                            "aws:RequestedRegion": ["us-east-1", "us-east-2"]
                        }
                    }
                }
            ]
        }"#;
        let doc: PolicyDocument = serde_json::from_str(json).unwrap();
        let stmt = &doc.statement[0];
        assert_eq!(stmt.sid.as_deref(), Some("DenyNonUsEast"));
        assert_eq!(stmt.effect, Effect::Deny);
        let cond = stmt.condition.as_ref().unwrap();
        let inner = cond.get("StringNotEquals").unwrap();
        assert_eq!(
            inner.get("aws:RequestedRegion").unwrap().as_slice(),
            &["us-east-1", "us-east-2"]
        );
    }

    #[test]
    fn deserialize_not_action_not_resource() {
        let json = r#"{
            "Statement": [
                {
                    "Effect": "Deny",
                    "NotAction": ["iam:*", "sts:*"],
                    "NotResource": "*"
                }
            ]
        }"#;
        let doc: PolicyDocument = serde_json::from_str(json).unwrap();
        let stmt = &doc.statement[0];
        assert!(stmt.action.is_none());
        assert!(stmt.resource.is_none());
        assert_eq!(
            stmt.not_action.as_ref().unwrap().as_slice(),
            &["iam:*", "sts:*"]
        );
        assert_eq!(stmt.not_resource.as_ref().unwrap().as_slice(), &["*"]);
    }

    #[test]
    fn eval_decision_aws_str() {
        assert_eq!(EvalDecision::Allowed.as_aws_str(), "allowed");
        assert_eq!(EvalDecision::ExplicitDenied.as_aws_str(), "explicitDeny");
        assert_eq!(EvalDecision::ImplicitDenied.as_aws_str(), "implicitDeny");
    }
}
