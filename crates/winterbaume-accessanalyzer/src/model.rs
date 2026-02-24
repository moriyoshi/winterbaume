//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-accessanalyzer

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplyArchiveRuleRequest {
    #[serde(rename = "analyzerArn")]
    #[serde(default)]
    pub analyzer_arn: String,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "ruleName")]
    #[serde(default)]
    pub rule_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelPolicyGenerationRequest {
    #[serde(rename = "jobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelPolicyGenerationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CheckAccessNotGrantedRequest {
    #[serde(default)]
    pub access: Vec<Access>,
    #[serde(rename = "policyDocument")]
    #[serde(default)]
    pub policy_document: String,
    #[serde(rename = "policyType")]
    #[serde(default)]
    pub policy_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Access {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CheckAccessNotGrantedResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasons: Option<Vec<ReasonSummary>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReasonSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "statementId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_id: Option<String>,
    #[serde(rename = "statementIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_index: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CheckNoNewAccessRequest {
    #[serde(rename = "existingPolicyDocument")]
    #[serde(default)]
    pub existing_policy_document: String,
    #[serde(rename = "newPolicyDocument")]
    #[serde(default)]
    pub new_policy_document: String,
    #[serde(rename = "policyType")]
    #[serde(default)]
    pub policy_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CheckNoNewAccessResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasons: Option<Vec<ReasonSummary>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CheckNoPublicAccessRequest {
    #[serde(rename = "policyDocument")]
    #[serde(default)]
    pub policy_document: String,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    pub resource_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CheckNoPublicAccessResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasons: Option<Vec<ReasonSummary>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAccessPreviewRequest {
    #[serde(rename = "analyzerArn")]
    #[serde(default)]
    pub analyzer_arn: String,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    pub configurations: std::collections::HashMap<String, Configuration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Configuration {
    #[serde(rename = "dynamodbStream")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamodb_stream: Option<DynamodbStreamConfiguration>,
    #[serde(rename = "dynamodbTable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamodb_table: Option<DynamodbTableConfiguration>,
    #[serde(rename = "ebsSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs_snapshot: Option<EbsSnapshotConfiguration>,
    #[serde(rename = "ecrRepository")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecr_repository: Option<EcrRepositoryConfiguration>,
    #[serde(rename = "efsFileSystem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efs_file_system: Option<EfsFileSystemConfiguration>,
    #[serde(rename = "iamRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role: Option<IamRoleConfiguration>,
    #[serde(rename = "kmsKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<KmsKeyConfiguration>,
    #[serde(rename = "rdsDbClusterSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_db_cluster_snapshot: Option<RdsDbClusterSnapshotConfiguration>,
    #[serde(rename = "rdsDbSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_db_snapshot: Option<RdsDbSnapshotConfiguration>,
    #[serde(rename = "s3Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket: Option<S3BucketConfiguration>,
    #[serde(rename = "s3ExpressDirectoryBucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_express_directory_bucket: Option<S3ExpressDirectoryBucketConfiguration>,
    #[serde(rename = "secretsManagerSecret")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_secret: Option<SecretsManagerSecretConfiguration>,
    #[serde(rename = "snsTopic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic: Option<SnsTopicConfiguration>,
    #[serde(rename = "sqsQueue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sqs_queue: Option<SqsQueueConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DynamodbStreamConfiguration {
    #[serde(rename = "streamPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DynamodbTableConfiguration {
    #[serde(rename = "tablePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EbsSnapshotConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    #[serde(rename = "kmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "userIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EcrRepositoryConfiguration {
    #[serde(rename = "repositoryPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EfsFileSystemConfiguration {
    #[serde(rename = "fileSystemPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IamRoleConfiguration {
    #[serde(rename = "trustPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KmsKeyConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grants: Option<Vec<KmsGrantConfiguration>>,
    #[serde(rename = "keyPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_policies: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KmsGrantConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<KmsGrantConstraints>,
    #[serde(rename = "granteePrincipal")]
    #[serde(default)]
    pub grantee_principal: String,
    #[serde(rename = "issuingAccount")]
    #[serde(default)]
    pub issuing_account: String,
    #[serde(default)]
    pub operations: Vec<String>,
    #[serde(rename = "retiringPrincipal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retiring_principal: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KmsGrantConstraints {
    #[serde(rename = "encryptionContextEquals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context_equals: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "encryptionContextSubset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_context_subset: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RdsDbClusterSnapshotConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, RdsDbClusterSnapshotAttributeValue>>,
    #[serde(rename = "kmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RdsDbClusterSnapshotAttributeValue {
    #[serde(rename = "accountIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RdsDbSnapshotConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, RdsDbSnapshotAttributeValue>>,
    #[serde(rename = "kmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RdsDbSnapshotAttributeValue {
    #[serde(rename = "accountIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3BucketConfiguration {
    #[serde(rename = "accessPoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_points: Option<std::collections::HashMap<String, S3AccessPointConfiguration>>,
    #[serde(rename = "bucketAclGrants")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_acl_grants: Option<Vec<S3BucketAclGrantConfiguration>>,
    #[serde(rename = "bucketPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_policy: Option<String>,
    #[serde(rename = "bucketPublicAccessBlock")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_public_access_block: Option<S3PublicAccessBlockConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3AccessPointConfiguration {
    #[serde(rename = "accessPointPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_policy: Option<String>,
    #[serde(rename = "networkOrigin")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_origin: Option<NetworkOriginConfiguration>,
    #[serde(rename = "publicAccessBlock")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access_block: Option<S3PublicAccessBlockConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkOriginConfiguration {
    #[serde(rename = "internetConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internet_configuration: Option<InternetConfiguration>,
    #[serde(rename = "vpcConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_configuration: Option<VpcConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InternetConfiguration {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcConfiguration {
    #[serde(rename = "vpcId")]
    #[serde(default)]
    pub vpc_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3PublicAccessBlockConfiguration {
    #[serde(rename = "ignorePublicAcls")]
    #[serde(default)]
    pub ignore_public_acls: bool,
    #[serde(rename = "restrictPublicBuckets")]
    #[serde(default)]
    pub restrict_public_buckets: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3BucketAclGrantConfiguration {
    #[serde(default)]
    pub grantee: AclGrantee,
    #[serde(default)]
    pub permission: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AclGrantee {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3ExpressDirectoryBucketConfiguration {
    #[serde(rename = "accessPoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_points:
        Option<std::collections::HashMap<String, S3ExpressDirectoryAccessPointConfiguration>>,
    #[serde(rename = "bucketPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3ExpressDirectoryAccessPointConfiguration {
    #[serde(rename = "accessPointPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_policy: Option<String>,
    #[serde(rename = "networkOrigin")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_origin: Option<NetworkOriginConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecretsManagerSecretConfiguration {
    #[serde(rename = "kmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "secretPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnsTopicConfiguration {
    #[serde(rename = "topicPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SqsQueueConfiguration {
    #[serde(rename = "queuePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAccessPreviewResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAnalyzerRequest {
    #[serde(rename = "analyzerName")]
    #[serde(default)]
    pub analyzer_name: String,
    #[serde(rename = "archiveRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_rules: Option<Vec<InlineArchiveRule>>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<AnalyzerConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InlineArchiveRule {
    #[serde(default)]
    pub filter: std::collections::HashMap<String, Criterion>,
    #[serde(rename = "ruleName")]
    #[serde(default)]
    pub rule_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Criterion {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eq: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exists: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neq: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyzerConfiguration {
    #[serde(rename = "internalAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_access: Option<InternalAccessConfiguration>,
    #[serde(rename = "unusedAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unused_access: Option<UnusedAccessConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InternalAccessConfiguration {
    #[serde(rename = "analysisRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_rule: Option<InternalAccessAnalysisRule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InternalAccessAnalysisRule {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusions: Option<Vec<InternalAccessAnalysisRuleCriteria>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InternalAccessAnalysisRuleCriteria {
    #[serde(rename = "accountIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    #[serde(rename = "resourceArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arns: Option<Vec<String>>,
    #[serde(rename = "resourceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnusedAccessConfiguration {
    #[serde(rename = "analysisRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_rule: Option<AnalysisRule>,
    #[serde(rename = "unusedAccessAge")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unused_access_age: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalysisRule {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusions: Option<Vec<AnalysisRuleCriteria>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalysisRuleCriteria {
    #[serde(rename = "accountIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    #[serde(rename = "resourceTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<Vec<std::collections::HashMap<String, String>>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAnalyzerResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateArchiveRuleRequest {
    #[serde(rename = "analyzerName")]
    #[serde(default)]
    pub analyzer_name: String,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    pub filter: std::collections::HashMap<String, Criterion>,
    #[serde(rename = "ruleName")]
    #[serde(default)]
    pub rule_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAnalyzerRequest {
    #[serde(rename = "analyzerName")]
    #[serde(default)]
    pub analyzer_name: String,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteArchiveRuleRequest {
    #[serde(rename = "analyzerName")]
    #[serde(default)]
    pub analyzer_name: String,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "ruleName")]
    #[serde(default)]
    pub rule_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GenerateFindingRecommendationRequest {
    #[serde(rename = "analyzerArn")]
    #[serde(default)]
    pub analyzer_arn: String,
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccessPreviewRequest {
    #[serde(rename = "accessPreviewId")]
    #[serde(default)]
    pub access_preview_id: String,
    #[serde(rename = "analyzerArn")]
    #[serde(default)]
    pub analyzer_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccessPreviewResponse {
    #[serde(rename = "accessPreview")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_preview: Option<AccessPreview>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessPreview {
    #[serde(rename = "analyzerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyzer_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<std::collections::HashMap<String, Configuration>>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<AccessPreviewStatusReason>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessPreviewStatusReason {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAnalyzedResourceRequest {
    #[serde(rename = "analyzerArn")]
    #[serde(default)]
    pub analyzer_arn: String,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAnalyzedResourceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<AnalyzedResource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyzedResource {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
    #[serde(rename = "analyzedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyzed_at: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "isPublic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "resourceOwnerAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner_account: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "sharedVia")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_via: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAnalyzerRequest {
    #[serde(rename = "analyzerName")]
    #[serde(default)]
    pub analyzer_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAnalyzerResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<AnalyzerSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyzerSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<AnalyzerConfiguration>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "lastResourceAnalyzed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_resource_analyzed: Option<String>,
    #[serde(rename = "lastResourceAnalyzedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_resource_analyzed_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<StatusReason>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StatusReason {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetArchiveRuleRequest {
    #[serde(rename = "analyzerName")]
    #[serde(default)]
    pub analyzer_name: String,
    #[serde(rename = "ruleName")]
    #[serde(default)]
    pub rule_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetArchiveRuleResponse {
    #[serde(rename = "archiveRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_rule: Option<ArchiveRuleSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ArchiveRuleSummary {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<std::collections::HashMap<String, Criterion>>,
    #[serde(rename = "ruleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFindingRecommendationRequest {
    #[serde(rename = "analyzerArn")]
    #[serde(default)]
    pub analyzer_arn: String,
    #[serde(default)]
    pub id: String,
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
pub struct GetFindingRecommendationResponse {
    #[serde(rename = "completedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<RecommendationError>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "recommendationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_type: Option<String>,
    #[serde(rename = "recommendedSteps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_steps: Option<Vec<RecommendedStep>>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecommendationError {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecommendedStep {
    #[serde(rename = "unusedPermissionsRecommendedStep")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unused_permissions_recommended_step: Option<UnusedPermissionsRecommendedStep>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnusedPermissionsRecommendedStep {
    #[serde(rename = "existingPolicyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub existing_policy_id: Option<String>,
    #[serde(rename = "policyUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_updated_at: Option<String>,
    #[serde(rename = "recommendedAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_action: Option<String>,
    #[serde(rename = "recommendedPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFindingRequest {
    #[serde(rename = "analyzerArn")]
    #[serde(default)]
    pub analyzer_arn: String,
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFindingResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding: Option<Finding>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Finding {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Vec<String>>,
    #[serde(rename = "analyzedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyzed_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isPublic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(rename = "resourceControlPolicyRestriction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_control_policy_restriction: Option<String>,
    #[serde(rename = "resourceOwnerAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner_account: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<FindingSource>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FindingSource {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<FindingSourceDetail>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FindingSourceDetail {
    #[serde(rename = "accessPointAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_account: Option<String>,
    #[serde(rename = "accessPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFindingV2Request {
    #[serde(rename = "analyzerArn")]
    #[serde(default)]
    pub analyzer_arn: String,
    #[serde(default)]
    pub id: String,
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
pub struct GetFindingV2Response {
    #[serde(rename = "analyzedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyzed_at: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "findingDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_details: Option<Vec<FindingDetails>>,
    #[serde(rename = "findingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(rename = "resourceOwnerAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner_account: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FindingDetails {
    #[serde(rename = "externalAccessDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_access_details: Option<ExternalAccessDetails>,
    #[serde(rename = "internalAccessDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_access_details: Option<InternalAccessDetails>,
    #[serde(rename = "unusedIamRoleDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unused_iam_role_details: Option<UnusedIamRoleDetails>,
    #[serde(rename = "unusedIamUserAccessKeyDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unused_iam_user_access_key_details: Option<UnusedIamUserAccessKeyDetails>,
    #[serde(rename = "unusedIamUserPasswordDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unused_iam_user_password_details: Option<UnusedIamUserPasswordDetails>,
    #[serde(rename = "unusedPermissionDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unused_permission_details: Option<UnusedPermissionDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExternalAccessDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "isPublic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "resourceControlPolicyRestriction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_control_policy_restriction: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<FindingSource>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InternalAccessDetails {
    #[serde(rename = "accessType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "principalOwnerAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_owner_account: Option<String>,
    #[serde(rename = "principalType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<String>,
    #[serde(rename = "resourceControlPolicyRestriction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_control_policy_restriction: Option<String>,
    #[serde(rename = "serviceControlPolicyRestriction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_control_policy_restriction: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<FindingSource>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnusedIamRoleDetails {
    #[serde(rename = "lastAccessed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_accessed: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnusedIamUserAccessKeyDetails {
    #[serde(rename = "accessKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    #[serde(rename = "lastAccessed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_accessed: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnusedIamUserPasswordDetails {
    #[serde(rename = "lastAccessed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_accessed: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnusedPermissionDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<UnusedAction>>,
    #[serde(rename = "lastAccessed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_accessed: Option<String>,
    #[serde(rename = "serviceNamespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_namespace: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnusedAction {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "lastAccessed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_accessed: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFindingsStatisticsRequest {
    #[serde(rename = "analyzerArn")]
    #[serde(default)]
    pub analyzer_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFindingsStatisticsResponse {
    #[serde(rename = "findingsStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub findings_statistics: Option<Vec<FindingsStatistics>>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FindingsStatistics {
    #[serde(rename = "externalAccessFindingsStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_access_findings_statistics: Option<ExternalAccessFindingsStatistics>,
    #[serde(rename = "internalAccessFindingsStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_access_findings_statistics: Option<InternalAccessFindingsStatistics>,
    #[serde(rename = "unusedAccessFindingsStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unused_access_findings_statistics: Option<UnusedAccessFindingsStatistics>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExternalAccessFindingsStatistics {
    #[serde(rename = "resourceTypeStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type_statistics: Option<std::collections::HashMap<String, ResourceTypeDetails>>,
    #[serde(rename = "totalActiveFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_active_findings: Option<i32>,
    #[serde(rename = "totalArchivedFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_archived_findings: Option<i32>,
    #[serde(rename = "totalResolvedFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_resolved_findings: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceTypeDetails {
    #[serde(rename = "totalActiveCrossAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_active_cross_account: Option<i32>,
    #[serde(rename = "totalActiveErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_active_errors: Option<i32>,
    #[serde(rename = "totalActivePublic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_active_public: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InternalAccessFindingsStatistics {
    #[serde(rename = "resourceTypeStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type_statistics:
        Option<std::collections::HashMap<String, InternalAccessResourceTypeDetails>>,
    #[serde(rename = "totalActiveFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_active_findings: Option<i32>,
    #[serde(rename = "totalArchivedFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_archived_findings: Option<i32>,
    #[serde(rename = "totalResolvedFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_resolved_findings: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InternalAccessResourceTypeDetails {
    #[serde(rename = "totalActiveFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_active_findings: Option<i32>,
    #[serde(rename = "totalArchivedFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_archived_findings: Option<i32>,
    #[serde(rename = "totalResolvedFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_resolved_findings: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnusedAccessFindingsStatistics {
    #[serde(rename = "topAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_accounts: Option<Vec<FindingAggregationAccountDetails>>,
    #[serde(rename = "totalActiveFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_active_findings: Option<i32>,
    #[serde(rename = "totalArchivedFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_archived_findings: Option<i32>,
    #[serde(rename = "totalResolvedFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_resolved_findings: Option<i32>,
    #[serde(rename = "unusedAccessTypeStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unused_access_type_statistics: Option<Vec<UnusedAccessTypeStatistics>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FindingAggregationAccountDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<std::collections::HashMap<String, i32>>,
    #[serde(rename = "numberOfActiveFindings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_active_findings: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnusedAccessTypeStatistics {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    #[serde(rename = "unusedAccessType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unused_access_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetGeneratedPolicyRequest {
    #[serde(rename = "includeResourcePlaceholders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_resource_placeholders: Option<bool>,
    #[serde(rename = "includeServiceLevelTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_service_level_template: Option<bool>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetGeneratedPolicyResponse {
    #[serde(rename = "generatedPolicyResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_policy_result: Option<GeneratedPolicyResult>,
    #[serde(rename = "jobDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_details: Option<JobDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeneratedPolicyResult {
    #[serde(rename = "generatedPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_policies: Option<Vec<GeneratedPolicy>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<GeneratedPolicyProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeneratedPolicy {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeneratedPolicyProperties {
    #[serde(rename = "cloudTrailProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_trail_properties: Option<CloudTrailProperties>,
    #[serde(rename = "isComplete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_complete: Option<bool>,
    #[serde(rename = "principalArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudTrailProperties {
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "trailProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trail_properties: Option<Vec<TrailProperties>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrailProperties {
    #[serde(rename = "allRegions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_regions: Option<bool>,
    #[serde(rename = "cloudTrailArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_trail_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobDetails {
    #[serde(rename = "completedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_on: Option<String>,
    #[serde(rename = "jobError")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_error: Option<JobError>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "startedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_on: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobError {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccessPreviewFindingsRequest {
    #[serde(rename = "accessPreviewId")]
    #[serde(default)]
    pub access_preview_id: String,
    #[serde(rename = "analyzerArn")]
    #[serde(default)]
    pub analyzer_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<std::collections::HashMap<String, Criterion>>,
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
pub struct ListAccessPreviewFindingsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub findings: Option<Vec<AccessPreviewFinding>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessPreviewFinding {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Vec<String>>,
    #[serde(rename = "changeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "existingFindingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub existing_finding_id: Option<String>,
    #[serde(rename = "existingFindingStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub existing_finding_status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isPublic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(rename = "resourceControlPolicyRestriction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_control_policy_restriction: Option<String>,
    #[serde(rename = "resourceOwnerAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner_account: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<FindingSource>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccessPreviewsRequest {
    #[serde(rename = "analyzerArn")]
    #[serde(default)]
    pub analyzer_arn: String,
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
pub struct ListAccessPreviewsResponse {
    #[serde(rename = "accessPreviews")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_previews: Option<Vec<AccessPreviewSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessPreviewSummary {
    #[serde(rename = "analyzerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyzer_arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<AccessPreviewStatusReason>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAnalyzedResourcesRequest {
    #[serde(rename = "analyzerArn")]
    #[serde(default)]
    pub analyzer_arn: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAnalyzedResourcesResponse {
    #[serde(rename = "analyzedResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyzed_resources: Option<Vec<AnalyzedResourceSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyzedResourceSummary {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "resourceOwnerAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner_account: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAnalyzersRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAnalyzersResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyzers: Option<Vec<AnalyzerSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListArchiveRulesRequest {
    #[serde(rename = "analyzerName")]
    #[serde(default)]
    pub analyzer_name: String,
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
pub struct ListArchiveRulesResponse {
    #[serde(rename = "archiveRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_rules: Option<Vec<ArchiveRuleSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFindingsRequest {
    #[serde(rename = "analyzerArn")]
    #[serde(default)]
    pub analyzer_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<std::collections::HashMap<String, Criterion>>,
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
    pub sort: Option<SortCriteria>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SortCriteria {
    #[serde(rename = "attributeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[serde(rename = "orderBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFindingsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub findings: Option<Vec<FindingSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FindingSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Vec<String>>,
    #[serde(rename = "analyzedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyzed_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "isPublic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(rename = "resourceControlPolicyRestriction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_control_policy_restriction: Option<String>,
    #[serde(rename = "resourceOwnerAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner_account: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<FindingSource>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFindingsV2Request {
    #[serde(rename = "analyzerArn")]
    #[serde(default)]
    pub analyzer_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<std::collections::HashMap<String, Criterion>>,
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
    pub sort: Option<SortCriteria>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFindingsV2Response {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub findings: Option<Vec<FindingSummaryV2>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FindingSummaryV2 {
    #[serde(rename = "analyzedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyzed_at: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "findingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(rename = "resourceOwnerAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner_account: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPolicyGenerationsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "principalArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPolicyGenerationsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "policyGenerations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_generations: Option<Vec<PolicyGeneration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PolicyGeneration {
    #[serde(rename = "completedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_on: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "principalArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_arn: Option<String>,
    #[serde(rename = "startedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_on: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
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
pub struct StartPolicyGenerationRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "cloudTrailDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_trail_details: Option<CloudTrailDetails>,
    #[serde(rename = "policyGenerationDetails")]
    #[serde(default)]
    pub policy_generation_details: PolicyGenerationDetails,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudTrailDetails {
    #[serde(rename = "accessRole")]
    #[serde(default)]
    pub access_role: String,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    pub start_time: String,
    #[serde(default)]
    pub trails: Vec<Trail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Trail {
    #[serde(rename = "allRegions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_regions: Option<bool>,
    #[serde(rename = "cloudTrailArn")]
    #[serde(default)]
    pub cloud_trail_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PolicyGenerationDetails {
    #[serde(rename = "principalArn")]
    #[serde(default)]
    pub principal_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartPolicyGenerationResponse {
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartResourceScanRequest {
    #[serde(rename = "analyzerArn")]
    #[serde(default)]
    pub analyzer_arn: String,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "resourceOwnerAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner_account: Option<String>,
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

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAnalyzerRequest {
    #[serde(rename = "analyzerName")]
    #[serde(default)]
    pub analyzer_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<AnalyzerConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAnalyzerResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<AnalyzerConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateArchiveRuleRequest {
    #[serde(rename = "analyzerName")]
    #[serde(default)]
    pub analyzer_name: String,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    pub filter: std::collections::HashMap<String, Criterion>,
    #[serde(rename = "ruleName")]
    #[serde(default)]
    pub rule_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFindingsRequest {
    #[serde(rename = "analyzerArn")]
    #[serde(default)]
    pub analyzer_arn: String,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidatePolicyRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "policyDocument")]
    #[serde(default)]
    pub policy_document: String,
    #[serde(rename = "policyType")]
    #[serde(default)]
    pub policy_type: String,
    #[serde(rename = "validatePolicyResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_policy_resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidatePolicyResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub findings: Option<Vec<ValidatePolicyFinding>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidatePolicyFinding {
    #[serde(rename = "findingDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_details: Option<String>,
    #[serde(rename = "findingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_type: Option<String>,
    #[serde(rename = "issueCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_code: Option<String>,
    #[serde(rename = "learnMoreLink")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub learn_more_link: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<Location>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Location {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<Vec<PathElement>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span: Option<Span>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PathElement {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substring: Option<Substring>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Substring {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Span {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<Position>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<Position>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Position {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}
