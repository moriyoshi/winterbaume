use std::collections::HashMap;
use std::fmt;

use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::{Deserialize, Serialize};

// ── StringOrVec ────────────────────────────────────────────────────────

/// A value that can be deserialised from either a single JSON string or an
/// array of strings.
///
/// JSON booleans and numbers are accepted wherever a string is and are kept in
/// their canonical string form, because IAM admits both `"false"` and `false`
/// ( and both `"10"` and `10` ) as condition values.
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

            fn visit_bool<E: de::Error>(self, v: bool) -> Result<Self::Value, E> {
                Ok(StringOrVec(vec![v.to_string()]))
            }

            fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
                Ok(StringOrVec(vec![v.to_string()]))
            }

            fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
                Ok(StringOrVec(vec![v.to_string()]))
            }

            fn visit_f64<E: de::Error>(self, v: f64) -> Result<Self::Value, E> {
                Ok(StringOrVec(vec![v.to_string()]))
            }

            fn visit_seq<A: de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut v = Vec::new();
                while let Some(s) = seq.next_element::<ScalarString>()? {
                    v.push(s.0);
                }
                Ok(StringOrVec(v))
            }
        }

        deserializer.deserialize_any(SoVVisitor)
    }
}

/// One scalar policy value: a string, or a boolean / number rendered as one.
struct ScalarString(String);

impl<'de> Deserialize<'de> for ScalarString {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ScalarVisitor;

        impl Visitor<'_> for ScalarVisitor {
            type Value = ScalarString;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a string, boolean or number")
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                Ok(ScalarString(v.to_owned()))
            }

            fn visit_bool<E: de::Error>(self, v: bool) -> Result<Self::Value, E> {
                Ok(ScalarString(v.to_string()))
            }

            fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> {
                Ok(ScalarString(v.to_string()))
            }

            fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> {
                Ok(ScalarString(v.to_string()))
            }

            fn visit_f64<E: de::Error>(self, v: f64) -> Result<Self::Value, E> {
                Ok(ScalarString(v.to_string()))
            }
        }

        deserializer.deserialize_any(ScalarVisitor)
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

impl PrincipalSpec {
    /// Whether this principal element names every caller: the bare `"*"`, or
    /// an `AWS` entry containing `"*"` ( `{"AWS": "*"}` ).
    pub fn is_wildcard(&self) -> bool {
        match self {
            PrincipalSpec::Wildcard => true,
            PrincipalSpec::Map(map) => map
                .get("AWS")
                .is_some_and(|values| values.iter().any(|v| v == "*")),
        }
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

    /// IAM admits both a single statement object and an array of statements.
    #[serde(deserialize_with = "deserialize_statements")]
    pub statement: Vec<Statement>,
}

fn deserialize_statements<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Vec<Statement>, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum OneOrMany {
        Many(Vec<Statement>),
        One(Box<Statement>),
    }

    Ok(match OneOrMany::deserialize(deserializer)? {
        OneOrMany::Many(statements) => statements,
        OneOrMany::One(statement) => vec![*statement],
    })
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

// ── EvalOptions ────────────────────────────────────────────────────────

/// How `Condition` blocks are treated during evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConditionMode {
    /// Every `Condition` block is treated as satisfied. This is the behaviour
    /// of [`crate::evaluate`] and what the IAM `Simulate*` handlers rely on.
    #[default]
    AssumeSatisfied,
    /// `Condition` blocks are evaluated against
    /// [`SimulationContext::context_keys`] with IAM's operator semantics. See
    /// [`crate::condition::evaluate_condition_block`].
    Evaluate,
}

/// How `Principal` / `NotPrincipal` are treated during evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PrincipalMode {
    /// The elements are not consulted ( identity-policy evaluation ).
    #[default]
    Ignore,
    /// Resource-policy evaluation for a caller whose identity is unknown: a
    /// statement applies only when it carries a wildcard `Principal`
    /// ( see [`PrincipalSpec::is_wildcard`] ) and no `NotPrincipal`. Statements
    /// naming specific principals, using `NotPrincipal`, or carrying no
    /// `Principal` at all are skipped, because whether they cover the caller
    /// cannot be decided.
    WildcardOnly,
}

/// Knobs for [`crate::evaluate_with_options`]. The default reproduces
/// [`crate::evaluate`].
#[derive(Debug, Clone, Default)]
pub struct EvalOptions {
    pub condition_mode: ConditionMode,
    pub principal_mode: PrincipalMode,
    /// Lower-cased glob patterns ( `*`, `?` ) of condition keys whose value the
    /// caller of the engine cannot determine, as opposed to keys that are
    /// genuinely absent from the request. With [`ConditionMode::Evaluate`], a
    /// statement whose `Condition` references such a key that is not present
    /// in [`SimulationContext::context_keys`] is skipped as undecidable rather
    /// than run through IAM's missing-key rules.
    pub unknowable_context_keys: Vec<String>,
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
    fn deserialize_string_or_vec_scalars() {
        let s: StringOrVec = serde_json::from_str("false").unwrap();
        assert_eq!(s.as_slice(), &["false"]);
        let s: StringOrVec = serde_json::from_str("10").unwrap();
        assert_eq!(s.as_slice(), &["10"]);
        let s: StringOrVec = serde_json::from_str(r#"[true, 5, "x"]"#).unwrap();
        assert_eq!(s.as_slice(), &["true", "5", "x"]);
    }

    #[test]
    fn deserialize_single_statement_object() {
        let doc: PolicyDocument = serde_json::from_str(
            r#"{"Statement": {"Effect": "Deny", "Action": "s3:*", "Resource": "*"}}"#,
        )
        .unwrap();
        assert_eq!(doc.statement.len(), 1);
        assert_eq!(doc.statement[0].effect, Effect::Deny);
    }

    #[test]
    fn principal_wildcard_forms() {
        let p: PrincipalSpec = serde_json::from_str(r#""*""#).unwrap();
        assert!(p.is_wildcard());
        let p: PrincipalSpec = serde_json::from_str(r#"{"AWS": "*"}"#).unwrap();
        assert!(p.is_wildcard());
        let p: PrincipalSpec =
            serde_json::from_str(r#"{"AWS": ["arn:aws:iam::1:root", "*"]}"#).unwrap();
        assert!(p.is_wildcard());
        let p: PrincipalSpec =
            serde_json::from_str(r#"{"AWS": "arn:aws:iam::123456789012:root"}"#).unwrap();
        assert!(!p.is_wildcard());
        let p: PrincipalSpec =
            serde_json::from_str(r#"{"Service": "cloudtrail.amazonaws.com"}"#).unwrap();
        assert!(!p.is_wildcard());
    }

    #[test]
    fn eval_decision_aws_str() {
        assert_eq!(EvalDecision::Allowed.as_aws_str(), "allowed");
        assert_eq!(EvalDecision::ExplicitDenied.as_aws_str(), "explicitDeny");
        assert_eq!(EvalDecision::ImplicitDenied.as_aws_str(), "implicitDeny");
    }
}
