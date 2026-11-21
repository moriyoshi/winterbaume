//! Opt-in enforcement of S3 bucket policies ( explicit `Deny` only ).
//!
//! Switched on with [`crate::S3Service::with_bucket_policy_enforcement`]; see
//! that method for the user-facing contract. This module holds the two pure
//! pieces: the mapping from a data-plane request to the IAM action(s) and
//! resource ARN(s) it needs ( [`required_permissions`] ), and the request
//! context handed to `winterbaume-iam-rule-eval` ( [`request_context`] ).
//!
//! Action names and resource types follow the Service Authorization Reference
//! for Amazon S3 and the "Permissions" sections of the S3 API reference.

use std::collections::HashMap;

use http::HeaderMap;
use winterbaume_core::MockRequest;
use winterbaume_iam_rule_eval::{
    ConditionMode, EvalDecision, EvalOptions, PolicyDocument, PolicySource, PrincipalMode,
    SimulationContext, evaluate_with_options,
};

/// Request header that overrides the URI scheme when deciding
/// `aws:SecureTransport` ( `https` means secure, anything else plain ).
pub(crate) const FORWARDED_PROTO_HEADER: &str = "x-forwarded-proto";

/// Request header whose first entry supplies `aws:SourceIp`.
pub(crate) const FORWARDED_FOR_HEADER: &str = "x-forwarded-for";

/// Condition keys describing the caller's identity. The emulator does not
/// authenticate callers, so these are undecidable rather than absent: a
/// statement conditioned on one is skipped, for the same reason a statement
/// naming a specific `Principal` is.
const UNKNOWABLE_CONTEXT_KEYS: &[&str] = &[
    "aws:principalarn",
    "aws:principalaccount",
    "aws:principalorgid",
    "aws:principalorgpaths",
    "aws:principaltag/*",
    "aws:principaltype",
    "aws:principalisawsservice",
    "aws:principalservicename",
    "aws:principalservicenameslist",
    "aws:userid",
    "aws:username",
    "aws:sourceidentity",
    "aws:rolesessionname",
    "aws:federatedprovider",
    "aws:tokenissuetime",
    "aws:multifactorauthpresent",
    "aws:multifactorauthage",
    "aws:ec2instancesourcevpc",
    "aws:ec2instancesourceprivateipv4",
];

/// Request headers that surface verbatim as `s3:<name>` condition keys.
const HEADER_CONDITION_KEYS: &[(&str, &str)] = &[
    ("x-amz-acl", "s3:x-amz-acl"),
    ("x-amz-content-sha256", "s3:x-amz-content-sha256"),
    ("x-amz-grant-full-control", "s3:x-amz-grant-full-control"),
    ("x-amz-grant-read", "s3:x-amz-grant-read"),
    ("x-amz-grant-read-acp", "s3:x-amz-grant-read-acp"),
    ("x-amz-grant-write", "s3:x-amz-grant-write"),
    ("x-amz-grant-write-acp", "s3:x-amz-grant-write-acp"),
    ("x-amz-metadata-directive", "s3:x-amz-metadata-directive"),
    (
        "x-amz-server-side-encryption",
        "s3:x-amz-server-side-encryption",
    ),
    (
        "x-amz-server-side-encryption-aws-kms-key-id",
        "s3:x-amz-server-side-encryption-aws-kms-key-id",
    ),
    (
        "x-amz-server-side-encryption-customer-algorithm",
        "s3:x-amz-server-side-encryption-customer-algorithm",
    ),
    ("x-amz-storage-class", "s3:x-amz-storage-class"),
    (
        "x-amz-website-redirect-location",
        "s3:x-amz-website-redirect-location",
    ),
    ("x-amz-object-lock-mode", "s3:object-lock-mode"),
    ("x-amz-object-lock-legal-hold", "s3:object-lock-legal-hold"),
    (
        "x-amz-object-lock-retain-until-date",
        "s3:object-lock-retain-until-date",
    ),
    ("if-match", "s3:if-match"),
    ("if-none-match", "s3:if-none-match"),
    ("user-agent", "aws:useragent"),
    ("referer", "aws:referer"),
];

/// One IAM permission a request needs: an action on a bucket or object.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Permission {
    pub action: &'static str,
    /// Bucket whose policy governs this permission ( for copies, the source
    /// permission names the source bucket ).
    pub bucket: String,
    /// `None` for a bucket-level action.
    pub key: Option<String>,
    /// Condition keys ( lower-cased ) specific to this permission.
    pub context: Vec<(String, String)>,
}

impl Permission {
    fn bucket(action: &'static str, bucket: &str) -> Self {
        Self {
            action,
            bucket: bucket.to_string(),
            key: None,
            context: Vec::new(),
        }
    }

    pub(crate) fn object(action: &'static str, bucket: &str, key: &str) -> Self {
        Self {
            action,
            bucket: bucket.to_string(),
            key: Some(key.to_string()),
            context: Vec::new(),
        }
    }

    /// An object permission whose action depends on whether a specific
    /// version is addressed; the versioned form also carries `s3:versionid`.
    pub(crate) fn object_version(
        plain: &'static str,
        versioned: &'static str,
        bucket: &str,
        key: &str,
        version_id: Option<&str>,
    ) -> Self {
        match version_id.filter(|v| !v.is_empty()) {
            Some(version_id) => {
                let mut permission = Self::object(versioned, bucket, key);
                permission
                    .context
                    .push(("s3:versionid".to_string(), version_id.to_string()));
                permission
            }
            None => Self::object(plain, bucket, key),
        }
    }

    /// `arn:aws:s3:::bucket` or `arn:aws:s3:::bucket/key`.
    pub(crate) fn resource_arn(&self) -> String {
        match &self.key {
            Some(key) => format!("arn:aws:s3:::{}/{}", self.bucket, key),
            None => format!("arn:aws:s3:::{}", self.bucket),
        }
    }
}

/// Map a request to the permissions it needs.
///
/// Returns an empty list for everything that is not an enforced data-plane
/// operation: service-level calls, bucket creation / deletion, every bucket
/// configuration sub-resource ( including `?policy` itself ), directory-bucket
/// operations that have no `s3:` action ( `CreateSession`, `RenameObject` ),
/// and `DeleteObjects`, which is authorised per key by its handler.
///
/// The query checks mirror the order of `S3Service::dispatch`.
pub(crate) fn required_permissions(
    method: &str,
    bucket: &str,
    key: Option<&str>,
    query: &HashMap<String, String>,
    headers: &HeaderMap,
) -> Vec<Permission> {
    let has = |name: &str| query.contains_key(name);
    let version_id = query.get("versionId").map(String::as_str);

    let Some(key) = key else {
        return match method {
            "HEAD" => vec![Permission::bucket("s3:ListBucket", bucket)],
            "GET" => {
                let config_before_versions = query
                    .keys()
                    .any(|k| CONFIG_BEFORE_VERSIONS.contains(&k.as_str()));
                let config_after_versions = query
                    .keys()
                    .any(|k| CONFIG_AFTER_VERSIONS.contains(&k.as_str()));
                if config_before_versions {
                    Vec::new()
                } else if has("versions") {
                    vec![list_permission("s3:ListBucketVersions", bucket, query)]
                } else if config_after_versions {
                    Vec::new()
                } else if has("uploads") {
                    vec![Permission::bucket("s3:ListBucketMultipartUploads", bucket)]
                } else {
                    // ListObjects / ListObjectsV2.
                    vec![list_permission("s3:ListBucket", bucket, query)]
                }
            }
            _ => Vec::new(),
        };
    };

    let versioned = |plain: &'static str, versioned: &'static str| {
        Permission::object_version(plain, versioned, bucket, key, version_id)
    };
    let object = |action: &'static str| Permission::object(action, bucket, key);
    let copy = || {
        let mut permissions = Vec::new();
        if let Some((source_bucket, source_key, source_version)) = parse_copy_source(headers) {
            permissions.push(Permission::object_version(
                "s3:GetObject",
                "s3:GetObjectVersion",
                &source_bucket,
                &source_key,
                source_version.as_deref(),
            ));
        }
        permissions.push(object("s3:PutObject"));
        permissions
    };
    let is_copy = headers.contains_key("x-amz-copy-source");

    match method {
        "PUT" => {
            if has("partNumber") && has("uploadId") {
                if is_copy {
                    copy()
                } else {
                    vec![object("s3:PutObject")]
                }
            } else if has("acl") {
                vec![versioned("s3:PutObjectAcl", "s3:PutObjectVersionAcl")]
            } else if has("tagging") {
                vec![versioned(
                    "s3:PutObjectTagging",
                    "s3:PutObjectVersionTagging",
                )]
            } else if has("legal-hold") {
                vec![object("s3:PutObjectLegalHold")]
            } else if has("retention") {
                vec![object("s3:PutObjectRetention")]
            } else if has("renameObject") {
                Vec::new()
            } else if has("encryption") {
                vec![object("s3:UpdateObjectEncryption")]
            } else if is_copy {
                copy()
            } else {
                vec![object("s3:PutObject")]
            }
        }
        "GET" => {
            if has("acl") {
                vec![versioned("s3:GetObjectAcl", "s3:GetObjectVersionAcl")]
            } else if has("tagging") {
                vec![versioned(
                    "s3:GetObjectTagging",
                    "s3:GetObjectVersionTagging",
                )]
            } else if has("legal-hold") {
                vec![object("s3:GetObjectLegalHold")]
            } else if has("attributes") {
                vec![
                    versioned("s3:GetObject", "s3:GetObjectVersion"),
                    versioned("s3:GetObjectAttributes", "s3:GetObjectVersionAttributes"),
                ]
            } else if has("retention") {
                vec![object("s3:GetObjectRetention")]
            } else if has("uploadId") {
                vec![object("s3:ListMultipartUploadParts")]
            } else if has("torrent") {
                vec![object("s3:GetObjectTorrent")]
            } else {
                vec![versioned("s3:GetObject", "s3:GetObjectVersion")]
            }
        }
        "HEAD" => vec![versioned("s3:GetObject", "s3:GetObjectVersion")],
        "DELETE" => {
            if has("uploadId") {
                vec![object("s3:AbortMultipartUpload")]
            } else if has("tagging") {
                vec![versioned(
                    "s3:DeleteObjectTagging",
                    "s3:DeleteObjectVersionTagging",
                )]
            } else {
                vec![versioned("s3:DeleteObject", "s3:DeleteObjectVersion")]
            }
        }
        "POST" => {
            if has("uploads") || has("uploadId") {
                vec![object("s3:PutObject")]
            } else if has("select") {
                vec![object("s3:GetObject")]
            } else if has("restore") {
                vec![object("s3:RestoreObject")]
            } else {
                Vec::new()
            }
        }
        _ => Vec::new(),
    }
}

/// Query keys that `S3Service::dispatch` routes to a bucket-configuration
/// ( control-plane ) handler for `GET /{Bucket}`, split around the point where
/// it tests `versions`. Whatever is left falls through to `uploads` and then
/// to the object listings. Keep both lists in step with `dispatch`.
const CONFIG_BEFORE_VERSIONS: &[&str] = &[
    "lifecycle",
    "notification",
    "inventory",
    "location",
    "versioning",
    "acl",
    "policy",
    "tagging",
    "accelerate",
    "requestPayment",
    "cors",
    "encryption",
    "object-lock",
    "logging",
    "replication",
    "website",
    "policyStatus",
    "ownershipControls",
    "publicAccessBlock",
];
const CONFIG_AFTER_VERSIONS: &[&str] = &[
    "analytics",
    "intelligent-tiering",
    "metrics",
    "abac",
    "metadataConfiguration",
    "metadataTable",
    "session",
];

/// `s3:ListBucket` / `s3:ListBucketVersions` with the listing parameters as
/// condition keys. A parameter that was not sent leaves its key absent; an
/// explicit `prefix=` yields the empty string.
fn list_permission(
    action: &'static str,
    bucket: &str,
    query: &HashMap<String, String>,
) -> Permission {
    let mut permission = Permission::bucket(action, bucket);
    for (parameter, key) in [
        ("prefix", "s3:prefix"),
        ("delimiter", "s3:delimiter"),
        ("max-keys", "s3:max-keys"),
    ] {
        if let Some(value) = query.get(parameter) {
            permission.context.push((key.to_string(), value.clone()));
        }
    }
    permission
}

/// Split `x-amz-copy-source` into bucket, key and optional version id.
/// Access-point ARN sources and malformed values yield `None`.
fn parse_copy_source(headers: &HeaderMap) -> Option<(String, String, Option<String>)> {
    let raw = headers.get("x-amz-copy-source")?.to_str().ok()?;
    let (path, version_id) = match raw.split_once("?versionId=") {
        Some((path, version_id)) => (path, Some(percent_decode(version_id))),
        None => (raw, None),
    };
    let path = percent_decode(path);
    let path = path.trim_start_matches('/');
    if path.starts_with("arn:") {
        return None;
    }
    let (bucket, key) = path.split_once('/')?;
    if bucket.is_empty() || key.is_empty() {
        return None;
    }
    Some((bucket.to_string(), key.to_string(), version_id))
}

fn percent_decode(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' {
            if let Some(byte) = value
                .get(i + 1..i + 3)
                .and_then(|hex| u8::from_str_radix(hex, 16).ok())
            {
                out.push(byte);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// Whether the request counts as TLS for `aws:SecureTransport`.
///
/// `x-forwarded-proto`, when present, wins ( its first entry must be `https` );
/// otherwise the scheme of the request URI decides.
pub(crate) fn is_secure_transport(request: &MockRequest) -> bool {
    if let Some(proto) = request
        .headers
        .get(FORWARDED_PROTO_HEADER)
        .and_then(|v| v.to_str().ok())
    {
        return proto
            .split(',')
            .next()
            .is_some_and(|p| p.trim().eq_ignore_ascii_case("https"));
    }
    request
        .uri
        .get(..8)
        .is_some_and(|scheme| scheme.eq_ignore_ascii_case("https://"))
}

/// Build the request-wide condition context ( keys lower-cased ).
pub(crate) fn request_context(
    request: &MockRequest,
    query: &HashMap<String, String>,
) -> SimulationContext {
    let mut context = SimulationContext::default();
    let mut set = |key: &str, value: String| {
        context
            .context_keys
            .entry(key.to_string())
            .or_default()
            .push(value);
    };
    let header = |name: &str| request.headers.get(name).and_then(|v| v.to_str().ok());

    set(
        "aws:securetransport",
        is_secure_transport(request).to_string(),
    );
    let now = chrono::Utc::now();
    set(
        "aws:currenttime",
        now.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
    );
    set("aws:epochtime", now.timestamp().to_string());
    if let Some(address) = header(FORWARDED_FOR_HEADER)
        .and_then(|v| v.split(',').next())
        .map(str::trim)
        .filter(|v| !v.is_empty())
    {
        set("aws:sourceip", address.to_string());
    }

    for (name, key) in HEADER_CONDITION_KEYS {
        if let Some(value) = header(name) {
            set(key, value.to_string());
        }
    }
    if let Some(source) = header("x-amz-copy-source") {
        // Policies spell the source as `bucket/key`.
        set(
            "s3:x-amz-copy-source",
            percent_decode(source).trim_start_matches('/').to_string(),
        );
    }
    if let Some(tagging) = header("x-amz-tagging") {
        for pair in tagging.split('&').filter(|p| !p.is_empty()) {
            let (tag_key, tag_value) = pair.split_once('=').unwrap_or((pair, ""));
            let tag_key = percent_decode(&tag_key.replace('+', " "));
            let tag_value = percent_decode(&tag_value.replace('+', " "));
            set(
                &format!("s3:requestobjecttag/{}", tag_key.to_ascii_lowercase()),
                tag_value,
            );
            set("s3:requestobjecttagkeys", tag_key);
        }
    }

    // Signature facts are read off the request; nothing is verified.
    let authorization = header("authorization");
    if let Some(authorization) = authorization {
        set("s3:authtype", "REST-HEADER".to_string());
        if authorization.starts_with("AWS4-HMAC-SHA256") {
            set("s3:signatureversion", "AWS4-HMAC-SHA256".to_string());
        } else if authorization.starts_with("AWS ") {
            set("s3:signatureversion", "AWS".to_string());
        }
    } else if let Some(algorithm) = query.get("X-Amz-Algorithm") {
        set("s3:authtype", "REST-QUERY-STRING".to_string());
        set("s3:signatureversion", algorithm.clone());
    } else if query.contains_key("Signature") && query.contains_key("AWSAccessKeyId") {
        set("s3:authtype", "REST-QUERY-STRING".to_string());
        set("s3:signatureversion", "AWS".to_string());
    }

    context
}

/// A bucket's policy, parsed once, ready to answer explicit-deny questions.
pub(crate) struct PolicyGuard {
    policies: Vec<(PolicyDocument, PolicySource)>,
    options: EvalOptions,
}

impl PolicyGuard {
    /// `policy_json` is the stored policy text. A missing policy, or one that
    /// does not parse as an IAM policy document, denies nothing ( the latter
    /// is logged ): the emulator stores whatever `PutBucketPolicy` was given,
    /// and an unreadable document must not take request handling down.
    pub(crate) fn new(bucket: &str, policy_json: Option<&str>) -> Self {
        let policies = match policy_json.map(serde_json::from_str::<PolicyDocument>) {
            Some(Ok(document)) => vec![(
                document,
                PolicySource {
                    id: format!("arn:aws:s3:::{bucket}"),
                    policy_type: "ResourcePolicy".to_string(),
                },
            )],
            Some(Err(error)) => {
                tracing::warn!(
                    bucket,
                    %error,
                    "bucket policy is not a valid policy document; not enforcing it"
                );
                Vec::new()
            }
            None => Vec::new(),
        };
        Self {
            policies,
            options: EvalOptions {
                condition_mode: ConditionMode::Evaluate,
                principal_mode: PrincipalMode::WildcardOnly,
                unknowable_context_keys: UNKNOWABLE_CONTEXT_KEYS
                    .iter()
                    .map(|k| (*k).to_string())
                    .collect(),
            },
        }
    }

    /// Whether an applicable `Deny` statement covers `permission`.
    pub(crate) fn denies(&self, permission: &Permission, base: &SimulationContext) -> bool {
        if self.policies.is_empty() {
            return false;
        }
        let mut context = base.clone();
        for (key, value) in &permission.context {
            context
                .context_keys
                .insert(key.clone(), vec![value.clone()]);
        }
        evaluate_with_options(
            &self.policies,
            permission.action,
            &permission.resource_arn(),
            &context,
            &self.options,
        )
        .decision
            == EvalDecision::ExplicitDenied
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn query(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
            .collect()
    }

    fn headers(pairs: &[(&'static str, &str)]) -> HeaderMap {
        let mut map = HeaderMap::new();
        for (name, value) in pairs {
            map.insert(*name, value.parse().unwrap());
        }
        map
    }

    fn actions(
        method: &str,
        key: Option<&str>,
        q: &[(&str, &str)],
        h: &[(&'static str, &str)],
    ) -> Vec<(&'static str, String)> {
        required_permissions(method, "b", key, &query(q), &headers(h))
            .into_iter()
            .map(|p| (p.action, p.resource_arn()))
            .collect()
    }

    fn one(action: &'static str, arn: &str) -> Vec<(&'static str, String)> {
        vec![(action, arn.to_string())]
    }

    const BUCKET: &str = "arn:aws:s3:::b";
    const OBJECT: &str = "arn:aws:s3:::b/dir/k";

    #[test]
    fn bucket_level_mapping() {
        assert_eq!(
            actions("HEAD", None, &[], &[]),
            one("s3:ListBucket", BUCKET)
        );
        assert_eq!(actions("GET", None, &[], &[]), one("s3:ListBucket", BUCKET));
        assert_eq!(
            actions("GET", None, &[("list-type", "2"), ("prefix", "a/")], &[]),
            one("s3:ListBucket", BUCKET)
        );
        assert_eq!(
            actions("GET", None, &[("versions", "")], &[]),
            one("s3:ListBucketVersions", BUCKET)
        );
        assert_eq!(
            actions("GET", None, &[("uploads", "")], &[]),
            one("s3:ListBucketMultipartUploads", BUCKET)
        );
    }

    #[test]
    fn control_plane_is_not_enforced() {
        for sub in [
            "policy",
            "policyStatus",
            "lifecycle",
            "versioning",
            "acl",
            "tagging",
            "cors",
            "encryption",
            "publicAccessBlock",
            "location",
            "session",
        ] {
            for method in ["GET", "PUT", "DELETE"] {
                assert!(
                    actions(method, None, &[(sub, "")], &[]).is_empty(),
                    "{method} ?{sub}"
                );
            }
        }
        // CreateBucket, DeleteBucket, DeleteObjects ( per key in the handler ).
        assert!(actions("PUT", None, &[], &[]).is_empty());
        assert!(actions("DELETE", None, &[], &[]).is_empty());
        assert!(actions("POST", None, &[("delete", "")], &[]).is_empty());
        assert!(actions("PUT", Some("dir/k"), &[("renameObject", "")], &[]).is_empty());
    }

    #[test]
    fn object_read_write_delete_mapping() {
        let k = Some("dir/k");
        let v = [("versionId", "v1")];
        assert_eq!(actions("GET", k, &[], &[]), one("s3:GetObject", OBJECT));
        assert_eq!(
            actions("GET", k, &v, &[]),
            one("s3:GetObjectVersion", OBJECT)
        );
        assert_eq!(actions("HEAD", k, &[], &[]), one("s3:GetObject", OBJECT));
        assert_eq!(
            actions("HEAD", k, &v, &[]),
            one("s3:GetObjectVersion", OBJECT)
        );
        assert_eq!(actions("PUT", k, &[], &[]), one("s3:PutObject", OBJECT));
        assert_eq!(
            actions("DELETE", k, &[], &[]),
            one("s3:DeleteObject", OBJECT)
        );
        assert_eq!(
            actions("DELETE", k, &v, &[]),
            one("s3:DeleteObjectVersion", OBJECT)
        );
        assert_eq!(
            actions("POST", k, &[("select", ""), ("select-type", "2")], &[]),
            one("s3:GetObject", OBJECT)
        );
        assert_eq!(
            actions("POST", k, &[("restore", "")], &[]),
            one("s3:RestoreObject", OBJECT)
        );
        assert_eq!(
            actions("GET", k, &[("torrent", "")], &[]),
            one("s3:GetObjectTorrent", OBJECT)
        );
        assert_eq!(
            actions("PUT", k, &[("encryption", "")], &[]),
            one("s3:UpdateObjectEncryption", OBJECT)
        );
    }

    #[test]
    fn versioned_permission_carries_version_id() {
        let permissions = required_permissions(
            "GET",
            "b",
            Some("k"),
            &query(&[("versionId", "v1")]),
            &HeaderMap::new(),
        );
        assert_eq!(
            permissions[0].context,
            vec![("s3:versionid".to_string(), "v1".to_string())]
        );
    }

    #[test]
    fn sub_resource_mapping() {
        let k = Some("dir/k");
        let v = ("versionId", "v1");
        for (method, sub, plain, versioned) in [
            ("GET", "acl", "s3:GetObjectAcl", "s3:GetObjectVersionAcl"),
            ("PUT", "acl", "s3:PutObjectAcl", "s3:PutObjectVersionAcl"),
            (
                "GET",
                "tagging",
                "s3:GetObjectTagging",
                "s3:GetObjectVersionTagging",
            ),
            (
                "PUT",
                "tagging",
                "s3:PutObjectTagging",
                "s3:PutObjectVersionTagging",
            ),
            (
                "DELETE",
                "tagging",
                "s3:DeleteObjectTagging",
                "s3:DeleteObjectVersionTagging",
            ),
        ] {
            assert_eq!(actions(method, k, &[(sub, "")], &[]), one(plain, OBJECT));
            assert_eq!(
                actions(method, k, &[(sub, ""), v], &[]),
                one(versioned, OBJECT)
            );
        }
        for (method, sub, action) in [
            ("GET", "legal-hold", "s3:GetObjectLegalHold"),
            ("PUT", "legal-hold", "s3:PutObjectLegalHold"),
            ("GET", "retention", "s3:GetObjectRetention"),
            ("PUT", "retention", "s3:PutObjectRetention"),
        ] {
            assert_eq!(actions(method, k, &[(sub, "")], &[]), one(action, OBJECT));
        }
        assert_eq!(
            actions("GET", k, &[("attributes", "")], &[]),
            vec![
                ("s3:GetObject", OBJECT.to_string()),
                ("s3:GetObjectAttributes", OBJECT.to_string())
            ]
        );
        assert_eq!(
            actions("GET", k, &[("attributes", ""), v], &[]),
            vec![
                ("s3:GetObjectVersion", OBJECT.to_string()),
                ("s3:GetObjectVersionAttributes", OBJECT.to_string())
            ]
        );
    }

    #[test]
    fn multipart_mapping() {
        let k = Some("dir/k");
        let part = [("partNumber", "1"), ("uploadId", "u")];
        assert_eq!(
            actions("POST", k, &[("uploads", "")], &[]),
            one("s3:PutObject", OBJECT)
        );
        assert_eq!(actions("PUT", k, &part, &[]), one("s3:PutObject", OBJECT));
        assert_eq!(
            actions("POST", k, &[("uploadId", "u")], &[]),
            one("s3:PutObject", OBJECT)
        );
        assert_eq!(
            actions("DELETE", k, &[("uploadId", "u")], &[]),
            one("s3:AbortMultipartUpload", OBJECT)
        );
        assert_eq!(
            actions("GET", k, &[("uploadId", "u")], &[]),
            one("s3:ListMultipartUploadParts", OBJECT)
        );
        assert_eq!(
            actions("PUT", k, &part, &[("x-amz-copy-source", "src/a%20b")]),
            vec![
                ("s3:GetObject", "arn:aws:s3:::src/a b".to_string()),
                ("s3:PutObject", OBJECT.to_string())
            ]
        );
    }

    #[test]
    fn copy_mapping() {
        let k = Some("dir/k");
        assert_eq!(
            actions(
                "PUT",
                k,
                &[],
                &[("x-amz-copy-source", "/src/path/to%2Fkey")]
            ),
            vec![
                ("s3:GetObject", "arn:aws:s3:::src/path/to/key".to_string()),
                ("s3:PutObject", OBJECT.to_string())
            ]
        );
        let permissions = required_permissions(
            "PUT",
            "b",
            k,
            &HashMap::new(),
            &headers(&[("x-amz-copy-source", "src/key?versionId=v9")]),
        );
        assert_eq!(permissions[0].action, "s3:GetObjectVersion");
        assert_eq!(permissions[0].bucket, "src");
        assert_eq!(permissions[0].key.as_deref(), Some("key"));
        assert_eq!(
            permissions[0].context,
            vec![("s3:versionid".to_string(), "v9".to_string())]
        );
        // An unparseable source still needs the destination write.
        assert_eq!(
            actions("PUT", k, &[], &[("x-amz-copy-source", "nokey")]),
            one("s3:PutObject", OBJECT)
        );
    }

    #[test]
    fn list_parameters_become_condition_keys() {
        let permissions = required_permissions(
            "GET",
            "b",
            None,
            &query(&[("prefix", ""), ("max-keys", "5")]),
            &HeaderMap::new(),
        );
        let mut context = permissions[0].context.clone();
        context.sort();
        assert_eq!(
            context,
            vec![
                ("s3:max-keys".to_string(), "5".to_string()),
                ("s3:prefix".to_string(), String::new()),
            ]
        );
        // Multipart-upload listings do not carry the listing keys.
        let permissions = required_permissions(
            "GET",
            "b",
            None,
            &query(&[("uploads", ""), ("prefix", "a")]),
            &HeaderMap::new(),
        );
        assert!(permissions[0].context.is_empty());
    }

    fn request(uri: &str, h: &[(&'static str, &str)]) -> MockRequest {
        MockRequest {
            method: "GET".to_string(),
            uri: uri.to_string(),
            headers: headers(h),
            body: bytes::Bytes::new(),
        }
    }

    #[test]
    fn secure_transport_signal() {
        assert!(is_secure_transport(&request(
            "https://s3.amazonaws.com/b",
            &[]
        )));
        assert!(!is_secure_transport(&request(
            "http://localhost:4566/b",
            &[]
        )));
        assert!(!is_secure_transport(&request(
            "https://s3.amazonaws.com/b",
            &[("x-forwarded-proto", "http")]
        )));
        assert!(is_secure_transport(&request(
            "http://localhost/b",
            &[("x-forwarded-proto", "HTTPS, http")]
        )));
    }

    #[test]
    fn request_context_keys() {
        let context = request_context(
            &request(
                "http://localhost/b/k",
                &[
                    ("x-amz-acl", "public-read"),
                    ("x-amz-server-side-encryption", "aws:kms"),
                    ("x-amz-storage-class", "GLACIER"),
                    ("x-amz-copy-source", "/src/a%20b"),
                    ("x-amz-tagging", "Env=prod&team=a+b"),
                    ("x-forwarded-for", "203.0.113.9, 10.0.0.1"),
                    ("authorization", "AWS4-HMAC-SHA256 Credential=x"),
                ],
            ),
            &HashMap::new(),
        );
        let get = |key: &str| context.context_keys.get(key).cloned().unwrap_or_default();
        assert_eq!(get("aws:securetransport"), ["false"]);
        assert_eq!(get("s3:x-amz-acl"), ["public-read"]);
        assert_eq!(get("s3:x-amz-server-side-encryption"), ["aws:kms"]);
        assert_eq!(get("s3:x-amz-storage-class"), ["GLACIER"]);
        assert_eq!(get("s3:x-amz-copy-source"), ["src/a b"]);
        assert_eq!(get("s3:requestobjecttag/env"), ["prod"]);
        assert_eq!(get("s3:requestobjecttag/team"), ["a b"]);
        assert_eq!(get("s3:requestobjecttagkeys"), ["Env", "team"]);
        assert_eq!(get("aws:sourceip"), ["203.0.113.9"]);
        assert_eq!(get("s3:authtype"), ["REST-HEADER"]);
        assert_eq!(get("s3:signatureversion"), ["AWS4-HMAC-SHA256"]);
        assert!(!get("aws:currenttime").is_empty());
        assert!(!context.context_keys.contains_key("s3:prefix"));
    }

    #[test]
    fn guard_tolerates_missing_and_malformed_policies() {
        let permission = Permission::object("s3:PutObject", "b", "k");
        let context = SimulationContext::default();
        assert!(!PolicyGuard::new("b", None).denies(&permission, &context));
        assert!(!PolicyGuard::new("b", Some("not json")).denies(&permission, &context));
        assert!(!PolicyGuard::new("b", Some(r#"{"Statement": 7}"#)).denies(&permission, &context));
        assert!(
            PolicyGuard::new(
                "b",
                Some(
                    r#"{"Statement":[{"Effect":"Deny","Principal":"*","Action":"s3:PutObject","Resource":"arn:aws:s3:::b/*"}]}"#
                )
            )
            .denies(&permission, &context)
        );
    }
}
