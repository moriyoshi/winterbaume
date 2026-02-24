//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-macie2

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceptInvitationRequest {
    #[serde(rename = "administratorAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrator_account_id: Option<String>,
    #[serde(rename = "invitationId")]
    #[serde(default)]
    pub invitation_id: String,
    #[serde(rename = "masterAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_account: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceptInvitationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetCustomDataIdentifiersRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetCustomDataIdentifiersResponse {
    #[serde(rename = "customDataIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data_identifiers: Option<Vec<BatchGetCustomDataIdentifierSummary>>,
    #[serde(rename = "notFoundIdentifierIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_found_identifier_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetCustomDataIdentifierSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateAutomatedDiscoveryAccountsRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<AutomatedDiscoveryAccountUpdate>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedDiscoveryAccountUpdate {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateAutomatedDiscoveryAccountsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<AutomatedDiscoveryAccountUpdateError>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedDiscoveryAccountUpdateError {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAllowListRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    pub client_token: String,
    #[serde(default)]
    pub criteria: AllowListCriteria,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AllowListCriteria {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
    #[serde(rename = "s3WordsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_words_list: Option<S3WordsList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3WordsList {
    #[serde(rename = "bucketName")]
    #[serde(default)]
    pub bucket_name: String,
    #[serde(rename = "objectKey")]
    #[serde(default)]
    pub object_key: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAllowListResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateClassificationJobRequest {
    #[serde(rename = "allowListIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_list_ids: Option<Vec<String>>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    pub client_token: String,
    #[serde(rename = "customDataIdentifierIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data_identifier_ids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "initialRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_run: Option<bool>,
    #[serde(rename = "jobType")]
    #[serde(default)]
    pub job_type: String,
    #[serde(rename = "managedDataIdentifierIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_data_identifier_ids: Option<Vec<String>>,
    #[serde(rename = "managedDataIdentifierSelector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_data_identifier_selector: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "s3JobDefinition")]
    #[serde(default)]
    pub s3_job_definition: S3JobDefinition,
    #[serde(rename = "samplingPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_percentage: Option<i32>,
    #[serde(rename = "scheduleFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_frequency: Option<JobScheduleFrequency>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3JobDefinition {
    #[serde(rename = "bucketCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_criteria: Option<S3BucketCriteriaForJob>,
    #[serde(rename = "bucketDefinitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_definitions: Option<Vec<S3BucketDefinitionForJob>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scoping: Option<Scoping>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3BucketCriteriaForJob {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excludes: Option<CriteriaBlockForJob>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<CriteriaBlockForJob>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CriteriaBlockForJob {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<CriteriaForJob>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CriteriaForJob {
    #[serde(rename = "simpleCriterion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_criterion: Option<SimpleCriterionForJob>,
    #[serde(rename = "tagCriterion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_criterion: Option<TagCriterionForJob>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SimpleCriterionForJob {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparator: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagCriterionForJob {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparator: Option<String>,
    #[serde(rename = "tagValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<Vec<TagCriterionPairForJob>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagCriterionPairForJob {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3BucketDefinitionForJob {
    #[serde(rename = "accountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(default)]
    pub buckets: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Scoping {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excludes: Option<JobScopingBlock>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<JobScopingBlock>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobScopingBlock {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<JobScopeTerm>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobScopeTerm {
    #[serde(rename = "simpleScopeTerm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_scope_term: Option<SimpleScopeTerm>,
    #[serde(rename = "tagScopeTerm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_scope_term: Option<TagScopeTerm>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SimpleScopeTerm {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparator: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagScopeTerm {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparator: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "tagValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<Vec<TagValuePair>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagValuePair {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobScheduleFrequency {
    #[serde(rename = "dailySchedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_schedule: Option<DailySchedule>,
    #[serde(rename = "monthlySchedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_schedule: Option<MonthlySchedule>,
    #[serde(rename = "weeklySchedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_schedule: Option<WeeklySchedule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DailySchedule {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MonthlySchedule {
    #[serde(rename = "dayOfMonth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_month: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WeeklySchedule {
    #[serde(rename = "dayOfWeek")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateClassificationJobResponse {
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCustomDataIdentifierRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ignoreWords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_words: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    #[serde(rename = "maximumMatchDistance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_match_distance: Option<i32>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub regex: String,
    #[serde(rename = "severityLevels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_levels: Option<Vec<SeverityLevel>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SeverityLevel {
    #[serde(rename = "occurrencesThreshold")]
    #[serde(default)]
    pub occurrences_threshold: i64,
    #[serde(default)]
    pub severity: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCustomDataIdentifierResponse {
    #[serde(rename = "customDataIdentifierId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data_identifier_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFindingsFilterRequest {
    #[serde(default)]
    pub action: String,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "findingCriteria")]
    #[serde(default)]
    pub finding_criteria: FindingCriteria,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FindingCriteria {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criterion: Option<std::collections::HashMap<String, CriterionAdditionalProperties>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CriterionAdditionalProperties {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eq: Option<Vec<String>>,
    #[serde(rename = "eqExactMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eq_exact_match: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gt: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gte: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lte: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neq: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFindingsFilterResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateInvitationsRequest {
    #[serde(rename = "accountIds")]
    #[serde(default)]
    pub account_ids: Vec<String>,
    #[serde(rename = "disableEmailNotification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_email_notification: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateInvitationsResponse {
    #[serde(rename = "unprocessedAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<UnprocessedAccount>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnprocessedAccount {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
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
pub struct CreateMemberRequest {
    #[serde(default)]
    pub account: AccountDetail,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountDetail {
    #[serde(rename = "accountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(default)]
    pub email: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMemberResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSampleFindingsRequest {
    #[serde(rename = "findingTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSampleFindingsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeclineInvitationsRequest {
    #[serde(rename = "accountIds")]
    #[serde(default)]
    pub account_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeclineInvitationsResponse {
    #[serde(rename = "unprocessedAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<UnprocessedAccount>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAllowListRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "ignoreJobChecks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_job_checks: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAllowListResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCustomDataIdentifierRequest {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCustomDataIdentifierResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFindingsFilterRequest {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFindingsFilterResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteInvitationsRequest {
    #[serde(rename = "accountIds")]
    #[serde(default)]
    pub account_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteInvitationsResponse {
    #[serde(rename = "unprocessedAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_accounts: Option<Vec<UnprocessedAccount>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMemberRequest {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMemberResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBucketsRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criteria: Option<std::collections::HashMap<String, BucketCriteriaAdditionalProperties>>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_criteria: Option<BucketSortCriteria>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BucketCriteriaAdditionalProperties {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eq: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gt: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gte: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lte: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neq: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BucketSortCriteria {
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
pub struct DescribeBucketsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buckets: Option<Vec<BucketMetadata>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BucketMetadata {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "allowsUnencryptedObjectUploads")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_unencrypted_object_uploads: Option<String>,
    #[serde(rename = "automatedDiscoveryMonitoringStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated_discovery_monitoring_status: Option<String>,
    #[serde(rename = "bucketArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_arn: Option<String>,
    #[serde(rename = "bucketCreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_created_at: Option<String>,
    #[serde(rename = "bucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    #[serde(rename = "classifiableObjectCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifiable_object_count: Option<i64>,
    #[serde(rename = "classifiableSizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifiable_size_in_bytes: Option<i64>,
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "jobDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_details: Option<JobDetails>,
    #[serde(rename = "lastAutomatedDiscoveryTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_automated_discovery_time: Option<String>,
    #[serde(rename = "lastUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    #[serde(rename = "objectCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_count: Option<i64>,
    #[serde(rename = "objectCountByEncryptionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_count_by_encryption_type: Option<ObjectCountByEncryptionType>,
    #[serde(rename = "publicAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access: Option<BucketPublicAccess>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "replicationDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_details: Option<ReplicationDetails>,
    #[serde(rename = "sensitivityScore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitivity_score: Option<i32>,
    #[serde(rename = "serverSideEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_side_encryption: Option<BucketServerSideEncryption>,
    #[serde(rename = "sharedAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_access: Option<String>,
    #[serde(rename = "sizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_bytes: Option<i64>,
    #[serde(rename = "sizeInBytesCompressed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_bytes_compressed: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<KeyValuePair>>,
    #[serde(rename = "unclassifiableObjectCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unclassifiable_object_count: Option<ObjectLevelStatistics>,
    #[serde(rename = "unclassifiableObjectSizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unclassifiable_object_size_in_bytes: Option<ObjectLevelStatistics>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioning: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobDetails {
    #[serde(rename = "isDefinedInJob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_defined_in_job: Option<String>,
    #[serde(rename = "isMonitoredByJob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_monitored_by_job: Option<String>,
    #[serde(rename = "lastJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_job_id: Option<String>,
    #[serde(rename = "lastJobRunTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_job_run_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ObjectCountByEncryptionType {
    #[serde(rename = "customerManaged")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_managed: Option<i64>,
    #[serde(rename = "kmsManaged")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_managed: Option<i64>,
    #[serde(rename = "s3Managed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_managed: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unencrypted: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BucketPublicAccess {
    #[serde(rename = "effectivePermission")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_permission: Option<String>,
    #[serde(rename = "permissionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_configuration: Option<BucketPermissionConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BucketPermissionConfiguration {
    #[serde(rename = "accountLevelPermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_level_permissions: Option<AccountLevelPermissions>,
    #[serde(rename = "bucketLevelPermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_level_permissions: Option<BucketLevelPermissions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountLevelPermissions {
    #[serde(rename = "blockPublicAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_public_access: Option<BlockPublicAccess>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BlockPublicAccess {
    #[serde(rename = "blockPublicAcls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_public_acls: Option<bool>,
    #[serde(rename = "blockPublicPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_public_policy: Option<bool>,
    #[serde(rename = "ignorePublicAcls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_public_acls: Option<bool>,
    #[serde(rename = "restrictPublicBuckets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrict_public_buckets: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BucketLevelPermissions {
    #[serde(rename = "accessControlList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_list: Option<AccessControlList>,
    #[serde(rename = "blockPublicAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_public_access: Option<BlockPublicAccess>,
    #[serde(rename = "bucketPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_policy: Option<BucketPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessControlList {
    #[serde(rename = "allowsPublicReadAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_public_read_access: Option<bool>,
    #[serde(rename = "allowsPublicWriteAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_public_write_access: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BucketPolicy {
    #[serde(rename = "allowsPublicReadAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_public_read_access: Option<bool>,
    #[serde(rename = "allowsPublicWriteAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_public_write_access: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicated: Option<bool>,
    #[serde(rename = "replicatedExternally")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicated_externally: Option<bool>,
    #[serde(rename = "replicationAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_accounts: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BucketServerSideEncryption {
    #[serde(rename = "kmsMasterKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_master_key_id: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KeyValuePair {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ObjectLevelStatistics {
    #[serde(rename = "fileType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<i64>,
    #[serde(rename = "storageClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeClassificationJobRequest {
    #[serde(rename = "jobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeClassificationJobResponse {
    #[serde(rename = "allowListIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_list_ids: Option<Vec<String>>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "customDataIdentifierIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data_identifier_ids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "initialRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_run: Option<bool>,
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "jobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "jobType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_type: Option<String>,
    #[serde(rename = "lastRunErrorStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_error_status: Option<LastRunErrorStatus>,
    #[serde(rename = "lastRunTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_time: Option<String>,
    #[serde(rename = "managedDataIdentifierIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_data_identifier_ids: Option<Vec<String>>,
    #[serde(rename = "managedDataIdentifierSelector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_data_identifier_selector: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "s3JobDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_job_definition: Option<S3JobDefinition>,
    #[serde(rename = "samplingPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_percentage: Option<i32>,
    #[serde(rename = "scheduleFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_frequency: Option<JobScheduleFrequency>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<Statistics>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "userPausedDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_paused_details: Option<UserPausedDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LastRunErrorStatus {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Statistics {
    #[serde(rename = "approximateNumberOfObjectsToProcess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approximate_number_of_objects_to_process: Option<f64>,
    #[serde(rename = "numberOfRuns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_runs: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserPausedDetails {
    #[serde(rename = "jobExpiresAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_expires_at: Option<String>,
    #[serde(rename = "jobImminentExpirationHealthEventArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_imminent_expiration_health_event_arn: Option<String>,
    #[serde(rename = "jobPausedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_paused_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeOrganizationConfigurationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeOrganizationConfigurationResponse {
    #[serde(rename = "autoEnable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_enable: Option<bool>,
    #[serde(rename = "maxAccountLimitReached")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_account_limit_reached: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableMacieRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableMacieResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableOrganizationAdminAccountRequest {
    #[serde(rename = "adminAccountId")]
    #[serde(default)]
    pub admin_account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableOrganizationAdminAccountResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateFromAdministratorAccountRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateFromAdministratorAccountResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateFromMasterAccountRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateFromMasterAccountResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateMemberRequest {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateMemberResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableMacieRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "findingPublishingFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_publishing_frequency: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableMacieResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableOrganizationAdminAccountRequest {
    #[serde(rename = "adminAccountId")]
    #[serde(default)]
    pub admin_account_id: String,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableOrganizationAdminAccountResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAdministratorAccountRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAdministratorAccountResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrator: Option<Invitation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Invitation {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "invitationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitation_id: Option<String>,
    #[serde(rename = "invitedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_at: Option<String>,
    #[serde(rename = "relationshipStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAllowListRequest {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAllowListResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criteria: Option<AllowListCriteria>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AllowListStatus>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AllowListStatus {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAutomatedDiscoveryConfigurationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAutomatedDiscoveryConfigurationResponse {
    #[serde(rename = "autoEnableOrganizationMembers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_enable_organization_members: Option<String>,
    #[serde(rename = "classificationScopeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_scope_id: Option<String>,
    #[serde(rename = "disabledAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_at: Option<String>,
    #[serde(rename = "firstEnabledAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_enabled_at: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    #[serde(rename = "sensitivityInspectionTemplateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitivity_inspection_template_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBucketStatisticsRequest {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBucketStatisticsResponse {
    #[serde(rename = "bucketCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_count: Option<i64>,
    #[serde(rename = "bucketCountByEffectivePermission")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_count_by_effective_permission: Option<BucketCountByEffectivePermission>,
    #[serde(rename = "bucketCountByEncryptionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_count_by_encryption_type: Option<BucketCountByEncryptionType>,
    #[serde(rename = "bucketCountByObjectEncryptionRequirement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_count_by_object_encryption_requirement:
        Option<BucketCountPolicyAllowsUnencryptedObjectUploads>,
    #[serde(rename = "bucketCountBySharedAccessType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_count_by_shared_access_type: Option<BucketCountBySharedAccessType>,
    #[serde(rename = "bucketStatisticsBySensitivity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_statistics_by_sensitivity: Option<BucketStatisticsBySensitivity>,
    #[serde(rename = "classifiableObjectCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifiable_object_count: Option<i64>,
    #[serde(rename = "classifiableSizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifiable_size_in_bytes: Option<i64>,
    #[serde(rename = "lastUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    #[serde(rename = "objectCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_count: Option<i64>,
    #[serde(rename = "sizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_bytes: Option<i64>,
    #[serde(rename = "sizeInBytesCompressed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_bytes_compressed: Option<i64>,
    #[serde(rename = "unclassifiableObjectCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unclassifiable_object_count: Option<ObjectLevelStatistics>,
    #[serde(rename = "unclassifiableObjectSizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unclassifiable_object_size_in_bytes: Option<ObjectLevelStatistics>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BucketCountByEffectivePermission {
    #[serde(rename = "publiclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<i64>,
    #[serde(rename = "publiclyReadable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_readable: Option<i64>,
    #[serde(rename = "publiclyWritable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_writable: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BucketCountByEncryptionType {
    #[serde(rename = "kmsManaged")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_managed: Option<i64>,
    #[serde(rename = "s3Managed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_managed: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unencrypted: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BucketCountPolicyAllowsUnencryptedObjectUploads {
    #[serde(rename = "allowsUnencryptedObjectUploads")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_unencrypted_object_uploads: Option<i64>,
    #[serde(rename = "deniesUnencryptedObjectUploads")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub denies_unencrypted_object_uploads: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BucketCountBySharedAccessType {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal: Option<i64>,
    #[serde(rename = "notShared")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_shared: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BucketStatisticsBySensitivity {
    #[serde(rename = "classificationError")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_error: Option<SensitivityAggregations>,
    #[serde(rename = "notClassified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_classified: Option<SensitivityAggregations>,
    #[serde(rename = "notSensitive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_sensitive: Option<SensitivityAggregations>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive: Option<SensitivityAggregations>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SensitivityAggregations {
    #[serde(rename = "classifiableSizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifiable_size_in_bytes: Option<i64>,
    #[serde(rename = "publiclyAccessibleCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible_count: Option<i64>,
    #[serde(rename = "totalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[serde(rename = "totalSizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_size_in_bytes: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetClassificationExportConfigurationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetClassificationExportConfigurationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ClassificationExportConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClassificationExportConfiguration {
    #[serde(rename = "s3Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_destination: Option<S3Destination>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Destination {
    #[serde(rename = "bucketName")]
    #[serde(default)]
    pub bucket_name: String,
    #[serde(rename = "expectedBucketOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_bucket_owner: Option<String>,
    #[serde(rename = "keyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_prefix: Option<String>,
    #[serde(rename = "kmsKeyArn")]
    #[serde(default)]
    pub kms_key_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetClassificationScopeRequest {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetClassificationScopeResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3: Option<S3ClassificationScope>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3ClassificationScope {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excludes: Option<S3ClassificationScopeExclusion>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3ClassificationScopeExclusion {
    #[serde(rename = "bucketNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCustomDataIdentifierRequest {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCustomDataIdentifierResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ignoreWords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_words: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    #[serde(rename = "maximumMatchDistance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_match_distance: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
    #[serde(rename = "severityLevels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_levels: Option<Vec<SeverityLevel>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFindingStatisticsRequest {
    #[serde(rename = "findingCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_criteria: Option<FindingCriteria>,
    #[serde(rename = "groupBy")]
    #[serde(default)]
    pub group_by: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "sortCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_criteria: Option<FindingStatisticsSortCriteria>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FindingStatisticsSortCriteria {
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
pub struct GetFindingStatisticsResponse {
    #[serde(rename = "countsByGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counts_by_group: Option<Vec<GroupCount>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GroupCount {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(rename = "groupKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFindingsFilterRequest {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFindingsFilterResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "findingCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_criteria: Option<FindingCriteria>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFindingsPublicationConfigurationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFindingsPublicationConfigurationResponse {
    #[serde(rename = "securityHubConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_hub_configuration: Option<SecurityHubConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecurityHubConfiguration {
    #[serde(rename = "publishClassificationFindings")]
    #[serde(default)]
    pub publish_classification_findings: bool,
    #[serde(rename = "publishPolicyFindings")]
    #[serde(default)]
    pub publish_policy_findings: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFindingsRequest {
    #[serde(rename = "findingIds")]
    #[serde(default)]
    pub finding_ids: Vec<String>,
    #[serde(rename = "sortCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_criteria: Option<SortCriteria>,
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
pub struct GetFindingsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub findings: Option<Vec<Finding>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Finding {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "classificationDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_details: Option<ClassificationDetails>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition: Option<String>,
    #[serde(rename = "policyDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_details: Option<PolicyDetails>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "resourcesAffected")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources_affected: Option<ResourcesAffected>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample: Option<bool>,
    #[serde(rename = "schemaVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<Severity>,
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
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClassificationDetails {
    #[serde(rename = "detailedResultsLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_results_location: Option<String>,
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "originType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<ClassificationResult>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClassificationResult {
    #[serde(rename = "additionalOccurrences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_occurrences: Option<bool>,
    #[serde(rename = "customDataIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data_identifiers: Option<CustomDataIdentifiers>,
    #[serde(rename = "mimeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(rename = "sensitiveData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive_data: Option<Vec<SensitiveDataItem>>,
    #[serde(rename = "sizeClassified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_classified: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ClassificationResultStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomDataIdentifiers {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detections: Option<Vec<CustomDetection>>,
    #[serde(rename = "totalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomDetection {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occurrences: Option<Occurrences>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Occurrences {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cells: Option<Vec<Cell>>,
    #[serde(rename = "lineRanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_ranges: Option<Vec<Range>>,
    #[serde(rename = "offsetRanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_ranges: Option<Vec<Range>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<Vec<Page>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<Record>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Cell {
    #[serde(rename = "cellReference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell_reference: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i64>,
    #[serde(rename = "columnName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Range {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,
    #[serde(rename = "startColumn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_column: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Page {
    #[serde(rename = "lineRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_range: Option<Range>,
    #[serde(rename = "offsetRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_range: Option<Range>,
    #[serde(rename = "pageNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Record {
    #[serde(rename = "jsonPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_path: Option<String>,
    #[serde(rename = "recordIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_index: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SensitiveDataItem {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detections: Option<Vec<DefaultDetection>>,
    #[serde(rename = "totalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DefaultDetection {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occurrences: Option<Occurrences>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClassificationResultStatus {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PolicyDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<FindingAction>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<FindingActor>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FindingAction {
    #[serde(rename = "actionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    #[serde(rename = "apiCallDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_call_details: Option<ApiCallDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApiCallDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<String>,
    #[serde(rename = "apiServiceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_service_name: Option<String>,
    #[serde(rename = "firstSeen")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_seen: Option<String>,
    #[serde(rename = "lastSeen")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FindingActor {
    #[serde(rename = "domainDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_details: Option<DomainDetails>,
    #[serde(rename = "ipAddressDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_details: Option<IpAddressDetails>,
    #[serde(rename = "userIdentity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_identity: Option<UserIdentity>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainDetails {
    #[serde(rename = "domainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IpAddressDetails {
    #[serde(rename = "ipAddressV4")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_v4: Option<String>,
    #[serde(rename = "ipCity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_city: Option<IpCity>,
    #[serde(rename = "ipCountry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_country: Option<IpCountry>,
    #[serde(rename = "ipGeoLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_geo_location: Option<IpGeoLocation>,
    #[serde(rename = "ipOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_owner: Option<IpOwner>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IpCity {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IpCountry {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IpGeoLocation {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lon: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IpOwner {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<String>,
    #[serde(rename = "asnOrg")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn_org: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserIdentity {
    #[serde(rename = "assumedRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assumed_role: Option<AssumedRole>,
    #[serde(rename = "awsAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account: Option<AwsAccount>,
    #[serde(rename = "awsService")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_service: Option<AwsService>,
    #[serde(rename = "federatedUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federated_user: Option<FederatedUser>,
    #[serde(rename = "iamUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_user: Option<IamUser>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<UserIdentityRoot>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssumedRole {
    #[serde(rename = "accessKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "principalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "sessionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_context: Option<SessionContext>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SessionContext {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<SessionContextAttributes>,
    #[serde(rename = "sessionIssuer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_issuer: Option<SessionIssuer>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SessionContextAttributes {
    #[serde(rename = "creationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "mfaAuthenticated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_authenticated: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SessionIssuer {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "principalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "userName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsAccount {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "principalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsService {
    #[serde(rename = "invokedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoked_by: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FederatedUser {
    #[serde(rename = "accessKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "principalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "sessionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_context: Option<SessionContext>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IamUser {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "principalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "userName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserIdentityRoot {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "principalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourcesAffected {
    #[serde(rename = "s3Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket: Option<S3Bucket>,
    #[serde(rename = "s3Object")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object: Option<S3Object>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Bucket {
    #[serde(rename = "allowsUnencryptedObjectUploads")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_unencrypted_object_uploads: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "defaultServerSideEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_server_side_encryption: Option<ServerSideEncryption>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<S3BucketOwner>,
    #[serde(rename = "publicAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access: Option<BucketPublicAccess>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<KeyValuePair>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServerSideEncryption {
    #[serde(rename = "encryptionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    #[serde(rename = "kmsMasterKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_master_key_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3BucketOwner {
    #[serde(rename = "displayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Object {
    #[serde(rename = "bucketArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_arn: Option<String>,
    #[serde(rename = "eTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "lastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "publicAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_access: Option<bool>,
    #[serde(rename = "serverSideEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_side_encryption: Option<ServerSideEncryption>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(rename = "storageClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<KeyValuePair>>,
    #[serde(rename = "versionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Severity {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInvitationsCountRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInvitationsCountResponse {
    #[serde(rename = "invitationsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitations_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMacieSessionRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMacieSessionResponse {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "findingPublishingFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_publishing_frequency: Option<String>,
    #[serde(rename = "serviceRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMasterAccountRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMasterAccountResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master: Option<Invitation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMemberRequest {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMemberResponse {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "administratorAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrator_account_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "invitedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_at: Option<String>,
    #[serde(rename = "masterAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_account_id: Option<String>,
    #[serde(rename = "relationshipStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourceProfileRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourceProfileResponse {
    #[serde(rename = "profileUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_updated_at: Option<String>,
    #[serde(rename = "sensitivityScore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitivity_score: Option<i32>,
    #[serde(rename = "sensitivityScoreOverridden")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitivity_score_overridden: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<ResourceStatistics>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceStatistics {
    #[serde(rename = "totalBytesClassified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_bytes_classified: Option<i64>,
    #[serde(rename = "totalDetections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_detections: Option<i64>,
    #[serde(rename = "totalDetectionsSuppressed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_detections_suppressed: Option<i64>,
    #[serde(rename = "totalItemsClassified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items_classified: Option<i64>,
    #[serde(rename = "totalItemsSensitive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items_sensitive: Option<i64>,
    #[serde(rename = "totalItemsSkipped")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items_skipped: Option<i64>,
    #[serde(rename = "totalItemsSkippedInvalidEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items_skipped_invalid_encryption: Option<i64>,
    #[serde(rename = "totalItemsSkippedInvalidKms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items_skipped_invalid_kms: Option<i64>,
    #[serde(rename = "totalItemsSkippedPermissionDenied")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items_skipped_permission_denied: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRevealConfigurationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRevealConfigurationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<RevealConfiguration>,
    #[serde(rename = "retrievalConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieval_configuration: Option<RetrievalConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RevealConfiguration {
    #[serde(rename = "kmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetrievalConfiguration {
    #[serde(rename = "externalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "retrievalMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieval_mode: Option<String>,
    #[serde(rename = "roleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSensitiveDataOccurrencesAvailabilityRequest {
    #[serde(rename = "findingId")]
    #[serde(default)]
    pub finding_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSensitiveDataOccurrencesAvailabilityResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasons: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSensitiveDataOccurrencesRequest {
    #[serde(rename = "findingId")]
    #[serde(default)]
    pub finding_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSensitiveDataOccurrencesResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "sensitiveDataOccurrences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive_data_occurrences:
        Option<std::collections::HashMap<String, Vec<DetectedDataDetails>>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectedDataDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSensitivityInspectionTemplateRequest {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSensitivityInspectionTemplateResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excludes: Option<SensitivityInspectionTemplateExcludes>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<SensitivityInspectionTemplateIncludes>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "sensitivityInspectionTemplateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitivity_inspection_template_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SensitivityInspectionTemplateExcludes {
    #[serde(rename = "managedDataIdentifierIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_data_identifier_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SensitivityInspectionTemplateIncludes {
    #[serde(rename = "allowListIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_list_ids: Option<Vec<String>>,
    #[serde(rename = "customDataIdentifierIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data_identifier_ids: Option<Vec<String>>,
    #[serde(rename = "managedDataIdentifierIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_data_identifier_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUsageStatisticsRequest {
    #[serde(rename = "filterBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_by: Option<Vec<UsageStatisticsFilter>>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<UsageStatisticsSortBy>,
    #[serde(rename = "timeRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_range: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UsageStatisticsFilter {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparator: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UsageStatisticsSortBy {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "orderBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUsageStatisticsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<UsageRecord>>,
    #[serde(rename = "timeRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_range: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UsageRecord {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "automatedDiscoveryFreeTrialStartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated_discovery_free_trial_start_date: Option<String>,
    #[serde(rename = "freeTrialStartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_trial_start_date: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Vec<UsageByAccount>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UsageByAccount {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "estimatedCost")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_cost: Option<String>,
    #[serde(rename = "serviceLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_limit: Option<ServiceLimit>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceLimit {
    #[serde(rename = "isServiceLimited")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_service_limited: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUsageTotalsRequest {
    #[serde(rename = "timeRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_range: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUsageTotalsResponse {
    #[serde(rename = "timeRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_range: Option<String>,
    #[serde(rename = "usageTotals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_totals: Option<Vec<UsageTotal>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UsageTotal {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "estimatedCost")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_cost: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAllowListsRequest {
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
pub struct ListAllowListsResponse {
    #[serde(rename = "allowLists")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_lists: Option<Vec<AllowListSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AllowListSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAutomatedDiscoveryAccountsRequest {
    #[serde(rename = "accountIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
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
pub struct ListAutomatedDiscoveryAccountsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<AutomatedDiscoveryAccount>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutomatedDiscoveryAccount {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListClassificationJobsRequest {
    #[serde(rename = "filterCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_criteria: Option<ListJobsFilterCriteria>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_criteria: Option<ListJobsSortCriteria>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListJobsFilterCriteria {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excludes: Option<Vec<ListJobsFilterTerm>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Vec<ListJobsFilterTerm>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListJobsFilterTerm {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparator: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListJobsSortCriteria {
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
pub struct ListClassificationJobsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<JobSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobSummary {
    #[serde(rename = "bucketCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_criteria: Option<S3BucketCriteriaForJob>,
    #[serde(rename = "bucketDefinitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_definitions: Option<Vec<S3BucketDefinitionForJob>>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "jobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "jobType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_type: Option<String>,
    #[serde(rename = "lastRunErrorStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_error_status: Option<LastRunErrorStatus>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "userPausedDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_paused_details: Option<UserPausedDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListClassificationScopesRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListClassificationScopesResponse {
    #[serde(rename = "classificationScopes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_scopes: Option<Vec<ClassificationScopeSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClassificationScopeSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCustomDataIdentifiersRequest {
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
pub struct ListCustomDataIdentifiersResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CustomDataIdentifierSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomDataIdentifierSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFindingsFiltersRequest {
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
pub struct ListFindingsFiltersResponse {
    #[serde(rename = "findingsFilterListItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub findings_filter_list_items: Option<Vec<FindingsFilterListItem>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FindingsFilterListItem {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFindingsRequest {
    #[serde(rename = "findingCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_criteria: Option<FindingCriteria>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_criteria: Option<SortCriteria>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFindingsResponse {
    #[serde(rename = "findingIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_ids: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListInvitationsRequest {
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
pub struct ListInvitationsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitations: Option<Vec<Invitation>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListManagedDataIdentifiersRequest {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListManagedDataIdentifiersResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ManagedDataIdentifierSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedDataIdentifierSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMembersRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "onlyAssociated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_associated: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMembersResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<Member>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Member {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "administratorAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrator_account_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "invitedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invited_at: Option<String>,
    #[serde(rename = "masterAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_account_id: Option<String>,
    #[serde(rename = "relationshipStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOrganizationAdminAccountsRequest {
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
pub struct ListOrganizationAdminAccountsResponse {
    #[serde(rename = "adminAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_accounts: Option<Vec<AdminAccount>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdminAccount {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourceProfileArtifactsRequest {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourceProfileArtifactsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Vec<ResourceProfileArtifact>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceProfileArtifact {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "classificationResultStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classification_result_status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitive: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourceProfileDetectionsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourceProfileDetectionsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detections: Option<Vec<Detection>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Detection {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppressed: Option<bool>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSensitivityInspectionTemplatesRequest {
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
pub struct ListSensitivityInspectionTemplatesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sensitivityInspectionTemplates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitivity_inspection_templates: Option<Vec<SensitivityInspectionTemplatesEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SensitivityInspectionTemplatesEntry {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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
pub struct PutClassificationExportConfigurationRequest {
    #[serde(default)]
    pub configuration: ClassificationExportConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutClassificationExportConfigurationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ClassificationExportConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutFindingsPublicationConfigurationRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "securityHubConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_hub_configuration: Option<SecurityHubConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutFindingsPublicationConfigurationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchResourcesRequest {
    #[serde(rename = "bucketCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_criteria: Option<SearchResourcesBucketCriteria>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_criteria: Option<SearchResourcesSortCriteria>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchResourcesBucketCriteria {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excludes: Option<SearchResourcesCriteriaBlock>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<SearchResourcesCriteriaBlock>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchResourcesCriteriaBlock {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<SearchResourcesCriteria>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchResourcesCriteria {
    #[serde(rename = "simpleCriterion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_criterion: Option<SearchResourcesSimpleCriterion>,
    #[serde(rename = "tagCriterion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_criterion: Option<SearchResourcesTagCriterion>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchResourcesSimpleCriterion {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparator: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchResourcesTagCriterion {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparator: Option<String>,
    #[serde(rename = "tagValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<Vec<SearchResourcesTagCriterionPair>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchResourcesTagCriterionPair {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchResourcesSortCriteria {
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
pub struct SearchResourcesResponse {
    #[serde(rename = "matchingResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matching_resources: Option<Vec<MatchingResource>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MatchingResource {
    #[serde(rename = "matchingBucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matching_bucket: Option<MatchingBucket>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MatchingBucket {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "automatedDiscoveryMonitoringStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated_discovery_monitoring_status: Option<String>,
    #[serde(rename = "bucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    #[serde(rename = "classifiableObjectCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifiable_object_count: Option<i64>,
    #[serde(rename = "classifiableSizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifiable_size_in_bytes: Option<i64>,
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "jobDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_details: Option<JobDetails>,
    #[serde(rename = "lastAutomatedDiscoveryTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_automated_discovery_time: Option<String>,
    #[serde(rename = "objectCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_count: Option<i64>,
    #[serde(rename = "objectCountByEncryptionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_count_by_encryption_type: Option<ObjectCountByEncryptionType>,
    #[serde(rename = "sensitivityScore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitivity_score: Option<i32>,
    #[serde(rename = "sizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_bytes: Option<i64>,
    #[serde(rename = "sizeInBytesCompressed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_bytes_compressed: Option<i64>,
    #[serde(rename = "unclassifiableObjectCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unclassifiable_object_count: Option<ObjectLevelStatistics>,
    #[serde(rename = "unclassifiableObjectSizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unclassifiable_object_size_in_bytes: Option<ObjectLevelStatistics>,
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
pub struct TestCustomDataIdentifierRequest {
    #[serde(rename = "ignoreWords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_words: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    #[serde(rename = "maximumMatchDistance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_match_distance: Option<i32>,
    #[serde(default)]
    pub regex: String,
    #[serde(rename = "sampleText")]
    #[serde(default)]
    pub sample_text: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestCustomDataIdentifierResponse {
    #[serde(rename = "matchCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_count: Option<i32>,
}

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
pub struct UpdateAllowListRequest {
    #[serde(default)]
    pub criteria: AllowListCriteria,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAllowListResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAutomatedDiscoveryConfigurationRequest {
    #[serde(rename = "autoEnableOrganizationMembers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_enable_organization_members: Option<String>,
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAutomatedDiscoveryConfigurationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateClassificationJobRequest {
    #[serde(rename = "jobId")]
    #[serde(default)]
    pub job_id: String,
    #[serde(rename = "jobStatus")]
    #[serde(default)]
    pub job_status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateClassificationJobResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateClassificationScopeRequest {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3: Option<S3ClassificationScopeUpdate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3ClassificationScopeUpdate {
    #[serde(default)]
    pub excludes: S3ClassificationScopeExclusionUpdate,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3ClassificationScopeExclusionUpdate {
    #[serde(rename = "bucketNames")]
    #[serde(default)]
    pub bucket_names: Vec<String>,
    #[serde(default)]
    pub operation: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateClassificationScopeResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFindingsFilterRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "findingCriteria")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_criteria: Option<FindingCriteria>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFindingsFilterResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMacieSessionRequest {
    #[serde(rename = "findingPublishingFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finding_publishing_frequency: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMacieSessionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMemberSessionRequest {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMemberSessionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateOrganizationConfigurationRequest {
    #[serde(rename = "autoEnable")]
    #[serde(default)]
    pub auto_enable: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateOrganizationConfigurationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResourceProfileDetectionsRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "suppressDataIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppress_data_identifiers: Option<Vec<SuppressDataIdentifier>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SuppressDataIdentifier {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResourceProfileDetectionsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResourceProfileRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "sensitivityScoreOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sensitivity_score_override: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResourceProfileResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRevealConfigurationRequest {
    #[serde(default)]
    pub configuration: RevealConfiguration,
    #[serde(rename = "retrievalConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieval_configuration: Option<UpdateRetrievalConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRetrievalConfiguration {
    #[serde(rename = "retrievalMode")]
    #[serde(default)]
    pub retrieval_mode: String,
    #[serde(rename = "roleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRevealConfigurationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<RevealConfiguration>,
    #[serde(rename = "retrievalConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieval_configuration: Option<RetrievalConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSensitivityInspectionTemplateRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excludes: Option<SensitivityInspectionTemplateExcludes>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<SensitivityInspectionTemplateIncludes>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSensitivityInspectionTemplateResponse {}
