//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-signer

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddProfilePermissionRequest {
    #[serde(default)]
    pub action: String,
    #[serde(default)]
    pub principal: String,
    #[serde(rename = "profileName")]
    #[serde(default)]
    pub profile_name: String,
    #[serde(rename = "profileVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_version: Option<String>,
    #[serde(rename = "revisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    #[serde(rename = "statementId")]
    #[serde(default)]
    pub statement_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddProfilePermissionResponse {
    #[serde(rename = "revisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelSigningProfileRequest {
    #[serde(rename = "profileName")]
    #[serde(default)]
    pub profile_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSigningJobRequest {
    #[serde(rename = "jobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSigningJobResponse {
    #[serde(rename = "completedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<f64>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "jobInvoker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_invoker: Option<String>,
    #[serde(rename = "jobOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_owner: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<SigningPlatformOverrides>,
    #[serde(rename = "platformDisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_display_name: Option<String>,
    #[serde(rename = "platformId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_id: Option<String>,
    #[serde(rename = "profileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<String>,
    #[serde(rename = "profileVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_version: Option<String>,
    #[serde(rename = "requestedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_by: Option<String>,
    #[serde(rename = "revocationRecord")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_record: Option<SigningJobRevocationRecord>,
    #[serde(rename = "signatureExpiresAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_expires_at: Option<f64>,
    #[serde(rename = "signedObject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_object: Option<SignedObject>,
    #[serde(rename = "signingMaterial")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_material: Option<SigningMaterial>,
    #[serde(rename = "signingParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SigningPlatformOverrides {
    #[serde(rename = "signingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_configuration: Option<SigningConfigurationOverrides>,
    #[serde(rename = "signingImageFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_image_format: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SigningConfigurationOverrides {
    #[serde(rename = "encryptionAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_algorithm: Option<String>,
    #[serde(rename = "hashAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_algorithm: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SigningJobRevocationRecord {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "revokedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked_at: Option<f64>,
    #[serde(rename = "revokedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked_by: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SignedObject {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3: Option<S3SignedObject>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3SignedObject {
    #[serde(rename = "bucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SigningMaterial {
    #[serde(rename = "certificateArn")]
    #[serde(default)]
    pub certificate_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Source {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3: Option<S3Source>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Source {
    #[serde(rename = "bucketName")]
    #[serde(default)]
    pub bucket_name: String,
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRevocationStatusRequest {
    #[serde(rename = "certificateHashes")]
    #[serde(default)]
    pub certificate_hashes: Vec<String>,
    #[serde(rename = "jobArn")]
    #[serde(default)]
    pub job_arn: String,
    #[serde(rename = "platformId")]
    #[serde(default)]
    pub platform_id: String,
    #[serde(rename = "profileVersionArn")]
    #[serde(default)]
    pub profile_version_arn: String,
    #[serde(rename = "signatureTimestamp")]
    #[serde(default)]
    pub signature_timestamp: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRevocationStatusResponse {
    #[serde(rename = "revokedEntities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked_entities: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSigningPlatformRequest {
    #[serde(rename = "platformId")]
    #[serde(default)]
    pub platform_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSigningPlatformResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "maxSizeInMB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size_in_m_b: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner: Option<String>,
    #[serde(rename = "platformId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_id: Option<String>,
    #[serde(rename = "revocationSupported")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_supported: Option<bool>,
    #[serde(rename = "signingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_configuration: Option<SigningConfiguration>,
    #[serde(rename = "signingImageFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_image_format: Option<SigningImageFormat>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SigningConfiguration {
    #[serde(rename = "encryptionAlgorithmOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_algorithm_options: Option<EncryptionAlgorithmOptions>,
    #[serde(rename = "hashAlgorithmOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_algorithm_options: Option<HashAlgorithmOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptionAlgorithmOptions {
    #[serde(rename = "allowedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<Vec<String>>,
    #[serde(rename = "defaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HashAlgorithmOptions {
    #[serde(rename = "allowedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<Vec<String>>,
    #[serde(rename = "defaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SigningImageFormat {
    #[serde(rename = "defaultFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_format: Option<String>,
    #[serde(rename = "supportedFormats")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_formats: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSigningProfileRequest {
    #[serde(rename = "profileName")]
    #[serde(default)]
    pub profile_name: String,
    #[serde(rename = "profileOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSigningProfileResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<SigningPlatformOverrides>,
    #[serde(rename = "platformDisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_display_name: Option<String>,
    #[serde(rename = "platformId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_id: Option<String>,
    #[serde(rename = "profileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<String>,
    #[serde(rename = "profileVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_version: Option<String>,
    #[serde(rename = "profileVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_version_arn: Option<String>,
    #[serde(rename = "revocationRecord")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_record: Option<SigningProfileRevocationRecord>,
    #[serde(rename = "signatureValidityPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_validity_period: Option<SignatureValidityPeriod>,
    #[serde(rename = "signingMaterial")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_material: Option<SigningMaterial>,
    #[serde(rename = "signingParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SigningProfileRevocationRecord {
    #[serde(rename = "revocationEffectiveFrom")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_effective_from: Option<f64>,
    #[serde(rename = "revokedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked_at: Option<f64>,
    #[serde(rename = "revokedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked_by: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SignatureValidityPeriod {
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProfilePermissionsRequest {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "profileName")]
    #[serde(default)]
    pub profile_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProfilePermissionsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<Permission>>,
    #[serde(rename = "policySizeBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_size_bytes: Option<i32>,
    #[serde(rename = "revisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Permission {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<String>,
    #[serde(rename = "profileVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_version: Option<String>,
    #[serde(rename = "statementId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSigningJobsRequest {
    #[serde(rename = "isRevoked")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_revoked: Option<bool>,
    #[serde(rename = "jobInvoker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_invoker: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "platformId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_id: Option<String>,
    #[serde(rename = "requestedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_by: Option<String>,
    #[serde(rename = "signatureExpiresAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_expires_after: Option<f64>,
    #[serde(rename = "signatureExpiresBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_expires_before: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSigningJobsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<SigningJob>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SigningJob {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "isRevoked")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_revoked: Option<bool>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "jobInvoker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_invoker: Option<String>,
    #[serde(rename = "jobOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_owner: Option<String>,
    #[serde(rename = "platformDisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_display_name: Option<String>,
    #[serde(rename = "platformId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_id: Option<String>,
    #[serde(rename = "profileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<String>,
    #[serde(rename = "profileVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_version: Option<String>,
    #[serde(rename = "signatureExpiresAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_expires_at: Option<f64>,
    #[serde(rename = "signedObject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_object: Option<SignedObject>,
    #[serde(rename = "signingMaterial")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_material: Option<SigningMaterial>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSigningPlatformsRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSigningPlatformsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platforms: Option<Vec<SigningPlatform>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SigningPlatform {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "maxSizeInMB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size_in_m_b: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner: Option<String>,
    #[serde(rename = "platformId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_id: Option<String>,
    #[serde(rename = "revocationSupported")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_supported: Option<bool>,
    #[serde(rename = "signingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_configuration: Option<SigningConfiguration>,
    #[serde(rename = "signingImageFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_image_format: Option<SigningImageFormat>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSigningProfilesRequest {
    #[serde(rename = "includeCanceled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_canceled: Option<bool>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "platformId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSigningProfilesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<SigningProfile>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SigningProfile {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "platformDisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_display_name: Option<String>,
    #[serde(rename = "platformId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_id: Option<String>,
    #[serde(rename = "profileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<String>,
    #[serde(rename = "profileVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_version: Option<String>,
    #[serde(rename = "profileVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_version_arn: Option<String>,
    #[serde(rename = "signatureValidityPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_validity_period: Option<SignatureValidityPeriod>,
    #[serde(rename = "signingMaterial")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_material: Option<SigningMaterial>,
    #[serde(rename = "signingParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutSigningProfileRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<SigningPlatformOverrides>,
    #[serde(rename = "platformId")]
    #[serde(default)]
    pub platform_id: String,
    #[serde(rename = "profileName")]
    #[serde(default)]
    pub profile_name: String,
    #[serde(rename = "signatureValidityPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_validity_period: Option<SignatureValidityPeriod>,
    #[serde(rename = "signingMaterial")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_material: Option<SigningMaterial>,
    #[serde(rename = "signingParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutSigningProfileResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "profileVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_version: Option<String>,
    #[serde(rename = "profileVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_version_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveProfilePermissionRequest {
    #[serde(rename = "profileName")]
    #[serde(default)]
    pub profile_name: String,
    #[serde(rename = "revisionId")]
    #[serde(default)]
    pub revision_id: String,
    #[serde(rename = "statementId")]
    #[serde(default)]
    pub statement_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveProfilePermissionResponse {
    #[serde(rename = "revisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RevokeSignatureRequest {
    #[serde(rename = "jobId")]
    #[serde(default)]
    pub job_id: String,
    #[serde(rename = "jobOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_owner: Option<String>,
    #[serde(default)]
    pub reason: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RevokeSigningProfileRequest {
    #[serde(rename = "effectiveTime")]
    #[serde(default)]
    pub effective_time: f64,
    #[serde(rename = "profileName")]
    #[serde(default)]
    pub profile_name: String,
    #[serde(rename = "profileVersion")]
    #[serde(default)]
    pub profile_version: String,
    #[serde(default)]
    pub reason: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SignPayloadRequest {
    #[serde(default)]
    pub payload: String,
    #[serde(rename = "payloadFormat")]
    #[serde(default)]
    pub payload_format: String,
    #[serde(rename = "profileName")]
    #[serde(default)]
    pub profile_name: String,
    #[serde(rename = "profileOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SignPayloadResponse {
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "jobOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_owner: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSigningJobRequest {
    #[serde(rename = "clientRequestToken")]
    #[serde(default)]
    pub client_request_token: String,
    #[serde(default)]
    pub destination: Destination,
    #[serde(rename = "profileName")]
    #[serde(default)]
    pub profile_name: String,
    #[serde(rename = "profileOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_owner: Option<String>,
    #[serde(default)]
    pub source: Source,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Destination {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3: Option<S3Destination>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Destination {
    #[serde(rename = "bucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSigningJobResponse {
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "jobOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "tagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}
