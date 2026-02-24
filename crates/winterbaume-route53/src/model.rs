//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-route53

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCheckerIpRangesRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateHealthCheckRequest")]
pub struct UpdateHealthCheckRequest {
    #[serde(rename = "AlarmIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_identifier: Option<AlarmIdentifier>,
    #[serde(rename = "ChildHealthChecks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_health_checks: Option<ChildHealthCheckList>,
    #[serde(rename = "Disabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(rename = "EnableSNI")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_s_n_i: Option<bool>,
    #[serde(rename = "FailureThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_threshold: Option<i32>,
    #[serde(rename = "FullyQualifiedDomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fully_qualified_domain_name: Option<String>,
    #[serde(rename = "HealthCheckId")]
    #[serde(default)]
    pub health_check_id: String,
    #[serde(rename = "HealthCheckVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_version: Option<i64>,
    #[serde(rename = "HealthThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_threshold: Option<i32>,
    #[serde(rename = "IPAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_p_address: Option<String>,
    #[serde(rename = "InsufficientDataHealthStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insufficient_data_health_status: Option<String>,
    #[serde(rename = "Inverted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inverted: Option<bool>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "Regions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<HealthCheckRegionList>,
    #[serde(rename = "ResetElements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_elements: Option<ResettableElementNameList>,
    #[serde(rename = "ResourcePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_path: Option<String>,
    #[serde(rename = "SearchString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_string: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AlarmIdentifier")]
pub struct AlarmIdentifier {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Region")]
    #[serde(default)]
    pub region: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChildHealthCheckList {
    #[serde(
        rename = "ChildHealthCheck",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ChildHealthCheckList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ChildHealthCheckList {
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
pub struct HealthCheckRegionList {
    #[serde(rename = "Region", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for HealthCheckRegionList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for HealthCheckRegionList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResettableElementNameList {
    #[serde(
        rename = "ResettableElementName",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ResettableElementNameList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ResettableElementNameList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateTrafficPolicyCommentRequest")]
pub struct UpdateTrafficPolicyCommentRequest {
    #[serde(rename = "Comment")]
    #[serde(default)]
    pub comment: String,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Version")]
    #[serde(default)]
    pub version: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteQueryLoggingConfigRequest")]
pub struct DeleteQueryLoggingConfigRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTrafficPoliciesRequest")]
pub struct ListTrafficPoliciesRequest {
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "TrafficPolicyIdMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_policy_id_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateReusableDelegationSetRequest")]
pub struct CreateReusableDelegationSetRequest {
    #[serde(rename = "CallerReference")]
    #[serde(default)]
    pub caller_reference: String,
    #[serde(rename = "HostedZoneId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeactivateKeySigningKeyRequest")]
pub struct DeactivateKeySigningKeyRequest {
    #[serde(rename = "HostedZoneId")]
    #[serde(default)]
    pub hosted_zone_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTrafficPolicyInstanceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateCidrCollectionRequest")]
pub struct CreateCidrCollectionRequest {
    #[serde(rename = "CallerReference")]
    #[serde(default)]
    pub caller_reference: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ChangeResourceRecordSetsRequest")]
pub struct ChangeResourceRecordSetsRequest {
    #[serde(rename = "ChangeBatch")]
    #[serde(default)]
    pub change_batch: ChangeBatch,
    #[serde(rename = "HostedZoneId")]
    #[serde(default)]
    pub hosted_zone_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ChangeBatch")]
pub struct ChangeBatch {
    #[serde(rename = "Changes")]
    #[serde(default)]
    pub changes: Changes,
    #[serde(rename = "Comment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Changes {
    #[serde(rename = "Change", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Change>,
}
impl From<Vec<Change>> for Changes {
    fn from(v: Vec<Change>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Change> for Changes {
    fn from_iter<I: IntoIterator<Item = Change>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Change>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlChangeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Change>,
}

impl From<Vec<Change>> for XmlChangeList {
    fn from(v: Vec<Change>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Change> for XmlChangeList {
    fn from_iter<I: IntoIterator<Item = Change>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Change")]
pub struct Change {
    #[serde(rename = "Action")]
    #[serde(default)]
    pub action: String,
    #[serde(rename = "ResourceRecordSet")]
    #[serde(default)]
    pub resource_record_set: ResourceRecordSet,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResourceRecordSet")]
pub struct ResourceRecordSet {
    #[serde(rename = "AliasTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_target: Option<AliasTarget>,
    #[serde(rename = "CidrRoutingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_routing_config: Option<CidrRoutingConfig>,
    #[serde(rename = "Failover")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failover: Option<String>,
    #[serde(rename = "GeoLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_location: Option<GeoLocation>,
    #[serde(rename = "GeoProximityLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_proximity_location: Option<GeoProximityLocation>,
    #[serde(rename = "HealthCheckId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_id: Option<String>,
    #[serde(rename = "MultiValueAnswer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_value_answer: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "ResourceRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_records: Option<ResourceRecords>,
    #[serde(rename = "SetIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_identifier: Option<String>,
    #[serde(rename = "TTL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_t_l: Option<i64>,
    #[serde(rename = "TrafficPolicyInstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_policy_instance_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "Weight")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AliasTarget")]
pub struct AliasTarget {
    #[serde(rename = "DNSName")]
    #[serde(default)]
    pub d_n_s_name: String,
    #[serde(rename = "EvaluateTargetHealth")]
    #[serde(default)]
    pub evaluate_target_health: bool,
    #[serde(rename = "HostedZoneId")]
    #[serde(default)]
    pub hosted_zone_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CidrRoutingConfig")]
pub struct CidrRoutingConfig {
    #[serde(rename = "CollectionId")]
    #[serde(default)]
    pub collection_id: String,
    #[serde(rename = "LocationName")]
    #[serde(default)]
    pub location_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GeoLocation")]
pub struct GeoLocation {
    #[serde(rename = "ContinentCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continent_code: Option<String>,
    #[serde(rename = "CountryCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(rename = "SubdivisionCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdivision_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GeoProximityLocation")]
pub struct GeoProximityLocation {
    #[serde(rename = "AWSRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_w_s_region: Option<String>,
    #[serde(rename = "Bias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bias: Option<i32>,
    #[serde(rename = "Coordinates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<Coordinates>,
    #[serde(rename = "LocalZoneGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_zone_group: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Coordinates")]
pub struct Coordinates {
    #[serde(rename = "Latitude")]
    #[serde(default)]
    pub latitude: String,
    #[serde(rename = "Longitude")]
    #[serde(default)]
    pub longitude: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceRecords {
    #[serde(
        rename = "ResourceRecord",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ResourceRecord>,
}
impl From<Vec<ResourceRecord>> for ResourceRecords {
    fn from(v: Vec<ResourceRecord>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ResourceRecord> for ResourceRecords {
    fn from_iter<I: IntoIterator<Item = ResourceRecord>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ResourceRecord>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlResourceRecordList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ResourceRecord>,
}

impl From<Vec<ResourceRecord>> for XmlResourceRecordList {
    fn from(v: Vec<ResourceRecord>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ResourceRecord> for XmlResourceRecordList {
    fn from_iter<I: IntoIterator<Item = ResourceRecord>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResourceRecord")]
pub struct ResourceRecord {
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetDNSSECResponse")]
pub struct GetDNSSECResponse {
    #[serde(rename = "KeySigningKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_signing_keys: Option<KeySigningKeys>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DNSSECStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KeySigningKeys {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<KeySigningKey>,
}
impl From<Vec<KeySigningKey>> for KeySigningKeys {
    fn from(v: Vec<KeySigningKey>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<KeySigningKey> for KeySigningKeys {
    fn from_iter<I: IntoIterator<Item = KeySigningKey>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<KeySigningKey>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlKeySigningKeyList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<KeySigningKey>,
}

impl From<Vec<KeySigningKey>> for XmlKeySigningKeyList {
    fn from(v: Vec<KeySigningKey>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<KeySigningKey> for XmlKeySigningKeyList {
    fn from_iter<I: IntoIterator<Item = KeySigningKey>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "KeySigningKey")]
pub struct KeySigningKey {
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(rename = "DNSKEYRecord")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_n_s_k_e_y_record: Option<String>,
    #[serde(rename = "DSRecord")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_s_record: Option<String>,
    #[serde(rename = "DigestAlgorithmMnemonic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest_algorithm_mnemonic: Option<String>,
    #[serde(rename = "DigestAlgorithmType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest_algorithm_type: Option<i32>,
    #[serde(rename = "DigestValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest_value: Option<String>,
    #[serde(rename = "Flag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flag: Option<i32>,
    #[serde(rename = "KeyTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_tag: Option<i32>,
    #[serde(rename = "KmsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_arn: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PublicKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[serde(rename = "SigningAlgorithmMnemonic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_algorithm_mnemonic: Option<String>,
    #[serde(rename = "SigningAlgorithmType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_algorithm_type: Option<i32>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DNSSECStatus")]
pub struct DNSSECStatus {
    #[serde(rename = "ServeSignature")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serve_signature: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetHealthCheckLastFailureReasonRequest")]
pub struct GetHealthCheckLastFailureReasonRequest {
    #[serde(rename = "HealthCheckId")]
    #[serde(default)]
    pub health_check_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateTrafficPolicyVersionResponse")]
pub struct CreateTrafficPolicyVersionResponse {
    #[serde(rename = "TrafficPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_policy: Option<TrafficPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TrafficPolicy")]
pub struct TrafficPolicy {
    #[serde(rename = "Comment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "Document")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateTrafficPolicyVersionRequest")]
pub struct CreateTrafficPolicyVersionRequest {
    #[serde(rename = "Comment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "Document")]
    #[serde(default)]
    pub document: String,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetHostedZoneRequest")]
pub struct GetHostedZoneRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetHostedZoneLimitRequest")]
pub struct GetHostedZoneLimitRequest {
    #[serde(rename = "HostedZoneId")]
    #[serde(default)]
    pub hosted_zone_id: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTrafficPolicyInstancesByHostedZoneRequest")]
pub struct ListTrafficPolicyInstancesByHostedZoneRequest {
    #[serde(rename = "HostedZoneId")]
    #[serde(default)]
    pub hosted_zone_id: String,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "TrafficPolicyInstanceNameMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_policy_instance_name_marker: Option<String>,
    #[serde(rename = "TrafficPolicyInstanceTypeMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_policy_instance_type_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListCidrLocationsResponse")]
pub struct ListCidrLocationsResponse {
    #[serde(rename = "CidrLocations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_locations: Option<LocationSummaries>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LocationSummaries {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<LocationSummary>,
}
impl From<Vec<LocationSummary>> for LocationSummaries {
    fn from(v: Vec<LocationSummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<LocationSummary> for LocationSummaries {
    fn from_iter<I: IntoIterator<Item = LocationSummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<LocationSummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlLocationSummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<LocationSummary>,
}

impl From<Vec<LocationSummary>> for XmlLocationSummaryList {
    fn from(v: Vec<LocationSummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<LocationSummary> for XmlLocationSummaryList {
    fn from_iter<I: IntoIterator<Item = LocationSummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LocationSummary")]
pub struct LocationSummary {
    #[serde(rename = "LocationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTrafficPolicyInstancesByPolicyResponse")]
pub struct ListTrafficPolicyInstancesByPolicyResponse {
    #[serde(rename = "HostedZoneIdMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id_marker: Option<String>,
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "TrafficPolicyInstanceNameMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_policy_instance_name_marker: Option<String>,
    #[serde(rename = "TrafficPolicyInstanceTypeMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_policy_instance_type_marker: Option<String>,
    #[serde(rename = "TrafficPolicyInstances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_policy_instances: Option<TrafficPolicyInstances>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrafficPolicyInstances {
    #[serde(
        rename = "TrafficPolicyInstance",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<TrafficPolicyInstance>,
}
impl From<Vec<TrafficPolicyInstance>> for TrafficPolicyInstances {
    fn from(v: Vec<TrafficPolicyInstance>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<TrafficPolicyInstance> for TrafficPolicyInstances {
    fn from_iter<I: IntoIterator<Item = TrafficPolicyInstance>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<TrafficPolicyInstance>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTrafficPolicyInstanceList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<TrafficPolicyInstance>,
}

impl From<Vec<TrafficPolicyInstance>> for XmlTrafficPolicyInstanceList {
    fn from(v: Vec<TrafficPolicyInstance>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<TrafficPolicyInstance> for XmlTrafficPolicyInstanceList {
    fn from_iter<I: IntoIterator<Item = TrafficPolicyInstance>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TrafficPolicyInstance")]
pub struct TrafficPolicyInstance {
    #[serde(rename = "HostedZoneId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "TTL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_t_l: Option<i64>,
    #[serde(rename = "TrafficPolicyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_policy_id: Option<String>,
    #[serde(rename = "TrafficPolicyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_policy_type: Option<String>,
    #[serde(rename = "TrafficPolicyVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_policy_version: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVPCAssociationAuthorizationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TestDNSAnswerRequest")]
pub struct TestDNSAnswerRequest {
    #[serde(rename = "EDNS0ClientSubnetIP")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_d_n_s0_client_subnet_i_p: Option<String>,
    #[serde(rename = "EDNS0ClientSubnetMask")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_d_n_s0_client_subnet_mask: Option<String>,
    #[serde(rename = "HostedZoneId")]
    #[serde(default)]
    pub hosted_zone_id: String,
    #[serde(rename = "RecordName")]
    #[serde(default)]
    pub record_name: String,
    #[serde(rename = "RecordType")]
    #[serde(default)]
    pub record_type: String,
    #[serde(rename = "ResolverIP")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_i_p: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DisassociateVPCFromHostedZoneResponse")]
pub struct DisassociateVPCFromHostedZoneResponse {
    #[serde(rename = "ChangeInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_info: Option<ChangeInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ChangeInfo")]
pub struct ChangeInfo {
    #[serde(rename = "Comment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "SubmittedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListCidrBlocksRequest")]
pub struct ListCidrBlocksRequest {
    #[serde(rename = "CollectionId")]
    #[serde(default)]
    pub collection_id: String,
    #[serde(rename = "LocationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteCidrCollectionRequest")]
pub struct DeleteCidrCollectionRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListQueryLoggingConfigsRequest")]
pub struct ListQueryLoggingConfigsRequest {
    #[serde(rename = "HostedZoneId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTrafficPolicyVersionsRequest")]
pub struct ListTrafficPolicyVersionsRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "TrafficPolicyVersionMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_policy_version_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetDNSSECRequest")]
pub struct GetDNSSECRequest {
    #[serde(rename = "HostedZoneId")]
    #[serde(default)]
    pub hosted_zone_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateCidrCollectionResponse")]
pub struct CreateCidrCollectionResponse {
    #[serde(rename = "Collection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection: Option<CidrCollection>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CidrCollection")]
pub struct CidrCollection {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DisableHostedZoneDNSSECResponse")]
pub struct DisableHostedZoneDNSSECResponse {
    #[serde(rename = "ChangeInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_info: Option<ChangeInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteTrafficPolicyRequest")]
pub struct DeleteTrafficPolicyRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Version")]
    #[serde(default)]
    pub version: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetTrafficPolicyInstanceRequest")]
pub struct GetTrafficPolicyInstanceRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateHostedZoneRequest")]
pub struct CreateHostedZoneRequest {
    #[serde(rename = "CallerReference")]
    #[serde(default)]
    pub caller_reference: String,
    #[serde(rename = "DelegationSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegation_set_id: Option<String>,
    #[serde(rename = "HostedZoneConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_config: Option<HostedZoneConfig>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "VPC")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_p_c: Option<VPC>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "HostedZoneConfig")]
pub struct HostedZoneConfig {
    #[serde(rename = "Comment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "PrivateZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_zone: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "VPC")]
pub struct VPC {
    #[serde(rename = "VPCId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_p_c_id: Option<String>,
    #[serde(rename = "VPCRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_p_c_region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTrafficPoliciesResponse")]
pub struct ListTrafficPoliciesResponse {
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "TrafficPolicyIdMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_policy_id_marker: Option<String>,
    #[serde(rename = "TrafficPolicySummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_policy_summaries: Option<TrafficPolicySummaries>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrafficPolicySummaries {
    #[serde(
        rename = "TrafficPolicySummary",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<TrafficPolicySummary>,
}
impl From<Vec<TrafficPolicySummary>> for TrafficPolicySummaries {
    fn from(v: Vec<TrafficPolicySummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<TrafficPolicySummary> for TrafficPolicySummaries {
    fn from_iter<I: IntoIterator<Item = TrafficPolicySummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<TrafficPolicySummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTrafficPolicySummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<TrafficPolicySummary>,
}

impl From<Vec<TrafficPolicySummary>> for XmlTrafficPolicySummaryList {
    fn from(v: Vec<TrafficPolicySummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<TrafficPolicySummary> for XmlTrafficPolicySummaryList {
    fn from_iter<I: IntoIterator<Item = TrafficPolicySummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TrafficPolicySummary")]
pub struct TrafficPolicySummary {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LatestVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "TrafficPolicyCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_policy_count: Option<i32>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetReusableDelegationSetRequest")]
pub struct GetReusableDelegationSetRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateReusableDelegationSetResponse")]
pub struct CreateReusableDelegationSetResponse {
    #[serde(rename = "DelegationSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegation_set: Option<DelegationSet>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DelegationSet")]
pub struct DelegationSet {
    #[serde(rename = "CallerReference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caller_reference: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "NameServers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_servers: Option<DelegationSetNameServers>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DelegationSetNameServers {
    #[serde(rename = "NameServer", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for DelegationSetNameServers {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for DelegationSetNameServers {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ChangeCidrCollectionRequest")]
pub struct ChangeCidrCollectionRequest {
    #[serde(rename = "Changes")]
    #[serde(default)]
    pub changes: CidrCollectionChanges,
    #[serde(rename = "CollectionVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_version: Option<i64>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CidrCollectionChanges {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<CidrCollectionChange>,
}
impl From<Vec<CidrCollectionChange>> for CidrCollectionChanges {
    fn from(v: Vec<CidrCollectionChange>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<CidrCollectionChange> for CidrCollectionChanges {
    fn from_iter<I: IntoIterator<Item = CidrCollectionChange>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<CidrCollectionChange>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlCidrCollectionChangeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<CidrCollectionChange>,
}

impl From<Vec<CidrCollectionChange>> for XmlCidrCollectionChangeList {
    fn from(v: Vec<CidrCollectionChange>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<CidrCollectionChange> for XmlCidrCollectionChangeList {
    fn from_iter<I: IntoIterator<Item = CidrCollectionChange>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CidrCollectionChange")]
pub struct CidrCollectionChange {
    #[serde(rename = "Action")]
    #[serde(default)]
    pub action: String,
    #[serde(rename = "CidrList")]
    #[serde(default)]
    pub cidr_list: CidrList,
    #[serde(rename = "LocationName")]
    #[serde(default)]
    pub location_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CidrList {
    #[serde(rename = "Cidr", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for CidrList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for CidrList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateHealthCheckRequest")]
pub struct CreateHealthCheckRequest {
    #[serde(rename = "CallerReference")]
    #[serde(default)]
    pub caller_reference: String,
    #[serde(rename = "HealthCheckConfig")]
    #[serde(default)]
    pub health_check_config: HealthCheckConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "HealthCheckConfig")]
pub struct HealthCheckConfig {
    #[serde(rename = "AlarmIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_identifier: Option<AlarmIdentifier>,
    #[serde(rename = "ChildHealthChecks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_health_checks: Option<ChildHealthCheckList>,
    #[serde(rename = "Disabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(rename = "EnableSNI")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_s_n_i: Option<bool>,
    #[serde(rename = "FailureThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_threshold: Option<i32>,
    #[serde(rename = "FullyQualifiedDomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fully_qualified_domain_name: Option<String>,
    #[serde(rename = "HealthThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_threshold: Option<i32>,
    #[serde(rename = "IPAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_p_address: Option<String>,
    #[serde(rename = "InsufficientDataHealthStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insufficient_data_health_status: Option<String>,
    #[serde(rename = "Inverted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inverted: Option<bool>,
    #[serde(rename = "MeasureLatency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measure_latency: Option<bool>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "Regions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<HealthCheckRegionList>,
    #[serde(rename = "RequestInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_interval: Option<i32>,
    #[serde(rename = "ResourcePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_path: Option<String>,
    #[serde(rename = "RoutingControlArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_control_arn: Option<String>,
    #[serde(rename = "SearchString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_string: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetTrafficPolicyResponse")]
pub struct GetTrafficPolicyResponse {
    #[serde(rename = "TrafficPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_policy: Option<TrafficPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetGeoLocationResponse")]
pub struct GetGeoLocationResponse {
    #[serde(rename = "GeoLocationDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_location_details: Option<GeoLocationDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GeoLocationDetails")]
pub struct GeoLocationDetails {
    #[serde(rename = "ContinentCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continent_code: Option<String>,
    #[serde(rename = "ContinentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continent_name: Option<String>,
    #[serde(rename = "CountryCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(rename = "CountryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_name: Option<String>,
    #[serde(rename = "SubdivisionCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdivision_code: Option<String>,
    #[serde(rename = "SubdivisionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdivision_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetHealthCheckStatusResponse")]
pub struct GetHealthCheckStatusResponse {
    #[serde(rename = "HealthCheckObservations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_observations: Option<HealthCheckObservations>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HealthCheckObservations {
    #[serde(
        rename = "HealthCheckObservation",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<HealthCheckObservation>,
}
impl From<Vec<HealthCheckObservation>> for HealthCheckObservations {
    fn from(v: Vec<HealthCheckObservation>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<HealthCheckObservation> for HealthCheckObservations {
    fn from_iter<I: IntoIterator<Item = HealthCheckObservation>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<HealthCheckObservation>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlHealthCheckObservationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<HealthCheckObservation>,
}

impl From<Vec<HealthCheckObservation>> for XmlHealthCheckObservationList {
    fn from(v: Vec<HealthCheckObservation>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<HealthCheckObservation> for XmlHealthCheckObservationList {
    fn from_iter<I: IntoIterator<Item = HealthCheckObservation>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "HealthCheckObservation")]
pub struct HealthCheckObservation {
    #[serde(rename = "IPAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_p_address: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "StatusReport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_report: Option<StatusReport>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StatusReport")]
pub struct StatusReport {
    #[serde(rename = "CheckedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked_time: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AssociateVPCWithHostedZoneResponse")]
pub struct AssociateVPCWithHostedZoneResponse {
    #[serde(rename = "ChangeInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_info: Option<ChangeInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateTrafficPolicyResponse")]
pub struct CreateTrafficPolicyResponse {
    #[serde(rename = "TrafficPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_policy: Option<TrafficPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChangeTagsForResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateKeySigningKeyRequest")]
pub struct CreateKeySigningKeyRequest {
    #[serde(rename = "CallerReference")]
    #[serde(default)]
    pub caller_reference: String,
    #[serde(rename = "HostedZoneId")]
    #[serde(default)]
    pub hosted_zone_id: String,
    #[serde(rename = "KeyManagementServiceArn")]
    #[serde(default)]
    pub key_management_service_arn: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateVPCAssociationAuthorizationResponse")]
pub struct CreateVPCAssociationAuthorizationResponse {
    #[serde(rename = "HostedZoneId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id: Option<String>,
    #[serde(rename = "VPC")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_p_c: Option<VPC>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeactivateKeySigningKeyResponse")]
pub struct DeactivateKeySigningKeyResponse {
    #[serde(rename = "ChangeInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_info: Option<ChangeInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetChangeResponse")]
pub struct GetChangeResponse {
    #[serde(rename = "ChangeInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_info: Option<ChangeInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetHostedZoneCountResponse")]
pub struct GetHostedZoneCountResponse {
    #[serde(rename = "HostedZoneCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetTrafficPolicyInstanceCountResponse")]
pub struct GetTrafficPolicyInstanceCountResponse {
    #[serde(rename = "TrafficPolicyInstanceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_policy_instance_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteHostedZoneRequest")]
pub struct DeleteHostedZoneRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListCidrCollectionsResponse")]
pub struct ListCidrCollectionsResponse {
    #[serde(rename = "CidrCollections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_collections: Option<CollectionSummaries>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CollectionSummaries {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<CollectionSummary>,
}
impl From<Vec<CollectionSummary>> for CollectionSummaries {
    fn from(v: Vec<CollectionSummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<CollectionSummary> for CollectionSummaries {
    fn from_iter<I: IntoIterator<Item = CollectionSummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<CollectionSummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlCollectionSummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<CollectionSummary>,
}

impl From<Vec<CollectionSummary>> for XmlCollectionSummaryList {
    fn from(v: Vec<CollectionSummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<CollectionSummary> for XmlCollectionSummaryList {
    fn from_iter<I: IntoIterator<Item = CollectionSummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CollectionSummary")]
pub struct CollectionSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateHostedZoneCommentRequest")]
pub struct UpdateHostedZoneCommentRequest {
    #[serde(rename = "Comment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListHostedZonesByVPCRequest")]
pub struct ListHostedZonesByVPCRequest {
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "VPCId")]
    #[serde(default)]
    pub v_p_c_id: String,
    #[serde(rename = "VPCRegion")]
    #[serde(default)]
    pub v_p_c_region: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateTrafficPolicyInstanceRequest")]
pub struct UpdateTrafficPolicyInstanceRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "TTL")]
    #[serde(default)]
    pub t_t_l: i64,
    #[serde(rename = "TrafficPolicyId")]
    #[serde(default)]
    pub traffic_policy_id: String,
    #[serde(rename = "TrafficPolicyVersion")]
    #[serde(default)]
    pub traffic_policy_version: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateTrafficPolicyInstanceResponse")]
pub struct UpdateTrafficPolicyInstanceResponse {
    #[serde(rename = "TrafficPolicyInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_policy_instance: Option<TrafficPolicyInstance>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTrafficPolicyInstancesByHostedZoneResponse")]
pub struct ListTrafficPolicyInstancesByHostedZoneResponse {
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "TrafficPolicyInstanceNameMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_policy_instance_name_marker: Option<String>,
    #[serde(rename = "TrafficPolicyInstanceTypeMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_policy_instance_type_marker: Option<String>,
    #[serde(rename = "TrafficPolicyInstances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_policy_instances: Option<TrafficPolicyInstances>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteTrafficPolicyInstanceRequest")]
pub struct DeleteTrafficPolicyInstanceRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteKeySigningKeyRequest")]
pub struct DeleteKeySigningKeyRequest {
    #[serde(rename = "HostedZoneId")]
    #[serde(default)]
    pub hosted_zone_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateHostedZoneCommentResponse")]
pub struct UpdateHostedZoneCommentResponse {
    #[serde(rename = "HostedZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone: Option<HostedZone>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "HostedZone")]
pub struct HostedZone {
    #[serde(rename = "CallerReference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caller_reference: Option<String>,
    #[serde(rename = "Config")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<HostedZoneConfig>,
    #[serde(rename = "Features")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<HostedZoneFeatures>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LinkedService")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_service: Option<LinkedService>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ResourceRecordSetCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_record_set_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "HostedZoneFeatures")]
pub struct HostedZoneFeatures {
    #[serde(rename = "AcceleratedRecoveryStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerated_recovery_status: Option<String>,
    #[serde(rename = "FailureReasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reasons: Option<HostedZoneFailureReasons>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "HostedZoneFailureReasons")]
pub struct HostedZoneFailureReasons {
    #[serde(rename = "AcceleratedRecovery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerated_recovery: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LinkedService")]
pub struct LinkedService {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ServicePrincipal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_principal: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListCidrBlocksResponse")]
pub struct ListCidrBlocksResponse {
    #[serde(rename = "CidrBlocks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_blocks: Option<CidrBlockSummaries>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CidrBlockSummaries {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<CidrBlockSummary>,
}
impl From<Vec<CidrBlockSummary>> for CidrBlockSummaries {
    fn from(v: Vec<CidrBlockSummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<CidrBlockSummary> for CidrBlockSummaries {
    fn from_iter<I: IntoIterator<Item = CidrBlockSummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<CidrBlockSummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlCidrBlockSummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<CidrBlockSummary>,
}

impl From<Vec<CidrBlockSummary>> for XmlCidrBlockSummaryList {
    fn from(v: Vec<CidrBlockSummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<CidrBlockSummary> for XmlCidrBlockSummaryList {
    fn from_iter<I: IntoIterator<Item = CidrBlockSummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CidrBlockSummary")]
pub struct CidrBlockSummary {
    #[serde(rename = "CidrBlock")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_block: Option<String>,
    #[serde(rename = "LocationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateHostedZoneResponse")]
pub struct CreateHostedZoneResponse {
    #[serde(rename = "ChangeInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_info: Option<ChangeInfo>,
    #[serde(rename = "DelegationSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegation_set: Option<DelegationSet>,
    #[serde(rename = "HostedZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone: Option<HostedZone>,
    #[serde(rename = "VPC")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_p_c: Option<VPC>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ChangeTagsForResourceRequest")]
pub struct ChangeTagsForResourceRequest {
    #[serde(rename = "AddTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_tags: Option<TagList>,
    #[serde(rename = "RemoveTagKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_tag_keys: Option<TagKeyList>,
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    pub resource_type: String,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
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
#[serde(rename = "DeleteReusableDelegationSetRequest")]
pub struct DeleteReusableDelegationSetRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteReusableDelegationSetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetHealthCheckResponse")]
pub struct GetHealthCheckResponse {
    #[serde(rename = "HealthCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<HealthCheck>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "HealthCheck")]
pub struct HealthCheck {
    #[serde(rename = "CallerReference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caller_reference: Option<String>,
    #[serde(rename = "CloudWatchAlarmConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_alarm_configuration: Option<CloudWatchAlarmConfiguration>,
    #[serde(rename = "HealthCheckConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_config: Option<HealthCheckConfig>,
    #[serde(rename = "HealthCheckVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_version: Option<i64>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LinkedService")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_service: Option<LinkedService>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CloudWatchAlarmConfiguration")]
pub struct CloudWatchAlarmConfiguration {
    #[serde(rename = "ComparisonOperator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_operator: Option<String>,
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<DimensionList>,
    #[serde(rename = "EvaluationPeriods")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_periods: Option<i32>,
    #[serde(rename = "MetricName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "Period")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    #[serde(rename = "Statistic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic: Option<String>,
    #[serde(rename = "Threshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DimensionList {
    #[serde(rename = "Dimension", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Dimension>,
}
impl From<Vec<Dimension>> for DimensionList {
    fn from(v: Vec<Dimension>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Dimension> for DimensionList {
    fn from_iter<I: IntoIterator<Item = Dimension>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Dimension>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDimensionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Dimension>,
}

impl From<Vec<Dimension>> for XmlDimensionList {
    fn from(v: Vec<Dimension>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Dimension> for XmlDimensionList {
    fn from_iter<I: IntoIterator<Item = Dimension>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Dimension")]
pub struct Dimension {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetHealthCheckCountResponse")]
pub struct GetHealthCheckCountResponse {
    #[serde(rename = "HealthCheckCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetTrafficPolicyRequest")]
pub struct GetTrafficPolicyRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Version")]
    #[serde(default)]
    pub version: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTrafficPolicyVersionsResponse")]
pub struct ListTrafficPolicyVersionsResponse {
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "TrafficPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_policies: Option<TrafficPolicies>,
    #[serde(rename = "TrafficPolicyVersionMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_policy_version_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrafficPolicies {
    #[serde(
        rename = "TrafficPolicy",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<TrafficPolicy>,
}
impl From<Vec<TrafficPolicy>> for TrafficPolicies {
    fn from(v: Vec<TrafficPolicy>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<TrafficPolicy> for TrafficPolicies {
    fn from_iter<I: IntoIterator<Item = TrafficPolicy>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<TrafficPolicy>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTrafficPolicyList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<TrafficPolicy>,
}

impl From<Vec<TrafficPolicy>> for XmlTrafficPolicyList {
    fn from(v: Vec<TrafficPolicy>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<TrafficPolicy> for XmlTrafficPolicyList {
    fn from_iter<I: IntoIterator<Item = TrafficPolicy>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteVPCAssociationAuthorizationRequest")]
pub struct DeleteVPCAssociationAuthorizationRequest {
    #[serde(rename = "HostedZoneId")]
    #[serde(default)]
    pub hosted_zone_id: String,
    #[serde(rename = "VPC")]
    #[serde(default)]
    pub v_p_c: VPC,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetHostedZoneResponse")]
pub struct GetHostedZoneResponse {
    #[serde(rename = "DelegationSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegation_set: Option<DelegationSet>,
    #[serde(rename = "HostedZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone: Option<HostedZone>,
    #[serde(rename = "VPCs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_p_cs: Option<VPCs>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VPCs {
    #[serde(rename = "VPC", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<VPC>,
}
impl From<Vec<VPC>> for VPCs {
    fn from(v: Vec<VPC>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<VPC> for VPCs {
    fn from_iter<I: IntoIterator<Item = VPC>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<VPC>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlVPCList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<VPC>,
}

impl From<Vec<VPC>> for XmlVPCList {
    fn from(v: Vec<VPC>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<VPC> for XmlVPCList {
    fn from_iter<I: IntoIterator<Item = VPC>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateHealthCheckResponse")]
pub struct CreateHealthCheckResponse {
    #[serde(rename = "HealthCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<HealthCheck>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateHealthCheckResponse")]
pub struct UpdateHealthCheckResponse {
    #[serde(rename = "HealthCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<HealthCheck>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteHealthCheckRequest")]
pub struct DeleteHealthCheckRequest {
    #[serde(rename = "HealthCheckId")]
    #[serde(default)]
    pub health_check_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetHostedZoneLimitResponse")]
pub struct GetHostedZoneLimitResponse {
    #[serde(rename = "Count")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<HostedZoneLimit>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "HostedZoneLimit")]
pub struct HostedZoneLimit {
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ChangeResourceRecordSetsResponse")]
pub struct ChangeResourceRecordSetsResponse {
    #[serde(rename = "ChangeInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_info: Option<ChangeInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetReusableDelegationSetLimitRequest")]
pub struct GetReusableDelegationSetLimitRequest {
    #[serde(rename = "DelegationSetId")]
    #[serde(default)]
    pub delegation_set_id: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListHostedZonesByNameRequest")]
pub struct ListHostedZonesByNameRequest {
    #[serde(rename = "DNSName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_n_s_name: Option<String>,
    #[serde(rename = "HostedZoneId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateHostedZoneFeaturesRequest")]
pub struct UpdateHostedZoneFeaturesRequest {
    #[serde(rename = "EnableAcceleratedRecovery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_accelerated_recovery: Option<bool>,
    #[serde(rename = "HostedZoneId")]
    #[serde(default)]
    pub hosted_zone_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateQueryLoggingConfigResponse")]
pub struct CreateQueryLoggingConfigResponse {
    #[serde(rename = "QueryLoggingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_logging_config: Option<QueryLoggingConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "QueryLoggingConfig")]
pub struct QueryLoggingConfig {
    #[serde(rename = "CloudWatchLogsLogGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_log_group_arn: Option<String>,
    #[serde(rename = "HostedZoneId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteKeySigningKeyResponse")]
pub struct DeleteKeySigningKeyResponse {
    #[serde(rename = "ChangeInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_info: Option<ChangeInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCidrCollectionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateTrafficPolicyRequest")]
pub struct CreateTrafficPolicyRequest {
    #[serde(rename = "Comment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "Document")]
    #[serde(default)]
    pub document: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetHealthCheckLastFailureReasonResponse")]
pub struct GetHealthCheckLastFailureReasonResponse {
    #[serde(rename = "HealthCheckObservations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_observations: Option<HealthCheckObservations>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTrafficPolicyInstancesByPolicyRequest")]
pub struct ListTrafficPolicyInstancesByPolicyRequest {
    #[serde(rename = "HostedZoneIdMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id_marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "TrafficPolicyId")]
    #[serde(default)]
    pub traffic_policy_id: String,
    #[serde(rename = "TrafficPolicyInstanceNameMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_policy_instance_name_marker: Option<String>,
    #[serde(rename = "TrafficPolicyInstanceTypeMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_policy_instance_type_marker: Option<String>,
    #[serde(rename = "TrafficPolicyVersion")]
    #[serde(default)]
    pub traffic_policy_version: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListVPCAssociationAuthorizationsRequest")]
pub struct ListVPCAssociationAuthorizationsRequest {
    #[serde(rename = "HostedZoneId")]
    #[serde(default)]
    pub hosted_zone_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateVPCAssociationAuthorizationRequest")]
pub struct CreateVPCAssociationAuthorizationRequest {
    #[serde(rename = "HostedZoneId")]
    #[serde(default)]
    pub hosted_zone_id: String,
    #[serde(rename = "VPC")]
    #[serde(default)]
    pub v_p_c: VPC,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListHostedZonesRequest")]
pub struct ListHostedZonesRequest {
    #[serde(rename = "DelegationSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegation_set_id: Option<String>,
    #[serde(rename = "HostedZoneType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_type: Option<String>,
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
#[serde(rename = "GetHealthCheckRequest")]
pub struct GetHealthCheckRequest {
    #[serde(rename = "HealthCheckId")]
    #[serde(default)]
    pub health_check_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTagsForResourceResponse")]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "ResourceTagSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tag_set: Option<ResourceTagSet>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResourceTagSet")]
pub struct ResourceTagSet {
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetChangeRequest")]
pub struct GetChangeRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EnableHostedZoneDNSSECResponse")]
pub struct EnableHostedZoneDNSSECResponse {
    #[serde(rename = "ChangeInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_info: Option<ChangeInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetHealthCheckCountRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetGeoLocationRequest")]
pub struct GetGeoLocationRequest {
    #[serde(rename = "ContinentCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continent_code: Option<String>,
    #[serde(rename = "CountryCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(rename = "SubdivisionCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdivision_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListHostedZonesByNameResponse")]
pub struct ListHostedZonesByNameResponse {
    #[serde(rename = "DNSName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_n_s_name: Option<String>,
    #[serde(rename = "HostedZoneId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id: Option<String>,
    #[serde(rename = "HostedZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zones: Option<HostedZones>,
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "NextDNSName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_d_n_s_name: Option<String>,
    #[serde(rename = "NextHostedZoneId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_hosted_zone_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HostedZones {
    #[serde(rename = "HostedZone", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<HostedZone>,
}
impl From<Vec<HostedZone>> for HostedZones {
    fn from(v: Vec<HostedZone>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<HostedZone> for HostedZones {
    fn from_iter<I: IntoIterator<Item = HostedZone>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<HostedZone>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlHostedZoneList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<HostedZone>,
}

impl From<Vec<HostedZone>> for XmlHostedZoneList {
    fn from(v: Vec<HostedZone>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<HostedZone> for XmlHostedZoneList {
    fn from_iter<I: IntoIterator<Item = HostedZone>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListCidrLocationsRequest")]
pub struct ListCidrLocationsRequest {
    #[serde(rename = "CollectionId")]
    #[serde(default)]
    pub collection_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateHostedZoneFeaturesResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListReusableDelegationSetsRequest")]
pub struct ListReusableDelegationSetsRequest {
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
#[serde(rename = "GetReusableDelegationSetLimitResponse")]
pub struct GetReusableDelegationSetLimitResponse {
    #[serde(rename = "Count")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<ReusableDelegationSetLimit>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ReusableDelegationSetLimit")]
pub struct ReusableDelegationSetLimit {
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTrafficPolicyInstancesRequest")]
pub struct ListTrafficPolicyInstancesRequest {
    #[serde(rename = "HostedZoneIdMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id_marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "TrafficPolicyInstanceNameMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_policy_instance_name_marker: Option<String>,
    #[serde(rename = "TrafficPolicyInstanceTypeMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_policy_instance_type_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetQueryLoggingConfigRequest")]
pub struct GetQueryLoggingConfigRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateQueryLoggingConfigRequest")]
pub struct CreateQueryLoggingConfigRequest {
    #[serde(rename = "CloudWatchLogsLogGroupArn")]
    #[serde(default)]
    pub cloud_watch_logs_log_group_arn: String,
    #[serde(rename = "HostedZoneId")]
    #[serde(default)]
    pub hosted_zone_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTrafficPolicyInstancesResponse")]
pub struct ListTrafficPolicyInstancesResponse {
    #[serde(rename = "HostedZoneIdMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id_marker: Option<String>,
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "TrafficPolicyInstanceNameMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_policy_instance_name_marker: Option<String>,
    #[serde(rename = "TrafficPolicyInstanceTypeMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_policy_instance_type_marker: Option<String>,
    #[serde(rename = "TrafficPolicyInstances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_policy_instances: Option<TrafficPolicyInstances>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListResourceRecordSetsRequest")]
pub struct ListResourceRecordSetsRequest {
    #[serde(rename = "HostedZoneId")]
    #[serde(default)]
    pub hosted_zone_id: String,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "StartRecordIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_record_identifier: Option<String>,
    #[serde(rename = "StartRecordName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_record_name: Option<String>,
    #[serde(rename = "StartRecordType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_record_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetHealthCheckStatusRequest")]
pub struct GetHealthCheckStatusRequest {
    #[serde(rename = "HealthCheckId")]
    #[serde(default)]
    pub health_check_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ActivateKeySigningKeyRequest")]
pub struct ActivateKeySigningKeyRequest {
    #[serde(rename = "HostedZoneId")]
    #[serde(default)]
    pub hosted_zone_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ActivateKeySigningKeyResponse")]
pub struct ActivateKeySigningKeyResponse {
    #[serde(rename = "ChangeInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_info: Option<ChangeInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TestDNSAnswerResponse")]
pub struct TestDNSAnswerResponse {
    #[serde(rename = "Nameserver")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nameserver: Option<String>,
    #[serde(rename = "Protocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "RecordData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_data: Option<RecordData>,
    #[serde(rename = "RecordName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_name: Option<String>,
    #[serde(rename = "RecordType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_type: Option<String>,
    #[serde(rename = "ResponseCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecordData {
    #[serde(
        rename = "RecordDataEntry",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for RecordData {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for RecordData {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListHostedZonesResponse")]
pub struct ListHostedZonesResponse {
    #[serde(rename = "HostedZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zones: Option<HostedZones>,
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
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
#[serde(rename = "CreateKeySigningKeyResponse")]
pub struct CreateKeySigningKeyResponse {
    #[serde(rename = "ChangeInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_info: Option<ChangeInfo>,
    #[serde(rename = "KeySigningKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_signing_key: Option<KeySigningKey>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ChangeCidrCollectionResponse")]
pub struct ChangeCidrCollectionResponse {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListResourceRecordSetsResponse")]
pub struct ListResourceRecordSetsResponse {
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "NextRecordIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_record_identifier: Option<String>,
    #[serde(rename = "NextRecordName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_record_name: Option<String>,
    #[serde(rename = "NextRecordType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_record_type: Option<String>,
    #[serde(rename = "ResourceRecordSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_record_sets: Option<ResourceRecordSets>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceRecordSets {
    #[serde(
        rename = "ResourceRecordSet",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ResourceRecordSet>,
}
impl From<Vec<ResourceRecordSet>> for ResourceRecordSets {
    fn from(v: Vec<ResourceRecordSet>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ResourceRecordSet> for ResourceRecordSets {
    fn from_iter<I: IntoIterator<Item = ResourceRecordSet>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ResourceRecordSet>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlResourceRecordSetList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ResourceRecordSet>,
}

impl From<Vec<ResourceRecordSet>> for XmlResourceRecordSetList {
    fn from(v: Vec<ResourceRecordSet>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ResourceRecordSet> for XmlResourceRecordSetList {
    fn from_iter<I: IntoIterator<Item = ResourceRecordSet>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AssociateVPCWithHostedZoneRequest")]
pub struct AssociateVPCWithHostedZoneRequest {
    #[serde(rename = "Comment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "HostedZoneId")]
    #[serde(default)]
    pub hosted_zone_id: String,
    #[serde(rename = "VPC")]
    #[serde(default)]
    pub v_p_c: VPC,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteHostedZoneResponse")]
pub struct DeleteHostedZoneResponse {
    #[serde(rename = "ChangeInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_info: Option<ChangeInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteQueryLoggingConfigResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetAccountLimitRequest")]
pub struct GetAccountLimitRequest {
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListGeoLocationsRequest")]
pub struct ListGeoLocationsRequest {
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "StartContinentCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_continent_code: Option<String>,
    #[serde(rename = "StartCountryCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_country_code: Option<String>,
    #[serde(rename = "StartSubdivisionCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_subdivision_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListHealthChecksResponse")]
pub struct ListHealthChecksResponse {
    #[serde(rename = "HealthChecks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_checks: Option<HealthChecks>,
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
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
pub struct HealthChecks {
    #[serde(rename = "HealthCheck", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<HealthCheck>,
}
impl From<Vec<HealthCheck>> for HealthChecks {
    fn from(v: Vec<HealthCheck>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<HealthCheck> for HealthChecks {
    fn from_iter<I: IntoIterator<Item = HealthCheck>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<HealthCheck>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlHealthCheckList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<HealthCheck>,
}

impl From<Vec<HealthCheck>> for XmlHealthCheckList {
    fn from(v: Vec<HealthCheck>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<HealthCheck> for XmlHealthCheckList {
    fn from_iter<I: IntoIterator<Item = HealthCheck>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTrafficPolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListGeoLocationsResponse")]
pub struct ListGeoLocationsResponse {
    #[serde(rename = "GeoLocationDetailsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_location_details_list: Option<GeoLocationDetailsList>,
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "NextContinentCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_continent_code: Option<String>,
    #[serde(rename = "NextCountryCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_country_code: Option<String>,
    #[serde(rename = "NextSubdivisionCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_subdivision_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeoLocationDetailsList {
    #[serde(
        rename = "GeoLocationDetails",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<GeoLocationDetails>,
}
impl From<Vec<GeoLocationDetails>> for GeoLocationDetailsList {
    fn from(v: Vec<GeoLocationDetails>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<GeoLocationDetails> for GeoLocationDetailsList {
    fn from_iter<I: IntoIterator<Item = GeoLocationDetails>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<GeoLocationDetails>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlGeoLocationDetailsList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<GeoLocationDetails>,
}

impl From<Vec<GeoLocationDetails>> for XmlGeoLocationDetailsList {
    fn from(v: Vec<GeoLocationDetails>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<GeoLocationDetails> for XmlGeoLocationDetailsList {
    fn from_iter<I: IntoIterator<Item = GeoLocationDetails>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTagsForResourcesResponse")]
pub struct ListTagsForResourcesResponse {
    #[serde(rename = "ResourceTagSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tag_sets: Option<ResourceTagSetList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceTagSetList {
    #[serde(
        rename = "ResourceTagSet",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ResourceTagSet>,
}
impl From<Vec<ResourceTagSet>> for ResourceTagSetList {
    fn from(v: Vec<ResourceTagSet>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ResourceTagSet> for ResourceTagSetList {
    fn from_iter<I: IntoIterator<Item = ResourceTagSet>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ResourceTagSet>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlResourceTagSetList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ResourceTagSet>,
}

impl From<Vec<ResourceTagSet>> for XmlResourceTagSetList {
    fn from(v: Vec<ResourceTagSet>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ResourceTagSet> for XmlResourceTagSetList {
    fn from_iter<I: IntoIterator<Item = ResourceTagSet>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetHostedZoneCountRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTagsForResourcesRequest")]
pub struct ListTagsForResourcesRequest {
    #[serde(rename = "ResourceIds")]
    #[serde(default)]
    pub resource_ids: TagResourceIdList,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    pub resource_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceIdList {
    #[serde(rename = "ResourceId", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for TagResourceIdList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for TagResourceIdList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetReusableDelegationSetResponse")]
pub struct GetReusableDelegationSetResponse {
    #[serde(rename = "DelegationSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegation_set: Option<DelegationSet>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetQueryLoggingConfigResponse")]
pub struct GetQueryLoggingConfigResponse {
    #[serde(rename = "QueryLoggingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_logging_config: Option<QueryLoggingConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetTrafficPolicyInstanceResponse")]
pub struct GetTrafficPolicyInstanceResponse {
    #[serde(rename = "TrafficPolicyInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_policy_instance: Option<TrafficPolicyInstance>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteHealthCheckResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DisassociateVPCFromHostedZoneRequest")]
pub struct DisassociateVPCFromHostedZoneRequest {
    #[serde(rename = "Comment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "HostedZoneId")]
    #[serde(default)]
    pub hosted_zone_id: String,
    #[serde(rename = "VPC")]
    #[serde(default)]
    pub v_p_c: VPC,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateTrafficPolicyInstanceRequest")]
pub struct CreateTrafficPolicyInstanceRequest {
    #[serde(rename = "HostedZoneId")]
    #[serde(default)]
    pub hosted_zone_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "TTL")]
    #[serde(default)]
    pub t_t_l: i64,
    #[serde(rename = "TrafficPolicyId")]
    #[serde(default)]
    pub traffic_policy_id: String,
    #[serde(rename = "TrafficPolicyVersion")]
    #[serde(default)]
    pub traffic_policy_version: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetAccountLimitResponse")]
pub struct GetAccountLimitResponse {
    #[serde(rename = "Count")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<AccountLimit>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AccountLimit")]
pub struct AccountLimit {
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListVPCAssociationAuthorizationsResponse")]
pub struct ListVPCAssociationAuthorizationsResponse {
    #[serde(rename = "HostedZoneId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "VPCs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_p_cs: Option<VPCs>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListHostedZonesByVPCResponse")]
pub struct ListHostedZonesByVPCResponse {
    #[serde(rename = "HostedZoneSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_summaries: Option<HostedZoneSummaries>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HostedZoneSummaries {
    #[serde(
        rename = "HostedZoneSummary",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<HostedZoneSummary>,
}
impl From<Vec<HostedZoneSummary>> for HostedZoneSummaries {
    fn from(v: Vec<HostedZoneSummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<HostedZoneSummary> for HostedZoneSummaries {
    fn from_iter<I: IntoIterator<Item = HostedZoneSummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<HostedZoneSummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlHostedZoneSummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<HostedZoneSummary>,
}

impl From<Vec<HostedZoneSummary>> for XmlHostedZoneSummaryList {
    fn from(v: Vec<HostedZoneSummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<HostedZoneSummary> for XmlHostedZoneSummaryList {
    fn from_iter<I: IntoIterator<Item = HostedZoneSummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "HostedZoneSummary")]
pub struct HostedZoneSummary {
    #[serde(rename = "HostedZoneId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Owner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<HostedZoneOwner>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "HostedZoneOwner")]
pub struct HostedZoneOwner {
    #[serde(rename = "OwningAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owning_account: Option<String>,
    #[serde(rename = "OwningService")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owning_service: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateTrafficPolicyCommentResponse")]
pub struct UpdateTrafficPolicyCommentResponse {
    #[serde(rename = "TrafficPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_policy: Option<TrafficPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateTrafficPolicyInstanceResponse")]
pub struct CreateTrafficPolicyInstanceResponse {
    #[serde(rename = "TrafficPolicyInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_policy_instance: Option<TrafficPolicyInstance>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListReusableDelegationSetsResponse")]
pub struct ListReusableDelegationSetsResponse {
    #[serde(rename = "DelegationSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegation_sets: Option<DelegationSets>,
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
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
pub struct DelegationSets {
    #[serde(
        rename = "DelegationSet",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DelegationSet>,
}
impl From<Vec<DelegationSet>> for DelegationSets {
    fn from(v: Vec<DelegationSet>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DelegationSet> for DelegationSets {
    fn from_iter<I: IntoIterator<Item = DelegationSet>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DelegationSet>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDelegationSetList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DelegationSet>,
}

impl From<Vec<DelegationSet>> for XmlDelegationSetList {
    fn from(v: Vec<DelegationSet>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DelegationSet> for XmlDelegationSetList {
    fn from_iter<I: IntoIterator<Item = DelegationSet>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListCidrCollectionsRequest")]
pub struct ListCidrCollectionsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTagsForResourceRequest")]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceId")]
    #[serde(default)]
    pub resource_id: String,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    pub resource_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DisableHostedZoneDNSSECRequest")]
pub struct DisableHostedZoneDNSSECRequest {
    #[serde(rename = "HostedZoneId")]
    #[serde(default)]
    pub hosted_zone_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EnableHostedZoneDNSSECRequest")]
pub struct EnableHostedZoneDNSSECRequest {
    #[serde(rename = "HostedZoneId")]
    #[serde(default)]
    pub hosted_zone_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTrafficPolicyInstanceCountRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListHealthChecksRequest")]
pub struct ListHealthChecksRequest {
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
#[serde(rename = "ListQueryLoggingConfigsResponse")]
pub struct ListQueryLoggingConfigsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "QueryLoggingConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_logging_configs: Option<QueryLoggingConfigs>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryLoggingConfigs {
    #[serde(
        rename = "QueryLoggingConfig",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<QueryLoggingConfig>,
}
impl From<Vec<QueryLoggingConfig>> for QueryLoggingConfigs {
    fn from(v: Vec<QueryLoggingConfig>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<QueryLoggingConfig> for QueryLoggingConfigs {
    fn from_iter<I: IntoIterator<Item = QueryLoggingConfig>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<QueryLoggingConfig>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlQueryLoggingConfigList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<QueryLoggingConfig>,
}

impl From<Vec<QueryLoggingConfig>> for XmlQueryLoggingConfigList {
    fn from(v: Vec<QueryLoggingConfig>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<QueryLoggingConfig> for XmlQueryLoggingConfigList {
    fn from_iter<I: IntoIterator<Item = QueryLoggingConfig>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetCheckerIpRangesResponse")]
pub struct GetCheckerIpRangesResponse {
    #[serde(rename = "CheckerIpRanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checker_ip_ranges: Option<CheckerIpRanges>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CheckerIpRanges {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for CheckerIpRanges {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for CheckerIpRanges {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}
