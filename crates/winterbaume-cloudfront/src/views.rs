//! Serde-compatible view types for CloudFront state snapshots.

/// Serde helper module for serialising `Vec<u8>` as lowercase hex strings.
mod hex_serde {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let hex_str: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        serializer.serialize_str(&hex_str)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.len() % 2 != 0 {
            return Err(serde::de::Error::custom("hex string has odd length"));
        }
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(serde::de::Error::custom))
            .collect()
    }
}

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::CloudFrontService;
use crate::state::CloudFrontState;
use crate::types::{
    AnycastIpListData, CachePolicyData, CloudFrontFunctionData, ConnectionFunctionData,
    ConnectionGroupData, ContinuousDeploymentPolicyData, DefaultCacheBehavior, Distribution,
    DistributionTenantData, FieldLevelEncryptionData, FieldLevelEncryptionProfileData,
    Invalidation, KeyGroupData, KeyValueStoreData, MonitoringSubscriptionData, OaiData, Origin,
    OriginAccessControlData, OriginRequestPolicyData, PublicKeyData, RealtimeLogConfigData,
    ResourcePolicyData, ResponseHeadersPolicyData, StreamingDistributionData, TrustStoreData,
    VpcOriginData, WebAclAssociation,
};

/// Serializable view of the entire CloudFront state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CloudFrontStateView {
    /// Distributions keyed by distribution ID.
    #[serde(default)]
    pub distributions: HashMap<String, DistributionView>,
    /// Invalidations keyed by distribution ID.
    #[serde(default)]
    pub invalidations: HashMap<String, Vec<InvalidationView>>,
    /// Key groups keyed by key group ID.
    #[serde(default)]
    pub key_groups: HashMap<String, KeyGroupView>,
    /// Origin access controls keyed by OAC ID.
    #[serde(default)]
    pub origin_access_controls: HashMap<String, OriginAccessControlView>,
    /// Public keys keyed by public key ID.
    #[serde(default)]
    pub public_keys: HashMap<String, PublicKeyView>,
    /// Cache policies keyed by policy ID.
    #[serde(default)]
    pub cache_policies: HashMap<String, CachePolicyView>,
    /// Origin request policies keyed by policy ID.
    #[serde(default)]
    pub origin_request_policies: HashMap<String, OriginRequestPolicyView>,
    /// Response headers policies keyed by policy ID.
    #[serde(default)]
    pub response_headers_policies: HashMap<String, ResponseHeadersPolicyView>,
    /// CloudFront functions keyed by function name.
    #[serde(default)]
    pub functions: HashMap<String, CloudFrontFunctionView>,
    /// Anycast IP lists keyed by list ID.
    #[serde(default)]
    pub anycast_ip_lists: HashMap<String, AnycastIpListView>,
    /// Origin Access Identities keyed by OAI ID.
    #[serde(default)]
    pub oais: HashMap<String, OaiView>,
    /// Streaming distributions keyed by distribution ID.
    #[serde(default)]
    pub streaming_distributions: HashMap<String, StreamingDistributionView>,
    /// Key-value stores keyed by store name.
    #[serde(default)]
    pub key_value_stores: HashMap<String, KeyValueStoreView>,
    /// Trust stores keyed by trust store ID.
    #[serde(default)]
    pub trust_stores: HashMap<String, TrustStoreView>,
    /// VPC origins keyed by VPC origin ID.
    #[serde(default)]
    pub vpc_origins: HashMap<String, VpcOriginView>,
    /// Realtime log configs keyed by config ARN.
    #[serde(default)]
    pub realtime_log_configs: HashMap<String, RealtimeLogConfigView>,
    /// Field-level encryption configurations keyed by config ID.
    #[serde(default)]
    pub field_level_encryptions: HashMap<String, FieldLevelEncryptionView>,
    /// Field-level encryption profiles keyed by profile ID.
    #[serde(default)]
    pub field_level_encryption_profiles: HashMap<String, FieldLevelEncryptionProfileView>,
    /// Connection functions keyed by function ID.
    #[serde(default)]
    pub connection_functions: HashMap<String, ConnectionFunctionView>,
    /// Connection groups keyed by group ID.
    #[serde(default)]
    pub connection_groups: HashMap<String, ConnectionGroupView>,
    /// Continuous deployment policies keyed by policy ID.
    #[serde(default)]
    pub continuous_deployment_policies: HashMap<String, ContinuousDeploymentPolicyView>,
    /// Distribution tenants keyed by tenant ID.
    #[serde(default)]
    pub distribution_tenants: HashMap<String, DistributionTenantView>,
    /// Monitoring subscriptions keyed by distribution ID.
    #[serde(default)]
    pub monitoring_subscriptions: HashMap<String, MonitoringSubscriptionView>,
    /// Resource policies keyed by resource ARN.
    #[serde(default)]
    pub resource_policies: HashMap<String, ResourcePolicyView>,
    /// WebACL associations keyed by resource ID.
    #[serde(default)]
    pub web_acl_associations: HashMap<String, WebAclAssociationView>,
}

/// Serializable view of a CloudFront distribution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionView {
    pub id: String,
    pub arn: String,
    pub domain_name: String,
    pub status: String,
    pub origins: Vec<OriginView>,
    pub default_cache_behavior: DefaultCacheBehaviorView,
    pub caller_reference: String,
    pub created_at: Option<String>,
    pub etag: String,
    pub enabled: bool,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    /// Full DistributionConfig stored for faithful echo-back on Get/List.
    #[serde(default)]
    pub raw_config: Option<crate::model::DistributionConfig>,
    /// `custom_error_response` nested blocks.
    #[serde(default)]
    pub custom_error_response: Vec<serde_json::Value>,
    /// `logging_config` nested block.
    #[serde(default)]
    pub logging_config: Option<serde_json::Value>,
    /// `ordered_cache_behavior` nested blocks.
    #[serde(default)]
    pub ordered_cache_behavior: Vec<serde_json::Value>,
    /// `origin_group` nested blocks.
    #[serde(default)]
    pub origin_group: Vec<serde_json::Value>,
}

/// Serializable view of a CloudFront origin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OriginView {
    pub id: String,
    pub domain_name: String,
}

/// Serializable view of a default cache behavior.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultCacheBehaviorView {
    pub target_origin_id: String,
    pub viewer_protocol_policy: String,
    #[serde(default)]
    pub allowed_methods: Vec<String>,
    #[serde(default)]
    pub cached_methods: Vec<String>,
    #[serde(default)]
    pub forwarded_values_query_string: bool,
    #[serde(default = "default_cookies_forward")]
    pub forwarded_values_cookies_forward: String,
    #[serde(default)]
    pub compress: bool,
}

fn default_cookies_forward() -> String {
    "none".to_string()
}

/// Serializable view of a CloudFront invalidation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvalidationView {
    pub id: String,
    pub distribution_id: String,
    pub status: String,
    pub create_time: Option<String>,
    pub caller_reference: String,
    pub paths: Vec<String>,
}

/// Serializable view of a CloudFront key group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyGroupView {
    pub id: String,
    pub name: String,
    pub comment: Option<String>,
    pub items: Vec<String>,
    pub last_modified_time: Option<String>,
    pub etag: String,
}

/// Serializable view of a CloudFront origin access control.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OriginAccessControlView {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub origin_access_control_origin_type: String,
    pub signing_behavior: String,
    pub signing_protocol: String,
    pub etag: String,
}

/// Serializable view of a CloudFront public key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicKeyView {
    pub id: String,
    pub name: String,
    pub caller_reference: String,
    pub encoded_key: String,
    pub comment: Option<String>,
    pub created_time: Option<String>,
    pub etag: String,
}

/// Serializable view of a CloudFront cache policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachePolicyView {
    pub id: String,
    pub last_modified_time: Option<String>,
    pub config: crate::model::CachePolicyConfig,
    pub etag: String,
}

/// Serializable view of a CloudFront origin request policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OriginRequestPolicyView {
    pub id: String,
    pub last_modified_time: Option<String>,
    pub config: crate::model::OriginRequestPolicyConfig,
    pub etag: String,
}

/// Serializable view of a CloudFront response headers policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseHeadersPolicyView {
    pub id: String,
    pub last_modified_time: Option<String>,
    pub config: crate::model::ResponseHeadersPolicyConfig,
    pub etag: String,
}

/// Serializable view of a CloudFront function.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudFrontFunctionView {
    pub name: String,
    pub arn: String,
    pub status: String,
    pub created_time: Option<String>,
    pub last_modified_time: Option<String>,
    pub config: crate::model::FunctionConfig,
    #[serde(with = "hex_serde")]
    pub code: Vec<u8>,
    pub etag: String,
}

/// Serializable view of a CloudFront anycast IP list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnycastIpListView {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub status: String,
    pub ip_count: i32,
    #[serde(default)]
    pub ip_address_type: Option<String>,
    pub last_modified_time: Option<String>,
    pub etag: String,
}

/// Serializable view of a CloudFront Origin Access Identity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OaiView {
    pub id: String,
    pub caller_reference: String,
    pub comment: String,
    pub s3_canonical_user_id: String,
    pub etag: String,
}

/// Serializable view of a CloudFront streaming distribution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingDistributionView {
    pub id: String,
    pub arn: String,
    pub domain_name: String,
    pub status: String,
    pub last_modified_time: Option<String>,
    pub config: crate::model::StreamingDistributionConfig,
    pub etag: String,
}

/// Serializable view of a CloudFront key-value store.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyValueStoreView {
    pub name: String,
    pub arn: String,
    pub id: String,
    pub comment: Option<String>,
    pub last_modified_time: Option<String>,
    pub status: String,
    pub etag: String,
}

/// Serializable view of a CloudFront trust store.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustStoreView {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub last_modified_time: Option<String>,
    pub status: String,
    pub number_of_ca_certificates: i32,
    pub etag: String,
}

/// Serializable view of a CloudFront VPC origin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcOriginView {
    pub id: String,
    pub arn: String,
    pub account_id: String,
    pub created_time: Option<String>,
    pub last_modified_time: Option<String>,
    pub status: String,
    pub config: crate::model::VpcOriginEndpointConfig,
    pub etag: String,
}

/// Serializable view of a CloudFront realtime log config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeLogConfigView {
    pub name: String,
    pub arn: String,
    pub sampling_rate: i64,
    #[serde(default)]
    pub end_points: Vec<crate::model::EndPoint>,
    #[serde(default)]
    pub fields: Vec<String>,
}

/// Serializable view of a CloudFront field-level encryption configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldLevelEncryptionView {
    pub id: String,
    pub last_modified_time: Option<String>,
    pub config: crate::model::FieldLevelEncryptionConfig,
    pub etag: String,
}

/// Serializable view of a CloudFront field-level encryption profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldLevelEncryptionProfileView {
    pub id: String,
    pub last_modified_time: Option<String>,
    pub config: crate::model::FieldLevelEncryptionProfileConfig,
    pub etag: String,
}

/// Serializable view of a CloudFront connection function.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionFunctionView {
    pub id: String,
    pub name: String,
    pub arn: String,
    pub status: String,
    pub stage: String,
    pub created_time: Option<String>,
    pub last_modified_time: Option<String>,
    pub config: crate::model::FunctionConfig,
    pub code: String,
    pub etag: String,
}

/// Serializable view of a CloudFront connection group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionGroupView {
    pub id: String,
    pub name: String,
    pub arn: String,
    pub status: String,
    pub routing_endpoint: String,
    pub enabled: bool,
    pub ipv6_enabled: bool,
    pub is_default: bool,
    #[serde(default)]
    pub anycast_ip_list_id: Option<String>,
    pub created_time: Option<String>,
    pub last_modified_time: Option<String>,
    pub etag: String,
}

/// Serializable view of a CloudFront continuous deployment policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinuousDeploymentPolicyView {
    pub id: String,
    pub last_modified_time: Option<String>,
    pub config: crate::model::ContinuousDeploymentPolicyConfig,
    pub etag: String,
}

/// Serializable view of a CloudFront distribution tenant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionTenantView {
    pub id: String,
    pub name: String,
    pub arn: String,
    pub distribution_id: String,
    #[serde(default)]
    pub connection_group_id: Option<String>,
    pub enabled: bool,
    pub status: String,
    pub domains: crate::model::DomainResultList,
    #[serde(default)]
    pub customizations: Option<crate::model::Customizations>,
    #[serde(default)]
    pub parameters: Option<crate::model::Parameters>,
    pub created_time: Option<String>,
    pub last_modified_time: Option<String>,
    pub etag: String,
}

/// Serializable view of a CloudFront monitoring subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringSubscriptionView {
    pub distribution_id: String,
    pub realtime_metrics_subscription_status: String,
}

/// Serializable view of a CloudFront resource policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcePolicyView {
    pub resource_arn: String,
    pub policy_document: String,
}

/// Serializable view of a CloudFront WebACL association.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAclAssociationView {
    pub resource_id: String,
    pub web_acl_arn: String,
}

// ---------------------------------------------------------------------------
// From conversions
// ---------------------------------------------------------------------------

impl From<&Distribution> for DistributionView {
    fn from(d: &Distribution) -> Self {
        DistributionView {
            id: d.id.clone(),
            arn: d.arn.clone(),
            domain_name: d.domain_name.clone(),
            status: d.status.clone(),
            origins: d.origins.iter().map(OriginView::from).collect(),
            default_cache_behavior: DefaultCacheBehaviorView::from(&d.default_cache_behavior),
            caller_reference: d.caller_reference.clone(),
            created_at: Some(d.created_at.to_rfc3339()),
            etag: d.etag.clone(),
            enabled: d.enabled,
            tags: d.tags.clone(),
            raw_config: Some(d.raw_config.clone()),
            custom_error_response: vec![],
            logging_config: None,
            ordered_cache_behavior: vec![],
            origin_group: vec![],
        }
    }
}

impl From<DistributionView> for Distribution {
    fn from(v: DistributionView) -> Self {
        let created_at = v
            .created_at
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        Distribution {
            id: v.id,
            arn: v.arn,
            domain_name: v.domain_name,
            status: v.status,
            origins: v.origins.into_iter().map(Origin::from).collect(),
            default_cache_behavior: DefaultCacheBehavior::from(v.default_cache_behavior),
            caller_reference: v.caller_reference,
            created_at,
            etag: v.etag,
            enabled: v.enabled,
            tags: v.tags,
            raw_config: v.raw_config.unwrap_or_default(),
        }
    }
}

impl From<&Origin> for OriginView {
    fn from(o: &Origin) -> Self {
        OriginView {
            id: o.id.clone(),
            domain_name: o.domain_name.clone(),
        }
    }
}

impl From<OriginView> for Origin {
    fn from(v: OriginView) -> Self {
        Origin {
            id: v.id,
            domain_name: v.domain_name,
        }
    }
}

impl From<&DefaultCacheBehavior> for DefaultCacheBehaviorView {
    fn from(d: &DefaultCacheBehavior) -> Self {
        DefaultCacheBehaviorView {
            target_origin_id: d.target_origin_id.clone(),
            viewer_protocol_policy: d.viewer_protocol_policy.clone(),
            allowed_methods: d.allowed_methods.clone(),
            cached_methods: d.cached_methods.clone(),
            forwarded_values_query_string: d.forwarded_values_query_string,
            forwarded_values_cookies_forward: d.forwarded_values_cookies_forward.clone(),
            compress: d.compress,
        }
    }
}

impl From<DefaultCacheBehaviorView> for DefaultCacheBehavior {
    fn from(v: DefaultCacheBehaviorView) -> Self {
        DefaultCacheBehavior {
            target_origin_id: v.target_origin_id,
            viewer_protocol_policy: v.viewer_protocol_policy,
            allowed_methods: v.allowed_methods,
            cached_methods: v.cached_methods,
            forwarded_values_query_string: v.forwarded_values_query_string,
            forwarded_values_cookies_forward: v.forwarded_values_cookies_forward,
            compress: v.compress,
        }
    }
}

impl From<&Invalidation> for InvalidationView {
    fn from(i: &Invalidation) -> Self {
        InvalidationView {
            id: i.id.clone(),
            distribution_id: i.distribution_id.clone(),
            status: i.status.clone(),
            create_time: Some(i.create_time.to_rfc3339()),
            caller_reference: i.caller_reference.clone(),
            paths: i.paths.clone(),
        }
    }
}

impl From<InvalidationView> for Invalidation {
    fn from(v: InvalidationView) -> Self {
        let create_time = v
            .create_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        Invalidation {
            id: v.id,
            distribution_id: v.distribution_id,
            status: v.status,
            create_time,
            caller_reference: v.caller_reference,
            paths: v.paths,
        }
    }
}

impl From<&KeyGroupData> for KeyGroupView {
    fn from(kg: &KeyGroupData) -> Self {
        KeyGroupView {
            id: kg.id.clone(),
            name: kg.name.clone(),
            comment: kg.comment.clone(),
            items: kg.items.clone(),
            last_modified_time: Some(kg.last_modified_time.to_rfc3339()),
            etag: kg.etag.clone(),
        }
    }
}

impl From<KeyGroupView> for KeyGroupData {
    fn from(v: KeyGroupView) -> Self {
        let last_modified_time = v
            .last_modified_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        KeyGroupData {
            id: v.id,
            name: v.name,
            comment: v.comment,
            items: v.items,
            last_modified_time,
            etag: v.etag,
        }
    }
}

impl From<&OriginAccessControlData> for OriginAccessControlView {
    fn from(oac: &OriginAccessControlData) -> Self {
        OriginAccessControlView {
            id: oac.id.clone(),
            name: oac.name.clone(),
            description: oac.description.clone(),
            origin_access_control_origin_type: oac.origin_access_control_origin_type.clone(),
            signing_behavior: oac.signing_behavior.clone(),
            signing_protocol: oac.signing_protocol.clone(),
            etag: oac.etag.clone(),
        }
    }
}

impl From<OriginAccessControlView> for OriginAccessControlData {
    fn from(v: OriginAccessControlView) -> Self {
        OriginAccessControlData {
            id: v.id,
            name: v.name,
            description: v.description,
            origin_access_control_origin_type: v.origin_access_control_origin_type,
            signing_behavior: v.signing_behavior,
            signing_protocol: v.signing_protocol,
            etag: v.etag,
        }
    }
}

impl From<&PublicKeyData> for PublicKeyView {
    fn from(pk: &PublicKeyData) -> Self {
        PublicKeyView {
            id: pk.id.clone(),
            name: pk.name.clone(),
            caller_reference: pk.caller_reference.clone(),
            encoded_key: pk.encoded_key.clone(),
            comment: pk.comment.clone(),
            created_time: Some(pk.created_time.to_rfc3339()),
            etag: pk.etag.clone(),
        }
    }
}

impl From<PublicKeyView> for PublicKeyData {
    fn from(v: PublicKeyView) -> Self {
        let created_time = v
            .created_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        PublicKeyData {
            id: v.id,
            name: v.name,
            caller_reference: v.caller_reference,
            encoded_key: v.encoded_key,
            comment: v.comment,
            created_time,
            etag: v.etag,
        }
    }
}

impl From<&CachePolicyData> for CachePolicyView {
    fn from(cp: &CachePolicyData) -> Self {
        CachePolicyView {
            id: cp.id.clone(),
            last_modified_time: Some(cp.last_modified_time.to_rfc3339()),
            config: cp.config.clone(),
            etag: cp.etag.clone(),
        }
    }
}

impl From<CachePolicyView> for CachePolicyData {
    fn from(v: CachePolicyView) -> Self {
        let last_modified_time = v
            .last_modified_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        CachePolicyData {
            id: v.id,
            last_modified_time,
            config: v.config,
            etag: v.etag,
        }
    }
}

impl From<&OriginRequestPolicyData> for OriginRequestPolicyView {
    fn from(orp: &OriginRequestPolicyData) -> Self {
        OriginRequestPolicyView {
            id: orp.id.clone(),
            last_modified_time: Some(orp.last_modified_time.to_rfc3339()),
            config: orp.config.clone(),
            etag: orp.etag.clone(),
        }
    }
}

impl From<OriginRequestPolicyView> for OriginRequestPolicyData {
    fn from(v: OriginRequestPolicyView) -> Self {
        let last_modified_time = v
            .last_modified_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        OriginRequestPolicyData {
            id: v.id,
            last_modified_time,
            config: v.config,
            etag: v.etag,
        }
    }
}

impl From<&ResponseHeadersPolicyData> for ResponseHeadersPolicyView {
    fn from(rhp: &ResponseHeadersPolicyData) -> Self {
        ResponseHeadersPolicyView {
            id: rhp.id.clone(),
            last_modified_time: Some(rhp.last_modified_time.to_rfc3339()),
            config: rhp.config.clone(),
            etag: rhp.etag.clone(),
        }
    }
}

impl From<ResponseHeadersPolicyView> for ResponseHeadersPolicyData {
    fn from(v: ResponseHeadersPolicyView) -> Self {
        let last_modified_time = v
            .last_modified_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        ResponseHeadersPolicyData {
            id: v.id,
            last_modified_time,
            config: v.config,
            etag: v.etag,
        }
    }
}

impl From<&CloudFrontFunctionData> for CloudFrontFunctionView {
    fn from(f: &CloudFrontFunctionData) -> Self {
        CloudFrontFunctionView {
            name: f.name.clone(),
            arn: f.arn.clone(),
            status: f.status.clone(),
            created_time: Some(f.created_time.to_rfc3339()),
            last_modified_time: Some(f.last_modified_time.to_rfc3339()),
            config: f.config.clone(),
            code: f.code.clone(),
            etag: f.etag.clone(),
        }
    }
}

impl From<CloudFrontFunctionView> for CloudFrontFunctionData {
    fn from(v: CloudFrontFunctionView) -> Self {
        let created_time = v
            .created_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        let last_modified_time = v
            .last_modified_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        CloudFrontFunctionData {
            name: v.name,
            arn: v.arn,
            status: v.status,
            created_time,
            last_modified_time,
            config: v.config,
            code: v.code,
            etag: v.etag,
        }
    }
}

impl From<&AnycastIpListData> for AnycastIpListView {
    fn from(l: &AnycastIpListData) -> Self {
        AnycastIpListView {
            id: l.id.clone(),
            arn: l.arn.clone(),
            name: l.name.clone(),
            status: l.status.clone(),
            ip_count: l.ip_count,
            ip_address_type: l.ip_address_type.clone(),
            last_modified_time: Some(l.last_modified_time.to_rfc3339()),
            etag: l.etag.clone(),
        }
    }
}

impl From<AnycastIpListView> for AnycastIpListData {
    fn from(v: AnycastIpListView) -> Self {
        let last_modified_time = v
            .last_modified_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        AnycastIpListData {
            id: v.id,
            arn: v.arn,
            name: v.name,
            status: v.status,
            ip_count: v.ip_count,
            ip_address_type: v.ip_address_type,
            last_modified_time,
            etag: v.etag,
        }
    }
}

impl From<&OaiData> for OaiView {
    fn from(o: &OaiData) -> Self {
        OaiView {
            id: o.id.clone(),
            caller_reference: o.caller_reference.clone(),
            comment: o.comment.clone(),
            s3_canonical_user_id: o.s3_canonical_user_id.clone(),
            etag: o.etag.clone(),
        }
    }
}

impl From<OaiView> for OaiData {
    fn from(v: OaiView) -> Self {
        OaiData {
            id: v.id,
            caller_reference: v.caller_reference,
            comment: v.comment,
            s3_canonical_user_id: v.s3_canonical_user_id,
            etag: v.etag,
        }
    }
}

impl From<&StreamingDistributionData> for StreamingDistributionView {
    fn from(s: &StreamingDistributionData) -> Self {
        StreamingDistributionView {
            id: s.id.clone(),
            arn: s.arn.clone(),
            domain_name: s.domain_name.clone(),
            status: s.status.clone(),
            last_modified_time: Some(s.last_modified_time.to_rfc3339()),
            config: s.config.clone(),
            etag: s.etag.clone(),
        }
    }
}

impl From<StreamingDistributionView> for StreamingDistributionData {
    fn from(v: StreamingDistributionView) -> Self {
        let last_modified_time = v
            .last_modified_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        StreamingDistributionData {
            id: v.id,
            arn: v.arn,
            domain_name: v.domain_name,
            status: v.status,
            last_modified_time,
            config: v.config,
            etag: v.etag,
        }
    }
}

impl From<&KeyValueStoreData> for KeyValueStoreView {
    fn from(k: &KeyValueStoreData) -> Self {
        KeyValueStoreView {
            name: k.name.clone(),
            arn: k.arn.clone(),
            id: k.id.clone(),
            comment: k.comment.clone(),
            last_modified_time: Some(k.last_modified_time.to_rfc3339()),
            status: k.status.clone(),
            etag: k.etag.clone(),
        }
    }
}

impl From<KeyValueStoreView> for KeyValueStoreData {
    fn from(v: KeyValueStoreView) -> Self {
        let last_modified_time = v
            .last_modified_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        KeyValueStoreData {
            name: v.name,
            arn: v.arn,
            id: v.id,
            comment: v.comment,
            last_modified_time,
            status: v.status,
            etag: v.etag,
        }
    }
}

impl From<&TrustStoreData> for TrustStoreView {
    fn from(t: &TrustStoreData) -> Self {
        TrustStoreView {
            id: t.id.clone(),
            arn: t.arn.clone(),
            name: t.name.clone(),
            last_modified_time: Some(t.last_modified_time.to_rfc3339()),
            status: t.status.clone(),
            number_of_ca_certificates: t.number_of_ca_certificates,
            etag: t.etag.clone(),
        }
    }
}

impl From<TrustStoreView> for TrustStoreData {
    fn from(v: TrustStoreView) -> Self {
        let last_modified_time = v
            .last_modified_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        TrustStoreData {
            id: v.id,
            arn: v.arn,
            name: v.name,
            last_modified_time,
            status: v.status,
            number_of_ca_certificates: v.number_of_ca_certificates,
            etag: v.etag,
        }
    }
}

impl From<&VpcOriginData> for VpcOriginView {
    fn from(vpc: &VpcOriginData) -> Self {
        VpcOriginView {
            id: vpc.id.clone(),
            arn: vpc.arn.clone(),
            account_id: vpc.account_id.clone(),
            created_time: Some(vpc.created_time.to_rfc3339()),
            last_modified_time: Some(vpc.last_modified_time.to_rfc3339()),
            status: vpc.status.clone(),
            config: vpc.config.clone(),
            etag: vpc.etag.clone(),
        }
    }
}

impl From<VpcOriginView> for VpcOriginData {
    fn from(v: VpcOriginView) -> Self {
        let created_time = v
            .created_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        let last_modified_time = v
            .last_modified_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        VpcOriginData {
            id: v.id,
            arn: v.arn,
            account_id: v.account_id,
            created_time,
            last_modified_time,
            status: v.status,
            config: v.config,
            etag: v.etag,
        }
    }
}

impl From<&RealtimeLogConfigData> for RealtimeLogConfigView {
    fn from(r: &RealtimeLogConfigData) -> Self {
        RealtimeLogConfigView {
            name: r.name.clone(),
            arn: r.arn.clone(),
            sampling_rate: r.sampling_rate,
            end_points: r.end_points.clone(),
            fields: r.fields.clone(),
        }
    }
}

impl From<RealtimeLogConfigView> for RealtimeLogConfigData {
    fn from(v: RealtimeLogConfigView) -> Self {
        RealtimeLogConfigData {
            name: v.name,
            arn: v.arn,
            sampling_rate: v.sampling_rate,
            end_points: v.end_points,
            fields: v.fields,
        }
    }
}

impl From<&FieldLevelEncryptionData> for FieldLevelEncryptionView {
    fn from(f: &FieldLevelEncryptionData) -> Self {
        FieldLevelEncryptionView {
            id: f.id.clone(),
            last_modified_time: Some(f.last_modified_time.to_rfc3339()),
            config: f.config.clone(),
            etag: f.etag.clone(),
        }
    }
}

impl From<FieldLevelEncryptionView> for FieldLevelEncryptionData {
    fn from(v: FieldLevelEncryptionView) -> Self {
        let last_modified_time = v
            .last_modified_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        FieldLevelEncryptionData {
            id: v.id,
            last_modified_time,
            config: v.config,
            etag: v.etag,
        }
    }
}

impl From<&FieldLevelEncryptionProfileData> for FieldLevelEncryptionProfileView {
    fn from(f: &FieldLevelEncryptionProfileData) -> Self {
        FieldLevelEncryptionProfileView {
            id: f.id.clone(),
            last_modified_time: Some(f.last_modified_time.to_rfc3339()),
            config: f.config.clone(),
            etag: f.etag.clone(),
        }
    }
}

impl From<FieldLevelEncryptionProfileView> for FieldLevelEncryptionProfileData {
    fn from(v: FieldLevelEncryptionProfileView) -> Self {
        let last_modified_time = v
            .last_modified_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        FieldLevelEncryptionProfileData {
            id: v.id,
            last_modified_time,
            config: v.config,
            etag: v.etag,
        }
    }
}

impl From<&ConnectionFunctionData> for ConnectionFunctionView {
    fn from(f: &ConnectionFunctionData) -> Self {
        ConnectionFunctionView {
            id: f.id.clone(),
            name: f.name.clone(),
            arn: f.arn.clone(),
            status: f.status.clone(),
            stage: f.stage.clone(),
            created_time: Some(f.created_time.to_rfc3339()),
            last_modified_time: Some(f.last_modified_time.to_rfc3339()),
            config: f.config.clone(),
            code: f.code.clone(),
            etag: f.etag.clone(),
        }
    }
}

impl From<ConnectionFunctionView> for ConnectionFunctionData {
    fn from(v: ConnectionFunctionView) -> Self {
        let created_time = v
            .created_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        let last_modified_time = v
            .last_modified_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        ConnectionFunctionData {
            id: v.id,
            name: v.name,
            arn: v.arn,
            status: v.status,
            stage: v.stage,
            created_time,
            last_modified_time,
            config: v.config,
            code: v.code,
            etag: v.etag,
        }
    }
}

impl From<&ConnectionGroupData> for ConnectionGroupView {
    fn from(cg: &ConnectionGroupData) -> Self {
        ConnectionGroupView {
            id: cg.id.clone(),
            name: cg.name.clone(),
            arn: cg.arn.clone(),
            status: cg.status.clone(),
            routing_endpoint: cg.routing_endpoint.clone(),
            enabled: cg.enabled,
            ipv6_enabled: cg.ipv6_enabled,
            is_default: cg.is_default,
            anycast_ip_list_id: cg.anycast_ip_list_id.clone(),
            created_time: Some(cg.created_time.to_rfc3339()),
            last_modified_time: Some(cg.last_modified_time.to_rfc3339()),
            etag: cg.etag.clone(),
        }
    }
}

impl From<ConnectionGroupView> for ConnectionGroupData {
    fn from(v: ConnectionGroupView) -> Self {
        let created_time = v
            .created_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        let last_modified_time = v
            .last_modified_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        ConnectionGroupData {
            id: v.id,
            name: v.name,
            arn: v.arn,
            status: v.status,
            routing_endpoint: v.routing_endpoint,
            enabled: v.enabled,
            ipv6_enabled: v.ipv6_enabled,
            is_default: v.is_default,
            anycast_ip_list_id: v.anycast_ip_list_id,
            created_time,
            last_modified_time,
            etag: v.etag,
        }
    }
}

impl From<&ContinuousDeploymentPolicyData> for ContinuousDeploymentPolicyView {
    fn from(cdp: &ContinuousDeploymentPolicyData) -> Self {
        ContinuousDeploymentPolicyView {
            id: cdp.id.clone(),
            last_modified_time: Some(cdp.last_modified_time.to_rfc3339()),
            config: cdp.config.clone(),
            etag: cdp.etag.clone(),
        }
    }
}

impl From<ContinuousDeploymentPolicyView> for ContinuousDeploymentPolicyData {
    fn from(v: ContinuousDeploymentPolicyView) -> Self {
        let last_modified_time = v
            .last_modified_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        ContinuousDeploymentPolicyData {
            id: v.id,
            last_modified_time,
            config: v.config,
            etag: v.etag,
        }
    }
}

impl From<&DistributionTenantData> for DistributionTenantView {
    fn from(dt: &DistributionTenantData) -> Self {
        DistributionTenantView {
            id: dt.id.clone(),
            name: dt.name.clone(),
            arn: dt.arn.clone(),
            distribution_id: dt.distribution_id.clone(),
            connection_group_id: dt.connection_group_id.clone(),
            enabled: dt.enabled,
            status: dt.status.clone(),
            domains: dt.domains.clone(),
            customizations: dt.customizations.clone(),
            parameters: dt.parameters.clone(),
            created_time: Some(dt.created_time.to_rfc3339()),
            last_modified_time: Some(dt.last_modified_time.to_rfc3339()),
            etag: dt.etag.clone(),
        }
    }
}

impl From<DistributionTenantView> for DistributionTenantData {
    fn from(v: DistributionTenantView) -> Self {
        let created_time = v
            .created_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        let last_modified_time = v
            .last_modified_time
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        DistributionTenantData {
            id: v.id,
            name: v.name,
            arn: v.arn,
            distribution_id: v.distribution_id,
            connection_group_id: v.connection_group_id,
            enabled: v.enabled,
            status: v.status,
            domains: v.domains,
            customizations: v.customizations,
            parameters: v.parameters,
            created_time,
            last_modified_time,
            etag: v.etag,
        }
    }
}

impl From<&MonitoringSubscriptionData> for MonitoringSubscriptionView {
    fn from(ms: &MonitoringSubscriptionData) -> Self {
        MonitoringSubscriptionView {
            distribution_id: ms.distribution_id.clone(),
            realtime_metrics_subscription_status: ms.realtime_metrics_subscription_status.clone(),
        }
    }
}

impl From<MonitoringSubscriptionView> for MonitoringSubscriptionData {
    fn from(v: MonitoringSubscriptionView) -> Self {
        MonitoringSubscriptionData {
            distribution_id: v.distribution_id,
            realtime_metrics_subscription_status: v.realtime_metrics_subscription_status,
        }
    }
}

impl From<&ResourcePolicyData> for ResourcePolicyView {
    fn from(rp: &ResourcePolicyData) -> Self {
        ResourcePolicyView {
            resource_arn: rp.resource_arn.clone(),
            policy_document: rp.policy_document.clone(),
        }
    }
}

impl From<ResourcePolicyView> for ResourcePolicyData {
    fn from(v: ResourcePolicyView) -> Self {
        ResourcePolicyData {
            resource_arn: v.resource_arn,
            policy_document: v.policy_document,
        }
    }
}

impl From<&WebAclAssociation> for WebAclAssociationView {
    fn from(a: &WebAclAssociation) -> Self {
        WebAclAssociationView {
            resource_id: a.resource_id.clone(),
            web_acl_arn: a.web_acl_arn.clone(),
        }
    }
}

impl From<WebAclAssociationView> for WebAclAssociation {
    fn from(v: WebAclAssociationView) -> Self {
        WebAclAssociation {
            resource_id: v.resource_id,
            web_acl_arn: v.web_acl_arn,
        }
    }
}

// ---------------------------------------------------------------------------
// StatefulService implementation
// ---------------------------------------------------------------------------

impl StatefulService for CloudFrontService {
    type StateView = CloudFrontStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;

        let distributions = guard
            .distributions
            .iter()
            .map(|(k, v)| (k.clone(), DistributionView::from(v)))
            .collect();
        let invalidations = guard
            .invalidations
            .iter()
            .map(|(k, invs)| (k.clone(), invs.iter().map(InvalidationView::from).collect()))
            .collect();
        let key_groups = guard
            .key_groups
            .iter()
            .map(|(k, v)| (k.clone(), KeyGroupView::from(v)))
            .collect();
        let origin_access_controls = guard
            .origin_access_controls
            .iter()
            .map(|(k, v)| (k.clone(), OriginAccessControlView::from(v)))
            .collect();
        let public_keys = guard
            .public_keys
            .iter()
            .map(|(k, v)| (k.clone(), PublicKeyView::from(v)))
            .collect();
        let cache_policies = guard
            .cache_policies
            .iter()
            .map(|(k, v)| (k.clone(), CachePolicyView::from(v)))
            .collect();
        let origin_request_policies = guard
            .origin_request_policies
            .iter()
            .map(|(k, v)| (k.clone(), OriginRequestPolicyView::from(v)))
            .collect();
        let response_headers_policies = guard
            .response_headers_policies
            .iter()
            .map(|(k, v)| (k.clone(), ResponseHeadersPolicyView::from(v)))
            .collect();
        let functions = guard
            .functions
            .iter()
            .map(|(k, v)| (k.clone(), CloudFrontFunctionView::from(v)))
            .collect();
        let anycast_ip_lists = guard
            .anycast_ip_lists
            .iter()
            .map(|(k, v)| (k.clone(), AnycastIpListView::from(v)))
            .collect();
        let oais = guard
            .oais
            .iter()
            .map(|(k, v)| (k.clone(), OaiView::from(v)))
            .collect();
        let streaming_distributions = guard
            .streaming_distributions
            .iter()
            .map(|(k, v)| (k.clone(), StreamingDistributionView::from(v)))
            .collect();
        let key_value_stores = guard
            .key_value_stores
            .iter()
            .map(|(k, v)| (k.clone(), KeyValueStoreView::from(v)))
            .collect();
        let trust_stores = guard
            .trust_stores
            .iter()
            .map(|(k, v)| (k.clone(), TrustStoreView::from(v)))
            .collect();
        let vpc_origins = guard
            .vpc_origins
            .iter()
            .map(|(k, v)| (k.clone(), VpcOriginView::from(v)))
            .collect();
        let realtime_log_configs = guard
            .realtime_log_configs
            .iter()
            .map(|(k, v)| (k.clone(), RealtimeLogConfigView::from(v)))
            .collect();
        let field_level_encryptions = guard
            .field_level_encryptions
            .iter()
            .map(|(k, v)| (k.clone(), FieldLevelEncryptionView::from(v)))
            .collect();
        let field_level_encryption_profiles = guard
            .field_level_encryption_profiles
            .iter()
            .map(|(k, v)| (k.clone(), FieldLevelEncryptionProfileView::from(v)))
            .collect();

        let connection_functions = guard
            .connection_functions
            .iter()
            .map(|(k, v)| (k.clone(), ConnectionFunctionView::from(v)))
            .collect();
        let connection_groups = guard
            .connection_groups
            .iter()
            .map(|(k, v)| (k.clone(), ConnectionGroupView::from(v)))
            .collect();
        let continuous_deployment_policies = guard
            .continuous_deployment_policies
            .iter()
            .map(|(k, v)| (k.clone(), ContinuousDeploymentPolicyView::from(v)))
            .collect();
        let distribution_tenants = guard
            .distribution_tenants
            .iter()
            .map(|(k, v)| (k.clone(), DistributionTenantView::from(v)))
            .collect();
        let monitoring_subscriptions = guard
            .monitoring_subscriptions
            .iter()
            .map(|(k, v)| (k.clone(), MonitoringSubscriptionView::from(v)))
            .collect();
        let resource_policies = guard
            .resource_policies
            .iter()
            .map(|(k, v)| (k.clone(), ResourcePolicyView::from(v)))
            .collect();
        let web_acl_associations = guard
            .web_acl_associations
            .iter()
            .map(|(k, v)| (k.clone(), WebAclAssociationView::from(v)))
            .collect();

        CloudFrontStateView {
            distributions,
            invalidations,
            key_groups,
            origin_access_controls,
            public_keys,
            cache_policies,
            origin_request_policies,
            response_headers_policies,
            functions,
            anycast_ip_lists,
            oais,
            streaming_distributions,
            key_value_stores,
            trust_stores,
            vpc_origins,
            realtime_log_configs,
            field_level_encryptions,
            field_level_encryption_profiles,
            connection_functions,
            connection_groups,
            continuous_deployment_policies,
            distribution_tenants,
            monitoring_subscriptions,
            resource_policies,
            web_acl_associations,
        }
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let new_state = CloudFrontState {
            distributions: view
                .distributions
                .into_values()
                .map(|v| {
                    let d = Distribution::from(v);
                    (d.id.clone(), d)
                })
                .collect(),
            invalidations: view
                .invalidations
                .into_iter()
                .map(|(k, invs)| (k, invs.into_iter().map(Invalidation::from).collect()))
                .collect(),
            key_groups: view
                .key_groups
                .into_values()
                .map(|v| {
                    let kg = KeyGroupData::from(v);
                    (kg.id.clone(), kg)
                })
                .collect(),
            origin_access_controls: view
                .origin_access_controls
                .into_values()
                .map(|v| {
                    let oac = OriginAccessControlData::from(v);
                    (oac.id.clone(), oac)
                })
                .collect(),
            public_keys: view
                .public_keys
                .into_values()
                .map(|v| {
                    let pk = PublicKeyData::from(v);
                    (pk.id.clone(), pk)
                })
                .collect(),
            cache_policies: view
                .cache_policies
                .into_values()
                .map(|v| {
                    let cp = CachePolicyData::from(v);
                    (cp.id.clone(), cp)
                })
                .collect(),
            origin_request_policies: view
                .origin_request_policies
                .into_values()
                .map(|v| {
                    let orp = OriginRequestPolicyData::from(v);
                    (orp.id.clone(), orp)
                })
                .collect(),
            response_headers_policies: view
                .response_headers_policies
                .into_values()
                .map(|v| {
                    let rhp = ResponseHeadersPolicyData::from(v);
                    (rhp.id.clone(), rhp)
                })
                .collect(),
            functions: view
                .functions
                .into_values()
                .map(|v| {
                    let f = CloudFrontFunctionData::from(v);
                    (f.name.clone(), f)
                })
                .collect(),
            anycast_ip_lists: view
                .anycast_ip_lists
                .into_values()
                .map(|v| {
                    let l = AnycastIpListData::from(v);
                    (l.id.clone(), l)
                })
                .collect(),
            oais: view
                .oais
                .into_values()
                .map(|v| {
                    let o = OaiData::from(v);
                    (o.id.clone(), o)
                })
                .collect(),
            streaming_distributions: view
                .streaming_distributions
                .into_values()
                .map(|v| {
                    let s = StreamingDistributionData::from(v);
                    (s.id.clone(), s)
                })
                .collect(),
            key_value_stores: view
                .key_value_stores
                .into_values()
                .map(|v| {
                    let k = KeyValueStoreData::from(v);
                    (k.name.clone(), k)
                })
                .collect(),
            trust_stores: view
                .trust_stores
                .into_values()
                .map(|v| {
                    let t = TrustStoreData::from(v);
                    (t.id.clone(), t)
                })
                .collect(),
            vpc_origins: view
                .vpc_origins
                .into_values()
                .map(|v| {
                    let vpc = VpcOriginData::from(v);
                    (vpc.id.clone(), vpc)
                })
                .collect(),
            realtime_log_configs: view
                .realtime_log_configs
                .into_values()
                .map(|v| {
                    let r = RealtimeLogConfigData::from(v);
                    (r.arn.clone(), r)
                })
                .collect(),
            field_level_encryptions: view
                .field_level_encryptions
                .into_values()
                .map(|v| {
                    let f = FieldLevelEncryptionData::from(v);
                    (f.id.clone(), f)
                })
                .collect(),
            field_level_encryption_profiles: view
                .field_level_encryption_profiles
                .into_values()
                .map(|v| {
                    let f = FieldLevelEncryptionProfileData::from(v);
                    (f.id.clone(), f)
                })
                .collect(),
            connection_functions: view
                .connection_functions
                .into_values()
                .map(|v| {
                    let f = ConnectionFunctionData::from(v);
                    (f.id.clone(), f)
                })
                .collect(),
            connection_groups: view
                .connection_groups
                .into_values()
                .map(|v| {
                    let cg = ConnectionGroupData::from(v);
                    (cg.id.clone(), cg)
                })
                .collect(),
            continuous_deployment_policies: view
                .continuous_deployment_policies
                .into_values()
                .map(|v| {
                    let cdp = ContinuousDeploymentPolicyData::from(v);
                    (cdp.id.clone(), cdp)
                })
                .collect(),
            distribution_tenants: view
                .distribution_tenants
                .into_values()
                .map(|v| {
                    let dt = DistributionTenantData::from(v);
                    (dt.id.clone(), dt)
                })
                .collect(),
            monitoring_subscriptions: view
                .monitoring_subscriptions
                .into_values()
                .map(|v| {
                    let ms = MonitoringSubscriptionData::from(v);
                    (ms.distribution_id.clone(), ms)
                })
                .collect(),
            resource_policies: view
                .resource_policies
                .into_values()
                .map(|v| {
                    let rp = ResourcePolicyData::from(v);
                    (rp.resource_arn.clone(), rp)
                })
                .collect(),
            web_acl_associations: view
                .web_acl_associations
                .into_values()
                .map(|v| {
                    let a = WebAclAssociation::from(v);
                    (a.resource_id.clone(), a)
                })
                .collect(),
        };

        {
            let state = self.state.get(account_id, region);
            *state.write().await = new_state;
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for v in view.distributions.into_values() {
                let d = Distribution::from(v);
                guard.distributions.insert(d.id.clone(), d);
            }
            for (k, invs) in view.invalidations {
                let entries = guard.invalidations.entry(k).or_default();
                for iv in invs {
                    entries.push(Invalidation::from(iv));
                }
            }
            for v in view.key_groups.into_values() {
                let kg = KeyGroupData::from(v);
                guard.key_groups.insert(kg.id.clone(), kg);
            }
            for v in view.origin_access_controls.into_values() {
                let oac = OriginAccessControlData::from(v);
                guard.origin_access_controls.insert(oac.id.clone(), oac);
            }
            for v in view.public_keys.into_values() {
                let pk = PublicKeyData::from(v);
                guard.public_keys.insert(pk.id.clone(), pk);
            }
            for v in view.cache_policies.into_values() {
                let cp = CachePolicyData::from(v);
                guard.cache_policies.insert(cp.id.clone(), cp);
            }
            for v in view.origin_request_policies.into_values() {
                let orp = OriginRequestPolicyData::from(v);
                guard.origin_request_policies.insert(orp.id.clone(), orp);
            }
            for v in view.response_headers_policies.into_values() {
                let rhp = ResponseHeadersPolicyData::from(v);
                guard.response_headers_policies.insert(rhp.id.clone(), rhp);
            }
            for v in view.functions.into_values() {
                let f = CloudFrontFunctionData::from(v);
                guard.functions.insert(f.name.clone(), f);
            }
            for v in view.anycast_ip_lists.into_values() {
                let l = AnycastIpListData::from(v);
                guard.anycast_ip_lists.insert(l.id.clone(), l);
            }
            for v in view.oais.into_values() {
                let o = OaiData::from(v);
                guard.oais.insert(o.id.clone(), o);
            }
            for v in view.streaming_distributions.into_values() {
                let s = StreamingDistributionData::from(v);
                guard.streaming_distributions.insert(s.id.clone(), s);
            }
            for v in view.key_value_stores.into_values() {
                let k = KeyValueStoreData::from(v);
                guard.key_value_stores.insert(k.name.clone(), k);
            }
            for v in view.trust_stores.into_values() {
                let t = TrustStoreData::from(v);
                guard.trust_stores.insert(t.id.clone(), t);
            }
            for v in view.vpc_origins.into_values() {
                let vpc = VpcOriginData::from(v);
                guard.vpc_origins.insert(vpc.id.clone(), vpc);
            }
            for v in view.realtime_log_configs.into_values() {
                let r = RealtimeLogConfigData::from(v);
                guard.realtime_log_configs.insert(r.arn.clone(), r);
            }
            for v in view.field_level_encryptions.into_values() {
                let f = FieldLevelEncryptionData::from(v);
                guard.field_level_encryptions.insert(f.id.clone(), f);
            }
            for v in view.field_level_encryption_profiles.into_values() {
                let f = FieldLevelEncryptionProfileData::from(v);
                guard
                    .field_level_encryption_profiles
                    .insert(f.id.clone(), f);
            }
            for v in view.connection_functions.into_values() {
                let f = ConnectionFunctionData::from(v);
                guard.connection_functions.insert(f.id.clone(), f);
            }
            for v in view.connection_groups.into_values() {
                let cg = ConnectionGroupData::from(v);
                guard.connection_groups.insert(cg.id.clone(), cg);
            }
            for v in view.continuous_deployment_policies.into_values() {
                let cdp = ContinuousDeploymentPolicyData::from(v);
                guard
                    .continuous_deployment_policies
                    .insert(cdp.id.clone(), cdp);
            }
            for v in view.distribution_tenants.into_values() {
                let dt = DistributionTenantData::from(v);
                guard.distribution_tenants.insert(dt.id.clone(), dt);
            }
            for v in view.monitoring_subscriptions.into_values() {
                let ms = MonitoringSubscriptionData::from(v);
                guard
                    .monitoring_subscriptions
                    .insert(ms.distribution_id.clone(), ms);
            }
            for v in view.resource_policies.into_values() {
                let rp = ResourcePolicyData::from(v);
                guard.resource_policies.insert(rp.resource_arn.clone(), rp);
            }
            for v in view.web_acl_associations.into_values() {
                let a = WebAclAssociation::from(v);
                guard.web_acl_associations.insert(a.resource_id.clone(), a);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
