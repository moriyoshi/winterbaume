//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-ecr

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchCheckLayerAvailabilityRequest {
    #[serde(rename = "layerDigests")]
    #[serde(default)]
    pub layer_digests: Vec<String>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchCheckLayerAvailabilityResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<LayerFailure>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<Layer>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LayerFailure {
    #[serde(rename = "failureCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    #[serde(rename = "failureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "layerDigest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_digest: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Layer {
    #[serde(rename = "layerAvailability")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_availability: Option<String>,
    #[serde(rename = "layerDigest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_digest: Option<String>,
    #[serde(rename = "layerSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_size: Option<i64>,
    #[serde(rename = "mediaType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteImageRequest {
    #[serde(rename = "imageIds")]
    #[serde(default)]
    pub image_ids: Vec<ImageIdentifier>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImageIdentifier {
    #[serde(rename = "imageDigest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_digest: Option<String>,
    #[serde(rename = "imageTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tag: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteImageResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<ImageFailure>>,
    #[serde(rename = "imageIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_ids: Option<Vec<ImageIdentifier>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImageFailure {
    #[serde(rename = "failureCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    #[serde(rename = "failureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "imageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<ImageIdentifier>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetImageRequest {
    #[serde(rename = "acceptedMediaTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_media_types: Option<Vec<String>>,
    #[serde(rename = "imageIds")]
    #[serde(default)]
    pub image_ids: Vec<ImageIdentifier>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetImageResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<ImageFailure>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<Image>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Image {
    #[serde(rename = "imageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<ImageIdentifier>,
    #[serde(rename = "imageManifest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_manifest: Option<String>,
    #[serde(rename = "imageManifestMediaType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_manifest_media_type: Option<String>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetRepositoryScanningConfigurationRequest {
    #[serde(rename = "repositoryNames")]
    #[serde(default)]
    pub repository_names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetRepositoryScanningConfigurationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<RepositoryScanningConfigurationFailure>>,
    #[serde(rename = "scanningConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanning_configurations: Option<Vec<RepositoryScanningConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RepositoryScanningConfigurationFailure {
    #[serde(rename = "failureCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    #[serde(rename = "failureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RepositoryScanningConfiguration {
    #[serde(rename = "appliedScanFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_scan_filters: Option<Vec<ScanningRepositoryFilter>>,
    #[serde(rename = "repositoryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_arn: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
    #[serde(rename = "scanFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_frequency: Option<String>,
    #[serde(rename = "scanOnPush")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_on_push: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScanningRepositoryFilter {
    #[serde(default)]
    pub filter: String,
    #[serde(rename = "filterType")]
    #[serde(default)]
    pub filter_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CompleteLayerUploadRequest {
    #[serde(rename = "layerDigests")]
    #[serde(default)]
    pub layer_digests: Vec<String>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
    #[serde(rename = "uploadId")]
    #[serde(default)]
    pub upload_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CompleteLayerUploadResponse {
    #[serde(rename = "layerDigest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_digest: Option<String>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
    #[serde(rename = "uploadId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePullThroughCacheRuleRequest {
    #[serde(rename = "credentialArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_arn: Option<String>,
    #[serde(rename = "customRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_role_arn: Option<String>,
    #[serde(rename = "ecrRepositoryPrefix")]
    #[serde(default)]
    pub ecr_repository_prefix: String,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "upstreamRegistry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstream_registry: Option<String>,
    #[serde(rename = "upstreamRegistryUrl")]
    #[serde(default)]
    pub upstream_registry_url: String,
    #[serde(rename = "upstreamRepositoryPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstream_repository_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePullThroughCacheRuleResponse {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "credentialArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_arn: Option<String>,
    #[serde(rename = "customRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_role_arn: Option<String>,
    #[serde(rename = "ecrRepositoryPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecr_repository_prefix: Option<String>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "upstreamRegistry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstream_registry: Option<String>,
    #[serde(rename = "upstreamRegistryUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstream_registry_url: Option<String>,
    #[serde(rename = "upstreamRepositoryPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstream_repository_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRepositoryCreationTemplateRequest {
    #[serde(rename = "appliedFor")]
    #[serde(default)]
    pub applied_for: Vec<String>,
    #[serde(rename = "customRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "encryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfigurationForRepositoryCreationTemplate>,
    #[serde(rename = "imageTagMutability")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tag_mutability: Option<String>,
    #[serde(rename = "imageTagMutabilityExclusionFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tag_mutability_exclusion_filters: Option<Vec<ImageTagMutabilityExclusionFilter>>,
    #[serde(rename = "lifecyclePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy: Option<String>,
    #[serde(default)]
    pub prefix: String,
    #[serde(rename = "repositoryPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_policy: Option<String>,
    #[serde(rename = "resourceTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptionConfigurationForRepositoryCreationTemplate {
    #[serde(rename = "encryptionType")]
    #[serde(default)]
    pub encryption_type: String,
    #[serde(rename = "kmsKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImageTagMutabilityExclusionFilter {
    #[serde(default)]
    pub filter: String,
    #[serde(rename = "filterType")]
    #[serde(default)]
    pub filter_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRepositoryCreationTemplateResponse {
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryCreationTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_creation_template: Option<RepositoryCreationTemplate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RepositoryCreationTemplate {
    #[serde(rename = "appliedFor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_for: Option<Vec<String>>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "customRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "encryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfigurationForRepositoryCreationTemplate>,
    #[serde(rename = "imageTagMutability")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tag_mutability: Option<String>,
    #[serde(rename = "imageTagMutabilityExclusionFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tag_mutability_exclusion_filters: Option<Vec<ImageTagMutabilityExclusionFilter>>,
    #[serde(rename = "lifecyclePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "repositoryPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_policy: Option<String>,
    #[serde(rename = "resourceTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<Vec<Tag>>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRepositoryRequest {
    #[serde(rename = "encryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "imageScanningConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_scanning_configuration: Option<ImageScanningConfiguration>,
    #[serde(rename = "imageTagMutability")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tag_mutability: Option<String>,
    #[serde(rename = "imageTagMutabilityExclusionFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tag_mutability_exclusion_filters: Option<Vec<ImageTagMutabilityExclusionFilter>>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptionConfiguration {
    #[serde(rename = "encryptionType")]
    #[serde(default)]
    pub encryption_type: String,
    #[serde(rename = "kmsKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImageScanningConfiguration {
    #[serde(rename = "scanOnPush")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_on_push: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRepositoryResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Repository {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "encryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[serde(rename = "imageScanningConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_scanning_configuration: Option<ImageScanningConfiguration>,
    #[serde(rename = "imageTagMutability")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tag_mutability: Option<String>,
    #[serde(rename = "imageTagMutabilityExclusionFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tag_mutability_exclusion_filters: Option<Vec<ImageTagMutabilityExclusionFilter>>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_arn: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
    #[serde(rename = "repositoryUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLifecyclePolicyRequest {
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLifecyclePolicyResponse {
    #[serde(rename = "lastEvaluatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_evaluated_at: Option<f64>,
    #[serde(rename = "lifecyclePolicyText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy_text: Option<String>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePullThroughCacheRuleRequest {
    #[serde(rename = "ecrRepositoryPrefix")]
    #[serde(default)]
    pub ecr_repository_prefix: String,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePullThroughCacheRuleResponse {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "credentialArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_arn: Option<String>,
    #[serde(rename = "customRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_role_arn: Option<String>,
    #[serde(rename = "ecrRepositoryPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecr_repository_prefix: Option<String>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "upstreamRegistryUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstream_registry_url: Option<String>,
    #[serde(rename = "upstreamRepositoryPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstream_repository_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRegistryPolicyRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRegistryPolicyResponse {
    #[serde(rename = "policyText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_text: Option<String>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRepositoryCreationTemplateRequest {
    #[serde(default)]
    pub prefix: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRepositoryCreationTemplateResponse {
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryCreationTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_creation_template: Option<RepositoryCreationTemplate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRepositoryPolicyRequest {
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRepositoryPolicyResponse {
    #[serde(rename = "policyText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_text: Option<String>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRepositoryRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRepositoryResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSigningConfigurationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSigningConfigurationResponse {
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "signingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_configuration: Option<SigningConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SigningConfiguration {
    #[serde(default)]
    pub rules: Vec<SigningRule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SigningRule {
    #[serde(rename = "repositoryFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_filters: Option<Vec<SigningRepositoryFilter>>,
    #[serde(rename = "signingProfileArn")]
    #[serde(default)]
    pub signing_profile_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SigningRepositoryFilter {
    #[serde(default)]
    pub filter: String,
    #[serde(rename = "filterType")]
    #[serde(default)]
    pub filter_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterPullTimeUpdateExclusionRequest {
    #[serde(rename = "principalArn")]
    #[serde(default)]
    pub principal_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterPullTimeUpdateExclusionResponse {
    #[serde(rename = "principalArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeImageReplicationStatusRequest {
    #[serde(rename = "imageId")]
    #[serde(default)]
    pub image_id: ImageIdentifier,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeImageReplicationStatusResponse {
    #[serde(rename = "imageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<ImageIdentifier>,
    #[serde(rename = "replicationStatuses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_statuses: Option<Vec<ImageReplicationStatus>>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImageReplicationStatus {
    #[serde(rename = "failureCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeImageScanFindingsRequest {
    #[serde(rename = "imageId")]
    #[serde(default)]
    pub image_id: ImageIdentifier,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeImageScanFindingsResponse {
    #[serde(rename = "imageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<ImageIdentifier>,
    #[serde(rename = "imageScanFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_scan_findings: Option<ImageScanFindings>,
    #[serde(rename = "imageScanStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_scan_status: Option<ImageScanStatus>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImageScanFindings {
    #[serde(rename = "enhancedFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_findings: Option<Vec<EnhancedImageScanFinding>>,
    #[serde(rename = "findingSeverityCounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_severity_counts: Option<std::collections::HashMap<String, i32>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub findings: Option<Vec<ImageScanFinding>>,
    #[serde(rename = "imageScanCompletedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_scan_completed_at: Option<f64>,
    #[serde(rename = "vulnerabilitySourceUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerability_source_updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnhancedImageScanFinding {
    #[serde(rename = "awsAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "exploitAvailable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exploit_available: Option<String>,
    #[serde(rename = "findingArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_arn: Option<String>,
    #[serde(rename = "firstObservedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_observed_at: Option<f64>,
    #[serde(rename = "fixAvailable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fix_available: Option<String>,
    #[serde(rename = "lastObservedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_observed_at: Option<f64>,
    #[serde(rename = "packageVulnerabilityDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_vulnerability_details: Option<PackageVulnerabilityDetails>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation: Option<Remediation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<Resource>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[serde(rename = "scoreDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_details: Option<ScoreDetails>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageVulnerabilityDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvss: Option<Vec<CvssScore>>,
    #[serde(rename = "referenceUrls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_urls: Option<Vec<String>>,
    #[serde(rename = "relatedVulnerabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_vulnerabilities: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "sourceUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    #[serde(rename = "vendorCreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_created_at: Option<f64>,
    #[serde(rename = "vendorSeverity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_severity: Option<String>,
    #[serde(rename = "vendorUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_updated_at: Option<f64>,
    #[serde(rename = "vulnerabilityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerability_id: Option<String>,
    #[serde(rename = "vulnerablePackages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerable_packages: Option<Vec<VulnerablePackage>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CvssScore {
    #[serde(rename = "baseScore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_score: Option<f64>,
    #[serde(rename = "scoringVector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scoring_vector: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VulnerablePackage {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epoch: Option<i32>,
    #[serde(rename = "filePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(rename = "fixedInVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_in_version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "packageManager")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_manager: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release: Option<String>,
    #[serde(rename = "sourceLayerHash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_layer_hash: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Remediation {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation: Option<Recommendation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Recommendation {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Resource {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<ResourceDetails>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceDetails {
    #[serde(rename = "awsEcrContainerImage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ecr_container_image: Option<AwsEcrContainerImageDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsEcrContainerImageDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(rename = "imageHash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_hash: Option<String>,
    #[serde(rename = "imageTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tags: Option<Vec<String>>,
    #[serde(rename = "inUseCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_use_count: Option<i64>,
    #[serde(rename = "lastInUseAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_in_use_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "pushedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pushed_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScoreDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvss: Option<CvssScoreDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CvssScoreDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustments: Option<Vec<CvssScoreAdjustment>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[serde(rename = "scoreSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_source: Option<String>,
    #[serde(rename = "scoringVector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scoring_vector: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CvssScoreAdjustment {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImageScanFinding {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Attribute {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImageScanStatus {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeImageSigningStatusRequest {
    #[serde(rename = "imageId")]
    #[serde(default)]
    pub image_id: ImageIdentifier,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeImageSigningStatusResponse {
    #[serde(rename = "imageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<ImageIdentifier>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
    #[serde(rename = "signingStatuses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_statuses: Option<Vec<ImageSigningStatus>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImageSigningStatus {
    #[serde(rename = "failureCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    #[serde(rename = "failureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "signingProfileArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_profile_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeImagesRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<DescribeImagesFilter>,
    #[serde(rename = "imageIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_ids: Option<Vec<ImageIdentifier>>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeImagesFilter {
    #[serde(rename = "imageStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_status: Option<String>,
    #[serde(rename = "tagStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeImagesResponse {
    #[serde(rename = "imageDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_details: Option<Vec<ImageDetail>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImageDetail {
    #[serde(rename = "artifactMediaType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_media_type: Option<String>,
    #[serde(rename = "imageDigest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_digest: Option<String>,
    #[serde(rename = "imageManifestMediaType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_manifest_media_type: Option<String>,
    #[serde(rename = "imagePushedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pushed_at: Option<f64>,
    #[serde(rename = "imageScanFindingsSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_scan_findings_summary: Option<ImageScanFindingsSummary>,
    #[serde(rename = "imageScanStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_scan_status: Option<ImageScanStatus>,
    #[serde(rename = "imageSizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size_in_bytes: Option<i64>,
    #[serde(rename = "imageStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_status: Option<String>,
    #[serde(rename = "imageTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tags: Option<Vec<String>>,
    #[serde(rename = "lastActivatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_activated_at: Option<f64>,
    #[serde(rename = "lastArchivedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_archived_at: Option<f64>,
    #[serde(rename = "lastRecordedPullTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_recorded_pull_time: Option<f64>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
    #[serde(rename = "subjectManifestDigest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_manifest_digest: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImageScanFindingsSummary {
    #[serde(rename = "findingSeverityCounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_severity_counts: Option<std::collections::HashMap<String, i32>>,
    #[serde(rename = "imageScanCompletedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_scan_completed_at: Option<f64>,
    #[serde(rename = "vulnerabilitySourceUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vulnerability_source_updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePullThroughCacheRulesRequest {
    #[serde(rename = "ecrRepositoryPrefixes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecr_repository_prefixes: Option<Vec<String>>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePullThroughCacheRulesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "pullThroughCacheRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_through_cache_rules: Option<Vec<PullThroughCacheRule>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PullThroughCacheRule {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "credentialArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_arn: Option<String>,
    #[serde(rename = "customRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_role_arn: Option<String>,
    #[serde(rename = "ecrRepositoryPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecr_repository_prefix: Option<String>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
    #[serde(rename = "upstreamRegistry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstream_registry: Option<String>,
    #[serde(rename = "upstreamRegistryUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstream_registry_url: Option<String>,
    #[serde(rename = "upstreamRepositoryPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstream_repository_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRegistryRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRegistryResponse {
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "replicationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_configuration: Option<ReplicationConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationConfiguration {
    #[serde(default)]
    pub rules: Vec<ReplicationRule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationRule {
    #[serde(default)]
    pub destinations: Vec<ReplicationDestination>,
    #[serde(rename = "repositoryFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_filters: Option<Vec<RepositoryFilter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationDestination {
    #[serde(default)]
    pub region: String,
    #[serde(rename = "registryId")]
    #[serde(default)]
    pub registry_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RepositoryFilter {
    #[serde(default)]
    pub filter: String,
    #[serde(rename = "filterType")]
    #[serde(default)]
    pub filter_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRepositoriesRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRepositoriesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repositories: Option<Vec<Repository>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRepositoryCreationTemplatesRequest {
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
    pub prefixes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRepositoryCreationTemplatesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryCreationTemplates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_creation_templates: Option<Vec<RepositoryCreationTemplate>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccountSettingRequest {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccountSettingResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAuthorizationTokenRequest {
    #[serde(rename = "registryIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAuthorizationTokenResponse {
    #[serde(rename = "authorizationData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_data: Option<Vec<AuthorizationData>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthorizationData {
    #[serde(rename = "authorizationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_token: Option<String>,
    #[serde(rename = "expiresAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<f64>,
    #[serde(rename = "proxyEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_endpoint: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDownloadUrlForLayerRequest {
    #[serde(rename = "layerDigest")]
    #[serde(default)]
    pub layer_digest: String,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDownloadUrlForLayerResponse {
    #[serde(rename = "downloadUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    #[serde(rename = "layerDigest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_digest: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLifecyclePolicyPreviewRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<LifecyclePolicyPreviewFilter>,
    #[serde(rename = "imageIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_ids: Option<Vec<ImageIdentifier>>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LifecyclePolicyPreviewFilter {
    #[serde(rename = "tagStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLifecyclePolicyPreviewResponse {
    #[serde(rename = "lifecyclePolicyText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy_text: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "previewResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_results: Option<Vec<LifecyclePolicyPreviewResult>>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<LifecyclePolicyPreviewSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LifecyclePolicyPreviewResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<LifecyclePolicyRuleAction>,
    #[serde(rename = "appliedRulePriority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_rule_priority: Option<i32>,
    #[serde(rename = "imageDigest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_digest: Option<String>,
    #[serde(rename = "imagePushedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pushed_at: Option<f64>,
    #[serde(rename = "imageTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tags: Option<Vec<String>>,
    #[serde(rename = "storageClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LifecyclePolicyRuleAction {
    #[serde(rename = "targetStorageClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_storage_class: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LifecyclePolicyPreviewSummary {
    #[serde(rename = "expiringImageTotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiring_image_total_count: Option<i32>,
    #[serde(rename = "transitioningImageTotalCounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transitioning_image_total_counts: Option<Vec<TransitioningImageTotalCount>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransitioningImageTotalCount {
    #[serde(rename = "imageTotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_total_count: Option<i32>,
    #[serde(rename = "targetStorageClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_storage_class: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLifecyclePolicyRequest {
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLifecyclePolicyResponse {
    #[serde(rename = "lastEvaluatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_evaluated_at: Option<f64>,
    #[serde(rename = "lifecyclePolicyText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy_text: Option<String>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRegistryPolicyRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRegistryPolicyResponse {
    #[serde(rename = "policyText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_text: Option<String>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRegistryScanningConfigurationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRegistryScanningConfigurationResponse {
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "scanningConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scanning_configuration: Option<RegistryScanningConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegistryScanningConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<RegistryScanningRule>>,
    #[serde(rename = "scanType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegistryScanningRule {
    #[serde(rename = "repositoryFilters")]
    #[serde(default)]
    pub repository_filters: Vec<ScanningRepositoryFilter>,
    #[serde(rename = "scanFrequency")]
    #[serde(default)]
    pub scan_frequency: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRepositoryPolicyRequest {
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRepositoryPolicyResponse {
    #[serde(rename = "policyText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_text: Option<String>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSigningConfigurationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSigningConfigurationResponse {
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "signingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_configuration: Option<SigningConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InitiateLayerUploadRequest {
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InitiateLayerUploadResponse {
    #[serde(rename = "partSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_size: Option<i64>,
    #[serde(rename = "uploadId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListImageReferrersRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ListImageReferrersFilter>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
    #[serde(rename = "subjectId")]
    #[serde(default)]
    pub subject_id: SubjectIdentifier,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListImageReferrersFilter {
    #[serde(rename = "artifactStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_status: Option<String>,
    #[serde(rename = "artifactTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubjectIdentifier {
    #[serde(rename = "imageDigest")]
    #[serde(default)]
    pub image_digest: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListImageReferrersResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrers: Option<Vec<ImageReferrer>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImageReferrer {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "artifactStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_status: Option<String>,
    #[serde(rename = "artifactType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[serde(rename = "mediaType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListImagesRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ListImagesFilter>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListImagesFilter {
    #[serde(rename = "imageStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_status: Option<String>,
    #[serde(rename = "tagStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListImagesResponse {
    #[serde(rename = "imageIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_ids: Option<Vec<ImageIdentifier>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPullTimeUpdateExclusionsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPullTimeUpdateExclusionsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "pullTimeUpdateExclusions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_time_update_exclusions: Option<Vec<String>>,
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
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAccountSettingRequest {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAccountSettingResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutImageRequest {
    #[serde(rename = "imageDigest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_digest: Option<String>,
    #[serde(rename = "imageManifest")]
    #[serde(default)]
    pub image_manifest: String,
    #[serde(rename = "imageManifestMediaType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_manifest_media_type: Option<String>,
    #[serde(rename = "imageTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tag: Option<String>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutImageResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutImageScanningConfigurationRequest {
    #[serde(rename = "imageScanningConfiguration")]
    #[serde(default)]
    pub image_scanning_configuration: ImageScanningConfiguration,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutImageScanningConfigurationResponse {
    #[serde(rename = "imageScanningConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_scanning_configuration: Option<ImageScanningConfiguration>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutImageTagMutabilityRequest {
    #[serde(rename = "imageTagMutability")]
    #[serde(default)]
    pub image_tag_mutability: String,
    #[serde(rename = "imageTagMutabilityExclusionFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tag_mutability_exclusion_filters: Option<Vec<ImageTagMutabilityExclusionFilter>>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutImageTagMutabilityResponse {
    #[serde(rename = "imageTagMutability")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tag_mutability: Option<String>,
    #[serde(rename = "imageTagMutabilityExclusionFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tag_mutability_exclusion_filters: Option<Vec<ImageTagMutabilityExclusionFilter>>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutLifecyclePolicyRequest {
    #[serde(rename = "lifecyclePolicyText")]
    #[serde(default)]
    pub lifecycle_policy_text: String,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutLifecyclePolicyResponse {
    #[serde(rename = "lifecyclePolicyText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy_text: Option<String>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRegistryPolicyRequest {
    #[serde(rename = "policyText")]
    #[serde(default)]
    pub policy_text: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRegistryPolicyResponse {
    #[serde(rename = "policyText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_text: Option<String>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRegistryScanningConfigurationRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<RegistryScanningRule>>,
    #[serde(rename = "scanType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRegistryScanningConfigurationResponse {
    #[serde(rename = "registryScanningConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_scanning_configuration: Option<RegistryScanningConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutReplicationConfigurationRequest {
    #[serde(rename = "replicationConfiguration")]
    #[serde(default)]
    pub replication_configuration: ReplicationConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutReplicationConfigurationResponse {
    #[serde(rename = "replicationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_configuration: Option<ReplicationConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutSigningConfigurationRequest {
    #[serde(rename = "signingConfiguration")]
    #[serde(default)]
    pub signing_configuration: SigningConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutSigningConfigurationResponse {
    #[serde(rename = "signingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_configuration: Option<SigningConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterPullTimeUpdateExclusionRequest {
    #[serde(rename = "principalArn")]
    #[serde(default)]
    pub principal_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterPullTimeUpdateExclusionResponse {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "principalArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetRepositoryPolicyRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[serde(rename = "policyText")]
    #[serde(default)]
    pub policy_text: String,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetRepositoryPolicyResponse {
    #[serde(rename = "policyText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_text: Option<String>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartImageScanRequest {
    #[serde(rename = "imageId")]
    #[serde(default)]
    pub image_id: ImageIdentifier,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartImageScanResponse {
    #[serde(rename = "imageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<ImageIdentifier>,
    #[serde(rename = "imageScanStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_scan_status: Option<ImageScanStatus>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartLifecyclePolicyPreviewRequest {
    #[serde(rename = "lifecyclePolicyText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy_text: Option<String>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartLifecyclePolicyPreviewResponse {
    #[serde(rename = "lifecyclePolicyText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy_text: Option<String>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    pub tags: Vec<Tag>,
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

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateImageStorageClassRequest {
    #[serde(rename = "imageId")]
    #[serde(default)]
    pub image_id: ImageIdentifier,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
    #[serde(rename = "targetStorageClass")]
    #[serde(default)]
    pub target_storage_class: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateImageStorageClassResponse {
    #[serde(rename = "imageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<ImageIdentifier>,
    #[serde(rename = "imageStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_status: Option<String>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePullThroughCacheRuleRequest {
    #[serde(rename = "credentialArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_arn: Option<String>,
    #[serde(rename = "customRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_role_arn: Option<String>,
    #[serde(rename = "ecrRepositoryPrefix")]
    #[serde(default)]
    pub ecr_repository_prefix: String,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePullThroughCacheRuleResponse {
    #[serde(rename = "credentialArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_arn: Option<String>,
    #[serde(rename = "customRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_role_arn: Option<String>,
    #[serde(rename = "ecrRepositoryPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecr_repository_prefix: Option<String>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
    #[serde(rename = "upstreamRepositoryPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstream_repository_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRepositoryCreationTemplateRequest {
    #[serde(rename = "appliedFor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_for: Option<Vec<String>>,
    #[serde(rename = "customRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "encryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfigurationForRepositoryCreationTemplate>,
    #[serde(rename = "imageTagMutability")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tag_mutability: Option<String>,
    #[serde(rename = "imageTagMutabilityExclusionFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_tag_mutability_exclusion_filters: Option<Vec<ImageTagMutabilityExclusionFilter>>,
    #[serde(rename = "lifecyclePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy: Option<String>,
    #[serde(default)]
    pub prefix: String,
    #[serde(rename = "repositoryPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_policy: Option<String>,
    #[serde(rename = "resourceTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRepositoryCreationTemplateResponse {
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryCreationTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_creation_template: Option<RepositoryCreationTemplate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UploadLayerPartRequest {
    #[serde(rename = "layerPartBlob")]
    #[serde(default)]
    pub layer_part_blob: String,
    #[serde(rename = "partFirstByte")]
    #[serde(default)]
    pub part_first_byte: i64,
    #[serde(rename = "partLastByte")]
    #[serde(default)]
    pub part_last_byte: i64,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
    #[serde(rename = "uploadId")]
    #[serde(default)]
    pub upload_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UploadLayerPartResponse {
    #[serde(rename = "lastByteReceived")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_byte_received: Option<i64>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
    #[serde(rename = "uploadId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidatePullThroughCacheRuleRequest {
    #[serde(rename = "ecrRepositoryPrefix")]
    #[serde(default)]
    pub ecr_repository_prefix: String,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidatePullThroughCacheRuleResponse {
    #[serde(rename = "credentialArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_arn: Option<String>,
    #[serde(rename = "customRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_role_arn: Option<String>,
    #[serde(rename = "ecrRepositoryPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecr_repository_prefix: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure: Option<String>,
    #[serde(rename = "isValid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_valid: Option<bool>,
    #[serde(rename = "registryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_id: Option<String>,
    #[serde(rename = "upstreamRegistryUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstream_registry_url: Option<String>,
    #[serde(rename = "upstreamRepositoryPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstream_repository_prefix: Option<String>,
}
