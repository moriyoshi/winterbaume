//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-cloudformation

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribePublisherResult")]
pub struct DescribePublisherOutput {
    #[serde(rename = "IdentityProvider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider: Option<String>,
    #[serde(rename = "PublisherId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_id: Option<String>,
    #[serde(rename = "PublisherProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_profile: Option<String>,
    #[serde(rename = "PublisherStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetStackPolicyInput")]
pub struct GetStackPolicyInput {
    #[serde(rename = "StackName")]
    #[serde(default)]
    pub stack_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateStackInstancesResult")]
pub struct UpdateStackInstancesOutput {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteStackInstancesResult")]
pub struct DeleteStackInstancesOutput {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListStackSetOperationsResult")]
pub struct ListStackSetOperationsOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Summaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summaries: Option<StackSetOperationSummaries>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StackSetOperationSummaries {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<StackSetOperationSummary>,
}
impl From<Vec<StackSetOperationSummary>> for StackSetOperationSummaries {
    fn from(v: Vec<StackSetOperationSummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<StackSetOperationSummary> for StackSetOperationSummaries {
    fn from_iter<I: IntoIterator<Item = StackSetOperationSummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<StackSetOperationSummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlStackSetOperationSummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<StackSetOperationSummary>,
}

impl From<Vec<StackSetOperationSummary>> for XmlStackSetOperationSummaryList {
    fn from(v: Vec<StackSetOperationSummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<StackSetOperationSummary> for XmlStackSetOperationSummaryList {
    fn from_iter<I: IntoIterator<Item = StackSetOperationSummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StackSetOperationSummary")]
pub struct StackSetOperationSummary {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "EndTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<String>,
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(rename = "OperationPreferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_preferences: Option<StackSetOperationPreferences>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<StackSetOperationStatusDetails>,
    #[serde(rename = "StatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StackSetOperationPreferences")]
pub struct StackSetOperationPreferences {
    #[serde(rename = "ConcurrencyMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency_mode: Option<String>,
    #[serde(rename = "FailureToleranceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_tolerance_count: Option<i32>,
    #[serde(rename = "FailureTolerancePercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_tolerance_percentage: Option<i32>,
    #[serde(rename = "MaxConcurrentCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent_count: Option<i32>,
    #[serde(rename = "MaxConcurrentPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrent_percentage: Option<i32>,
    #[serde(rename = "RegionConcurrencyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_concurrency_type: Option<String>,
    #[serde(rename = "RegionOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_order: Option<RegionList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegionList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for RegionList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for RegionList {
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
#[serde(rename = "StackSetOperationStatusDetails")]
pub struct StackSetOperationStatusDetails {
    #[serde(rename = "FailedStackInstancesCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_stack_instances_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateStackSetInput")]
pub struct UpdateStackSetInput {
    #[serde(rename = "Accounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<AccountList>,
    #[serde(rename = "AdministrationRoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administration_role_a_r_n: Option<String>,
    #[serde(rename = "AutoDeployment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_deployment: Option<AutoDeployment>,
    #[serde(rename = "CallAs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_as: Option<String>,
    #[serde(rename = "Capabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Capabilities>,
    #[serde(rename = "DeploymentTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_targets: Option<DeploymentTargets>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ExecutionRoleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_name: Option<String>,
    #[serde(rename = "ManagedExecution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_execution: Option<ManagedExecution>,
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(rename = "OperationPreferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_preferences: Option<StackSetOperationPreferences>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Parameters>,
    #[serde(rename = "PermissionModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_model: Option<String>,
    #[serde(rename = "Regions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<RegionList>,
    #[serde(rename = "StackSetName")]
    #[serde(default)]
    pub stack_set_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[serde(rename = "TemplateBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_body: Option<String>,
    #[serde(rename = "TemplateURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_u_r_l: Option<String>,
    #[serde(rename = "UsePreviousTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_previous_template: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for AccountList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for AccountList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AutoDeployment")]
pub struct AutoDeployment {
    #[serde(rename = "DependsOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<StackSetARNList>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "RetainStacksOnAccountRemoval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_stacks_on_account_removal: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StackSetARNList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for StackSetARNList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for StackSetARNList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Capabilities {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for Capabilities {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for Capabilities {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeploymentTargets")]
pub struct DeploymentTargets {
    #[serde(rename = "AccountFilterType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_filter_type: Option<String>,
    #[serde(rename = "Accounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<AccountList>,
    #[serde(rename = "AccountsUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts_url: Option<String>,
    #[serde(rename = "OrganizationalUnitIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit_ids: Option<OrganizationalUnitIdList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrganizationalUnitIdList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for OrganizationalUnitIdList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for OrganizationalUnitIdList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ManagedExecution")]
pub struct ManagedExecution {
    #[serde(rename = "Active")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Parameters {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Parameter>,
}
impl From<Vec<Parameter>> for Parameters {
    fn from(v: Vec<Parameter>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Parameter> for Parameters {
    fn from_iter<I: IntoIterator<Item = Parameter>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Parameter>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlParameterList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Parameter>,
}

impl From<Vec<Parameter>> for XmlParameterList {
    fn from(v: Vec<Parameter>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Parameter> for XmlParameterList {
    fn from_iter<I: IntoIterator<Item = Parameter>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Parameter")]
pub struct Parameter {
    #[serde(rename = "ParameterKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_key: Option<String>,
    #[serde(rename = "ParameterValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_value: Option<String>,
    #[serde(rename = "ResolvedValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_value: Option<String>,
    #[serde(rename = "UsePreviousValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_previous_value: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tags {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Tag>,
}
impl From<Vec<Tag>> for Tags {
    fn from(v: Vec<Tag>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Tag> for Tags {
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
#[serde(rename = "ListImportsResult")]
pub struct ListImportsOutput {
    #[serde(rename = "Imports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imports: Option<Imports>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Imports {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for Imports {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for Imports {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListStackInstancesInput")]
pub struct ListStackInstancesInput {
    #[serde(rename = "CallAs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_as: Option<String>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<StackInstanceFilters>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StackInstanceAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_instance_account: Option<String>,
    #[serde(rename = "StackInstanceRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_instance_region: Option<String>,
    #[serde(rename = "StackSetName")]
    #[serde(default)]
    pub stack_set_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StackInstanceFilters {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<StackInstanceFilter>,
}
impl From<Vec<StackInstanceFilter>> for StackInstanceFilters {
    fn from(v: Vec<StackInstanceFilter>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<StackInstanceFilter> for StackInstanceFilters {
    fn from_iter<I: IntoIterator<Item = StackInstanceFilter>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<StackInstanceFilter>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlStackInstanceFilterList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<StackInstanceFilter>,
}

impl From<Vec<StackInstanceFilter>> for XmlStackInstanceFilterList {
    fn from(v: Vec<StackInstanceFilter>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<StackInstanceFilter> for XmlStackInstanceFilterList {
    fn from_iter<I: IntoIterator<Item = StackInstanceFilter>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StackInstanceFilter")]
pub struct StackInstanceFilter {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DetectStackSetDriftResult")]
pub struct DetectStackSetDriftOutput {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ContinueUpdateRollbackInput")]
pub struct ContinueUpdateRollbackInput {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "ResourcesToSkip")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources_to_skip: Option<ResourcesToSkip>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
    #[serde(rename = "StackName")]
    #[serde(default)]
    pub stack_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourcesToSkip {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ResourcesToSkip {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ResourcesToSkip {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateTerminationProtectionInput")]
pub struct UpdateTerminationProtectionInput {
    #[serde(rename = "EnableTerminationProtection")]
    #[serde(default)]
    pub enable_termination_protection: bool,
    #[serde(rename = "StackName")]
    #[serde(default)]
    pub stack_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PublishTypeInput")]
pub struct PublishTypeInput {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "PublicVersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_version_number: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeGeneratedTemplateResult")]
pub struct DescribeGeneratedTemplateOutput {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "GeneratedTemplateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_template_id: Option<String>,
    #[serde(rename = "GeneratedTemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_template_name: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<String>,
    #[serde(rename = "Progress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<TemplateProgress>,
    #[serde(rename = "Resources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<ResourceDetails>,
    #[serde(rename = "StackId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "TemplateConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_configuration: Option<TemplateConfiguration>,
    #[serde(rename = "TotalWarnings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_warnings: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TemplateProgress")]
pub struct TemplateProgress {
    #[serde(rename = "ResourcesFailed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources_failed: Option<i32>,
    #[serde(rename = "ResourcesPending")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources_pending: Option<i32>,
    #[serde(rename = "ResourcesProcessing")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources_processing: Option<i32>,
    #[serde(rename = "ResourcesSucceeded")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources_succeeded: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceDetails {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ResourceDetail>,
}
impl From<Vec<ResourceDetail>> for ResourceDetails {
    fn from(v: Vec<ResourceDetail>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ResourceDetail> for ResourceDetails {
    fn from_iter<I: IntoIterator<Item = ResourceDetail>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ResourceDetail>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlResourceDetailList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ResourceDetail>,
}

impl From<Vec<ResourceDetail>> for XmlResourceDetailList {
    fn from(v: Vec<ResourceDetail>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ResourceDetail> for XmlResourceDetailList {
    fn from_iter<I: IntoIterator<Item = ResourceDetail>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResourceDetail")]
pub struct ResourceDetail {
    #[serde(rename = "LogicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_resource_id: Option<String>,
    #[serde(rename = "ResourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ResourceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_status: Option<String>,
    #[serde(rename = "ResourceStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_status_reason: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "Warnings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<WarningDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WarningDetails {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<WarningDetail>,
}
impl From<Vec<WarningDetail>> for WarningDetails {
    fn from(v: Vec<WarningDetail>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<WarningDetail> for WarningDetails {
    fn from_iter<I: IntoIterator<Item = WarningDetail>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<WarningDetail>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlWarningDetailList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<WarningDetail>,
}

impl From<Vec<WarningDetail>> for XmlWarningDetailList {
    fn from(v: Vec<WarningDetail>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<WarningDetail> for XmlWarningDetailList {
    fn from_iter<I: IntoIterator<Item = WarningDetail>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "WarningDetail")]
pub struct WarningDetail {
    #[serde(rename = "Properties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<WarningProperties>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WarningProperties {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<WarningProperty>,
}
impl From<Vec<WarningProperty>> for WarningProperties {
    fn from(v: Vec<WarningProperty>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<WarningProperty> for WarningProperties {
    fn from_iter<I: IntoIterator<Item = WarningProperty>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<WarningProperty>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlWarningPropertyList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<WarningProperty>,
}

impl From<Vec<WarningProperty>> for XmlWarningPropertyList {
    fn from(v: Vec<WarningProperty>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<WarningProperty> for XmlWarningPropertyList {
    fn from_iter<I: IntoIterator<Item = WarningProperty>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "WarningProperty")]
pub struct WarningProperty {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "PropertyPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_path: Option<String>,
    #[serde(rename = "Required")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TemplateConfiguration")]
pub struct TemplateConfiguration {
    #[serde(rename = "DeletionPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_policy: Option<String>,
    #[serde(rename = "UpdateReplacePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_replace_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetHookResultInput")]
pub struct GetHookResultInput {
    #[serde(rename = "HookResultId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_result_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RegisterPublisherInput")]
pub struct RegisterPublisherInput {
    #[serde(rename = "AcceptTermsAndConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_terms_and_conditions: Option<bool>,
    #[serde(rename = "ConnectionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteChangeSetInput")]
pub struct DeleteChangeSetInput {
    #[serde(rename = "ChangeSetName")]
    #[serde(default)]
    pub change_set_name: String,
    #[serde(rename = "StackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteChangeSetOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeTypeRegistrationResult")]
pub struct DescribeTypeRegistrationOutput {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ProgressStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_status: Option<String>,
    #[serde(rename = "TypeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_arn: Option<String>,
    #[serde(rename = "TypeVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_version_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetGeneratedTemplateInput")]
pub struct GetGeneratedTemplateInput {
    #[serde(rename = "Format")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "GeneratedTemplateName")]
    #[serde(default)]
    pub generated_template_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListImportsInput")]
pub struct ListImportsInput {
    #[serde(rename = "ExportName")]
    #[serde(default)]
    pub export_name: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListResourceScanResourcesResult")]
pub struct ListResourceScanResourcesOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Resources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<ScannedResources>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScannedResources {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ScannedResource>,
}
impl From<Vec<ScannedResource>> for ScannedResources {
    fn from(v: Vec<ScannedResource>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ScannedResource> for ScannedResources {
    fn from_iter<I: IntoIterator<Item = ScannedResource>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ScannedResource>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlScannedResourceList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ScannedResource>,
}

impl From<Vec<ScannedResource>> for XmlScannedResourceList {
    fn from(v: Vec<ScannedResource>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ScannedResource> for XmlScannedResourceList {
    fn from_iter<I: IntoIterator<Item = ScannedResource>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ScannedResource")]
pub struct ScannedResource {
    #[serde(rename = "ManagedByStack")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_by_stack: Option<bool>,
    #[serde(rename = "ResourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteStackSetInput")]
pub struct DeleteStackSetInput {
    #[serde(rename = "CallAs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_as: Option<String>,
    #[serde(rename = "StackSetName")]
    #[serde(default)]
    pub stack_set_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeChangeSetHooksResult")]
pub struct DescribeChangeSetHooksOutput {
    #[serde(rename = "ChangeSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_set_id: Option<String>,
    #[serde(rename = "ChangeSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_set_name: Option<String>,
    #[serde(rename = "Hooks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hooks: Option<ChangeSetHooks>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StackId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
    #[serde(rename = "StackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChangeSetHooks {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ChangeSetHook>,
}
impl From<Vec<ChangeSetHook>> for ChangeSetHooks {
    fn from(v: Vec<ChangeSetHook>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ChangeSetHook> for ChangeSetHooks {
    fn from_iter<I: IntoIterator<Item = ChangeSetHook>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ChangeSetHook>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlChangeSetHookList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ChangeSetHook>,
}

impl From<Vec<ChangeSetHook>> for XmlChangeSetHookList {
    fn from(v: Vec<ChangeSetHook>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ChangeSetHook> for XmlChangeSetHookList {
    fn from_iter<I: IntoIterator<Item = ChangeSetHook>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ChangeSetHook")]
pub struct ChangeSetHook {
    #[serde(rename = "FailureMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_mode: Option<String>,
    #[serde(rename = "InvocationPoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_point: Option<String>,
    #[serde(rename = "TargetDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_details: Option<ChangeSetHookTargetDetails>,
    #[serde(rename = "TypeConfigurationVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_configuration_version_id: Option<String>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[serde(rename = "TypeVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ChangeSetHookTargetDetails")]
pub struct ChangeSetHookTargetDetails {
    #[serde(rename = "ResourceTargetDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_target_details: Option<ChangeSetHookResourceTargetDetails>,
    #[serde(rename = "TargetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ChangeSetHookResourceTargetDetails")]
pub struct ChangeSetHookResourceTargetDetails {
    #[serde(rename = "LogicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_resource_id: Option<String>,
    #[serde(rename = "ResourceAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_action: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListStackSetsResult")]
pub struct ListStackSetsOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Summaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summaries: Option<StackSetSummaries>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StackSetSummaries {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<StackSetSummary>,
}
impl From<Vec<StackSetSummary>> for StackSetSummaries {
    fn from(v: Vec<StackSetSummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<StackSetSummary> for StackSetSummaries {
    fn from_iter<I: IntoIterator<Item = StackSetSummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<StackSetSummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlStackSetSummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<StackSetSummary>,
}

impl From<Vec<StackSetSummary>> for XmlStackSetSummaryList {
    fn from(v: Vec<StackSetSummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<StackSetSummary> for XmlStackSetSummaryList {
    fn from_iter<I: IntoIterator<Item = StackSetSummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StackSetSummary")]
pub struct StackSetSummary {
    #[serde(rename = "AutoDeployment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_deployment: Option<AutoDeployment>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DriftStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drift_status: Option<String>,
    #[serde(rename = "LastDriftCheckTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_drift_check_timestamp: Option<String>,
    #[serde(rename = "ManagedExecution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_execution: Option<ManagedExecution>,
    #[serde(rename = "PermissionModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_model: Option<String>,
    #[serde(rename = "StackSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_id: Option<String>,
    #[serde(rename = "StackSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RegisterTypeResult")]
pub struct RegisterTypeOutput {
    #[serde(rename = "RegistrationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateGeneratedTemplateResult")]
pub struct UpdateGeneratedTemplateOutput {
    #[serde(rename = "GeneratedTemplateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_template_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EstimateTemplateCostInput")]
pub struct EstimateTemplateCostInput {
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Parameters>,
    #[serde(rename = "TemplateBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_body: Option<String>,
    #[serde(rename = "TemplateURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_u_r_l: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ActivateTypeInput")]
pub struct ActivateTypeInput {
    #[serde(rename = "AutoUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_update: Option<bool>,
    #[serde(rename = "ExecutionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(rename = "LoggingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_config: Option<LoggingConfig>,
    #[serde(rename = "MajorVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major_version: Option<i64>,
    #[serde(rename = "PublicTypeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_type_arn: Option<String>,
    #[serde(rename = "PublisherId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[serde(rename = "TypeNameAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name_alias: Option<String>,
    #[serde(rename = "VersionBump")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_bump: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LoggingConfig")]
pub struct LoggingConfig {
    #[serde(rename = "LogGroupName")]
    #[serde(default)]
    pub log_group_name: String,
    #[serde(rename = "LogRoleArn")]
    #[serde(default)]
    pub log_role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CancelUpdateStackInput")]
pub struct CancelUpdateStackInput {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "StackName")]
    #[serde(default)]
    pub stack_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RegisterPublisherResult")]
pub struct RegisterPublisherOutput {
    #[serde(rename = "PublisherId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetTemplateSummaryInput")]
pub struct GetTemplateSummaryInput {
    #[serde(rename = "CallAs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_as: Option<String>,
    #[serde(rename = "StackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_name: Option<String>,
    #[serde(rename = "StackSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_name: Option<String>,
    #[serde(rename = "TemplateBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_body: Option<String>,
    #[serde(rename = "TemplateSummaryConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_summary_config: Option<TemplateSummaryConfig>,
    #[serde(rename = "TemplateURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_u_r_l: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TemplateSummaryConfig")]
pub struct TemplateSummaryConfig {
    #[serde(rename = "TreatUnrecognizedResourceTypesAsWarnings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treat_unrecognized_resource_types_as_warnings: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListGeneratedTemplatesResult")]
pub struct ListGeneratedTemplatesOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Summaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summaries: Option<TemplateSummaries>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TemplateSummaries {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<TemplateSummary>,
}
impl From<Vec<TemplateSummary>> for TemplateSummaries {
    fn from(v: Vec<TemplateSummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<TemplateSummary> for TemplateSummaries {
    fn from_iter<I: IntoIterator<Item = TemplateSummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<TemplateSummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTemplateSummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<TemplateSummary>,
}

impl From<Vec<TemplateSummary>> for XmlTemplateSummaryList {
    fn from(v: Vec<TemplateSummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<TemplateSummary> for XmlTemplateSummaryList {
    fn from_iter<I: IntoIterator<Item = TemplateSummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TemplateSummary")]
pub struct TemplateSummary {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "GeneratedTemplateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_template_id: Option<String>,
    #[serde(rename = "GeneratedTemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_template_name: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<String>,
    #[serde(rename = "NumberOfResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_resources: Option<i32>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListStackSetAutoDeploymentTargetsInput")]
pub struct ListStackSetAutoDeploymentTargetsInput {
    #[serde(rename = "CallAs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_as: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StackSetName")]
    #[serde(default)]
    pub stack_set_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetStackPolicyResult")]
pub struct GetStackPolicyOutput {
    #[serde(rename = "StackPolicyBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_policy_body: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListResourceScansInput")]
pub struct ListResourceScansInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ScanTypeFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type_filter: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RollbackStackResult")]
pub struct RollbackStackOutput {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(rename = "StackId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SetTypeConfigurationResult")]
pub struct SetTypeConfigurationOutput {
    #[serde(rename = "ConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopStackSetOperationOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DetectStackSetDriftInput")]
pub struct DetectStackSetDriftInput {
    #[serde(rename = "CallAs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_as: Option<String>,
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(rename = "OperationPreferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_preferences: Option<StackSetOperationPreferences>,
    #[serde(rename = "StackSetName")]
    #[serde(default)]
    pub stack_set_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateStackResult")]
pub struct CreateStackOutput {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(rename = "StackId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeactivateOrganizationsAccessInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateStackRefactorResult")]
pub struct CreateStackRefactorOutput {
    #[serde(rename = "StackRefactorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_refactor_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DetectStackResourceDriftResult")]
pub struct DetectStackResourceDriftOutput {
    #[serde(rename = "StackResourceDrift")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_resource_drift: Option<StackResourceDrift>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StackResourceDrift")]
pub struct StackResourceDrift {
    #[serde(rename = "ActualProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_properties: Option<String>,
    #[serde(rename = "DriftStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drift_status_reason: Option<String>,
    #[serde(rename = "ExpectedProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_properties: Option<String>,
    #[serde(rename = "LogicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_resource_id: Option<String>,
    #[serde(rename = "ModuleInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_info: Option<ModuleInfo>,
    #[serde(rename = "PhysicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_resource_id: Option<String>,
    #[serde(rename = "PhysicalResourceIdContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_resource_id_context: Option<PhysicalResourceIdContext>,
    #[serde(rename = "PropertyDifferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_differences: Option<PropertyDifferences>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "StackId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
    #[serde(rename = "StackResourceDriftStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_resource_drift_status: Option<String>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModuleInfo")]
pub struct ModuleInfo {
    #[serde(rename = "LogicalIdHierarchy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_id_hierarchy: Option<String>,
    #[serde(rename = "TypeHierarchy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_hierarchy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PhysicalResourceIdContext {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PhysicalResourceIdContextKeyValuePair>,
}
impl From<Vec<PhysicalResourceIdContextKeyValuePair>> for PhysicalResourceIdContext {
    fn from(v: Vec<PhysicalResourceIdContextKeyValuePair>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<PhysicalResourceIdContextKeyValuePair> for PhysicalResourceIdContext {
    fn from_iter<I: IntoIterator<Item = PhysicalResourceIdContextKeyValuePair>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<PhysicalResourceIdContextKeyValuePair>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlPhysicalResourceIdContextKeyValuePairList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<PhysicalResourceIdContextKeyValuePair>,
}

impl From<Vec<PhysicalResourceIdContextKeyValuePair>>
    for XmlPhysicalResourceIdContextKeyValuePairList
{
    fn from(v: Vec<PhysicalResourceIdContextKeyValuePair>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<PhysicalResourceIdContextKeyValuePair>
    for XmlPhysicalResourceIdContextKeyValuePairList
{
    fn from_iter<I: IntoIterator<Item = PhysicalResourceIdContextKeyValuePair>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PhysicalResourceIdContextKeyValuePair")]
pub struct PhysicalResourceIdContextKeyValuePair {
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
pub struct PropertyDifferences {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PropertyDifference>,
}
impl From<Vec<PropertyDifference>> for PropertyDifferences {
    fn from(v: Vec<PropertyDifference>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<PropertyDifference> for PropertyDifferences {
    fn from_iter<I: IntoIterator<Item = PropertyDifference>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<PropertyDifference>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlPropertyDifferenceList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<PropertyDifference>,
}

impl From<Vec<PropertyDifference>> for XmlPropertyDifferenceList {
    fn from(v: Vec<PropertyDifference>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<PropertyDifference> for XmlPropertyDifferenceList {
    fn from_iter<I: IntoIterator<Item = PropertyDifference>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PropertyDifference")]
pub struct PropertyDifference {
    #[serde(rename = "ActualValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_value: Option<String>,
    #[serde(rename = "DifferenceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub difference_type: Option<String>,
    #[serde(rename = "ExpectedValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_value: Option<String>,
    #[serde(rename = "PropertyPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListStackSetOperationResultsInput")]
pub struct ListStackSetOperationResultsInput {
    #[serde(rename = "CallAs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_as: Option<String>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<OperationResultFilters>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OperationId")]
    #[serde(default)]
    pub operation_id: String,
    #[serde(rename = "StackSetName")]
    #[serde(default)]
    pub stack_set_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OperationResultFilters {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<OperationResultFilter>,
}
impl From<Vec<OperationResultFilter>> for OperationResultFilters {
    fn from(v: Vec<OperationResultFilter>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<OperationResultFilter> for OperationResultFilters {
    fn from_iter<I: IntoIterator<Item = OperationResultFilter>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<OperationResultFilter>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlOperationResultFilterList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<OperationResultFilter>,
}

impl From<Vec<OperationResultFilter>> for XmlOperationResultFilterList {
    fn from(v: Vec<OperationResultFilter>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<OperationResultFilter> for XmlOperationResultFilterList {
    fn from_iter<I: IntoIterator<Item = OperationResultFilter>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OperationResultFilter")]
pub struct OperationResultFilter {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateStackInstancesResult")]
pub struct CreateStackInstancesOutput {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListStacksResult")]
pub struct ListStacksOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StackSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_summaries: Option<StackSummaries>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StackSummaries {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<StackSummary>,
}
impl From<Vec<StackSummary>> for StackSummaries {
    fn from(v: Vec<StackSummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<StackSummary> for StackSummaries {
    fn from_iter<I: IntoIterator<Item = StackSummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<StackSummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlStackSummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<StackSummary>,
}

impl From<Vec<StackSummary>> for XmlStackSummaryList {
    fn from(v: Vec<StackSummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<StackSummary> for XmlStackSummaryList {
    fn from_iter<I: IntoIterator<Item = StackSummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StackSummary")]
pub struct StackSummary {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "DeletionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_time: Option<String>,
    #[serde(rename = "DriftInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drift_information: Option<StackDriftInformationSummary>,
    #[serde(rename = "LastOperations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_operations: Option<LastOperations>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<String>,
    #[serde(rename = "ParentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "RootId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_id: Option<String>,
    #[serde(rename = "StackId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
    #[serde(rename = "StackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_name: Option<String>,
    #[serde(rename = "StackStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_status: Option<String>,
    #[serde(rename = "StackStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_status_reason: Option<String>,
    #[serde(rename = "TemplateDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StackDriftInformationSummary")]
pub struct StackDriftInformationSummary {
    #[serde(rename = "LastCheckTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_check_timestamp: Option<String>,
    #[serde(rename = "StackDriftStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_drift_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LastOperations {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<OperationEntry>,
}
impl From<Vec<OperationEntry>> for LastOperations {
    fn from(v: Vec<OperationEntry>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<OperationEntry> for LastOperations {
    fn from_iter<I: IntoIterator<Item = OperationEntry>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<OperationEntry>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlOperationEntryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<OperationEntry>,
}

impl From<Vec<OperationEntry>> for XmlOperationEntryList {
    fn from(v: Vec<OperationEntry>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<OperationEntry> for XmlOperationEntryList {
    fn from_iter<I: IntoIterator<Item = OperationEntry>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OperationEntry")]
pub struct OperationEntry {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(rename = "OperationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTypeVersionsInput")]
pub struct ListTypeVersionsInput {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "DeprecatedStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated_status: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PublisherId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeactivateTypeInput")]
pub struct DeactivateTypeInput {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SignalResourceInput")]
pub struct SignalResourceInput {
    #[serde(rename = "LogicalResourceId")]
    #[serde(default)]
    pub logical_resource_id: String,
    #[serde(rename = "StackName")]
    #[serde(default)]
    pub stack_name: String,
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
    #[serde(rename = "UniqueId")]
    #[serde(default)]
    pub unique_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListResourceScanResourcesInput")]
pub struct ListResourceScanResourcesInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<String>,
    #[serde(rename = "ResourceScanId")]
    #[serde(default)]
    pub resource_scan_id: String,
    #[serde(rename = "ResourceTypePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type_prefix: Option<String>,
    #[serde(rename = "TagKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    #[serde(rename = "TagValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeStackSetOperationInput")]
pub struct DescribeStackSetOperationInput {
    #[serde(rename = "CallAs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_as: Option<String>,
    #[serde(rename = "OperationId")]
    #[serde(default)]
    pub operation_id: String,
    #[serde(rename = "StackSetName")]
    #[serde(default)]
    pub stack_set_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeOrganizationsAccessInput")]
pub struct DescribeOrganizationsAccessInput {
    #[serde(rename = "CallAs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_as: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListStackSetOperationResultsResult")]
pub struct ListStackSetOperationResultsOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Summaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summaries: Option<StackSetOperationResultSummaries>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StackSetOperationResultSummaries {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<StackSetOperationResultSummary>,
}
impl From<Vec<StackSetOperationResultSummary>> for StackSetOperationResultSummaries {
    fn from(v: Vec<StackSetOperationResultSummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<StackSetOperationResultSummary> for StackSetOperationResultSummaries {
    fn from_iter<I: IntoIterator<Item = StackSetOperationResultSummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<StackSetOperationResultSummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlStackSetOperationResultSummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<StackSetOperationResultSummary>,
}

impl From<Vec<StackSetOperationResultSummary>> for XmlStackSetOperationResultSummaryList {
    fn from(v: Vec<StackSetOperationResultSummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<StackSetOperationResultSummary> for XmlStackSetOperationResultSummaryList {
    fn from_iter<I: IntoIterator<Item = StackSetOperationResultSummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StackSetOperationResultSummary")]
pub struct StackSetOperationResultSummary {
    #[serde(rename = "Account")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(rename = "AccountGateResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_gate_result: Option<AccountGateResult>,
    #[serde(rename = "OrganizationalUnitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit_id: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AccountGateResult")]
pub struct AccountGateResult {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteStackInput")]
pub struct DeleteStackInput {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "DeletionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_mode: Option<String>,
    #[serde(rename = "RetainResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_resources: Option<RetainResources>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
    #[serde(rename = "StackName")]
    #[serde(default)]
    pub stack_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetainResources {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for RetainResources {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for RetainResources {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RegisterTypeInput")]
pub struct RegisterTypeInput {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "ExecutionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(rename = "LoggingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_config: Option<LoggingConfig>,
    #[serde(rename = "SchemaHandlerPackage")]
    #[serde(default)]
    pub schema_handler_package: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    pub type_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeAccountLimitsResult")]
pub struct DescribeAccountLimitsOutput {
    #[serde(rename = "AccountLimits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_limits: Option<AccountLimitList>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountLimitList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<AccountLimit>,
}
impl From<Vec<AccountLimit>> for AccountLimitList {
    fn from(v: Vec<AccountLimit>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<AccountLimit> for AccountLimitList {
    fn from_iter<I: IntoIterator<Item = AccountLimit>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<AccountLimit>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlAccountLimitList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<AccountLimit>,
}

impl From<Vec<AccountLimit>> for XmlAccountLimitList {
    fn from(v: Vec<AccountLimit>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<AccountLimit> for XmlAccountLimitList {
    fn from_iter<I: IntoIterator<Item = AccountLimit>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AccountLimit")]
pub struct AccountLimit {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecuteChangeSetOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContinueUpdateRollbackOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeTypeRegistrationInput")]
pub struct DescribeTypeRegistrationInput {
    #[serde(rename = "RegistrationToken")]
    #[serde(default)]
    pub registration_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListResourceScansResult")]
pub struct ListResourceScansOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceScanSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_scan_summaries: Option<ResourceScanSummaries>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceScanSummaries {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ResourceScanSummary>,
}
impl From<Vec<ResourceScanSummary>> for ResourceScanSummaries {
    fn from(v: Vec<ResourceScanSummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ResourceScanSummary> for ResourceScanSummaries {
    fn from_iter<I: IntoIterator<Item = ResourceScanSummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ResourceScanSummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlResourceScanSummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ResourceScanSummary>,
}

impl From<Vec<ResourceScanSummary>> for XmlResourceScanSummaryList {
    fn from(v: Vec<ResourceScanSummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ResourceScanSummary> for XmlResourceScanSummaryList {
    fn from_iter<I: IntoIterator<Item = ResourceScanSummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResourceScanSummary")]
pub struct ResourceScanSummary {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "PercentageCompleted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage_completed: Option<f64>,
    #[serde(rename = "ResourceScanId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_scan_id: Option<String>,
    #[serde(rename = "ScanType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateChangeSetInput")]
pub struct CreateChangeSetInput {
    #[serde(rename = "Capabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Capabilities>,
    #[serde(rename = "ChangeSetName")]
    #[serde(default)]
    pub change_set_name: String,
    #[serde(rename = "ChangeSetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_set_type: Option<String>,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "DeploymentMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_mode: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ImportExistingResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_existing_resources: Option<bool>,
    #[serde(rename = "IncludeNestedStacks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_nested_stacks: Option<bool>,
    #[serde(rename = "NotificationARNs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_a_r_ns: Option<NotificationARNs>,
    #[serde(rename = "OnStackFailure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_stack_failure: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Parameters>,
    #[serde(rename = "ResourceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<ResourceTypes>,
    #[serde(rename = "ResourcesToImport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources_to_import: Option<ResourcesToImport>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
    #[serde(rename = "RollbackConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_configuration: Option<RollbackConfiguration>,
    #[serde(rename = "StackName")]
    #[serde(default)]
    pub stack_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[serde(rename = "TemplateBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_body: Option<String>,
    #[serde(rename = "TemplateURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_u_r_l: Option<String>,
    #[serde(rename = "UsePreviousTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_previous_template: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NotificationARNs {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for NotificationARNs {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for NotificationARNs {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceTypes {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ResourceTypes {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ResourceTypes {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourcesToImport {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ResourceToImport>,
}
impl From<Vec<ResourceToImport>> for ResourcesToImport {
    fn from(v: Vec<ResourceToImport>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ResourceToImport> for ResourcesToImport {
    fn from_iter<I: IntoIterator<Item = ResourceToImport>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ResourceToImport>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlResourceToImportList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ResourceToImport>,
}

impl From<Vec<ResourceToImport>> for XmlResourceToImportList {
    fn from(v: Vec<ResourceToImport>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ResourceToImport> for XmlResourceToImportList {
    fn from_iter<I: IntoIterator<Item = ResourceToImport>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResourceToImport")]
pub struct ResourceToImport {
    #[serde(rename = "LogicalResourceId")]
    #[serde(default)]
    pub logical_resource_id: String,
    #[serde(rename = "ResourceIdentifier")]
    #[serde(default)]
    pub resource_identifier: std::collections::HashMap<String, String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    pub resource_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RollbackConfiguration")]
pub struct RollbackConfiguration {
    #[serde(rename = "MonitoringTimeInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_time_in_minutes: Option<i32>,
    #[serde(rename = "RollbackTriggers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_triggers: Option<RollbackTriggers>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RollbackTriggers {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<RollbackTrigger>,
}
impl From<Vec<RollbackTrigger>> for RollbackTriggers {
    fn from(v: Vec<RollbackTrigger>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<RollbackTrigger> for RollbackTriggers {
    fn from_iter<I: IntoIterator<Item = RollbackTrigger>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<RollbackTrigger>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlRollbackTriggerList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<RollbackTrigger>,
}

impl From<Vec<RollbackTrigger>> for XmlRollbackTriggerList {
    fn from(v: Vec<RollbackTrigger>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<RollbackTrigger> for XmlRollbackTriggerList {
    fn from_iter<I: IntoIterator<Item = RollbackTrigger>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RollbackTrigger")]
pub struct RollbackTrigger {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateStackInstancesInput")]
pub struct CreateStackInstancesInput {
    #[serde(rename = "Accounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<AccountList>,
    #[serde(rename = "CallAs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_as: Option<String>,
    #[serde(rename = "DeploymentTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_targets: Option<DeploymentTargets>,
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(rename = "OperationPreferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_preferences: Option<StackSetOperationPreferences>,
    #[serde(rename = "ParameterOverrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_overrides: Option<Parameters>,
    #[serde(rename = "Regions")]
    #[serde(default)]
    pub regions: RegionList,
    #[serde(rename = "StackSetName")]
    #[serde(default)]
    pub stack_set_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListResourceScanRelatedResourcesResult")]
pub struct ListResourceScanRelatedResourcesOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RelatedResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_resources: Option<RelatedResources>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RelatedResources {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ScannedResource>,
}
impl From<Vec<ScannedResource>> for RelatedResources {
    fn from(v: Vec<ScannedResource>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ScannedResource> for RelatedResources {
    fn from_iter<I: IntoIterator<Item = ScannedResource>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeStacksResult")]
pub struct DescribeStacksOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Stacks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stacks: Option<Stacks>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Stacks {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Stack>,
}
impl From<Vec<Stack>> for Stacks {
    fn from(v: Vec<Stack>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Stack> for Stacks {
    fn from_iter<I: IntoIterator<Item = Stack>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Stack>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlStackList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Stack>,
}

impl From<Vec<Stack>> for XmlStackList {
    fn from(v: Vec<Stack>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Stack> for XmlStackList {
    fn from_iter<I: IntoIterator<Item = Stack>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Stack")]
pub struct Stack {
    #[serde(rename = "Capabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Capabilities>,
    #[serde(rename = "ChangeSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_set_id: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "DeletionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_mode: Option<String>,
    #[serde(rename = "DeletionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_time: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DetailedStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_status: Option<String>,
    #[serde(rename = "DisableRollback")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_rollback: Option<bool>,
    #[serde(rename = "DriftInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drift_information: Option<StackDriftInformation>,
    #[serde(rename = "EnableTerminationProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_termination_protection: Option<bool>,
    #[serde(rename = "LastOperations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_operations: Option<LastOperations>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<String>,
    #[serde(rename = "NotificationARNs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_a_r_ns: Option<NotificationARNs>,
    #[serde(rename = "Outputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Outputs>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Parameters>,
    #[serde(rename = "ParentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(rename = "RetainExceptOnCreate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_except_on_create: Option<bool>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
    #[serde(rename = "RollbackConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_configuration: Option<RollbackConfiguration>,
    #[serde(rename = "RootId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_id: Option<String>,
    #[serde(rename = "StackId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
    #[serde(rename = "StackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_name: Option<String>,
    #[serde(rename = "StackStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_status: Option<String>,
    #[serde(rename = "StackStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_status_reason: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[serde(rename = "TimeoutInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_minutes: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StackDriftInformation")]
pub struct StackDriftInformation {
    #[serde(rename = "LastCheckTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_check_timestamp: Option<String>,
    #[serde(rename = "StackDriftStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_drift_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Outputs {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Output>,
}
impl From<Vec<Output>> for Outputs {
    fn from(v: Vec<Output>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Output> for Outputs {
    fn from_iter<I: IntoIterator<Item = Output>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Output>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlOutputList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Output>,
}

impl From<Vec<Output>> for XmlOutputList {
    fn from(v: Vec<Output>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Output> for XmlOutputList {
    fn from_iter<I: IntoIterator<Item = Output>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Output")]
pub struct Output {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ExportName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_name: Option<String>,
    #[serde(rename = "OutputKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_key: Option<String>,
    #[serde(rename = "OutputValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TestTypeResult")]
pub struct TestTypeOutput {
    #[serde(rename = "TypeVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_version_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "BatchDescribeTypeConfigurationsInput")]
pub struct BatchDescribeTypeConfigurationsInput {
    #[serde(rename = "TypeConfigurationIdentifiers")]
    #[serde(default)]
    pub type_configuration_identifiers: TypeConfigurationIdentifiers,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TypeConfigurationIdentifiers {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<TypeConfigurationIdentifier>,
}
impl From<Vec<TypeConfigurationIdentifier>> for TypeConfigurationIdentifiers {
    fn from(v: Vec<TypeConfigurationIdentifier>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<TypeConfigurationIdentifier> for TypeConfigurationIdentifiers {
    fn from_iter<I: IntoIterator<Item = TypeConfigurationIdentifier>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<TypeConfigurationIdentifier>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTypeConfigurationIdentifierList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<TypeConfigurationIdentifier>,
}

impl From<Vec<TypeConfigurationIdentifier>> for XmlTypeConfigurationIdentifierList {
    fn from(v: Vec<TypeConfigurationIdentifier>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<TypeConfigurationIdentifier> for XmlTypeConfigurationIdentifierList {
    fn from_iter<I: IntoIterator<Item = TypeConfigurationIdentifier>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TypeConfigurationIdentifier")]
pub struct TypeConfigurationIdentifier {
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "TypeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_arn: Option<String>,
    #[serde(rename = "TypeConfigurationAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_configuration_alias: Option<String>,
    #[serde(rename = "TypeConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_configuration_arn: Option<String>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeactivateTypeOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RecordHandlerProgressInput")]
pub struct RecordHandlerProgressInput {
    #[serde(rename = "BearerToken")]
    #[serde(default)]
    pub bearer_token: String,
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "CurrentOperationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_operation_status: Option<String>,
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "OperationStatus")]
    #[serde(default)]
    pub operation_status: String,
    #[serde(rename = "ResourceModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_model: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListStacksInput")]
pub struct ListStacksInput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StackStatusFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_status_filter: Option<StackStatusFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StackStatusFilter {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for StackStatusFilter {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for StackStatusFilter {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListGeneratedTemplatesInput")]
pub struct ListGeneratedTemplatesInput {
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
#[serde(rename = "ListResourceScanRelatedResourcesInput")]
pub struct ListResourceScanRelatedResourcesInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceScanId")]
    #[serde(default)]
    pub resource_scan_id: String,
    #[serde(rename = "Resources")]
    #[serde(default)]
    pub resources: ScannedResourceIdentifiers,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScannedResourceIdentifiers {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ScannedResourceIdentifier>,
}
impl From<Vec<ScannedResourceIdentifier>> for ScannedResourceIdentifiers {
    fn from(v: Vec<ScannedResourceIdentifier>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ScannedResourceIdentifier> for ScannedResourceIdentifiers {
    fn from_iter<I: IntoIterator<Item = ScannedResourceIdentifier>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ScannedResourceIdentifier>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlScannedResourceIdentifierList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ScannedResourceIdentifier>,
}

impl From<Vec<ScannedResourceIdentifier>> for XmlScannedResourceIdentifierList {
    fn from(v: Vec<ScannedResourceIdentifier>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ScannedResourceIdentifier> for XmlScannedResourceIdentifierList {
    fn from_iter<I: IntoIterator<Item = ScannedResourceIdentifier>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ScannedResourceIdentifier")]
pub struct ScannedResourceIdentifier {
    #[serde(rename = "ResourceIdentifier")]
    #[serde(default)]
    pub resource_identifier: std::collections::HashMap<String, String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    pub resource_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEventsResult")]
pub struct DescribeEventsOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OperationEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_events: Option<OperationEvents>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OperationEvents {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<OperationEvent>,
}
impl From<Vec<OperationEvent>> for OperationEvents {
    fn from(v: Vec<OperationEvent>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<OperationEvent> for OperationEvents {
    fn from_iter<I: IntoIterator<Item = OperationEvent>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<OperationEvent>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlOperationEventList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<OperationEvent>,
}

impl From<Vec<OperationEvent>> for XmlOperationEventList {
    fn from(v: Vec<OperationEvent>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<OperationEvent> for XmlOperationEventList {
    fn from_iter<I: IntoIterator<Item = OperationEvent>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OperationEvent")]
pub struct OperationEvent {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "DetailedStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_status: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "EventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(rename = "EventType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<String>,
    #[serde(rename = "HookFailureMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_failure_mode: Option<String>,
    #[serde(rename = "HookInvocationPoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_invocation_point: Option<String>,
    #[serde(rename = "HookStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_status: Option<String>,
    #[serde(rename = "HookStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_status_reason: Option<String>,
    #[serde(rename = "HookType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_type: Option<String>,
    #[serde(rename = "LogicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_resource_id: Option<String>,
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(rename = "OperationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_status: Option<String>,
    #[serde(rename = "OperationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<String>,
    #[serde(rename = "PhysicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_resource_id: Option<String>,
    #[serde(rename = "ResourceProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_properties: Option<String>,
    #[serde(rename = "ResourceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_status: Option<String>,
    #[serde(rename = "ResourceStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_status_reason: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "StackId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "ValidationFailureMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_failure_mode: Option<String>,
    #[serde(rename = "ValidationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_name: Option<String>,
    #[serde(rename = "ValidationPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_path: Option<String>,
    #[serde(rename = "ValidationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<String>,
    #[serde(rename = "ValidationStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTypeRegistrationsInput")]
pub struct ListTypeRegistrationsInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RegistrationStatusFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_status_filter: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "TypeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_arn: Option<String>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListStackSetAutoDeploymentTargetsResult")]
pub struct ListStackSetAutoDeploymentTargetsOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Summaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summaries: Option<StackSetAutoDeploymentTargetSummaries>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StackSetAutoDeploymentTargetSummaries {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<StackSetAutoDeploymentTargetSummary>,
}
impl From<Vec<StackSetAutoDeploymentTargetSummary>> for StackSetAutoDeploymentTargetSummaries {
    fn from(v: Vec<StackSetAutoDeploymentTargetSummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<StackSetAutoDeploymentTargetSummary> for StackSetAutoDeploymentTargetSummaries {
    fn from_iter<I: IntoIterator<Item = StackSetAutoDeploymentTargetSummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<StackSetAutoDeploymentTargetSummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlStackSetAutoDeploymentTargetSummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<StackSetAutoDeploymentTargetSummary>,
}

impl From<Vec<StackSetAutoDeploymentTargetSummary>> for XmlStackSetAutoDeploymentTargetSummaryList {
    fn from(v: Vec<StackSetAutoDeploymentTargetSummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<StackSetAutoDeploymentTargetSummary>
    for XmlStackSetAutoDeploymentTargetSummaryList
{
    fn from_iter<I: IntoIterator<Item = StackSetAutoDeploymentTargetSummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StackSetAutoDeploymentTargetSummary")]
pub struct StackSetAutoDeploymentTargetSummary {
    #[serde(rename = "OrganizationalUnitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit_id: Option<String>,
    #[serde(rename = "Regions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<RegionList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StopStackSetOperationInput")]
pub struct StopStackSetOperationInput {
    #[serde(rename = "CallAs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_as: Option<String>,
    #[serde(rename = "OperationId")]
    #[serde(default)]
    pub operation_id: String,
    #[serde(rename = "StackSetName")]
    #[serde(default)]
    pub stack_set_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateGeneratedTemplateInput")]
pub struct CreateGeneratedTemplateInput {
    #[serde(rename = "GeneratedTemplateName")]
    #[serde(default)]
    pub generated_template_name: String,
    #[serde(rename = "Resources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<ResourceDefinitions>,
    #[serde(rename = "StackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_name: Option<String>,
    #[serde(rename = "TemplateConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_configuration: Option<TemplateConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceDefinitions {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ResourceDefinition>,
}
impl From<Vec<ResourceDefinition>> for ResourceDefinitions {
    fn from(v: Vec<ResourceDefinition>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ResourceDefinition> for ResourceDefinitions {
    fn from_iter<I: IntoIterator<Item = ResourceDefinition>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ResourceDefinition>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlResourceDefinitionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ResourceDefinition>,
}

impl From<Vec<ResourceDefinition>> for XmlResourceDefinitionList {
    fn from(v: Vec<ResourceDefinition>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ResourceDefinition> for XmlResourceDefinitionList {
    fn from_iter<I: IntoIterator<Item = ResourceDefinition>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResourceDefinition")]
pub struct ResourceDefinition {
    #[serde(rename = "LogicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_resource_id: Option<String>,
    #[serde(rename = "ResourceIdentifier")]
    #[serde(default)]
    pub resource_identifier: std::collections::HashMap<String, String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    pub resource_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeactivateOrganizationsAccessOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActivateOrganizationsAccessOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListExportsInput")]
pub struct ListExportsInput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SetStackPolicyInput")]
pub struct SetStackPolicyInput {
    #[serde(rename = "StackName")]
    #[serde(default)]
    pub stack_name: String,
    #[serde(rename = "StackPolicyBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_policy_body: Option<String>,
    #[serde(rename = "StackPolicyURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_policy_u_r_l: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeStackResourcesResult")]
pub struct DescribeStackResourcesOutput {
    #[serde(rename = "StackResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_resources: Option<StackResources>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StackResources {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<StackResource>,
}
impl From<Vec<StackResource>> for StackResources {
    fn from(v: Vec<StackResource>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<StackResource> for StackResources {
    fn from_iter<I: IntoIterator<Item = StackResource>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<StackResource>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlStackResourceList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<StackResource>,
}

impl From<Vec<StackResource>> for XmlStackResourceList {
    fn from(v: Vec<StackResource>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<StackResource> for XmlStackResourceList {
    fn from_iter<I: IntoIterator<Item = StackResource>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StackResource")]
pub struct StackResource {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DriftInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drift_information: Option<StackResourceDriftInformation>,
    #[serde(rename = "LogicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_resource_id: Option<String>,
    #[serde(rename = "ModuleInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_info: Option<ModuleInfo>,
    #[serde(rename = "PhysicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_resource_id: Option<String>,
    #[serde(rename = "ResourceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_status: Option<String>,
    #[serde(rename = "ResourceStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_status_reason: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "StackId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
    #[serde(rename = "StackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_name: Option<String>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StackResourceDriftInformation")]
pub struct StackResourceDriftInformation {
    #[serde(rename = "LastCheckTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_check_timestamp: Option<String>,
    #[serde(rename = "StackResourceDriftStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_resource_drift_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DetectStackResourceDriftInput")]
pub struct DetectStackResourceDriftInput {
    #[serde(rename = "LogicalResourceId")]
    #[serde(default)]
    pub logical_resource_id: String,
    #[serde(rename = "StackName")]
    #[serde(default)]
    pub stack_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeAccountLimitsInput")]
pub struct DescribeAccountLimitsInput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeStackDriftDetectionStatusResult")]
pub struct DescribeStackDriftDetectionStatusOutput {
    #[serde(rename = "DetectionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detection_status: Option<String>,
    #[serde(rename = "DetectionStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detection_status_reason: Option<String>,
    #[serde(rename = "DriftedStackResourceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drifted_stack_resource_count: Option<i32>,
    #[serde(rename = "StackDriftDetectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_drift_detection_id: Option<String>,
    #[serde(rename = "StackDriftStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_drift_status: Option<String>,
    #[serde(rename = "StackId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTypeRegistrationsResult")]
pub struct ListTypeRegistrationsOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RegistrationTokenList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_token_list: Option<RegistrationTokenList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegistrationTokenList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for RegistrationTokenList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for RegistrationTokenList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListStackSetOperationsInput")]
pub struct ListStackSetOperationsInput {
    #[serde(rename = "CallAs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_as: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StackSetName")]
    #[serde(default)]
    pub stack_set_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeStackSetResult")]
pub struct DescribeStackSetOutput {
    #[serde(rename = "StackSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set: Option<StackSet>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StackSet")]
pub struct StackSet {
    #[serde(rename = "AdministrationRoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administration_role_a_r_n: Option<String>,
    #[serde(rename = "AutoDeployment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_deployment: Option<AutoDeployment>,
    #[serde(rename = "Capabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Capabilities>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ExecutionRoleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_name: Option<String>,
    #[serde(rename = "ManagedExecution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_execution: Option<ManagedExecution>,
    #[serde(rename = "OrganizationalUnitIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit_ids: Option<OrganizationalUnitIdList>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Parameters>,
    #[serde(rename = "PermissionModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_model: Option<String>,
    #[serde(rename = "Regions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<RegionList>,
    #[serde(rename = "StackSetARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_a_r_n: Option<String>,
    #[serde(rename = "StackSetDriftDetectionDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_drift_detection_details: Option<StackSetDriftDetectionDetails>,
    #[serde(rename = "StackSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_id: Option<String>,
    #[serde(rename = "StackSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[serde(rename = "TemplateBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_body: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StackSetDriftDetectionDetails")]
pub struct StackSetDriftDetectionDetails {
    #[serde(rename = "DriftDetectionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drift_detection_status: Option<String>,
    #[serde(rename = "DriftStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drift_status: Option<String>,
    #[serde(rename = "DriftedStackInstancesCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drifted_stack_instances_count: Option<i32>,
    #[serde(rename = "FailedStackInstancesCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_stack_instances_count: Option<i32>,
    #[serde(rename = "InProgressStackInstancesCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_progress_stack_instances_count: Option<i32>,
    #[serde(rename = "InSyncStackInstancesCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_sync_stack_instances_count: Option<i32>,
    #[serde(rename = "LastDriftCheckTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_drift_check_timestamp: Option<String>,
    #[serde(rename = "TotalStackInstancesCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_stack_instances_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListStackInstanceResourceDriftsResult")]
pub struct ListStackInstanceResourceDriftsOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Summaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summaries: Option<StackInstanceResourceDriftsSummaries>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StackInstanceResourceDriftsSummaries {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<StackInstanceResourceDriftsSummary>,
}
impl From<Vec<StackInstanceResourceDriftsSummary>> for StackInstanceResourceDriftsSummaries {
    fn from(v: Vec<StackInstanceResourceDriftsSummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<StackInstanceResourceDriftsSummary> for StackInstanceResourceDriftsSummaries {
    fn from_iter<I: IntoIterator<Item = StackInstanceResourceDriftsSummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<StackInstanceResourceDriftsSummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlStackInstanceResourceDriftsSummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<StackInstanceResourceDriftsSummary>,
}

impl From<Vec<StackInstanceResourceDriftsSummary>> for XmlStackInstanceResourceDriftsSummaryList {
    fn from(v: Vec<StackInstanceResourceDriftsSummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<StackInstanceResourceDriftsSummary>
    for XmlStackInstanceResourceDriftsSummaryList
{
    fn from_iter<I: IntoIterator<Item = StackInstanceResourceDriftsSummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StackInstanceResourceDriftsSummary")]
pub struct StackInstanceResourceDriftsSummary {
    #[serde(rename = "LogicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_resource_id: Option<String>,
    #[serde(rename = "PhysicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_resource_id: Option<String>,
    #[serde(rename = "PhysicalResourceIdContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_resource_id_context: Option<PhysicalResourceIdContext>,
    #[serde(rename = "PropertyDifferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_differences: Option<PropertyDifferences>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "StackId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
    #[serde(rename = "StackResourceDriftStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_resource_drift_status: Option<String>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeStacksInput")]
pub struct DescribeStacksInput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTypeVersionsResult")]
pub struct ListTypeVersionsOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TypeVersionSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_version_summaries: Option<TypeVersionSummaries>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TypeVersionSummaries {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<TypeVersionSummary>,
}
impl From<Vec<TypeVersionSummary>> for TypeVersionSummaries {
    fn from(v: Vec<TypeVersionSummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<TypeVersionSummary> for TypeVersionSummaries {
    fn from_iter<I: IntoIterator<Item = TypeVersionSummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<TypeVersionSummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTypeVersionSummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<TypeVersionSummary>,
}

impl From<Vec<TypeVersionSummary>> for XmlTypeVersionSummaryList {
    fn from(v: Vec<TypeVersionSummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<TypeVersionSummary> for XmlTypeVersionSummaryList {
    fn from_iter<I: IntoIterator<Item = TypeVersionSummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TypeVersionSummary")]
pub struct TypeVersionSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "IsDefaultVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default_version: Option<bool>,
    #[serde(rename = "PublicVersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_version_number: Option<String>,
    #[serde(rename = "TimeCreated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_created: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ActivateTypeResult")]
pub struct ActivateTypeOutput {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StartResourceScanResult")]
pub struct StartResourceScanOutput {
    #[serde(rename = "ResourceScanId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_scan_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateStackInput")]
pub struct UpdateStackInput {
    #[serde(rename = "Capabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Capabilities>,
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "DisableRollback")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_rollback: Option<bool>,
    #[serde(rename = "NotificationARNs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_a_r_ns: Option<NotificationARNs>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Parameters>,
    #[serde(rename = "ResourceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<ResourceTypes>,
    #[serde(rename = "RetainExceptOnCreate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_except_on_create: Option<bool>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
    #[serde(rename = "RollbackConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_configuration: Option<RollbackConfiguration>,
    #[serde(rename = "StackName")]
    #[serde(default)]
    pub stack_name: String,
    #[serde(rename = "StackPolicyBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_policy_body: Option<String>,
    #[serde(rename = "StackPolicyDuringUpdateBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_policy_during_update_body: Option<String>,
    #[serde(rename = "StackPolicyDuringUpdateURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_policy_during_update_u_r_l: Option<String>,
    #[serde(rename = "StackPolicyURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_policy_u_r_l: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[serde(rename = "TemplateBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_body: Option<String>,
    #[serde(rename = "TemplateURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_u_r_l: Option<String>,
    #[serde(rename = "UsePreviousTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_previous_template: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateStackResult")]
pub struct UpdateStackOutput {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(rename = "StackId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterTypeOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListChangeSetsResult")]
pub struct ListChangeSetsOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Summaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summaries: Option<ChangeSetSummaries>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChangeSetSummaries {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ChangeSetSummary>,
}
impl From<Vec<ChangeSetSummary>> for ChangeSetSummaries {
    fn from(v: Vec<ChangeSetSummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ChangeSetSummary> for ChangeSetSummaries {
    fn from_iter<I: IntoIterator<Item = ChangeSetSummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ChangeSetSummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlChangeSetSummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ChangeSetSummary>,
}

impl From<Vec<ChangeSetSummary>> for XmlChangeSetSummaryList {
    fn from(v: Vec<ChangeSetSummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ChangeSetSummary> for XmlChangeSetSummaryList {
    fn from_iter<I: IntoIterator<Item = ChangeSetSummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ChangeSetSummary")]
pub struct ChangeSetSummary {
    #[serde(rename = "ChangeSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_set_id: Option<String>,
    #[serde(rename = "ChangeSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_set_name: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ExecutionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_status: Option<String>,
    #[serde(rename = "ImportExistingResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_existing_resources: Option<bool>,
    #[serde(rename = "IncludeNestedStacks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_nested_stacks: Option<bool>,
    #[serde(rename = "ParentChangeSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_change_set_id: Option<String>,
    #[serde(rename = "RootChangeSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_change_set_id: Option<String>,
    #[serde(rename = "StackId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
    #[serde(rename = "StackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeStackEventsResult")]
pub struct DescribeStackEventsOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StackEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_events: Option<StackEvents>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StackEvents {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<StackEvent>,
}
impl From<Vec<StackEvent>> for StackEvents {
    fn from(v: Vec<StackEvent>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<StackEvent> for StackEvents {
    fn from_iter<I: IntoIterator<Item = StackEvent>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<StackEvent>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlStackEventList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<StackEvent>,
}

impl From<Vec<StackEvent>> for XmlStackEventList {
    fn from(v: Vec<StackEvent>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<StackEvent> for XmlStackEventList {
    fn from_iter<I: IntoIterator<Item = StackEvent>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StackEvent")]
pub struct StackEvent {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "DetailedStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_status: Option<String>,
    #[serde(rename = "EventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(rename = "HookFailureMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_failure_mode: Option<String>,
    #[serde(rename = "HookInvocationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_invocation_id: Option<String>,
    #[serde(rename = "HookInvocationPoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_invocation_point: Option<String>,
    #[serde(rename = "HookStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_status: Option<String>,
    #[serde(rename = "HookStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_status_reason: Option<String>,
    #[serde(rename = "HookType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_type: Option<String>,
    #[serde(rename = "LogicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_resource_id: Option<String>,
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(rename = "PhysicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_resource_id: Option<String>,
    #[serde(rename = "ResourceProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_properties: Option<String>,
    #[serde(rename = "ResourceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_status: Option<String>,
    #[serde(rename = "ResourceStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_status_reason: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "StackId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
    #[serde(rename = "StackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_name: Option<String>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteStackSetOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetGeneratedTemplateResult")]
pub struct GetGeneratedTemplateOutput {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TemplateBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_body: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ExecuteChangeSetInput")]
pub struct ExecuteChangeSetInput {
    #[serde(rename = "ChangeSetName")]
    #[serde(default)]
    pub change_set_name: String,
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "DisableRollback")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_rollback: Option<bool>,
    #[serde(rename = "RetainExceptOnCreate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_except_on_create: Option<bool>,
    #[serde(rename = "StackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListStackInstanceResourceDriftsInput")]
pub struct ListStackInstanceResourceDriftsInput {
    #[serde(rename = "CallAs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_as: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OperationId")]
    #[serde(default)]
    pub operation_id: String,
    #[serde(rename = "StackInstanceAccount")]
    #[serde(default)]
    pub stack_instance_account: String,
    #[serde(rename = "StackInstanceRegion")]
    #[serde(default)]
    pub stack_instance_region: String,
    #[serde(rename = "StackInstanceResourceDriftStatuses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_instance_resource_drift_statuses: Option<StackResourceDriftStatusFilters>,
    #[serde(rename = "StackSetName")]
    #[serde(default)]
    pub stack_set_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StackResourceDriftStatusFilters {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for StackResourceDriftStatusFilters {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for StackResourceDriftStatusFilters {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListStackResourcesInput")]
pub struct ListStackResourcesInput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StackName")]
    #[serde(default)]
    pub stack_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeStackInstanceInput")]
pub struct DescribeStackInstanceInput {
    #[serde(rename = "CallAs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_as: Option<String>,
    #[serde(rename = "StackInstanceAccount")]
    #[serde(default)]
    pub stack_instance_account: String,
    #[serde(rename = "StackInstanceRegion")]
    #[serde(default)]
    pub stack_instance_region: String,
    #[serde(rename = "StackSetName")]
    #[serde(default)]
    pub stack_set_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeregisterTypeInput")]
pub struct DeregisterTypeInput {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ImportStacksToStackSetInput")]
pub struct ImportStacksToStackSetInput {
    #[serde(rename = "CallAs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_as: Option<String>,
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(rename = "OperationPreferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_preferences: Option<StackSetOperationPreferences>,
    #[serde(rename = "OrganizationalUnitIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit_ids: Option<OrganizationalUnitIdList>,
    #[serde(rename = "StackIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_ids: Option<StackIdList>,
    #[serde(rename = "StackIdsUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_ids_url: Option<String>,
    #[serde(rename = "StackSetName")]
    #[serde(default)]
    pub stack_set_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StackIdList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for StackIdList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for StackIdList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteStackInstancesInput")]
pub struct DeleteStackInstancesInput {
    #[serde(rename = "Accounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<AccountList>,
    #[serde(rename = "CallAs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_as: Option<String>,
    #[serde(rename = "DeploymentTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_targets: Option<DeploymentTargets>,
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(rename = "OperationPreferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_preferences: Option<StackSetOperationPreferences>,
    #[serde(rename = "Regions")]
    #[serde(default)]
    pub regions: RegionList,
    #[serde(rename = "RetainStacks")]
    #[serde(default)]
    pub retain_stacks: bool,
    #[serde(rename = "StackSetName")]
    #[serde(default)]
    pub stack_set_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeTypeResult")]
pub struct DescribeTypeOutput {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "AutoUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_update: Option<bool>,
    #[serde(rename = "ConfigurationSchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_schema: Option<String>,
    #[serde(rename = "DefaultVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version_id: Option<String>,
    #[serde(rename = "DeprecatedStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated_status: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DocumentationUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,
    #[serde(rename = "ExecutionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(rename = "IsActivated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_activated: Option<bool>,
    #[serde(rename = "IsDefaultVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default_version: Option<bool>,
    #[serde(rename = "LastUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    #[serde(rename = "LatestPublicVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_public_version: Option<String>,
    #[serde(rename = "LoggingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_config: Option<LoggingConfig>,
    #[serde(rename = "OriginalTypeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_type_arn: Option<String>,
    #[serde(rename = "OriginalTypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_type_name: Option<String>,
    #[serde(rename = "ProvisioningType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_type: Option<String>,
    #[serde(rename = "PublicVersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_version_number: Option<String>,
    #[serde(rename = "PublisherId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_id: Option<String>,
    #[serde(rename = "RequiredActivatedTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_activated_types: Option<RequiredActivatedTypes>,
    #[serde(rename = "Schema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(rename = "SourceUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    #[serde(rename = "TimeCreated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_created: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[serde(rename = "TypeTestsStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_tests_status: Option<String>,
    #[serde(rename = "TypeTestsStatusDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_tests_status_description: Option<String>,
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RequiredActivatedTypes {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<RequiredActivatedType>,
}
impl From<Vec<RequiredActivatedType>> for RequiredActivatedTypes {
    fn from(v: Vec<RequiredActivatedType>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<RequiredActivatedType> for RequiredActivatedTypes {
    fn from_iter<I: IntoIterator<Item = RequiredActivatedType>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<RequiredActivatedType>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlRequiredActivatedTypeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<RequiredActivatedType>,
}

impl From<Vec<RequiredActivatedType>> for XmlRequiredActivatedTypeList {
    fn from(v: Vec<RequiredActivatedType>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<RequiredActivatedType> for XmlRequiredActivatedTypeList {
    fn from_iter<I: IntoIterator<Item = RequiredActivatedType>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RequiredActivatedType")]
pub struct RequiredActivatedType {
    #[serde(rename = "OriginalTypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_type_name: Option<String>,
    #[serde(rename = "PublisherId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_id: Option<String>,
    #[serde(rename = "SupportedMajorVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_major_versions: Option<SupportedMajorVersions>,
    #[serde(rename = "TypeNameAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name_alias: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SupportedMajorVersions {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<i32>,
}
impl From<Vec<i32>> for SupportedMajorVersions {
    fn from(v: Vec<i32>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<i32> for SupportedMajorVersions {
    fn from_iter<I: IntoIterator<Item = i32>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<i32>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Xmli32List {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<i32>,
}

impl From<Vec<i32>> for Xmli32List {
    fn from(v: Vec<i32>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<i32> for Xmli32List {
    fn from_iter<I: IntoIterator<Item = i32>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeStackEventsInput")]
pub struct DescribeStackEventsInput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StackName")]
    #[serde(default)]
    pub stack_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SetTypeConfigurationInput")]
pub struct SetTypeConfigurationInput {
    #[serde(rename = "Configuration")]
    #[serde(default)]
    pub configuration: String,
    #[serde(rename = "ConfigurationAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_alias: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "TypeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_arn: Option<String>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeStackSetOperationResult")]
pub struct DescribeStackSetOperationOutput {
    #[serde(rename = "StackSetOperation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_operation: Option<StackSetOperation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StackSetOperation")]
pub struct StackSetOperation {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "AdministrationRoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administration_role_a_r_n: Option<String>,
    #[serde(rename = "CreationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<String>,
    #[serde(rename = "DeploymentTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_targets: Option<DeploymentTargets>,
    #[serde(rename = "EndTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_timestamp: Option<String>,
    #[serde(rename = "ExecutionRoleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_name: Option<String>,
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(rename = "OperationPreferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_preferences: Option<StackSetOperationPreferences>,
    #[serde(rename = "RetainStacks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_stacks: Option<bool>,
    #[serde(rename = "StackSetDriftDetectionDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_drift_detection_details: Option<StackSetDriftDetectionDetails>,
    #[serde(rename = "StackSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<StackSetOperationStatusDetails>,
    #[serde(rename = "StatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecordHandlerProgressOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateGeneratedTemplateResult")]
pub struct CreateGeneratedTemplateOutput {
    #[serde(rename = "GeneratedTemplateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_template_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListChangeSetsInput")]
pub struct ListChangeSetsInput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StackName")]
    #[serde(default)]
    pub stack_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateGeneratedTemplateInput")]
pub struct UpdateGeneratedTemplateInput {
    #[serde(rename = "AddResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_resources: Option<ResourceDefinitions>,
    #[serde(rename = "GeneratedTemplateName")]
    #[serde(default)]
    pub generated_template_name: String,
    #[serde(rename = "NewGeneratedTemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_generated_template_name: Option<String>,
    #[serde(rename = "RefreshAllResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_all_resources: Option<bool>,
    #[serde(rename = "RemoveResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_resources: Option<JazzLogicalResourceIds>,
    #[serde(rename = "TemplateConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_configuration: Option<TemplateConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JazzLogicalResourceIds {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for JazzLogicalResourceIds {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for JazzLogicalResourceIds {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeStackResourceDriftsInput")]
pub struct DescribeStackResourceDriftsInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StackName")]
    #[serde(default)]
    pub stack_name: String,
    #[serde(rename = "StackResourceDriftStatusFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_resource_drift_status_filters: Option<StackResourceDriftStatusFilters>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetTypeDefaultVersionOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeTypeInput")]
pub struct DescribeTypeInput {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "PublicVersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_version_number: Option<String>,
    #[serde(rename = "PublisherId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "BatchDescribeTypeConfigurationsResult")]
pub struct BatchDescribeTypeConfigurationsOutput {
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<BatchDescribeTypeConfigurationsErrors>,
    #[serde(rename = "TypeConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_configurations: Option<TypeConfigurationDetailsList>,
    #[serde(rename = "UnprocessedTypeConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_type_configurations: Option<UnprocessedTypeConfigurations>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDescribeTypeConfigurationsErrors {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<BatchDescribeTypeConfigurationsError>,
}
impl From<Vec<BatchDescribeTypeConfigurationsError>> for BatchDescribeTypeConfigurationsErrors {
    fn from(v: Vec<BatchDescribeTypeConfigurationsError>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<BatchDescribeTypeConfigurationsError> for BatchDescribeTypeConfigurationsErrors {
    fn from_iter<I: IntoIterator<Item = BatchDescribeTypeConfigurationsError>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<BatchDescribeTypeConfigurationsError>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlBatchDescribeTypeConfigurationsErrorList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<BatchDescribeTypeConfigurationsError>,
}

impl From<Vec<BatchDescribeTypeConfigurationsError>>
    for XmlBatchDescribeTypeConfigurationsErrorList
{
    fn from(v: Vec<BatchDescribeTypeConfigurationsError>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<BatchDescribeTypeConfigurationsError>
    for XmlBatchDescribeTypeConfigurationsErrorList
{
    fn from_iter<I: IntoIterator<Item = BatchDescribeTypeConfigurationsError>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "BatchDescribeTypeConfigurationsError")]
pub struct BatchDescribeTypeConfigurationsError {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "TypeConfigurationIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_configuration_identifier: Option<TypeConfigurationIdentifier>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TypeConfigurationDetailsList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<TypeConfigurationDetails>,
}
impl From<Vec<TypeConfigurationDetails>> for TypeConfigurationDetailsList {
    fn from(v: Vec<TypeConfigurationDetails>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<TypeConfigurationDetails> for TypeConfigurationDetailsList {
    fn from_iter<I: IntoIterator<Item = TypeConfigurationDetails>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<TypeConfigurationDetails>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTypeConfigurationDetailsList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<TypeConfigurationDetails>,
}

impl From<Vec<TypeConfigurationDetails>> for XmlTypeConfigurationDetailsList {
    fn from(v: Vec<TypeConfigurationDetails>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<TypeConfigurationDetails> for XmlTypeConfigurationDetailsList {
    fn from_iter<I: IntoIterator<Item = TypeConfigurationDetails>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TypeConfigurationDetails")]
pub struct TypeConfigurationDetails {
    #[serde(rename = "Alias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
    #[serde(rename = "IsDefaultConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default_configuration: Option<bool>,
    #[serde(rename = "LastUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    #[serde(rename = "TypeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_arn: Option<String>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnprocessedTypeConfigurations {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<TypeConfigurationIdentifier>,
}
impl From<Vec<TypeConfigurationIdentifier>> for UnprocessedTypeConfigurations {
    fn from(v: Vec<TypeConfigurationIdentifier>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<TypeConfigurationIdentifier> for UnprocessedTypeConfigurations {
    fn from_iter<I: IntoIterator<Item = TypeConfigurationIdentifier>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateStackRefactorInput")]
pub struct CreateStackRefactorInput {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EnableStackCreation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_stack_creation: Option<bool>,
    #[serde(rename = "ResourceMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_mappings: Option<ResourceMappings>,
    #[serde(rename = "StackDefinitions")]
    #[serde(default)]
    pub stack_definitions: StackDefinitions,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceMappings {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ResourceMapping>,
}
impl From<Vec<ResourceMapping>> for ResourceMappings {
    fn from(v: Vec<ResourceMapping>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ResourceMapping> for ResourceMappings {
    fn from_iter<I: IntoIterator<Item = ResourceMapping>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ResourceMapping>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlResourceMappingList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ResourceMapping>,
}

impl From<Vec<ResourceMapping>> for XmlResourceMappingList {
    fn from(v: Vec<ResourceMapping>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ResourceMapping> for XmlResourceMappingList {
    fn from_iter<I: IntoIterator<Item = ResourceMapping>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResourceMapping")]
pub struct ResourceMapping {
    #[serde(rename = "Destination")]
    #[serde(default)]
    pub destination: ResourceLocation,
    #[serde(rename = "Source")]
    #[serde(default)]
    pub source: ResourceLocation,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResourceLocation")]
pub struct ResourceLocation {
    #[serde(rename = "LogicalResourceId")]
    #[serde(default)]
    pub logical_resource_id: String,
    #[serde(rename = "StackName")]
    #[serde(default)]
    pub stack_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StackDefinitions {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<StackDefinition>,
}
impl From<Vec<StackDefinition>> for StackDefinitions {
    fn from(v: Vec<StackDefinition>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<StackDefinition> for StackDefinitions {
    fn from_iter<I: IntoIterator<Item = StackDefinition>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<StackDefinition>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlStackDefinitionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<StackDefinition>,
}

impl From<Vec<StackDefinition>> for XmlStackDefinitionList {
    fn from(v: Vec<StackDefinition>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<StackDefinition> for XmlStackDefinitionList {
    fn from_iter<I: IntoIterator<Item = StackDefinition>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StackDefinition")]
pub struct StackDefinition {
    #[serde(rename = "StackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_name: Option<String>,
    #[serde(rename = "TemplateBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_body: Option<String>,
    #[serde(rename = "TemplateURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_u_r_l: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeChangeSetInput")]
pub struct DescribeChangeSetInput {
    #[serde(rename = "ChangeSetName")]
    #[serde(default)]
    pub change_set_name: String,
    #[serde(rename = "IncludePropertyValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_property_values: Option<bool>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListHookResultsResult")]
pub struct ListHookResultsOutput {
    #[serde(rename = "HookResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_results: Option<HookResultSummaries>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TargetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    #[serde(rename = "TargetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HookResultSummaries {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<HookResultSummary>,
}
impl From<Vec<HookResultSummary>> for HookResultSummaries {
    fn from(v: Vec<HookResultSummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<HookResultSummary> for HookResultSummaries {
    fn from_iter<I: IntoIterator<Item = HookResultSummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<HookResultSummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlHookResultSummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<HookResultSummary>,
}

impl From<Vec<HookResultSummary>> for XmlHookResultSummaryList {
    fn from(v: Vec<HookResultSummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<HookResultSummary> for XmlHookResultSummaryList {
    fn from_iter<I: IntoIterator<Item = HookResultSummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "HookResultSummary")]
pub struct HookResultSummary {
    #[serde(rename = "FailureMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_mode: Option<String>,
    #[serde(rename = "HookExecutionTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_execution_target: Option<String>,
    #[serde(rename = "HookResultId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_result_id: Option<String>,
    #[serde(rename = "HookStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_status_reason: Option<String>,
    #[serde(rename = "InvocationPoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_point: Option<String>,
    #[serde(rename = "InvokedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoked_at: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TargetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    #[serde(rename = "TargetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    #[serde(rename = "TypeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_arn: Option<String>,
    #[serde(rename = "TypeConfigurationVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_configuration_version_id: Option<String>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[serde(rename = "TypeVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeOrganizationsAccessResult")]
pub struct DescribeOrganizationsAccessOutput {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetTemplateResult")]
pub struct GetTemplateOutput {
    #[serde(rename = "StagesAvailable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stages_available: Option<StageList>,
    #[serde(rename = "TemplateBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_body: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StageList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for StageList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for StageList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetTemplateSummaryResult")]
pub struct GetTemplateSummaryOutput {
    #[serde(rename = "Capabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Capabilities>,
    #[serde(rename = "CapabilitiesReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities_reason: Option<String>,
    #[serde(rename = "DeclaredTransforms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub declared_transforms: Option<TransformsList>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Metadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParameterDeclarations>,
    #[serde(rename = "ResourceIdentifierSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier_summaries: Option<ResourceIdentifierSummaries>,
    #[serde(rename = "ResourceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<ResourceTypes>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "Warnings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Warnings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransformsList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for TransformsList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for TransformsList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParameterDeclarations {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ParameterDeclaration>,
}
impl From<Vec<ParameterDeclaration>> for ParameterDeclarations {
    fn from(v: Vec<ParameterDeclaration>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ParameterDeclaration> for ParameterDeclarations {
    fn from_iter<I: IntoIterator<Item = ParameterDeclaration>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ParameterDeclaration>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlParameterDeclarationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ParameterDeclaration>,
}

impl From<Vec<ParameterDeclaration>> for XmlParameterDeclarationList {
    fn from(v: Vec<ParameterDeclaration>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ParameterDeclaration> for XmlParameterDeclarationList {
    fn from_iter<I: IntoIterator<Item = ParameterDeclaration>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ParameterDeclaration")]
pub struct ParameterDeclaration {
    #[serde(rename = "DefaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "NoEcho")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_echo: Option<bool>,
    #[serde(rename = "ParameterConstraints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_constraints: Option<ParameterConstraints>,
    #[serde(rename = "ParameterKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_key: Option<String>,
    #[serde(rename = "ParameterType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ParameterConstraints")]
pub struct ParameterConstraints {
    #[serde(rename = "AllowedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<AllowedValues>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AllowedValues {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for AllowedValues {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for AllowedValues {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceIdentifierSummaries {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ResourceIdentifierSummary>,
}
impl From<Vec<ResourceIdentifierSummary>> for ResourceIdentifierSummaries {
    fn from(v: Vec<ResourceIdentifierSummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ResourceIdentifierSummary> for ResourceIdentifierSummaries {
    fn from_iter<I: IntoIterator<Item = ResourceIdentifierSummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ResourceIdentifierSummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlResourceIdentifierSummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ResourceIdentifierSummary>,
}

impl From<Vec<ResourceIdentifierSummary>> for XmlResourceIdentifierSummaryList {
    fn from(v: Vec<ResourceIdentifierSummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ResourceIdentifierSummary> for XmlResourceIdentifierSummaryList {
    fn from_iter<I: IntoIterator<Item = ResourceIdentifierSummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResourceIdentifierSummary")]
pub struct ResourceIdentifierSummary {
    #[serde(rename = "LogicalResourceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_resource_ids: Option<LogicalResourceIds>,
    #[serde(rename = "ResourceIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifiers: Option<ResourceIdentifiers>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogicalResourceIds {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for LogicalResourceIds {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for LogicalResourceIds {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceIdentifiers {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ResourceIdentifiers {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ResourceIdentifiers {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Warnings")]
pub struct Warnings {
    #[serde(rename = "UnrecognizedResourceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unrecognized_resource_types: Option<ResourceTypes>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ImportStacksToStackSetResult")]
pub struct ImportStacksToStackSetOutput {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetTemplateInput")]
pub struct GetTemplateInput {
    #[serde(rename = "ChangeSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_set_name: Option<String>,
    #[serde(rename = "StackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_name: Option<String>,
    #[serde(rename = "TemplateStage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_stage: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListStackRefactorActionsResult")]
pub struct ListStackRefactorActionsOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StackRefactorActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_refactor_actions: Option<StackRefactorActions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StackRefactorActions {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<StackRefactorAction>,
}
impl From<Vec<StackRefactorAction>> for StackRefactorActions {
    fn from(v: Vec<StackRefactorAction>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<StackRefactorAction> for StackRefactorActions {
    fn from_iter<I: IntoIterator<Item = StackRefactorAction>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<StackRefactorAction>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlStackRefactorActionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<StackRefactorAction>,
}

impl From<Vec<StackRefactorAction>> for XmlStackRefactorActionList {
    fn from(v: Vec<StackRefactorAction>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<StackRefactorAction> for XmlStackRefactorActionList {
    fn from_iter<I: IntoIterator<Item = StackRefactorAction>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StackRefactorAction")]
pub struct StackRefactorAction {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Detection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detection: Option<String>,
    #[serde(rename = "DetectionReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detection_reason: Option<String>,
    #[serde(rename = "Entity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<String>,
    #[serde(rename = "PhysicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_resource_id: Option<String>,
    #[serde(rename = "ResourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<String>,
    #[serde(rename = "ResourceMapping")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_mapping: Option<ResourceMapping>,
    #[serde(rename = "TagResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_resources: Option<StackRefactorTagResources>,
    #[serde(rename = "UntagResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub untag_resources: Option<StackRefactorUntagResources>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StackRefactorTagResources {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Tag>,
}
impl From<Vec<Tag>> for StackRefactorTagResources {
    fn from(v: Vec<Tag>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Tag> for StackRefactorTagResources {
    fn from_iter<I: IntoIterator<Item = Tag>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StackRefactorUntagResources {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for StackRefactorUntagResources {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for StackRefactorUntagResources {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTypesResult")]
pub struct ListTypesOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TypeSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_summaries: Option<TypeSummaries>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TypeSummaries {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<TypeSummary>,
}
impl From<Vec<TypeSummary>> for TypeSummaries {
    fn from(v: Vec<TypeSummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<TypeSummary> for TypeSummaries {
    fn from_iter<I: IntoIterator<Item = TypeSummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<TypeSummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTypeSummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<TypeSummary>,
}

impl From<Vec<TypeSummary>> for XmlTypeSummaryList {
    fn from(v: Vec<TypeSummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<TypeSummary> for XmlTypeSummaryList {
    fn from_iter<I: IntoIterator<Item = TypeSummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TypeSummary")]
pub struct TypeSummary {
    #[serde(rename = "DefaultVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_version_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "IsActivated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_activated: Option<bool>,
    #[serde(rename = "LastUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    #[serde(rename = "LatestPublicVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_public_version: Option<String>,
    #[serde(rename = "OriginalTypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_type_name: Option<String>,
    #[serde(rename = "PublicVersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_version_number: Option<String>,
    #[serde(rename = "PublisherId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_id: Option<String>,
    #[serde(rename = "PublisherIdentity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_identity: Option<String>,
    #[serde(rename = "PublisherName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_name: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "TypeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_arn: Option<String>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateStackSetResult")]
pub struct UpdateStackSetOutput {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ValidateTemplateInput")]
pub struct ValidateTemplateInput {
    #[serde(rename = "TemplateBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_body: Option<String>,
    #[serde(rename = "TemplateURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_u_r_l: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateChangeSetResult")]
pub struct CreateChangeSetOutput {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "StackId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActivateOrganizationsAccessInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TestTypeInput")]
pub struct TestTypeInput {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "LogDeliveryBucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_delivery_bucket: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeStackInstanceResult")]
pub struct DescribeStackInstanceOutput {
    #[serde(rename = "StackInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_instance: Option<StackInstance>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StackInstance")]
pub struct StackInstance {
    #[serde(rename = "Account")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(rename = "DriftStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drift_status: Option<String>,
    #[serde(rename = "LastDriftCheckTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_drift_check_timestamp: Option<String>,
    #[serde(rename = "LastOperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_operation_id: Option<String>,
    #[serde(rename = "OrganizationalUnitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit_id: Option<String>,
    #[serde(rename = "ParameterOverrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_overrides: Option<Parameters>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "StackId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
    #[serde(rename = "StackInstanceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_instance_status: Option<StackInstanceComprehensiveStatus>,
    #[serde(rename = "StackSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StackInstanceComprehensiveStatus")]
pub struct StackInstanceComprehensiveStatus {
    #[serde(rename = "DetailedStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListStackRefactorActionsInput")]
pub struct ListStackRefactorActionsInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StackRefactorId")]
    #[serde(default)]
    pub stack_refactor_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListStackRefactorsResult")]
pub struct ListStackRefactorsOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StackRefactorSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_refactor_summaries: Option<StackRefactorSummaries>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StackRefactorSummaries {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<StackRefactorSummary>,
}
impl From<Vec<StackRefactorSummary>> for StackRefactorSummaries {
    fn from(v: Vec<StackRefactorSummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<StackRefactorSummary> for StackRefactorSummaries {
    fn from_iter<I: IntoIterator<Item = StackRefactorSummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<StackRefactorSummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlStackRefactorSummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<StackRefactorSummary>,
}

impl From<Vec<StackRefactorSummary>> for XmlStackRefactorSummaryList {
    fn from(v: Vec<StackRefactorSummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<StackRefactorSummary> for XmlStackRefactorSummaryList {
    fn from_iter<I: IntoIterator<Item = StackRefactorSummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StackRefactorSummary")]
pub struct StackRefactorSummary {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ExecutionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_status: Option<String>,
    #[serde(rename = "ExecutionStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_status_reason: Option<String>,
    #[serde(rename = "StackRefactorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_refactor_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTypesInput")]
pub struct ListTypesInput {
    #[serde(rename = "DeprecatedStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated_status: Option<String>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<TypeFilters>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ProvisioningType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_type: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TypeFilters")]
pub struct TypeFilters {
    #[serde(rename = "Category")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "PublisherId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_id: Option<String>,
    #[serde(rename = "TypeNamePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateTerminationProtectionResult")]
pub struct UpdateTerminationProtectionOutput {
    #[serde(rename = "StackId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListHookResultsInput")]
pub struct ListHookResultsInput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TargetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    #[serde(rename = "TargetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    #[serde(rename = "TypeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ValidateTemplateResult")]
pub struct ValidateTemplateOutput {
    #[serde(rename = "Capabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Capabilities>,
    #[serde(rename = "CapabilitiesReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities_reason: Option<String>,
    #[serde(rename = "DeclaredTransforms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub declared_transforms: Option<TransformsList>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<TemplateParameters>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TemplateParameters {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<TemplateParameter>,
}
impl From<Vec<TemplateParameter>> for TemplateParameters {
    fn from(v: Vec<TemplateParameter>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<TemplateParameter> for TemplateParameters {
    fn from_iter<I: IntoIterator<Item = TemplateParameter>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<TemplateParameter>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTemplateParameterList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<TemplateParameter>,
}

impl From<Vec<TemplateParameter>> for XmlTemplateParameterList {
    fn from(v: Vec<TemplateParameter>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<TemplateParameter> for XmlTemplateParameterList {
    fn from_iter<I: IntoIterator<Item = TemplateParameter>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TemplateParameter")]
pub struct TemplateParameter {
    #[serde(rename = "DefaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "NoEcho")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_echo: Option<bool>,
    #[serde(rename = "ParameterKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeResourceScanInput")]
pub struct DescribeResourceScanInput {
    #[serde(rename = "ResourceScanId")]
    #[serde(default)]
    pub resource_scan_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribePublisherInput")]
pub struct DescribePublisherInput {
    #[serde(rename = "PublisherId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeStackRefactorResult")]
pub struct DescribeStackRefactorOutput {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ExecutionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_status: Option<String>,
    #[serde(rename = "ExecutionStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_status_reason: Option<String>,
    #[serde(rename = "StackIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_ids: Option<StackIds>,
    #[serde(rename = "StackRefactorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_refactor_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StackIds {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for StackIds {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for StackIds {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SetTypeDefaultVersionInput")]
pub struct SetTypeDefaultVersionInput {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeStackSetInput")]
pub struct DescribeStackSetInput {
    #[serde(rename = "CallAs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_as: Option<String>,
    #[serde(rename = "StackSetName")]
    #[serde(default)]
    pub stack_set_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeResourceScanResult")]
pub struct DescribeResourceScanOutput {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "PercentageCompleted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage_completed: Option<f64>,
    #[serde(rename = "ResourceScanId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_scan_id: Option<String>,
    #[serde(rename = "ResourceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<ResourceTypes>,
    #[serde(rename = "ResourcesRead")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources_read: Option<i32>,
    #[serde(rename = "ResourcesScanned")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources_scanned: Option<i32>,
    #[serde(rename = "ScanFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_filters: Option<ScanFilters>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScanFilters {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ScanFilter>,
}
impl From<Vec<ScanFilter>> for ScanFilters {
    fn from(v: Vec<ScanFilter>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ScanFilter> for ScanFilters {
    fn from_iter<I: IntoIterator<Item = ScanFilter>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ScanFilter>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlScanFilterList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ScanFilter>,
}

impl From<Vec<ScanFilter>> for XmlScanFilterList {
    fn from(v: Vec<ScanFilter>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ScanFilter> for XmlScanFilterList {
    fn from_iter<I: IntoIterator<Item = ScanFilter>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ScanFilter")]
pub struct ScanFilter {
    #[serde(rename = "Types")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<ResourceTypeFilters>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceTypeFilters {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ResourceTypeFilters {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ResourceTypeFilters {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RollbackStackInput")]
pub struct RollbackStackInput {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "RetainExceptOnCreate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_except_on_create: Option<bool>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
    #[serde(rename = "StackName")]
    #[serde(default)]
    pub stack_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteGeneratedTemplateInput")]
pub struct DeleteGeneratedTemplateInput {
    #[serde(rename = "GeneratedTemplateName")]
    #[serde(default)]
    pub generated_template_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeStackResourceResult")]
pub struct DescribeStackResourceOutput {
    #[serde(rename = "StackResourceDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_resource_detail: Option<StackResourceDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StackResourceDetail")]
pub struct StackResourceDetail {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DriftInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drift_information: Option<StackResourceDriftInformation>,
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    #[serde(rename = "LogicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_resource_id: Option<String>,
    #[serde(rename = "Metadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<String>,
    #[serde(rename = "ModuleInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_info: Option<ModuleInfo>,
    #[serde(rename = "PhysicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_resource_id: Option<String>,
    #[serde(rename = "ResourceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_status: Option<String>,
    #[serde(rename = "ResourceStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_status_reason: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "StackId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
    #[serde(rename = "StackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DetectStackDriftInput")]
pub struct DetectStackDriftInput {
    #[serde(rename = "LogicalResourceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_resource_ids: Option<LogicalResourceIds>,
    #[serde(rename = "StackName")]
    #[serde(default)]
    pub stack_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeStackResourceInput")]
pub struct DescribeStackResourceInput {
    #[serde(rename = "LogicalResourceId")]
    #[serde(default)]
    pub logical_resource_id: String,
    #[serde(rename = "StackName")]
    #[serde(default)]
    pub stack_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EstimateTemplateCostResult")]
pub struct EstimateTemplateCostOutput {
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListStackRefactorsInput")]
pub struct ListStackRefactorsInput {
    #[serde(rename = "ExecutionStatusFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_status_filter: Option<StackRefactorExecutionStatusFilter>,
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
pub struct StackRefactorExecutionStatusFilter {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for StackRefactorExecutionStatusFilter {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for StackRefactorExecutionStatusFilter {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListStackInstancesResult")]
pub struct ListStackInstancesOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Summaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summaries: Option<StackInstanceSummaries>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StackInstanceSummaries {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<StackInstanceSummary>,
}
impl From<Vec<StackInstanceSummary>> for StackInstanceSummaries {
    fn from(v: Vec<StackInstanceSummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<StackInstanceSummary> for StackInstanceSummaries {
    fn from_iter<I: IntoIterator<Item = StackInstanceSummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<StackInstanceSummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlStackInstanceSummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<StackInstanceSummary>,
}

impl From<Vec<StackInstanceSummary>> for XmlStackInstanceSummaryList {
    fn from(v: Vec<StackInstanceSummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<StackInstanceSummary> for XmlStackInstanceSummaryList {
    fn from_iter<I: IntoIterator<Item = StackInstanceSummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StackInstanceSummary")]
pub struct StackInstanceSummary {
    #[serde(rename = "Account")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(rename = "DriftStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drift_status: Option<String>,
    #[serde(rename = "LastDriftCheckTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_drift_check_timestamp: Option<String>,
    #[serde(rename = "LastOperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_operation_id: Option<String>,
    #[serde(rename = "OrganizationalUnitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit_id: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "StackId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
    #[serde(rename = "StackInstanceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_instance_status: Option<StackInstanceComprehensiveStatus>,
    #[serde(rename = "StackSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StartResourceScanInput")]
pub struct StartResourceScanInput {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "ScanFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_filters: Option<ScanFilters>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetHookResultResult")]
pub struct GetHookResultOutput {
    #[serde(rename = "Annotations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<AnnotationList>,
    #[serde(rename = "FailureMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_mode: Option<String>,
    #[serde(rename = "HookResultId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_result_id: Option<String>,
    #[serde(rename = "HookStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_status_reason: Option<String>,
    #[serde(rename = "InvocationPoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_point: Option<String>,
    #[serde(rename = "InvokedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoked_at: Option<String>,
    #[serde(rename = "OriginalTypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_type_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Target")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<HookTarget>,
    #[serde(rename = "TypeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_arn: Option<String>,
    #[serde(rename = "TypeConfigurationVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_configuration_version_id: Option<String>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[serde(rename = "TypeVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnnotationList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Annotation>,
}
impl From<Vec<Annotation>> for AnnotationList {
    fn from(v: Vec<Annotation>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Annotation> for AnnotationList {
    fn from_iter<I: IntoIterator<Item = Annotation>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Annotation>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlAnnotationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Annotation>,
}

impl From<Vec<Annotation>> for XmlAnnotationList {
    fn from(v: Vec<Annotation>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Annotation> for XmlAnnotationList {
    fn from_iter<I: IntoIterator<Item = Annotation>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Annotation")]
pub struct Annotation {
    #[serde(rename = "AnnotationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_name: Option<String>,
    #[serde(rename = "RemediationLink")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation_link: Option<String>,
    #[serde(rename = "RemediationMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation_message: Option<String>,
    #[serde(rename = "SeverityLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity_level: Option<String>,
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
#[serde(rename = "HookTarget")]
pub struct HookTarget {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "TargetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    #[serde(rename = "TargetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    #[serde(rename = "TargetTypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateStackSetInput")]
pub struct CreateStackSetInput {
    #[serde(rename = "AdministrationRoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administration_role_a_r_n: Option<String>,
    #[serde(rename = "AutoDeployment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_deployment: Option<AutoDeployment>,
    #[serde(rename = "CallAs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_as: Option<String>,
    #[serde(rename = "Capabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Capabilities>,
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ExecutionRoleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_name: Option<String>,
    #[serde(rename = "ManagedExecution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_execution: Option<ManagedExecution>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Parameters>,
    #[serde(rename = "PermissionModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_model: Option<String>,
    #[serde(rename = "StackId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
    #[serde(rename = "StackSetName")]
    #[serde(default)]
    pub stack_set_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[serde(rename = "TemplateBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_body: Option<String>,
    #[serde(rename = "TemplateURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_u_r_l: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeChangeSetResult")]
pub struct DescribeChangeSetOutput {
    #[serde(rename = "Capabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Capabilities>,
    #[serde(rename = "ChangeSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_set_id: Option<String>,
    #[serde(rename = "ChangeSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_set_name: Option<String>,
    #[serde(rename = "Changes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changes: Option<Changes>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "DeploymentMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_mode: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ExecutionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_status: Option<String>,
    #[serde(rename = "ImportExistingResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_existing_resources: Option<bool>,
    #[serde(rename = "IncludeNestedStacks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_nested_stacks: Option<bool>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "NotificationARNs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_a_r_ns: Option<NotificationARNs>,
    #[serde(rename = "OnStackFailure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_stack_failure: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Parameters>,
    #[serde(rename = "ParentChangeSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_change_set_id: Option<String>,
    #[serde(rename = "RollbackConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_configuration: Option<RollbackConfiguration>,
    #[serde(rename = "RootChangeSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_change_set_id: Option<String>,
    #[serde(rename = "StackDriftStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_drift_status: Option<String>,
    #[serde(rename = "StackId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
    #[serde(rename = "StackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Changes {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(rename = "HookInvocationCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_invocation_count: Option<i32>,
    #[serde(rename = "ResourceChange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_change: Option<ResourceChange>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResourceChange")]
pub struct ResourceChange {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "AfterContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_context: Option<String>,
    #[serde(rename = "BeforeContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_context: Option<String>,
    #[serde(rename = "ChangeSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_set_id: Option<String>,
    #[serde(rename = "Details")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<ResourceChangeDetails>,
    #[serde(rename = "LogicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_resource_id: Option<String>,
    #[serde(rename = "ModuleInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_info: Option<ModuleInfo>,
    #[serde(rename = "PhysicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_resource_id: Option<String>,
    #[serde(rename = "PolicyAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_action: Option<String>,
    #[serde(rename = "PreviousDeploymentContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_deployment_context: Option<String>,
    #[serde(rename = "Replacement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement: Option<String>,
    #[serde(rename = "ResourceDriftIgnoredAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_drift_ignored_attributes: Option<ResourceDriftIgnoredAttributes>,
    #[serde(rename = "ResourceDriftStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_drift_status: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "Scope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Scope>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceChangeDetails {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ResourceChangeDetail>,
}
impl From<Vec<ResourceChangeDetail>> for ResourceChangeDetails {
    fn from(v: Vec<ResourceChangeDetail>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ResourceChangeDetail> for ResourceChangeDetails {
    fn from_iter<I: IntoIterator<Item = ResourceChangeDetail>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ResourceChangeDetail>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlResourceChangeDetailList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ResourceChangeDetail>,
}

impl From<Vec<ResourceChangeDetail>> for XmlResourceChangeDetailList {
    fn from(v: Vec<ResourceChangeDetail>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ResourceChangeDetail> for XmlResourceChangeDetailList {
    fn from_iter<I: IntoIterator<Item = ResourceChangeDetail>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResourceChangeDetail")]
pub struct ResourceChangeDetail {
    #[serde(rename = "CausingEntity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub causing_entity: Option<String>,
    #[serde(rename = "ChangeSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_source: Option<String>,
    #[serde(rename = "Evaluation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation: Option<String>,
    #[serde(rename = "Target")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<ResourceTargetDefinition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResourceTargetDefinition")]
pub struct ResourceTargetDefinition {
    #[serde(rename = "AfterValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_value: Option<String>,
    #[serde(rename = "AfterValueFrom")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_value_from: Option<String>,
    #[serde(rename = "Attribute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<String>,
    #[serde(rename = "AttributeChangeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_change_type: Option<String>,
    #[serde(rename = "BeforeValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_value: Option<String>,
    #[serde(rename = "BeforeValueFrom")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_value_from: Option<String>,
    #[serde(rename = "Drift")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drift: Option<LiveResourceDrift>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "RequiresRecreation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_recreation: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LiveResourceDrift")]
pub struct LiveResourceDrift {
    #[serde(rename = "ActualValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_value: Option<String>,
    #[serde(rename = "DriftDetectionTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drift_detection_timestamp: Option<String>,
    #[serde(rename = "PreviousValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceDriftIgnoredAttributes {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ResourceDriftIgnoredAttribute>,
}
impl From<Vec<ResourceDriftIgnoredAttribute>> for ResourceDriftIgnoredAttributes {
    fn from(v: Vec<ResourceDriftIgnoredAttribute>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ResourceDriftIgnoredAttribute> for ResourceDriftIgnoredAttributes {
    fn from_iter<I: IntoIterator<Item = ResourceDriftIgnoredAttribute>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ResourceDriftIgnoredAttribute>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlResourceDriftIgnoredAttributeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ResourceDriftIgnoredAttribute>,
}

impl From<Vec<ResourceDriftIgnoredAttribute>> for XmlResourceDriftIgnoredAttributeList {
    fn from(v: Vec<ResourceDriftIgnoredAttribute>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ResourceDriftIgnoredAttribute> for XmlResourceDriftIgnoredAttributeList {
    fn from_iter<I: IntoIterator<Item = ResourceDriftIgnoredAttribute>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResourceDriftIgnoredAttribute")]
pub struct ResourceDriftIgnoredAttribute {
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "Reason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Scope {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for Scope {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for Scope {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeGeneratedTemplateInput")]
pub struct DescribeGeneratedTemplateInput {
    #[serde(rename = "GeneratedTemplateName")]
    #[serde(default)]
    pub generated_template_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeChangeSetHooksInput")]
pub struct DescribeChangeSetHooksInput {
    #[serde(rename = "ChangeSetName")]
    #[serde(default)]
    pub change_set_name: String,
    #[serde(rename = "LogicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_resource_id: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeStackDriftDetectionStatusInput")]
pub struct DescribeStackDriftDetectionStatusInput {
    #[serde(rename = "StackDriftDetectionId")]
    #[serde(default)]
    pub stack_drift_detection_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PublishTypeResult")]
pub struct PublishTypeOutput {
    #[serde(rename = "PublicTypeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_type_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateStackSetResult")]
pub struct CreateStackSetOutput {
    #[serde(rename = "StackSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_set_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListStackResourcesResult")]
pub struct ListStackResourcesOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StackResourceSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_resource_summaries: Option<StackResourceSummaries>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StackResourceSummaries {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<StackResourceSummary>,
}
impl From<Vec<StackResourceSummary>> for StackResourceSummaries {
    fn from(v: Vec<StackResourceSummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<StackResourceSummary> for StackResourceSummaries {
    fn from_iter<I: IntoIterator<Item = StackResourceSummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<StackResourceSummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlStackResourceSummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<StackResourceSummary>,
}

impl From<Vec<StackResourceSummary>> for XmlStackResourceSummaryList {
    fn from(v: Vec<StackResourceSummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<StackResourceSummary> for XmlStackResourceSummaryList {
    fn from_iter<I: IntoIterator<Item = StackResourceSummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StackResourceSummary")]
pub struct StackResourceSummary {
    #[serde(rename = "DriftInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drift_information: Option<StackResourceDriftInformationSummary>,
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<String>,
    #[serde(rename = "LogicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_resource_id: Option<String>,
    #[serde(rename = "ModuleInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_info: Option<ModuleInfo>,
    #[serde(rename = "PhysicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_resource_id: Option<String>,
    #[serde(rename = "ResourceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_status: Option<String>,
    #[serde(rename = "ResourceStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_status_reason: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StackResourceDriftInformationSummary")]
pub struct StackResourceDriftInformationSummary {
    #[serde(rename = "LastCheckTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_check_timestamp: Option<String>,
    #[serde(rename = "StackResourceDriftStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_resource_drift_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEventsInput")]
pub struct DescribeEventsInput {
    #[serde(rename = "ChangeSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_set_name: Option<String>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<EventFilter>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(rename = "StackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EventFilter")]
pub struct EventFilter {
    #[serde(rename = "FailedEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_events: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeStackResourceDriftsResult")]
pub struct DescribeStackResourceDriftsOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StackResourceDrifts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_resource_drifts: Option<StackResourceDrifts>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StackResourceDrifts {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<StackResourceDrift>,
}
impl From<Vec<StackResourceDrift>> for StackResourceDrifts {
    fn from(v: Vec<StackResourceDrift>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<StackResourceDrift> for StackResourceDrifts {
    fn from_iter<I: IntoIterator<Item = StackResourceDrift>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<StackResourceDrift>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlStackResourceDriftList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<StackResourceDrift>,
}

impl From<Vec<StackResourceDrift>> for XmlStackResourceDriftList {
    fn from(v: Vec<StackResourceDrift>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<StackResourceDrift> for XmlStackResourceDriftList {
    fn from_iter<I: IntoIterator<Item = StackResourceDrift>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeStackResourcesInput")]
pub struct DescribeStackResourcesInput {
    #[serde(rename = "LogicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_resource_id: Option<String>,
    #[serde(rename = "PhysicalResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_resource_id: Option<String>,
    #[serde(rename = "StackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeStackRefactorInput")]
pub struct DescribeStackRefactorInput {
    #[serde(rename = "StackRefactorId")]
    #[serde(default)]
    pub stack_refactor_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListStackSetsInput")]
pub struct ListStackSetsInput {
    #[serde(rename = "CallAs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_as: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateStackInput")]
pub struct CreateStackInput {
    #[serde(rename = "Capabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Capabilities>,
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "DisableRollback")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_rollback: Option<bool>,
    #[serde(rename = "EnableTerminationProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_termination_protection: Option<bool>,
    #[serde(rename = "NotificationARNs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_a_r_ns: Option<NotificationARNs>,
    #[serde(rename = "OnFailure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Parameters>,
    #[serde(rename = "ResourceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<ResourceTypes>,
    #[serde(rename = "RetainExceptOnCreate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_except_on_create: Option<bool>,
    #[serde(rename = "RoleARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_a_r_n: Option<String>,
    #[serde(rename = "RollbackConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_configuration: Option<RollbackConfiguration>,
    #[serde(rename = "StackName")]
    #[serde(default)]
    pub stack_name: String,
    #[serde(rename = "StackPolicyBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_policy_body: Option<String>,
    #[serde(rename = "StackPolicyURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_policy_u_r_l: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[serde(rename = "TemplateBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_body: Option<String>,
    #[serde(rename = "TemplateURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_u_r_l: Option<String>,
    #[serde(rename = "TimeoutInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_minutes: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListExportsResult")]
pub struct ListExportsOutput {
    #[serde(rename = "Exports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exports: Option<Exports>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Exports {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Export>,
}
impl From<Vec<Export>> for Exports {
    fn from(v: Vec<Export>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Export> for Exports {
    fn from_iter<I: IntoIterator<Item = Export>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Export>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlExportList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Export>,
}

impl From<Vec<Export>> for XmlExportList {
    fn from(v: Vec<Export>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Export> for XmlExportList {
    fn from_iter<I: IntoIterator<Item = Export>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Export")]
pub struct Export {
    #[serde(rename = "ExportingStackId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exporting_stack_id: Option<String>,
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
#[serde(rename = "UpdateStackInstancesInput")]
pub struct UpdateStackInstancesInput {
    #[serde(rename = "Accounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<AccountList>,
    #[serde(rename = "CallAs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_as: Option<String>,
    #[serde(rename = "DeploymentTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_targets: Option<DeploymentTargets>,
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(rename = "OperationPreferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_preferences: Option<StackSetOperationPreferences>,
    #[serde(rename = "ParameterOverrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_overrides: Option<Parameters>,
    #[serde(rename = "Regions")]
    #[serde(default)]
    pub regions: RegionList,
    #[serde(rename = "StackSetName")]
    #[serde(default)]
    pub stack_set_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DetectStackDriftResult")]
pub struct DetectStackDriftOutput {
    #[serde(rename = "StackDriftDetectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_drift_detection_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ExecuteStackRefactorInput")]
pub struct ExecuteStackRefactorInput {
    #[serde(rename = "StackRefactorId")]
    #[serde(default)]
    pub stack_refactor_id: String,
}
