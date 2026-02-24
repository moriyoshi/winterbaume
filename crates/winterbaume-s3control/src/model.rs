//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-s3-control

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetMultiRegionAccessPointRequest")]
pub struct GetMultiRegionAccessPointRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutAccessGrantsInstanceResourcePolicyRequest")]
pub struct PutAccessGrantsInstanceResourcePolicyRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Organization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(rename = "Policy")]
    #[serde(default)]
    pub policy: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListRegionalBucketsResult")]
pub struct ListRegionalBucketsResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RegionalBucketList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regional_bucket_list: Option<RegionalBucketList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegionalBucketList {
    #[serde(
        rename = "RegionalBucket",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<RegionalBucket>,
}
impl From<Vec<RegionalBucket>> for RegionalBucketList {
    fn from(v: Vec<RegionalBucket>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<RegionalBucket> for RegionalBucketList {
    fn from_iter<I: IntoIterator<Item = RegionalBucket>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<RegionalBucket>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlRegionalBucketList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<RegionalBucket>,
}

impl From<Vec<RegionalBucket>> for XmlRegionalBucketList {
    fn from(v: Vec<RegionalBucket>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<RegionalBucket> for XmlRegionalBucketList {
    fn from_iter<I: IntoIterator<Item = RegionalBucket>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RegionalBucket")]
pub struct RegionalBucket {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(rename = "BucketArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_arn: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "OutpostId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_id: Option<String>,
    #[serde(rename = "PublicAccessBlockEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access_block_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketPolicyRequest")]
pub struct GetBucketPolicyRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetMultiRegionAccessPointPolicyStatusRequest")]
pub struct GetMultiRegionAccessPointPolicyStatusRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetMultiRegionAccessPointPolicyResult")]
pub struct GetMultiRegionAccessPointPolicyResult {
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<MultiRegionAccessPointPolicyDocument>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MultiRegionAccessPointPolicyDocument")]
pub struct MultiRegionAccessPointPolicyDocument {
    #[serde(rename = "Established")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub established: Option<EstablishedMultiRegionAccessPointPolicy>,
    #[serde(rename = "Proposed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposed: Option<ProposedMultiRegionAccessPointPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EstablishedMultiRegionAccessPointPolicy")]
pub struct EstablishedMultiRegionAccessPointPolicy {
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ProposedMultiRegionAccessPointPolicy")]
pub struct ProposedMultiRegionAccessPointPolicy {
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListStorageLensGroupsRequest")]
pub struct ListStorageLensGroupsRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ReplicationConfiguration")]
pub struct ReplicationConfiguration {
    #[serde(rename = "Role")]
    #[serde(default)]
    pub role: String,
    #[serde(rename = "Rules")]
    #[serde(default)]
    pub rules: ReplicationRules,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationRules {
    #[serde(rename = "Rule", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ReplicationRule>,
}
impl From<Vec<ReplicationRule>> for ReplicationRules {
    fn from(v: Vec<ReplicationRule>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ReplicationRule> for ReplicationRules {
    fn from_iter<I: IntoIterator<Item = ReplicationRule>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ReplicationRule>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlReplicationRuleList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ReplicationRule>,
}

impl From<Vec<ReplicationRule>> for XmlReplicationRuleList {
    fn from(v: Vec<ReplicationRule>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ReplicationRule> for XmlReplicationRuleList {
    fn from_iter<I: IntoIterator<Item = ReplicationRule>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ReplicationRule")]
pub struct ReplicationRule {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "DeleteMarkerReplication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_marker_replication: Option<DeleteMarkerReplication>,
    #[serde(rename = "Destination")]
    #[serde(default)]
    pub destination: Destination,
    #[serde(rename = "ExistingObjectReplication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub existing_object_replication: Option<ExistingObjectReplication>,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ReplicationRuleFilter>,
    #[serde(rename = "ID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_d: Option<String>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "SourceSelectionCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_selection_criteria: Option<SourceSelectionCriteria>,
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteMarkerReplication")]
pub struct DeleteMarkerReplication {
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Destination")]
pub struct Destination {
    #[serde(rename = "AccessControlTranslation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_translation: Option<AccessControlTranslation>,
    #[serde(rename = "Account")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "Metrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Metrics>,
    #[serde(rename = "ReplicationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_time: Option<ReplicationTime>,
    #[serde(rename = "StorageClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AccessControlTranslation")]
pub struct AccessControlTranslation {
    #[serde(rename = "Owner")]
    #[serde(default)]
    pub owner: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EncryptionConfiguration")]
pub struct EncryptionConfiguration {
    #[serde(rename = "ReplicaKmsKeyID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_kms_key_i_d: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Metrics")]
pub struct Metrics {
    #[serde(rename = "EventThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_threshold: Option<ReplicationTimeValue>,
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ReplicationTimeValue")]
pub struct ReplicationTimeValue {
    #[serde(rename = "Minutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minutes: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ReplicationTime")]
pub struct ReplicationTime {
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
    #[serde(rename = "Time")]
    #[serde(default)]
    pub time: ReplicationTimeValue,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ExistingObjectReplication")]
pub struct ExistingObjectReplication {
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ReplicationRuleFilter")]
pub struct ReplicationRuleFilter {
    #[serde(rename = "And")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and: Option<ReplicationRuleAndOperator>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "Tag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<S3Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ReplicationRuleAndOperator")]
pub struct ReplicationRuleAndOperator {
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<S3TagSet>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3TagSet {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<S3Tag>,
}
impl From<Vec<S3Tag>> for S3TagSet {
    fn from(v: Vec<S3Tag>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<S3Tag> for S3TagSet {
    fn from_iter<I: IntoIterator<Item = S3Tag>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<S3Tag>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlS3TagList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<S3Tag>,
}

impl From<Vec<S3Tag>> for XmlS3TagList {
    fn from(v: Vec<S3Tag>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<S3Tag> for XmlS3TagList {
    fn from_iter<I: IntoIterator<Item = S3Tag>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "S3Tag")]
pub struct S3Tag {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SourceSelectionCriteria")]
pub struct SourceSelectionCriteria {
    #[serde(rename = "ReplicaModifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_modifications: Option<ReplicaModifications>,
    #[serde(rename = "SseKmsEncryptedObjects")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sse_kms_encrypted_objects: Option<SseKmsEncryptedObjects>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ReplicaModifications")]
pub struct ReplicaModifications {
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SseKmsEncryptedObjects")]
pub struct SseKmsEncryptedObjects {
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateBucketRequest")]
pub struct CreateBucketRequest {
    #[serde(rename = "ACL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_c_l: Option<String>,
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "CreateBucketConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_bucket_configuration: Option<CreateBucketConfiguration>,
    #[serde(rename = "GrantFullControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_full_control: Option<String>,
    #[serde(rename = "GrantRead")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_read: Option<String>,
    #[serde(rename = "GrantReadACP")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_read_a_c_p: Option<String>,
    #[serde(rename = "GrantWrite")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_write: Option<String>,
    #[serde(rename = "GrantWriteACP")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_write_a_c_p: Option<String>,
    #[serde(rename = "ObjectLockEnabledForBucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_lock_enabled_for_bucket: Option<bool>,
    #[serde(rename = "OutpostId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateBucketConfiguration")]
pub struct CreateBucketConfiguration {
    #[serde(rename = "LocationConstraint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_constraint: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListMultiRegionAccessPointsRequest")]
pub struct ListMultiRegionAccessPointsRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
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
#[serde(rename = "DeleteMultiRegionAccessPointRequest")]
pub struct DeleteMultiRegionAccessPointRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    pub client_token: String,
    #[serde(rename = "Details")]
    #[serde(default)]
    pub details: DeleteMultiRegionAccessPointInput,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteMultiRegionAccessPointInput")]
pub struct DeleteMultiRegionAccessPointInput {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListAccessPointsRequest")]
pub struct ListAccessPointsRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(rename = "DataSourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,
    #[serde(rename = "DataSourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_type: Option<String>,
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
#[serde(rename = "GetAccessPointPolicyStatusForObjectLambdaRequest")]
pub struct GetAccessPointPolicyStatusForObjectLambdaRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetAccessGrantResult")]
pub struct GetAccessGrantResult {
    #[serde(rename = "AccessGrantArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_grant_arn: Option<String>,
    #[serde(rename = "AccessGrantId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_grant_id: Option<String>,
    #[serde(rename = "AccessGrantsLocationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_grants_location_configuration: Option<AccessGrantsLocationConfiguration>,
    #[serde(rename = "AccessGrantsLocationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_grants_location_id: Option<String>,
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "GrantScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_scope: Option<String>,
    #[serde(rename = "Grantee")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grantee: Option<Grantee>,
    #[serde(rename = "Permission")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AccessGrantsLocationConfiguration")]
pub struct AccessGrantsLocationConfiguration {
    #[serde(rename = "S3SubPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_sub_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Grantee")]
pub struct Grantee {
    #[serde(rename = "GranteeIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grantee_identifier: Option<String>,
    #[serde(rename = "GranteeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grantee_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListAccessGrantsLocationsRequest")]
pub struct ListAccessGrantsLocationsRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "LocationScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_scope: Option<String>,
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
#[serde(rename = "AssociateAccessGrantsIdentityCenterRequest")]
pub struct AssociateAccessGrantsIdentityCenterRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "IdentityCenterArn")]
    #[serde(default)]
    pub identity_center_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetAccessGrantsInstanceForPrefixRequest")]
pub struct GetAccessGrantsInstanceForPrefixRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "S3Prefix")]
    #[serde(default)]
    pub s3_prefix: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketReplicationResult")]
pub struct GetBucketReplicationResult {
    #[serde(rename = "ReplicationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_configuration: Option<ReplicationConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateJobRequest")]
pub struct CreateJobRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    pub client_request_token: String,
    #[serde(rename = "ConfirmationRequired")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_required: Option<bool>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Manifest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest: Option<JobManifest>,
    #[serde(rename = "ManifestGenerator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_generator: Option<JobManifestGenerator>,
    #[serde(rename = "Operation")]
    #[serde(default)]
    pub operation: JobOperation,
    #[serde(rename = "Priority")]
    #[serde(default)]
    pub priority: i32,
    #[serde(rename = "Report")]
    #[serde(default)]
    pub report: JobReport,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<S3TagSet>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "JobManifest")]
pub struct JobManifest {
    #[serde(rename = "Location")]
    #[serde(default)]
    pub location: JobManifestLocation,
    #[serde(rename = "Spec")]
    #[serde(default)]
    pub spec: JobManifestSpec,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "JobManifestLocation")]
pub struct JobManifestLocation {
    #[serde(rename = "ETag")]
    #[serde(default)]
    pub e_tag: String,
    #[serde(rename = "ObjectArn")]
    #[serde(default)]
    pub object_arn: String,
    #[serde(rename = "ObjectVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "JobManifestSpec")]
pub struct JobManifestSpec {
    #[serde(rename = "Fields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<JobManifestFieldList>,
    #[serde(rename = "Format")]
    #[serde(default)]
    pub format: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobManifestFieldList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for JobManifestFieldList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for JobManifestFieldList {
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
#[serde(rename = "JobManifestGenerator")]
pub struct JobManifestGenerator {
    #[serde(rename = "S3JobManifestGenerator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_job_manifest_generator: Option<S3JobManifestGenerator>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "S3JobManifestGenerator")]
pub struct S3JobManifestGenerator {
    #[serde(rename = "EnableManifestOutput")]
    #[serde(default)]
    pub enable_manifest_output: bool,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<JobManifestGeneratorFilter>,
    #[serde(rename = "ManifestOutputLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_output_location: Option<S3ManifestOutputLocation>,
    #[serde(rename = "SourceBucket")]
    #[serde(default)]
    pub source_bucket: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "JobManifestGeneratorFilter")]
pub struct JobManifestGeneratorFilter {
    #[serde(rename = "CreatedAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    #[serde(rename = "CreatedBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    #[serde(rename = "EligibleForReplication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eligible_for_replication: Option<bool>,
    #[serde(rename = "KeyNameConstraint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_name_constraint: Option<KeyNameConstraint>,
    #[serde(rename = "MatchAnyObjectEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_any_object_encryption: Option<ObjectEncryptionFilterList>,
    #[serde(rename = "MatchAnyStorageClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_any_storage_class: Option<StorageClassList>,
    #[serde(rename = "ObjectReplicationStatuses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_replication_statuses: Option<ReplicationStatusFilterList>,
    #[serde(rename = "ObjectSizeGreaterThanBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_size_greater_than_bytes: Option<i64>,
    #[serde(rename = "ObjectSizeLessThanBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_size_less_than_bytes: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "KeyNameConstraint")]
pub struct KeyNameConstraint {
    #[serde(rename = "MatchAnyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_any_prefix: Option<NonEmptyMaxLength1024StringList>,
    #[serde(rename = "MatchAnySubstring")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_any_substring: Option<NonEmptyMaxLength1024StringList>,
    #[serde(rename = "MatchAnySuffix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_any_suffix: Option<NonEmptyMaxLength1024StringList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NonEmptyMaxLength1024StringList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for NonEmptyMaxLength1024StringList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for NonEmptyMaxLength1024StringList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ObjectEncryptionFilterList {
    #[serde(
        rename = "ObjectEncryption",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ObjectEncryptionFilter>,
}
impl From<Vec<ObjectEncryptionFilter>> for ObjectEncryptionFilterList {
    fn from(v: Vec<ObjectEncryptionFilter>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ObjectEncryptionFilter> for ObjectEncryptionFilterList {
    fn from_iter<I: IntoIterator<Item = ObjectEncryptionFilter>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ObjectEncryptionFilter>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlObjectEncryptionFilterList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ObjectEncryptionFilter>,
}

impl From<Vec<ObjectEncryptionFilter>> for XmlObjectEncryptionFilterList {
    fn from(v: Vec<ObjectEncryptionFilter>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ObjectEncryptionFilter> for XmlObjectEncryptionFilterList {
    fn from_iter<I: IntoIterator<Item = ObjectEncryptionFilter>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ObjectEncryptionFilter")]
pub struct ObjectEncryptionFilter {
    #[serde(rename = "DSSE-KMS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_s_s_e_k_m_s: Option<DSSEKMSFilter>,
    #[serde(rename = "NOT-SSE")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_o_t_s_s_e: Option<NotSSEFilter>,
    #[serde(rename = "SSE-C")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_c: Option<SSECFilter>,
    #[serde(rename = "SSE-KMS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_k_m_s: Option<SSEKMSFilter>,
    #[serde(rename = "SSE-S3")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_s3: Option<SSES3Filter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DSSEKMSFilter")]
pub struct DSSEKMSFilter {
    #[serde(rename = "KmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NotSSEFilter {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SSECFilter {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SSEKMSFilter")]
pub struct SSEKMSFilter {
    #[serde(rename = "BucketKeyEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_key_enabled: Option<bool>,
    #[serde(rename = "KmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SSES3Filter {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StorageClassList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for StorageClassList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for StorageClassList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationStatusFilterList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ReplicationStatusFilterList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ReplicationStatusFilterList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "S3ManifestOutputLocation")]
pub struct S3ManifestOutputLocation {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedManifestBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_manifest_bucket_owner: Option<String>,
    #[serde(rename = "ManifestEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_encryption: Option<GeneratedManifestEncryption>,
    #[serde(rename = "ManifestFormat")]
    #[serde(default)]
    pub manifest_format: String,
    #[serde(rename = "ManifestPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GeneratedManifestEncryption")]
pub struct GeneratedManifestEncryption {
    #[serde(rename = "SSE-KMS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_k_m_s: Option<SSEKMSEncryption>,
    #[serde(rename = "SSE-S3")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_s3: Option<SSES3Encryption>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SSE-KMS")]
pub struct SSEKMSEncryption {
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SSES3Encryption {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "JobOperation")]
pub struct JobOperation {
    #[serde(rename = "LambdaInvoke")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_invoke: Option<LambdaInvokeOperation>,
    #[serde(rename = "S3ComputeObjectChecksum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_compute_object_checksum: Option<S3ComputeObjectChecksumOperation>,
    #[serde(rename = "S3DeleteObjectTagging")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_delete_object_tagging: Option<S3DeleteObjectTaggingOperation>,
    #[serde(rename = "S3InitiateRestoreObject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_initiate_restore_object: Option<S3InitiateRestoreObjectOperation>,
    #[serde(rename = "S3PutObjectAcl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_put_object_acl: Option<S3SetObjectAclOperation>,
    #[serde(rename = "S3PutObjectCopy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_put_object_copy: Option<S3CopyObjectOperation>,
    #[serde(rename = "S3PutObjectLegalHold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_put_object_legal_hold: Option<S3SetObjectLegalHoldOperation>,
    #[serde(rename = "S3PutObjectRetention")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_put_object_retention: Option<S3SetObjectRetentionOperation>,
    #[serde(rename = "S3PutObjectTagging")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_put_object_tagging: Option<S3SetObjectTaggingOperation>,
    #[serde(rename = "S3ReplicateObject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_replicate_object: Option<S3ReplicateObjectOperation>,
    #[serde(rename = "S3UpdateObjectEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_update_object_encryption: Option<S3UpdateObjectEncryptionOperation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LambdaInvokeOperation")]
pub struct LambdaInvokeOperation {
    #[serde(rename = "FunctionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arn: Option<String>,
    #[serde(rename = "InvocationSchemaVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_schema_version: Option<String>,
    #[serde(rename = "UserArguments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_arguments: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "S3ComputeObjectChecksumOperation")]
pub struct S3ComputeObjectChecksumOperation {
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
    #[serde(rename = "ChecksumType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3DeleteObjectTaggingOperation {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "S3InitiateRestoreObjectOperation")]
pub struct S3InitiateRestoreObjectOperation {
    #[serde(rename = "ExpirationInDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_in_days: Option<i32>,
    #[serde(rename = "GlacierJobTier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glacier_job_tier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "S3SetObjectAclOperation")]
pub struct S3SetObjectAclOperation {
    #[serde(rename = "AccessControlPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_policy: Option<S3AccessControlPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "S3AccessControlPolicy")]
pub struct S3AccessControlPolicy {
    #[serde(rename = "AccessControlList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_list: Option<S3AccessControlList>,
    #[serde(rename = "CannedAccessControlList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canned_access_control_list: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "S3AccessControlList")]
pub struct S3AccessControlList {
    #[serde(rename = "Grants")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grants: Option<S3GrantList>,
    #[serde(rename = "Owner")]
    #[serde(default)]
    pub owner: S3ObjectOwner,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3GrantList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<S3Grant>,
}
impl From<Vec<S3Grant>> for S3GrantList {
    fn from(v: Vec<S3Grant>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<S3Grant> for S3GrantList {
    fn from_iter<I: IntoIterator<Item = S3Grant>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<S3Grant>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlS3GrantList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<S3Grant>,
}

impl From<Vec<S3Grant>> for XmlS3GrantList {
    fn from(v: Vec<S3Grant>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<S3Grant> for XmlS3GrantList {
    fn from_iter<I: IntoIterator<Item = S3Grant>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "S3Grant")]
pub struct S3Grant {
    #[serde(rename = "Grantee")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grantee: Option<S3Grantee>,
    #[serde(rename = "Permission")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "S3Grantee")]
pub struct S3Grantee {
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "Identifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(rename = "TypeIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "S3ObjectOwner")]
pub struct S3ObjectOwner {
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "ID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_d: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "S3CopyObjectOperation")]
pub struct S3CopyObjectOperation {
    #[serde(rename = "AccessControlGrants")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_grants: Option<S3GrantList>,
    #[serde(rename = "BucketKeyEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_key_enabled: Option<bool>,
    #[serde(rename = "CannedAccessControlList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canned_access_control_list: Option<String>,
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
    #[serde(rename = "MetadataDirective")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_directive: Option<String>,
    #[serde(rename = "ModifiedSinceConstraint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_since_constraint: Option<String>,
    #[serde(rename = "NewObjectMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_object_metadata: Option<S3ObjectMetadata>,
    #[serde(rename = "NewObjectTagging")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_object_tagging: Option<S3TagSet>,
    #[serde(rename = "ObjectLockLegalHoldStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_lock_legal_hold_status: Option<String>,
    #[serde(rename = "ObjectLockMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_lock_mode: Option<String>,
    #[serde(rename = "ObjectLockRetainUntilDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_lock_retain_until_date: Option<String>,
    #[serde(rename = "RedirectLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_location: Option<String>,
    #[serde(rename = "RequesterPays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_pays: Option<bool>,
    #[serde(rename = "SSEAwsKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_aws_kms_key_id: Option<String>,
    #[serde(rename = "StorageClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
    #[serde(rename = "TargetKeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_key_prefix: Option<String>,
    #[serde(rename = "TargetResource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_resource: Option<String>,
    #[serde(rename = "UnModifiedSinceConstraint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub un_modified_since_constraint: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "S3ObjectMetadata")]
pub struct S3ObjectMetadata {
    #[serde(rename = "CacheControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<String>,
    #[serde(rename = "ContentDisposition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_disposition: Option<String>,
    #[serde(rename = "ContentEncoding")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_encoding: Option<String>,
    #[serde(rename = "ContentLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_language: Option<String>,
    #[serde(rename = "ContentLength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_length: Option<i64>,
    #[serde(rename = "ContentMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_m_d5: Option<String>,
    #[serde(rename = "ContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "HttpExpiresDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_expires_date: Option<String>,
    #[serde(rename = "RequesterCharged")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_charged: Option<bool>,
    #[serde(rename = "SSEAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_algorithm: Option<String>,
    #[serde(rename = "UserMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_metadata: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "S3SetObjectLegalHoldOperation")]
pub struct S3SetObjectLegalHoldOperation {
    #[serde(rename = "LegalHold")]
    #[serde(default)]
    pub legal_hold: S3ObjectLockLegalHold,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "S3ObjectLockLegalHold")]
pub struct S3ObjectLockLegalHold {
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "S3SetObjectRetentionOperation")]
pub struct S3SetObjectRetentionOperation {
    #[serde(rename = "BypassGovernanceRetention")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_governance_retention: Option<bool>,
    #[serde(rename = "Retention")]
    #[serde(default)]
    pub retention: S3Retention,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "S3Retention")]
pub struct S3Retention {
    #[serde(rename = "Mode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "RetainUntilDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_until_date: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "S3SetObjectTaggingOperation")]
pub struct S3SetObjectTaggingOperation {
    #[serde(rename = "TagSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_set: Option<S3TagSet>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3ReplicateObjectOperation {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "S3UpdateObjectEncryptionOperation")]
pub struct S3UpdateObjectEncryptionOperation {
    #[serde(rename = "ObjectEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_encryption: Option<ObjectEncryption>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ObjectEncryption")]
pub struct ObjectEncryption {
    #[serde(rename = "SSE-KMS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_k_m_s: Option<S3UpdateObjectEncryptionSSEKMS>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SSE-KMS")]
pub struct S3UpdateObjectEncryptionSSEKMS {
    #[serde(rename = "BucketKeyEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_key_enabled: Option<bool>,
    #[serde(rename = "KMSKeyArn")]
    #[serde(default)]
    pub k_m_s_key_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "JobReport")]
pub struct JobReport {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "Format")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "ReportScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_scope: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListAccessGrantsLocationsResult")]
pub struct ListAccessGrantsLocationsResult {
    #[serde(rename = "AccessGrantsLocationsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_grants_locations_list: Option<AccessGrantsLocationsList>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessGrantsLocationsList {
    #[serde(
        rename = "AccessGrantsLocation",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ListAccessGrantsLocationsEntry>,
}
impl From<Vec<ListAccessGrantsLocationsEntry>> for AccessGrantsLocationsList {
    fn from(v: Vec<ListAccessGrantsLocationsEntry>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ListAccessGrantsLocationsEntry> for AccessGrantsLocationsList {
    fn from_iter<I: IntoIterator<Item = ListAccessGrantsLocationsEntry>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ListAccessGrantsLocationsEntry>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlListAccessGrantsLocationsEntryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ListAccessGrantsLocationsEntry>,
}

impl From<Vec<ListAccessGrantsLocationsEntry>> for XmlListAccessGrantsLocationsEntryList {
    fn from(v: Vec<ListAccessGrantsLocationsEntry>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ListAccessGrantsLocationsEntry> for XmlListAccessGrantsLocationsEntryList {
    fn from_iter<I: IntoIterator<Item = ListAccessGrantsLocationsEntry>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListAccessGrantsLocationsEntry")]
pub struct ListAccessGrantsLocationsEntry {
    #[serde(rename = "AccessGrantsLocationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_grants_location_arn: Option<String>,
    #[serde(rename = "AccessGrantsLocationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_grants_location_id: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "IAMRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_a_m_role_arn: Option<String>,
    #[serde(rename = "LocationScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_scope: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TagResourceRequest")]
pub struct TagResourceRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: TagList,
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
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetMultiRegionAccessPointResult")]
pub struct GetMultiRegionAccessPointResult {
    #[serde(rename = "AccessPoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point: Option<MultiRegionAccessPointReport>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MultiRegionAccessPointReport")]
pub struct MultiRegionAccessPointReport {
    #[serde(rename = "Alias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PublicAccessBlock")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access_block: Option<PublicAccessBlockConfiguration>,
    #[serde(rename = "Regions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<RegionReportList>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PublicAccessBlockConfiguration")]
pub struct PublicAccessBlockConfiguration {
    #[serde(rename = "BlockPublicAcls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_public_acls: Option<bool>,
    #[serde(rename = "BlockPublicPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_public_policy: Option<bool>,
    #[serde(rename = "IgnorePublicAcls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_public_acls: Option<bool>,
    #[serde(rename = "RestrictPublicBuckets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrict_public_buckets: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegionReportList {
    #[serde(rename = "Region", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<RegionReport>,
}
impl From<Vec<RegionReport>> for RegionReportList {
    fn from(v: Vec<RegionReport>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<RegionReport> for RegionReportList {
    fn from_iter<I: IntoIterator<Item = RegionReport>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<RegionReport>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlRegionReportList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<RegionReport>,
}

impl From<Vec<RegionReport>> for XmlRegionReportList {
    fn from(v: Vec<RegionReport>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<RegionReport> for XmlRegionReportList {
    fn from_iter<I: IntoIterator<Item = RegionReport>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RegionReport")]
pub struct RegionReport {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(rename = "BucketAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_account_id: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListAccessGrantsInstancesRequest")]
pub struct ListAccessGrantsInstancesRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
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
#[serde(rename = "GetBucketResult")]
pub struct GetBucketResult {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "PublicAccessBlockEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access_block_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateMultiRegionAccessPointRequest")]
pub struct CreateMultiRegionAccessPointRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    pub client_token: String,
    #[serde(rename = "Details")]
    #[serde(default)]
    pub details: CreateMultiRegionAccessPointInput,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateMultiRegionAccessPointInput")]
pub struct CreateMultiRegionAccessPointInput {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "PublicAccessBlock")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access_block: Option<PublicAccessBlockConfiguration>,
    #[serde(rename = "Regions")]
    #[serde(default)]
    pub regions: RegionCreationList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegionCreationList {
    #[serde(rename = "Region", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Region>,
}
impl From<Vec<Region>> for RegionCreationList {
    fn from(v: Vec<Region>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Region> for RegionCreationList {
    fn from_iter<I: IntoIterator<Item = Region>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Region>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlRegionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Region>,
}

impl From<Vec<Region>> for XmlRegionList {
    fn from(v: Vec<Region>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Region> for XmlRegionList {
    fn from_iter<I: IntoIterator<Item = Region>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Region")]
pub struct Region {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "BucketAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_account_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateAccessPointForObjectLambdaRequest")]
pub struct CreateAccessPointForObjectLambdaRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Configuration")]
    #[serde(default)]
    pub configuration: ObjectLambdaConfiguration,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ObjectLambdaConfiguration")]
pub struct ObjectLambdaConfiguration {
    #[serde(rename = "AllowedFeatures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_features: Option<ObjectLambdaAllowedFeaturesList>,
    #[serde(rename = "CloudWatchMetricsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_metrics_enabled: Option<bool>,
    #[serde(rename = "SupportingAccessPoint")]
    #[serde(default)]
    pub supporting_access_point: String,
    #[serde(rename = "TransformationConfigurations")]
    #[serde(default)]
    pub transformation_configurations: ObjectLambdaTransformationConfigurationsList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ObjectLambdaAllowedFeaturesList {
    #[serde(
        rename = "AllowedFeature",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ObjectLambdaAllowedFeaturesList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ObjectLambdaAllowedFeaturesList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ObjectLambdaTransformationConfigurationsList {
    #[serde(
        rename = "TransformationConfiguration",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ObjectLambdaTransformationConfiguration>,
}
impl From<Vec<ObjectLambdaTransformationConfiguration>>
    for ObjectLambdaTransformationConfigurationsList
{
    fn from(v: Vec<ObjectLambdaTransformationConfiguration>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ObjectLambdaTransformationConfiguration>
    for ObjectLambdaTransformationConfigurationsList
{
    fn from_iter<I: IntoIterator<Item = ObjectLambdaTransformationConfiguration>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ObjectLambdaTransformationConfiguration>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlObjectLambdaTransformationConfigurationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ObjectLambdaTransformationConfiguration>,
}

impl From<Vec<ObjectLambdaTransformationConfiguration>>
    for XmlObjectLambdaTransformationConfigurationList
{
    fn from(v: Vec<ObjectLambdaTransformationConfiguration>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ObjectLambdaTransformationConfiguration>
    for XmlObjectLambdaTransformationConfigurationList
{
    fn from_iter<I: IntoIterator<Item = ObjectLambdaTransformationConfiguration>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ObjectLambdaTransformationConfiguration")]
pub struct ObjectLambdaTransformationConfiguration {
    #[serde(rename = "Actions")]
    #[serde(default)]
    pub actions: ObjectLambdaTransformationConfigurationActionsList,
    #[serde(rename = "ContentTransformation")]
    #[serde(default)]
    pub content_transformation: ObjectLambdaContentTransformation,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ObjectLambdaTransformationConfigurationActionsList {
    #[serde(rename = "Action", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ObjectLambdaTransformationConfigurationActionsList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ObjectLambdaTransformationConfigurationActionsList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ObjectLambdaContentTransformation")]
pub struct ObjectLambdaContentTransformation {
    #[serde(rename = "AwsLambda")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_lambda: Option<AwsLambdaTransformation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AwsLambdaTransformation")]
pub struct AwsLambdaTransformation {
    #[serde(rename = "FunctionArn")]
    #[serde(default)]
    pub function_arn: String,
    #[serde(rename = "FunctionPayload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_payload: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetAccessPointConfigurationForObjectLambdaRequest")]
pub struct GetAccessPointConfigurationForObjectLambdaRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateStorageLensGroupRequest")]
pub struct UpdateStorageLensGroupRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "StorageLensGroup")]
    #[serde(default)]
    pub storage_lens_group: StorageLensGroup,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StorageLensGroup")]
pub struct StorageLensGroup {
    #[serde(rename = "Filter")]
    #[serde(default)]
    pub filter: StorageLensGroupFilter,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "StorageLensGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_lens_group_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StorageLensGroupFilter")]
pub struct StorageLensGroupFilter {
    #[serde(rename = "And")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and: Option<StorageLensGroupAndOperator>,
    #[serde(rename = "MatchAnyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_any_prefix: Option<MatchAnyPrefix>,
    #[serde(rename = "MatchAnySuffix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_any_suffix: Option<MatchAnySuffix>,
    #[serde(rename = "MatchAnyTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_any_tag: Option<MatchAnyTag>,
    #[serde(rename = "MatchObjectAge")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_object_age: Option<MatchObjectAge>,
    #[serde(rename = "MatchObjectSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_object_size: Option<MatchObjectSize>,
    #[serde(rename = "Or")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or: Option<StorageLensGroupOrOperator>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StorageLensGroupAndOperator")]
pub struct StorageLensGroupAndOperator {
    #[serde(rename = "MatchAnyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_any_prefix: Option<MatchAnyPrefix>,
    #[serde(rename = "MatchAnySuffix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_any_suffix: Option<MatchAnySuffix>,
    #[serde(rename = "MatchAnyTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_any_tag: Option<MatchAnyTag>,
    #[serde(rename = "MatchObjectAge")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_object_age: Option<MatchObjectAge>,
    #[serde(rename = "MatchObjectSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_object_size: Option<MatchObjectSize>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MatchAnyPrefix {
    #[serde(rename = "Prefix", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for MatchAnyPrefix {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for MatchAnyPrefix {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MatchAnySuffix {
    #[serde(rename = "Suffix", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for MatchAnySuffix {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for MatchAnySuffix {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MatchAnyTag {
    #[serde(rename = "Tag", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<S3Tag>,
}
impl From<Vec<S3Tag>> for MatchAnyTag {
    fn from(v: Vec<S3Tag>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<S3Tag> for MatchAnyTag {
    fn from_iter<I: IntoIterator<Item = S3Tag>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MatchObjectAge")]
pub struct MatchObjectAge {
    #[serde(rename = "DaysGreaterThan")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_greater_than: Option<i32>,
    #[serde(rename = "DaysLessThan")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_less_than: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MatchObjectSize")]
pub struct MatchObjectSize {
    #[serde(rename = "BytesGreaterThan")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_greater_than: Option<i64>,
    #[serde(rename = "BytesLessThan")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_less_than: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StorageLensGroupOrOperator")]
pub struct StorageLensGroupOrOperator {
    #[serde(rename = "MatchAnyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_any_prefix: Option<MatchAnyPrefix>,
    #[serde(rename = "MatchAnySuffix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_any_suffix: Option<MatchAnySuffix>,
    #[serde(rename = "MatchAnyTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_any_tag: Option<MatchAnyTag>,
    #[serde(rename = "MatchObjectAge")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_object_age: Option<MatchObjectAge>,
    #[serde(rename = "MatchObjectSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_object_size: Option<MatchObjectSize>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketReplicationRequest")]
pub struct GetBucketReplicationRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetStorageLensConfigurationTaggingRequest")]
pub struct GetStorageLensConfigurationTaggingRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "ConfigId")]
    #[serde(default)]
    pub config_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListRegionalBucketsRequest")]
pub struct ListRegionalBucketsRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OutpostId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteBucketReplicationRequest")]
pub struct DeleteBucketReplicationRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetAccessPointPolicyRequest")]
pub struct GetAccessPointPolicyRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateJobResult")]
pub struct CreateJobResult {
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetAccessPointPolicyStatusRequest")]
pub struct GetAccessPointPolicyStatusRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutStorageLensConfigurationTaggingRequest")]
pub struct PutStorageLensConfigurationTaggingRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "ConfigId")]
    #[serde(default)]
    pub config_id: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: StorageLensTags,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StorageLensTags {
    #[serde(rename = "Tag", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<StorageLensTag>,
}
impl From<Vec<StorageLensTag>> for StorageLensTags {
    fn from(v: Vec<StorageLensTag>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<StorageLensTag> for StorageLensTags {
    fn from_iter<I: IntoIterator<Item = StorageLensTag>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<StorageLensTag>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlStorageLensTagList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<StorageLensTag>,
}

impl From<Vec<StorageLensTag>> for XmlStorageLensTagList {
    fn from(v: Vec<StorageLensTag>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<StorageLensTag> for XmlStorageLensTagList {
    fn from_iter<I: IntoIterator<Item = StorageLensTag>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StorageLensTag")]
pub struct StorageLensTag {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteBucketTaggingRequest")]
pub struct DeleteBucketTaggingRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeJobResult")]
pub struct DescribeJobResult {
    #[serde(rename = "Job")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job: Option<JobDescriptor>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "JobDescriptor")]
pub struct JobDescriptor {
    #[serde(rename = "ConfirmationRequired")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_required: Option<bool>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "FailureReasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reasons: Option<JobFailureList>,
    #[serde(rename = "GeneratedManifestDescriptor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_manifest_descriptor: Option<S3GeneratedManifestDescriptor>,
    #[serde(rename = "JobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "Manifest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest: Option<JobManifest>,
    #[serde(rename = "ManifestGenerator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_generator: Option<JobManifestGenerator>,
    #[serde(rename = "Operation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<JobOperation>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "ProgressSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_summary: Option<JobProgressSummary>,
    #[serde(rename = "Report")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report: Option<JobReport>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusUpdateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_update_reason: Option<String>,
    #[serde(rename = "SuspendedCause")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspended_cause: Option<String>,
    #[serde(rename = "SuspendedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspended_date: Option<String>,
    #[serde(rename = "TerminationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_date: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobFailureList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<JobFailure>,
}
impl From<Vec<JobFailure>> for JobFailureList {
    fn from(v: Vec<JobFailure>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<JobFailure> for JobFailureList {
    fn from_iter<I: IntoIterator<Item = JobFailure>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<JobFailure>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlJobFailureList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<JobFailure>,
}

impl From<Vec<JobFailure>> for XmlJobFailureList {
    fn from(v: Vec<JobFailure>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<JobFailure> for XmlJobFailureList {
    fn from_iter<I: IntoIterator<Item = JobFailure>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "JobFailure")]
pub struct JobFailure {
    #[serde(rename = "FailureCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "S3GeneratedManifestDescriptor")]
pub struct S3GeneratedManifestDescriptor {
    #[serde(rename = "Format")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<JobManifestLocation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "JobProgressSummary")]
pub struct JobProgressSummary {
    #[serde(rename = "NumberOfTasksFailed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_tasks_failed: Option<i64>,
    #[serde(rename = "NumberOfTasksSucceeded")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_tasks_succeeded: Option<i64>,
    #[serde(rename = "Timers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timers: Option<JobTimers>,
    #[serde(rename = "TotalNumberOfTasks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_number_of_tasks: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "JobTimers")]
pub struct JobTimers {
    #[serde(rename = "ElapsedTimeInActiveSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elapsed_time_in_active_seconds: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetAccessGrantsInstanceResourcePolicyResult")]
pub struct GetAccessGrantsInstanceResourcePolicyResult {
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "Organization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetAccessPointPolicyForObjectLambdaResult")]
pub struct GetAccessPointPolicyForObjectLambdaResult {
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListAccessPointsResult")]
pub struct ListAccessPointsResult {
    #[serde(rename = "AccessPointList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_list: Option<AccessPointList>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessPointList {
    #[serde(rename = "AccessPoint", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<AccessPoint>,
}
impl From<Vec<AccessPoint>> for AccessPointList {
    fn from(v: Vec<AccessPoint>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<AccessPoint> for AccessPointList {
    fn from_iter<I: IntoIterator<Item = AccessPoint>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<AccessPoint>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlAccessPointList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<AccessPoint>,
}

impl From<Vec<AccessPoint>> for XmlAccessPointList {
    fn from(v: Vec<AccessPoint>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<AccessPoint> for XmlAccessPointList {
    fn from_iter<I: IntoIterator<Item = AccessPoint>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AccessPoint")]
pub struct AccessPoint {
    #[serde(rename = "AccessPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_arn: Option<String>,
    #[serde(rename = "Alias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(rename = "BucketAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_account_id: Option<String>,
    #[serde(rename = "DataSourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,
    #[serde(rename = "DataSourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_type: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NetworkOrigin")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_origin: Option<String>,
    #[serde(rename = "VpcConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_configuration: Option<VpcConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "VpcConfiguration")]
pub struct VpcConfiguration {
    #[serde(rename = "VpcId")]
    #[serde(default)]
    pub vpc_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateAccessGrantsLocationResult")]
pub struct UpdateAccessGrantsLocationResult {
    #[serde(rename = "AccessGrantsLocationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_grants_location_arn: Option<String>,
    #[serde(rename = "AccessGrantsLocationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_grants_location_id: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "IAMRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_a_m_role_arn: Option<String>,
    #[serde(rename = "LocationScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_scope: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteAccessGrantsInstanceRequest")]
pub struct DeleteAccessGrantsInstanceRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteAccessGrantsLocationRequest")]
pub struct DeleteAccessGrantsLocationRequest {
    #[serde(rename = "AccessGrantsLocationId")]
    #[serde(default)]
    pub access_grants_location_id: String,
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutMultiRegionAccessPointPolicyRequest")]
pub struct PutMultiRegionAccessPointPolicyRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    pub client_token: String,
    #[serde(rename = "Details")]
    #[serde(default)]
    pub details: PutMultiRegionAccessPointPolicyInput,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutMultiRegionAccessPointPolicyInput")]
pub struct PutMultiRegionAccessPointPolicyInput {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Policy")]
    #[serde(default)]
    pub policy: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateAccessGrantsInstanceRequest")]
pub struct CreateAccessGrantsInstanceRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "IdentityCenterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_center_arn: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateMultiRegionAccessPointResult")]
pub struct CreateMultiRegionAccessPointResult {
    #[serde(rename = "RequestTokenARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_token_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteBucketLifecycleConfigurationRequest")]
pub struct DeleteBucketLifecycleConfigurationRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteJobTaggingRequest")]
pub struct DeleteJobTaggingRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetAccessPointScopeRequest")]
pub struct GetAccessPointScopeRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutPublicAccessBlockRequest")]
pub struct PutPublicAccessBlockRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "PublicAccessBlockConfiguration")]
    #[serde(default)]
    pub public_access_block_configuration: PublicAccessBlockConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetAccessPointPolicyForObjectLambdaRequest")]
pub struct GetAccessPointPolicyForObjectLambdaRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Tagging")]
pub struct Tagging {
    #[serde(rename = "TagSet")]
    #[serde(default)]
    pub tag_set: S3TagSet,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SubmitMultiRegionAccessPointRoutesRequest")]
pub struct SubmitMultiRegionAccessPointRoutesRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Mrap")]
    #[serde(default)]
    pub mrap: String,
    #[serde(rename = "RouteUpdates")]
    #[serde(default)]
    pub route_updates: RouteList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouteList {
    #[serde(rename = "Route", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<MultiRegionAccessPointRoute>,
}
impl From<Vec<MultiRegionAccessPointRoute>> for RouteList {
    fn from(v: Vec<MultiRegionAccessPointRoute>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<MultiRegionAccessPointRoute> for RouteList {
    fn from_iter<I: IntoIterator<Item = MultiRegionAccessPointRoute>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<MultiRegionAccessPointRoute>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlMultiRegionAccessPointRouteList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<MultiRegionAccessPointRoute>,
}

impl From<Vec<MultiRegionAccessPointRoute>> for XmlMultiRegionAccessPointRouteList {
    fn from(v: Vec<MultiRegionAccessPointRoute>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<MultiRegionAccessPointRoute> for XmlMultiRegionAccessPointRouteList {
    fn from_iter<I: IntoIterator<Item = MultiRegionAccessPointRoute>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MultiRegionAccessPointRoute")]
pub struct MultiRegionAccessPointRoute {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "TrafficDialPercentage")]
    #[serde(default)]
    pub traffic_dial_percentage: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StorageLensConfiguration")]
pub struct StorageLensConfiguration {
    #[serde(rename = "AccountLevel")]
    #[serde(default)]
    pub account_level: AccountLevel,
    #[serde(rename = "AwsOrg")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_org: Option<StorageLensAwsOrg>,
    #[serde(rename = "DataExport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_export: Option<StorageLensDataExport>,
    #[serde(rename = "Exclude")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Exclude>,
    #[serde(rename = "ExpandedPrefixesDataExport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expanded_prefixes_data_export: Option<StorageLensExpandedPrefixesDataExport>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Include")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Include>,
    #[serde(rename = "IsEnabled")]
    #[serde(default)]
    pub is_enabled: bool,
    #[serde(rename = "PrefixDelimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_delimiter: Option<String>,
    #[serde(rename = "StorageLensArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_lens_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AccountLevel")]
pub struct AccountLevel {
    #[serde(rename = "ActivityMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_metrics: Option<ActivityMetrics>,
    #[serde(rename = "AdvancedCostOptimizationMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_cost_optimization_metrics: Option<AdvancedCostOptimizationMetrics>,
    #[serde(rename = "AdvancedDataProtectionMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_data_protection_metrics: Option<AdvancedDataProtectionMetrics>,
    #[serde(rename = "AdvancedPerformanceMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_performance_metrics: Option<AdvancedPerformanceMetrics>,
    #[serde(rename = "BucketLevel")]
    #[serde(default)]
    pub bucket_level: BucketLevel,
    #[serde(rename = "DetailedStatusCodesMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_status_codes_metrics: Option<DetailedStatusCodesMetrics>,
    #[serde(rename = "StorageLensGroupLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_lens_group_level: Option<StorageLensGroupLevel>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ActivityMetrics")]
pub struct ActivityMetrics {
    #[serde(rename = "IsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AdvancedCostOptimizationMetrics")]
pub struct AdvancedCostOptimizationMetrics {
    #[serde(rename = "IsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AdvancedDataProtectionMetrics")]
pub struct AdvancedDataProtectionMetrics {
    #[serde(rename = "IsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AdvancedPerformanceMetrics")]
pub struct AdvancedPerformanceMetrics {
    #[serde(rename = "IsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "BucketLevel")]
pub struct BucketLevel {
    #[serde(rename = "ActivityMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_metrics: Option<ActivityMetrics>,
    #[serde(rename = "AdvancedCostOptimizationMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_cost_optimization_metrics: Option<AdvancedCostOptimizationMetrics>,
    #[serde(rename = "AdvancedDataProtectionMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_data_protection_metrics: Option<AdvancedDataProtectionMetrics>,
    #[serde(rename = "AdvancedPerformanceMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_performance_metrics: Option<AdvancedPerformanceMetrics>,
    #[serde(rename = "DetailedStatusCodesMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_status_codes_metrics: Option<DetailedStatusCodesMetrics>,
    #[serde(rename = "PrefixLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_level: Option<PrefixLevel>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DetailedStatusCodesMetrics")]
pub struct DetailedStatusCodesMetrics {
    #[serde(rename = "IsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PrefixLevel")]
pub struct PrefixLevel {
    #[serde(rename = "StorageMetrics")]
    #[serde(default)]
    pub storage_metrics: PrefixLevelStorageMetrics,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PrefixLevelStorageMetrics")]
pub struct PrefixLevelStorageMetrics {
    #[serde(rename = "IsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[serde(rename = "SelectionCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_criteria: Option<SelectionCriteria>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SelectionCriteria")]
pub struct SelectionCriteria {
    #[serde(rename = "Delimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    #[serde(rename = "MaxDepth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_depth: Option<i32>,
    #[serde(rename = "MinStorageBytesPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_storage_bytes_percentage: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StorageLensGroupLevel")]
pub struct StorageLensGroupLevel {
    #[serde(rename = "SelectionCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_criteria: Option<StorageLensGroupLevelSelectionCriteria>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StorageLensGroupLevelSelectionCriteria")]
pub struct StorageLensGroupLevelSelectionCriteria {
    #[serde(rename = "Exclude")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<StorageLensGroupLevelExclude>,
    #[serde(rename = "Include")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<StorageLensGroupLevelInclude>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StorageLensGroupLevelExclude {
    #[serde(rename = "Arn", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for StorageLensGroupLevelExclude {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for StorageLensGroupLevelExclude {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StorageLensGroupLevelInclude {
    #[serde(rename = "Arn", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for StorageLensGroupLevelInclude {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for StorageLensGroupLevelInclude {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StorageLensAwsOrg")]
pub struct StorageLensAwsOrg {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StorageLensDataExport")]
pub struct StorageLensDataExport {
    #[serde(rename = "CloudWatchMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_metrics: Option<CloudWatchMetrics>,
    #[serde(rename = "S3BucketDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_destination: Option<S3BucketDestination>,
    #[serde(rename = "StorageLensTableDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_lens_table_destination: Option<StorageLensTableDestination>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CloudWatchMetrics")]
pub struct CloudWatchMetrics {
    #[serde(rename = "IsEnabled")]
    #[serde(default)]
    pub is_enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "S3BucketDestination")]
pub struct S3BucketDestination {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "Encryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<StorageLensDataExportEncryption>,
    #[serde(rename = "Format")]
    #[serde(default)]
    pub format: String,
    #[serde(rename = "OutputSchemaVersion")]
    #[serde(default)]
    pub output_schema_version: String,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StorageLensDataExportEncryption")]
pub struct StorageLensDataExportEncryption {
    #[serde(rename = "SSE-KMS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_k_m_s: Option<SSEKMS>,
    #[serde(rename = "SSE-S3")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_s3: Option<SSES3>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SSE-KMS")]
pub struct SSEKMS {
    #[serde(rename = "KeyId")]
    #[serde(default)]
    pub key_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SSES3 {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StorageLensTableDestination")]
pub struct StorageLensTableDestination {
    #[serde(rename = "Encryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<StorageLensDataExportEncryption>,
    #[serde(rename = "IsEnabled")]
    #[serde(default)]
    pub is_enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Exclude")]
pub struct Exclude {
    #[serde(rename = "Buckets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buckets: Option<Buckets>,
    #[serde(rename = "Regions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Regions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Buckets {
    #[serde(rename = "Arn", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for Buckets {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for Buckets {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Regions {
    #[serde(rename = "Region", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for Regions {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for Regions {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StorageLensExpandedPrefixesDataExport")]
pub struct StorageLensExpandedPrefixesDataExport {
    #[serde(rename = "S3BucketDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_destination: Option<S3BucketDestination>,
    #[serde(rename = "StorageLensTableDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_lens_table_destination: Option<StorageLensTableDestination>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Include")]
pub struct Include {
    #[serde(rename = "Buckets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buckets: Option<Buckets>,
    #[serde(rename = "Regions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Regions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteBucketPolicyRequest")]
pub struct DeleteBucketPolicyRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteStorageLensGroupRequest")]
pub struct DeleteStorageLensGroupRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListCallerAccessGrantsResult")]
pub struct ListCallerAccessGrantsResult {
    #[serde(rename = "CallerAccessGrantsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caller_access_grants_list: Option<CallerAccessGrantsList>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CallerAccessGrantsList {
    #[serde(rename = "AccessGrant", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ListCallerAccessGrantsEntry>,
}
impl From<Vec<ListCallerAccessGrantsEntry>> for CallerAccessGrantsList {
    fn from(v: Vec<ListCallerAccessGrantsEntry>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ListCallerAccessGrantsEntry> for CallerAccessGrantsList {
    fn from_iter<I: IntoIterator<Item = ListCallerAccessGrantsEntry>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ListCallerAccessGrantsEntry>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlListCallerAccessGrantsEntryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ListCallerAccessGrantsEntry>,
}

impl From<Vec<ListCallerAccessGrantsEntry>> for XmlListCallerAccessGrantsEntryList {
    fn from(v: Vec<ListCallerAccessGrantsEntry>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ListCallerAccessGrantsEntry> for XmlListCallerAccessGrantsEntryList {
    fn from_iter<I: IntoIterator<Item = ListCallerAccessGrantsEntry>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListCallerAccessGrantsEntry")]
pub struct ListCallerAccessGrantsEntry {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
    #[serde(rename = "GrantScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_scope: Option<String>,
    #[serde(rename = "Permission")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetMultiRegionAccessPointRoutesResult")]
pub struct GetMultiRegionAccessPointRoutesResult {
    #[serde(rename = "Mrap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mrap: Option<String>,
    #[serde(rename = "Routes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routes: Option<RouteList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetStorageLensGroupRequest")]
pub struct GetStorageLensGroupRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateAccessGrantResult")]
pub struct CreateAccessGrantResult {
    #[serde(rename = "AccessGrantArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_grant_arn: Option<String>,
    #[serde(rename = "AccessGrantId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_grant_id: Option<String>,
    #[serde(rename = "AccessGrantsLocationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_grants_location_configuration: Option<AccessGrantsLocationConfiguration>,
    #[serde(rename = "AccessGrantsLocationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_grants_location_id: Option<String>,
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "GrantScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_scope: Option<String>,
    #[serde(rename = "Grantee")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grantee: Option<Grantee>,
    #[serde(rename = "Permission")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListStorageLensGroupsResult")]
pub struct ListStorageLensGroupsResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StorageLensGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_lens_group_list: Option<Vec<ListStorageLensGroupEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListStorageLensGroupEntry")]
pub struct ListStorageLensGroupEntry {
    #[serde(rename = "HomeRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_region: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "StorageLensGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_lens_group_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetAccessPointConfigurationForObjectLambdaResult")]
pub struct GetAccessPointConfigurationForObjectLambdaResult {
    #[serde(rename = "Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ObjectLambdaConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DissociateAccessGrantsIdentityCenterRequest")]
pub struct DissociateAccessGrantsIdentityCenterRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListMultiRegionAccessPointsResult")]
pub struct ListMultiRegionAccessPointsResult {
    #[serde(rename = "AccessPoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_points: Option<MultiRegionAccessPointReportList>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiRegionAccessPointReportList {
    #[serde(rename = "AccessPoint", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<MultiRegionAccessPointReport>,
}
impl From<Vec<MultiRegionAccessPointReport>> for MultiRegionAccessPointReportList {
    fn from(v: Vec<MultiRegionAccessPointReport>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<MultiRegionAccessPointReport> for MultiRegionAccessPointReportList {
    fn from_iter<I: IntoIterator<Item = MultiRegionAccessPointReport>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<MultiRegionAccessPointReport>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlMultiRegionAccessPointReportList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<MultiRegionAccessPointReport>,
}

impl From<Vec<MultiRegionAccessPointReport>> for XmlMultiRegionAccessPointReportList {
    fn from(v: Vec<MultiRegionAccessPointReport>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<MultiRegionAccessPointReport> for XmlMultiRegionAccessPointReportList {
    fn from_iter<I: IntoIterator<Item = MultiRegionAccessPointReport>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteAccessPointPolicyRequest")]
pub struct DeleteAccessPointPolicyRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetJobTaggingResult")]
pub struct GetJobTaggingResult {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<S3TagSet>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeletePublicAccessBlockRequest")]
pub struct DeletePublicAccessBlockRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListAccessPointsForObjectLambdaRequest")]
pub struct ListAccessPointsForObjectLambdaRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
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
#[serde(rename = "UpdateJobPriorityRequest")]
pub struct UpdateJobPriorityRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
    #[serde(rename = "Priority")]
    #[serde(default)]
    pub priority: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateAccessPointForObjectLambdaResult")]
pub struct CreateAccessPointForObjectLambdaResult {
    #[serde(rename = "Alias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<ObjectLambdaAccessPointAlias>,
    #[serde(rename = "ObjectLambdaAccessPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_lambda_access_point_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ObjectLambdaAccessPointAlias")]
pub struct ObjectLambdaAccessPointAlias {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UntagResourceRequest")]
pub struct UntagResourceRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: TagKeyList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagKeyList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
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
#[serde(rename = "CreateAccessGrantsLocationRequest")]
pub struct CreateAccessGrantsLocationRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "IAMRoleArn")]
    #[serde(default)]
    pub i_a_m_role_arn: String,
    #[serde(rename = "LocationScope")]
    #[serde(default)]
    pub location_scope: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetAccessGrantsInstanceResult")]
pub struct GetAccessGrantsInstanceResult {
    #[serde(rename = "AccessGrantsInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_grants_instance_arn: Option<String>,
    #[serde(rename = "AccessGrantsInstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_grants_instance_id: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "IdentityCenterApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_center_application_arn: Option<String>,
    #[serde(rename = "IdentityCenterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_center_arn: Option<String>,
    #[serde(rename = "IdentityCenterInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_center_instance_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateStorageLensGroupRequest")]
pub struct CreateStorageLensGroupRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "StorageLensGroup")]
    #[serde(default)]
    pub storage_lens_group: StorageLensGroup,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketRequest")]
pub struct GetBucketRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListAccessPointsForDirectoryBucketsResult")]
pub struct ListAccessPointsForDirectoryBucketsResult {
    #[serde(rename = "AccessPointList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_list: Option<AccessPointList>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetAccessGrantsInstanceRequest")]
pub struct GetAccessGrantsInstanceRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetAccessGrantsInstanceResourcePolicyRequest")]
pub struct GetAccessGrantsInstanceResourcePolicyRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetAccessPointForObjectLambdaRequest")]
pub struct GetAccessPointForObjectLambdaRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutJobTaggingRequest")]
pub struct PutJobTaggingRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: S3TagSet,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteAccessGrantsInstanceResourcePolicyRequest")]
pub struct DeleteAccessGrantsInstanceResourcePolicyRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubmitMultiRegionAccessPointRoutesResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteBucketRequest")]
pub struct DeleteBucketRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListStorageLensConfigurationsRequest")]
pub struct ListStorageLensConfigurationsRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutAccessPointPolicyForObjectLambdaRequest")]
pub struct PutAccessPointPolicyForObjectLambdaRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Policy")]
    #[serde(default)]
    pub policy: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutAccessPointScopeRequest")]
pub struct PutAccessPointScopeRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: Scope,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Scope")]
pub struct Scope {
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ScopePermissionList>,
    #[serde(rename = "Prefixes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefixes: Option<PrefixesList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScopePermissionList {
    #[serde(rename = "Permission", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ScopePermissionList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ScopePermissionList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PrefixesList {
    #[serde(rename = "Prefix", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for PrefixesList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for PrefixesList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetAccessPointScopeResult")]
pub struct GetAccessPointScopeResult {
    #[serde(rename = "Scope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Scope>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateAccessGrantsInstanceResult")]
pub struct CreateAccessGrantsInstanceResult {
    #[serde(rename = "AccessGrantsInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_grants_instance_arn: Option<String>,
    #[serde(rename = "AccessGrantsInstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_grants_instance_id: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "IdentityCenterApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_center_application_arn: Option<String>,
    #[serde(rename = "IdentityCenterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_center_arn: Option<String>,
    #[serde(rename = "IdentityCenterInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_center_instance_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteAccessPointRequest")]
pub struct DeleteAccessPointRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListCallerAccessGrantsRequest")]
pub struct ListCallerAccessGrantsRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "AllowedByApplication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_by_application: Option<bool>,
    #[serde(rename = "GrantScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_scope: Option<String>,
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
#[serde(rename = "GetStorageLensConfigurationRequest")]
pub struct GetStorageLensConfigurationRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "ConfigId")]
    #[serde(default)]
    pub config_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetMultiRegionAccessPointPolicyStatusResult")]
pub struct GetMultiRegionAccessPointPolicyStatusResult {
    #[serde(rename = "Established")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub established: Option<PolicyStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PolicyStatus")]
pub struct PolicyStatus {
    #[serde(rename = "IsPublic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetStorageLensGroupResult")]
pub struct GetStorageLensGroupResult {
    #[serde(rename = "StorageLensGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_lens_group: Option<StorageLensGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutBucketLifecycleConfigurationRequest")]
pub struct PutBucketLifecycleConfigurationRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "LifecycleConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_configuration: Option<LifecycleConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LifecycleConfiguration")]
pub struct LifecycleConfiguration {
    #[serde(rename = "Rules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<LifecycleRules>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LifecycleRules {
    #[serde(rename = "Rule", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<LifecycleRule>,
}
impl From<Vec<LifecycleRule>> for LifecycleRules {
    fn from(v: Vec<LifecycleRule>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<LifecycleRule> for LifecycleRules {
    fn from_iter<I: IntoIterator<Item = LifecycleRule>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<LifecycleRule>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlLifecycleRuleList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<LifecycleRule>,
}

impl From<Vec<LifecycleRule>> for XmlLifecycleRuleList {
    fn from(v: Vec<LifecycleRule>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<LifecycleRule> for XmlLifecycleRuleList {
    fn from_iter<I: IntoIterator<Item = LifecycleRule>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LifecycleRule")]
pub struct LifecycleRule {
    #[serde(rename = "AbortIncompleteMultipartUpload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abort_incomplete_multipart_upload: Option<AbortIncompleteMultipartUpload>,
    #[serde(rename = "Expiration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<LifecycleExpiration>,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<LifecycleRuleFilter>,
    #[serde(rename = "ID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_d: Option<String>,
    #[serde(rename = "NoncurrentVersionExpiration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noncurrent_version_expiration: Option<NoncurrentVersionExpiration>,
    #[serde(rename = "NoncurrentVersionTransitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noncurrent_version_transitions: Option<NoncurrentVersionTransitionList>,
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
    #[serde(rename = "Transitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transitions: Option<TransitionList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AbortIncompleteMultipartUpload")]
pub struct AbortIncompleteMultipartUpload {
    #[serde(rename = "DaysAfterInitiation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_after_initiation: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LifecycleExpiration")]
pub struct LifecycleExpiration {
    #[serde(rename = "Date")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "Days")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i32>,
    #[serde(rename = "ExpiredObjectDeleteMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_object_delete_marker: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LifecycleRuleFilter")]
pub struct LifecycleRuleFilter {
    #[serde(rename = "And")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and: Option<LifecycleRuleAndOperator>,
    #[serde(rename = "ObjectSizeGreaterThan")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_size_greater_than: Option<i64>,
    #[serde(rename = "ObjectSizeLessThan")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_size_less_than: Option<i64>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "Tag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<S3Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LifecycleRuleAndOperator")]
pub struct LifecycleRuleAndOperator {
    #[serde(rename = "ObjectSizeGreaterThan")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_size_greater_than: Option<i64>,
    #[serde(rename = "ObjectSizeLessThan")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_size_less_than: Option<i64>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<S3TagSet>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "NoncurrentVersionExpiration")]
pub struct NoncurrentVersionExpiration {
    #[serde(rename = "NewerNoncurrentVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newer_noncurrent_versions: Option<i32>,
    #[serde(rename = "NoncurrentDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noncurrent_days: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NoncurrentVersionTransitionList {
    #[serde(
        rename = "NoncurrentVersionTransition",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<NoncurrentVersionTransition>,
}
impl From<Vec<NoncurrentVersionTransition>> for NoncurrentVersionTransitionList {
    fn from(v: Vec<NoncurrentVersionTransition>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<NoncurrentVersionTransition> for NoncurrentVersionTransitionList {
    fn from_iter<I: IntoIterator<Item = NoncurrentVersionTransition>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<NoncurrentVersionTransition>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlNoncurrentVersionTransitionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<NoncurrentVersionTransition>,
}

impl From<Vec<NoncurrentVersionTransition>> for XmlNoncurrentVersionTransitionList {
    fn from(v: Vec<NoncurrentVersionTransition>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<NoncurrentVersionTransition> for XmlNoncurrentVersionTransitionList {
    fn from_iter<I: IntoIterator<Item = NoncurrentVersionTransition>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "NoncurrentVersionTransition")]
pub struct NoncurrentVersionTransition {
    #[serde(rename = "NoncurrentDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noncurrent_days: Option<i32>,
    #[serde(rename = "StorageClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransitionList {
    #[serde(rename = "Transition", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Transition>,
}
impl From<Vec<Transition>> for TransitionList {
    fn from(v: Vec<Transition>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Transition> for TransitionList {
    fn from_iter<I: IntoIterator<Item = Transition>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Transition>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTransitionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Transition>,
}

impl From<Vec<Transition>> for XmlTransitionList {
    fn from(v: Vec<Transition>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Transition> for XmlTransitionList {
    fn from_iter<I: IntoIterator<Item = Transition>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Transition")]
pub struct Transition {
    #[serde(rename = "Date")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "Days")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i32>,
    #[serde(rename = "StorageClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateBucketResult")]
pub struct CreateBucketResult {
    #[serde(rename = "BucketArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteJobTaggingResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteStorageLensConfigurationTaggingResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutAccessGrantsInstanceResourcePolicyResult")]
pub struct PutAccessGrantsInstanceResourcePolicyResult {
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "Organization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListAccessGrantsInstancesResult")]
pub struct ListAccessGrantsInstancesResult {
    #[serde(rename = "AccessGrantsInstancesList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_grants_instances_list: Option<AccessGrantsInstancesList>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessGrantsInstancesList {
    #[serde(
        rename = "AccessGrantsInstance",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ListAccessGrantsInstanceEntry>,
}
impl From<Vec<ListAccessGrantsInstanceEntry>> for AccessGrantsInstancesList {
    fn from(v: Vec<ListAccessGrantsInstanceEntry>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ListAccessGrantsInstanceEntry> for AccessGrantsInstancesList {
    fn from_iter<I: IntoIterator<Item = ListAccessGrantsInstanceEntry>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ListAccessGrantsInstanceEntry>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlListAccessGrantsInstanceEntryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ListAccessGrantsInstanceEntry>,
}

impl From<Vec<ListAccessGrantsInstanceEntry>> for XmlListAccessGrantsInstanceEntryList {
    fn from(v: Vec<ListAccessGrantsInstanceEntry>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ListAccessGrantsInstanceEntry> for XmlListAccessGrantsInstanceEntryList {
    fn from_iter<I: IntoIterator<Item = ListAccessGrantsInstanceEntry>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListAccessGrantsInstanceEntry")]
pub struct ListAccessGrantsInstanceEntry {
    #[serde(rename = "AccessGrantsInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_grants_instance_arn: Option<String>,
    #[serde(rename = "AccessGrantsInstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_grants_instance_id: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "IdentityCenterApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_center_application_arn: Option<String>,
    #[serde(rename = "IdentityCenterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_center_arn: Option<String>,
    #[serde(rename = "IdentityCenterInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_center_instance_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeMultiRegionAccessPointOperationResult")]
pub struct DescribeMultiRegionAccessPointOperationResult {
    #[serde(rename = "AsyncOperation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub async_operation: Option<AsyncOperation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AsyncOperation")]
pub struct AsyncOperation {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "Operation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(rename = "RequestParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<AsyncRequestParameters>,
    #[serde(rename = "RequestStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_status: Option<String>,
    #[serde(rename = "RequestTokenARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_token_a_r_n: Option<String>,
    #[serde(rename = "ResponseDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_details: Option<AsyncResponseDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AsyncRequestParameters")]
pub struct AsyncRequestParameters {
    #[serde(rename = "CreateMultiRegionAccessPointRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_multi_region_access_point_request: Option<CreateMultiRegionAccessPointInput>,
    #[serde(rename = "DeleteMultiRegionAccessPointRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_multi_region_access_point_request: Option<DeleteMultiRegionAccessPointInput>,
    #[serde(rename = "PutMultiRegionAccessPointPolicyRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put_multi_region_access_point_policy_request: Option<PutMultiRegionAccessPointPolicyInput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AsyncResponseDetails")]
pub struct AsyncResponseDetails {
    #[serde(rename = "ErrorDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<AsyncErrorDetails>,
    #[serde(rename = "MultiRegionAccessPointDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_access_point_details: Option<MultiRegionAccessPointsAsyncResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AsyncErrorDetails")]
pub struct AsyncErrorDetails {
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Resource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MultiRegionAccessPointsAsyncResponse")]
pub struct MultiRegionAccessPointsAsyncResponse {
    #[serde(rename = "Regions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<MultiRegionAccessPointRegionalResponseList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiRegionAccessPointRegionalResponseList {
    #[serde(rename = "Region", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<MultiRegionAccessPointRegionalResponse>,
}
impl From<Vec<MultiRegionAccessPointRegionalResponse>>
    for MultiRegionAccessPointRegionalResponseList
{
    fn from(v: Vec<MultiRegionAccessPointRegionalResponse>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<MultiRegionAccessPointRegionalResponse>
    for MultiRegionAccessPointRegionalResponseList
{
    fn from_iter<I: IntoIterator<Item = MultiRegionAccessPointRegionalResponse>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<MultiRegionAccessPointRegionalResponse>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlMultiRegionAccessPointRegionalResponseList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<MultiRegionAccessPointRegionalResponse>,
}

impl From<Vec<MultiRegionAccessPointRegionalResponse>>
    for XmlMultiRegionAccessPointRegionalResponseList
{
    fn from(v: Vec<MultiRegionAccessPointRegionalResponse>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<MultiRegionAccessPointRegionalResponse>
    for XmlMultiRegionAccessPointRegionalResponseList
{
    fn from_iter<I: IntoIterator<Item = MultiRegionAccessPointRegionalResponse>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MultiRegionAccessPointRegionalResponse")]
pub struct MultiRegionAccessPointRegionalResponse {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RequestStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetJobTaggingRequest")]
pub struct GetJobTaggingRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteAccessPointScopeRequest")]
pub struct DeleteAccessPointScopeRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutAccessPointConfigurationForObjectLambdaRequest")]
pub struct PutAccessPointConfigurationForObjectLambdaRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Configuration")]
    #[serde(default)]
    pub configuration: ObjectLambdaConfiguration,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutBucketPolicyRequest")]
pub struct PutBucketPolicyRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ConfirmRemoveSelfBucketAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm_remove_self_bucket_access: Option<bool>,
    #[serde(rename = "Policy")]
    #[serde(default)]
    pub policy: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketLifecycleConfigurationResult")]
pub struct GetBucketLifecycleConfigurationResult {
    #[serde(rename = "Rules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<LifecycleRules>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketVersioningRequest")]
pub struct GetBucketVersioningRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketLifecycleConfigurationRequest")]
pub struct GetBucketLifecycleConfigurationRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetAccessPointForObjectLambdaResult")]
pub struct GetAccessPointForObjectLambdaResult {
    #[serde(rename = "Alias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<ObjectLambdaAccessPointAlias>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PublicAccessBlockConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access_block_configuration: Option<PublicAccessBlockConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetMultiRegionAccessPointPolicyRequest")]
pub struct GetMultiRegionAccessPointPolicyRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateAccessPointRequest")]
pub struct CreateAccessPointRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "BucketAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_account_id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "PublicAccessBlockConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access_block_configuration: Option<PublicAccessBlockConfiguration>,
    #[serde(rename = "Scope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Scope>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "VpcConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_configuration: Option<VpcConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListAccessGrantsResult")]
pub struct ListAccessGrantsResult {
    #[serde(rename = "AccessGrantsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_grants_list: Option<AccessGrantsList>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessGrantsList {
    #[serde(rename = "AccessGrant", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ListAccessGrantEntry>,
}
impl From<Vec<ListAccessGrantEntry>> for AccessGrantsList {
    fn from(v: Vec<ListAccessGrantEntry>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ListAccessGrantEntry> for AccessGrantsList {
    fn from_iter<I: IntoIterator<Item = ListAccessGrantEntry>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ListAccessGrantEntry>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlListAccessGrantEntryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ListAccessGrantEntry>,
}

impl From<Vec<ListAccessGrantEntry>> for XmlListAccessGrantEntryList {
    fn from(v: Vec<ListAccessGrantEntry>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ListAccessGrantEntry> for XmlListAccessGrantEntryList {
    fn from_iter<I: IntoIterator<Item = ListAccessGrantEntry>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListAccessGrantEntry")]
pub struct ListAccessGrantEntry {
    #[serde(rename = "AccessGrantArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_grant_arn: Option<String>,
    #[serde(rename = "AccessGrantId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_grant_id: Option<String>,
    #[serde(rename = "AccessGrantsLocationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_grants_location_configuration: Option<AccessGrantsLocationConfiguration>,
    #[serde(rename = "AccessGrantsLocationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_grants_location_id: Option<String>,
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "GrantScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_scope: Option<String>,
    #[serde(rename = "Grantee")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grantee: Option<Grantee>,
    #[serde(rename = "Permission")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetAccessPointPolicyStatusResult")]
pub struct GetAccessPointPolicyStatusResult {
    #[serde(rename = "PolicyStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_status: Option<PolicyStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListJobsResult")]
pub struct ListJobsResult {
    #[serde(rename = "Jobs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<JobListDescriptorList>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobListDescriptorList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<JobListDescriptor>,
}
impl From<Vec<JobListDescriptor>> for JobListDescriptorList {
    fn from(v: Vec<JobListDescriptor>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<JobListDescriptor> for JobListDescriptorList {
    fn from_iter<I: IntoIterator<Item = JobListDescriptor>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<JobListDescriptor>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlJobListDescriptorList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<JobListDescriptor>,
}

impl From<Vec<JobListDescriptor>> for XmlJobListDescriptorList {
    fn from(v: Vec<JobListDescriptor>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<JobListDescriptor> for XmlJobListDescriptorList {
    fn from_iter<I: IntoIterator<Item = JobListDescriptor>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "JobListDescriptor")]
pub struct JobListDescriptor {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "Operation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "ProgressSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_summary: Option<JobProgressSummary>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TerminationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_date: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutBucketVersioningRequest")]
pub struct PutBucketVersioningRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "MFA")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_f_a: Option<String>,
    #[serde(rename = "VersioningConfiguration")]
    #[serde(default)]
    pub versioning_configuration: VersioningConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "VersioningConfiguration")]
pub struct VersioningConfiguration {
    #[serde(rename = "MfaDelete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_f_a_delete: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutStorageLensConfigurationTaggingResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetStorageLensConfigurationTaggingResult")]
pub struct GetStorageLensConfigurationTaggingResult {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<StorageLensTags>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteAccessPointPolicyForObjectLambdaRequest")]
pub struct DeleteAccessPointPolicyForObjectLambdaRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetAccessGrantRequest")]
pub struct GetAccessGrantRequest {
    #[serde(rename = "AccessGrantId")]
    #[serde(default)]
    pub access_grant_id: String,
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetAccessPointPolicyResult")]
pub struct GetAccessPointPolicyResult {
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetStorageLensConfigurationResult")]
pub struct GetStorageLensConfigurationResult {
    #[serde(rename = "StorageLensConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_lens_configuration: Option<StorageLensConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListAccessPointsForDirectoryBucketsRequest")]
pub struct ListAccessPointsForDirectoryBucketsRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "DirectoryBucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_bucket: Option<String>,
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
#[serde(rename = "DeleteStorageLensConfigurationTaggingRequest")]
pub struct DeleteStorageLensConfigurationTaggingRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "ConfigId")]
    #[serde(default)]
    pub config_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutBucketReplicationRequest")]
pub struct PutBucketReplicationRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ReplicationConfiguration")]
    #[serde(default)]
    pub replication_configuration: ReplicationConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetAccessGrantsInstanceForPrefixResult")]
pub struct GetAccessGrantsInstanceForPrefixResult {
    #[serde(rename = "AccessGrantsInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_grants_instance_arn: Option<String>,
    #[serde(rename = "AccessGrantsInstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_grants_instance_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateAccessGrantsLocationRequest")]
pub struct UpdateAccessGrantsLocationRequest {
    #[serde(rename = "AccessGrantsLocationId")]
    #[serde(default)]
    pub access_grants_location_id: String,
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "IAMRoleArn")]
    #[serde(default)]
    pub i_a_m_role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateAccessGrantsLocationResult")]
pub struct CreateAccessGrantsLocationResult {
    #[serde(rename = "AccessGrantsLocationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_grants_location_arn: Option<String>,
    #[serde(rename = "AccessGrantsLocationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_grants_location_id: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "IAMRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_a_m_role_arn: Option<String>,
    #[serde(rename = "LocationScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_scope: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetAccessGrantsLocationResult")]
pub struct GetAccessGrantsLocationResult {
    #[serde(rename = "AccessGrantsLocationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_grants_location_arn: Option<String>,
    #[serde(rename = "AccessGrantsLocationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_grants_location_id: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "IAMRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_a_m_role_arn: Option<String>,
    #[serde(rename = "LocationScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_scope: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetPublicAccessBlockOutput")]
pub struct GetPublicAccessBlockOutput {
    #[serde(rename = "PublicAccessBlockConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access_block_configuration: Option<PublicAccessBlockConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTagsForResourceRequest")]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutAccessPointPolicyRequest")]
pub struct PutAccessPointPolicyRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Policy")]
    #[serde(default)]
    pub policy: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutStorageLensConfigurationRequest")]
pub struct PutStorageLensConfigurationRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "ConfigId")]
    #[serde(default)]
    pub config_id: String,
    #[serde(rename = "StorageLensConfiguration")]
    #[serde(default)]
    pub storage_lens_configuration: StorageLensConfiguration,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<StorageLensTags>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateJobStatusResult")]
pub struct UpdateJobStatusResult {
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusUpdateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_update_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutBucketTaggingRequest")]
pub struct PutBucketTaggingRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "Tagging")]
    #[serde(default)]
    pub tagging: Tagging,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetAccessPointResult")]
pub struct GetAccessPointResult {
    #[serde(rename = "AccessPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_arn: Option<String>,
    #[serde(rename = "Alias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(rename = "BucketAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_account_id: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "DataSourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,
    #[serde(rename = "DataSourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_type: Option<String>,
    #[serde(rename = "Endpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NetworkOrigin")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_origin: Option<String>,
    #[serde(rename = "PublicAccessBlockConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access_block_configuration: Option<PublicAccessBlockConfiguration>,
    #[serde(rename = "VpcConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_configuration: Option<VpcConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListAccessGrantsRequest")]
pub struct ListAccessGrantsRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
    #[serde(rename = "GrantScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_scope: Option<String>,
    #[serde(rename = "GranteeIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grantee_identifier: Option<String>,
    #[serde(rename = "GranteeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grantee_type: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Permission")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteMultiRegionAccessPointResult")]
pub struct DeleteMultiRegionAccessPointResult {
    #[serde(rename = "RequestTokenARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_token_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetAccessGrantsLocationRequest")]
pub struct GetAccessGrantsLocationRequest {
    #[serde(rename = "AccessGrantsLocationId")]
    #[serde(default)]
    pub access_grants_location_id: String,
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeJobRequest")]
pub struct DescribeJobRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetPublicAccessBlockRequest")]
pub struct GetPublicAccessBlockRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListStorageLensConfigurationResult")]
pub struct ListStorageLensConfigurationsResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StorageLensConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_lens_configuration_list: Option<Vec<ListStorageLensConfigurationEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListStorageLensConfigurationEntry")]
pub struct ListStorageLensConfigurationEntry {
    #[serde(rename = "HomeRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_region: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[serde(rename = "StorageLensArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_lens_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketPolicyResult")]
pub struct GetBucketPolicyResult {
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetMultiRegionAccessPointRoutesRequest")]
pub struct GetMultiRegionAccessPointRoutesRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Mrap")]
    #[serde(default)]
    pub mrap: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteStorageLensConfigurationRequest")]
pub struct DeleteStorageLensConfigurationRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "ConfigId")]
    #[serde(default)]
    pub config_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetDataAccessResult")]
pub struct GetDataAccessResult {
    #[serde(rename = "Credentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Credentials>,
    #[serde(rename = "Grantee")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grantee: Option<Grantee>,
    #[serde(rename = "MatchedGrantTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matched_grant_target: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Credentials")]
pub struct Credentials {
    #[serde(rename = "AccessKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    #[serde(rename = "Expiration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    #[serde(rename = "SecretAccessKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,
    #[serde(rename = "SessionToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketTaggingResult")]
pub struct GetBucketTaggingResult {
    #[serde(rename = "TagSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_set: Option<S3TagSet>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTagsForResourceResult")]
pub struct ListTagsForResourceResult {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutJobTaggingResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeMultiRegionAccessPointOperationRequest")]
pub struct DescribeMultiRegionAccessPointOperationRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "RequestTokenARN")]
    #[serde(default)]
    pub request_token_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateJobPriorityResult")]
pub struct UpdateJobPriorityResult {
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutMultiRegionAccessPointPolicyResult")]
pub struct PutMultiRegionAccessPointPolicyResult {
    #[serde(rename = "RequestTokenARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_token_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListAccessPointsForObjectLambdaResult")]
pub struct ListAccessPointsForObjectLambdaResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ObjectLambdaAccessPointList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_lambda_access_point_list: Option<ObjectLambdaAccessPointList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ObjectLambdaAccessPointList {
    #[serde(
        rename = "ObjectLambdaAccessPoint",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ObjectLambdaAccessPoint>,
}
impl From<Vec<ObjectLambdaAccessPoint>> for ObjectLambdaAccessPointList {
    fn from(v: Vec<ObjectLambdaAccessPoint>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ObjectLambdaAccessPoint> for ObjectLambdaAccessPointList {
    fn from_iter<I: IntoIterator<Item = ObjectLambdaAccessPoint>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ObjectLambdaAccessPoint>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlObjectLambdaAccessPointList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ObjectLambdaAccessPoint>,
}

impl From<Vec<ObjectLambdaAccessPoint>> for XmlObjectLambdaAccessPointList {
    fn from(v: Vec<ObjectLambdaAccessPoint>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ObjectLambdaAccessPoint> for XmlObjectLambdaAccessPointList {
    fn from_iter<I: IntoIterator<Item = ObjectLambdaAccessPoint>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ObjectLambdaAccessPoint")]
pub struct ObjectLambdaAccessPoint {
    #[serde(rename = "Alias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<ObjectLambdaAccessPointAlias>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ObjectLambdaAccessPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_lambda_access_point_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateAccessPointResult")]
pub struct CreateAccessPointResult {
    #[serde(rename = "AccessPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_arn: Option<String>,
    #[serde(rename = "Alias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetAccessPointPolicyStatusForObjectLambdaResult")]
pub struct GetAccessPointPolicyStatusForObjectLambdaResult {
    #[serde(rename = "PolicyStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_status: Option<PolicyStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateJobStatusRequest")]
pub struct UpdateJobStatusRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
    #[serde(rename = "RequestedJobStatus")]
    #[serde(default)]
    pub requested_job_status: String,
    #[serde(rename = "StatusUpdateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_update_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetAccessPointRequest")]
pub struct GetAccessPointRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateAccessGrantRequest")]
pub struct CreateAccessGrantRequest {
    #[serde(rename = "AccessGrantsLocationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_grants_location_configuration: Option<AccessGrantsLocationConfiguration>,
    #[serde(rename = "AccessGrantsLocationId")]
    #[serde(default)]
    pub access_grants_location_id: String,
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
    #[serde(rename = "Grantee")]
    #[serde(default)]
    pub grantee: Grantee,
    #[serde(rename = "Permission")]
    #[serde(default)]
    pub permission: String,
    #[serde(rename = "S3PrefixType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_prefix_type: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteAccessPointForObjectLambdaRequest")]
pub struct DeleteAccessPointForObjectLambdaRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetDataAccessRequest")]
pub struct GetDataAccessRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "DurationSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i32>,
    #[serde(rename = "Permission")]
    #[serde(default)]
    pub permission: String,
    #[serde(rename = "Privilege")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privilege: Option<String>,
    #[serde(rename = "Target")]
    #[serde(default)]
    pub target: String,
    #[serde(rename = "TargetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketTaggingRequest")]
pub struct GetBucketTaggingRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListJobsRequest")]
pub struct ListJobsRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "JobStatuses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_statuses: Option<JobStatusList>,
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
pub struct JobStatusList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for JobStatusList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for JobStatusList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketVersioningResult")]
pub struct GetBucketVersioningResult {
    #[serde(rename = "MfaDelete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_f_a_delete: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteAccessGrantRequest")]
pub struct DeleteAccessGrantRequest {
    #[serde(rename = "AccessGrantId")]
    #[serde(default)]
    pub access_grant_id: String,
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
}
