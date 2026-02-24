//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-codegurureviewer

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateRepositoryRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "KMSKeyDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_key_details: Option<KMSKeyDetails>,
    #[serde(rename = "Repository")]
    #[serde(default)]
    pub repository: Repository,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KMSKeyDetails {
    #[serde(rename = "EncryptionOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_option: Option<String>,
    #[serde(rename = "KMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_key_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Repository {
    #[serde(rename = "Bitbucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitbucket: Option<ThirdPartySourceRepository>,
    #[serde(rename = "CodeCommit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_commit: Option<CodeCommitRepository>,
    #[serde(rename = "GitHubEnterpriseServer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_hub_enterprise_server: Option<ThirdPartySourceRepository>,
    #[serde(rename = "S3Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket: Option<S3Repository>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThirdPartySourceRepository {
    #[serde(rename = "ConnectionArn")]
    #[serde(default)]
    pub connection_arn: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Owner")]
    #[serde(default)]
    pub owner: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeCommitRepository {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Repository {
    #[serde(rename = "BucketName")]
    #[serde(default)]
    pub bucket_name: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateRepositoryResponse {
    #[serde(rename = "RepositoryAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_association: Option<RepositoryAssociation>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RepositoryAssociation {
    #[serde(rename = "AssociationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_arn: Option<String>,
    #[serde(rename = "AssociationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    #[serde(rename = "ConnectionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_arn: Option<String>,
    #[serde(rename = "CreatedTimeStamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time_stamp: Option<f64>,
    #[serde(rename = "KMSKeyDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_key_details: Option<KMSKeyDetails>,
    #[serde(rename = "LastUpdatedTimeStamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time_stamp: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Owner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "ProviderType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<String>,
    #[serde(rename = "S3RepositoryDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_repository_details: Option<S3RepositoryDetails>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3RepositoryDetails {
    #[serde(rename = "BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    #[serde(rename = "CodeArtifacts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_artifacts: Option<CodeArtifacts>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeArtifacts {
    #[serde(rename = "BuildArtifactsObjectKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_artifacts_object_key: Option<String>,
    #[serde(rename = "SourceCodeArtifactsObjectKey")]
    #[serde(default)]
    pub source_code_artifacts_object_key: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCodeReviewRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RepositoryAssociationArn")]
    #[serde(default)]
    pub repository_association_arn: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: CodeReviewType,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeReviewType {
    #[serde(rename = "AnalysisTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_types: Option<Vec<String>>,
    #[serde(rename = "RepositoryAnalysis")]
    #[serde(default)]
    pub repository_analysis: RepositoryAnalysis,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RepositoryAnalysis {
    #[serde(rename = "RepositoryHead")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_head: Option<RepositoryHeadSourceCodeType>,
    #[serde(rename = "SourceCodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code_type: Option<SourceCodeType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RepositoryHeadSourceCodeType {
    #[serde(rename = "BranchName")]
    #[serde(default)]
    pub branch_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceCodeType {
    #[serde(rename = "BranchDiff")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_diff: Option<BranchDiffSourceCodeType>,
    #[serde(rename = "CommitDiff")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_diff: Option<CommitDiffSourceCodeType>,
    #[serde(rename = "RepositoryHead")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_head: Option<RepositoryHeadSourceCodeType>,
    #[serde(rename = "RequestMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_metadata: Option<RequestMetadata>,
    #[serde(rename = "S3BucketRepository")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_repository: Option<S3BucketRepository>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BranchDiffSourceCodeType {
    #[serde(rename = "DestinationBranchName")]
    #[serde(default)]
    pub destination_branch_name: String,
    #[serde(rename = "SourceBranchName")]
    #[serde(default)]
    pub source_branch_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CommitDiffSourceCodeType {
    #[serde(rename = "DestinationCommit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_commit: Option<String>,
    #[serde(rename = "MergeBaseCommit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_base_commit: Option<String>,
    #[serde(rename = "SourceCommit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_commit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RequestMetadata {
    #[serde(rename = "EventInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_info: Option<EventInfo>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Requester")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester: Option<String>,
    #[serde(rename = "VendorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventInfo {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3BucketRepository {
    #[serde(rename = "Details")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<S3RepositoryDetails>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCodeReviewResponse {
    #[serde(rename = "CodeReview")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_review: Option<CodeReview>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeReview {
    #[serde(rename = "AnalysisTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_types: Option<Vec<String>>,
    #[serde(rename = "AssociationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_arn: Option<String>,
    #[serde(rename = "CodeReviewArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_review_arn: Option<String>,
    #[serde(rename = "ConfigFileState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_file_state: Option<String>,
    #[serde(rename = "CreatedTimeStamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time_stamp: Option<f64>,
    #[serde(rename = "LastUpdatedTimeStamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time_stamp: Option<f64>,
    #[serde(rename = "Metrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Metrics>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Owner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "ProviderType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<String>,
    #[serde(rename = "PullRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_id: Option<String>,
    #[serde(rename = "RepositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
    #[serde(rename = "SourceCodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code_type: Option<SourceCodeType>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Metrics {
    #[serde(rename = "FindingsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub findings_count: Option<i64>,
    #[serde(rename = "MeteredLinesOfCodeCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metered_lines_of_code_count: Option<i64>,
    #[serde(rename = "SuppressedLinesOfCodeCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppressed_lines_of_code_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCodeReviewRequest {
    #[serde(rename = "CodeReviewArn")]
    #[serde(default)]
    pub code_review_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCodeReviewResponse {
    #[serde(rename = "CodeReview")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_review: Option<CodeReview>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRecommendationFeedbackRequest {
    #[serde(rename = "CodeReviewArn")]
    #[serde(default)]
    pub code_review_arn: String,
    #[serde(rename = "RecommendationId")]
    #[serde(default)]
    pub recommendation_id: String,
    #[serde(rename = "UserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRecommendationFeedbackResponse {
    #[serde(rename = "RecommendationFeedback")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_feedback: Option<RecommendationFeedback>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecommendationFeedback {
    #[serde(rename = "CodeReviewArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_review_arn: Option<String>,
    #[serde(rename = "CreatedTimeStamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time_stamp: Option<f64>,
    #[serde(rename = "LastUpdatedTimeStamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time_stamp: Option<f64>,
    #[serde(rename = "Reactions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactions: Option<Vec<String>>,
    #[serde(rename = "RecommendationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_id: Option<String>,
    #[serde(rename = "UserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRepositoryAssociationRequest {
    #[serde(rename = "AssociationArn")]
    #[serde(default)]
    pub association_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRepositoryAssociationResponse {
    #[serde(rename = "RepositoryAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_association: Option<RepositoryAssociation>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateRepositoryRequest {
    #[serde(rename = "AssociationArn")]
    #[serde(default)]
    pub association_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateRepositoryResponse {
    #[serde(rename = "RepositoryAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_association: Option<RepositoryAssociation>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCodeReviewsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ProviderTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_types: Option<Vec<String>>,
    #[serde(rename = "RepositoryNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_names: Option<Vec<String>>,
    #[serde(rename = "States")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<String>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCodeReviewsResponse {
    #[serde(rename = "CodeReviewSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_review_summaries: Option<Vec<CodeReviewSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeReviewSummary {
    #[serde(rename = "CodeReviewArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_review_arn: Option<String>,
    #[serde(rename = "CreatedTimeStamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time_stamp: Option<f64>,
    #[serde(rename = "LastUpdatedTimeStamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time_stamp: Option<f64>,
    #[serde(rename = "MetricsSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_summary: Option<MetricsSummary>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Owner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "ProviderType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<String>,
    #[serde(rename = "PullRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_request_id: Option<String>,
    #[serde(rename = "RepositoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
    #[serde(rename = "SourceCodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_code_type: Option<SourceCodeType>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricsSummary {
    #[serde(rename = "FindingsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub findings_count: Option<i64>,
    #[serde(rename = "MeteredLinesOfCodeCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metered_lines_of_code_count: Option<i64>,
    #[serde(rename = "SuppressedLinesOfCodeCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppressed_lines_of_code_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRecommendationFeedbackRequest {
    #[serde(rename = "CodeReviewArn")]
    #[serde(default)]
    pub code_review_arn: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RecommendationIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_ids: Option<Vec<String>>,
    #[serde(rename = "UserIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRecommendationFeedbackResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RecommendationFeedbackSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_feedback_summaries: Option<Vec<RecommendationFeedbackSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecommendationFeedbackSummary {
    #[serde(rename = "Reactions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactions: Option<Vec<String>>,
    #[serde(rename = "RecommendationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_id: Option<String>,
    #[serde(rename = "UserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRecommendationsRequest {
    #[serde(rename = "CodeReviewArn")]
    #[serde(default)]
    pub code_review_arn: String,
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
pub struct ListRecommendationsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RecommendationSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_summaries: Option<Vec<RecommendationSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecommendationSummary {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EndLine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_line: Option<i32>,
    #[serde(rename = "FilePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(rename = "RecommendationCategory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_category: Option<String>,
    #[serde(rename = "RecommendationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_id: Option<String>,
    #[serde(rename = "RuleMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_metadata: Option<RuleMetadata>,
    #[serde(rename = "Severity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(rename = "StartLine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_line: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleMetadata {
    #[serde(rename = "LongDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_description: Option<String>,
    #[serde(rename = "RuleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    #[serde(rename = "RuleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    #[serde(rename = "RuleTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_tags: Option<Vec<String>>,
    #[serde(rename = "ShortDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRepositoryAssociationsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "Names")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Owners")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owners: Option<Vec<String>>,
    #[serde(rename = "ProviderTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_types: Option<Vec<String>>,
    #[serde(rename = "States")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRepositoryAssociationsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RepositoryAssociationSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_association_summaries: Option<Vec<RepositoryAssociationSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RepositoryAssociationSummary {
    #[serde(rename = "AssociationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_arn: Option<String>,
    #[serde(rename = "AssociationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    #[serde(rename = "ConnectionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_arn: Option<String>,
    #[serde(rename = "LastUpdatedTimeStamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time_stamp: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Owner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "ProviderType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_type: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRecommendationFeedbackRequest {
    #[serde(rename = "CodeReviewArn")]
    #[serde(default)]
    pub code_review_arn: String,
    #[serde(rename = "Reactions")]
    #[serde(default)]
    pub reactions: Vec<String>,
    #[serde(rename = "RecommendationId")]
    #[serde(default)]
    pub recommendation_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRecommendationFeedbackResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}
