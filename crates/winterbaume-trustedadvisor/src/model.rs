//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-trustedadvisor

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateRecommendationResourceExclusionRequest {
    #[serde(rename = "recommendationResourceExclusions")]
    #[serde(default)]
    pub recommendation_resource_exclusions: Vec<RecommendationResourceExclusion>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecommendationResourceExclusion {
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "isExcluded")]
    #[serde(default)]
    pub is_excluded: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateRecommendationResourceExclusionResponse {
    #[serde(rename = "batchUpdateRecommendationResourceExclusionErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_update_recommendation_resource_exclusion_errors:
        Option<Vec<UpdateRecommendationResourceExclusionError>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRecommendationResourceExclusionError {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
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
pub struct GetOrganizationRecommendationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOrganizationRecommendationResponse {
    #[serde(rename = "organizationRecommendation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_recommendation: Option<OrganizationRecommendation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationRecommendation {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "awsServices")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_services: Option<Vec<String>>,
    #[serde(rename = "checkArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "createdBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    #[serde(rename = "lifecycleStage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_stage: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "pillarSpecificAggregates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pillar_specific_aggregates: Option<RecommendationPillarSpecificAggregates>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pillars: Option<Vec<String>>,
    #[serde(rename = "resolvedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_at: Option<String>,
    #[serde(rename = "resourcesAggregates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources_aggregates: Option<RecommendationResourcesAggregates>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "updateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_reason: Option<String>,
    #[serde(rename = "updateReasonCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_reason_code: Option<String>,
    #[serde(rename = "updatedOnBehalfOf")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_on_behalf_of: Option<String>,
    #[serde(rename = "updatedOnBehalfOfJobTitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_on_behalf_of_job_title: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecommendationPillarSpecificAggregates {
    #[serde(rename = "costOptimizing")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_optimizing: Option<RecommendationCostOptimizingAggregates>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecommendationCostOptimizingAggregates {
    #[serde(rename = "estimatedMonthlySavings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_monthly_savings: Option<f64>,
    #[serde(rename = "estimatedPercentMonthlySavings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_percent_monthly_savings: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecommendationResourcesAggregates {
    #[serde(rename = "errorCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_count: Option<i64>,
    #[serde(rename = "excludedCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_count: Option<i64>,
    #[serde(rename = "okCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ok_count: Option<i64>,
    #[serde(rename = "warningCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRecommendationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRecommendationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation: Option<Recommendation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Recommendation {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "awsServices")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_services: Option<Vec<String>>,
    #[serde(rename = "checkArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "createdBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    #[serde(rename = "lifecycleStage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_stage: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "pillarSpecificAggregates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pillar_specific_aggregates: Option<RecommendationPillarSpecificAggregates>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pillars: Option<Vec<String>>,
    #[serde(rename = "resolvedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_at: Option<String>,
    #[serde(rename = "resourcesAggregates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources_aggregates: Option<RecommendationResourcesAggregates>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "updateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_reason: Option<String>,
    #[serde(rename = "updateReasonCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_reason_code: Option<String>,
    #[serde(rename = "updatedOnBehalfOf")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_on_behalf_of: Option<String>,
    #[serde(rename = "updatedOnBehalfOfJobTitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_on_behalf_of_job_title: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListChecksRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListChecksResponse {
    #[serde(rename = "checkSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_summaries: Option<Vec<CheckSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CheckSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "awsServices")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_services: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pillars: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOrganizationRecommendationAccountsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOrganizationRecommendationAccountsResponse {
    #[serde(rename = "accountRecommendationLifecycleSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_recommendation_lifecycle_summaries:
        Option<Vec<AccountRecommendationLifecycleSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountRecommendationLifecycleSummary {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "accountRecommendationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_recommendation_arn: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    #[serde(rename = "lifecycleStage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_stage: Option<String>,
    #[serde(rename = "updateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_reason: Option<String>,
    #[serde(rename = "updateReasonCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_reason_code: Option<String>,
    #[serde(rename = "updatedOnBehalfOf")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_on_behalf_of: Option<String>,
    #[serde(rename = "updatedOnBehalfOfJobTitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_on_behalf_of_job_title: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOrganizationRecommendationResourcesRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOrganizationRecommendationResourcesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "organizationRecommendationResourceSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_recommendation_resource_summaries:
        Option<Vec<OrganizationRecommendationResourceSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationRecommendationResourceSummary {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "awsResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_resource_id: Option<String>,
    #[serde(rename = "exclusionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusion_status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "recommendationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_arn: Option<String>,
    #[serde(rename = "regionCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOrganizationRecommendationsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOrganizationRecommendationsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "organizationRecommendationSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_recommendation_summaries: Option<Vec<OrganizationRecommendationSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationRecommendationSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "awsServices")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_services: Option<Vec<String>>,
    #[serde(rename = "checkArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    #[serde(rename = "lifecycleStage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_stage: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "pillarSpecificAggregates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pillar_specific_aggregates: Option<RecommendationPillarSpecificAggregates>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pillars: Option<Vec<String>>,
    #[serde(rename = "resourcesAggregates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources_aggregates: Option<RecommendationResourcesAggregates>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRecommendationResourcesRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRecommendationResourcesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "recommendationResourceSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_resource_summaries: Option<Vec<RecommendationResourceSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecommendationResourceSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "awsResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_resource_id: Option<String>,
    #[serde(rename = "exclusionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusion_status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "recommendationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_arn: Option<String>,
    #[serde(rename = "regionCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRecommendationsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRecommendationsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "recommendationSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_summaries: Option<Vec<RecommendationSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecommendationSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "awsServices")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_services: Option<Vec<String>>,
    #[serde(rename = "checkArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    #[serde(rename = "lifecycleStage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_stage: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "pillarSpecificAggregates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pillar_specific_aggregates: Option<RecommendationPillarSpecificAggregates>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pillars: Option<Vec<String>>,
    #[serde(rename = "resourcesAggregates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources_aggregates: Option<RecommendationResourcesAggregates>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateOrganizationRecommendationLifecycleRequest {
    #[serde(rename = "lifecycleStage")]
    #[serde(default)]
    pub lifecycle_stage: String,
    #[serde(rename = "updateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_reason: Option<String>,
    #[serde(rename = "updateReasonCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_reason_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRecommendationLifecycleRequest {
    #[serde(rename = "lifecycleStage")]
    #[serde(default)]
    pub lifecycle_stage: String,
    #[serde(rename = "updateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_reason: Option<String>,
    #[serde(rename = "updateReasonCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_reason_code: Option<String>,
}
