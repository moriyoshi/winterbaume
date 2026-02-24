//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-cloudfront

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListAnycastIpListsRequest")]
pub struct ListAnycastIpListsRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "VpcOriginList")]
pub struct VpcOriginList {
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<VpcOriginSummaryList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcOriginSummaryList {
    #[serde(
        rename = "VpcOriginSummary",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<VpcOriginSummary>,
}
impl From<Vec<VpcOriginSummary>> for VpcOriginSummaryList {
    fn from(v: Vec<VpcOriginSummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<VpcOriginSummary> for VpcOriginSummaryList {
    fn from_iter<I: IntoIterator<Item = VpcOriginSummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<VpcOriginSummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlVpcOriginSummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<VpcOriginSummary>,
}

impl From<Vec<VpcOriginSummary>> for XmlVpcOriginSummaryList {
    fn from(v: Vec<VpcOriginSummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<VpcOriginSummary> for XmlVpcOriginSummaryList {
    fn from_iter<I: IntoIterator<Item = VpcOriginSummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "VpcOriginSummary")]
pub struct VpcOriginSummary {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OriginEndpointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_endpoint_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateOriginRequestPolicyResult")]
pub struct CreateOriginRequestPolicyResult {
    #[serde(rename = "OriginRequestPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_request_policy: Option<OriginRequestPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OriginRequestPolicy")]
pub struct OriginRequestPolicy {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(rename = "OriginRequestPolicyConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_request_policy_config: Option<OriginRequestPolicyConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OriginRequestPolicyConfig")]
pub struct OriginRequestPolicyConfig {
    #[serde(rename = "Comment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "CookiesConfig")]
    #[serde(default)]
    pub cookies_config: OriginRequestPolicyCookiesConfig,
    #[serde(rename = "HeadersConfig")]
    #[serde(default)]
    pub headers_config: OriginRequestPolicyHeadersConfig,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "QueryStringsConfig")]
    #[serde(default)]
    pub query_strings_config: OriginRequestPolicyQueryStringsConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OriginRequestPolicyCookiesConfig")]
pub struct OriginRequestPolicyCookiesConfig {
    #[serde(rename = "CookieBehavior")]
    #[serde(default)]
    pub cookie_behavior: String,
    #[serde(rename = "Cookies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<CookieNames>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CookieNames")]
pub struct CookieNames {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<CookieNameList>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    pub quantity: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CookieNameList {
    #[serde(rename = "Name", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for CookieNameList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for CookieNameList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<String>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlStringList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<String>,
}

impl From<Vec<String>> for XmlStringList {
    fn from(v: Vec<String>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<String> for XmlStringList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OriginRequestPolicyHeadersConfig")]
pub struct OriginRequestPolicyHeadersConfig {
    #[serde(rename = "HeaderBehavior")]
    #[serde(default)]
    pub header_behavior: String,
    #[serde(rename = "Headers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Headers>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Headers")]
pub struct Headers {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<HeaderList>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    pub quantity: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HeaderList {
    #[serde(rename = "Name", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for HeaderList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for HeaderList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OriginRequestPolicyQueryStringsConfig")]
pub struct OriginRequestPolicyQueryStringsConfig {
    #[serde(rename = "QueryStringBehavior")]
    #[serde(default)]
    pub query_string_behavior: String,
    #[serde(rename = "QueryStrings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_strings: Option<QueryStringNames>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "QueryStringNames")]
pub struct QueryStringNames {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<QueryStringNamesList>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    pub quantity: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryStringNamesList {
    #[serde(rename = "Name", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for QueryStringNamesList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for QueryStringNamesList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "KeyValueStore")]
pub struct KeyValueStore {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "Comment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateInvalidationResult")]
pub struct CreateInvalidationResult {
    #[serde(rename = "Invalidation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalidation: Option<Invalidation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Invalidation")]
pub struct Invalidation {
    #[serde(rename = "CreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "InvalidationBatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalidation_batch: Option<InvalidationBatch>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "InvalidationBatch")]
pub struct InvalidationBatch {
    #[serde(rename = "CallerReference")]
    #[serde(default)]
    pub caller_reference: String,
    #[serde(rename = "Paths")]
    #[serde(default)]
    pub paths: Paths,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Paths")]
pub struct Paths {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<PathList>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    pub quantity: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PathList {
    #[serde(rename = "Path", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for PathList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for PathList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListDistributionsByRealtimeLogConfigRequest")]
pub struct ListDistributionsByRealtimeLogConfigRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "RealtimeLogConfigArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realtime_log_config_arn: Option<String>,
    #[serde(rename = "RealtimeLogConfigName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realtime_log_config_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TestResult")]
pub struct TestResult {
    #[serde(rename = "ComputeUtilization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_utilization: Option<String>,
    #[serde(rename = "FunctionErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_error_message: Option<String>,
    #[serde(rename = "FunctionExecutionLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_execution_logs: Option<FunctionExecutionLogList>,
    #[serde(rename = "FunctionOutput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_output: Option<String>,
    #[serde(rename = "FunctionSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_summary: Option<FunctionSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FunctionExecutionLogList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for FunctionExecutionLogList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for FunctionExecutionLogList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "FunctionSummary")]
pub struct FunctionSummary {
    #[serde(rename = "FunctionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_config: Option<FunctionConfig>,
    #[serde(rename = "FunctionMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_metadata: Option<FunctionMetadata>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "FunctionConfig")]
pub struct FunctionConfig {
    #[serde(rename = "Comment")]
    #[serde(default)]
    pub comment: String,
    #[serde(rename = "KeyValueStoreAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_value_store_associations: Option<KeyValueStoreAssociations>,
    #[serde(rename = "Runtime")]
    #[serde(default)]
    pub runtime: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "KeyValueStoreAssociations")]
pub struct KeyValueStoreAssociations {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<KeyValueStoreAssociationList>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    pub quantity: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KeyValueStoreAssociationList {
    #[serde(
        rename = "KeyValueStoreAssociation",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<KeyValueStoreAssociation>,
}
impl From<Vec<KeyValueStoreAssociation>> for KeyValueStoreAssociationList {
    fn from(v: Vec<KeyValueStoreAssociation>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<KeyValueStoreAssociation> for KeyValueStoreAssociationList {
    fn from_iter<I: IntoIterator<Item = KeyValueStoreAssociation>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<KeyValueStoreAssociation>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlKeyValueStoreAssociationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<KeyValueStoreAssociation>,
}

impl From<Vec<KeyValueStoreAssociation>> for XmlKeyValueStoreAssociationList {
    fn from(v: Vec<KeyValueStoreAssociation>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<KeyValueStoreAssociation> for XmlKeyValueStoreAssociationList {
    fn from_iter<I: IntoIterator<Item = KeyValueStoreAssociation>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "KeyValueStoreAssociation")]
pub struct KeyValueStoreAssociation {
    #[serde(rename = "KeyValueStoreARN")]
    #[serde(default)]
    pub key_value_store_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "FunctionMetadata")]
pub struct FunctionMetadata {
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "FunctionARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_a_r_n: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(rename = "Stage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "VerifyDnsConfigurationResult")]
pub struct VerifyDnsConfigurationResult {
    #[serde(rename = "DnsConfigurationList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_configuration_list: Option<DnsConfigurationList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DnsConfigurationList {
    #[serde(
        rename = "DnsConfiguration",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DnsConfiguration>,
}
impl From<Vec<DnsConfiguration>> for DnsConfigurationList {
    fn from(v: Vec<DnsConfiguration>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DnsConfiguration> for DnsConfigurationList {
    fn from_iter<I: IntoIterator<Item = DnsConfiguration>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DnsConfiguration>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDnsConfigurationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DnsConfiguration>,
}

impl From<Vec<DnsConfiguration>> for XmlDnsConfigurationList {
    fn from(v: Vec<DnsConfiguration>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DnsConfiguration> for XmlDnsConfigurationList {
    fn from_iter<I: IntoIterator<Item = DnsConfiguration>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DnsConfiguration")]
pub struct DnsConfiguration {
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "Reason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AssociateDistributionTenantWebACLRequest")]
pub struct AssociateDistributionTenantWebACLRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match: Option<String>,
    #[serde(rename = "WebACLArn")]
    #[serde(default)]
    pub web_a_c_l_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListDistributionsByVpcOriginIdResult")]
pub struct ListDistributionsByVpcOriginIdResult {
    #[serde(rename = "DistributionIdList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_id_list: Option<DistributionIdList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DistributionIdList")]
pub struct DistributionIdList {
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<DistributionIdListSummary>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DistributionIdListSummary {
    #[serde(
        rename = "DistributionId",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for DistributionIdListSummary {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for DistributionIdListSummary {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListInvalidationsForDistributionTenantRequest")]
pub struct ListInvalidationsForDistributionTenantRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetDistributionConfigResult")]
pub struct GetDistributionConfigResult {
    #[serde(rename = "DistributionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_config: Option<DistributionConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DistributionConfig")]
pub struct DistributionConfig {
    #[serde(rename = "Aliases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Aliases>,
    #[serde(rename = "AnycastIpListId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anycast_ip_list_id: Option<String>,
    #[serde(rename = "CacheBehaviors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_behaviors: Option<CacheBehaviors>,
    #[serde(rename = "CallerReference")]
    #[serde(default)]
    pub caller_reference: String,
    #[serde(rename = "Comment")]
    #[serde(default)]
    pub comment: String,
    #[serde(rename = "ConnectionFunctionAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_function_association: Option<ConnectionFunctionAssociation>,
    #[serde(rename = "ConnectionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_mode: Option<String>,
    #[serde(rename = "ContinuousDeploymentPolicyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuous_deployment_policy_id: Option<String>,
    #[serde(rename = "CustomErrorResponses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_error_responses: Option<CustomErrorResponses>,
    #[serde(rename = "DefaultCacheBehavior")]
    #[serde(default)]
    pub default_cache_behavior: DefaultCacheBehavior,
    #[serde(rename = "DefaultRootObject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_root_object: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "HttpVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_version: Option<String>,
    #[serde(rename = "IsIPV6Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_i_p_v6_enabled: Option<bool>,
    #[serde(rename = "Logging")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<LoggingConfig>,
    #[serde(rename = "OriginGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_groups: Option<OriginGroups>,
    #[serde(rename = "Origins")]
    #[serde(default)]
    pub origins: Origins,
    #[serde(rename = "PriceClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_class: Option<String>,
    #[serde(rename = "Restrictions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Restrictions>,
    #[serde(rename = "Staging")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staging: Option<bool>,
    #[serde(rename = "TenantConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_config: Option<TenantConfig>,
    #[serde(rename = "ViewerCertificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_certificate: Option<ViewerCertificate>,
    #[serde(rename = "ViewerMtlsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_mtls_config: Option<ViewerMtlsConfig>,
    #[serde(rename = "WebACLId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_a_c_l_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Aliases")]
pub struct Aliases {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<AliasList>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    pub quantity: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AliasList {
    #[serde(rename = "CNAME", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for AliasList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for AliasList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CacheBehaviors")]
pub struct CacheBehaviors {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<CacheBehaviorList>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    pub quantity: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CacheBehaviorList {
    #[serde(
        rename = "CacheBehavior",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<CacheBehavior>,
}
impl From<Vec<CacheBehavior>> for CacheBehaviorList {
    fn from(v: Vec<CacheBehavior>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<CacheBehavior> for CacheBehaviorList {
    fn from_iter<I: IntoIterator<Item = CacheBehavior>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<CacheBehavior>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlCacheBehaviorList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<CacheBehavior>,
}

impl From<Vec<CacheBehavior>> for XmlCacheBehaviorList {
    fn from(v: Vec<CacheBehavior>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<CacheBehavior> for XmlCacheBehaviorList {
    fn from_iter<I: IntoIterator<Item = CacheBehavior>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CacheBehavior")]
pub struct CacheBehavior {
    #[serde(rename = "AllowedMethods")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_methods: Option<AllowedMethods>,
    #[serde(rename = "CachePolicyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_policy_id: Option<String>,
    #[serde(rename = "Compress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compress: Option<bool>,
    #[serde(rename = "DefaultTTL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_t_t_l: Option<i64>,
    #[serde(rename = "FieldLevelEncryptionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_level_encryption_id: Option<String>,
    #[serde(rename = "ForwardedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forwarded_values: Option<ForwardedValues>,
    #[serde(rename = "FunctionAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_associations: Option<FunctionAssociations>,
    #[serde(rename = "GrpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grpc_config: Option<GrpcConfig>,
    #[serde(rename = "LambdaFunctionAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_associations: Option<LambdaFunctionAssociations>,
    #[serde(rename = "MaxTTL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_t_t_l: Option<i64>,
    #[serde(rename = "MinTTL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_t_t_l: Option<i64>,
    #[serde(rename = "OriginRequestPolicyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_request_policy_id: Option<String>,
    #[serde(rename = "PathPattern")]
    #[serde(default)]
    pub path_pattern: String,
    #[serde(rename = "RealtimeLogConfigArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realtime_log_config_arn: Option<String>,
    #[serde(rename = "ResponseHeadersPolicyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_headers_policy_id: Option<String>,
    #[serde(rename = "SmoothStreaming")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smooth_streaming: Option<bool>,
    #[serde(rename = "TargetOriginId")]
    #[serde(default)]
    pub target_origin_id: String,
    #[serde(rename = "TrustedKeyGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_key_groups: Option<TrustedKeyGroups>,
    #[serde(rename = "TrustedSigners")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_signers: Option<TrustedSigners>,
    #[serde(rename = "ViewerProtocolPolicy")]
    #[serde(default)]
    pub viewer_protocol_policy: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AllowedMethods")]
pub struct AllowedMethods {
    #[serde(rename = "CachedMethods")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_methods: Option<CachedMethods>,
    #[serde(rename = "Items")]
    #[serde(default)]
    pub items: MethodsList,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    pub quantity: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CachedMethods")]
pub struct CachedMethods {
    #[serde(rename = "Items")]
    #[serde(default)]
    pub items: MethodsList,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    pub quantity: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MethodsList {
    #[serde(rename = "Method", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for MethodsList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for MethodsList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ForwardedValues")]
pub struct ForwardedValues {
    #[serde(rename = "Cookies")]
    #[serde(default)]
    pub cookies: CookiePreference,
    #[serde(rename = "Headers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Headers>,
    #[serde(rename = "QueryString")]
    #[serde(default)]
    pub query_string: bool,
    #[serde(rename = "QueryStringCacheKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string_cache_keys: Option<QueryStringCacheKeys>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CookiePreference")]
pub struct CookiePreference {
    #[serde(rename = "Forward")]
    #[serde(default)]
    pub forward: String,
    #[serde(rename = "WhitelistedNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelisted_names: Option<CookieNames>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "QueryStringCacheKeys")]
pub struct QueryStringCacheKeys {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<QueryStringCacheKeysList>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    pub quantity: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryStringCacheKeysList {
    #[serde(rename = "Name", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for QueryStringCacheKeysList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for QueryStringCacheKeysList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "FunctionAssociations")]
pub struct FunctionAssociations {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<FunctionAssociationList>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    pub quantity: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FunctionAssociationList {
    #[serde(
        rename = "FunctionAssociation",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<FunctionAssociation>,
}
impl From<Vec<FunctionAssociation>> for FunctionAssociationList {
    fn from(v: Vec<FunctionAssociation>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<FunctionAssociation> for FunctionAssociationList {
    fn from_iter<I: IntoIterator<Item = FunctionAssociation>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<FunctionAssociation>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlFunctionAssociationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<FunctionAssociation>,
}

impl From<Vec<FunctionAssociation>> for XmlFunctionAssociationList {
    fn from(v: Vec<FunctionAssociation>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<FunctionAssociation> for XmlFunctionAssociationList {
    fn from_iter<I: IntoIterator<Item = FunctionAssociation>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "FunctionAssociation")]
pub struct FunctionAssociation {
    #[serde(rename = "EventType")]
    #[serde(default)]
    pub event_type: String,
    #[serde(rename = "FunctionARN")]
    #[serde(default)]
    pub function_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GrpcConfig")]
pub struct GrpcConfig {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LambdaFunctionAssociations")]
pub struct LambdaFunctionAssociations {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<LambdaFunctionAssociationList>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    pub quantity: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LambdaFunctionAssociationList {
    #[serde(
        rename = "LambdaFunctionAssociation",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<LambdaFunctionAssociation>,
}
impl From<Vec<LambdaFunctionAssociation>> for LambdaFunctionAssociationList {
    fn from(v: Vec<LambdaFunctionAssociation>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<LambdaFunctionAssociation> for LambdaFunctionAssociationList {
    fn from_iter<I: IntoIterator<Item = LambdaFunctionAssociation>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<LambdaFunctionAssociation>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlLambdaFunctionAssociationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<LambdaFunctionAssociation>,
}

impl From<Vec<LambdaFunctionAssociation>> for XmlLambdaFunctionAssociationList {
    fn from(v: Vec<LambdaFunctionAssociation>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<LambdaFunctionAssociation> for XmlLambdaFunctionAssociationList {
    fn from_iter<I: IntoIterator<Item = LambdaFunctionAssociation>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LambdaFunctionAssociation")]
pub struct LambdaFunctionAssociation {
    #[serde(rename = "EventType")]
    #[serde(default)]
    pub event_type: String,
    #[serde(rename = "IncludeBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_body: Option<bool>,
    #[serde(rename = "LambdaFunctionARN")]
    #[serde(default)]
    pub lambda_function_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TrustedKeyGroups")]
pub struct TrustedKeyGroups {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<TrustedKeyGroupIdList>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    pub quantity: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrustedKeyGroupIdList {
    #[serde(rename = "KeyGroup", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for TrustedKeyGroupIdList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for TrustedKeyGroupIdList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TrustedSigners")]
pub struct TrustedSigners {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<AwsAccountNumberList>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    pub quantity: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsAccountNumberList {
    #[serde(
        rename = "AwsAccountNumber",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for AwsAccountNumberList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for AwsAccountNumberList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ConnectionFunctionAssociation")]
pub struct ConnectionFunctionAssociation {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CustomErrorResponses")]
pub struct CustomErrorResponses {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<CustomErrorResponseList>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    pub quantity: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomErrorResponseList {
    #[serde(
        rename = "CustomErrorResponse",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<CustomErrorResponse>,
}
impl From<Vec<CustomErrorResponse>> for CustomErrorResponseList {
    fn from(v: Vec<CustomErrorResponse>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<CustomErrorResponse> for CustomErrorResponseList {
    fn from_iter<I: IntoIterator<Item = CustomErrorResponse>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<CustomErrorResponse>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlCustomErrorResponseList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<CustomErrorResponse>,
}

impl From<Vec<CustomErrorResponse>> for XmlCustomErrorResponseList {
    fn from(v: Vec<CustomErrorResponse>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<CustomErrorResponse> for XmlCustomErrorResponseList {
    fn from_iter<I: IntoIterator<Item = CustomErrorResponse>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CustomErrorResponse")]
pub struct CustomErrorResponse {
    #[serde(rename = "ErrorCachingMinTTL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_caching_min_t_t_l: Option<i64>,
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    pub error_code: i32,
    #[serde(rename = "ResponseCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<String>,
    #[serde(rename = "ResponsePagePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_page_path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DefaultCacheBehavior")]
pub struct DefaultCacheBehavior {
    #[serde(rename = "AllowedMethods")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_methods: Option<AllowedMethods>,
    #[serde(rename = "CachePolicyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_policy_id: Option<String>,
    #[serde(rename = "Compress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compress: Option<bool>,
    #[serde(rename = "DefaultTTL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_t_t_l: Option<i64>,
    #[serde(rename = "FieldLevelEncryptionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_level_encryption_id: Option<String>,
    #[serde(rename = "ForwardedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forwarded_values: Option<ForwardedValues>,
    #[serde(rename = "FunctionAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_associations: Option<FunctionAssociations>,
    #[serde(rename = "GrpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grpc_config: Option<GrpcConfig>,
    #[serde(rename = "LambdaFunctionAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_associations: Option<LambdaFunctionAssociations>,
    #[serde(rename = "MaxTTL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_t_t_l: Option<i64>,
    #[serde(rename = "MinTTL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_t_t_l: Option<i64>,
    #[serde(rename = "OriginRequestPolicyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_request_policy_id: Option<String>,
    #[serde(rename = "RealtimeLogConfigArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realtime_log_config_arn: Option<String>,
    #[serde(rename = "ResponseHeadersPolicyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_headers_policy_id: Option<String>,
    #[serde(rename = "SmoothStreaming")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smooth_streaming: Option<bool>,
    #[serde(rename = "TargetOriginId")]
    #[serde(default)]
    pub target_origin_id: String,
    #[serde(rename = "TrustedKeyGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_key_groups: Option<TrustedKeyGroups>,
    #[serde(rename = "TrustedSigners")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_signers: Option<TrustedSigners>,
    #[serde(rename = "ViewerProtocolPolicy")]
    #[serde(default)]
    pub viewer_protocol_policy: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LoggingConfig")]
pub struct LoggingConfig {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "IncludeCookies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_cookies: Option<bool>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OriginGroups")]
pub struct OriginGroups {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<OriginGroupList>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    pub quantity: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OriginGroupList {
    #[serde(rename = "OriginGroup", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<OriginGroup>,
}
impl From<Vec<OriginGroup>> for OriginGroupList {
    fn from(v: Vec<OriginGroup>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<OriginGroup> for OriginGroupList {
    fn from_iter<I: IntoIterator<Item = OriginGroup>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<OriginGroup>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlOriginGroupList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<OriginGroup>,
}

impl From<Vec<OriginGroup>> for XmlOriginGroupList {
    fn from(v: Vec<OriginGroup>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<OriginGroup> for XmlOriginGroupList {
    fn from_iter<I: IntoIterator<Item = OriginGroup>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OriginGroup")]
pub struct OriginGroup {
    #[serde(rename = "FailoverCriteria")]
    #[serde(default)]
    pub failover_criteria: OriginGroupFailoverCriteria,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Members")]
    #[serde(default)]
    pub members: OriginGroupMembers,
    #[serde(rename = "SelectionCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_criteria: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OriginGroupFailoverCriteria")]
pub struct OriginGroupFailoverCriteria {
    #[serde(rename = "StatusCodes")]
    #[serde(default)]
    pub status_codes: StatusCodes,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StatusCodes")]
pub struct StatusCodes {
    #[serde(rename = "Items")]
    #[serde(default)]
    pub items: StatusCodeList,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    pub quantity: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StatusCodeList {
    #[serde(rename = "StatusCode", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<i32>,
}
impl From<Vec<i32>> for StatusCodeList {
    fn from(v: Vec<i32>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<i32> for StatusCodeList {
    fn from_iter<I: IntoIterator<Item = i32>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<i32>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Xmli32List {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<i32>,
}

impl From<Vec<i32>> for Xmli32List {
    fn from(v: Vec<i32>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<i32> for Xmli32List {
    fn from_iter<I: IntoIterator<Item = i32>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OriginGroupMembers")]
pub struct OriginGroupMembers {
    #[serde(rename = "Items")]
    #[serde(default)]
    pub items: OriginGroupMemberList,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    pub quantity: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OriginGroupMemberList {
    #[serde(
        rename = "OriginGroupMember",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<OriginGroupMember>,
}
impl From<Vec<OriginGroupMember>> for OriginGroupMemberList {
    fn from(v: Vec<OriginGroupMember>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<OriginGroupMember> for OriginGroupMemberList {
    fn from_iter<I: IntoIterator<Item = OriginGroupMember>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<OriginGroupMember>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlOriginGroupMemberList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<OriginGroupMember>,
}

impl From<Vec<OriginGroupMember>> for XmlOriginGroupMemberList {
    fn from(v: Vec<OriginGroupMember>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<OriginGroupMember> for XmlOriginGroupMemberList {
    fn from_iter<I: IntoIterator<Item = OriginGroupMember>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OriginGroupMember")]
pub struct OriginGroupMember {
    #[serde(rename = "OriginId")]
    #[serde(default)]
    pub origin_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Origins")]
pub struct Origins {
    #[serde(rename = "Items")]
    #[serde(default)]
    pub items: OriginList,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    pub quantity: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OriginList {
    #[serde(rename = "Origin", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Origin>,
}
impl From<Vec<Origin>> for OriginList {
    fn from(v: Vec<Origin>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Origin> for OriginList {
    fn from_iter<I: IntoIterator<Item = Origin>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Origin>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlOriginList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Origin>,
}

impl From<Vec<Origin>> for XmlOriginList {
    fn from(v: Vec<Origin>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Origin> for XmlOriginList {
    fn from_iter<I: IntoIterator<Item = Origin>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Origin")]
pub struct Origin {
    #[serde(rename = "ConnectionAttempts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_attempts: Option<i32>,
    #[serde(rename = "ConnectionTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_timeout: Option<i32>,
    #[serde(rename = "CustomHeaders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_headers: Option<CustomHeaders>,
    #[serde(rename = "CustomOriginConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_origin_config: Option<CustomOriginConfig>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "OriginAccessControlId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_access_control_id: Option<String>,
    #[serde(rename = "OriginPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_path: Option<String>,
    #[serde(rename = "OriginShield")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_shield: Option<OriginShield>,
    #[serde(rename = "ResponseCompletionTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_completion_timeout: Option<i32>,
    #[serde(rename = "S3OriginConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_origin_config: Option<S3OriginConfig>,
    #[serde(rename = "VpcOriginConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_origin_config: Option<VpcOriginConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CustomHeaders")]
pub struct CustomHeaders {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<OriginCustomHeadersList>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    pub quantity: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OriginCustomHeadersList {
    #[serde(
        rename = "OriginCustomHeader",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<OriginCustomHeader>,
}
impl From<Vec<OriginCustomHeader>> for OriginCustomHeadersList {
    fn from(v: Vec<OriginCustomHeader>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<OriginCustomHeader> for OriginCustomHeadersList {
    fn from_iter<I: IntoIterator<Item = OriginCustomHeader>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<OriginCustomHeader>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlOriginCustomHeaderList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<OriginCustomHeader>,
}

impl From<Vec<OriginCustomHeader>> for XmlOriginCustomHeaderList {
    fn from(v: Vec<OriginCustomHeader>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<OriginCustomHeader> for XmlOriginCustomHeaderList {
    fn from_iter<I: IntoIterator<Item = OriginCustomHeader>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OriginCustomHeader")]
pub struct OriginCustomHeader {
    #[serde(rename = "HeaderName")]
    #[serde(default)]
    pub header_name: String,
    #[serde(rename = "HeaderValue")]
    #[serde(default)]
    pub header_value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CustomOriginConfig")]
pub struct CustomOriginConfig {
    #[serde(rename = "HTTPPort")]
    #[serde(default)]
    pub h_t_t_p_port: i32,
    #[serde(rename = "HTTPSPort")]
    #[serde(default)]
    pub h_t_t_p_s_port: i32,
    #[serde(rename = "IpAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(rename = "OriginKeepaliveTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_keepalive_timeout: Option<i32>,
    #[serde(rename = "OriginMtlsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_mtls_config: Option<OriginMtlsConfig>,
    #[serde(rename = "OriginProtocolPolicy")]
    #[serde(default)]
    pub origin_protocol_policy: String,
    #[serde(rename = "OriginReadTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_read_timeout: Option<i32>,
    #[serde(rename = "OriginSslProtocols")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_ssl_protocols: Option<OriginSslProtocols>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OriginMtlsConfig")]
pub struct OriginMtlsConfig {
    #[serde(rename = "ClientCertificateArn")]
    #[serde(default)]
    pub client_certificate_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OriginSslProtocols")]
pub struct OriginSslProtocols {
    #[serde(rename = "Items")]
    #[serde(default)]
    pub items: SslProtocolsList,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    pub quantity: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SslProtocolsList {
    #[serde(rename = "SslProtocol", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for SslProtocolsList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for SslProtocolsList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OriginShield")]
pub struct OriginShield {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "OriginShieldRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_shield_region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "S3OriginConfig")]
pub struct S3OriginConfig {
    #[serde(rename = "OriginAccessIdentity")]
    #[serde(default)]
    pub origin_access_identity: String,
    #[serde(rename = "OriginReadTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_read_timeout: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "VpcOriginConfig")]
pub struct VpcOriginConfig {
    #[serde(rename = "OriginKeepaliveTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_keepalive_timeout: Option<i32>,
    #[serde(rename = "OriginReadTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_read_timeout: Option<i32>,
    #[serde(rename = "OwnerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account_id: Option<String>,
    #[serde(rename = "VpcOriginId")]
    #[serde(default)]
    pub vpc_origin_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Restrictions")]
pub struct Restrictions {
    #[serde(rename = "GeoRestriction")]
    #[serde(default)]
    pub geo_restriction: GeoRestriction,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GeoRestriction")]
pub struct GeoRestriction {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<LocationList>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    pub quantity: i32,
    #[serde(rename = "RestrictionType")]
    #[serde(default)]
    pub restriction_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LocationList {
    #[serde(rename = "Location", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for LocationList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for LocationList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TenantConfig")]
pub struct TenantConfig {
    #[serde(rename = "ParameterDefinitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_definitions: Option<ParameterDefinitions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParameterDefinitions {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ParameterDefinition>,
}
impl From<Vec<ParameterDefinition>> for ParameterDefinitions {
    fn from(v: Vec<ParameterDefinition>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ParameterDefinition> for ParameterDefinitions {
    fn from_iter<I: IntoIterator<Item = ParameterDefinition>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ParameterDefinition>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlParameterDefinitionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ParameterDefinition>,
}

impl From<Vec<ParameterDefinition>> for XmlParameterDefinitionList {
    fn from(v: Vec<ParameterDefinition>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ParameterDefinition> for XmlParameterDefinitionList {
    fn from_iter<I: IntoIterator<Item = ParameterDefinition>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ParameterDefinition")]
pub struct ParameterDefinition {
    #[serde(rename = "Definition")]
    #[serde(default)]
    pub definition: ParameterDefinitionSchema,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ParameterDefinitionSchema")]
pub struct ParameterDefinitionSchema {
    #[serde(rename = "StringSchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_schema: Option<StringSchemaConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StringSchemaConfig")]
pub struct StringSchemaConfig {
    #[serde(rename = "Comment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "DefaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "Required")]
    #[serde(default)]
    pub required: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ViewerCertificate")]
pub struct ViewerCertificate {
    #[serde(rename = "ACMCertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_c_m_certificate_arn: Option<String>,
    #[serde(rename = "Certificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[serde(rename = "CertificateSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_source: Option<String>,
    #[serde(rename = "CloudFrontDefaultCertificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_front_default_certificate: Option<bool>,
    #[serde(rename = "IAMCertificateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_a_m_certificate_id: Option<String>,
    #[serde(rename = "MinimumProtocolVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_protocol_version: Option<String>,
    #[serde(rename = "SSLSupportMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_l_support_method: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ViewerMtlsConfig")]
pub struct ViewerMtlsConfig {
    #[serde(rename = "Mode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "TrustStoreConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_store_config: Option<TrustStoreConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TrustStoreConfig")]
pub struct TrustStoreConfig {
    #[serde(rename = "AdvertiseTrustStoreCaNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advertise_trust_store_ca_names: Option<bool>,
    #[serde(rename = "IgnoreCertificateExpiry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_certificate_expiry: Option<bool>,
    #[serde(rename = "TrustStoreId")]
    #[serde(default)]
    pub trust_store_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetResponseHeadersPolicyConfigResult")]
pub struct GetResponseHeadersPolicyConfigResult {
    #[serde(rename = "ResponseHeadersPolicyConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_headers_policy_config: Option<ResponseHeadersPolicyConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResponseHeadersPolicyConfig")]
pub struct ResponseHeadersPolicyConfig {
    #[serde(rename = "Comment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "CorsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cors_config: Option<ResponseHeadersPolicyCorsConfig>,
    #[serde(rename = "CustomHeadersConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_headers_config: Option<ResponseHeadersPolicyCustomHeadersConfig>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RemoveHeadersConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_headers_config: Option<ResponseHeadersPolicyRemoveHeadersConfig>,
    #[serde(rename = "SecurityHeadersConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_headers_config: Option<ResponseHeadersPolicySecurityHeadersConfig>,
    #[serde(rename = "ServerTimingHeadersConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_timing_headers_config: Option<ResponseHeadersPolicyServerTimingHeadersConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResponseHeadersPolicyCorsConfig")]
pub struct ResponseHeadersPolicyCorsConfig {
    #[serde(rename = "AccessControlAllowCredentials")]
    #[serde(default)]
    pub access_control_allow_credentials: bool,
    #[serde(rename = "AccessControlAllowHeaders")]
    #[serde(default)]
    pub access_control_allow_headers: ResponseHeadersPolicyAccessControlAllowHeaders,
    #[serde(rename = "AccessControlAllowMethods")]
    #[serde(default)]
    pub access_control_allow_methods: ResponseHeadersPolicyAccessControlAllowMethods,
    #[serde(rename = "AccessControlAllowOrigins")]
    #[serde(default)]
    pub access_control_allow_origins: ResponseHeadersPolicyAccessControlAllowOrigins,
    #[serde(rename = "AccessControlExposeHeaders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_expose_headers: Option<ResponseHeadersPolicyAccessControlExposeHeaders>,
    #[serde(rename = "AccessControlMaxAgeSec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_max_age_sec: Option<i32>,
    #[serde(rename = "OriginOverride")]
    #[serde(default)]
    pub origin_override: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResponseHeadersPolicyAccessControlAllowHeaders")]
pub struct ResponseHeadersPolicyAccessControlAllowHeaders {
    #[serde(rename = "Items")]
    #[serde(default)]
    pub items: AccessControlAllowHeadersList,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    pub quantity: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessControlAllowHeadersList {
    #[serde(rename = "Header", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for AccessControlAllowHeadersList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for AccessControlAllowHeadersList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResponseHeadersPolicyAccessControlAllowMethods")]
pub struct ResponseHeadersPolicyAccessControlAllowMethods {
    #[serde(rename = "Items")]
    #[serde(default)]
    pub items: AccessControlAllowMethodsList,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    pub quantity: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessControlAllowMethodsList {
    #[serde(rename = "Method", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for AccessControlAllowMethodsList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for AccessControlAllowMethodsList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResponseHeadersPolicyAccessControlAllowOrigins")]
pub struct ResponseHeadersPolicyAccessControlAllowOrigins {
    #[serde(rename = "Items")]
    #[serde(default)]
    pub items: AccessControlAllowOriginsList,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    pub quantity: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessControlAllowOriginsList {
    #[serde(rename = "Origin", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for AccessControlAllowOriginsList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for AccessControlAllowOriginsList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResponseHeadersPolicyAccessControlExposeHeaders")]
pub struct ResponseHeadersPolicyAccessControlExposeHeaders {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<AccessControlExposeHeadersList>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    pub quantity: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessControlExposeHeadersList {
    #[serde(rename = "Header", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for AccessControlExposeHeadersList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for AccessControlExposeHeadersList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResponseHeadersPolicyCustomHeadersConfig")]
pub struct ResponseHeadersPolicyCustomHeadersConfig {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<ResponseHeadersPolicyCustomHeaderList>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    pub quantity: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResponseHeadersPolicyCustomHeaderList {
    #[serde(
        rename = "ResponseHeadersPolicyCustomHeader",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ResponseHeadersPolicyCustomHeader>,
}
impl From<Vec<ResponseHeadersPolicyCustomHeader>> for ResponseHeadersPolicyCustomHeaderList {
    fn from(v: Vec<ResponseHeadersPolicyCustomHeader>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ResponseHeadersPolicyCustomHeader> for ResponseHeadersPolicyCustomHeaderList {
    fn from_iter<I: IntoIterator<Item = ResponseHeadersPolicyCustomHeader>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ResponseHeadersPolicyCustomHeader>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlResponseHeadersPolicyCustomHeaderList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ResponseHeadersPolicyCustomHeader>,
}

impl From<Vec<ResponseHeadersPolicyCustomHeader>> for XmlResponseHeadersPolicyCustomHeaderList {
    fn from(v: Vec<ResponseHeadersPolicyCustomHeader>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ResponseHeadersPolicyCustomHeader> for XmlResponseHeadersPolicyCustomHeaderList {
    fn from_iter<I: IntoIterator<Item = ResponseHeadersPolicyCustomHeader>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResponseHeadersPolicyCustomHeader")]
pub struct ResponseHeadersPolicyCustomHeader {
    #[serde(rename = "Header")]
    #[serde(default)]
    pub header: String,
    #[serde(rename = "Override")]
    #[serde(default)]
    pub r#override: bool,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResponseHeadersPolicyRemoveHeadersConfig")]
pub struct ResponseHeadersPolicyRemoveHeadersConfig {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<ResponseHeadersPolicyRemoveHeaderList>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    pub quantity: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResponseHeadersPolicyRemoveHeaderList {
    #[serde(
        rename = "ResponseHeadersPolicyRemoveHeader",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ResponseHeadersPolicyRemoveHeader>,
}
impl From<Vec<ResponseHeadersPolicyRemoveHeader>> for ResponseHeadersPolicyRemoveHeaderList {
    fn from(v: Vec<ResponseHeadersPolicyRemoveHeader>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ResponseHeadersPolicyRemoveHeader> for ResponseHeadersPolicyRemoveHeaderList {
    fn from_iter<I: IntoIterator<Item = ResponseHeadersPolicyRemoveHeader>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ResponseHeadersPolicyRemoveHeader>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlResponseHeadersPolicyRemoveHeaderList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ResponseHeadersPolicyRemoveHeader>,
}

impl From<Vec<ResponseHeadersPolicyRemoveHeader>> for XmlResponseHeadersPolicyRemoveHeaderList {
    fn from(v: Vec<ResponseHeadersPolicyRemoveHeader>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ResponseHeadersPolicyRemoveHeader> for XmlResponseHeadersPolicyRemoveHeaderList {
    fn from_iter<I: IntoIterator<Item = ResponseHeadersPolicyRemoveHeader>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResponseHeadersPolicyRemoveHeader")]
pub struct ResponseHeadersPolicyRemoveHeader {
    #[serde(rename = "Header")]
    #[serde(default)]
    pub header: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResponseHeadersPolicySecurityHeadersConfig")]
pub struct ResponseHeadersPolicySecurityHeadersConfig {
    #[serde(rename = "ContentSecurityPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_security_policy: Option<ResponseHeadersPolicyContentSecurityPolicy>,
    #[serde(rename = "ContentTypeOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type_options: Option<ResponseHeadersPolicyContentTypeOptions>,
    #[serde(rename = "FrameOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_options: Option<ResponseHeadersPolicyFrameOptions>,
    #[serde(rename = "ReferrerPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrer_policy: Option<ResponseHeadersPolicyReferrerPolicy>,
    #[serde(rename = "StrictTransportSecurity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict_transport_security: Option<ResponseHeadersPolicyStrictTransportSecurity>,
    #[serde(rename = "XSSProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_s_s_protection: Option<ResponseHeadersPolicyXSSProtection>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResponseHeadersPolicyContentSecurityPolicy")]
pub struct ResponseHeadersPolicyContentSecurityPolicy {
    #[serde(rename = "ContentSecurityPolicy")]
    #[serde(default)]
    pub content_security_policy: String,
    #[serde(rename = "Override")]
    #[serde(default)]
    pub r#override: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResponseHeadersPolicyContentTypeOptions")]
pub struct ResponseHeadersPolicyContentTypeOptions {
    #[serde(rename = "Override")]
    #[serde(default)]
    pub r#override: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResponseHeadersPolicyFrameOptions")]
pub struct ResponseHeadersPolicyFrameOptions {
    #[serde(rename = "FrameOption")]
    #[serde(default)]
    pub frame_option: String,
    #[serde(rename = "Override")]
    #[serde(default)]
    pub r#override: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResponseHeadersPolicyReferrerPolicy")]
pub struct ResponseHeadersPolicyReferrerPolicy {
    #[serde(rename = "Override")]
    #[serde(default)]
    pub r#override: bool,
    #[serde(rename = "ReferrerPolicy")]
    #[serde(default)]
    pub referrer_policy: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResponseHeadersPolicyStrictTransportSecurity")]
pub struct ResponseHeadersPolicyStrictTransportSecurity {
    #[serde(rename = "AccessControlMaxAgeSec")]
    #[serde(default)]
    pub access_control_max_age_sec: i32,
    #[serde(rename = "IncludeSubdomains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_subdomains: Option<bool>,
    #[serde(rename = "Override")]
    #[serde(default)]
    pub r#override: bool,
    #[serde(rename = "Preload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preload: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResponseHeadersPolicyXSSProtection")]
pub struct ResponseHeadersPolicyXSSProtection {
    #[serde(rename = "ModeBlock")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode_block: Option<bool>,
    #[serde(rename = "Override")]
    #[serde(default)]
    pub r#override: bool,
    #[serde(rename = "Protection")]
    #[serde(default)]
    pub protection: bool,
    #[serde(rename = "ReportUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResponseHeadersPolicyServerTimingHeadersConfig")]
pub struct ResponseHeadersPolicyServerTimingHeadersConfig {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "SamplingRate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_rate: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutResourcePolicyResult")]
pub struct PutResourcePolicyResult {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateContinuousDeploymentPolicyRequest")]
pub struct CreateContinuousDeploymentPolicyRequest {
    #[serde(rename = "ContinuousDeploymentPolicyConfig")]
    #[serde(default)]
    pub continuous_deployment_policy_config: ContinuousDeploymentPolicyConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ContinuousDeploymentPolicyConfig")]
pub struct ContinuousDeploymentPolicyConfig {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "StagingDistributionDnsNames")]
    #[serde(default)]
    pub staging_distribution_dns_names: StagingDistributionDnsNames,
    #[serde(rename = "TrafficConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_config: Option<TrafficConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StagingDistributionDnsNames")]
pub struct StagingDistributionDnsNames {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<StagingDistributionDnsNameList>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    pub quantity: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StagingDistributionDnsNameList {
    #[serde(rename = "DnsName", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for StagingDistributionDnsNameList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for StagingDistributionDnsNameList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TrafficConfig")]
pub struct TrafficConfig {
    #[serde(rename = "SingleHeaderConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_header_config: Option<ContinuousDeploymentSingleHeaderConfig>,
    #[serde(rename = "SingleWeightConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_weight_config: Option<ContinuousDeploymentSingleWeightConfig>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ContinuousDeploymentSingleHeaderConfig")]
pub struct ContinuousDeploymentSingleHeaderConfig {
    #[serde(rename = "Header")]
    #[serde(default)]
    pub header: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ContinuousDeploymentSingleWeightConfig")]
pub struct ContinuousDeploymentSingleWeightConfig {
    #[serde(rename = "SessionStickinessConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_stickiness_config: Option<SessionStickinessConfig>,
    #[serde(rename = "Weight")]
    #[serde(default)]
    pub weight: f32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SessionStickinessConfig")]
pub struct SessionStickinessConfig {
    #[serde(rename = "IdleTTL")]
    #[serde(default)]
    pub idle_t_t_l: i32,
    #[serde(rename = "MaximumTTL")]
    #[serde(default)]
    pub maximum_t_t_l: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteTrustStoreRequest")]
pub struct DeleteTrustStoreRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    pub if_match: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetFieldLevelEncryptionRequest")]
pub struct GetFieldLevelEncryptionRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateResponseHeadersPolicyRequest")]
pub struct UpdateResponseHeadersPolicyRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match: Option<String>,
    #[serde(rename = "ResponseHeadersPolicyConfig")]
    #[serde(default)]
    pub response_headers_policy_config: ResponseHeadersPolicyConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DistributionIdOwnerList")]
pub struct DistributionIdOwnerList {
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<DistributionIdOwnerItemList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DistributionIdOwnerItemList {
    #[serde(
        rename = "DistributionIdOwner",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DistributionIdOwner>,
}
impl From<Vec<DistributionIdOwner>> for DistributionIdOwnerItemList {
    fn from(v: Vec<DistributionIdOwner>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DistributionIdOwner> for DistributionIdOwnerItemList {
    fn from_iter<I: IntoIterator<Item = DistributionIdOwner>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DistributionIdOwner>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDistributionIdOwnerList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DistributionIdOwner>,
}

impl From<Vec<DistributionIdOwner>> for XmlDistributionIdOwnerList {
    fn from(v: Vec<DistributionIdOwner>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DistributionIdOwner> for XmlDistributionIdOwnerList {
    fn from_iter<I: IntoIterator<Item = DistributionIdOwner>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DistributionIdOwner")]
pub struct DistributionIdOwner {
    #[serde(rename = "DistributionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_id: Option<String>,
    #[serde(rename = "OwnerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetOriginAccessControlResult")]
pub struct GetOriginAccessControlResult {
    #[serde(rename = "OriginAccessControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_access_control: Option<OriginAccessControl>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OriginAccessControl")]
pub struct OriginAccessControl {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "OriginAccessControlConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_access_control_config: Option<OriginAccessControlConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OriginAccessControlConfig")]
pub struct OriginAccessControlConfig {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OriginAccessControlOriginType")]
    #[serde(default)]
    pub origin_access_control_origin_type: String,
    #[serde(rename = "SigningBehavior")]
    #[serde(default)]
    pub signing_behavior: String,
    #[serde(rename = "SigningProtocol")]
    #[serde(default)]
    pub signing_protocol: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteCloudFrontOriginAccessIdentityRequest")]
pub struct DeleteCloudFrontOriginAccessIdentityRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateAnycastIpListRequest")]
pub struct CreateAnycastIpListRequest {
    #[serde(rename = "IpAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(rename = "IpCount")]
    #[serde(default)]
    pub ip_count: i32,
    #[serde(rename = "IpamCidrConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipam_cidr_configs: Option<IpamCidrConfigList>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IpamCidrConfigList {
    #[serde(
        rename = "IpamCidrConfig",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<IpamCidrConfig>,
}
impl From<Vec<IpamCidrConfig>> for IpamCidrConfigList {
    fn from(v: Vec<IpamCidrConfig>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<IpamCidrConfig> for IpamCidrConfigList {
    fn from_iter<I: IntoIterator<Item = IpamCidrConfig>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<IpamCidrConfig>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlIpamCidrConfigList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<IpamCidrConfig>,
}

impl From<Vec<IpamCidrConfig>> for XmlIpamCidrConfigList {
    fn from(v: Vec<IpamCidrConfig>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<IpamCidrConfig> for XmlIpamCidrConfigList {
    fn from_iter<I: IntoIterator<Item = IpamCidrConfig>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "IpamCidrConfig")]
pub struct IpamCidrConfig {
    #[serde(rename = "AnycastIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anycast_ip: Option<String>,
    #[serde(rename = "Cidr")]
    #[serde(default)]
    pub cidr: String,
    #[serde(rename = "IpamPoolArn")]
    #[serde(default)]
    pub ipam_pool_arn: String,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Tags")]
pub struct Tags {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagList {
    #[serde(rename = "Tag", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Tag>,
}
impl From<Vec<Tag>> for TagList {
    fn from(v: Vec<Tag>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Tag> for TagList {
    fn from_iter<I: IntoIterator<Item = Tag>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Tag>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTagList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Tag>,
}

impl From<Vec<Tag>> for XmlTagList {
    fn from(v: Vec<Tag>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Tag> for XmlTagList {
    fn from_iter<I: IntoIterator<Item = Tag>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Tag")]
pub struct Tag {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DistributionTenant")]
pub struct DistributionTenant {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ConnectionGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_group_id: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "Customizations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customizations: Option<Customizations>,
    #[serde(rename = "DistributionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_id: Option<String>,
    #[serde(rename = "Domains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<DomainResultList>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Parameters>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Customizations")]
pub struct Customizations {
    #[serde(rename = "Certificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<Certificate>,
    #[serde(rename = "GeoRestrictions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_restrictions: Option<GeoRestrictionCustomization>,
    #[serde(rename = "WebAcl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_acl: Option<WebAclCustomization>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Certificate")]
pub struct Certificate {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GeoRestrictionCustomization")]
pub struct GeoRestrictionCustomization {
    #[serde(rename = "Locations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<LocationList>,
    #[serde(rename = "RestrictionType")]
    #[serde(default)]
    pub restriction_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "WebAclCustomization")]
pub struct WebAclCustomization {
    #[serde(rename = "Action")]
    #[serde(default)]
    pub action: String,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainResultList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<DomainResult>,
}
impl From<Vec<DomainResult>> for DomainResultList {
    fn from(v: Vec<DomainResult>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DomainResult> for DomainResultList {
    fn from_iter<I: IntoIterator<Item = DomainResult>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DomainResult>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDomainResultList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DomainResult>,
}

impl From<Vec<DomainResult>> for XmlDomainResultList {
    fn from(v: Vec<DomainResult>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DomainResult> for XmlDomainResultList {
    fn from_iter<I: IntoIterator<Item = DomainResult>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DomainResult")]
pub struct DomainResult {
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Parameters {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Parameter>,
}
impl From<Vec<Parameter>> for Parameters {
    fn from(v: Vec<Parameter>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Parameter> for Parameters {
    fn from_iter<I: IntoIterator<Item = Parameter>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Parameter>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlParameterList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Parameter>,
}

impl From<Vec<Parameter>> for XmlParameterList {
    fn from(v: Vec<Parameter>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Parameter> for XmlParameterList {
    fn from_iter<I: IntoIterator<Item = Parameter>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Parameter")]
pub struct Parameter {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetCloudFrontOriginAccessIdentityRequest")]
pub struct GetCloudFrontOriginAccessIdentityRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListFieldLevelEncryptionProfilesResult")]
pub struct ListFieldLevelEncryptionProfilesResult {
    #[serde(rename = "FieldLevelEncryptionProfileList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_level_encryption_profile_list: Option<FieldLevelEncryptionProfileList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "FieldLevelEncryptionProfileList")]
pub struct FieldLevelEncryptionProfileList {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<FieldLevelEncryptionProfileSummaryList>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FieldLevelEncryptionProfileSummaryList {
    #[serde(
        rename = "FieldLevelEncryptionProfileSummary",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<FieldLevelEncryptionProfileSummary>,
}
impl From<Vec<FieldLevelEncryptionProfileSummary>> for FieldLevelEncryptionProfileSummaryList {
    fn from(v: Vec<FieldLevelEncryptionProfileSummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<FieldLevelEncryptionProfileSummary> for FieldLevelEncryptionProfileSummaryList {
    fn from_iter<I: IntoIterator<Item = FieldLevelEncryptionProfileSummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<FieldLevelEncryptionProfileSummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlFieldLevelEncryptionProfileSummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<FieldLevelEncryptionProfileSummary>,
}

impl From<Vec<FieldLevelEncryptionProfileSummary>> for XmlFieldLevelEncryptionProfileSummaryList {
    fn from(v: Vec<FieldLevelEncryptionProfileSummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<FieldLevelEncryptionProfileSummary>
    for XmlFieldLevelEncryptionProfileSummaryList
{
    fn from_iter<I: IntoIterator<Item = FieldLevelEncryptionProfileSummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "FieldLevelEncryptionProfileSummary")]
pub struct FieldLevelEncryptionProfileSummary {
    #[serde(rename = "Comment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "EncryptionEntities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_entities: Option<EncryptionEntities>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EncryptionEntities")]
pub struct EncryptionEntities {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<EncryptionEntityList>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    pub quantity: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptionEntityList {
    #[serde(
        rename = "EncryptionEntity",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<EncryptionEntity>,
}
impl From<Vec<EncryptionEntity>> for EncryptionEntityList {
    fn from(v: Vec<EncryptionEntity>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<EncryptionEntity> for EncryptionEntityList {
    fn from_iter<I: IntoIterator<Item = EncryptionEntity>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<EncryptionEntity>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlEncryptionEntityList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<EncryptionEntity>,
}

impl From<Vec<EncryptionEntity>> for XmlEncryptionEntityList {
    fn from(v: Vec<EncryptionEntity>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<EncryptionEntity> for XmlEncryptionEntityList {
    fn from_iter<I: IntoIterator<Item = EncryptionEntity>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EncryptionEntity")]
pub struct EncryptionEntity {
    #[serde(rename = "FieldPatterns")]
    #[serde(default)]
    pub field_patterns: FieldPatterns,
    #[serde(rename = "ProviderId")]
    #[serde(default)]
    pub provider_id: String,
    #[serde(rename = "PublicKeyId")]
    #[serde(default)]
    pub public_key_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "FieldPatterns")]
pub struct FieldPatterns {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<FieldPatternList>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    pub quantity: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FieldPatternList {
    #[serde(
        rename = "FieldPattern",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for FieldPatternList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for FieldPatternList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListStreamingDistributionsResult")]
pub struct ListStreamingDistributionsResult {
    #[serde(rename = "StreamingDistributionList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streaming_distribution_list: Option<StreamingDistributionList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StreamingDistributionList")]
pub struct StreamingDistributionList {
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<StreamingDistributionSummaryList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StreamingDistributionSummaryList {
    #[serde(
        rename = "StreamingDistributionSummary",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<StreamingDistributionSummary>,
}
impl From<Vec<StreamingDistributionSummary>> for StreamingDistributionSummaryList {
    fn from(v: Vec<StreamingDistributionSummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<StreamingDistributionSummary> for StreamingDistributionSummaryList {
    fn from_iter<I: IntoIterator<Item = StreamingDistributionSummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<StreamingDistributionSummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlStreamingDistributionSummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<StreamingDistributionSummary>,
}

impl From<Vec<StreamingDistributionSummary>> for XmlStreamingDistributionSummaryList {
    fn from(v: Vec<StreamingDistributionSummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<StreamingDistributionSummary> for XmlStreamingDistributionSummaryList {
    fn from_iter<I: IntoIterator<Item = StreamingDistributionSummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StreamingDistributionSummary")]
pub struct StreamingDistributionSummary {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "Aliases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Aliases>,
    #[serde(rename = "Comment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(rename = "PriceClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_class: Option<String>,
    #[serde(rename = "S3Origin")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_origin: Option<S3Origin>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TrustedSigners")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_signers: Option<TrustedSigners>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "S3Origin")]
pub struct S3Origin {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "OriginAccessIdentity")]
    #[serde(default)]
    pub origin_access_identity: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetPublicKeyRequest")]
pub struct GetPublicKeyRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTagsForResourceRequest")]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "Resource")]
    #[serde(default)]
    pub resource: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetFieldLevelEncryptionProfileRequest")]
pub struct GetFieldLevelEncryptionProfileRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListDistributionsByTrustStoreRequest")]
pub struct ListDistributionsByTrustStoreRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "TrustStoreIdentifier")]
    #[serde(default)]
    pub trust_store_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetFieldLevelEncryptionProfileConfigRequest")]
pub struct GetFieldLevelEncryptionProfileConfigRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ConnectionGroup")]
pub struct ConnectionGroup {
    #[serde(rename = "AnycastIpListId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anycast_ip_list_id: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Ipv6Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_enabled: Option<bool>,
    #[serde(rename = "IsDefault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RoutingEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_endpoint: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OriginAccessControlList")]
pub struct OriginAccessControlList {
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<OriginAccessControlSummaryList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OriginAccessControlSummaryList {
    #[serde(
        rename = "OriginAccessControlSummary",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<OriginAccessControlSummary>,
}
impl From<Vec<OriginAccessControlSummary>> for OriginAccessControlSummaryList {
    fn from(v: Vec<OriginAccessControlSummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<OriginAccessControlSummary> for OriginAccessControlSummaryList {
    fn from_iter<I: IntoIterator<Item = OriginAccessControlSummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<OriginAccessControlSummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlOriginAccessControlSummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<OriginAccessControlSummary>,
}

impl From<Vec<OriginAccessControlSummary>> for XmlOriginAccessControlSummaryList {
    fn from(v: Vec<OriginAccessControlSummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<OriginAccessControlSummary> for XmlOriginAccessControlSummaryList {
    fn from_iter<I: IntoIterator<Item = OriginAccessControlSummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OriginAccessControlSummary")]
pub struct OriginAccessControlSummary {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OriginAccessControlOriginType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_access_control_origin_type: Option<String>,
    #[serde(rename = "SigningBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_behavior: Option<String>,
    #[serde(rename = "SigningProtocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_protocol: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListResponseHeadersPoliciesResult")]
pub struct ListResponseHeadersPoliciesResult {
    #[serde(rename = "ResponseHeadersPolicyList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_headers_policy_list: Option<ResponseHeadersPolicyList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResponseHeadersPolicyList")]
pub struct ResponseHeadersPolicyList {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<ResponseHeadersPolicySummaryList>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResponseHeadersPolicySummaryList {
    #[serde(
        rename = "ResponseHeadersPolicySummary",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ResponseHeadersPolicySummary>,
}
impl From<Vec<ResponseHeadersPolicySummary>> for ResponseHeadersPolicySummaryList {
    fn from(v: Vec<ResponseHeadersPolicySummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ResponseHeadersPolicySummary> for ResponseHeadersPolicySummaryList {
    fn from_iter<I: IntoIterator<Item = ResponseHeadersPolicySummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ResponseHeadersPolicySummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlResponseHeadersPolicySummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ResponseHeadersPolicySummary>,
}

impl From<Vec<ResponseHeadersPolicySummary>> for XmlResponseHeadersPolicySummaryList {
    fn from(v: Vec<ResponseHeadersPolicySummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ResponseHeadersPolicySummary> for XmlResponseHeadersPolicySummaryList {
    fn from_iter<I: IntoIterator<Item = ResponseHeadersPolicySummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResponseHeadersPolicySummary")]
pub struct ResponseHeadersPolicySummary {
    #[serde(rename = "ResponseHeadersPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_headers_policy: Option<ResponseHeadersPolicy>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResponseHeadersPolicy")]
pub struct ResponseHeadersPolicy {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(rename = "ResponseHeadersPolicyConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_headers_policy_config: Option<ResponseHeadersPolicyConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetConnectionGroupByRoutingEndpointRequest")]
pub struct GetConnectionGroupByRoutingEndpointRequest {
    #[serde(rename = "RoutingEndpoint")]
    #[serde(default)]
    pub routing_endpoint: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListDistributionsByWebACLIdRequest")]
pub struct ListDistributionsByWebACLIdRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "WebACLId")]
    #[serde(default)]
    pub web_a_c_l_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DistributionConfigWithTags")]
pub struct DistributionConfigWithTags {
    #[serde(rename = "DistributionConfig")]
    #[serde(default)]
    pub distribution_config: DistributionConfig,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Tags,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateConnectionGroupRequest")]
pub struct CreateConnectionGroupRequest {
    #[serde(rename = "AnycastIpListId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anycast_ip_list_id: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "Ipv6Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_enabled: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TagKeys")]
pub struct TagKeys {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<TagKeyList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagKeyList {
    #[serde(rename = "Key", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for TagKeyList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for TagKeyList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateInvalidationForDistributionTenantResult")]
pub struct CreateInvalidationForDistributionTenantResult {
    #[serde(rename = "Invalidation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalidation: Option<Invalidation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListDistributionsByKeyGroupRequest")]
pub struct ListDistributionsByKeyGroupRequest {
    #[serde(rename = "KeyGroupId")]
    #[serde(default)]
    pub key_group_id: String,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateContinuousDeploymentPolicyRequest")]
pub struct UpdateContinuousDeploymentPolicyRequest {
    #[serde(rename = "ContinuousDeploymentPolicyConfig")]
    #[serde(default)]
    pub continuous_deployment_policy_config: ContinuousDeploymentPolicyConfig,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteFieldLevelEncryptionConfigRequest")]
pub struct DeleteFieldLevelEncryptionConfigRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdatePublicKeyRequest")]
pub struct UpdatePublicKeyRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match: Option<String>,
    #[serde(rename = "PublicKeyConfig")]
    #[serde(default)]
    pub public_key_config: PublicKeyConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PublicKeyConfig")]
pub struct PublicKeyConfig {
    #[serde(rename = "CallerReference")]
    #[serde(default)]
    pub caller_reference: String,
    #[serde(rename = "Comment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "EncodedKey")]
    #[serde(default)]
    pub encoded_key: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateConnectionFunctionResult")]
pub struct CreateConnectionFunctionResult {
    #[serde(rename = "ConnectionFunctionSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_function_summary: Option<ConnectionFunctionSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ConnectionFunctionSummary")]
pub struct ConnectionFunctionSummary {
    #[serde(rename = "ConnectionFunctionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_function_arn: Option<String>,
    #[serde(rename = "ConnectionFunctionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_function_config: Option<FunctionConfig>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Stage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PublicKeyList")]
pub struct PublicKeyList {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<PublicKeySummaryList>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PublicKeySummaryList {
    #[serde(
        rename = "PublicKeySummary",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<PublicKeySummary>,
}
impl From<Vec<PublicKeySummary>> for PublicKeySummaryList {
    fn from(v: Vec<PublicKeySummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<PublicKeySummary> for PublicKeySummaryList {
    fn from_iter<I: IntoIterator<Item = PublicKeySummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<PublicKeySummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlPublicKeySummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<PublicKeySummary>,
}

impl From<Vec<PublicKeySummary>> for XmlPublicKeySummaryList {
    fn from(v: Vec<PublicKeySummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<PublicKeySummary> for XmlPublicKeySummaryList {
    fn from_iter<I: IntoIterator<Item = PublicKeySummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PublicKeySummary")]
pub struct PublicKeySummary {
    #[serde(rename = "Comment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "EncodedKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoded_key: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateFunctionRequest")]
pub struct UpdateFunctionRequest {
    #[serde(rename = "FunctionCode")]
    #[serde(default)]
    pub function_code: String,
    #[serde(rename = "FunctionConfig")]
    #[serde(default)]
    pub function_config: FunctionConfig,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    pub if_match: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetManagedCertificateDetailsRequest")]
pub struct GetManagedCertificateDetailsRequest {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateInvalidationRequest")]
pub struct CreateInvalidationRequest {
    #[serde(rename = "DistributionId")]
    #[serde(default)]
    pub distribution_id: String,
    #[serde(rename = "InvalidationBatch")]
    #[serde(default)]
    pub invalidation_batch: InvalidationBatch,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDistributionTenantRequest")]
pub struct CreateDistributionTenantRequest {
    #[serde(rename = "ConnectionGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_group_id: Option<String>,
    #[serde(rename = "Customizations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customizations: Option<Customizations>,
    #[serde(rename = "DistributionId")]
    #[serde(default)]
    pub distribution_id: String,
    #[serde(rename = "Domains")]
    #[serde(default)]
    pub domains: DomainList,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "ManagedCertificateRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_certificate_request: Option<ManagedCertificateRequest>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Parameters>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<DomainItem>,
}
impl From<Vec<DomainItem>> for DomainList {
    fn from(v: Vec<DomainItem>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DomainItem> for DomainList {
    fn from_iter<I: IntoIterator<Item = DomainItem>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DomainItem>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDomainItemList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DomainItem>,
}

impl From<Vec<DomainItem>> for XmlDomainItemList {
    fn from(v: Vec<DomainItem>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DomainItem> for XmlDomainItemList {
    fn from_iter<I: IntoIterator<Item = DomainItem>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DomainItem")]
pub struct DomainItem {
    #[serde(rename = "Domain")]
    #[serde(default)]
    pub domain: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ManagedCertificateRequest")]
pub struct ManagedCertificateRequest {
    #[serde(rename = "CertificateTransparencyLoggingPreference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_transparency_logging_preference: Option<String>,
    #[serde(rename = "PrimaryDomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_domain_name: Option<String>,
    #[serde(rename = "ValidationTokenHost")]
    #[serde(default)]
    pub validation_token_host: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateVpcOriginRequest")]
pub struct CreateVpcOriginRequest {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[serde(rename = "VpcOriginEndpointConfig")]
    #[serde(default)]
    pub vpc_origin_endpoint_config: VpcOriginEndpointConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "VpcOriginEndpointConfig")]
pub struct VpcOriginEndpointConfig {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "HTTPPort")]
    #[serde(default)]
    pub h_t_t_p_port: i32,
    #[serde(rename = "HTTPSPort")]
    #[serde(default)]
    pub h_t_t_p_s_port: i32,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OriginProtocolPolicy")]
    #[serde(default)]
    pub origin_protocol_policy: String,
    #[serde(rename = "OriginSslProtocols")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_ssl_protocols: Option<OriginSslProtocols>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListDistributionsByOwnedResourceRequest")]
pub struct ListDistributionsByOwnedResourceRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListDistributionsByConnectionFunctionRequest")]
pub struct ListDistributionsByConnectionFunctionRequest {
    #[serde(rename = "ConnectionFunctionIdentifier")]
    #[serde(default)]
    pub connection_function_identifier: String,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListResponseHeadersPoliciesRequest")]
pub struct ListResponseHeadersPoliciesRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "KeyGroupList")]
pub struct KeyGroupList {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<KeyGroupSummaryList>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KeyGroupSummaryList {
    #[serde(
        rename = "KeyGroupSummary",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<KeyGroupSummary>,
}
impl From<Vec<KeyGroupSummary>> for KeyGroupSummaryList {
    fn from(v: Vec<KeyGroupSummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<KeyGroupSummary> for KeyGroupSummaryList {
    fn from_iter<I: IntoIterator<Item = KeyGroupSummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<KeyGroupSummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlKeyGroupSummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<KeyGroupSummary>,
}

impl From<Vec<KeyGroupSummary>> for XmlKeyGroupSummaryList {
    fn from(v: Vec<KeyGroupSummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<KeyGroupSummary> for XmlKeyGroupSummaryList {
    fn from_iter<I: IntoIterator<Item = KeyGroupSummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "KeyGroupSummary")]
pub struct KeyGroupSummary {
    #[serde(rename = "KeyGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_group: Option<KeyGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "KeyGroup")]
pub struct KeyGroup {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "KeyGroupConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_group_config: Option<KeyGroupConfig>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "KeyGroupConfig")]
pub struct KeyGroupConfig {
    #[serde(rename = "Comment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "Items")]
    #[serde(default)]
    pub items: PublicKeyIdList,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PublicKeyIdList {
    #[serde(rename = "PublicKey", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for PublicKeyIdList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for PublicKeyIdList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteConnectionGroupRequest")]
pub struct DeleteConnectionGroupRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    pub if_match: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetCachePolicyConfigResult")]
pub struct GetCachePolicyConfigResult {
    #[serde(rename = "CachePolicyConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_policy_config: Option<CachePolicyConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CachePolicyConfig")]
pub struct CachePolicyConfig {
    #[serde(rename = "Comment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "DefaultTTL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_t_t_l: Option<i64>,
    #[serde(rename = "MaxTTL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_t_t_l: Option<i64>,
    #[serde(rename = "MinTTL")]
    #[serde(default)]
    pub min_t_t_l: i64,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ParametersInCacheKeyAndForwardedToOrigin")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters_in_cache_key_and_forwarded_to_origin:
        Option<ParametersInCacheKeyAndForwardedToOrigin>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ParametersInCacheKeyAndForwardedToOrigin")]
pub struct ParametersInCacheKeyAndForwardedToOrigin {
    #[serde(rename = "CookiesConfig")]
    #[serde(default)]
    pub cookies_config: CachePolicyCookiesConfig,
    #[serde(rename = "EnableAcceptEncodingBrotli")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_accept_encoding_brotli: Option<bool>,
    #[serde(rename = "EnableAcceptEncodingGzip")]
    #[serde(default)]
    pub enable_accept_encoding_gzip: bool,
    #[serde(rename = "HeadersConfig")]
    #[serde(default)]
    pub headers_config: CachePolicyHeadersConfig,
    #[serde(rename = "QueryStringsConfig")]
    #[serde(default)]
    pub query_strings_config: CachePolicyQueryStringsConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CachePolicyCookiesConfig")]
pub struct CachePolicyCookiesConfig {
    #[serde(rename = "CookieBehavior")]
    #[serde(default)]
    pub cookie_behavior: String,
    #[serde(rename = "Cookies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookies: Option<CookieNames>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CachePolicyHeadersConfig")]
pub struct CachePolicyHeadersConfig {
    #[serde(rename = "HeaderBehavior")]
    #[serde(default)]
    pub header_behavior: String,
    #[serde(rename = "Headers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Headers>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CachePolicyQueryStringsConfig")]
pub struct CachePolicyQueryStringsConfig {
    #[serde(rename = "QueryStringBehavior")]
    #[serde(default)]
    pub query_string_behavior: String,
    #[serde(rename = "QueryStrings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_strings: Option<QueryStringNames>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateTrustStoreRequest")]
pub struct UpdateTrustStoreRequest {
    #[serde(rename = "CaCertificatesBundleSource")]
    #[serde(default)]
    pub ca_certificates_bundle_source: CaCertificatesBundleSource,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    pub if_match: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CaCertificatesBundleSource")]
pub struct CaCertificatesBundleSource {
    #[serde(rename = "CaCertificatesBundleS3Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_certificates_bundle_s3_location: Option<CaCertificatesBundleS3Location>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CaCertificatesBundleS3Location")]
pub struct CaCertificatesBundleS3Location {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Region")]
    #[serde(default)]
    pub region: String,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "VpcOrigin")]
pub struct VpcOrigin {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "VpcOriginEndpointConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_origin_endpoint_config: Option<VpcOriginEndpointConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetKeyGroupConfigResult")]
pub struct GetKeyGroupConfigResult {
    #[serde(rename = "KeyGroupConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_group_config: Option<KeyGroupConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetOriginAccessControlConfigRequest")]
pub struct GetOriginAccessControlConfigRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListVpcOriginsResult")]
pub struct ListVpcOriginsResult {
    #[serde(rename = "VpcOriginList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_origin_list: Option<VpcOriginList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetInvalidationForDistributionTenantRequest")]
pub struct GetInvalidationForDistributionTenantRequest {
    #[serde(rename = "DistributionTenantId")]
    #[serde(default)]
    pub distribution_tenant_id: String,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetKeyGroupConfigRequest")]
pub struct GetKeyGroupConfigRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetResponseHeadersPolicyResult")]
pub struct GetResponseHeadersPolicyResult {
    #[serde(rename = "ResponseHeadersPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_headers_policy: Option<ResponseHeadersPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateDistributionResult")]
pub struct UpdateDistributionResult {
    #[serde(rename = "Distribution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution: Option<Distribution>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Distribution")]
pub struct Distribution {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "ActiveTrustedKeyGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_trusted_key_groups: Option<ActiveTrustedKeyGroups>,
    #[serde(rename = "ActiveTrustedSigners")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_trusted_signers: Option<ActiveTrustedSigners>,
    #[serde(rename = "AliasICPRecordals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_i_c_p_recordals: Option<AliasICPRecordals>,
    #[serde(rename = "DistributionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_config: Option<DistributionConfig>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "InProgressInvalidationBatches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_progress_invalidation_batches: Option<i32>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ActiveTrustedKeyGroups")]
pub struct ActiveTrustedKeyGroups {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<KGKeyPairIdsList>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KGKeyPairIdsList {
    #[serde(rename = "KeyGroup", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<KGKeyPairIds>,
}
impl From<Vec<KGKeyPairIds>> for KGKeyPairIdsList {
    fn from(v: Vec<KGKeyPairIds>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<KGKeyPairIds> for KGKeyPairIdsList {
    fn from_iter<I: IntoIterator<Item = KGKeyPairIds>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<KGKeyPairIds>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlKGKeyPairIdsList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<KGKeyPairIds>,
}

impl From<Vec<KGKeyPairIds>> for XmlKGKeyPairIdsList {
    fn from(v: Vec<KGKeyPairIds>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<KGKeyPairIds> for XmlKGKeyPairIdsList {
    fn from_iter<I: IntoIterator<Item = KGKeyPairIds>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "KGKeyPairIds")]
pub struct KGKeyPairIds {
    #[serde(rename = "KeyGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_group_id: Option<String>,
    #[serde(rename = "KeyPairIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair_ids: Option<KeyPairIds>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "KeyPairIds")]
pub struct KeyPairIds {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<KeyPairIdList>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KeyPairIdList {
    #[serde(rename = "KeyPairId", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for KeyPairIdList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for KeyPairIdList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ActiveTrustedSigners")]
pub struct ActiveTrustedSigners {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<SignerList>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SignerList {
    #[serde(rename = "Signer", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Signer>,
}
impl From<Vec<Signer>> for SignerList {
    fn from(v: Vec<Signer>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Signer> for SignerList {
    fn from_iter<I: IntoIterator<Item = Signer>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Signer>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlSignerList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Signer>,
}

impl From<Vec<Signer>> for XmlSignerList {
    fn from(v: Vec<Signer>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Signer> for XmlSignerList {
    fn from_iter<I: IntoIterator<Item = Signer>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Signer")]
pub struct Signer {
    #[serde(rename = "AwsAccountNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_number: Option<String>,
    #[serde(rename = "KeyPairIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair_ids: Option<KeyPairIds>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AliasICPRecordals {
    #[serde(
        rename = "AliasICPRecordal",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<AliasICPRecordal>,
}
impl From<Vec<AliasICPRecordal>> for AliasICPRecordals {
    fn from(v: Vec<AliasICPRecordal>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<AliasICPRecordal> for AliasICPRecordals {
    fn from_iter<I: IntoIterator<Item = AliasICPRecordal>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<AliasICPRecordal>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlAliasICPRecordalList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<AliasICPRecordal>,
}

impl From<Vec<AliasICPRecordal>> for XmlAliasICPRecordalList {
    fn from(v: Vec<AliasICPRecordal>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<AliasICPRecordal> for XmlAliasICPRecordalList {
    fn from_iter<I: IntoIterator<Item = AliasICPRecordal>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AliasICPRecordal")]
pub struct AliasICPRecordal {
    #[serde(rename = "CNAME")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_n_a_m_e: Option<String>,
    #[serde(rename = "ICPRecordalStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_c_p_recordal_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AssociateDistributionWebACLResult")]
pub struct AssociateDistributionWebACLResult {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "WebACLArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_a_c_l_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListPublicKeysRequest")]
pub struct ListPublicKeysRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateTrustStoreRequest")]
pub struct CreateTrustStoreRequest {
    #[serde(rename = "CaCertificatesBundleSource")]
    #[serde(default)]
    pub ca_certificates_bundle_source: CaCertificatesBundleSource,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteMonitoringSubscriptionRequest")]
pub struct DeleteMonitoringSubscriptionRequest {
    #[serde(rename = "DistributionId")]
    #[serde(default)]
    pub distribution_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateStreamingDistributionRequest")]
pub struct CreateStreamingDistributionRequest {
    #[serde(rename = "StreamingDistributionConfig")]
    #[serde(default)]
    pub streaming_distribution_config: StreamingDistributionConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StreamingDistributionConfig")]
pub struct StreamingDistributionConfig {
    #[serde(rename = "Aliases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Aliases>,
    #[serde(rename = "CallerReference")]
    #[serde(default)]
    pub caller_reference: String,
    #[serde(rename = "Comment")]
    #[serde(default)]
    pub comment: String,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "Logging")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<StreamingLoggingConfig>,
    #[serde(rename = "PriceClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_class: Option<String>,
    #[serde(rename = "S3Origin")]
    #[serde(default)]
    pub s3_origin: S3Origin,
    #[serde(rename = "TrustedSigners")]
    #[serde(default)]
    pub trusted_signers: TrustedSigners,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StreamingLoggingConfig")]
pub struct StreamingLoggingConfig {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    pub prefix: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateRealtimeLogConfigRequest")]
pub struct UpdateRealtimeLogConfigRequest {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "EndPoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_points: Option<EndPointList>,
    #[serde(rename = "Fields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<FieldList>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SamplingRate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_rate: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EndPointList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<EndPoint>,
}
impl From<Vec<EndPoint>> for EndPointList {
    fn from(v: Vec<EndPoint>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<EndPoint> for EndPointList {
    fn from_iter<I: IntoIterator<Item = EndPoint>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<EndPoint>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlEndPointList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<EndPoint>,
}

impl From<Vec<EndPoint>> for XmlEndPointList {
    fn from(v: Vec<EndPoint>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<EndPoint> for XmlEndPointList {
    fn from_iter<I: IntoIterator<Item = EndPoint>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EndPoint")]
pub struct EndPoint {
    #[serde(rename = "KinesisStreamConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_stream_config: Option<KinesisStreamConfig>,
    #[serde(rename = "StreamType")]
    #[serde(default)]
    pub stream_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "KinesisStreamConfig")]
pub struct KinesisStreamConfig {
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    pub role_a_r_n: String,
    #[serde(rename = "StreamARN")]
    #[serde(default)]
    pub stream_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FieldList {
    #[serde(rename = "Field", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for FieldList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for FieldList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListDistributionsByConnectionFunctionResult")]
pub struct ListDistributionsByConnectionFunctionResult {
    #[serde(rename = "DistributionList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_list: Option<DistributionList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DistributionList")]
pub struct DistributionList {
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<DistributionSummaryList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DistributionSummaryList {
    #[serde(
        rename = "DistributionSummary",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DistributionSummary>,
}
impl From<Vec<DistributionSummary>> for DistributionSummaryList {
    fn from(v: Vec<DistributionSummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DistributionSummary> for DistributionSummaryList {
    fn from_iter<I: IntoIterator<Item = DistributionSummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DistributionSummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDistributionSummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DistributionSummary>,
}

impl From<Vec<DistributionSummary>> for XmlDistributionSummaryList {
    fn from(v: Vec<DistributionSummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DistributionSummary> for XmlDistributionSummaryList {
    fn from_iter<I: IntoIterator<Item = DistributionSummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DistributionSummary")]
pub struct DistributionSummary {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "AliasICPRecordals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_i_c_p_recordals: Option<AliasICPRecordals>,
    #[serde(rename = "Aliases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Aliases>,
    #[serde(rename = "AnycastIpListId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anycast_ip_list_id: Option<String>,
    #[serde(rename = "CacheBehaviors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_behaviors: Option<CacheBehaviors>,
    #[serde(rename = "Comment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "ConnectionFunctionAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_function_association: Option<ConnectionFunctionAssociation>,
    #[serde(rename = "ConnectionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_mode: Option<String>,
    #[serde(rename = "CustomErrorResponses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_error_responses: Option<CustomErrorResponses>,
    #[serde(rename = "DefaultCacheBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_cache_behavior: Option<DefaultCacheBehavior>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "HttpVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_version: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IsIPV6Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_i_p_v6_enabled: Option<bool>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(rename = "OriginGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_groups: Option<OriginGroups>,
    #[serde(rename = "Origins")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origins: Option<Origins>,
    #[serde(rename = "PriceClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_class: Option<String>,
    #[serde(rename = "Restrictions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Restrictions>,
    #[serde(rename = "Staging")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staging: Option<bool>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "ViewerCertificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_certificate: Option<ViewerCertificate>,
    #[serde(rename = "ViewerMtlsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewer_mtls_config: Option<ViewerMtlsConfig>,
    #[serde(rename = "WebACLId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_a_c_l_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetCloudFrontOriginAccessIdentityConfigResult")]
pub struct GetCloudFrontOriginAccessIdentityConfigResult {
    #[serde(rename = "CloudFrontOriginAccessIdentityConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_front_origin_access_identity_config: Option<CloudFrontOriginAccessIdentityConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CloudFrontOriginAccessIdentityConfig")]
pub struct CloudFrontOriginAccessIdentityConfig {
    #[serde(rename = "CallerReference")]
    #[serde(default)]
    pub caller_reference: String,
    #[serde(rename = "Comment")]
    #[serde(default)]
    pub comment: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteContinuousDeploymentPolicyRequest")]
pub struct DeleteContinuousDeploymentPolicyRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteStreamingDistributionRequest")]
pub struct DeleteStreamingDistributionRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "FunctionList")]
pub struct FunctionList {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<FunctionSummaryList>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FunctionSummaryList {
    #[serde(
        rename = "FunctionSummary",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<FunctionSummary>,
}
impl From<Vec<FunctionSummary>> for FunctionSummaryList {
    fn from(v: Vec<FunctionSummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<FunctionSummary> for FunctionSummaryList {
    fn from_iter<I: IntoIterator<Item = FunctionSummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<FunctionSummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlFunctionSummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<FunctionSummary>,
}

impl From<Vec<FunctionSummary>> for XmlFunctionSummaryList {
    fn from(v: Vec<FunctionSummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<FunctionSummary> for XmlFunctionSummaryList {
    fn from_iter<I: IntoIterator<Item = FunctionSummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetMonitoringSubscriptionResult")]
pub struct GetMonitoringSubscriptionResult {
    #[serde(rename = "MonitoringSubscription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_subscription: Option<MonitoringSubscription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MonitoringSubscription")]
pub struct MonitoringSubscription {
    #[serde(rename = "RealtimeMetricsSubscriptionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realtime_metrics_subscription_config: Option<RealtimeMetricsSubscriptionConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RealtimeMetricsSubscriptionConfig")]
pub struct RealtimeMetricsSubscriptionConfig {
    #[serde(rename = "RealtimeMetricsSubscriptionStatus")]
    #[serde(default)]
    pub realtime_metrics_subscription_status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetInvalidationForDistributionTenantResult")]
pub struct GetInvalidationForDistributionTenantResult {
    #[serde(rename = "Invalidation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalidation: Option<Invalidation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeKeyValueStoreResult")]
pub struct DescribeKeyValueStoreResult {
    #[serde(rename = "KeyValueStore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_value_store: Option<KeyValueStore>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetStreamingDistributionConfigRequest")]
pub struct GetStreamingDistributionConfigRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListDomainConflictsRequest")]
pub struct ListDomainConflictsRequest {
    #[serde(rename = "Domain")]
    #[serde(default)]
    pub domain: String,
    #[serde(rename = "DomainControlValidationResource")]
    #[serde(default)]
    pub domain_control_validation_resource: DistributionResourceId,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DistributionResourceId")]
pub struct DistributionResourceId {
    #[serde(rename = "DistributionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_id: Option<String>,
    #[serde(rename = "DistributionTenantId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_tenant_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AssociateDistributionTenantWebACLResult")]
pub struct AssociateDistributionTenantWebACLResult {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "WebACLArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_a_c_l_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PublishConnectionFunctionRequest")]
pub struct PublishConnectionFunctionRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    pub if_match: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateFieldLevelEncryptionConfigResult")]
pub struct UpdateFieldLevelEncryptionConfigResult {
    #[serde(rename = "FieldLevelEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_level_encryption: Option<FieldLevelEncryption>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "FieldLevelEncryption")]
pub struct FieldLevelEncryption {
    #[serde(rename = "FieldLevelEncryptionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_level_encryption_config: Option<FieldLevelEncryptionConfig>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "FieldLevelEncryptionConfig")]
pub struct FieldLevelEncryptionConfig {
    #[serde(rename = "CallerReference")]
    #[serde(default)]
    pub caller_reference: String,
    #[serde(rename = "Comment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "ContentTypeProfileConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type_profile_config: Option<ContentTypeProfileConfig>,
    #[serde(rename = "QueryArgProfileConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_arg_profile_config: Option<QueryArgProfileConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ContentTypeProfileConfig")]
pub struct ContentTypeProfileConfig {
    #[serde(rename = "ContentTypeProfiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type_profiles: Option<ContentTypeProfiles>,
    #[serde(rename = "ForwardWhenContentTypeIsUnknown")]
    #[serde(default)]
    pub forward_when_content_type_is_unknown: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ContentTypeProfiles")]
pub struct ContentTypeProfiles {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<ContentTypeProfileList>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    pub quantity: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContentTypeProfileList {
    #[serde(
        rename = "ContentTypeProfile",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ContentTypeProfile>,
}
impl From<Vec<ContentTypeProfile>> for ContentTypeProfileList {
    fn from(v: Vec<ContentTypeProfile>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ContentTypeProfile> for ContentTypeProfileList {
    fn from_iter<I: IntoIterator<Item = ContentTypeProfile>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ContentTypeProfile>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlContentTypeProfileList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ContentTypeProfile>,
}

impl From<Vec<ContentTypeProfile>> for XmlContentTypeProfileList {
    fn from(v: Vec<ContentTypeProfile>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ContentTypeProfile> for XmlContentTypeProfileList {
    fn from_iter<I: IntoIterator<Item = ContentTypeProfile>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ContentTypeProfile")]
pub struct ContentTypeProfile {
    #[serde(rename = "ContentType")]
    #[serde(default)]
    pub content_type: String,
    #[serde(rename = "Format")]
    #[serde(default)]
    pub format: String,
    #[serde(rename = "ProfileId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "QueryArgProfileConfig")]
pub struct QueryArgProfileConfig {
    #[serde(rename = "ForwardWhenQueryArgProfileIsUnknown")]
    #[serde(default)]
    pub forward_when_query_arg_profile_is_unknown: bool,
    #[serde(rename = "QueryArgProfiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_arg_profiles: Option<QueryArgProfiles>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "QueryArgProfiles")]
pub struct QueryArgProfiles {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<QueryArgProfileList>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    pub quantity: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryArgProfileList {
    #[serde(
        rename = "QueryArgProfile",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<QueryArgProfile>,
}
impl From<Vec<QueryArgProfile>> for QueryArgProfileList {
    fn from(v: Vec<QueryArgProfile>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<QueryArgProfile> for QueryArgProfileList {
    fn from_iter<I: IntoIterator<Item = QueryArgProfile>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<QueryArgProfile>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlQueryArgProfileList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<QueryArgProfile>,
}

impl From<Vec<QueryArgProfile>> for XmlQueryArgProfileList {
    fn from(v: Vec<QueryArgProfile>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<QueryArgProfile> for XmlQueryArgProfileList {
    fn from_iter<I: IntoIterator<Item = QueryArgProfile>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "QueryArgProfile")]
pub struct QueryArgProfile {
    #[serde(rename = "ProfileId")]
    #[serde(default)]
    pub profile_id: String,
    #[serde(rename = "QueryArg")]
    #[serde(default)]
    pub query_arg: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateResponseHeadersPolicyRequest")]
pub struct CreateResponseHeadersPolicyRequest {
    #[serde(rename = "ResponseHeadersPolicyConfig")]
    #[serde(default)]
    pub response_headers_policy_config: ResponseHeadersPolicyConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetKeyGroupResult")]
pub struct GetKeyGroupResult {
    #[serde(rename = "KeyGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_group: Option<KeyGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListRealtimeLogConfigsResult")]
pub struct ListRealtimeLogConfigsResult {
    #[serde(rename = "RealtimeLogConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realtime_log_configs: Option<RealtimeLogConfigs>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RealtimeLogConfigs")]
pub struct RealtimeLogConfigs {
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<RealtimeLogConfigList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RealtimeLogConfigList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<RealtimeLogConfig>,
}
impl From<Vec<RealtimeLogConfig>> for RealtimeLogConfigList {
    fn from(v: Vec<RealtimeLogConfig>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<RealtimeLogConfig> for RealtimeLogConfigList {
    fn from_iter<I: IntoIterator<Item = RealtimeLogConfig>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<RealtimeLogConfig>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlRealtimeLogConfigList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<RealtimeLogConfig>,
}

impl From<Vec<RealtimeLogConfig>> for XmlRealtimeLogConfigList {
    fn from(v: Vec<RealtimeLogConfig>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<RealtimeLogConfig> for XmlRealtimeLogConfigList {
    fn from_iter<I: IntoIterator<Item = RealtimeLogConfig>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RealtimeLogConfig")]
pub struct RealtimeLogConfig {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "EndPoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_points: Option<EndPointList>,
    #[serde(rename = "Fields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<FieldList>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SamplingRate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_rate: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeletePublicKeyRequest")]
pub struct DeletePublicKeyRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetOriginRequestPolicyRequest")]
pub struct GetOriginRequestPolicyRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListConnectionGroupsRequest")]
pub struct ListConnectionGroupsRequest {
    #[serde(rename = "AssociationFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_filter: Option<ConnectionGroupAssociationFilter>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ConnectionGroupAssociationFilter")]
pub struct ConnectionGroupAssociationFilter {
    #[serde(rename = "AnycastIpListId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anycast_ip_list_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CachePolicy")]
pub struct CachePolicy {
    #[serde(rename = "CachePolicyConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_policy_config: Option<CachePolicyConfig>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListDomainConflictsResult")]
pub struct ListDomainConflictsResult {
    #[serde(rename = "DomainConflicts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_conflicts: Option<DomainConflictsList>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainConflictsList {
    #[serde(
        rename = "DomainConflicts",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DomainConflict>,
}
impl From<Vec<DomainConflict>> for DomainConflictsList {
    fn from(v: Vec<DomainConflict>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DomainConflict> for DomainConflictsList {
    fn from_iter<I: IntoIterator<Item = DomainConflict>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DomainConflict>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDomainConflictList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DomainConflict>,
}

impl From<Vec<DomainConflict>> for XmlDomainConflictList {
    fn from(v: Vec<DomainConflict>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DomainConflict> for XmlDomainConflictList {
    fn from_iter<I: IntoIterator<Item = DomainConflict>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DomainConflict")]
pub struct DomainConflict {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListOriginAccessControlsRequest")]
pub struct ListOriginAccessControlsRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetOriginRequestPolicyConfigResult")]
pub struct GetOriginRequestPolicyConfigResult {
    #[serde(rename = "OriginRequestPolicyConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_request_policy_config: Option<OriginRequestPolicyConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListDistributionsByAnycastIpListIdResult")]
pub struct ListDistributionsByAnycastIpListIdResult {
    #[serde(rename = "DistributionList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_list: Option<DistributionList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteResponseHeadersPolicyRequest")]
pub struct DeleteResponseHeadersPolicyRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetCloudFrontOriginAccessIdentityResult")]
pub struct GetCloudFrontOriginAccessIdentityResult {
    #[serde(rename = "CloudFrontOriginAccessIdentity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_front_origin_access_identity: Option<CloudFrontOriginAccessIdentity>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CloudFrontOriginAccessIdentity")]
pub struct CloudFrontOriginAccessIdentity {
    #[serde(rename = "CloudFrontOriginAccessIdentityConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_front_origin_access_identity_config: Option<CloudFrontOriginAccessIdentityConfig>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "S3CanonicalUserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_canonical_user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListAnycastIpListsResult")]
pub struct ListAnycastIpListsResult {
    #[serde(rename = "AnycastIpListCollection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anycast_ip_lists: Option<AnycastIpListCollection>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AnycastIpListCollection")]
pub struct AnycastIpListCollection {
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<AnycastIpListSummaries>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnycastIpListSummaries {
    #[serde(
        rename = "AnycastIpListSummary",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<AnycastIpListSummary>,
}
impl From<Vec<AnycastIpListSummary>> for AnycastIpListSummaries {
    fn from(v: Vec<AnycastIpListSummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<AnycastIpListSummary> for AnycastIpListSummaries {
    fn from_iter<I: IntoIterator<Item = AnycastIpListSummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<AnycastIpListSummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlAnycastIpListSummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<AnycastIpListSummary>,
}

impl From<Vec<AnycastIpListSummary>> for XmlAnycastIpListSummaryList {
    fn from(v: Vec<AnycastIpListSummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<AnycastIpListSummary> for XmlAnycastIpListSummaryList {
    fn from_iter<I: IntoIterator<Item = AnycastIpListSummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AnycastIpListSummary")]
pub struct AnycastIpListSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IpAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(rename = "IpCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_count: Option<i32>,
    #[serde(rename = "IpamConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipam_config: Option<IpamConfig>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "IpamConfig")]
pub struct IpamConfig {
    #[serde(rename = "IpamCidrConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipam_cidr_configs: Option<IpamCidrConfigList>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateOriginAccessControlResult")]
pub struct UpdateOriginAccessControlResult {
    #[serde(rename = "OriginAccessControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_access_control: Option<OriginAccessControl>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateConnectionFunctionResult")]
pub struct UpdateConnectionFunctionResult {
    #[serde(rename = "ConnectionFunctionSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_function_summary: Option<ConnectionFunctionSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateRealtimeLogConfigResult")]
pub struct UpdateRealtimeLogConfigResult {
    #[serde(rename = "RealtimeLogConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realtime_log_config: Option<RealtimeLogConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetTrustStoreRequest")]
pub struct GetTrustStoreRequest {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TagResourceRequest")]
pub struct TagResourceRequest {
    #[serde(rename = "Resource")]
    #[serde(default)]
    pub resource: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Tags,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetVpcOriginRequest")]
pub struct GetVpcOriginRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListKeyValueStoresRequest")]
pub struct ListKeyValueStoresRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetOriginRequestPolicyConfigRequest")]
pub struct GetOriginRequestPolicyConfigRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PublishFunctionRequest")]
pub struct PublishFunctionRequest {
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    pub if_match: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateStreamingDistributionRequest")]
pub struct UpdateStreamingDistributionRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match: Option<String>,
    #[serde(rename = "StreamingDistributionConfig")]
    #[serde(default)]
    pub streaming_distribution_config: StreamingDistributionConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateCachePolicyResult")]
pub struct CreateCachePolicyResult {
    #[serde(rename = "CachePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_policy: Option<CachePolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateFieldLevelEncryptionConfigRequest")]
pub struct CreateFieldLevelEncryptionConfigRequest {
    #[serde(rename = "FieldLevelEncryptionConfig")]
    #[serde(default)]
    pub field_level_encryption_config: FieldLevelEncryptionConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateOriginAccessControlRequest")]
pub struct CreateOriginAccessControlRequest {
    #[serde(rename = "OriginAccessControlConfig")]
    #[serde(default)]
    pub origin_access_control_config: OriginAccessControlConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateResponseHeadersPolicyResult")]
pub struct CreateResponseHeadersPolicyResult {
    #[serde(rename = "ResponseHeadersPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_headers_policy: Option<ResponseHeadersPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetKeyGroupRequest")]
pub struct GetKeyGroupRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateFieldLevelEncryptionProfileResult")]
pub struct CreateFieldLevelEncryptionProfileResult {
    #[serde(rename = "FieldLevelEncryptionProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_level_encryption_profile: Option<FieldLevelEncryptionProfile>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "FieldLevelEncryptionProfile")]
pub struct FieldLevelEncryptionProfile {
    #[serde(rename = "FieldLevelEncryptionProfileConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_level_encryption_profile_config: Option<FieldLevelEncryptionProfileConfig>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "FieldLevelEncryptionProfileConfig")]
pub struct FieldLevelEncryptionProfileConfig {
    #[serde(rename = "CallerReference")]
    #[serde(default)]
    pub caller_reference: String,
    #[serde(rename = "Comment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "EncryptionEntities")]
    #[serde(default)]
    pub encryption_entities: EncryptionEntities,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListDistributionsByCachePolicyIdRequest")]
pub struct ListDistributionsByCachePolicyIdRequest {
    #[serde(rename = "CachePolicyId")]
    #[serde(default)]
    pub cache_policy_id: String,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DisassociateDistributionWebACLRequest")]
pub struct DisassociateDistributionWebACLRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListDistributionsByKeyGroupResult")]
pub struct ListDistributionsByKeyGroupResult {
    #[serde(rename = "DistributionIdList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_id_list: Option<DistributionIdList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateAnycastIpListResult")]
pub struct CreateAnycastIpListResult {
    #[serde(rename = "AnycastIpList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anycast_ip_list: Option<AnycastIpList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AnycastIpList")]
pub struct AnycastIpList {
    #[serde(rename = "AnycastIps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anycast_ips: Option<AnycastIps>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IpAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(rename = "IpCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_count: Option<i32>,
    #[serde(rename = "IpamConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipam_config: Option<IpamConfig>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnycastIps {
    #[serde(rename = "AnycastIp", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for AnycastIps {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for AnycastIps {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListKeyGroupsRequest")]
pub struct ListKeyGroupsRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListFieldLevelEncryptionConfigsRequest")]
pub struct ListFieldLevelEncryptionConfigsRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTrustStoresResult")]
pub struct ListTrustStoresResult {
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "TrustStoreList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_store_list: Option<TrustStoreList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrustStoreList {
    #[serde(
        rename = "TrustStoreSummary",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<TrustStoreSummary>,
}
impl From<Vec<TrustStoreSummary>> for TrustStoreList {
    fn from(v: Vec<TrustStoreSummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<TrustStoreSummary> for TrustStoreList {
    fn from_iter<I: IntoIterator<Item = TrustStoreSummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<TrustStoreSummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTrustStoreSummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<TrustStoreSummary>,
}

impl From<Vec<TrustStoreSummary>> for XmlTrustStoreSummaryList {
    fn from(v: Vec<TrustStoreSummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<TrustStoreSummary> for XmlTrustStoreSummaryList {
    fn from_iter<I: IntoIterator<Item = TrustStoreSummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TrustStoreSummary")]
pub struct TrustStoreSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NumberOfCaCertificates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_ca_certificates: Option<i32>,
    #[serde(rename = "Reason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateConnectionGroupResult")]
pub struct UpdateConnectionGroupResult {
    #[serde(rename = "ConnectionGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_group: Option<ConnectionGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateKeyGroupRequest")]
pub struct CreateKeyGroupRequest {
    #[serde(rename = "KeyGroupConfig")]
    #[serde(default)]
    pub key_group_config: KeyGroupConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateKeyValueStoreRequest")]
pub struct CreateKeyValueStoreRequest {
    #[serde(rename = "Comment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "ImportSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_source: Option<ImportSource>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ImportSource")]
pub struct ImportSource {
    #[serde(rename = "SourceARN")]
    #[serde(default)]
    pub source_a_r_n: String,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    pub source_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreatePublicKeyResult")]
pub struct CreatePublicKeyResult {
    #[serde(rename = "PublicKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<PublicKey>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PublicKey")]
pub struct PublicKey {
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "PublicKeyConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key_config: Option<PublicKeyConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateContinuousDeploymentPolicyResult")]
pub struct UpdateContinuousDeploymentPolicyResult {
    #[serde(rename = "ContinuousDeploymentPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuous_deployment_policy: Option<ContinuousDeploymentPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ContinuousDeploymentPolicy")]
pub struct ContinuousDeploymentPolicy {
    #[serde(rename = "ContinuousDeploymentPolicyConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuous_deployment_policy_config: Option<ContinuousDeploymentPolicyConfig>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateDistributionWithStagingConfigRequest")]
pub struct UpdateDistributionWithStagingConfigRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match: Option<String>,
    #[serde(rename = "StagingDistributionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staging_distribution_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetDistributionTenantByDomainResult")]
pub struct GetDistributionTenantByDomainResult {
    #[serde(rename = "DistributionTenant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_tenant: Option<DistributionTenant>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateDomainAssociationRequest")]
pub struct UpdateDomainAssociationRequest {
    #[serde(rename = "Domain")]
    #[serde(default)]
    pub domain: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match: Option<String>,
    #[serde(rename = "TargetResource")]
    #[serde(default)]
    pub target_resource: DistributionResourceId,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateDomainAssociationResult")]
pub struct UpdateDomainAssociationResult {
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateOriginRequestPolicyRequest")]
pub struct UpdateOriginRequestPolicyRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match: Option<String>,
    #[serde(rename = "OriginRequestPolicyConfig")]
    #[serde(default)]
    pub origin_request_policy_config: OriginRequestPolicyConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetDistributionTenantResult")]
pub struct GetDistributionTenantResult {
    #[serde(rename = "DistributionTenant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_tenant: Option<DistributionTenant>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteFieldLevelEncryptionProfileRequest")]
pub struct DeleteFieldLevelEncryptionProfileRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateConnectionFunctionRequest")]
pub struct UpdateConnectionFunctionRequest {
    #[serde(rename = "ConnectionFunctionCode")]
    #[serde(default)]
    pub connection_function_code: String,
    #[serde(rename = "ConnectionFunctionConfig")]
    #[serde(default)]
    pub connection_function_config: FunctionConfig,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    pub if_match: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeFunctionRequest")]
pub struct DescribeFunctionRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Stage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateDistributionWithStagingConfigResult")]
pub struct UpdateDistributionWithStagingConfigResult {
    #[serde(rename = "Distribution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution: Option<Distribution>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListConflictingAliasesResult")]
pub struct ListConflictingAliasesResult {
    #[serde(rename = "ConflictingAliasesList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflicting_aliases_list: Option<ConflictingAliasesList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ConflictingAliasesList")]
pub struct ConflictingAliasesList {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<ConflictingAliases>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConflictingAliases {
    #[serde(
        rename = "ConflictingAlias",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ConflictingAlias>,
}
impl From<Vec<ConflictingAlias>> for ConflictingAliases {
    fn from(v: Vec<ConflictingAlias>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ConflictingAlias> for ConflictingAliases {
    fn from_iter<I: IntoIterator<Item = ConflictingAlias>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ConflictingAlias>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlConflictingAliasList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ConflictingAlias>,
}

impl From<Vec<ConflictingAlias>> for XmlConflictingAliasList {
    fn from(v: Vec<ConflictingAlias>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ConflictingAlias> for XmlConflictingAliasList {
    fn from_iter<I: IntoIterator<Item = ConflictingAlias>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ConflictingAlias")]
pub struct ConflictingAlias {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "Alias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "DistributionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListDistributionsByOwnedResourceResult")]
pub struct ListDistributionsByOwnedResourceResult {
    #[serde(rename = "DistributionList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_list: Option<DistributionIdOwnerList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetTrustStoreResult")]
pub struct GetTrustStoreResult {
    #[serde(rename = "TrustStore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_store: Option<TrustStore>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TrustStore")]
pub struct TrustStore {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NumberOfCaCertificates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_ca_certificates: Option<i32>,
    #[serde(rename = "Reason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListOriginAccessControlsResult")]
pub struct ListOriginAccessControlsResult {
    #[serde(rename = "OriginAccessControlList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_access_control_list: Option<OriginAccessControlList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PublishFunctionResult")]
pub struct PublishFunctionResult {
    #[serde(rename = "FunctionSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_summary: Option<FunctionSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateKeyGroupRequest")]
pub struct UpdateKeyGroupRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match: Option<String>,
    #[serde(rename = "KeyGroupConfig")]
    #[serde(default)]
    pub key_group_config: KeyGroupConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateVpcOriginRequest")]
pub struct UpdateVpcOriginRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    pub if_match: String,
    #[serde(rename = "VpcOriginEndpointConfig")]
    #[serde(default)]
    pub vpc_origin_endpoint_config: VpcOriginEndpointConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UntagResourceRequest")]
pub struct UntagResourceRequest {
    #[serde(rename = "Resource")]
    #[serde(default)]
    pub resource: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: TagKeys,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateVpcOriginResult")]
pub struct UpdateVpcOriginResult {
    #[serde(rename = "VpcOrigin")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_origin: Option<VpcOrigin>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListInvalidationsResult")]
pub struct ListInvalidationsResult {
    #[serde(rename = "InvalidationList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalidation_list: Option<InvalidationList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "InvalidationList")]
pub struct InvalidationList {
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<InvalidationSummaryList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InvalidationSummaryList {
    #[serde(
        rename = "InvalidationSummary",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<InvalidationSummary>,
}
impl From<Vec<InvalidationSummary>> for InvalidationSummaryList {
    fn from(v: Vec<InvalidationSummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<InvalidationSummary> for InvalidationSummaryList {
    fn from_iter<I: IntoIterator<Item = InvalidationSummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<InvalidationSummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlInvalidationSummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<InvalidationSummary>,
}

impl From<Vec<InvalidationSummary>> for XmlInvalidationSummaryList {
    fn from(v: Vec<InvalidationSummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<InvalidationSummary> for XmlInvalidationSummaryList {
    fn from_iter<I: IntoIterator<Item = InvalidationSummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "InvalidationSummary")]
pub struct InvalidationSummary {
    #[serde(rename = "CreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateCachePolicyRequest")]
pub struct CreateCachePolicyRequest {
    #[serde(rename = "CachePolicyConfig")]
    #[serde(default)]
    pub cache_policy_config: CachePolicyConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListDistributionTenantsByCustomizationRequest")]
pub struct ListDistributionTenantsByCustomizationRequest {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "WebACLArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_a_c_l_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListFunctionsRequest")]
pub struct ListFunctionsRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "Stage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDistributionWithTagsResult")]
pub struct CreateDistributionWithTagsResult {
    #[serde(rename = "Distribution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution: Option<Distribution>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteDistributionRequest")]
pub struct DeleteDistributionRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteAnycastIpListRequest")]
pub struct DeleteAnycastIpListRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    pub if_match: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetOriginAccessControlRequest")]
pub struct GetOriginAccessControlRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetCachePolicyResult")]
pub struct GetCachePolicyResult {
    #[serde(rename = "CachePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_policy: Option<CachePolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetContinuousDeploymentPolicyConfigRequest")]
pub struct GetContinuousDeploymentPolicyConfigRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListInvalidationsRequest")]
pub struct ListInvalidationsRequest {
    #[serde(rename = "DistributionId")]
    #[serde(default)]
    pub distribution_id: String,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTagsForResourceResult")]
pub struct ListTagsForResourceResult {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateKeyGroupResult")]
pub struct UpdateKeyGroupResult {
    #[serde(rename = "KeyGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_group: Option<KeyGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateMonitoringSubscriptionRequest")]
pub struct CreateMonitoringSubscriptionRequest {
    #[serde(rename = "DistributionId")]
    #[serde(default)]
    pub distribution_id: String,
    #[serde(rename = "MonitoringSubscription")]
    #[serde(default)]
    pub monitoring_subscription: MonitoringSubscription,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetFunctionRequest")]
pub struct GetFunctionRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Stage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OriginRequestPolicyList")]
pub struct OriginRequestPolicyList {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<OriginRequestPolicySummaryList>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OriginRequestPolicySummaryList {
    #[serde(
        rename = "OriginRequestPolicySummary",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<OriginRequestPolicySummary>,
}
impl From<Vec<OriginRequestPolicySummary>> for OriginRequestPolicySummaryList {
    fn from(v: Vec<OriginRequestPolicySummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<OriginRequestPolicySummary> for OriginRequestPolicySummaryList {
    fn from_iter<I: IntoIterator<Item = OriginRequestPolicySummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<OriginRequestPolicySummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlOriginRequestPolicySummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<OriginRequestPolicySummary>,
}

impl From<Vec<OriginRequestPolicySummary>> for XmlOriginRequestPolicySummaryList {
    fn from(v: Vec<OriginRequestPolicySummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<OriginRequestPolicySummary> for XmlOriginRequestPolicySummaryList {
    fn from_iter<I: IntoIterator<Item = OriginRequestPolicySummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OriginRequestPolicySummary")]
pub struct OriginRequestPolicySummary {
    #[serde(rename = "OriginRequestPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_request_policy: Option<OriginRequestPolicy>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateAnycastIpListResult")]
pub struct UpdateAnycastIpListResult {
    #[serde(rename = "AnycastIpList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anycast_ip_list: Option<AnycastIpList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateConnectionGroupRequest")]
pub struct UpdateConnectionGroupRequest {
    #[serde(rename = "AnycastIpListId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anycast_ip_list_id: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    pub if_match: String,
    #[serde(rename = "Ipv6Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListFunctionsResult")]
pub struct ListFunctionsResult {
    #[serde(rename = "FunctionList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_list: Option<FunctionList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateContinuousDeploymentPolicyResult")]
pub struct CreateContinuousDeploymentPolicyResult {
    #[serde(rename = "ContinuousDeploymentPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuous_deployment_policy: Option<ContinuousDeploymentPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListRealtimeLogConfigsRequest")]
pub struct ListRealtimeLogConfigsRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListContinuousDeploymentPoliciesResult")]
pub struct ListContinuousDeploymentPoliciesResult {
    #[serde(rename = "ContinuousDeploymentPolicyList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuous_deployment_policy_list: Option<ContinuousDeploymentPolicyList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ContinuousDeploymentPolicyList")]
pub struct ContinuousDeploymentPolicyList {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<ContinuousDeploymentPolicySummaryList>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContinuousDeploymentPolicySummaryList {
    #[serde(
        rename = "ContinuousDeploymentPolicySummary",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ContinuousDeploymentPolicySummary>,
}
impl From<Vec<ContinuousDeploymentPolicySummary>> for ContinuousDeploymentPolicySummaryList {
    fn from(v: Vec<ContinuousDeploymentPolicySummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ContinuousDeploymentPolicySummary> for ContinuousDeploymentPolicySummaryList {
    fn from_iter<I: IntoIterator<Item = ContinuousDeploymentPolicySummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ContinuousDeploymentPolicySummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlContinuousDeploymentPolicySummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ContinuousDeploymentPolicySummary>,
}

impl From<Vec<ContinuousDeploymentPolicySummary>> for XmlContinuousDeploymentPolicySummaryList {
    fn from(v: Vec<ContinuousDeploymentPolicySummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ContinuousDeploymentPolicySummary> for XmlContinuousDeploymentPolicySummaryList {
    fn from_iter<I: IntoIterator<Item = ContinuousDeploymentPolicySummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ContinuousDeploymentPolicySummary")]
pub struct ContinuousDeploymentPolicySummary {
    #[serde(rename = "ContinuousDeploymentPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuous_deployment_policy: Option<ContinuousDeploymentPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteFunctionRequest")]
pub struct DeleteFunctionRequest {
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    pub if_match: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateStreamingDistributionWithTagsRequest")]
pub struct CreateStreamingDistributionWithTagsRequest {
    #[serde(rename = "StreamingDistributionConfigWithTags")]
    #[serde(default)]
    pub streaming_distribution_config_with_tags: StreamingDistributionConfigWithTags,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StreamingDistributionConfigWithTags")]
pub struct StreamingDistributionConfigWithTags {
    #[serde(rename = "StreamingDistributionConfig")]
    #[serde(default)]
    pub streaming_distribution_config: StreamingDistributionConfig,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Tags,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetInvalidationRequest")]
pub struct GetInvalidationRequest {
    #[serde(rename = "DistributionId")]
    #[serde(default)]
    pub distribution_id: String,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "KeyValueStoreList")]
pub struct KeyValueStoreList {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<KeyValueStoreSummaryList>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KeyValueStoreSummaryList {
    #[serde(
        rename = "KeyValueStore",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<KeyValueStore>,
}
impl From<Vec<KeyValueStore>> for KeyValueStoreSummaryList {
    fn from(v: Vec<KeyValueStore>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<KeyValueStore> for KeyValueStoreSummaryList {
    fn from_iter<I: IntoIterator<Item = KeyValueStore>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<KeyValueStore>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlKeyValueStoreList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<KeyValueStore>,
}

impl From<Vec<KeyValueStore>> for XmlKeyValueStoreList {
    fn from(v: Vec<KeyValueStore>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<KeyValueStore> for XmlKeyValueStoreList {
    fn from_iter<I: IntoIterator<Item = KeyValueStore>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutResourcePolicyRequest")]
pub struct PutResourcePolicyRequest {
    #[serde(rename = "PolicyDocument")]
    #[serde(default)]
    pub policy_document: String,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetAnycastIpListRequest")]
pub struct GetAnycastIpListRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListOriginRequestPoliciesResult")]
pub struct ListOriginRequestPoliciesResult {
    #[serde(rename = "OriginRequestPolicyList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_request_policy_list: Option<OriginRequestPolicyList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetFieldLevelEncryptionResult")]
pub struct GetFieldLevelEncryptionResult {
    #[serde(rename = "FieldLevelEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_level_encryption: Option<FieldLevelEncryption>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListConnectionGroupsResult")]
pub struct ListConnectionGroupsResult {
    #[serde(rename = "ConnectionGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_groups: Option<ConnectionGroupSummaryList>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectionGroupSummaryList {
    #[serde(
        rename = "ConnectionGroupSummary",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ConnectionGroupSummary>,
}
impl From<Vec<ConnectionGroupSummary>> for ConnectionGroupSummaryList {
    fn from(v: Vec<ConnectionGroupSummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ConnectionGroupSummary> for ConnectionGroupSummaryList {
    fn from_iter<I: IntoIterator<Item = ConnectionGroupSummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ConnectionGroupSummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlConnectionGroupSummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ConnectionGroupSummary>,
}

impl From<Vec<ConnectionGroupSummary>> for XmlConnectionGroupSummaryList {
    fn from(v: Vec<ConnectionGroupSummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ConnectionGroupSummary> for XmlConnectionGroupSummaryList {
    fn from_iter<I: IntoIterator<Item = ConnectionGroupSummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ConnectionGroupSummary")]
pub struct ConnectionGroupSummary {
    #[serde(rename = "AnycastIpListId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anycast_ip_list_id: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IsDefault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RoutingEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_endpoint: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateRealtimeLogConfigResult")]
pub struct CreateRealtimeLogConfigResult {
    #[serde(rename = "RealtimeLogConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realtime_log_config: Option<RealtimeLogConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetManagedCertificateDetailsResult")]
pub struct GetManagedCertificateDetailsResult {
    #[serde(rename = "ManagedCertificateDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_certificate_details: Option<ManagedCertificateDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ManagedCertificateDetails")]
pub struct ManagedCertificateDetails {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "CertificateStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_status: Option<String>,
    #[serde(rename = "ValidationTokenDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_token_details: Option<ValidationTokenDetailList>,
    #[serde(rename = "ValidationTokenHost")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_token_host: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidationTokenDetailList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ValidationTokenDetail>,
}
impl From<Vec<ValidationTokenDetail>> for ValidationTokenDetailList {
    fn from(v: Vec<ValidationTokenDetail>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ValidationTokenDetail> for ValidationTokenDetailList {
    fn from_iter<I: IntoIterator<Item = ValidationTokenDetail>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ValidationTokenDetail>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlValidationTokenDetailList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ValidationTokenDetail>,
}

impl From<Vec<ValidationTokenDetail>> for XmlValidationTokenDetailList {
    fn from(v: Vec<ValidationTokenDetail>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ValidationTokenDetail> for XmlValidationTokenDetailList {
    fn from_iter<I: IntoIterator<Item = ValidationTokenDetail>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ValidationTokenDetail")]
pub struct ValidationTokenDetail {
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "RedirectFrom")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_from: Option<String>,
    #[serde(rename = "RedirectTo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_to: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DisassociateDistributionTenantWebACLRequest")]
pub struct DisassociateDistributionTenantWebACLRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CloudFrontOriginAccessIdentityList")]
pub struct CloudFrontOriginAccessIdentityList {
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<CloudFrontOriginAccessIdentitySummaryList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudFrontOriginAccessIdentitySummaryList {
    #[serde(
        rename = "CloudFrontOriginAccessIdentitySummary",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<CloudFrontOriginAccessIdentitySummary>,
}
impl From<Vec<CloudFrontOriginAccessIdentitySummary>>
    for CloudFrontOriginAccessIdentitySummaryList
{
    fn from(v: Vec<CloudFrontOriginAccessIdentitySummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<CloudFrontOriginAccessIdentitySummary>
    for CloudFrontOriginAccessIdentitySummaryList
{
    fn from_iter<I: IntoIterator<Item = CloudFrontOriginAccessIdentitySummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<CloudFrontOriginAccessIdentitySummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlCloudFrontOriginAccessIdentitySummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<CloudFrontOriginAccessIdentitySummary>,
}

impl From<Vec<CloudFrontOriginAccessIdentitySummary>>
    for XmlCloudFrontOriginAccessIdentitySummaryList
{
    fn from(v: Vec<CloudFrontOriginAccessIdentitySummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<CloudFrontOriginAccessIdentitySummary>
    for XmlCloudFrontOriginAccessIdentitySummaryList
{
    fn from_iter<I: IntoIterator<Item = CloudFrontOriginAccessIdentitySummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CloudFrontOriginAccessIdentitySummary")]
pub struct CloudFrontOriginAccessIdentitySummary {
    #[serde(rename = "Comment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "S3CanonicalUserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_canonical_user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListConnectionFunctionsResult")]
pub struct ListConnectionFunctionsResult {
    #[serde(rename = "ConnectionFunctions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_functions: Option<ConnectionFunctionSummaryList>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectionFunctionSummaryList {
    #[serde(
        rename = "ConnectionFunctionSummary",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ConnectionFunctionSummary>,
}
impl From<Vec<ConnectionFunctionSummary>> for ConnectionFunctionSummaryList {
    fn from(v: Vec<ConnectionFunctionSummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ConnectionFunctionSummary> for ConnectionFunctionSummaryList {
    fn from_iter<I: IntoIterator<Item = ConnectionFunctionSummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ConnectionFunctionSummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlConnectionFunctionSummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ConnectionFunctionSummary>,
}

impl From<Vec<ConnectionFunctionSummary>> for XmlConnectionFunctionSummaryList {
    fn from(v: Vec<ConnectionFunctionSummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ConnectionFunctionSummary> for XmlConnectionFunctionSummaryList {
    fn from_iter<I: IntoIterator<Item = ConnectionFunctionSummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateConnectionFunctionRequest")]
pub struct CreateConnectionFunctionRequest {
    #[serde(rename = "ConnectionFunctionCode")]
    #[serde(default)]
    pub connection_function_code: String,
    #[serde(rename = "ConnectionFunctionConfig")]
    #[serde(default)]
    pub connection_function_config: FunctionConfig,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "FieldLevelEncryptionList")]
pub struct FieldLevelEncryptionList {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<FieldLevelEncryptionSummaryList>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FieldLevelEncryptionSummaryList {
    #[serde(
        rename = "FieldLevelEncryptionSummary",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<FieldLevelEncryptionSummary>,
}
impl From<Vec<FieldLevelEncryptionSummary>> for FieldLevelEncryptionSummaryList {
    fn from(v: Vec<FieldLevelEncryptionSummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<FieldLevelEncryptionSummary> for FieldLevelEncryptionSummaryList {
    fn from_iter<I: IntoIterator<Item = FieldLevelEncryptionSummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<FieldLevelEncryptionSummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlFieldLevelEncryptionSummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<FieldLevelEncryptionSummary>,
}

impl From<Vec<FieldLevelEncryptionSummary>> for XmlFieldLevelEncryptionSummaryList {
    fn from(v: Vec<FieldLevelEncryptionSummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<FieldLevelEncryptionSummary> for XmlFieldLevelEncryptionSummaryList {
    fn from_iter<I: IntoIterator<Item = FieldLevelEncryptionSummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "FieldLevelEncryptionSummary")]
pub struct FieldLevelEncryptionSummary {
    #[serde(rename = "Comment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "ContentTypeProfileConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type_profile_config: Option<ContentTypeProfileConfig>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(rename = "QueryArgProfileConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_arg_profile_config: Option<QueryArgProfileConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateKeyValueStoreRequest")]
pub struct UpdateKeyValueStoreRequest {
    #[serde(rename = "Comment")]
    #[serde(default)]
    pub comment: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    pub if_match: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ConnectionFunctionTestResult")]
pub struct ConnectionFunctionTestResult {
    #[serde(rename = "ComputeUtilization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_utilization: Option<String>,
    #[serde(rename = "ConnectionFunctionErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_function_error_message: Option<String>,
    #[serde(rename = "ConnectionFunctionExecutionLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_function_execution_logs: Option<FunctionExecutionLogList>,
    #[serde(rename = "ConnectionFunctionOutput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_function_output: Option<String>,
    #[serde(rename = "ConnectionFunctionSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_function_summary: Option<ConnectionFunctionSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListVpcOriginsRequest")]
pub struct ListVpcOriginsRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateTrustStoreResult")]
pub struct CreateTrustStoreResult {
    #[serde(rename = "TrustStore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_store: Option<TrustStore>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListCloudFrontOriginAccessIdentitiesResult")]
pub struct ListCloudFrontOriginAccessIdentitiesResult {
    #[serde(rename = "CloudFrontOriginAccessIdentityList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_front_origin_access_identity_list: Option<CloudFrontOriginAccessIdentityList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetPublicKeyConfigResult")]
pub struct GetPublicKeyConfigResult {
    #[serde(rename = "PublicKeyConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key_config: Option<PublicKeyConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetCachePolicyConfigRequest")]
pub struct GetCachePolicyConfigRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetResourcePolicyRequest")]
pub struct GetResourcePolicyRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateFieldLevelEncryptionProfileRequest")]
pub struct UpdateFieldLevelEncryptionProfileRequest {
    #[serde(rename = "FieldLevelEncryptionProfileConfig")]
    #[serde(default)]
    pub field_level_encryption_profile_config: FieldLevelEncryptionProfileConfig,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateResponseHeadersPolicyResult")]
pub struct UpdateResponseHeadersPolicyResult {
    #[serde(rename = "ResponseHeadersPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_headers_policy: Option<ResponseHeadersPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListKeyGroupsResult")]
pub struct ListKeyGroupsResult {
    #[serde(rename = "KeyGroupList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_group_list: Option<KeyGroupList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateStreamingDistributionResult")]
pub struct CreateStreamingDistributionResult {
    #[serde(rename = "StreamingDistribution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streaming_distribution: Option<StreamingDistribution>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StreamingDistribution")]
pub struct StreamingDistribution {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "ActiveTrustedSigners")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_trusted_signers: Option<ActiveTrustedSigners>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StreamingDistributionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streaming_distribution_config: Option<StreamingDistributionConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetFieldLevelEncryptionConfigResult")]
pub struct GetFieldLevelEncryptionConfigResult {
    #[serde(rename = "FieldLevelEncryptionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_level_encryption_config: Option<FieldLevelEncryptionConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteCachePolicyRequest")]
pub struct DeleteCachePolicyRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMonitoringSubscriptionResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteResourcePolicyRequest")]
pub struct DeleteResourcePolicyRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetOriginRequestPolicyResult")]
pub struct GetOriginRequestPolicyResult {
    #[serde(rename = "OriginRequestPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_request_policy: Option<OriginRequestPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetStreamingDistributionResult")]
pub struct GetStreamingDistributionResult {
    #[serde(rename = "StreamingDistribution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streaming_distribution: Option<StreamingDistribution>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetPublicKeyConfigRequest")]
pub struct GetPublicKeyConfigRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListCachePoliciesRequest")]
pub struct ListCachePoliciesRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListDistributionsRequest")]
pub struct ListDistributionsRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListDistributionsByRealtimeLogConfigResult")]
pub struct ListDistributionsByRealtimeLogConfigResult {
    #[serde(rename = "DistributionList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_list: Option<DistributionList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListInvalidationsForDistributionTenantResult")]
pub struct ListInvalidationsForDistributionTenantResult {
    #[serde(rename = "InvalidationList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalidation_list: Option<InvalidationList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateMonitoringSubscriptionResult")]
pub struct CreateMonitoringSubscriptionResult {
    #[serde(rename = "MonitoringSubscription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_subscription: Option<MonitoringSubscription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteVpcOriginRequest")]
pub struct DeleteVpcOriginRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    pub if_match: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetFieldLevelEncryptionProfileConfigResult")]
pub struct GetFieldLevelEncryptionProfileConfigResult {
    #[serde(rename = "FieldLevelEncryptionProfileConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_level_encryption_profile_config: Option<FieldLevelEncryptionProfileConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetConnectionFunctionRequest")]
pub struct GetConnectionFunctionRequest {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
    #[serde(rename = "Stage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetInvalidationResult")]
pub struct GetInvalidationResult {
    #[serde(rename = "Invalidation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalidation: Option<Invalidation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListDistributionsByTrustStoreResult")]
pub struct ListDistributionsByTrustStoreResult {
    #[serde(rename = "DistributionList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_list: Option<DistributionList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDistributionTenantResult")]
pub struct CreateDistributionTenantResult {
    #[serde(rename = "DistributionTenant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_tenant: Option<DistributionTenant>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateStreamingDistributionWithTagsResult")]
pub struct CreateStreamingDistributionWithTagsResult {
    #[serde(rename = "StreamingDistribution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streaming_distribution: Option<StreamingDistribution>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateStreamingDistributionResult")]
pub struct UpdateStreamingDistributionResult {
    #[serde(rename = "StreamingDistribution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streaming_distribution: Option<StreamingDistribution>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListDistributionTenantsRequest")]
pub struct ListDistributionTenantsRequest {
    #[serde(rename = "AssociationFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_filter: Option<DistributionTenantAssociationFilter>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DistributionTenantAssociationFilter")]
pub struct DistributionTenantAssociationFilter {
    #[serde(rename = "ConnectionGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_group_id: Option<String>,
    #[serde(rename = "DistributionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDistributionRequest")]
pub struct CreateDistributionRequest {
    #[serde(rename = "DistributionConfig")]
    #[serde(default)]
    pub distribution_config: DistributionConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListContinuousDeploymentPoliciesRequest")]
pub struct ListContinuousDeploymentPoliciesRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListFieldLevelEncryptionProfilesRequest")]
pub struct ListFieldLevelEncryptionProfilesRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateFunctionResult")]
pub struct CreateFunctionResult {
    #[serde(rename = "FunctionSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_summary: Option<FunctionSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetCachePolicyRequest")]
pub struct GetCachePolicyRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetPublicKeyResult")]
pub struct GetPublicKeyResult {
    #[serde(rename = "PublicKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<PublicKey>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateCloudFrontOriginAccessIdentityResult")]
pub struct UpdateCloudFrontOriginAccessIdentityResult {
    #[serde(rename = "CloudFrontOriginAccessIdentity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_front_origin_access_identity: Option<CloudFrontOriginAccessIdentity>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListCloudFrontOriginAccessIdentitiesRequest")]
pub struct ListCloudFrontOriginAccessIdentitiesRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TestConnectionFunctionResult")]
pub struct TestConnectionFunctionResult {
    #[serde(rename = "ConnectionFunctionTestResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_function_test_result: Option<ConnectionFunctionTestResult>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetCloudFrontOriginAccessIdentityConfigRequest")]
pub struct GetCloudFrontOriginAccessIdentityConfigRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDistributionWithTagsRequest")]
pub struct CreateDistributionWithTagsRequest {
    #[serde(rename = "DistributionConfigWithTags")]
    #[serde(default)]
    pub distribution_config_with_tags: DistributionConfigWithTags,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DisassociateDistributionTenantWebACLResult")]
pub struct DisassociateDistributionTenantWebACLResult {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetContinuousDeploymentPolicyRequest")]
pub struct GetContinuousDeploymentPolicyRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetOriginAccessControlConfigResult")]
pub struct GetOriginAccessControlConfigResult {
    #[serde(rename = "OriginAccessControlConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_access_control_config: Option<OriginAccessControlConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetResponseHeadersPolicyConfigRequest")]
pub struct GetResponseHeadersPolicyConfigRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetStreamingDistributionConfigResult")]
pub struct GetStreamingDistributionConfigResult {
    #[serde(rename = "StreamingDistributionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streaming_distribution_config: Option<StreamingDistributionConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListFieldLevelEncryptionConfigsResult")]
pub struct ListFieldLevelEncryptionConfigsResult {
    #[serde(rename = "FieldLevelEncryptionList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_level_encryption_list: Option<FieldLevelEncryptionList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListDistributionsByResponseHeadersPolicyIdRequest")]
pub struct ListDistributionsByResponseHeadersPolicyIdRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "ResponseHeadersPolicyId")]
    #[serde(default)]
    pub response_headers_policy_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteKeyGroupRequest")]
pub struct DeleteKeyGroupRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListStreamingDistributionsRequest")]
pub struct ListStreamingDistributionsRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateFunctionResult")]
pub struct UpdateFunctionResult {
    #[serde(rename = "FunctionSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_summary: Option<FunctionSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateDistributionTenantResult")]
pub struct UpdateDistributionTenantResult {
    #[serde(rename = "DistributionTenant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_tenant: Option<DistributionTenant>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateCloudFrontOriginAccessIdentityResult")]
pub struct CreateCloudFrontOriginAccessIdentityResult {
    #[serde(rename = "CloudFrontOriginAccessIdentity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_front_origin_access_identity: Option<CloudFrontOriginAccessIdentity>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteRealtimeLogConfigRequest")]
pub struct DeleteRealtimeLogConfigRequest {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TestFunctionResult")]
pub struct TestFunctionResult {
    #[serde(rename = "TestResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_result: Option<TestResult>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "VerifyDnsConfigurationRequest")]
pub struct VerifyDnsConfigurationRequest {
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListDistributionsResult")]
pub struct ListDistributionsResult {
    #[serde(rename = "DistributionList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_list: Option<DistributionList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AssociateDistributionWebACLRequest")]
pub struct AssociateDistributionWebACLRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match: Option<String>,
    #[serde(rename = "WebACLArn")]
    #[serde(default)]
    pub web_a_c_l_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListDistributionsByConnectionModeRequest")]
pub struct ListDistributionsByConnectionModeRequest {
    #[serde(rename = "ConnectionMode")]
    #[serde(default)]
    pub connection_mode: String,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteVpcOriginResult")]
pub struct DeleteVpcOriginResult {
    #[serde(rename = "VpcOrigin")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_origin: Option<VpcOrigin>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListDistributionsByVpcOriginIdRequest")]
pub struct ListDistributionsByVpcOriginIdRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "VpcOriginId")]
    #[serde(default)]
    pub vpc_origin_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CopyDistributionResult")]
pub struct CopyDistributionResult {
    #[serde(rename = "Distribution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution: Option<Distribution>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDistributionResult")]
pub struct CreateDistributionResult {
    #[serde(rename = "Distribution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution: Option<Distribution>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DisassociateDistributionWebACLResult")]
pub struct DisassociateDistributionWebACLResult {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListDistributionsByConnectionModeResult")]
pub struct ListDistributionsByConnectionModeResult {
    #[serde(rename = "DistributionList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_list: Option<DistributionList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetContinuousDeploymentPolicyResult")]
pub struct GetContinuousDeploymentPolicyResult {
    #[serde(rename = "ContinuousDeploymentPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuous_deployment_policy: Option<ContinuousDeploymentPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeKeyValueStoreRequest")]
pub struct DescribeKeyValueStoreRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetConnectionGroupResult")]
pub struct GetConnectionGroupResult {
    #[serde(rename = "ConnectionGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_group: Option<ConnectionGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetDistributionTenantRequest")]
pub struct GetDistributionTenantRequest {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetFunctionResult")]
pub struct GetFunctionResult {
    #[serde(rename = "FunctionCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateKeyGroupResult")]
pub struct CreateKeyGroupResult {
    #[serde(rename = "KeyGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_group: Option<KeyGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetFieldLevelEncryptionConfigRequest")]
pub struct GetFieldLevelEncryptionConfigRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CopyDistributionRequest")]
pub struct CopyDistributionRequest {
    #[serde(rename = "CallerReference")]
    #[serde(default)]
    pub caller_reference: String,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match: Option<String>,
    #[serde(rename = "PrimaryDistributionId")]
    #[serde(default)]
    pub primary_distribution_id: String,
    #[serde(rename = "Staging")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staging: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetRealtimeLogConfigResult")]
pub struct GetRealtimeLogConfigResult {
    #[serde(rename = "RealtimeLogConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realtime_log_config: Option<RealtimeLogConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateFunctionRequest")]
pub struct CreateFunctionRequest {
    #[serde(rename = "FunctionCode")]
    #[serde(default)]
    pub function_code: String,
    #[serde(rename = "FunctionConfig")]
    #[serde(default)]
    pub function_config: FunctionConfig,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteConnectionFunctionRequest")]
pub struct DeleteConnectionFunctionRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    pub if_match: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreatePublicKeyRequest")]
pub struct CreatePublicKeyRequest {
    #[serde(rename = "PublicKeyConfig")]
    #[serde(default)]
    pub public_key_config: PublicKeyConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetResourcePolicyResult")]
pub struct GetResourcePolicyResult {
    #[serde(rename = "PolicyDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_document: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateCachePolicyResult")]
pub struct UpdateCachePolicyResult {
    #[serde(rename = "CachePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_policy: Option<CachePolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListDistributionTenantsResult")]
pub struct ListDistributionTenantsResult {
    #[serde(rename = "DistributionTenantList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_tenant_list: Option<DistributionTenantList>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DistributionTenantList {
    #[serde(
        rename = "DistributionTenantSummary",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DistributionTenantSummary>,
}
impl From<Vec<DistributionTenantSummary>> for DistributionTenantList {
    fn from(v: Vec<DistributionTenantSummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DistributionTenantSummary> for DistributionTenantList {
    fn from_iter<I: IntoIterator<Item = DistributionTenantSummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DistributionTenantSummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDistributionTenantSummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DistributionTenantSummary>,
}

impl From<Vec<DistributionTenantSummary>> for XmlDistributionTenantSummaryList {
    fn from(v: Vec<DistributionTenantSummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DistributionTenantSummary> for XmlDistributionTenantSummaryList {
    fn from_iter<I: IntoIterator<Item = DistributionTenantSummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DistributionTenantSummary")]
pub struct DistributionTenantSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ConnectionGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_group_id: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "Customizations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customizations: Option<Customizations>,
    #[serde(rename = "DistributionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_id: Option<String>,
    #[serde(rename = "Domains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<DomainResultList>,
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetDistributionConfigRequest")]
pub struct GetDistributionConfigRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PublishConnectionFunctionResult")]
pub struct PublishConnectionFunctionResult {
    #[serde(rename = "ConnectionFunctionSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_function_summary: Option<ConnectionFunctionSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateCachePolicyRequest")]
pub struct UpdateCachePolicyRequest {
    #[serde(rename = "CachePolicyConfig")]
    #[serde(default)]
    pub cache_policy_config: CachePolicyConfig,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateDistributionRequest")]
pub struct UpdateDistributionRequest {
    #[serde(rename = "DistributionConfig")]
    #[serde(default)]
    pub distribution_config: DistributionConfig,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetConnectionGroupRequest")]
pub struct GetConnectionGroupRequest {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListConflictingAliasesRequest")]
pub struct ListConflictingAliasesRequest {
    #[serde(rename = "Alias")]
    #[serde(default)]
    pub alias: String,
    #[serde(rename = "DistributionId")]
    #[serde(default)]
    pub distribution_id: String,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateFieldLevelEncryptionConfigRequest")]
pub struct UpdateFieldLevelEncryptionConfigRequest {
    #[serde(rename = "FieldLevelEncryptionConfig")]
    #[serde(default)]
    pub field_level_encryption_config: FieldLevelEncryptionConfig,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListDistributionTenantsByCustomizationResult")]
pub struct ListDistributionTenantsByCustomizationResult {
    #[serde(rename = "DistributionTenantList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_tenant_list: Option<DistributionTenantList>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateKeyValueStoreResult")]
pub struct UpdateKeyValueStoreResult {
    #[serde(rename = "KeyValueStore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_value_store: Option<KeyValueStore>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteKeyValueStoreRequest")]
pub struct DeleteKeyValueStoreRequest {
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    pub if_match: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateFieldLevelEncryptionProfileRequest")]
pub struct CreateFieldLevelEncryptionProfileRequest {
    #[serde(rename = "FieldLevelEncryptionProfileConfig")]
    #[serde(default)]
    pub field_level_encryption_profile_config: FieldLevelEncryptionProfileConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateFieldLevelEncryptionConfigResult")]
pub struct CreateFieldLevelEncryptionConfigResult {
    #[serde(rename = "FieldLevelEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_level_encryption: Option<FieldLevelEncryption>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetDistributionResult")]
pub struct GetDistributionResult {
    #[serde(rename = "Distribution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution: Option<Distribution>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetVpcOriginResult")]
pub struct GetVpcOriginResult {
    #[serde(rename = "VpcOrigin")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_origin: Option<VpcOrigin>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListKeyValueStoresResult")]
pub struct ListKeyValueStoresResult {
    #[serde(rename = "KeyValueStoreList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_value_store_list: Option<KeyValueStoreList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AssociateAliasRequest")]
pub struct AssociateAliasRequest {
    #[serde(rename = "Alias")]
    #[serde(default)]
    pub alias: String,
    #[serde(rename = "TargetDistributionId")]
    #[serde(default)]
    pub target_distribution_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateCloudFrontOriginAccessIdentityRequest")]
pub struct CreateCloudFrontOriginAccessIdentityRequest {
    #[serde(rename = "CloudFrontOriginAccessIdentityConfig")]
    #[serde(default)]
    pub cloud_front_origin_access_identity_config: CloudFrontOriginAccessIdentityConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetResponseHeadersPolicyRequest")]
pub struct GetResponseHeadersPolicyRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateOriginRequestPolicyRequest")]
pub struct CreateOriginRequestPolicyRequest {
    #[serde(rename = "OriginRequestPolicyConfig")]
    #[serde(default)]
    pub origin_request_policy_config: OriginRequestPolicyConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateInvalidationForDistributionTenantRequest")]
pub struct CreateInvalidationForDistributionTenantRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "InvalidationBatch")]
    #[serde(default)]
    pub invalidation_batch: InvalidationBatch,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetConnectionGroupByRoutingEndpointResult")]
pub struct GetConnectionGroupByRoutingEndpointResult {
    #[serde(rename = "ConnectionGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_group: Option<ConnectionGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListDistributionsByResponseHeadersPolicyIdResult")]
pub struct ListDistributionsByResponseHeadersPolicyIdResult {
    #[serde(rename = "DistributionIdList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_id_list: Option<DistributionIdList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListPublicKeysResult")]
pub struct ListPublicKeysResult {
    #[serde(rename = "PublicKeyList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key_list: Option<PublicKeyList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListDistributionsByAnycastIpListIdRequest")]
pub struct ListDistributionsByAnycastIpListIdRequest {
    #[serde(rename = "AnycastIpListId")]
    #[serde(default)]
    pub anycast_ip_list_id: String,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTrustStoresRequest")]
pub struct ListTrustStoresRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeConnectionFunctionRequest")]
pub struct DescribeConnectionFunctionRequest {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
    #[serde(rename = "Stage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetContinuousDeploymentPolicyConfigResult")]
pub struct GetContinuousDeploymentPolicyConfigResult {
    #[serde(rename = "ContinuousDeploymentPolicyConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuous_deployment_policy_config: Option<ContinuousDeploymentPolicyConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetMonitoringSubscriptionRequest")]
pub struct GetMonitoringSubscriptionRequest {
    #[serde(rename = "DistributionId")]
    #[serde(default)]
    pub distribution_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TestFunctionRequest")]
pub struct TestFunctionRequest {
    #[serde(rename = "EventObject")]
    #[serde(default)]
    pub event_object: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    pub if_match: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Stage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateKeyValueStoreResult")]
pub struct CreateKeyValueStoreResult {
    #[serde(rename = "KeyValueStore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_value_store: Option<KeyValueStore>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeFunctionResult")]
pub struct DescribeFunctionResult {
    #[serde(rename = "FunctionSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_summary: Option<FunctionSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetStreamingDistributionRequest")]
pub struct GetStreamingDistributionRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateOriginRequestPolicyResult")]
pub struct UpdateOriginRequestPolicyResult {
    #[serde(rename = "OriginRequestPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_request_policy: Option<OriginRequestPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdatePublicKeyResult")]
pub struct UpdatePublicKeyResult {
    #[serde(rename = "PublicKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<PublicKey>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteDistributionTenantRequest")]
pub struct DeleteDistributionTenantRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    pub if_match: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteOriginRequestPolicyRequest")]
pub struct DeleteOriginRequestPolicyRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeConnectionFunctionResult")]
pub struct DescribeConnectionFunctionResult {
    #[serde(rename = "ConnectionFunctionSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_function_summary: Option<ConnectionFunctionSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetDistributionRequest")]
pub struct GetDistributionRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetFieldLevelEncryptionProfileResult")]
pub struct GetFieldLevelEncryptionProfileResult {
    #[serde(rename = "FieldLevelEncryptionProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_level_encryption_profile: Option<FieldLevelEncryptionProfile>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetRealtimeLogConfigRequest")]
pub struct GetRealtimeLogConfigRequest {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CachePolicyList")]
pub struct CachePolicyList {
    #[serde(rename = "Items")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<CachePolicySummaryList>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CachePolicySummaryList {
    #[serde(
        rename = "CachePolicySummary",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<CachePolicySummary>,
}
impl From<Vec<CachePolicySummary>> for CachePolicySummaryList {
    fn from(v: Vec<CachePolicySummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<CachePolicySummary> for CachePolicySummaryList {
    fn from_iter<I: IntoIterator<Item = CachePolicySummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<CachePolicySummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlCachePolicySummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<CachePolicySummary>,
}

impl From<Vec<CachePolicySummary>> for XmlCachePolicySummaryList {
    fn from(v: Vec<CachePolicySummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<CachePolicySummary> for XmlCachePolicySummaryList {
    fn from_iter<I: IntoIterator<Item = CachePolicySummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CachePolicySummary")]
pub struct CachePolicySummary {
    #[serde(rename = "CachePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_policy: Option<CachePolicy>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListDistributionsByOriginRequestPolicyIdResult")]
pub struct ListDistributionsByOriginRequestPolicyIdResult {
    #[serde(rename = "DistributionIdList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_id_list: Option<DistributionIdList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListCachePoliciesResult")]
pub struct ListCachePoliciesResult {
    #[serde(rename = "CachePolicyList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_policy_list: Option<CachePolicyList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListDistributionsByWebACLIdResult")]
pub struct ListDistributionsByWebACLIdResult {
    #[serde(rename = "DistributionList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_list: Option<DistributionList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateAnycastIpListRequest")]
pub struct UpdateAnycastIpListRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    pub if_match: String,
    #[serde(rename = "IpAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateDistributionTenantRequest")]
pub struct UpdateDistributionTenantRequest {
    #[serde(rename = "ConnectionGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_group_id: Option<String>,
    #[serde(rename = "Customizations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customizations: Option<Customizations>,
    #[serde(rename = "DistributionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_id: Option<String>,
    #[serde(rename = "Domains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<DomainList>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    pub if_match: String,
    #[serde(rename = "ManagedCertificateRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_certificate_request: Option<ManagedCertificateRequest>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Parameters>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetConnectionFunctionResult")]
pub struct GetConnectionFunctionResult {
    #[serde(rename = "ConnectionFunctionCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_function_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListDistributionsByOriginRequestPolicyIdRequest")]
pub struct ListDistributionsByOriginRequestPolicyIdRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "OriginRequestPolicyId")]
    #[serde(default)]
    pub origin_request_policy_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateFieldLevelEncryptionProfileResult")]
pub struct UpdateFieldLevelEncryptionProfileResult {
    #[serde(rename = "FieldLevelEncryptionProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_level_encryption_profile: Option<FieldLevelEncryptionProfile>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateOriginAccessControlRequest")]
pub struct UpdateOriginAccessControlRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match: Option<String>,
    #[serde(rename = "OriginAccessControlConfig")]
    #[serde(default)]
    pub origin_access_control_config: OriginAccessControlConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateVpcOriginResult")]
pub struct CreateVpcOriginResult {
    #[serde(rename = "VpcOrigin")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_origin: Option<VpcOrigin>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateOriginAccessControlResult")]
pub struct CreateOriginAccessControlResult {
    #[serde(rename = "OriginAccessControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_access_control: Option<OriginAccessControl>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TestConnectionFunctionRequest")]
pub struct TestConnectionFunctionRequest {
    #[serde(rename = "ConnectionObject")]
    #[serde(default)]
    pub connection_object: String,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    pub if_match: String,
    #[serde(rename = "Stage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateRealtimeLogConfigRequest")]
pub struct CreateRealtimeLogConfigRequest {
    #[serde(rename = "EndPoints")]
    #[serde(default)]
    pub end_points: EndPointList,
    #[serde(rename = "Fields")]
    #[serde(default)]
    pub fields: FieldList,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "SamplingRate")]
    #[serde(default)]
    pub sampling_rate: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListOriginRequestPoliciesRequest")]
pub struct ListOriginRequestPoliciesRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetDistributionTenantByDomainRequest")]
pub struct GetDistributionTenantByDomainRequest {
    #[serde(rename = "Domain")]
    #[serde(default)]
    pub domain: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListDistributionsByCachePolicyIdResult")]
pub struct ListDistributionsByCachePolicyIdResult {
    #[serde(rename = "DistributionIdList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_id_list: Option<DistributionIdList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateConnectionGroupResult")]
pub struct CreateConnectionGroupResult {
    #[serde(rename = "ConnectionGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_group: Option<ConnectionGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteOriginAccessControlRequest")]
pub struct DeleteOriginAccessControlRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListConnectionFunctionsRequest")]
pub struct ListConnectionFunctionsRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "Stage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateTrustStoreResult")]
pub struct UpdateTrustStoreResult {
    #[serde(rename = "TrustStore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_store: Option<TrustStore>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetAnycastIpListResult")]
pub struct GetAnycastIpListResult {
    #[serde(rename = "AnycastIpList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anycast_ip_list: Option<AnycastIpList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateCloudFrontOriginAccessIdentityRequest")]
pub struct UpdateCloudFrontOriginAccessIdentityRequest {
    #[serde(rename = "CloudFrontOriginAccessIdentityConfig")]
    #[serde(default)]
    pub cloud_front_origin_access_identity_config: CloudFrontOriginAccessIdentityConfig,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match: Option<String>,
}
