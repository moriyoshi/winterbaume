//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-s3

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CORSConfiguration")]
pub struct CORSConfiguration {
    #[serde(rename = "CORSRule")]
    #[serde(default)]
    pub c_o_r_s_rules: Vec<CORSRule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CORSRule")]
pub struct CORSRule {
    #[serde(rename = "AllowedHeader")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_headers: Option<Vec<String>>,
    #[serde(rename = "AllowedMethod")]
    #[serde(default)]
    pub allowed_methods: Vec<String>,
    #[serde(rename = "AllowedOrigin")]
    #[serde(default)]
    pub allowed_origins: Vec<String>,
    #[serde(rename = "ExposeHeader")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expose_headers: Option<Vec<String>>,
    #[serde(rename = "ID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_d: Option<String>,
    #[serde(rename = "MaxAgeSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutObjectLegalHoldOutput")]
pub struct PutObjectLegalHoldOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetPublicAccessBlockRequest")]
pub struct GetPublicAccessBlockRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeletePublicAccessBlockRequest")]
pub struct DeletePublicAccessBlockRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RestoreRequest")]
pub struct RestoreRequest {
    #[serde(rename = "Days")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i32>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "GlacierJobParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glacier_job_parameters: Option<GlacierJobParameters>,
    #[serde(rename = "OutputLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location: Option<OutputLocation>,
    #[serde(rename = "SelectParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_parameters: Option<SelectParameters>,
    #[serde(rename = "Tier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GlacierJobParameters")]
pub struct GlacierJobParameters {
    #[serde(rename = "Tier")]
    #[serde(default)]
    pub tier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OutputLocation")]
pub struct OutputLocation {
    #[serde(rename = "S3")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3: Option<S3Location>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "S3Location")]
pub struct S3Location {
    #[serde(rename = "AccessControlList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_list: Option<Grants>,
    #[serde(rename = "BucketName")]
    #[serde(default)]
    pub bucket_name: String,
    #[serde(rename = "CannedACL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canned_a_c_l: Option<String>,
    #[serde(rename = "Encryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    pub prefix: String,
    #[serde(rename = "StorageClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
    #[serde(rename = "Tagging")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tagging: Option<Tagging>,
    #[serde(rename = "UserMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_metadata: Option<UserMetadata>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Grants {
    #[serde(rename = "Grant", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Grant>,
}
impl From<Vec<Grant>> for Grants {
    fn from(v: Vec<Grant>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Grant> for Grants {
    fn from_iter<I: IntoIterator<Item = Grant>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Grant>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlGrantList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Grant>,
}

impl From<Vec<Grant>> for XmlGrantList {
    fn from(v: Vec<Grant>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Grant> for XmlGrantList {
    fn from_iter<I: IntoIterator<Item = Grant>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Grant")]
pub struct Grant {
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
#[serde(rename = "Grantee")]
pub struct Grantee {
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "EmailAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(rename = "ID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_d: Option<String>,
    #[serde(rename = "xsi:type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "URI")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_r_i: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Encryption")]
pub struct Encryption {
    #[serde(rename = "EncryptionType")]
    #[serde(default)]
    pub encryption_type: String,
    #[serde(rename = "KMSContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_context: Option<String>,
    #[serde(rename = "KMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_key_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Tagging")]
pub struct Tagging {
    #[serde(rename = "TagSet")]
    #[serde(default)]
    pub tag_set: TagSet,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagSet {
    #[serde(rename = "Tag", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Tag>,
}
impl From<Vec<Tag>> for TagSet {
    fn from(v: Vec<Tag>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Tag> for TagSet {
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
pub struct UserMetadata {
    #[serde(
        rename = "MetadataEntry",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<MetadataEntry>,
}
impl From<Vec<MetadataEntry>> for UserMetadata {
    fn from(v: Vec<MetadataEntry>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<MetadataEntry> for UserMetadata {
    fn from_iter<I: IntoIterator<Item = MetadataEntry>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<MetadataEntry>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlMetadataEntryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<MetadataEntry>,
}

impl From<Vec<MetadataEntry>> for XmlMetadataEntryList {
    fn from(v: Vec<MetadataEntry>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<MetadataEntry> for XmlMetadataEntryList {
    fn from_iter<I: IntoIterator<Item = MetadataEntry>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MetadataEntry")]
pub struct MetadataEntry {
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
#[serde(rename = "SelectParameters")]
pub struct SelectParameters {
    #[serde(rename = "Expression")]
    #[serde(default)]
    pub expression: String,
    #[serde(rename = "ExpressionType")]
    #[serde(default)]
    pub expression_type: String,
    #[serde(rename = "InputSerialization")]
    #[serde(default)]
    pub input_serialization: InputSerialization,
    #[serde(rename = "OutputSerialization")]
    #[serde(default)]
    pub output_serialization: OutputSerialization,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "InputSerialization")]
pub struct InputSerialization {
    #[serde(rename = "CSV")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_s_v: Option<CSVInput>,
    #[serde(rename = "CompressionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression_type: Option<String>,
    #[serde(rename = "JSON")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub j_s_o_n: Option<JSONInput>,
    #[serde(rename = "Parquet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parquet: Option<ParquetInput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CSVInput")]
pub struct CSVInput {
    #[serde(rename = "AllowQuotedRecordDelimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_quoted_record_delimiter: Option<bool>,
    #[serde(rename = "Comments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "FieldDelimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_delimiter: Option<String>,
    #[serde(rename = "FileHeaderInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_header_info: Option<String>,
    #[serde(rename = "QuoteCharacter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_character: Option<String>,
    #[serde(rename = "QuoteEscapeCharacter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_escape_character: Option<String>,
    #[serde(rename = "RecordDelimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_delimiter: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "JSONInput")]
pub struct JSONInput {
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParquetInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OutputSerialization")]
pub struct OutputSerialization {
    #[serde(rename = "CSV")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_s_v: Option<CSVOutput>,
    #[serde(rename = "JSON")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub j_s_o_n: Option<JSONOutput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CSVOutput")]
pub struct CSVOutput {
    #[serde(rename = "FieldDelimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_delimiter: Option<String>,
    #[serde(rename = "QuoteCharacter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_character: Option<String>,
    #[serde(rename = "QuoteEscapeCharacter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_escape_character: Option<String>,
    #[serde(rename = "QuoteFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_fields: Option<String>,
    #[serde(rename = "RecordDelimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_delimiter: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "JSONOutput")]
pub struct JSONOutput {
    #[serde(rename = "RecordDelimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_delimiter: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "BucketLoggingStatus")]
pub struct GetBucketLoggingOutput {
    #[serde(rename = "LoggingEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_enabled: Option<LoggingEnabled>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LoggingEnabled")]
pub struct LoggingEnabled {
    #[serde(rename = "TargetBucket")]
    #[serde(default)]
    pub target_bucket: String,
    #[serde(rename = "TargetGrants")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_grants: Option<TargetGrants>,
    #[serde(rename = "TargetObjectKeyFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_object_key_format: Option<TargetObjectKeyFormat>,
    #[serde(rename = "TargetPrefix")]
    #[serde(default)]
    pub target_prefix: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetGrants {
    #[serde(rename = "Grant", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<TargetGrant>,
}
impl From<Vec<TargetGrant>> for TargetGrants {
    fn from(v: Vec<TargetGrant>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<TargetGrant> for TargetGrants {
    fn from_iter<I: IntoIterator<Item = TargetGrant>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<TargetGrant>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTargetGrantList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<TargetGrant>,
}

impl From<Vec<TargetGrant>> for XmlTargetGrantList {
    fn from(v: Vec<TargetGrant>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<TargetGrant> for XmlTargetGrantList {
    fn from_iter<I: IntoIterator<Item = TargetGrant>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TargetGrant")]
pub struct TargetGrant {
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
#[serde(rename = "TargetObjectKeyFormat")]
pub struct TargetObjectKeyFormat {
    #[serde(rename = "PartitionedPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partitioned_prefix: Option<PartitionedPrefix>,
    #[serde(rename = "SimplePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_prefix: Option<SimplePrefix>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PartitionedPrefix")]
pub struct PartitionedPrefix {
    #[serde(rename = "PartitionDateSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_date_source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SimplePrefix {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketMetadataTableConfigurationRequest")]
pub struct GetBucketMetadataTableConfigurationRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "IntelligentTieringConfiguration")]
pub struct IntelligentTieringConfiguration {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<IntelligentTieringFilter>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
    #[serde(rename = "Tiering")]
    #[serde(default)]
    pub tierings: Vec<Tiering>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "IntelligentTieringFilter")]
pub struct IntelligentTieringFilter {
    #[serde(rename = "And")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and: Option<IntelligentTieringAndOperator>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "Tag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "IntelligentTieringAndOperator")]
pub struct IntelligentTieringAndOperator {
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "Tag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Tiering")]
pub struct Tiering {
    #[serde(rename = "AccessTier")]
    #[serde(default)]
    pub access_tier: String,
    #[serde(rename = "Days")]
    #[serde(default)]
    pub days: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "HeadObjectRequest")]
pub struct HeadObjectRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ChecksumMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_mode: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match: Option<String>,
    #[serde(rename = "IfModifiedSince")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_modified_since: Option<String>,
    #[serde(rename = "IfNoneMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_none_match: Option<String>,
    #[serde(rename = "IfUnmodifiedSince")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_unmodified_since: Option<String>,
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "PartNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_number: Option<i32>,
    #[serde(rename = "Range")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
    #[serde(rename = "RequestPayer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_payer: Option<String>,
    #[serde(rename = "ResponseCacheControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_cache_control: Option<String>,
    #[serde(rename = "ResponseContentDisposition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_content_disposition: Option<String>,
    #[serde(rename = "ResponseContentEncoding")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_content_encoding: Option<String>,
    #[serde(rename = "ResponseContentLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_content_language: Option<String>,
    #[serde(rename = "ResponseContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_content_type: Option<String>,
    #[serde(rename = "ResponseExpires")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_expires: Option<String>,
    #[serde(rename = "SSECustomerAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_customer_algorithm: Option<String>,
    #[serde(rename = "SSECustomerKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_customer_key: Option<String>,
    #[serde(rename = "SSECustomerKeyMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_customer_key_m_d5: Option<String>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CopyObjectRequest")]
pub struct CopyObjectRequest {
    #[serde(rename = "ACL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_c_l: Option<String>,
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "BucketKeyEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_key_enabled: Option<bool>,
    #[serde(rename = "CacheControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<String>,
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
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
    #[serde(rename = "ContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "CopySource")]
    #[serde(default)]
    pub copy_source: String,
    #[serde(rename = "CopySourceIfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_source_if_match: Option<String>,
    #[serde(rename = "CopySourceIfModifiedSince")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_source_if_modified_since: Option<String>,
    #[serde(rename = "CopySourceIfNoneMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_source_if_none_match: Option<String>,
    #[serde(rename = "CopySourceIfUnmodifiedSince")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_source_if_unmodified_since: Option<String>,
    #[serde(rename = "CopySourceSSECustomerAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_source_s_s_e_customer_algorithm: Option<String>,
    #[serde(rename = "CopySourceSSECustomerKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_source_s_s_e_customer_key: Option<String>,
    #[serde(rename = "CopySourceSSECustomerKeyMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_source_s_s_e_customer_key_m_d5: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "ExpectedSourceBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_source_bucket_owner: Option<String>,
    #[serde(rename = "Expires")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
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
    #[serde(rename = "GrantWriteACP")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_write_a_c_p: Option<String>,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match: Option<String>,
    #[serde(rename = "IfNoneMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_none_match: Option<String>,
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Metadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "MetadataDirective")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_directive: Option<String>,
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
    #[serde(rename = "RequestPayer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_payer: Option<String>,
    #[serde(rename = "SSECustomerAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_customer_algorithm: Option<String>,
    #[serde(rename = "SSECustomerKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_customer_key: Option<String>,
    #[serde(rename = "SSECustomerKeyMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_customer_key_m_d5: Option<String>,
    #[serde(rename = "SSEKMSEncryptionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_k_m_s_encryption_context: Option<String>,
    #[serde(rename = "SSEKMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_k_m_s_key_id: Option<String>,
    #[serde(rename = "ServerSideEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_side_encryption: Option<String>,
    #[serde(rename = "StorageClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
    #[serde(rename = "Tagging")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tagging: Option<String>,
    #[serde(rename = "TaggingDirective")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tagging_directive: Option<String>,
    #[serde(rename = "WebsiteRedirectLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_redirect_location: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetObjectAttributesRequest")]
pub struct GetObjectAttributesRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "MaxParts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_parts: Option<i32>,
    #[serde(rename = "ObjectAttributes")]
    #[serde(default)]
    pub object_attributes: ObjectAttributesList,
    #[serde(rename = "PartNumberMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_number_marker: Option<String>,
    #[serde(rename = "RequestPayer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_payer: Option<String>,
    #[serde(rename = "SSECustomerAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_customer_algorithm: Option<String>,
    #[serde(rename = "SSECustomerKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_customer_key: Option<String>,
    #[serde(rename = "SSECustomerKeyMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_customer_key_m_d5: Option<String>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ObjectAttributesList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ObjectAttributesList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ObjectAttributesList {
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
#[serde(rename = "GetObjectTorrentRequest")]
pub struct GetObjectTorrentRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "RequestPayer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_payer: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "HeadBucketOutput")]
pub struct HeadBucketOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListVersionsResult")]
pub struct ListObjectVersionsOutput {
    #[serde(rename = "CommonPrefixes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_prefixes: Option<Vec<CommonPrefix>>,
    #[serde(rename = "DeleteMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_markers: Option<Vec<DeleteMarkerEntry>>,
    #[serde(rename = "Delimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    #[serde(rename = "EncodingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_type: Option<String>,
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "KeyMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_marker: Option<String>,
    #[serde(rename = "MaxKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_keys: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NextKeyMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_key_marker: Option<String>,
    #[serde(rename = "NextVersionIdMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_version_id_marker: Option<String>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "VersionIdMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id_marker: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<ObjectVersion>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CommonPrefix")]
pub struct CommonPrefix {
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteMarkerEntry")]
pub struct DeleteMarkerEntry {
    #[serde(rename = "IsLatest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_latest: Option<bool>,
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "LastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "Owner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<Owner>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Owner")]
pub struct Owner {
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
#[serde(rename = "ObjectVersion")]
pub struct ObjectVersion {
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<Vec<String>>,
    #[serde(rename = "ChecksumType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_type: Option<String>,
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "IsLatest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_latest: Option<bool>,
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "LastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "Owner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<Owner>,
    #[serde(rename = "RestoreStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_status: Option<RestoreStatus>,
    #[serde(rename = "Size")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "StorageClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RestoreStatus")]
pub struct RestoreStatus {
    #[serde(rename = "IsRestoreInProgress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_restore_in_progress: Option<bool>,
    #[serde(rename = "RestoreExpiryDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_expiry_date: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketMetadataConfigurationResult")]
pub struct GetBucketMetadataConfigurationResult {
    #[serde(rename = "MetadataConfigurationResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_configuration_result: Option<MetadataConfigurationResult>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MetadataConfigurationResult")]
pub struct MetadataConfigurationResult {
    #[serde(rename = "DestinationResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_result: Option<DestinationResult>,
    #[serde(rename = "InventoryTableConfigurationResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_table_configuration_result: Option<InventoryTableConfigurationResult>,
    #[serde(rename = "JournalTableConfigurationResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal_table_configuration_result: Option<JournalTableConfigurationResult>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DestinationResult")]
pub struct DestinationResult {
    #[serde(rename = "TableBucketArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_bucket_arn: Option<String>,
    #[serde(rename = "TableBucketType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_bucket_type: Option<String>,
    #[serde(rename = "TableNamespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_namespace: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "InventoryTableConfigurationResult")]
pub struct InventoryTableConfigurationResult {
    #[serde(rename = "ConfigurationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_state: Option<String>,
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,
    #[serde(rename = "TableArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_arn: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "TableStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ErrorDetails")]
pub struct ErrorDetails {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "JournalTableConfigurationResult")]
pub struct JournalTableConfigurationResult {
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,
    #[serde(rename = "RecordExpiration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_expiration: Option<RecordExpiration>,
    #[serde(rename = "TableArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_arn: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "TableStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RecordExpiration")]
pub struct RecordExpiration {
    #[serde(rename = "Days")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i32>,
    #[serde(rename = "Expiration")]
    #[serde(default)]
    pub expiration: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutBucketEncryptionRequest")]
pub struct PutBucketEncryptionRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
    #[serde(rename = "ContentMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_m_d5: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "ServerSideEncryptionConfiguration")]
    #[serde(default)]
    pub server_side_encryption_configuration: ServerSideEncryptionConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ServerSideEncryptionConfiguration")]
pub struct ServerSideEncryptionConfiguration {
    #[serde(rename = "Rule")]
    #[serde(default)]
    pub rules: Vec<ServerSideEncryptionRule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ServerSideEncryptionRule")]
pub struct ServerSideEncryptionRule {
    #[serde(rename = "ApplyServerSideEncryptionByDefault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_server_side_encryption_by_default: Option<ServerSideEncryptionByDefault>,
    #[serde(rename = "BlockedEncryptionTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_encryption_types: Option<BlockedEncryptionTypes>,
    #[serde(rename = "BucketKeyEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_key_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ServerSideEncryptionByDefault")]
pub struct ServerSideEncryptionByDefault {
    #[serde(rename = "KMSMasterKeyID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_master_key_i_d: Option<String>,
    #[serde(rename = "SSEAlgorithm")]
    #[serde(default)]
    pub s_s_e_algorithm: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "BlockedEncryptionTypes")]
pub struct BlockedEncryptionTypes {
    #[serde(rename = "EncryptionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<Vec<String>>,
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
#[serde(rename = "PutBucketInventoryConfigurationRequest")]
pub struct PutBucketInventoryConfigurationRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "InventoryConfiguration")]
    #[serde(default)]
    pub inventory_configuration: InventoryConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "InventoryConfiguration")]
pub struct InventoryConfiguration {
    #[serde(rename = "Destination")]
    #[serde(default)]
    pub destination: InventoryDestination,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<InventoryFilter>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IncludedObjectVersions")]
    #[serde(default)]
    pub included_object_versions: String,
    #[serde(rename = "IsEnabled")]
    #[serde(default)]
    pub is_enabled: bool,
    #[serde(rename = "OptionalFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_fields: Option<InventoryOptionalFields>,
    #[serde(rename = "Schedule")]
    #[serde(default)]
    pub schedule: InventorySchedule,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "InventoryDestination")]
pub struct InventoryDestination {
    #[serde(rename = "S3BucketDestination")]
    #[serde(default)]
    pub s3_bucket_destination: InventoryS3BucketDestination,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "InventoryS3BucketDestination")]
pub struct InventoryS3BucketDestination {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "Encryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<InventoryEncryption>,
    #[serde(rename = "Format")]
    #[serde(default)]
    pub format: String,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "InventoryEncryption")]
pub struct InventoryEncryption {
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
#[serde(rename = "InventoryFilter")]
pub struct InventoryFilter {
    #[serde(rename = "Prefix")]
    #[serde(default)]
    pub prefix: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InventoryOptionalFields {
    #[serde(rename = "Field", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for InventoryOptionalFields {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for InventoryOptionalFields {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "InventorySchedule")]
pub struct InventorySchedule {
    #[serde(rename = "Frequency")]
    #[serde(default)]
    pub frequency: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteBucketRequest")]
pub struct DeleteBucketRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutBucketLifecycleConfigurationRequest")]
pub struct PutBucketLifecycleConfigurationRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "LifecycleConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_configuration: Option<BucketLifecycleConfiguration>,
    #[serde(rename = "TransitionDefaultMinimumObjectSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_default_minimum_object_size: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "BucketLifecycleConfiguration")]
pub struct BucketLifecycleConfiguration {
    #[serde(rename = "Rule")]
    #[serde(default)]
    pub rules: Vec<LifecycleRule>,
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
    #[serde(rename = "NoncurrentVersionTransition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noncurrent_version_transitions: Option<Vec<NoncurrentVersionTransition>>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
    #[serde(rename = "Transition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transitions: Option<Vec<Transition>>,
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
    pub tag: Option<Tag>,
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
    #[serde(rename = "Tag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
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
#[serde(rename = "NoncurrentVersionTransition")]
pub struct NoncurrentVersionTransition {
    #[serde(rename = "NewerNoncurrentVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newer_noncurrent_versions: Option<i32>,
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
#[serde(rename = "GetBucketPolicyStatusRequest")]
pub struct GetBucketPolicyStatusRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AccessControlPolicy")]
pub struct GetBucketAclOutput {
    #[serde(rename = "AccessControlList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grants: Option<Grants>,
    #[serde(rename = "Owner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<Owner>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LifecycleConfiguration")]
pub struct GetBucketLifecycleConfigurationOutput {
    #[serde(rename = "Rule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<LifecycleRule>>,
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
    #[serde(rename = "BucketNamespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_namespace: Option<String>,
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
    #[serde(rename = "ObjectOwnership")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_ownership: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateBucketConfiguration")]
pub struct CreateBucketConfiguration {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<BucketInfo>,
    #[serde(rename = "Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<LocationInfo>,
    #[serde(rename = "LocationConstraint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_constraint: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagSet>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "BucketInfo")]
pub struct BucketInfo {
    #[serde(rename = "DataRedundancy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_redundancy: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LocationInfo")]
pub struct LocationInfo {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateBucketMetadataTableConfigurationRequest")]
pub struct CreateBucketMetadataTableConfigurationRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
    #[serde(rename = "ContentMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_m_d5: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "MetadataTableConfiguration")]
    #[serde(default)]
    pub metadata_table_configuration: MetadataTableConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MetadataTableConfiguration")]
pub struct MetadataTableConfiguration {
    #[serde(rename = "S3TablesDestination")]
    #[serde(default)]
    pub s3_tables_destination: S3TablesDestination,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "S3TablesDestination")]
pub struct S3TablesDestination {
    #[serde(rename = "TableBucketArn")]
    #[serde(default)]
    pub table_bucket_arn: String,
    #[serde(rename = "TableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketWebsiteRequest")]
pub struct GetBucketWebsiteRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListMultipartUploadsRequest")]
pub struct ListMultipartUploadsRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "Delimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    #[serde(rename = "EncodingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_type: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "KeyMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_marker: Option<String>,
    #[serde(rename = "MaxUploads")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_uploads: Option<i32>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "RequestPayer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_payer: Option<String>,
    #[serde(rename = "UploadIdMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_id_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutBucketMetricsConfigurationRequest")]
pub struct PutBucketMetricsConfigurationRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "MetricsConfiguration")]
    #[serde(default)]
    pub metrics_configuration: MetricsConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MetricsConfiguration")]
pub struct MetricsConfiguration {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<MetricsFilter>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MetricsFilter")]
pub struct MetricsFilter {
    #[serde(rename = "AccessPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_arn: Option<String>,
    #[serde(rename = "And")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and: Option<MetricsAndOperator>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "Tag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MetricsAndOperator")]
pub struct MetricsAndOperator {
    #[serde(rename = "AccessPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_arn: Option<String>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "Tag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutBucketTaggingRequest")]
pub struct PutBucketTaggingRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
    #[serde(rename = "ContentMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_m_d5: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "Tagging")]
    #[serde(default)]
    pub tagging: Tagging,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteBucketMetricsConfigurationRequest")]
pub struct DeleteBucketMetricsConfigurationRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "WebsiteConfiguration")]
pub struct WebsiteConfiguration {
    #[serde(rename = "ErrorDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_document: Option<ErrorDocument>,
    #[serde(rename = "IndexDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_document: Option<IndexDocument>,
    #[serde(rename = "RedirectAllRequestsTo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_all_requests_to: Option<RedirectAllRequestsTo>,
    #[serde(rename = "RoutingRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_rules: Option<RoutingRules>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ErrorDocument")]
pub struct ErrorDocument {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "IndexDocument")]
pub struct IndexDocument {
    #[serde(rename = "Suffix")]
    #[serde(default)]
    pub suffix: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RedirectAllRequestsTo")]
pub struct RedirectAllRequestsTo {
    #[serde(rename = "HostName")]
    #[serde(default)]
    pub host_name: String,
    #[serde(rename = "Protocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoutingRules {
    #[serde(rename = "RoutingRule", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<RoutingRule>,
}
impl From<Vec<RoutingRule>> for RoutingRules {
    fn from(v: Vec<RoutingRule>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<RoutingRule> for RoutingRules {
    fn from_iter<I: IntoIterator<Item = RoutingRule>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<RoutingRule>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlRoutingRuleList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<RoutingRule>,
}

impl From<Vec<RoutingRule>> for XmlRoutingRuleList {
    fn from(v: Vec<RoutingRule>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<RoutingRule> for XmlRoutingRuleList {
    fn from_iter<I: IntoIterator<Item = RoutingRule>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RoutingRule")]
pub struct RoutingRule {
    #[serde(rename = "Condition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<Condition>,
    #[serde(rename = "Redirect")]
    #[serde(default)]
    pub redirect: Redirect,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Condition")]
pub struct Condition {
    #[serde(rename = "HttpErrorCodeReturnedEquals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_error_code_returned_equals: Option<String>,
    #[serde(rename = "KeyPrefixEquals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_prefix_equals: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Redirect")]
pub struct Redirect {
    #[serde(rename = "HostName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    #[serde(rename = "HttpRedirectCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_redirect_code: Option<String>,
    #[serde(rename = "Protocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "ReplaceKeyPrefixWith")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_key_prefix_with: Option<String>,
    #[serde(rename = "ReplaceKeyWith")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_key_with: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutObjectLockConfigurationRequest")]
pub struct PutObjectLockConfigurationRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
    #[serde(rename = "ContentMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_m_d5: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "ObjectLockConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_lock_configuration: Option<ObjectLockConfiguration>,
    #[serde(rename = "RequestPayer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_payer: Option<String>,
    #[serde(rename = "Token")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ObjectLockConfiguration")]
pub struct ObjectLockConfiguration {
    #[serde(rename = "ObjectLockEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_lock_enabled: Option<String>,
    #[serde(rename = "Rule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<ObjectLockRule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ObjectLockRule")]
pub struct ObjectLockRule {
    #[serde(rename = "DefaultRetention")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_retention: Option<DefaultRetention>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DefaultRetention")]
pub struct DefaultRetention {
    #[serde(rename = "Days")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i32>,
    #[serde(rename = "Mode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "Years")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub years: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SelectObjectContentOutput")]
pub struct SelectObjectContentOutput {
    #[serde(rename = "Payload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<SelectObjectContentEventStream>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SelectObjectContentEventStream")]
pub struct SelectObjectContentEventStream {
    #[serde(rename = "Cont")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cont: Option<ContinuationEvent>,
    #[serde(rename = "End")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<EndEvent>,
    #[serde(rename = "Progress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<ProgressEvent>,
    #[serde(rename = "Records")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<RecordsEvent>,
    #[serde(rename = "Stats")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<StatsEvent>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContinuationEvent {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EndEvent {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ProgressEvent")]
pub struct ProgressEvent {
    #[serde(rename = "Details")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Progress>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Progress")]
pub struct Progress {
    #[serde(rename = "BytesProcessed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_processed: Option<i64>,
    #[serde(rename = "BytesReturned")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_returned: Option<i64>,
    #[serde(rename = "BytesScanned")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_scanned: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RecordsEvent")]
pub struct RecordsEvent {
    #[serde(rename = "Payload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StatsEvent")]
pub struct StatsEvent {
    #[serde(rename = "Details")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Stats>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Stats")]
pub struct Stats {
    #[serde(rename = "BytesProcessed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_processed: Option<i64>,
    #[serde(rename = "BytesReturned")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_returned: Option<i64>,
    #[serde(rename = "BytesScanned")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_scanned: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutObjectAclRequest")]
pub struct PutObjectAclRequest {
    #[serde(rename = "ACL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_c_l: Option<String>,
    #[serde(rename = "AccessControlPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_policy: Option<AccessControlPolicy>,
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
    #[serde(rename = "ContentMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_m_d5: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
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
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "RequestPayer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_payer: Option<String>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AccessControlPolicy")]
pub struct AccessControlPolicy {
    #[serde(rename = "AccessControlList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grants: Option<Grants>,
    #[serde(rename = "Owner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<Owner>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketOwnershipControlsOutput")]
pub struct GetBucketOwnershipControlsOutput {
    #[serde(rename = "OwnershipControls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership_controls: Option<OwnershipControls>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OwnershipControls")]
pub struct OwnershipControls {
    #[serde(rename = "Rule")]
    #[serde(default)]
    pub rules: Vec<OwnershipControlsRule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OwnershipControlsRule")]
pub struct OwnershipControlsRule {
    #[serde(rename = "ObjectOwnership")]
    #[serde(default)]
    pub object_ownership: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteObjectOutput")]
pub struct DeleteObjectOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketEncryptionRequest")]
pub struct GetBucketEncryptionRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketPolicyRequest")]
pub struct GetBucketPolicyRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListBucketsRequest")]
pub struct ListBucketsRequest {
    #[serde(rename = "BucketRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_region: Option<String>,
    #[serde(rename = "ContinuationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    #[serde(rename = "MaxBuckets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_buckets: Option<i32>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteBucketEncryptionRequest")]
pub struct DeleteBucketEncryptionRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListObjectsRequest")]
pub struct ListObjectsRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "Delimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    #[serde(rename = "EncodingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_type: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_keys: Option<i32>,
    #[serde(rename = "OptionalObjectAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_object_attributes: Option<OptionalObjectAttributesList>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "RequestPayer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_payer: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OptionalObjectAttributesList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for OptionalObjectAttributesList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for OptionalObjectAttributesList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutObjectOutput")]
pub struct PutObjectOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketMetricsConfigurationRequest")]
pub struct GetBucketMetricsConfigurationRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutBucketAbacRequest")]
pub struct PutBucketAbacRequest {
    #[serde(rename = "AbacStatus")]
    #[serde(default)]
    pub abac_status: AbacStatus,
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
    #[serde(rename = "ContentMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_m_d5: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AbacStatus")]
pub struct AbacStatus {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RenameObjectRequest")]
pub struct RenameObjectRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "DestinationIfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_if_match: Option<String>,
    #[serde(rename = "DestinationIfModifiedSince")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_if_modified_since: Option<String>,
    #[serde(rename = "DestinationIfNoneMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_if_none_match: Option<String>,
    #[serde(rename = "DestinationIfUnmodifiedSince")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_if_unmodified_since: Option<String>,
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "RenameSource")]
    #[serde(default)]
    pub rename_source: String,
    #[serde(rename = "SourceIfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_if_match: Option<String>,
    #[serde(rename = "SourceIfModifiedSince")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_if_modified_since: Option<String>,
    #[serde(rename = "SourceIfNoneMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_if_none_match: Option<String>,
    #[serde(rename = "SourceIfUnmodifiedSince")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_if_unmodified_since: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketVersioningRequest")]
pub struct GetBucketVersioningRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListBucketResult")]
pub struct ListObjectsOutput {
    #[serde(rename = "CommonPrefixes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_prefixes: Option<Vec<CommonPrefix>>,
    #[serde(rename = "Contents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<Vec<Object>>,
    #[serde(rename = "Delimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    #[serde(rename = "EncodingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_type: Option<String>,
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_keys: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Object")]
pub struct Object {
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<Vec<String>>,
    #[serde(rename = "ChecksumType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_type: Option<String>,
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "LastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "Owner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<Owner>,
    #[serde(rename = "RestoreStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_status: Option<RestoreStatus>,
    #[serde(rename = "Size")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "StorageClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListBucketAnalyticsConfigurationResult")]
pub struct ListBucketAnalyticsConfigurationsOutput {
    #[serde(rename = "AnalyticsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_configuration_list: Option<Vec<AnalyticsConfiguration>>,
    #[serde(rename = "ContinuationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "NextContinuationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_continuation_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AnalyticsConfiguration")]
pub struct AnalyticsConfiguration {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<AnalyticsFilter>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "StorageClassAnalysis")]
    #[serde(default)]
    pub storage_class_analysis: StorageClassAnalysis,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AnalyticsFilter")]
pub struct AnalyticsFilter {
    #[serde(rename = "And")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and: Option<AnalyticsAndOperator>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "Tag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AnalyticsAndOperator")]
pub struct AnalyticsAndOperator {
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "Tag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StorageClassAnalysis")]
pub struct StorageClassAnalysis {
    #[serde(rename = "DataExport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_export: Option<StorageClassAnalysisDataExport>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StorageClassAnalysisDataExport")]
pub struct StorageClassAnalysisDataExport {
    #[serde(rename = "Destination")]
    #[serde(default)]
    pub destination: AnalyticsExportDestination,
    #[serde(rename = "OutputSchemaVersion")]
    #[serde(default)]
    pub output_schema_version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AnalyticsExportDestination")]
pub struct AnalyticsExportDestination {
    #[serde(rename = "S3BucketDestination")]
    #[serde(default)]
    pub s3_bucket_destination: AnalyticsS3BucketDestination,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AnalyticsS3BucketDestination")]
pub struct AnalyticsS3BucketDestination {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "BucketAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_account_id: Option<String>,
    #[serde(rename = "Format")]
    #[serde(default)]
    pub format: String,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketMetadataTableConfigurationResult")]
pub struct GetBucketMetadataTableConfigurationResult {
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetails>,
    #[serde(rename = "MetadataTableConfigurationResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_table_configuration_result: Option<MetadataTableConfigurationResult>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MetadataTableConfigurationResult")]
pub struct MetadataTableConfigurationResult {
    #[serde(rename = "S3TablesDestinationResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_tables_destination_result: Option<S3TablesDestinationResult>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "S3TablesDestinationResult")]
pub struct S3TablesDestinationResult {
    #[serde(rename = "TableArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_arn: Option<String>,
    #[serde(rename = "TableBucketArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_bucket_arn: Option<String>,
    #[serde(rename = "TableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "TableNamespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_namespace: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutBucketIntelligentTieringConfigurationRequest")]
pub struct PutBucketIntelligentTieringConfigurationRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "IntelligentTieringConfiguration")]
    #[serde(default)]
    pub intelligent_tiering_configuration: IntelligentTieringConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "InventoryTableConfigurationUpdates")]
pub struct InventoryTableConfigurationUpdates {
    #[serde(rename = "ConfigurationState")]
    #[serde(default)]
    pub configuration_state: String,
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<MetadataTableEncryptionConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MetadataTableEncryptionConfiguration")]
pub struct MetadataTableEncryptionConfiguration {
    #[serde(rename = "KmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "SseAlgorithm")]
    #[serde(default)]
    pub sse_algorithm: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ReplicationConfiguration")]
pub struct ReplicationConfiguration {
    #[serde(rename = "Role")]
    #[serde(default)]
    pub role: String,
    #[serde(rename = "Rule")]
    #[serde(default)]
    pub rules: Vec<ReplicationRule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ReplicationRule")]
pub struct ReplicationRule {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
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
    pub tag: Option<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ReplicationRuleAndOperator")]
pub struct ReplicationRuleAndOperator {
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "Tag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
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
#[serde(rename = "DeleteBucketOwnershipControlsRequest")]
pub struct DeleteBucketOwnershipControlsRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketAbacOutput")]
pub struct GetBucketAbacOutput {
    #[serde(rename = "AbacStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abac_status: Option<AbacStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListBucketIntelligentTieringConfigurationsRequest")]
pub struct ListBucketIntelligentTieringConfigurationsRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ContinuationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateBucketMetadataJournalTableConfigurationRequest")]
pub struct UpdateBucketMetadataJournalTableConfigurationRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
    #[serde(rename = "ContentMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_m_d5: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "JournalTableConfiguration")]
    #[serde(default)]
    pub journal_table_configuration: JournalTableConfigurationUpdates,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "JournalTableConfigurationUpdates")]
pub struct JournalTableConfigurationUpdates {
    #[serde(rename = "RecordExpiration")]
    #[serde(default)]
    pub record_expiration: RecordExpiration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteBucketCorsRequest")]
pub struct DeleteBucketCorsRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteObjectRequest")]
pub struct DeleteObjectRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "BypassGovernanceRetention")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_governance_retention: Option<bool>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match: Option<String>,
    #[serde(rename = "IfMatchLastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match_last_modified_time: Option<String>,
    #[serde(rename = "IfMatchSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match_size: Option<i64>,
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "MFA")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_f_a: Option<String>,
    #[serde(rename = "RequestPayer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_payer: Option<String>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketAbacRequest")]
pub struct GetBucketAbacRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetObjectRetentionRequest")]
pub struct GetObjectRetentionRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "RequestPayer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_payer: Option<String>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListDirectoryBucketsRequest")]
pub struct ListDirectoryBucketsRequest {
    #[serde(rename = "ContinuationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    #[serde(rename = "MaxDirectoryBuckets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_directory_buckets: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketPolicyOutput")]
pub struct GetBucketPolicyOutput {
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutBucketLifecycleConfigurationOutput")]
pub struct PutBucketLifecycleConfigurationOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutObjectLegalHoldRequest")]
pub struct PutObjectLegalHoldRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
    #[serde(rename = "ContentMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_m_d5: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "LegalHold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_hold: Option<ObjectLockLegalHold>,
    #[serde(rename = "RequestPayer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_payer: Option<String>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ObjectLockLegalHold")]
pub struct ObjectLockLegalHold {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteBucketIntelligentTieringConfigurationRequest")]
pub struct DeleteBucketIntelligentTieringConfigurationRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AbortMultipartUploadOutput")]
pub struct AbortMultipartUploadOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListBucketInventoryConfigurationsRequest")]
pub struct ListBucketInventoryConfigurationsRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ContinuationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AbortMultipartUploadRequest")]
pub struct AbortMultipartUploadRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "IfMatchInitiatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match_initiated_time: Option<String>,
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "RequestPayer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_payer: Option<String>,
    #[serde(rename = "UploadId")]
    #[serde(default)]
    pub upload_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "InitiateMultipartUploadResult")]
pub struct CreateMultipartUploadOutput {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "UploadId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateMultipartUploadRequest")]
pub struct CreateMultipartUploadRequest {
    #[serde(rename = "ACL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_c_l: Option<String>,
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "BucketKeyEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_key_enabled: Option<bool>,
    #[serde(rename = "CacheControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<String>,
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
    #[serde(rename = "ChecksumType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_type: Option<String>,
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
    #[serde(rename = "ContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "Expires")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
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
    #[serde(rename = "GrantWriteACP")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_write_a_c_p: Option<String>,
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Metadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
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
    #[serde(rename = "RequestPayer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_payer: Option<String>,
    #[serde(rename = "SSECustomerAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_customer_algorithm: Option<String>,
    #[serde(rename = "SSECustomerKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_customer_key: Option<String>,
    #[serde(rename = "SSECustomerKeyMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_customer_key_m_d5: Option<String>,
    #[serde(rename = "SSEKMSEncryptionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_k_m_s_encryption_context: Option<String>,
    #[serde(rename = "SSEKMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_k_m_s_key_id: Option<String>,
    #[serde(rename = "ServerSideEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_side_encryption: Option<String>,
    #[serde(rename = "StorageClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
    #[serde(rename = "Tagging")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tagging: Option<String>,
    #[serde(rename = "WebsiteRedirectLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_redirect_location: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetObjectRequest")]
pub struct GetObjectRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ChecksumMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_mode: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match: Option<String>,
    #[serde(rename = "IfModifiedSince")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_modified_since: Option<String>,
    #[serde(rename = "IfNoneMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_none_match: Option<String>,
    #[serde(rename = "IfUnmodifiedSince")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_unmodified_since: Option<String>,
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "PartNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_number: Option<i32>,
    #[serde(rename = "Range")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
    #[serde(rename = "RequestPayer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_payer: Option<String>,
    #[serde(rename = "ResponseCacheControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_cache_control: Option<String>,
    #[serde(rename = "ResponseContentDisposition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_content_disposition: Option<String>,
    #[serde(rename = "ResponseContentEncoding")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_content_encoding: Option<String>,
    #[serde(rename = "ResponseContentLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_content_language: Option<String>,
    #[serde(rename = "ResponseContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_content_type: Option<String>,
    #[serde(rename = "ResponseExpires")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_expires: Option<String>,
    #[serde(rename = "SSECustomerAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_customer_algorithm: Option<String>,
    #[serde(rename = "SSECustomerKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_customer_key: Option<String>,
    #[serde(rename = "SSECustomerKeyMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_customer_key_m_d5: Option<String>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AccessControlPolicy")]
pub struct GetObjectAclOutput {
    #[serde(rename = "AccessControlList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grants: Option<Grants>,
    #[serde(rename = "Owner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<Owner>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListBucketResult")]
pub struct ListObjectsV2Output {
    #[serde(rename = "CommonPrefixes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_prefixes: Option<Vec<CommonPrefix>>,
    #[serde(rename = "Contents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: Option<Vec<Object>>,
    #[serde(rename = "ContinuationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    #[serde(rename = "Delimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    #[serde(rename = "EncodingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_type: Option<String>,
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "KeyCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_count: Option<i32>,
    #[serde(rename = "MaxKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_keys: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NextContinuationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_continuation_token: Option<String>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "StartAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_after: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteBucketLifecycleRequest")]
pub struct DeleteBucketLifecycleRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutBucketPolicyRequest")]
pub struct PutBucketPolicyRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
    #[serde(rename = "ConfirmRemoveSelfBucketAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm_remove_self_bucket_access: Option<bool>,
    #[serde(rename = "ContentMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_m_d5: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "Policy")]
    #[serde(default)]
    pub policy: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutObjectAclOutput")]
pub struct PutObjectAclOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AccelerateConfiguration")]
pub struct GetBucketAccelerateConfigurationOutput {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateBucketOutput")]
pub struct CreateBucketOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CopyObjectResult")]
pub struct CopyObjectResult {
    #[serde(rename = "ChecksumCRC32")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_c_r_c32: Option<String>,
    #[serde(rename = "ChecksumCRC32C")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_c_r_c32_c: Option<String>,
    #[serde(rename = "ChecksumCRC64NVME")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_c_r_c64_n_v_m_e: Option<String>,
    #[serde(rename = "ChecksumSHA1")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_s_h_a1: Option<String>,
    #[serde(rename = "ChecksumSHA256")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_s_h_a256: Option<String>,
    #[serde(rename = "ChecksumType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_type: Option<String>,
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "LastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "HeadBucketRequest")]
pub struct HeadBucketRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListObjectsV2Request")]
pub struct ListObjectsV2Request {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ContinuationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    #[serde(rename = "Delimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    #[serde(rename = "EncodingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_type: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "FetchOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fetch_owner: Option<bool>,
    #[serde(rename = "MaxKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_keys: Option<i32>,
    #[serde(rename = "OptionalObjectAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_object_attributes: Option<OptionalObjectAttributesList>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "RequestPayer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_payer: Option<String>,
    #[serde(rename = "StartAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_after: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RestoreObjectRequest")]
pub struct RestoreObjectRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "RequestPayer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_payer: Option<String>,
    #[serde(rename = "RestoreRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_request: Option<RestoreRequest>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteBucketReplicationRequest")]
pub struct DeleteBucketReplicationRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteBucketPolicyRequest")]
pub struct DeleteBucketPolicyRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketEncryptionOutput")]
pub struct GetBucketEncryptionOutput {
    #[serde(rename = "ServerSideEncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_side_encryption_configuration: Option<ServerSideEncryptionConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketRequestPaymentRequest")]
pub struct GetBucketRequestPaymentRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetObjectLockConfigurationOutput")]
pub struct GetObjectLockConfigurationOutput {
    #[serde(rename = "ObjectLockConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_lock_configuration: Option<ObjectLockConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListObjectVersionsRequest")]
pub struct ListObjectVersionsRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "Delimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    #[serde(rename = "EncodingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_type: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "KeyMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_marker: Option<String>,
    #[serde(rename = "MaxKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_keys: Option<i32>,
    #[serde(rename = "OptionalObjectAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_object_attributes: Option<OptionalObjectAttributesList>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "RequestPayer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_payer: Option<String>,
    #[serde(rename = "VersionIdMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Tagging")]
pub struct GetBucketTaggingOutput {
    #[serde(rename = "TagSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_set: Option<TagSet>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "WebsiteConfiguration")]
pub struct GetBucketWebsiteOutput {
    #[serde(rename = "ErrorDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_document: Option<ErrorDocument>,
    #[serde(rename = "IndexDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_document: Option<IndexDocument>,
    #[serde(rename = "RedirectAllRequestsTo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_all_requests_to: Option<RedirectAllRequestsTo>,
    #[serde(rename = "RoutingRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_rules: Option<RoutingRules>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteResult")]
pub struct DeleteObjectsOutput {
    #[serde(rename = "Deleted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<Vec<DeletedObject>>,
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeletedObject")]
pub struct DeletedObject {
    #[serde(rename = "DeleteMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_marker: Option<bool>,
    #[serde(rename = "DeleteMarkerVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_marker_version_id: Option<String>,
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Error")]
pub struct Error {
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutBucketAccelerateConfigurationRequest")]
pub struct PutBucketAccelerateConfigurationRequest {
    #[serde(rename = "AccelerateConfiguration")]
    #[serde(default)]
    pub accelerate_configuration: AccelerateConfiguration,
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AccelerateConfiguration")]
pub struct AccelerateConfiguration {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteBucketWebsiteRequest")]
pub struct DeleteBucketWebsiteRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutBucketWebsiteRequest")]
pub struct PutBucketWebsiteRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
    #[serde(rename = "ContentMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_m_d5: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "WebsiteConfiguration")]
    #[serde(default)]
    pub website_configuration: WebsiteConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListBucketMetricsConfigurationsRequest")]
pub struct ListBucketMetricsConfigurationsRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ContinuationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteBucketInventoryConfigurationRequest")]
pub struct DeleteBucketInventoryConfigurationRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteBucketMetadataConfigurationRequest")]
pub struct DeleteBucketMetadataConfigurationRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketIntelligentTieringConfigurationOutput")]
pub struct GetBucketIntelligentTieringConfigurationOutput {
    #[serde(rename = "IntelligentTieringConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intelligent_tiering_configuration: Option<IntelligentTieringConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketAnalyticsConfigurationOutput")]
pub struct GetBucketAnalyticsConfigurationOutput {
    #[serde(rename = "AnalyticsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analytics_configuration: Option<AnalyticsConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetObjectRetentionOutput")]
pub struct GetObjectRetentionOutput {
    #[serde(rename = "Retention")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention: Option<ObjectLockRetention>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ObjectLockRetention")]
pub struct ObjectLockRetention {
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
#[serde(rename = "ListBucketAnalyticsConfigurationsRequest")]
pub struct ListBucketAnalyticsConfigurationsRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ContinuationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketNotificationConfigurationRequest")]
pub struct GetBucketNotificationConfigurationRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListBucketIntelligentTieringConfigurationsOutput")]
pub struct ListBucketIntelligentTieringConfigurationsOutput {
    #[serde(rename = "ContinuationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    #[serde(rename = "IntelligentTieringConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intelligent_tiering_configuration_list: Option<Vec<IntelligentTieringConfiguration>>,
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "NextContinuationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_continuation_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketMetadataConfigurationRequest")]
pub struct GetBucketMetadataConfigurationRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CompletedMultipartUpload")]
pub struct CompletedMultipartUpload {
    #[serde(rename = "Part")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parts: Option<Vec<CompletedPart>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CompletedPart")]
pub struct CompletedPart {
    #[serde(rename = "ChecksumCRC32")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_c_r_c32: Option<String>,
    #[serde(rename = "ChecksumCRC32C")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_c_r_c32_c: Option<String>,
    #[serde(rename = "ChecksumCRC64NVME")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_c_r_c64_n_v_m_e: Option<String>,
    #[serde(rename = "ChecksumSHA1")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_s_h_a1: Option<String>,
    #[serde(rename = "ChecksumSHA256")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_s_h_a256: Option<String>,
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "PartNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_number: Option<i32>,
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
#[serde(rename = "Delete")]
pub struct Delete {
    #[serde(rename = "Object")]
    #[serde(default)]
    pub objects: Vec<ObjectIdentifier>,
    #[serde(rename = "Quiet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quiet: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ObjectIdentifier")]
pub struct ObjectIdentifier {
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(rename = "Size")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListInventoryConfigurationsResult")]
pub struct ListBucketInventoryConfigurationsOutput {
    #[serde(rename = "ContinuationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    #[serde(rename = "InventoryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_configuration_list: Option<Vec<InventoryConfiguration>>,
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "NextContinuationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_continuation_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketTaggingRequest")]
pub struct GetBucketTaggingRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListAllMyBucketsResult")]
pub struct ListBucketsOutput {
    #[serde(rename = "Buckets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buckets: Option<Buckets>,
    #[serde(rename = "ContinuationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    #[serde(rename = "Owner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<Owner>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Buckets {
    #[serde(rename = "Bucket", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Bucket>,
}
impl From<Vec<Bucket>> for Buckets {
    fn from(v: Vec<Bucket>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Bucket> for Buckets {
    fn from_iter<I: IntoIterator<Item = Bucket>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Bucket>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlBucketList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Bucket>,
}

impl From<Vec<Bucket>> for XmlBucketList {
    fn from(v: Vec<Bucket>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Bucket> for XmlBucketList {
    fn from_iter<I: IntoIterator<Item = Bucket>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Bucket")]
pub struct Bucket {
    #[serde(rename = "BucketArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_arn: Option<String>,
    #[serde(rename = "BucketRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_region: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketMetadataTableConfigurationOutput")]
pub struct GetBucketMetadataTableConfigurationOutput {
    #[serde(rename = "GetBucketMetadataTableConfigurationResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_bucket_metadata_table_configuration_result:
        Option<GetBucketMetadataTableConfigurationResult>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListPartsRequest")]
pub struct ListPartsRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "MaxParts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_parts: Option<i32>,
    #[serde(rename = "PartNumberMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_number_marker: Option<String>,
    #[serde(rename = "RequestPayer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_payer: Option<String>,
    #[serde(rename = "SSECustomerAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_customer_algorithm: Option<String>,
    #[serde(rename = "SSECustomerKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_customer_key: Option<String>,
    #[serde(rename = "SSECustomerKeyMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_customer_key_m_d5: Option<String>,
    #[serde(rename = "UploadId")]
    #[serde(default)]
    pub upload_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketLifecycleConfigurationRequest")]
pub struct GetBucketLifecycleConfigurationRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutBucketAnalyticsConfigurationRequest")]
pub struct PutBucketAnalyticsConfigurationRequest {
    #[serde(rename = "AnalyticsConfiguration")]
    #[serde(default)]
    pub analytics_configuration: AnalyticsConfiguration,
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutBucketCorsRequest")]
pub struct PutBucketCorsRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "CORSConfiguration")]
    #[serde(default)]
    pub c_o_r_s_configuration: CORSConfiguration,
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
    #[serde(rename = "ContentMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_m_d5: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutObjectRequest")]
pub struct PutObjectRequest {
    #[serde(rename = "ACL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_c_l: Option<String>,
    #[serde(rename = "Body")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "BucketKeyEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_key_enabled: Option<bool>,
    #[serde(rename = "CacheControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<String>,
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
    #[serde(rename = "ChecksumCRC32")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_c_r_c32: Option<String>,
    #[serde(rename = "ChecksumCRC32C")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_c_r_c32_c: Option<String>,
    #[serde(rename = "ChecksumCRC64NVME")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_c_r_c64_n_v_m_e: Option<String>,
    #[serde(rename = "ChecksumSHA1")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_s_h_a1: Option<String>,
    #[serde(rename = "ChecksumSHA256")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_s_h_a256: Option<String>,
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
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "Expires")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
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
    #[serde(rename = "GrantWriteACP")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_write_a_c_p: Option<String>,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match: Option<String>,
    #[serde(rename = "IfNoneMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_none_match: Option<String>,
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Metadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
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
    #[serde(rename = "RequestPayer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_payer: Option<String>,
    #[serde(rename = "SSECustomerAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_customer_algorithm: Option<String>,
    #[serde(rename = "SSECustomerKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_customer_key: Option<String>,
    #[serde(rename = "SSECustomerKeyMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_customer_key_m_d5: Option<String>,
    #[serde(rename = "SSEKMSEncryptionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_k_m_s_encryption_context: Option<String>,
    #[serde(rename = "SSEKMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_k_m_s_key_id: Option<String>,
    #[serde(rename = "ServerSideEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_side_encryption: Option<String>,
    #[serde(rename = "StorageClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
    #[serde(rename = "Tagging")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tagging: Option<String>,
    #[serde(rename = "WebsiteRedirectLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_redirect_location: Option<String>,
    #[serde(rename = "WriteOffsetBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_offset_bytes: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutObjectRetentionOutput")]
pub struct PutObjectRetentionOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutObjectTaggingRequest")]
pub struct PutObjectTaggingRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
    #[serde(rename = "ContentMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_m_d5: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "RequestPayer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_payer: Option<String>,
    #[serde(rename = "Tagging")]
    #[serde(default)]
    pub tagging: Tagging,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RenameObjectOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutObjectLockConfigurationOutput")]
pub struct PutObjectLockConfigurationOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SelectObjectContentRequest")]
pub struct SelectObjectContentRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "Expression")]
    #[serde(default)]
    pub expression: String,
    #[serde(rename = "ExpressionType")]
    #[serde(default)]
    pub expression_type: String,
    #[serde(rename = "InputSerialization")]
    #[serde(default)]
    pub input_serialization: InputSerialization,
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "OutputSerialization")]
    #[serde(default)]
    pub output_serialization: OutputSerialization,
    #[serde(rename = "RequestProgress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_progress: Option<RequestProgress>,
    #[serde(rename = "SSECustomerAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_customer_algorithm: Option<String>,
    #[serde(rename = "SSECustomerKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_customer_key: Option<String>,
    #[serde(rename = "SSECustomerKeyMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_customer_key_m_d5: Option<String>,
    #[serde(rename = "ScanRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_range: Option<ScanRange>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RequestProgress")]
pub struct RequestProgress {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ScanRange")]
pub struct ScanRange {
    #[serde(rename = "End")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<i64>,
    #[serde(rename = "Start")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UploadPartCopyOutput")]
pub struct UploadPartCopyOutput {
    #[serde(rename = "CopyPartResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_part_result: Option<CopyPartResult>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CopyPartResult")]
pub struct CopyPartResult {
    #[serde(rename = "ChecksumCRC32")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_c_r_c32: Option<String>,
    #[serde(rename = "ChecksumCRC32C")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_c_r_c32_c: Option<String>,
    #[serde(rename = "ChecksumCRC64NVME")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_c_r_c64_n_v_m_e: Option<String>,
    #[serde(rename = "ChecksumSHA1")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_s_h_a1: Option<String>,
    #[serde(rename = "ChecksumSHA256")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_s_h_a256: Option<String>,
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "LastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetObjectLockConfigurationRequest")]
pub struct GetObjectLockConfigurationRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MetadataConfiguration")]
pub struct MetadataConfiguration {
    #[serde(rename = "InventoryTableConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_table_configuration: Option<InventoryTableConfiguration>,
    #[serde(rename = "JournalTableConfiguration")]
    #[serde(default)]
    pub journal_table_configuration: JournalTableConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "InventoryTableConfiguration")]
pub struct InventoryTableConfiguration {
    #[serde(rename = "ConfigurationState")]
    #[serde(default)]
    pub configuration_state: String,
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<MetadataTableEncryptionConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "JournalTableConfiguration")]
pub struct JournalTableConfiguration {
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<MetadataTableEncryptionConfiguration>,
    #[serde(rename = "RecordExpiration")]
    #[serde(default)]
    pub record_expiration: RecordExpiration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListPartsResult")]
pub struct ListPartsOutput {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
    #[serde(rename = "ChecksumType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_type: Option<String>,
    #[serde(rename = "Initiator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator: Option<Initiator>,
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "MaxParts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_parts: Option<i32>,
    #[serde(rename = "NextPartNumberMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_part_number_marker: Option<String>,
    #[serde(rename = "Owner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<Owner>,
    #[serde(rename = "PartNumberMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_number_marker: Option<String>,
    #[serde(rename = "Part")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parts: Option<Vec<Part>>,
    #[serde(rename = "StorageClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
    #[serde(rename = "UploadId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Initiator")]
pub struct Initiator {
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
#[serde(rename = "Part")]
pub struct Part {
    #[serde(rename = "ChecksumCRC32")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_c_r_c32: Option<String>,
    #[serde(rename = "ChecksumCRC32C")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_c_r_c32_c: Option<String>,
    #[serde(rename = "ChecksumCRC64NVME")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_c_r_c64_n_v_m_e: Option<String>,
    #[serde(rename = "ChecksumSHA1")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_s_h_a1: Option<String>,
    #[serde(rename = "ChecksumSHA256")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_s_h_a256: Option<String>,
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "LastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "PartNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_number: Option<i32>,
    #[serde(rename = "Size")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketMetricsConfigurationOutput")]
pub struct GetBucketMetricsConfigurationOutput {
    #[serde(rename = "MetricsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_configuration: Option<MetricsConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutBucketRequestPaymentRequest")]
pub struct PutBucketRequestPaymentRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
    #[serde(rename = "ContentMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_m_d5: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "RequestPaymentConfiguration")]
    #[serde(default)]
    pub request_payment_configuration: RequestPaymentConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RequestPaymentConfiguration")]
pub struct RequestPaymentConfiguration {
    #[serde(rename = "Payer")]
    #[serde(default)]
    pub payer: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "NotificationConfiguration")]
pub struct NotificationConfiguration {
    #[serde(rename = "EventBridgeConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bridge_configuration: Option<EventBridgeConfiguration>,
    #[serde(rename = "CloudFunctionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_configurations: Option<Vec<LambdaFunctionConfiguration>>,
    #[serde(rename = "QueueConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_configurations: Option<Vec<QueueConfiguration>>,
    #[serde(rename = "TopicConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_configurations: Option<Vec<TopicConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventBridgeConfiguration {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LambdaFunctionConfiguration")]
pub struct LambdaFunctionConfiguration {
    #[serde(rename = "Event")]
    #[serde(default)]
    pub events: Vec<String>,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<NotificationConfigurationFilter>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "CloudFunction")]
    #[serde(default)]
    pub lambda_function_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "NotificationConfigurationFilter")]
pub struct NotificationConfigurationFilter {
    #[serde(rename = "S3Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<S3KeyFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "S3KeyFilter")]
pub struct S3KeyFilter {
    #[serde(rename = "FilterRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_rules: Option<Vec<FilterRule>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "FilterRule")]
pub struct FilterRule {
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
#[serde(rename = "QueueConfiguration")]
pub struct QueueConfiguration {
    #[serde(rename = "Event")]
    #[serde(default)]
    pub events: Vec<String>,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<NotificationConfigurationFilter>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Queue")]
    #[serde(default)]
    pub queue_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TopicConfiguration")]
pub struct TopicConfiguration {
    #[serde(rename = "Event")]
    #[serde(default)]
    pub events: Vec<String>,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<NotificationConfigurationFilter>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Topic")]
    #[serde(default)]
    pub topic_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RequestPaymentConfiguration")]
pub struct GetBucketRequestPaymentOutput {
    #[serde(rename = "Payer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetObjectOutput")]
pub struct GetObjectOutput {
    #[serde(rename = "Body")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "Metadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetObjectLegalHoldRequest")]
pub struct GetObjectLegalHoldRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "RequestPayer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_payer: Option<String>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetObjectAclRequest")]
pub struct GetObjectAclRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "RequestPayer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_payer: Option<String>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetObjectLegalHoldOutput")]
pub struct GetObjectLegalHoldOutput {
    #[serde(rename = "LegalHold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_hold: Option<ObjectLockLegalHold>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateBucketMetadataConfigurationRequest")]
pub struct CreateBucketMetadataConfigurationRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
    #[serde(rename = "ContentMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_m_d5: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "MetadataConfiguration")]
    #[serde(default)]
    pub metadata_configuration: MetadataConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CompleteMultipartUploadRequest")]
pub struct CompleteMultipartUploadRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ChecksumCRC32")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_c_r_c32: Option<String>,
    #[serde(rename = "ChecksumCRC32C")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_c_r_c32_c: Option<String>,
    #[serde(rename = "ChecksumCRC64NVME")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_c_r_c64_n_v_m_e: Option<String>,
    #[serde(rename = "ChecksumSHA1")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_s_h_a1: Option<String>,
    #[serde(rename = "ChecksumSHA256")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_s_h_a256: Option<String>,
    #[serde(rename = "ChecksumType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_type: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "IfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_match: Option<String>,
    #[serde(rename = "IfNoneMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub if_none_match: Option<String>,
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "MpuObjectSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpu_object_size: Option<i64>,
    #[serde(rename = "CompleteMultipartUpload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multipart_upload: Option<CompletedMultipartUpload>,
    #[serde(rename = "RequestPayer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_payer: Option<String>,
    #[serde(rename = "SSECustomerAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_customer_algorithm: Option<String>,
    #[serde(rename = "SSECustomerKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_customer_key: Option<String>,
    #[serde(rename = "SSECustomerKeyMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_customer_key_m_d5: Option<String>,
    #[serde(rename = "UploadId")]
    #[serde(default)]
    pub upload_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketOwnershipControlsRequest")]
pub struct GetBucketOwnershipControlsRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetObjectTaggingRequest")]
pub struct GetObjectTaggingRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "RequestPayer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_payer: Option<String>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteObjectTaggingOutput")]
pub struct DeleteObjectTaggingOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListMetricsConfigurationsResult")]
pub struct ListBucketMetricsConfigurationsOutput {
    #[serde(rename = "ContinuationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "MetricsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_configuration_list: Option<Vec<MetricsConfiguration>>,
    #[serde(rename = "NextContinuationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_continuation_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UploadPartRequest")]
pub struct UploadPartRequest {
    #[serde(rename = "Body")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
    #[serde(rename = "ChecksumCRC32")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_c_r_c32: Option<String>,
    #[serde(rename = "ChecksumCRC32C")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_c_r_c32_c: Option<String>,
    #[serde(rename = "ChecksumCRC64NVME")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_c_r_c64_n_v_m_e: Option<String>,
    #[serde(rename = "ChecksumSHA1")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_s_h_a1: Option<String>,
    #[serde(rename = "ChecksumSHA256")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_s_h_a256: Option<String>,
    #[serde(rename = "ContentLength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_length: Option<i64>,
    #[serde(rename = "ContentMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_m_d5: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "PartNumber")]
    #[serde(default)]
    pub part_number: i32,
    #[serde(rename = "RequestPayer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_payer: Option<String>,
    #[serde(rename = "SSECustomerAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_customer_algorithm: Option<String>,
    #[serde(rename = "SSECustomerKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_customer_key: Option<String>,
    #[serde(rename = "SSECustomerKeyMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_customer_key_m_d5: Option<String>,
    #[serde(rename = "UploadId")]
    #[serde(default)]
    pub upload_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketInventoryConfigurationOutput")]
pub struct GetBucketInventoryConfigurationOutput {
    #[serde(rename = "InventoryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_configuration: Option<InventoryConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketAclRequest")]
pub struct GetBucketAclRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LocationConstraint")]
pub struct GetBucketLocationOutput {
    #[serde(rename = "LocationConstraint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_constraint: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteBucketTaggingRequest")]
pub struct DeleteBucketTaggingRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketCorsRequest")]
pub struct GetBucketCorsRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketReplicationRequest")]
pub struct GetBucketReplicationRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketReplicationOutput")]
pub struct GetBucketReplicationOutput {
    #[serde(rename = "ReplicationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_configuration: Option<ReplicationConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteObjectsRequest")]
pub struct DeleteObjectsRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "BypassGovernanceRetention")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_governance_retention: Option<bool>,
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
    #[serde(rename = "Delete")]
    #[serde(default)]
    pub delete: Delete,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "MFA")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_f_a: Option<String>,
    #[serde(rename = "RequestPayer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_payer: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutBucketLoggingRequest")]
pub struct PutBucketLoggingRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "BucketLoggingStatus")]
    #[serde(default)]
    pub bucket_logging_status: BucketLoggingStatus,
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
    #[serde(rename = "ContentMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_m_d5: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "BucketLoggingStatus")]
pub struct BucketLoggingStatus {
    #[serde(rename = "LoggingEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_enabled: Option<LoggingEnabled>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CORSConfiguration")]
pub struct GetBucketCorsOutput {
    #[serde(rename = "CORSRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_o_r_s_rules: Option<Vec<CORSRule>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutBucketVersioningRequest")]
pub struct PutBucketVersioningRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
    #[serde(rename = "ContentMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_m_d5: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
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
#[serde(rename = "UpdateObjectEncryptionRequest")]
pub struct UpdateObjectEncryptionRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
    #[serde(rename = "ContentMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_m_d5: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "ObjectEncryption")]
    #[serde(default)]
    pub object_encryption: ObjectEncryption,
    #[serde(rename = "RequestPayer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_payer: Option<String>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ObjectEncryption")]
pub struct ObjectEncryption {
    #[serde(rename = "SSE-KMS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_k_m_s: Option<SSEKMSEncryption>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SSE-KMS")]
pub struct SSEKMSEncryption {
    #[serde(rename = "BucketKeyEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_key_enabled: Option<bool>,
    #[serde(rename = "KMSKeyArn")]
    #[serde(default)]
    pub k_m_s_key_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteObjectTaggingRequest")]
pub struct DeleteObjectTaggingRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UploadPartCopyRequest")]
pub struct UploadPartCopyRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "CopySource")]
    #[serde(default)]
    pub copy_source: String,
    #[serde(rename = "CopySourceIfMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_source_if_match: Option<String>,
    #[serde(rename = "CopySourceIfModifiedSince")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_source_if_modified_since: Option<String>,
    #[serde(rename = "CopySourceIfNoneMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_source_if_none_match: Option<String>,
    #[serde(rename = "CopySourceIfUnmodifiedSince")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_source_if_unmodified_since: Option<String>,
    #[serde(rename = "CopySourceRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_source_range: Option<String>,
    #[serde(rename = "CopySourceSSECustomerAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_source_s_s_e_customer_algorithm: Option<String>,
    #[serde(rename = "CopySourceSSECustomerKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_source_s_s_e_customer_key: Option<String>,
    #[serde(rename = "CopySourceSSECustomerKeyMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_source_s_s_e_customer_key_m_d5: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "ExpectedSourceBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_source_bucket_owner: Option<String>,
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "PartNumber")]
    #[serde(default)]
    pub part_number: i32,
    #[serde(rename = "RequestPayer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_payer: Option<String>,
    #[serde(rename = "SSECustomerAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_customer_algorithm: Option<String>,
    #[serde(rename = "SSECustomerKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_customer_key: Option<String>,
    #[serde(rename = "SSECustomerKeyMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_customer_key_m_d5: Option<String>,
    #[serde(rename = "UploadId")]
    #[serde(default)]
    pub upload_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateObjectEncryptionResponse")]
pub struct UpdateObjectEncryptionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketLoggingRequest")]
pub struct GetBucketLoggingRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketLocationRequest")]
pub struct GetBucketLocationRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CopyObjectOutput")]
pub struct CopyObjectOutput {
    #[serde(rename = "CopyObjectResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_object_result: Option<CopyObjectResult>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketMetadataConfigurationOutput")]
pub struct GetBucketMetadataConfigurationOutput {
    #[serde(rename = "GetBucketMetadataConfigurationResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_bucket_metadata_configuration_result: Option<GetBucketMetadataConfigurationResult>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketPolicyStatusOutput")]
pub struct GetBucketPolicyStatusOutput {
    #[serde(rename = "PolicyStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_status: Option<PolicyStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetObjectAttributesResponse")]
pub struct GetObjectAttributesOutput {
    #[serde(rename = "Checksum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<Checksum>,
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "ObjectParts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_parts: Option<GetObjectAttributesParts>,
    #[serde(rename = "ObjectSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_size: Option<i64>,
    #[serde(rename = "StorageClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Checksum")]
pub struct Checksum {
    #[serde(rename = "ChecksumCRC32")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_c_r_c32: Option<String>,
    #[serde(rename = "ChecksumCRC32C")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_c_r_c32_c: Option<String>,
    #[serde(rename = "ChecksumCRC64NVME")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_c_r_c64_n_v_m_e: Option<String>,
    #[serde(rename = "ChecksumSHA1")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_s_h_a1: Option<String>,
    #[serde(rename = "ChecksumSHA256")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_s_h_a256: Option<String>,
    #[serde(rename = "ChecksumType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetObjectAttributesParts")]
pub struct GetObjectAttributesParts {
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "MaxParts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_parts: Option<i32>,
    #[serde(rename = "NextPartNumberMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_part_number_marker: Option<String>,
    #[serde(rename = "PartNumberMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_number_marker: Option<String>,
    #[serde(rename = "Part")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parts: Option<Vec<ObjectPart>>,
    #[serde(rename = "PartsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_parts_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ObjectPart")]
pub struct ObjectPart {
    #[serde(rename = "ChecksumCRC32")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_c_r_c32: Option<String>,
    #[serde(rename = "ChecksumCRC32C")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_c_r_c32_c: Option<String>,
    #[serde(rename = "ChecksumCRC64NVME")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_c_r_c64_n_v_m_e: Option<String>,
    #[serde(rename = "ChecksumSHA1")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_s_h_a1: Option<String>,
    #[serde(rename = "ChecksumSHA256")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_s_h_a256: Option<String>,
    #[serde(rename = "PartNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_number: Option<i32>,
    #[serde(rename = "Size")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateSessionRequest")]
pub struct CreateSessionRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "BucketKeyEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_key_enabled: Option<bool>,
    #[serde(rename = "SSEKMSEncryptionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_k_m_s_encryption_context: Option<String>,
    #[serde(rename = "SSEKMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_k_m_s_key_id: Option<String>,
    #[serde(rename = "ServerSideEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_side_encryption: Option<String>,
    #[serde(rename = "SessionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteBucketAnalyticsConfigurationRequest")]
pub struct DeleteBucketAnalyticsConfigurationRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutObjectTaggingOutput")]
pub struct PutObjectTaggingOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UploadPartOutput")]
pub struct UploadPartOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "VersioningConfiguration")]
pub struct GetBucketVersioningOutput {
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
#[serde(rename = "WriteGetObjectResponseRequest")]
pub struct WriteGetObjectResponseRequest {
    #[serde(rename = "AcceptRanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_ranges: Option<String>,
    #[serde(rename = "Body")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "BucketKeyEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_key_enabled: Option<bool>,
    #[serde(rename = "CacheControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_control: Option<String>,
    #[serde(rename = "ChecksumCRC32")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_c_r_c32: Option<String>,
    #[serde(rename = "ChecksumCRC32C")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_c_r_c32_c: Option<String>,
    #[serde(rename = "ChecksumCRC64NVME")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_c_r_c64_n_v_m_e: Option<String>,
    #[serde(rename = "ChecksumSHA1")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_s_h_a1: Option<String>,
    #[serde(rename = "ChecksumSHA256")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_s_h_a256: Option<String>,
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
    #[serde(rename = "ContentRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_range: Option<String>,
    #[serde(rename = "ContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "DeleteMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_marker: Option<bool>,
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "Expiration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    #[serde(rename = "Expires")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    #[serde(rename = "LastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(rename = "Metadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "MissingMeta")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_meta: Option<i32>,
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
    #[serde(rename = "PartsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parts_count: Option<i32>,
    #[serde(rename = "ReplicationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_status: Option<String>,
    #[serde(rename = "RequestCharged")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_charged: Option<String>,
    #[serde(rename = "RequestRoute")]
    #[serde(default)]
    pub request_route: String,
    #[serde(rename = "RequestToken")]
    #[serde(default)]
    pub request_token: String,
    #[serde(rename = "Restore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore: Option<String>,
    #[serde(rename = "SSECustomerAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_customer_algorithm: Option<String>,
    #[serde(rename = "SSECustomerKeyMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_customer_key_m_d5: Option<String>,
    #[serde(rename = "SSEKMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_e_k_m_s_key_id: Option<String>,
    #[serde(rename = "ServerSideEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_side_encryption: Option<String>,
    #[serde(rename = "StatusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i32>,
    #[serde(rename = "StorageClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
    #[serde(rename = "TagCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_count: Option<i32>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RestoreObjectOutput")]
pub struct RestoreObjectOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteBucketMetadataTableConfigurationRequest")]
pub struct DeleteBucketMetadataTableConfigurationRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutObjectRetentionRequest")]
pub struct PutObjectRetentionRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "BypassGovernanceRetention")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_governance_retention: Option<bool>,
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
    #[serde(rename = "ContentMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_m_d5: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "RequestPayer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_payer: Option<String>,
    #[serde(rename = "Retention")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention: Option<ObjectLockRetention>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Tagging")]
pub struct GetObjectTaggingOutput {
    #[serde(rename = "TagSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_set: Option<TagSet>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CompleteMultipartUploadResult")]
pub struct CompleteMultipartUploadOutput {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(rename = "ChecksumCRC32")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_c_r_c32: Option<String>,
    #[serde(rename = "ChecksumCRC32C")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_c_r_c32_c: Option<String>,
    #[serde(rename = "ChecksumCRC64NVME")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_c_r_c64_n_v_m_e: Option<String>,
    #[serde(rename = "ChecksumSHA1")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_s_h_a1: Option<String>,
    #[serde(rename = "ChecksumSHA256")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_s_h_a256: Option<String>,
    #[serde(rename = "ChecksumType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_type: Option<String>,
    #[serde(rename = "ETag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "HeadObjectOutput")]
pub struct HeadObjectOutput {
    #[serde(rename = "Metadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketAccelerateConfigurationRequest")]
pub struct GetBucketAccelerateConfigurationRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "RequestPayer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_payer: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketInventoryConfigurationRequest")]
pub struct GetBucketInventoryConfigurationRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketAnalyticsConfigurationRequest")]
pub struct GetBucketAnalyticsConfigurationRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutBucketOwnershipControlsRequest")]
pub struct PutBucketOwnershipControlsRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
    #[serde(rename = "ContentMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_m_d5: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "OwnershipControls")]
    #[serde(default)]
    pub ownership_controls: OwnershipControls,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutPublicAccessBlockRequest")]
pub struct PutPublicAccessBlockRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
    #[serde(rename = "ContentMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_m_d5: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "PublicAccessBlockConfiguration")]
    #[serde(default)]
    pub public_access_block_configuration: PublicAccessBlockConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutBucketReplicationRequest")]
pub struct PutBucketReplicationRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
    #[serde(rename = "ContentMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_m_d5: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "ReplicationConfiguration")]
    #[serde(default)]
    pub replication_configuration: ReplicationConfiguration,
    #[serde(rename = "Token")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateSessionResult")]
pub struct CreateSessionOutput {
    #[serde(rename = "Credentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<SessionCredentials>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SessionCredentials")]
pub struct SessionCredentials {
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
#[serde(rename = "ListMultipartUploadsResult")]
pub struct ListMultipartUploadsOutput {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(rename = "CommonPrefixes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_prefixes: Option<Vec<CommonPrefix>>,
    #[serde(rename = "Delimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    #[serde(rename = "EncodingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_type: Option<String>,
    #[serde(rename = "IsTruncated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    #[serde(rename = "KeyMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_marker: Option<String>,
    #[serde(rename = "MaxUploads")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_uploads: Option<i32>,
    #[serde(rename = "NextKeyMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_key_marker: Option<String>,
    #[serde(rename = "NextUploadIdMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_upload_id_marker: Option<String>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "UploadIdMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_id_marker: Option<String>,
    #[serde(rename = "Upload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploads: Option<Vec<MultipartUpload>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MultipartUpload")]
pub struct MultipartUpload {
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
    #[serde(rename = "ChecksumType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_type: Option<String>,
    #[serde(rename = "Initiated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiated: Option<String>,
    #[serde(rename = "Initiator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator: Option<Initiator>,
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Owner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<Owner>,
    #[serde(rename = "StorageClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
    #[serde(rename = "UploadId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetObjectTorrentOutput")]
pub struct GetObjectTorrentOutput {
    #[serde(rename = "Body")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutBucketNotificationConfigurationRequest")]
pub struct PutBucketNotificationConfigurationRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "NotificationConfiguration")]
    #[serde(default)]
    pub notification_configuration: NotificationConfiguration,
    #[serde(rename = "SkipDestinationValidation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_destination_validation: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateBucketMetadataInventoryTableConfigurationRequest")]
pub struct UpdateBucketMetadataInventoryTableConfigurationRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
    #[serde(rename = "ContentMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_m_d5: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "InventoryTableConfiguration")]
    #[serde(default)]
    pub inventory_table_configuration: InventoryTableConfigurationUpdates,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListAllMyDirectoryBucketsResult")]
pub struct ListDirectoryBucketsOutput {
    #[serde(rename = "Buckets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buckets: Option<Buckets>,
    #[serde(rename = "ContinuationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetBucketIntelligentTieringConfigurationRequest")]
pub struct GetBucketIntelligentTieringConfigurationRequest {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutBucketAclRequest")]
pub struct PutBucketAclRequest {
    #[serde(rename = "ACL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_c_l: Option<String>,
    #[serde(rename = "AccessControlPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_policy: Option<AccessControlPolicy>,
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
    #[serde(rename = "ContentMD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_m_d5: Option<String>,
    #[serde(rename = "ExpectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
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
}
