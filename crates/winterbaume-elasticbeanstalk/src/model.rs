//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-elasticbeanstalk

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RetrieveEnvironmentInfoResult")]
pub struct RetrieveEnvironmentInfoResultMessage {
    #[serde(rename = "EnvironmentInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_info: Option<EnvironmentInfoDescriptionList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnvironmentInfoDescriptionList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<EnvironmentInfoDescription>,
}
impl From<Vec<EnvironmentInfoDescription>> for EnvironmentInfoDescriptionList {
    fn from(v: Vec<EnvironmentInfoDescription>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<EnvironmentInfoDescription> for EnvironmentInfoDescriptionList {
    fn from_iter<I: IntoIterator<Item = EnvironmentInfoDescription>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<EnvironmentInfoDescription>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlEnvironmentInfoDescriptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<EnvironmentInfoDescription>,
}

impl From<Vec<EnvironmentInfoDescription>> for XmlEnvironmentInfoDescriptionList {
    fn from(v: Vec<EnvironmentInfoDescription>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<EnvironmentInfoDescription> for XmlEnvironmentInfoDescriptionList {
    fn from_iter<I: IntoIterator<Item = EnvironmentInfoDescription>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EnvironmentInfoDescription")]
pub struct EnvironmentInfoDescription {
    #[serde(rename = "Ec2InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_instance_id: Option<String>,
    #[serde(rename = "InfoType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info_type: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "SampleTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_timestamp: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListPlatformVersionsResult")]
pub struct ListPlatformVersionsResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PlatformSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_summary_list: Option<PlatformSummaryList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PlatformSummaryList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PlatformSummary>,
}
impl From<Vec<PlatformSummary>> for PlatformSummaryList {
    fn from(v: Vec<PlatformSummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<PlatformSummary> for PlatformSummaryList {
    fn from_iter<I: IntoIterator<Item = PlatformSummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<PlatformSummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlPlatformSummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<PlatformSummary>,
}

impl From<Vec<PlatformSummary>> for XmlPlatformSummaryList {
    fn from(v: Vec<PlatformSummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<PlatformSummary> for XmlPlatformSummaryList {
    fn from_iter<I: IntoIterator<Item = PlatformSummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PlatformSummary")]
pub struct PlatformSummary {
    #[serde(rename = "OperatingSystemName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system_name: Option<String>,
    #[serde(rename = "OperatingSystemVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system_version: Option<String>,
    #[serde(rename = "PlatformArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_arn: Option<String>,
    #[serde(rename = "PlatformBranchLifecycleState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_branch_lifecycle_state: Option<String>,
    #[serde(rename = "PlatformBranchName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_branch_name: Option<String>,
    #[serde(rename = "PlatformCategory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_category: Option<String>,
    #[serde(rename = "PlatformLifecycleState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_lifecycle_state: Option<String>,
    #[serde(rename = "PlatformOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_owner: Option<String>,
    #[serde(rename = "PlatformStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_status: Option<String>,
    #[serde(rename = "PlatformVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    #[serde(rename = "SupportedAddonList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_addon_list: Option<SupportedAddonList>,
    #[serde(rename = "SupportedTierList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_tier_list: Option<SupportedTierList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SupportedAddonList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for SupportedAddonList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for SupportedAddonList {
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
pub struct SupportedTierList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for SupportedTierList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for SupportedTierList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RetrieveEnvironmentInfoMessage")]
pub struct RetrieveEnvironmentInfoMessage {
    #[serde(rename = "EnvironmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    #[serde(rename = "EnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    #[serde(rename = "InfoType")]
    #[serde(default)]
    pub info_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEnvironmentManagedActionsResult")]
pub struct DescribeEnvironmentManagedActionsResult {
    #[serde(rename = "ManagedActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_actions: Option<ManagedActions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedActions {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ManagedAction>,
}
impl From<Vec<ManagedAction>> for ManagedActions {
    fn from(v: Vec<ManagedAction>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ManagedAction> for ManagedActions {
    fn from_iter<I: IntoIterator<Item = ManagedAction>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ManagedAction>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlManagedActionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ManagedAction>,
}

impl From<Vec<ManagedAction>> for XmlManagedActionList {
    fn from(v: Vec<ManagedAction>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ManagedAction> for XmlManagedActionList {
    fn from_iter<I: IntoIterator<Item = ManagedAction>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ManagedAction")]
pub struct ManagedAction {
    #[serde(rename = "ActionDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_description: Option<String>,
    #[serde(rename = "ActionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(rename = "ActionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "WindowStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEventsMessage")]
pub struct DescribeEventsMessage {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "EnvironmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    #[serde(rename = "EnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PlatformArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_arn: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Severity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    #[serde(rename = "VersionLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_label: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RestartAppServerMessage")]
pub struct RestartAppServerMessage {
    #[serde(rename = "EnvironmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    #[serde(rename = "EnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreatePlatformVersionRequest")]
pub struct CreatePlatformVersionRequest {
    #[serde(rename = "EnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    #[serde(rename = "OptionSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_settings: Option<ConfigurationOptionSettingsList>,
    #[serde(rename = "PlatformDefinitionBundle")]
    #[serde(default)]
    pub platform_definition_bundle: S3Location,
    #[serde(rename = "PlatformName")]
    #[serde(default)]
    pub platform_name: String,
    #[serde(rename = "PlatformVersion")]
    #[serde(default)]
    pub platform_version: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigurationOptionSettingsList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ConfigurationOptionSetting>,
}
impl From<Vec<ConfigurationOptionSetting>> for ConfigurationOptionSettingsList {
    fn from(v: Vec<ConfigurationOptionSetting>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ConfigurationOptionSetting> for ConfigurationOptionSettingsList {
    fn from_iter<I: IntoIterator<Item = ConfigurationOptionSetting>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ConfigurationOptionSetting>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlConfigurationOptionSettingList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ConfigurationOptionSetting>,
}

impl From<Vec<ConfigurationOptionSetting>> for XmlConfigurationOptionSettingList {
    fn from(v: Vec<ConfigurationOptionSetting>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ConfigurationOptionSetting> for XmlConfigurationOptionSettingList {
    fn from_iter<I: IntoIterator<Item = ConfigurationOptionSetting>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ConfigurationOptionSetting")]
pub struct ConfigurationOptionSetting {
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "OptionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_name: Option<String>,
    #[serde(rename = "ResourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "S3Location")]
pub struct S3Location {
    #[serde(rename = "S3Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket: Option<String>,
    #[serde(rename = "S3Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key: Option<String>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateApplicationMessage")]
pub struct UpdateApplicationMessage {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateApplicationVersionMessage")]
pub struct CreateApplicationVersionMessage {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "AutoCreateApplication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_create_application: Option<bool>,
    #[serde(rename = "BuildConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_configuration: Option<BuildConfiguration>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Process")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process: Option<bool>,
    #[serde(rename = "SourceBuildInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_build_information: Option<SourceBuildInformation>,
    #[serde(rename = "SourceBundle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_bundle: Option<S3Location>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[serde(rename = "VersionLabel")]
    #[serde(default)]
    pub version_label: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "BuildConfiguration")]
pub struct BuildConfiguration {
    #[serde(rename = "ArtifactName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact_name: Option<String>,
    #[serde(rename = "CodeBuildServiceRole")]
    #[serde(default)]
    pub code_build_service_role: String,
    #[serde(rename = "ComputeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_type: Option<String>,
    #[serde(rename = "Image")]
    #[serde(default)]
    pub image: String,
    #[serde(rename = "TimeoutInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_minutes: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SourceBuildInformation")]
pub struct SourceBuildInformation {
    #[serde(rename = "SourceLocation")]
    #[serde(default)]
    pub source_location: String,
    #[serde(rename = "SourceRepository")]
    #[serde(default)]
    pub source_repository: String,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    pub source_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEnvironmentsResult")]
pub struct EnvironmentDescriptionsMessage {
    #[serde(rename = "Environments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environments: Option<EnvironmentDescriptionsList>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnvironmentDescriptionsList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<EnvironmentDescription>,
}
impl From<Vec<EnvironmentDescription>> for EnvironmentDescriptionsList {
    fn from(v: Vec<EnvironmentDescription>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<EnvironmentDescription> for EnvironmentDescriptionsList {
    fn from_iter<I: IntoIterator<Item = EnvironmentDescription>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<EnvironmentDescription>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlEnvironmentDescriptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<EnvironmentDescription>,
}

impl From<Vec<EnvironmentDescription>> for XmlEnvironmentDescriptionList {
    fn from(v: Vec<EnvironmentDescription>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<EnvironmentDescription> for XmlEnvironmentDescriptionList {
    fn from_iter<I: IntoIterator<Item = EnvironmentDescription>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateEnvironmentResult")]
pub struct EnvironmentDescription {
    #[serde(rename = "AbortableOperationInProgress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abortable_operation_in_progress: Option<bool>,
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(rename = "CNAME")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_n_a_m_e: Option<String>,
    #[serde(rename = "DateCreated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(rename = "DateUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EndpointURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_u_r_l: Option<String>,
    #[serde(rename = "EnvironmentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_arn: Option<String>,
    #[serde(rename = "EnvironmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    #[serde(rename = "EnvironmentLinks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_links: Option<EnvironmentLinks>,
    #[serde(rename = "EnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    #[serde(rename = "Health")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<String>,
    #[serde(rename = "HealthStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_status: Option<String>,
    #[serde(rename = "OperationsRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations_role: Option<String>,
    #[serde(rename = "PlatformArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_arn: Option<String>,
    #[serde(rename = "Resources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<EnvironmentResourcesDescription>,
    #[serde(rename = "SolutionStackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_stack_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    #[serde(rename = "Tier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<EnvironmentTier>,
    #[serde(rename = "VersionLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_label: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnvironmentLinks {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<EnvironmentLink>,
}
impl From<Vec<EnvironmentLink>> for EnvironmentLinks {
    fn from(v: Vec<EnvironmentLink>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<EnvironmentLink> for EnvironmentLinks {
    fn from_iter<I: IntoIterator<Item = EnvironmentLink>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<EnvironmentLink>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlEnvironmentLinkList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<EnvironmentLink>,
}

impl From<Vec<EnvironmentLink>> for XmlEnvironmentLinkList {
    fn from(v: Vec<EnvironmentLink>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<EnvironmentLink> for XmlEnvironmentLinkList {
    fn from_iter<I: IntoIterator<Item = EnvironmentLink>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EnvironmentLink")]
pub struct EnvironmentLink {
    #[serde(rename = "EnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    #[serde(rename = "LinkName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EnvironmentResourcesDescription")]
pub struct EnvironmentResourcesDescription {
    #[serde(rename = "LoadBalancer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer: Option<LoadBalancerDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LoadBalancerDescription")]
pub struct LoadBalancerDescription {
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "Listeners")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listeners: Option<LoadBalancerListenersDescription>,
    #[serde(rename = "LoadBalancerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoadBalancerListenersDescription {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Listener>,
}
impl From<Vec<Listener>> for LoadBalancerListenersDescription {
    fn from(v: Vec<Listener>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Listener> for LoadBalancerListenersDescription {
    fn from_iter<I: IntoIterator<Item = Listener>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Listener>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlListenerList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Listener>,
}

impl From<Vec<Listener>> for XmlListenerList {
    fn from(v: Vec<Listener>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Listener> for XmlListenerList {
    fn from_iter<I: IntoIterator<Item = Listener>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Listener")]
pub struct Listener {
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "Protocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EnvironmentTier")]
pub struct EnvironmentTier {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreatePlatformVersionResult")]
pub struct CreatePlatformVersionResult {
    #[serde(rename = "Builder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builder: Option<Builder>,
    #[serde(rename = "PlatformSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_summary: Option<PlatformSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Builder")]
pub struct Builder {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEnvironmentHealthRequest")]
pub struct DescribeEnvironmentHealthRequest {
    #[serde(rename = "AttributeNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_names: Option<EnvironmentHealthAttributes>,
    #[serde(rename = "EnvironmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    #[serde(rename = "EnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnvironmentHealthAttributes {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for EnvironmentHealthAttributes {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for EnvironmentHealthAttributes {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEnvironmentResourcesMessage")]
pub struct DescribeEnvironmentResourcesMessage {
    #[serde(rename = "EnvironmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    #[serde(rename = "EnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTagsForResourceMessage")]
pub struct ListTagsForResourceMessage {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteApplicationVersionMessage")]
pub struct DeleteApplicationVersionMessage {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "DeleteSourceBundle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_source_bundle: Option<bool>,
    #[serde(rename = "VersionLabel")]
    #[serde(default)]
    pub version_label: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateApplicationVersionMessage")]
pub struct UpdateApplicationVersionMessage {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "VersionLabel")]
    #[serde(default)]
    pub version_label: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateApplicationVersionResult")]
pub struct ApplicationVersionDescriptionMessage {
    #[serde(rename = "ApplicationVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version: Option<ApplicationVersionDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ApplicationVersionDescription")]
pub struct ApplicationVersionDescription {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(rename = "ApplicationVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_arn: Option<String>,
    #[serde(rename = "BuildArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_arn: Option<String>,
    #[serde(rename = "DateCreated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(rename = "DateUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "SourceBuildInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_build_information: Option<SourceBuildInformation>,
    #[serde(rename = "SourceBundle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_bundle: Option<S3Location>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "VersionLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_label: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEnvironmentsMessage")]
pub struct DescribeEnvironmentsMessage {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(rename = "EnvironmentIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_ids: Option<EnvironmentIdList>,
    #[serde(rename = "EnvironmentNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_names: Option<EnvironmentNamesList>,
    #[serde(rename = "IncludeDeleted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_deleted: Option<bool>,
    #[serde(rename = "IncludedDeletedBackTo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_deleted_back_to: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "VersionLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_label: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnvironmentIdList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for EnvironmentIdList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for EnvironmentIdList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnvironmentNamesList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for EnvironmentNamesList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for EnvironmentNamesList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateApplicationResourceLifecycleResult")]
pub struct ApplicationResourceLifecycleDescriptionMessage {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(rename = "ResourceLifecycleConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_lifecycle_config: Option<ApplicationResourceLifecycleConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ApplicationResourceLifecycleConfig")]
pub struct ApplicationResourceLifecycleConfig {
    #[serde(rename = "ServiceRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    #[serde(rename = "VersionLifecycleConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_lifecycle_config: Option<ApplicationVersionLifecycleConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ApplicationVersionLifecycleConfig")]
pub struct ApplicationVersionLifecycleConfig {
    #[serde(rename = "MaxAgeRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age_rule: Option<MaxAgeRule>,
    #[serde(rename = "MaxCountRule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_count_rule: Option<MaxCountRule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MaxAgeRule")]
pub struct MaxAgeRule {
    #[serde(rename = "DeleteSourceFromS3")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_source_from_s3: Option<bool>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "MaxAgeInDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age_in_days: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MaxCountRule")]
pub struct MaxCountRule {
    #[serde(rename = "DeleteSourceFromS3")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_source_from_s3: Option<bool>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "MaxCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateEnvironmentMessage")]
pub struct CreateEnvironmentMessage {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "CNAMEPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_n_a_m_e_prefix: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "OperationsRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations_role: Option<String>,
    #[serde(rename = "OptionSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_settings: Option<ConfigurationOptionSettingsList>,
    #[serde(rename = "OptionsToRemove")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options_to_remove: Option<OptionsSpecifierList>,
    #[serde(rename = "PlatformArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_arn: Option<String>,
    #[serde(rename = "SolutionStackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_stack_name: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    #[serde(rename = "Tier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<EnvironmentTier>,
    #[serde(rename = "VersionLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_label: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OptionsSpecifierList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<OptionSpecification>,
}
impl From<Vec<OptionSpecification>> for OptionsSpecifierList {
    fn from(v: Vec<OptionSpecification>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<OptionSpecification> for OptionsSpecifierList {
    fn from_iter<I: IntoIterator<Item = OptionSpecification>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<OptionSpecification>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlOptionSpecificationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<OptionSpecification>,
}

impl From<Vec<OptionSpecification>> for XmlOptionSpecificationList {
    fn from(v: Vec<OptionSpecification>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<OptionSpecification> for XmlOptionSpecificationList {
    fn from_iter<I: IntoIterator<Item = OptionSpecification>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OptionSpecification")]
pub struct OptionSpecification {
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "OptionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_name: Option<String>,
    #[serde(rename = "ResourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListPlatformVersionsRequest")]
pub struct ListPlatformVersionsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<PlatformFilters>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PlatformFilters {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PlatformFilter>,
}
impl From<Vec<PlatformFilter>> for PlatformFilters {
    fn from(v: Vec<PlatformFilter>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<PlatformFilter> for PlatformFilters {
    fn from_iter<I: IntoIterator<Item = PlatformFilter>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<PlatformFilter>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlPlatformFilterList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<PlatformFilter>,
}

impl From<Vec<PlatformFilter>> for XmlPlatformFilterList {
    fn from(v: Vec<PlatformFilter>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<PlatformFilter> for XmlPlatformFilterList {
    fn from_iter<I: IntoIterator<Item = PlatformFilter>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PlatformFilter")]
pub struct PlatformFilter {
    #[serde(rename = "Operator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<PlatformFilterValueList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PlatformFilterValueList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for PlatformFilterValueList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for PlatformFilterValueList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeConfigurationOptionsMessage")]
pub struct DescribeConfigurationOptionsMessage {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(rename = "EnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<OptionsSpecifierList>,
    #[serde(rename = "PlatformArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_arn: Option<String>,
    #[serde(rename = "SolutionStackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_stack_name: Option<String>,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEnvironmentHealthResult")]
pub struct DescribeEnvironmentHealthResult {
    #[serde(rename = "ApplicationMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_metrics: Option<ApplicationMetrics>,
    #[serde(rename = "Causes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub causes: Option<Causes>,
    #[serde(rename = "Color")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "EnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    #[serde(rename = "HealthStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_status: Option<String>,
    #[serde(rename = "InstancesHealth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_health: Option<InstanceHealthSummary>,
    #[serde(rename = "RefreshedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refreshed_at: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ApplicationMetrics")]
pub struct ApplicationMetrics {
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "Latency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency: Option<Latency>,
    #[serde(rename = "RequestCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_count: Option<i32>,
    #[serde(rename = "StatusCodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_codes: Option<StatusCodes>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Latency")]
pub struct Latency {
    #[serde(rename = "P10")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p10: Option<f64>,
    #[serde(rename = "P50")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p50: Option<f64>,
    #[serde(rename = "P75")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p75: Option<f64>,
    #[serde(rename = "P85")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p85: Option<f64>,
    #[serde(rename = "P90")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p90: Option<f64>,
    #[serde(rename = "P95")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p95: Option<f64>,
    #[serde(rename = "P99")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p99: Option<f64>,
    #[serde(rename = "P999")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p999: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StatusCodes")]
pub struct StatusCodes {
    #[serde(rename = "Status2xx")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status2xx: Option<i32>,
    #[serde(rename = "Status3xx")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status3xx: Option<i32>,
    #[serde(rename = "Status4xx")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status4xx: Option<i32>,
    #[serde(rename = "Status5xx")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status5xx: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Causes {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for Causes {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for Causes {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "InstanceHealthSummary")]
pub struct InstanceHealthSummary {
    #[serde(rename = "Degraded")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub degraded: Option<i32>,
    #[serde(rename = "Info")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<i32>,
    #[serde(rename = "NoData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_data: Option<i32>,
    #[serde(rename = "Ok")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ok: Option<i32>,
    #[serde(rename = "Pending")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<i32>,
    #[serde(rename = "Severe")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severe: Option<i32>,
    #[serde(rename = "Unknown")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown: Option<i32>,
    #[serde(rename = "Warning")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeApplicationVersionsMessage")]
pub struct DescribeApplicationVersionsMessage {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "VersionLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_labels: Option<VersionLabelsList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VersionLabelsList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for VersionLabelsList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for VersionLabelsList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RequestEnvironmentInfoMessage")]
pub struct RequestEnvironmentInfoMessage {
    #[serde(rename = "EnvironmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    #[serde(rename = "EnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    #[serde(rename = "InfoType")]
    #[serde(default)]
    pub info_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ComposeEnvironmentsMessage")]
pub struct ComposeEnvironmentsMessage {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "VersionLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_labels: Option<VersionLabels>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VersionLabels {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for VersionLabels {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for VersionLabels {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateApplicationResult")]
pub struct ApplicationDescriptionMessage {
    #[serde(rename = "Application")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<ApplicationDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ApplicationDescription")]
pub struct ApplicationDescription {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(rename = "ConfigurationTemplates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_templates: Option<ConfigurationTemplateNamesList>,
    #[serde(rename = "DateCreated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(rename = "DateUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ResourceLifecycleConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_lifecycle_config: Option<ApplicationResourceLifecycleConfig>,
    #[serde(rename = "Versions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<VersionLabelsList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigurationTemplateNamesList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ConfigurationTemplateNamesList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ConfigurationTemplateNamesList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AbortEnvironmentUpdateMessage")]
pub struct AbortEnvironmentUpdateMessage {
    #[serde(rename = "EnvironmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    #[serde(rename = "EnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteApplicationMessage")]
pub struct DeleteApplicationMessage {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "TerminateEnvByForce")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminate_env_by_force: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeletePlatformVersionRequest")]
pub struct DeletePlatformVersionRequest {
    #[serde(rename = "PlatformArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeApplicationsMessage")]
pub struct DescribeApplicationsMessage {
    #[serde(rename = "ApplicationNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_names: Option<ApplicationNamesList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationNamesList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ApplicationNamesList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ApplicationNamesList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeConfigurationSettingsResult")]
pub struct ConfigurationSettingsDescriptions {
    #[serde(rename = "ConfigurationSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_settings: Option<ConfigurationSettingsDescriptionList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigurationSettingsDescriptionList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ConfigurationSettingsDescription>,
}
impl From<Vec<ConfigurationSettingsDescription>> for ConfigurationSettingsDescriptionList {
    fn from(v: Vec<ConfigurationSettingsDescription>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ConfigurationSettingsDescription> for ConfigurationSettingsDescriptionList {
    fn from_iter<I: IntoIterator<Item = ConfigurationSettingsDescription>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ConfigurationSettingsDescription>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlConfigurationSettingsDescriptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ConfigurationSettingsDescription>,
}

impl From<Vec<ConfigurationSettingsDescription>> for XmlConfigurationSettingsDescriptionList {
    fn from(v: Vec<ConfigurationSettingsDescription>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ConfigurationSettingsDescription> for XmlConfigurationSettingsDescriptionList {
    fn from_iter<I: IntoIterator<Item = ConfigurationSettingsDescription>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateConfigurationTemplateResult")]
pub struct ConfigurationSettingsDescription {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(rename = "DateCreated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(rename = "DateUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    #[serde(rename = "DeploymentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_status: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    #[serde(rename = "OptionSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_settings: Option<ConfigurationOptionSettingsList>,
    #[serde(rename = "PlatformArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_arn: Option<String>,
    #[serde(rename = "SolutionStackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_stack_name: Option<String>,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEnvironmentManagedActionHistoryRequest")]
pub struct DescribeEnvironmentManagedActionHistoryRequest {
    #[serde(rename = "EnvironmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    #[serde(rename = "EnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEnvironmentManagedActionsRequest")]
pub struct DescribeEnvironmentManagedActionsRequest {
    #[serde(rename = "EnvironmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    #[serde(rename = "EnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribePlatformVersionRequest")]
pub struct DescribePlatformVersionRequest {
    #[serde(rename = "PlatformArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListPlatformBranchesResult")]
pub struct ListPlatformBranchesResult {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PlatformBranchSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_branch_summary_list: Option<PlatformBranchSummaryList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PlatformBranchSummaryList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PlatformBranchSummary>,
}
impl From<Vec<PlatformBranchSummary>> for PlatformBranchSummaryList {
    fn from(v: Vec<PlatformBranchSummary>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<PlatformBranchSummary> for PlatformBranchSummaryList {
    fn from_iter<I: IntoIterator<Item = PlatformBranchSummary>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<PlatformBranchSummary>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlPlatformBranchSummaryList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<PlatformBranchSummary>,
}

impl From<Vec<PlatformBranchSummary>> for XmlPlatformBranchSummaryList {
    fn from(v: Vec<PlatformBranchSummary>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<PlatformBranchSummary> for XmlPlatformBranchSummaryList {
    fn from_iter<I: IntoIterator<Item = PlatformBranchSummary>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PlatformBranchSummary")]
pub struct PlatformBranchSummary {
    #[serde(rename = "BranchName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    #[serde(rename = "BranchOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_order: Option<i32>,
    #[serde(rename = "LifecycleState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_state: Option<String>,
    #[serde(rename = "PlatformName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_name: Option<String>,
    #[serde(rename = "SupportedTierList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_tier_list: Option<SupportedTierList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListAvailableSolutionStacksResult")]
pub struct ListAvailableSolutionStacksResultMessage {
    #[serde(rename = "SolutionStackDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_stack_details: Option<AvailableSolutionStackDetailsList>,
    #[serde(rename = "SolutionStacks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_stacks: Option<AvailableSolutionStackNamesList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AvailableSolutionStackDetailsList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<SolutionStackDescription>,
}
impl From<Vec<SolutionStackDescription>> for AvailableSolutionStackDetailsList {
    fn from(v: Vec<SolutionStackDescription>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<SolutionStackDescription> for AvailableSolutionStackDetailsList {
    fn from_iter<I: IntoIterator<Item = SolutionStackDescription>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<SolutionStackDescription>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlSolutionStackDescriptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<SolutionStackDescription>,
}

impl From<Vec<SolutionStackDescription>> for XmlSolutionStackDescriptionList {
    fn from(v: Vec<SolutionStackDescription>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<SolutionStackDescription> for XmlSolutionStackDescriptionList {
    fn from_iter<I: IntoIterator<Item = SolutionStackDescription>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SolutionStackDescription")]
pub struct SolutionStackDescription {
    #[serde(rename = "PermittedFileTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permitted_file_types: Option<SolutionStackFileTypeList>,
    #[serde(rename = "SolutionStackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_stack_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SolutionStackFileTypeList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for SolutionStackFileTypeList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for SolutionStackFileTypeList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AvailableSolutionStackNamesList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for AvailableSolutionStackNamesList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for AvailableSolutionStackNamesList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribePlatformVersionResult")]
pub struct DescribePlatformVersionResult {
    #[serde(rename = "PlatformDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_description: Option<PlatformDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PlatformDescription")]
pub struct PlatformDescription {
    #[serde(rename = "CustomAmiList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_ami_list: Option<CustomAmiList>,
    #[serde(rename = "DateCreated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(rename = "DateUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Frameworks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frameworks: Option<PlatformFrameworks>,
    #[serde(rename = "Maintainer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintainer: Option<String>,
    #[serde(rename = "OperatingSystemName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system_name: Option<String>,
    #[serde(rename = "OperatingSystemVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system_version: Option<String>,
    #[serde(rename = "PlatformArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_arn: Option<String>,
    #[serde(rename = "PlatformBranchLifecycleState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_branch_lifecycle_state: Option<String>,
    #[serde(rename = "PlatformBranchName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_branch_name: Option<String>,
    #[serde(rename = "PlatformCategory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_category: Option<String>,
    #[serde(rename = "PlatformLifecycleState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_lifecycle_state: Option<String>,
    #[serde(rename = "PlatformName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_name: Option<String>,
    #[serde(rename = "PlatformOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_owner: Option<String>,
    #[serde(rename = "PlatformStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_status: Option<String>,
    #[serde(rename = "PlatformVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    #[serde(rename = "ProgrammingLanguages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub programming_languages: Option<PlatformProgrammingLanguages>,
    #[serde(rename = "SolutionStackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_stack_name: Option<String>,
    #[serde(rename = "SupportedAddonList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_addon_list: Option<SupportedAddonList>,
    #[serde(rename = "SupportedTierList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_tier_list: Option<SupportedTierList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomAmiList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<CustomAmi>,
}
impl From<Vec<CustomAmi>> for CustomAmiList {
    fn from(v: Vec<CustomAmi>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<CustomAmi> for CustomAmiList {
    fn from_iter<I: IntoIterator<Item = CustomAmi>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<CustomAmi>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlCustomAmiList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<CustomAmi>,
}

impl From<Vec<CustomAmi>> for XmlCustomAmiList {
    fn from(v: Vec<CustomAmi>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<CustomAmi> for XmlCustomAmiList {
    fn from_iter<I: IntoIterator<Item = CustomAmi>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CustomAmi")]
pub struct CustomAmi {
    #[serde(rename = "ImageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "VirtualizationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtualization_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PlatformFrameworks {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PlatformFramework>,
}
impl From<Vec<PlatformFramework>> for PlatformFrameworks {
    fn from(v: Vec<PlatformFramework>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<PlatformFramework> for PlatformFrameworks {
    fn from_iter<I: IntoIterator<Item = PlatformFramework>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<PlatformFramework>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlPlatformFrameworkList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<PlatformFramework>,
}

impl From<Vec<PlatformFramework>> for XmlPlatformFrameworkList {
    fn from(v: Vec<PlatformFramework>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<PlatformFramework> for XmlPlatformFrameworkList {
    fn from_iter<I: IntoIterator<Item = PlatformFramework>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PlatformFramework")]
pub struct PlatformFramework {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PlatformProgrammingLanguages {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PlatformProgrammingLanguage>,
}
impl From<Vec<PlatformProgrammingLanguage>> for PlatformProgrammingLanguages {
    fn from(v: Vec<PlatformProgrammingLanguage>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<PlatformProgrammingLanguage> for PlatformProgrammingLanguages {
    fn from_iter<I: IntoIterator<Item = PlatformProgrammingLanguage>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<PlatformProgrammingLanguage>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlPlatformProgrammingLanguageList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<PlatformProgrammingLanguage>,
}

impl From<Vec<PlatformProgrammingLanguage>> for XmlPlatformProgrammingLanguageList {
    fn from(v: Vec<PlatformProgrammingLanguage>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<PlatformProgrammingLanguage> for XmlPlatformProgrammingLanguageList {
    fn from_iter<I: IntoIterator<Item = PlatformProgrammingLanguage>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PlatformProgrammingLanguage")]
pub struct PlatformProgrammingLanguage {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DisassociateEnvironmentOperationsRoleMessage")]
pub struct DisassociateEnvironmentOperationsRoleMessage {
    #[serde(rename = "EnvironmentName")]
    #[serde(default)]
    pub environment_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEventsResult")]
pub struct EventDescriptionsMessage {
    #[serde(rename = "Events")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<EventDescriptionList>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventDescriptionList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<EventDescription>,
}
impl From<Vec<EventDescription>> for EventDescriptionList {
    fn from(v: Vec<EventDescription>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<EventDescription> for EventDescriptionList {
    fn from_iter<I: IntoIterator<Item = EventDescription>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<EventDescription>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlEventDescriptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<EventDescription>,
}

impl From<Vec<EventDescription>> for XmlEventDescriptionList {
    fn from(v: Vec<EventDescription>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<EventDescription> for XmlEventDescriptionList {
    fn from_iter<I: IntoIterator<Item = EventDescription>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EventDescription")]
pub struct EventDescription {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(rename = "EnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    #[serde(rename = "EventDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_date: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "PlatformArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_arn: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Severity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    #[serde(rename = "VersionLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_label: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ValidateConfigurationSettingsResult")]
pub struct ConfigurationSettingsValidationMessages {
    #[serde(rename = "Messages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<ValidationMessagesList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidationMessagesList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ValidationMessage>,
}
impl From<Vec<ValidationMessage>> for ValidationMessagesList {
    fn from(v: Vec<ValidationMessage>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ValidationMessage> for ValidationMessagesList {
    fn from_iter<I: IntoIterator<Item = ValidationMessage>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ValidationMessage>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlValidationMessageList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ValidationMessage>,
}

impl From<Vec<ValidationMessage>> for XmlValidationMessageList {
    fn from(v: Vec<ValidationMessage>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ValidationMessage> for XmlValidationMessageList {
    fn from_iter<I: IntoIterator<Item = ValidationMessage>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ValidationMessage")]
pub struct ValidationMessage {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "OptionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_name: Option<String>,
    #[serde(rename = "Severity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeApplicationsResult")]
pub struct ApplicationDescriptionsMessage {
    #[serde(rename = "Applications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applications: Option<ApplicationDescriptionList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationDescriptionList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ApplicationDescription>,
}
impl From<Vec<ApplicationDescription>> for ApplicationDescriptionList {
    fn from(v: Vec<ApplicationDescription>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ApplicationDescription> for ApplicationDescriptionList {
    fn from_iter<I: IntoIterator<Item = ApplicationDescription>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ApplicationDescription>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlApplicationDescriptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ApplicationDescription>,
}

impl From<Vec<ApplicationDescription>> for XmlApplicationDescriptionList {
    fn from(v: Vec<ApplicationDescription>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ApplicationDescription> for XmlApplicationDescriptionList {
    fn from_iter<I: IntoIterator<Item = ApplicationDescription>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteEnvironmentConfigurationMessage")]
pub struct DeleteEnvironmentConfigurationMessage {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "EnvironmentName")]
    #[serde(default)]
    pub environment_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeConfigurationOptionsResult")]
pub struct ConfigurationOptionsDescription {
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<ConfigurationOptionDescriptionsList>,
    #[serde(rename = "PlatformArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_arn: Option<String>,
    #[serde(rename = "SolutionStackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_stack_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigurationOptionDescriptionsList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ConfigurationOptionDescription>,
}
impl From<Vec<ConfigurationOptionDescription>> for ConfigurationOptionDescriptionsList {
    fn from(v: Vec<ConfigurationOptionDescription>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ConfigurationOptionDescription> for ConfigurationOptionDescriptionsList {
    fn from_iter<I: IntoIterator<Item = ConfigurationOptionDescription>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ConfigurationOptionDescription>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlConfigurationOptionDescriptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ConfigurationOptionDescription>,
}

impl From<Vec<ConfigurationOptionDescription>> for XmlConfigurationOptionDescriptionList {
    fn from(v: Vec<ConfigurationOptionDescription>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ConfigurationOptionDescription> for XmlConfigurationOptionDescriptionList {
    fn from_iter<I: IntoIterator<Item = ConfigurationOptionDescription>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ConfigurationOptionDescription")]
pub struct ConfigurationOptionDescription {
    #[serde(rename = "ChangeSeverity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_severity: Option<String>,
    #[serde(rename = "DefaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "MaxLength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i32>,
    #[serde(rename = "MaxValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<i32>,
    #[serde(rename = "MinValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "Regex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex: Option<OptionRestrictionRegex>,
    #[serde(rename = "UserDefined")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_defined: Option<bool>,
    #[serde(rename = "ValueOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_options: Option<ConfigurationOptionPossibleValues>,
    #[serde(rename = "ValueType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OptionRestrictionRegex")]
pub struct OptionRestrictionRegex {
    #[serde(rename = "Label")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "Pattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigurationOptionPossibleValues {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ConfigurationOptionPossibleValues {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ConfigurationOptionPossibleValues {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateStorageLocationResult")]
pub struct CreateStorageLocationResultMessage {
    #[serde(rename = "S3Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeletePlatformVersionResult")]
pub struct DeletePlatformVersionResult {
    #[serde(rename = "PlatformSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_summary: Option<PlatformSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateApplicationMessage")]
pub struct CreateApplicationMessage {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ResourceLifecycleConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_lifecycle_config: Option<ApplicationResourceLifecycleConfig>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTagsForResourceResult")]
pub struct ResourceTagsDescriptionMessage {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "ResourceTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
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

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AssociateEnvironmentOperationsRoleMessage")]
pub struct AssociateEnvironmentOperationsRoleMessage {
    #[serde(rename = "EnvironmentName")]
    #[serde(default)]
    pub environment_name: String,
    #[serde(rename = "OperationsRole")]
    #[serde(default)]
    pub operations_role: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateApplicationResourceLifecycleMessage")]
pub struct UpdateApplicationResourceLifecycleMessage {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "ResourceLifecycleConfig")]
    #[serde(default)]
    pub resource_lifecycle_config: ApplicationResourceLifecycleConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeConfigurationSettingsMessage")]
pub struct DescribeConfigurationSettingsMessage {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "EnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ApplyEnvironmentManagedActionRequest")]
pub struct ApplyEnvironmentManagedActionRequest {
    #[serde(rename = "ActionId")]
    #[serde(default)]
    pub action_id: String,
    #[serde(rename = "EnvironmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    #[serde(rename = "EnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateEnvironmentMessage")]
pub struct UpdateEnvironmentMessage {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EnvironmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    #[serde(rename = "EnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "OptionSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_settings: Option<ConfigurationOptionSettingsList>,
    #[serde(rename = "OptionsToRemove")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options_to_remove: Option<OptionsSpecifierList>,
    #[serde(rename = "PlatformArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_arn: Option<String>,
    #[serde(rename = "SolutionStackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_stack_name: Option<String>,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    #[serde(rename = "Tier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<EnvironmentTier>,
    #[serde(rename = "VersionLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_label: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CheckDNSAvailabilityResult")]
pub struct CheckDNSAvailabilityResultMessage {
    #[serde(rename = "Available")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,
    #[serde(rename = "FullyQualifiedCNAME")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fully_qualified_c_n_a_m_e: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeAccountAttributesResult")]
pub struct DescribeAccountAttributesResult {
    #[serde(rename = "ResourceQuotas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_quotas: Option<ResourceQuotas>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResourceQuotas")]
pub struct ResourceQuotas {
    #[serde(rename = "ApplicationQuota")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_quota: Option<ResourceQuota>,
    #[serde(rename = "ApplicationVersionQuota")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_version_quota: Option<ResourceQuota>,
    #[serde(rename = "ConfigurationTemplateQuota")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_template_quota: Option<ResourceQuota>,
    #[serde(rename = "CustomPlatformQuota")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_platform_quota: Option<ResourceQuota>,
    #[serde(rename = "EnvironmentQuota")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_quota: Option<ResourceQuota>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResourceQuota")]
pub struct ResourceQuota {
    #[serde(rename = "Maximum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeInstancesHealthRequest")]
pub struct DescribeInstancesHealthRequest {
    #[serde(rename = "AttributeNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_names: Option<InstancesHealthAttributes>,
    #[serde(rename = "EnvironmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    #[serde(rename = "EnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstancesHealthAttributes {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for InstancesHealthAttributes {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for InstancesHealthAttributes {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ValidateConfigurationSettingsMessage")]
pub struct ValidateConfigurationSettingsMessage {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "EnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    #[serde(rename = "OptionSettings")]
    #[serde(default)]
    pub option_settings: ConfigurationOptionSettingsList,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateConfigurationTemplateMessage")]
pub struct UpdateConfigurationTemplateMessage {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "OptionSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_settings: Option<ConfigurationOptionSettingsList>,
    #[serde(rename = "OptionsToRemove")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options_to_remove: Option<OptionsSpecifierList>,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    pub template_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateTagsForResourceMessage")]
pub struct UpdateTagsForResourceMessage {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "TagsToAdd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_to_add: Option<TagList>,
    #[serde(rename = "TagsToRemove")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_to_remove: Option<TagKeyList>,
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
#[serde(rename = "DescribeApplicationVersionsResult")]
pub struct ApplicationVersionDescriptionsMessage {
    #[serde(rename = "ApplicationVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_versions: Option<ApplicationVersionDescriptionList>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationVersionDescriptionList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ApplicationVersionDescription>,
}
impl From<Vec<ApplicationVersionDescription>> for ApplicationVersionDescriptionList {
    fn from(v: Vec<ApplicationVersionDescription>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ApplicationVersionDescription> for ApplicationVersionDescriptionList {
    fn from_iter<I: IntoIterator<Item = ApplicationVersionDescription>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ApplicationVersionDescription>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlApplicationVersionDescriptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ApplicationVersionDescription>,
}

impl From<Vec<ApplicationVersionDescription>> for XmlApplicationVersionDescriptionList {
    fn from(v: Vec<ApplicationVersionDescription>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ApplicationVersionDescription> for XmlApplicationVersionDescriptionList {
    fn from_iter<I: IntoIterator<Item = ApplicationVersionDescription>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CheckDNSAvailabilityMessage")]
pub struct CheckDNSAvailabilityMessage {
    #[serde(rename = "CNAMEPrefix")]
    #[serde(default)]
    pub c_n_a_m_e_prefix: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateConfigurationTemplateMessage")]
pub struct CreateConfigurationTemplateMessage {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EnvironmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    #[serde(rename = "OptionSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_settings: Option<ConfigurationOptionSettingsList>,
    #[serde(rename = "PlatformArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_arn: Option<String>,
    #[serde(rename = "SolutionStackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solution_stack_name: Option<String>,
    #[serde(rename = "SourceConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_configuration: Option<SourceConfiguration>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Tags>,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    pub template_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SourceConfiguration")]
pub struct SourceConfiguration {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListPlatformBranchesRequest")]
pub struct ListPlatformBranchesRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<SearchFilters>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchFilters {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<SearchFilter>,
}
impl From<Vec<SearchFilter>> for SearchFilters {
    fn from(v: Vec<SearchFilter>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<SearchFilter> for SearchFilters {
    fn from_iter<I: IntoIterator<Item = SearchFilter>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<SearchFilter>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlSearchFilterList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<SearchFilter>,
}

impl From<Vec<SearchFilter>> for XmlSearchFilterList {
    fn from(v: Vec<SearchFilter>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<SearchFilter> for XmlSearchFilterList {
    fn from_iter<I: IntoIterator<Item = SearchFilter>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SearchFilter")]
pub struct SearchFilter {
    #[serde(rename = "Attribute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<String>,
    #[serde(rename = "Operator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<SearchFilterValues>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchFilterValues {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for SearchFilterValues {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for SearchFilterValues {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEnvironmentManagedActionHistoryResult")]
pub struct DescribeEnvironmentManagedActionHistoryResult {
    #[serde(rename = "ManagedActionHistoryItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_action_history_items: Option<ManagedActionHistoryItems>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManagedActionHistoryItems {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ManagedActionHistoryItem>,
}
impl From<Vec<ManagedActionHistoryItem>> for ManagedActionHistoryItems {
    fn from(v: Vec<ManagedActionHistoryItem>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ManagedActionHistoryItem> for ManagedActionHistoryItems {
    fn from_iter<I: IntoIterator<Item = ManagedActionHistoryItem>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ManagedActionHistoryItem>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlManagedActionHistoryItemList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ManagedActionHistoryItem>,
}

impl From<Vec<ManagedActionHistoryItem>> for XmlManagedActionHistoryItemList {
    fn from(v: Vec<ManagedActionHistoryItem>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ManagedActionHistoryItem> for XmlManagedActionHistoryItemList {
    fn from_iter<I: IntoIterator<Item = ManagedActionHistoryItem>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ManagedActionHistoryItem")]
pub struct ManagedActionHistoryItem {
    #[serde(rename = "ActionDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_description: Option<String>,
    #[serde(rename = "ActionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(rename = "ActionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    #[serde(rename = "ExecutedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executed_time: Option<String>,
    #[serde(rename = "FailureDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_description: Option<String>,
    #[serde(rename = "FailureType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_type: Option<String>,
    #[serde(rename = "FinishedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_time: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEnvironmentResourcesResult")]
pub struct EnvironmentResourceDescriptionsMessage {
    #[serde(rename = "EnvironmentResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_resources: Option<EnvironmentResourceDescription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EnvironmentResourceDescription")]
pub struct EnvironmentResourceDescription {
    #[serde(rename = "AutoScalingGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_groups: Option<AutoScalingGroupList>,
    #[serde(rename = "EnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    #[serde(rename = "Instances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<InstanceList>,
    #[serde(rename = "LaunchConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_configurations: Option<LaunchConfigurationList>,
    #[serde(rename = "LaunchTemplates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_templates: Option<LaunchTemplateList>,
    #[serde(rename = "LoadBalancers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancers: Option<LoadBalancerList>,
    #[serde(rename = "Queues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queues: Option<QueueList>,
    #[serde(rename = "Triggers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggers: Option<TriggerList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoScalingGroupList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<AutoScalingGroup>,
}
impl From<Vec<AutoScalingGroup>> for AutoScalingGroupList {
    fn from(v: Vec<AutoScalingGroup>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<AutoScalingGroup> for AutoScalingGroupList {
    fn from_iter<I: IntoIterator<Item = AutoScalingGroup>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<AutoScalingGroup>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlAutoScalingGroupList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<AutoScalingGroup>,
}

impl From<Vec<AutoScalingGroup>> for XmlAutoScalingGroupList {
    fn from(v: Vec<AutoScalingGroup>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<AutoScalingGroup> for XmlAutoScalingGroupList {
    fn from_iter<I: IntoIterator<Item = AutoScalingGroup>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AutoScalingGroup")]
pub struct AutoScalingGroup {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Instance>,
}
impl From<Vec<Instance>> for InstanceList {
    fn from(v: Vec<Instance>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Instance> for InstanceList {
    fn from_iter<I: IntoIterator<Item = Instance>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Instance>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlInstanceList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Instance>,
}

impl From<Vec<Instance>> for XmlInstanceList {
    fn from(v: Vec<Instance>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Instance> for XmlInstanceList {
    fn from_iter<I: IntoIterator<Item = Instance>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Instance")]
pub struct Instance {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LaunchConfigurationList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<LaunchConfiguration>,
}
impl From<Vec<LaunchConfiguration>> for LaunchConfigurationList {
    fn from(v: Vec<LaunchConfiguration>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<LaunchConfiguration> for LaunchConfigurationList {
    fn from_iter<I: IntoIterator<Item = LaunchConfiguration>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<LaunchConfiguration>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlLaunchConfigurationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<LaunchConfiguration>,
}

impl From<Vec<LaunchConfiguration>> for XmlLaunchConfigurationList {
    fn from(v: Vec<LaunchConfiguration>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<LaunchConfiguration> for XmlLaunchConfigurationList {
    fn from_iter<I: IntoIterator<Item = LaunchConfiguration>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LaunchConfiguration")]
pub struct LaunchConfiguration {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LaunchTemplateList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<LaunchTemplate>,
}
impl From<Vec<LaunchTemplate>> for LaunchTemplateList {
    fn from(v: Vec<LaunchTemplate>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<LaunchTemplate> for LaunchTemplateList {
    fn from_iter<I: IntoIterator<Item = LaunchTemplate>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<LaunchTemplate>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlLaunchTemplateList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<LaunchTemplate>,
}

impl From<Vec<LaunchTemplate>> for XmlLaunchTemplateList {
    fn from(v: Vec<LaunchTemplate>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<LaunchTemplate> for XmlLaunchTemplateList {
    fn from_iter<I: IntoIterator<Item = LaunchTemplate>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LaunchTemplate")]
pub struct LaunchTemplate {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoadBalancerList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<LoadBalancer>,
}
impl From<Vec<LoadBalancer>> for LoadBalancerList {
    fn from(v: Vec<LoadBalancer>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<LoadBalancer> for LoadBalancerList {
    fn from_iter<I: IntoIterator<Item = LoadBalancer>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<LoadBalancer>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlLoadBalancerList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<LoadBalancer>,
}

impl From<Vec<LoadBalancer>> for XmlLoadBalancerList {
    fn from(v: Vec<LoadBalancer>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<LoadBalancer> for XmlLoadBalancerList {
    fn from_iter<I: IntoIterator<Item = LoadBalancer>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LoadBalancer")]
pub struct LoadBalancer {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueueList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Queue>,
}
impl From<Vec<Queue>> for QueueList {
    fn from(v: Vec<Queue>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Queue> for QueueList {
    fn from_iter<I: IntoIterator<Item = Queue>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Queue>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlQueueList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Queue>,
}

impl From<Vec<Queue>> for XmlQueueList {
    fn from(v: Vec<Queue>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Queue> for XmlQueueList {
    fn from_iter<I: IntoIterator<Item = Queue>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Queue")]
pub struct Queue {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "URL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_r_l: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TriggerList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Trigger>,
}
impl From<Vec<Trigger>> for TriggerList {
    fn from(v: Vec<Trigger>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Trigger> for TriggerList {
    fn from_iter<I: IntoIterator<Item = Trigger>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Trigger>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTriggerList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Trigger>,
}

impl From<Vec<Trigger>> for XmlTriggerList {
    fn from(v: Vec<Trigger>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Trigger> for XmlTriggerList {
    fn from_iter<I: IntoIterator<Item = Trigger>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Trigger")]
pub struct Trigger {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SwapEnvironmentCNAMEsMessage")]
pub struct SwapEnvironmentCNAMEsMessage {
    #[serde(rename = "DestinationEnvironmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_environment_id: Option<String>,
    #[serde(rename = "DestinationEnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_environment_name: Option<String>,
    #[serde(rename = "SourceEnvironmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_environment_id: Option<String>,
    #[serde(rename = "SourceEnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_environment_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TerminateEnvironmentMessage")]
pub struct TerminateEnvironmentMessage {
    #[serde(rename = "EnvironmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    #[serde(rename = "EnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    #[serde(rename = "ForceTerminate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_terminate: Option<bool>,
    #[serde(rename = "TerminateResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminate_resources: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ApplyEnvironmentManagedActionResult")]
pub struct ApplyEnvironmentManagedActionResult {
    #[serde(rename = "ActionDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_description: Option<String>,
    #[serde(rename = "ActionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(rename = "ActionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteConfigurationTemplateMessage")]
pub struct DeleteConfigurationTemplateMessage {
    #[serde(rename = "ApplicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    pub template_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeInstancesHealthResult")]
pub struct DescribeInstancesHealthResult {
    #[serde(rename = "InstanceHealthList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_health_list: Option<InstanceHealthList>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RefreshedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refreshed_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceHealthList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<SingleInstanceHealth>,
}
impl From<Vec<SingleInstanceHealth>> for InstanceHealthList {
    fn from(v: Vec<SingleInstanceHealth>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<SingleInstanceHealth> for InstanceHealthList {
    fn from_iter<I: IntoIterator<Item = SingleInstanceHealth>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<SingleInstanceHealth>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlSingleInstanceHealthList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<SingleInstanceHealth>,
}

impl From<Vec<SingleInstanceHealth>> for XmlSingleInstanceHealthList {
    fn from(v: Vec<SingleInstanceHealth>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<SingleInstanceHealth> for XmlSingleInstanceHealthList {
    fn from_iter<I: IntoIterator<Item = SingleInstanceHealth>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SingleInstanceHealth")]
pub struct SingleInstanceHealth {
    #[serde(rename = "ApplicationMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_metrics: Option<ApplicationMetrics>,
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "Causes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub causes: Option<Causes>,
    #[serde(rename = "Color")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "Deployment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment: Option<Deployment>,
    #[serde(rename = "HealthStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_status: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "LaunchedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launched_at: Option<String>,
    #[serde(rename = "System")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<SystemStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Deployment")]
pub struct Deployment {
    #[serde(rename = "DeploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<i64>,
    #[serde(rename = "DeploymentTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_time: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "VersionLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_label: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SystemStatus")]
pub struct SystemStatus {
    #[serde(rename = "CPUUtilization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_p_u_utilization: Option<CPUUtilization>,
    #[serde(rename = "LoadAverage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_average: Option<LoadAverage>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CPUUtilization")]
pub struct CPUUtilization {
    #[serde(rename = "IOWait")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_o_wait: Option<f64>,
    #[serde(rename = "IRQ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_r_q: Option<f64>,
    #[serde(rename = "Idle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle: Option<f64>,
    #[serde(rename = "Nice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nice: Option<f64>,
    #[serde(rename = "Privileged")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<f64>,
    #[serde(rename = "SoftIRQ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soft_i_r_q: Option<f64>,
    #[serde(rename = "System")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<f64>,
    #[serde(rename = "User")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoadAverage {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<f64>,
}
impl From<Vec<f64>> for LoadAverage {
    fn from(v: Vec<f64>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<f64> for LoadAverage {
    fn from_iter<I: IntoIterator<Item = f64>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<f64>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Xmlf64List {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<f64>,
}

impl From<Vec<f64>> for Xmlf64List {
    fn from(v: Vec<f64>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<f64> for Xmlf64List {
    fn from_iter<I: IntoIterator<Item = f64>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RebuildEnvironmentMessage")]
pub struct RebuildEnvironmentMessage {
    #[serde(rename = "EnvironmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<String>,
    #[serde(rename = "EnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
}
