use std::collections::HashMap;

use chrono::{DateTime, Utc};

pub use crate::model::{
    CachePolicyConfig, FunctionConfig, OriginRequestPolicyConfig, ResponseHeadersPolicyConfig,
    StreamingDistributionConfig, VpcOriginEndpointConfig,
};

#[derive(Debug, Clone)]
pub struct Distribution {
    pub id: String,
    pub arn: String,
    pub domain_name: String,
    pub status: String,
    pub origins: Vec<Origin>,
    pub default_cache_behavior: DefaultCacheBehavior,
    pub caller_reference: String,
    pub created_at: DateTime<Utc>,
    pub etag: String,
    pub enabled: bool,
    pub tags: HashMap<String, String>,
    /// Full DistributionConfig parsed from the request body, stored so that
    /// GetDistribution/GetDistributionConfig can echo back ALL fields (e.g.
    /// Restrictions, ViewerCertificate) without the provider panicking on nil
    /// pointer dereferences.
    pub raw_config: crate::model::DistributionConfig,
}

#[derive(Debug, Clone)]
pub struct Origin {
    pub id: String,
    pub domain_name: String,
}

#[derive(Debug, Clone)]
pub struct DefaultCacheBehavior {
    pub target_origin_id: String,
    pub viewer_protocol_policy: String,
    /// e.g. ["GET", "HEAD", "OPTIONS"]
    pub allowed_methods: Vec<String>,
    /// e.g. ["GET", "HEAD"] — subset of allowed_methods that are cached
    pub cached_methods: Vec<String>,
    /// Whether query strings are forwarded to the origin
    pub forwarded_values_query_string: bool,
    /// Cookie forwarding mode: "none", "whitelist", or "all"
    pub forwarded_values_cookies_forward: String,
    /// Whether gzip compression is enabled
    pub compress: bool,
}

#[derive(Debug, Clone)]
pub struct Invalidation {
    pub id: String,
    pub distribution_id: String,
    pub status: String,
    pub create_time: DateTime<Utc>,
    pub caller_reference: String,
    pub paths: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct KeyGroupData {
    pub id: String,
    pub name: String,
    pub comment: Option<String>,
    pub items: Vec<String>,
    pub last_modified_time: DateTime<Utc>,
    pub etag: String,
}

#[derive(Debug, Clone)]
pub struct OriginAccessControlData {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub origin_access_control_origin_type: String,
    pub signing_behavior: String,
    pub signing_protocol: String,
    pub etag: String,
}

#[derive(Debug, Clone)]
pub struct PublicKeyData {
    pub id: String,
    pub name: String,
    pub caller_reference: String,
    pub encoded_key: String,
    pub comment: Option<String>,
    pub created_time: DateTime<Utc>,
    pub etag: String,
}

/// A stored cache policy.
#[derive(Debug, Clone)]
pub struct CachePolicyData {
    pub id: String,
    pub last_modified_time: DateTime<Utc>,
    pub config: CachePolicyConfig,
    pub etag: String,
}

/// A stored origin request policy.
#[derive(Debug, Clone)]
pub struct OriginRequestPolicyData {
    pub id: String,
    pub last_modified_time: DateTime<Utc>,
    pub config: OriginRequestPolicyConfig,
    pub etag: String,
}

/// A stored response headers policy.
#[derive(Debug, Clone)]
pub struct ResponseHeadersPolicyData {
    pub id: String,
    pub last_modified_time: DateTime<Utc>,
    pub config: ResponseHeadersPolicyConfig,
    pub etag: String,
}

/// A stored CloudFront function.
#[derive(Debug, Clone)]
pub struct CloudFrontFunctionData {
    pub name: String,
    pub arn: String,
    pub status: String,
    pub created_time: DateTime<Utc>,
    pub last_modified_time: DateTime<Utc>,
    pub config: FunctionConfig,
    pub code: Vec<u8>,
    pub etag: String,
}

/// A stored CloudFront Origin Access Identity (legacy OAI).
#[derive(Debug, Clone)]
pub struct OaiData {
    pub id: String,
    pub caller_reference: String,
    pub comment: String,
    pub s3_canonical_user_id: String,
    pub etag: String,
}

/// A stored streaming distribution.
#[derive(Debug, Clone)]
pub struct StreamingDistributionData {
    pub id: String,
    pub arn: String,
    pub domain_name: String,
    pub status: String,
    pub last_modified_time: DateTime<Utc>,
    pub config: StreamingDistributionConfig,
    pub etag: String,
}

/// A stored key-value store.
#[derive(Debug, Clone)]
pub struct KeyValueStoreData {
    pub name: String,
    pub arn: String,
    pub id: String,
    pub comment: Option<String>,
    pub last_modified_time: DateTime<Utc>,
    pub status: String,
    pub etag: String,
}

/// A stored trust store.
#[derive(Debug, Clone)]
pub struct TrustStoreData {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub last_modified_time: DateTime<Utc>,
    pub status: String,
    pub number_of_ca_certificates: i32,
    pub etag: String,
}

/// A stored anycast IP list.
#[derive(Debug, Clone)]
pub struct AnycastIpListData {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub status: String,
    pub ip_count: i32,
    pub ip_address_type: Option<String>,
    pub last_modified_time: DateTime<Utc>,
    pub etag: String,
}

/// A stored VPC origin.
#[derive(Debug, Clone)]
pub struct VpcOriginData {
    pub id: String,
    pub arn: String,
    pub account_id: String,
    pub created_time: DateTime<Utc>,
    pub last_modified_time: DateTime<Utc>,
    pub status: String,
    pub config: VpcOriginEndpointConfig,
    pub etag: String,
}

/// A stored realtime log config.
#[derive(Debug, Clone)]
pub struct RealtimeLogConfigData {
    pub name: String,
    pub arn: String,
    pub sampling_rate: i64,
    pub end_points: Vec<crate::model::EndPoint>,
    pub fields: Vec<String>,
}

/// A stored field-level encryption configuration.
#[derive(Debug, Clone)]
pub struct FieldLevelEncryptionData {
    pub id: String,
    pub last_modified_time: DateTime<Utc>,
    pub config: crate::model::FieldLevelEncryptionConfig,
    pub etag: String,
}

/// A stored field-level encryption profile.
#[derive(Debug, Clone)]
pub struct FieldLevelEncryptionProfileData {
    pub id: String,
    pub last_modified_time: DateTime<Utc>,
    pub config: crate::model::FieldLevelEncryptionProfileConfig,
    pub etag: String,
}

/// A stored connection function.
#[derive(Debug, Clone)]
pub struct ConnectionFunctionData {
    pub id: String,
    pub name: String,
    pub arn: String,
    pub status: String,
    pub stage: String,
    pub created_time: DateTime<Utc>,
    pub last_modified_time: DateTime<Utc>,
    pub config: crate::model::FunctionConfig,
    pub code: String,
    pub etag: String,
}

/// A stored connection group.
#[derive(Debug, Clone)]
pub struct ConnectionGroupData {
    pub id: String,
    pub name: String,
    pub arn: String,
    pub status: String,
    pub routing_endpoint: String,
    pub enabled: bool,
    pub ipv6_enabled: bool,
    pub is_default: bool,
    pub anycast_ip_list_id: Option<String>,
    pub created_time: DateTime<Utc>,
    pub last_modified_time: DateTime<Utc>,
    pub etag: String,
}

/// A stored continuous deployment policy.
#[derive(Debug, Clone)]
pub struct ContinuousDeploymentPolicyData {
    pub id: String,
    pub last_modified_time: DateTime<Utc>,
    pub config: crate::model::ContinuousDeploymentPolicyConfig,
    pub etag: String,
}

/// A stored distribution tenant.
#[derive(Debug, Clone)]
pub struct DistributionTenantData {
    pub id: String,
    pub name: String,
    pub arn: String,
    pub distribution_id: String,
    pub connection_group_id: Option<String>,
    pub enabled: bool,
    pub status: String,
    pub domains: crate::model::DomainResultList,
    pub customizations: Option<crate::model::Customizations>,
    pub parameters: Option<crate::model::Parameters>,
    pub created_time: DateTime<Utc>,
    pub last_modified_time: DateTime<Utc>,
    pub etag: String,
}

/// A stored monitoring subscription for a distribution.
#[derive(Debug, Clone)]
pub struct MonitoringSubscriptionData {
    pub distribution_id: String,
    pub realtime_metrics_subscription_status: String,
}

/// A stored resource policy.
#[derive(Debug, Clone)]
pub struct ResourcePolicyData {
    pub resource_arn: String,
    pub policy_document: String,
}

/// A stored WebACL association.
#[derive(Debug, Clone)]
pub struct WebAclAssociation {
    pub resource_id: String,
    pub web_acl_arn: String,
}
