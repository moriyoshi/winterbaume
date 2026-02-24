//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-codeartifact

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateExternalConnectionRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateExternalConnectionResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<RepositoryDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RepositoryDescription {
    #[serde(rename = "administratorAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrator_account: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "domainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "domainOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_owner: Option<String>,
    #[serde(rename = "externalConnections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_connections: Option<Vec<RepositoryExternalConnectionInfo>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstreams: Option<Vec<UpstreamRepositoryInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RepositoryExternalConnectionInfo {
    #[serde(rename = "externalConnectionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_connection_name: Option<String>,
    #[serde(rename = "packageFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_format: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpstreamRepositoryInfo {
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CopyPackageVersionsRequest {
    #[serde(rename = "allowOverwrite")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_overwrite: Option<bool>,
    #[serde(rename = "includeFromUpstream")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_from_upstream: Option<bool>,
    #[serde(rename = "versionRevisions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_revisions: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CopyPackageVersionsResult {
    #[serde(rename = "failedVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_versions: Option<std::collections::HashMap<String, PackageVersionError>>,
    #[serde(rename = "successfulVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_versions:
        Option<std::collections::HashMap<String, SuccessfulPackageVersionInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageVersionError {
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SuccessfulPackageVersionInfo {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDomainRequest {
    #[serde(rename = "encryptionKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDomainResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<DomainDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainDescription {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "assetSizeBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_size_bytes: Option<i64>,
    #[serde(rename = "createdTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "encryptionKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "repositoryCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_count: Option<i32>,
    #[serde(rename = "s3BucketArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePackageGroupRequest {
    #[serde(rename = "contactInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_info: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "packageGroup")]
    #[serde(default)]
    pub package_group: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePackageGroupResult {
    #[serde(rename = "packageGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_group: Option<PackageGroupDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageGroupDescription {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "contactInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_info: Option<String>,
    #[serde(rename = "createdTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "domainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "domainOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_owner: Option<String>,
    #[serde(rename = "originConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_configuration: Option<PackageGroupOriginConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<PackageGroupReference>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageGroupOriginConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<std::collections::HashMap<String, PackageGroupOriginRestriction>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageGroupOriginRestriction {
    #[serde(rename = "effectiveMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_mode: Option<String>,
    #[serde(rename = "inheritedFrom")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherited_from: Option<PackageGroupReference>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "repositoriesCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repositories_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageGroupReference {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRepositoryRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstreams: Option<Vec<UpstreamRepository>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpstreamRepository {
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    pub repository_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRepositoryResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<RepositoryDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDomainPermissionsPolicyRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDomainPermissionsPolicyResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<ResourcePolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourcePolicy {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDomainRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDomainResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<DomainDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePackageGroupRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePackageGroupResult {
    #[serde(rename = "packageGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_group: Option<PackageGroupDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePackageRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePackageResult {
    #[serde(rename = "deletedPackage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_package: Option<PackageSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "originConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_configuration: Option<PackageOriginConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageOriginConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<PackageOriginRestrictions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageOriginRestrictions {
    #[serde(default)]
    pub publish: String,
    #[serde(default)]
    pub upstream: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePackageVersionsRequest {
    #[serde(rename = "expectedStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_status: Option<String>,
    #[serde(default)]
    pub versions: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePackageVersionsResult {
    #[serde(rename = "failedVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_versions: Option<std::collections::HashMap<String, PackageVersionError>>,
    #[serde(rename = "successfulVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_versions:
        Option<std::collections::HashMap<String, SuccessfulPackageVersionInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRepositoryPermissionsPolicyRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRepositoryPermissionsPolicyResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<ResourcePolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRepositoryRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRepositoryResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<RepositoryDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDomainRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDomainResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<DomainDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePackageGroupRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePackageGroupResult {
    #[serde(rename = "packageGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_group: Option<PackageGroupDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePackageRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePackageResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<PackageDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageDescription {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "originConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_configuration: Option<PackageOriginConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePackageVersionRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePackageVersionResult {
    #[serde(rename = "packageVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_version: Option<PackageVersionDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageVersionDescription {
    #[serde(rename = "displayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "homePage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_page: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub licenses: Option<Vec<LicenseInfo>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<PackageVersionOrigin>,
    #[serde(rename = "packageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
    #[serde(rename = "publishedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    #[serde(rename = "sourceCodeRepository")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code_repository: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LicenseInfo {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageVersionOrigin {
    #[serde(rename = "domainEntryPoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_entry_point: Option<DomainEntryPoint>,
    #[serde(rename = "originType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainEntryPoint {
    #[serde(rename = "externalConnectionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_connection_name: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRepositoryRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRepositoryResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<RepositoryDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateExternalConnectionRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateExternalConnectionResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<RepositoryDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisposePackageVersionsRequest {
    #[serde(rename = "expectedStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_status: Option<String>,
    #[serde(rename = "versionRevisions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_revisions: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    pub versions: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisposePackageVersionsResult {
    #[serde(rename = "failedVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_versions: Option<std::collections::HashMap<String, PackageVersionError>>,
    #[serde(rename = "successfulVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_versions:
        Option<std::collections::HashMap<String, SuccessfulPackageVersionInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAssociatedPackageGroupRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAssociatedPackageGroupResult {
    #[serde(rename = "associationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_type: Option<String>,
    #[serde(rename = "packageGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_group: Option<PackageGroupDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAuthorizationTokenRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAuthorizationTokenResult {
    #[serde(rename = "authorizationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDomainPermissionsPolicyRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDomainPermissionsPolicyResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<ResourcePolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPackageVersionAssetRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPackageVersionAssetResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset: Option<String>,
    #[serde(rename = "assetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_name: Option<String>,
    #[serde(rename = "packageVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_version: Option<String>,
    #[serde(rename = "packageVersionRevision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_version_revision: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPackageVersionReadmeRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPackageVersionReadmeResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "versionRevision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_revision: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRepositoryEndpointRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRepositoryEndpointResult {
    #[serde(rename = "repositoryEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_endpoint: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRepositoryPermissionsPolicyRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRepositoryPermissionsPolicyResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<ResourcePolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAllowedRepositoriesForGroupRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAllowedRepositoriesForGroupResult {
    #[serde(rename = "allowedRepositories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_repositories: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAssociatedPackagesRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAssociatedPackagesResult {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages: Option<Vec<AssociatedPackage>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociatedPackage {
    #[serde(rename = "associationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDomainsRequest {
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
pub struct ListDomainsResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<DomainSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "encryptionKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPackageGroupsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPackageGroupsResult {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "packageGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_groups: Option<Vec<PackageGroupSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageGroupSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "contactInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_info: Option<String>,
    #[serde(rename = "createdTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "domainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "domainOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_owner: Option<String>,
    #[serde(rename = "originConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_configuration: Option<PackageGroupOriginConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<PackageGroupReference>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPackageVersionAssetsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPackageVersionAssetsResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<AssetSummary>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "versionRevision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_revision: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hashes: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPackageVersionDependenciesRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPackageVersionDependenciesResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<PackageDependency>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "versionRevision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_revision: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageDependency {
    #[serde(rename = "dependencyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependency_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<String>,
    #[serde(rename = "versionRequirement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_requirement: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPackageVersionsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPackageVersionsResult {
    #[serde(rename = "defaultDisplayVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_display_version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<PackageVersionSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageVersionSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<PackageVersionOrigin>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPackagesRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPackagesResult {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages: Option<Vec<PackageSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRepositoriesInDomainRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRepositoriesInDomainResult {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repositories: Option<Vec<RepositorySummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RepositorySummary {
    #[serde(rename = "administratorAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrator_account: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "domainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "domainOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_owner: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRepositoriesRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRepositoriesResult {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repositories: Option<Vec<RepositorySummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSubPackageGroupsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSubPackageGroupsResult {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "packageGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_groups: Option<Vec<PackageGroupSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PublishPackageVersionRequest {
    #[serde(rename = "assetContent")]
    #[serde(default)]
    pub asset_content: String,
    #[serde(rename = "assetSHA256")]
    #[serde(default)]
    pub asset_s_h_a256: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PublishPackageVersionResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset: Option<AssetSummary>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "versionRevision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_revision: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutDomainPermissionsPolicyRequest {
    #[serde(default)]
    pub domain: String,
    #[serde(rename = "domainOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_owner: Option<String>,
    #[serde(rename = "policyDocument")]
    #[serde(default)]
    pub policy_document: String,
    #[serde(rename = "policyRevision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_revision: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutDomainPermissionsPolicyResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<ResourcePolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutPackageOriginConfigurationRequest {
    #[serde(default)]
    pub restrictions: PackageOriginRestrictions,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutPackageOriginConfigurationResult {
    #[serde(rename = "originConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_configuration: Option<PackageOriginConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRepositoryPermissionsPolicyRequest {
    #[serde(rename = "policyDocument")]
    #[serde(default)]
    pub policy_document: String,
    #[serde(rename = "policyRevision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_revision: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRepositoryPermissionsPolicyResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<ResourcePolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "tagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResult {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePackageGroupOriginConfigurationRequest {
    #[serde(rename = "addAllowedRepositories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_allowed_repositories: Option<Vec<PackageGroupAllowedRepository>>,
    #[serde(rename = "removeAllowedRepositories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_allowed_repositories: Option<Vec<PackageGroupAllowedRepository>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageGroupAllowedRepository {
    #[serde(rename = "originRestrictionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_restriction_type: Option<String>,
    #[serde(rename = "repositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePackageGroupOriginConfigurationResult {
    #[serde(rename = "allowedRepositoryUpdates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_repository_updates:
        Option<std::collections::HashMap<String, std::collections::HashMap<String, Vec<String>>>>,
    #[serde(rename = "packageGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_group: Option<PackageGroupDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePackageGroupRequest {
    #[serde(rename = "contactInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_info: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "packageGroup")]
    #[serde(default)]
    pub package_group: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePackageGroupResult {
    #[serde(rename = "packageGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_group: Option<PackageGroupDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePackageVersionsStatusRequest {
    #[serde(rename = "expectedStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_status: Option<String>,
    #[serde(rename = "targetStatus")]
    #[serde(default)]
    pub target_status: String,
    #[serde(rename = "versionRevisions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_revisions: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    pub versions: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePackageVersionsStatusResult {
    #[serde(rename = "failedVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_versions: Option<std::collections::HashMap<String, PackageVersionError>>,
    #[serde(rename = "successfulVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_versions:
        Option<std::collections::HashMap<String, SuccessfulPackageVersionInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRepositoryRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstreams: Option<Vec<UpstreamRepository>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRepositoryResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<RepositoryDescription>,
}
