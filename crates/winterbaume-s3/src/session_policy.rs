//! Opt-in enforcement of STS session policies on S3 data-plane requests.
//!
//! Switched on with [`crate::S3Service::with_session_policy_enforcement`];
//! see that method for the user-facing contract. Credentials from
//! `AssumeRole` ( and its SAML and web-identity forms ) with a `Policy` are
//! limited on AWS to what that session policy allows: a request it does not
//! allow, or explicitly denies, is refused with `403 AccessDenied`.
//!
//! S3 does not own the credentials, so it asks a [`SessionPolicyLookup`] for
//! the session policy of the access key that signed the request.
//! `winterbaume-server` implements the lookup with `winterbaume-sts`; S3 does
//! not depend on STS.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

use winterbaume_core::MockRequest;
use winterbaume_iam_rule_eval::{
    ConditionMode, EvalDecision, EvalOptions, PolicyDocument, PolicySource, PrincipalMode,
    SimulationContext, evaluate_with_options,
};

use crate::bucket_policy::{Permission, UNKNOWABLE_CONTEXT_KEYS};

/// Finds the session policy of temporary credentials by access key ID.
///
/// Implemented outside S3 by whatever issues the credentials;
/// `winterbaume-server` backs it with `StsService::session_policy`.
///
/// ```
/// use std::future::Future;
/// use std::pin::Pin;
/// use std::sync::Arc;
///
/// use winterbaume_s3::{S3Service, SessionPolicyLookup};
///
/// /// Every request signed with `ASIAEXAMPLE` may only read `bkt/public/*`.
/// struct Fixed;
///
/// impl SessionPolicyLookup for Fixed {
///     fn session_policy<'a>(
///         &'a self,
///         access_key_id: &'a str,
///     ) -> Pin<Box<dyn Future<Output = Option<String>> + Send + 'a>> {
///         Box::pin(async move {
///             (access_key_id == "ASIAEXAMPLE").then(|| {
///                 r#"{"Version":"2012-10-17","Statement":[{"Effect":"Allow",
///                     "Action":"s3:GetObject","Resource":"arn:aws:s3:::bkt/public/*"}]}"#
///                     .to_string()
///             })
///         })
///     }
/// }
///
/// let service = S3Service::new().with_session_policy_enforcement(Arc::new(Fixed));
/// assert!(service.session_policy_enforcement());
/// ```
pub trait SessionPolicyLookup: Send + Sync {
    /// The session policy document ( JSON ) of the credentials with this
    /// access key ID, or `None` when they carry no session policy or are not
    /// temporary credentials this lookup knows.
    fn session_policy<'a>(
        &'a self,
        access_key_id: &'a str,
    ) -> Pin<Box<dyn Future<Output = Option<String>> + Send + 'a>>;
}

/// The access key ID that signed the request: from the SigV4 `Authorization`
/// header, or from `X-Amz-Credential` for a presigned URL.
pub(crate) fn caller_access_key(
    request: &MockRequest,
    query: &HashMap<String, String>,
) -> Option<String> {
    winterbaume_core::auth::extract_access_key_from_headers(&request.headers).or_else(|| {
        query
            .get("X-Amz-Credential")
            .and_then(|credential| credential.split('/').next())
            .filter(|key| !key.is_empty())
            .map(str::to_string)
    })
}

/// A session policy, parsed once, ready to answer whether it allows a
/// permission.
pub(crate) struct SessionGuard {
    /// `None` when the stored document does not parse: it then allows
    /// nothing, since a session policy can only take permissions away.
    policies: Option<Vec<(PolicyDocument, PolicySource)>>,
    options: EvalOptions,
}

impl SessionGuard {
    pub(crate) fn new(policy_json: &str) -> Self {
        let policies = match serde_json::from_str::<PolicyDocument>(policy_json) {
            Ok(document) => Some(vec![(
                document,
                PolicySource {
                    id: "session-policy".to_string(),
                    policy_type: "SessionPolicy".to_string(),
                },
            )]),
            Err(error) => {
                tracing::warn!(
                    %error,
                    "session policy is not a valid policy document; denying the request"
                );
                None
            }
        };
        Self {
            policies,
            options: EvalOptions {
                condition_mode: ConditionMode::Evaluate,
                // Session policies are identity-side documents: no Principal.
                principal_mode: PrincipalMode::Ignore,
                unknowable_context_keys: UNKNOWABLE_CONTEXT_KEYS
                    .iter()
                    .map(|k| (*k).to_string())
                    .collect(),
            },
        }
    }

    /// The session policy's decision on `permission`: `Allowed` only when an
    /// `Allow` statement matches and no `Deny` does.
    pub(crate) fn decide(&self, permission: &Permission, base: &SimulationContext) -> EvalDecision {
        let Some(policies) = &self.policies else {
            return EvalDecision::ImplicitDenied;
        };
        let mut context = base.clone();
        for (key, value) in &permission.context {
            context
                .context_keys
                .insert(key.clone(), vec![value.clone()]);
        }
        evaluate_with_options(
            policies,
            permission.action,
            &permission.resource_arn(),
            &context,
            &self.options,
        )
        .decision
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const POLICY: &str = r#"{
        "Version": "2012-10-17",
        "Statement": [
            {"Effect": "Allow", "Action": ["s3:GetObject", "s3:PutObject"],
             "Resource": ["arn:aws:s3:::bkt/allowed/*"]},
            {"Effect": "Allow", "Action": "s3:ListBucket", "Resource": "arn:aws:s3:::bkt",
             "Condition": {"StringLike": {"s3:prefix": ["allowed/*"]}}},
            {"Effect": "Deny", "Action": "s3:PutObject",
             "Resource": "arn:aws:s3:::bkt/allowed/locked/*"}
        ]
    }"#;

    fn list(prefix: Option<&str>) -> Permission {
        let mut permission = Permission::object("s3:ListBucket", "bkt", "");
        permission.key = None;
        if let Some(prefix) = prefix {
            permission
                .context
                .push(("s3:prefix".to_string(), prefix.to_string()));
        }
        permission
    }

    #[test]
    fn allows_only_what_the_policy_allows() {
        let guard = SessionGuard::new(POLICY);
        let base = SimulationContext::default();
        let decide = |p: Permission| guard.decide(&p, &base);
        assert_eq!(
            decide(Permission::object("s3:PutObject", "bkt", "allowed/a")),
            EvalDecision::Allowed
        );
        assert_eq!(
            decide(Permission::object("s3:PutObject", "bkt", "outside/b")),
            EvalDecision::ImplicitDenied
        );
        assert_eq!(
            decide(Permission::object("s3:DeleteObject", "bkt", "allowed/a")),
            EvalDecision::ImplicitDenied
        );
        assert_eq!(
            decide(Permission::object(
                "s3:PutObject",
                "bkt",
                "allowed/locked/x"
            )),
            EvalDecision::ExplicitDenied
        );
        assert_eq!(decide(list(Some("allowed/x"))), EvalDecision::Allowed);
        assert_eq!(decide(list(Some("other/"))), EvalDecision::ImplicitDenied);
        // `s3:prefix` absent: the `StringLike` test fails, so nothing allows it.
        assert_eq!(decide(list(None)), EvalDecision::ImplicitDenied);
    }

    #[test]
    fn unparseable_policy_allows_nothing() {
        let guard = SessionGuard::new("not json");
        assert_eq!(
            guard.decide(
                &Permission::object("s3:GetObject", "bkt", "k"),
                &SimulationContext::default()
            ),
            EvalDecision::ImplicitDenied
        );
    }

    #[test]
    fn access_key_from_header_or_presigned_query() {
        let mut headers = http::HeaderMap::new();
        headers.insert(
            "authorization",
            "AWS4-HMAC-SHA256 Credential=ASIAHEADER/20260101/us-east-1/s3/aws4_request, SignedHeaders=host, Signature=0"
                .parse()
                .unwrap(),
        );
        let request = |headers: http::HeaderMap| MockRequest {
            method: "GET".to_string(),
            uri: "http://s3.us-east-1.amazonaws.com/bkt/k".to_string(),
            headers,
            body: bytes::Bytes::new(),
        };
        assert_eq!(
            caller_access_key(&request(headers), &HashMap::new()).as_deref(),
            Some("ASIAHEADER")
        );
        let query = HashMap::from([(
            "X-Amz-Credential".to_string(),
            "ASIAQUERY/20260101/us-east-1/s3/aws4_request".to_string(),
        )]);
        assert_eq!(
            caller_access_key(&request(http::HeaderMap::new()), &query).as_deref(),
            Some("ASIAQUERY")
        );
        assert_eq!(
            caller_access_key(&request(http::HeaderMap::new()), &HashMap::new()),
            None
        );
    }
}
