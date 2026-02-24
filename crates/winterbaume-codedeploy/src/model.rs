//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-codedeploy

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddTagsToOnPremisesInstancesInput {
    #[serde(rename = "instanceNames")]
    #[serde(default)]
    pub instance_names: Vec<String>,
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
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
pub struct BatchGetApplicationRevisionsInput {
    #[serde(rename = "applicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(default)]
    pub revisions: Vec<RevisionLocation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RevisionLocation {
    #[serde(rename = "appSpecContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_spec_content: Option<AppSpecContent>,
    #[serde(rename = "gitHubLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_hub_location: Option<GitHubLocation>,
    #[serde(rename = "revisionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_type: Option<String>,
    #[serde(rename = "s3Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_location: Option<S3Location>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string: Option<RawString>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AppSpecContent {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GitHubLocation {
    #[serde(rename = "commitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Location {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(rename = "bundleType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_type: Option<String>,
    #[serde(rename = "eTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_tag: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RawString {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetApplicationRevisionsOutput {
    #[serde(rename = "applicationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revisions: Option<Vec<RevisionInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RevisionInfo {
    #[serde(rename = "genericRevisionInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generic_revision_info: Option<GenericRevisionInfo>,
    #[serde(rename = "revisionLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_location: Option<RevisionLocation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GenericRevisionInfo {
    #[serde(rename = "deploymentGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_groups: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "firstUsedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_used_time: Option<f64>,
    #[serde(rename = "lastUsedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used_time: Option<f64>,
    #[serde(rename = "registerTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetApplicationsInput {
    #[serde(rename = "applicationNames")]
    #[serde(default)]
    pub application_names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetApplicationsOutput {
    #[serde(rename = "applicationsInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applications_info: Option<Vec<ApplicationInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationInfo {
    #[serde(rename = "applicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "applicationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(rename = "computePlatform")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_platform: Option<String>,
    #[serde(rename = "createTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    #[serde(rename = "gitHubAccountName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub git_hub_account_name: Option<String>,
    #[serde(rename = "linkedToGitHub")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_to_git_hub: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetDeploymentGroupsInput {
    #[serde(rename = "applicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "deploymentGroupNames")]
    #[serde(default)]
    pub deployment_group_names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetDeploymentGroupsOutput {
    #[serde(rename = "deploymentGroupsInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_groups_info: Option<Vec<DeploymentGroupInfo>>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeploymentGroupInfo {
    #[serde(rename = "alarmConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_configuration: Option<AlarmConfiguration>,
    #[serde(rename = "applicationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(rename = "autoRollbackConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_rollback_configuration: Option<AutoRollbackConfiguration>,
    #[serde(rename = "autoScalingGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_groups: Option<Vec<AutoScalingGroup>>,
    #[serde(rename = "blueGreenDeploymentConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue_green_deployment_configuration: Option<BlueGreenDeploymentConfiguration>,
    #[serde(rename = "computePlatform")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_platform: Option<String>,
    #[serde(rename = "deploymentConfigName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_config_name: Option<String>,
    #[serde(rename = "deploymentGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_group_id: Option<String>,
    #[serde(rename = "deploymentGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_group_name: Option<String>,
    #[serde(rename = "deploymentStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_style: Option<DeploymentStyle>,
    #[serde(rename = "ec2TagFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_tag_filters: Option<Vec<EC2TagFilter>>,
    #[serde(rename = "ec2TagSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_tag_set: Option<EC2TagSet>,
    #[serde(rename = "ecsServices")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_services: Option<Vec<ECSService>>,
    #[serde(rename = "lastAttemptedDeployment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_attempted_deployment: Option<LastDeploymentInfo>,
    #[serde(rename = "lastSuccessfulDeployment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_deployment: Option<LastDeploymentInfo>,
    #[serde(rename = "loadBalancerInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_info: Option<LoadBalancerInfo>,
    #[serde(rename = "onPremisesInstanceTagFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_premises_instance_tag_filters: Option<Vec<TagFilter>>,
    #[serde(rename = "onPremisesTagSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_premises_tag_set: Option<OnPremisesTagSet>,
    #[serde(rename = "outdatedInstancesStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outdated_instances_strategy: Option<String>,
    #[serde(rename = "serviceRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_arn: Option<String>,
    #[serde(rename = "targetRevision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_revision: Option<RevisionLocation>,
    #[serde(rename = "terminationHookEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_hook_enabled: Option<bool>,
    #[serde(rename = "triggerConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_configurations: Option<Vec<TriggerConfig>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AlarmConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarms: Option<Vec<Alarm>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "ignorePollAlarmFailure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_poll_alarm_failure: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Alarm {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoRollbackConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoScalingGroup {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "terminationHook")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_hook: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BlueGreenDeploymentConfiguration {
    #[serde(rename = "deploymentReadyOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_ready_option: Option<DeploymentReadyOption>,
    #[serde(rename = "greenFleetProvisioningOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub green_fleet_provisioning_option: Option<GreenFleetProvisioningOption>,
    #[serde(rename = "terminateBlueInstancesOnDeploymentSuccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminate_blue_instances_on_deployment_success: Option<BlueInstanceTerminationOption>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeploymentReadyOption {
    #[serde(rename = "actionOnTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_on_timeout: Option<String>,
    #[serde(rename = "waitTimeInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_time_in_minutes: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GreenFleetProvisioningOption {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BlueInstanceTerminationOption {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "terminationWaitTimeInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_wait_time_in_minutes: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeploymentStyle {
    #[serde(rename = "deploymentOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_option: Option<String>,
    #[serde(rename = "deploymentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EC2TagFilter {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EC2TagSet {
    #[serde(rename = "ec2TagSetList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_tag_set_list: Option<Vec<Vec<EC2TagFilter>>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ECSService {
    #[serde(rename = "clusterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(rename = "serviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LastDeploymentInfo {
    #[serde(rename = "createTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    #[serde(rename = "deploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoadBalancerInfo {
    #[serde(rename = "elbInfoList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elb_info_list: Option<Vec<ELBInfo>>,
    #[serde(rename = "targetGroupInfoList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_group_info_list: Option<Vec<TargetGroupInfo>>,
    #[serde(rename = "targetGroupPairInfoList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_group_pair_info_list: Option<Vec<TargetGroupPairInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ELBInfo {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetGroupInfo {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetGroupPairInfo {
    #[serde(rename = "prodTrafficRoute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prod_traffic_route: Option<TrafficRoute>,
    #[serde(rename = "targetGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_groups: Option<Vec<TargetGroupInfo>>,
    #[serde(rename = "testTrafficRoute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_traffic_route: Option<TrafficRoute>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrafficRoute {
    #[serde(rename = "listenerArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_arns: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagFilter {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OnPremisesTagSet {
    #[serde(rename = "onPremisesTagSetList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_premises_tag_set_list: Option<Vec<Vec<TagFilter>>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TriggerConfig {
    #[serde(rename = "triggerEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_events: Option<Vec<String>>,
    #[serde(rename = "triggerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_name: Option<String>,
    #[serde(rename = "triggerTargetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_target_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetDeploymentInstancesInput {
    #[serde(rename = "deploymentId")]
    #[serde(default)]
    pub deployment_id: String,
    #[serde(rename = "instanceIds")]
    #[serde(default)]
    pub instance_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetDeploymentInstancesOutput {
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "instancesSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_summary: Option<Vec<InstanceSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceSummary {
    #[serde(rename = "deploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(rename = "instanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "instanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(rename = "lifecycleEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_events: Option<Vec<LifecycleEvent>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LifecycleEvent {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostics: Option<Diagnostics>,
    #[serde(rename = "endTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "lifecycleEventName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_event_name: Option<String>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Diagnostics {
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "logTail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_tail: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "scriptName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetDeploymentTargetsInput {
    #[serde(rename = "deploymentId")]
    #[serde(default)]
    pub deployment_id: String,
    #[serde(rename = "targetIds")]
    #[serde(default)]
    pub target_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetDeploymentTargetsOutput {
    #[serde(rename = "deploymentTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_targets: Option<Vec<DeploymentTarget>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeploymentTarget {
    #[serde(rename = "cloudFormationTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_formation_target: Option<CloudFormationTarget>,
    #[serde(rename = "deploymentTargetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_target_type: Option<String>,
    #[serde(rename = "ecsTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_target: Option<ECSTarget>,
    #[serde(rename = "instanceTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_target: Option<InstanceTarget>,
    #[serde(rename = "lambdaTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_target: Option<LambdaTarget>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudFormationTarget {
    #[serde(rename = "deploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(rename = "lifecycleEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_events: Option<Vec<LifecycleEvent>>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "targetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    #[serde(rename = "targetVersionWeight")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_version_weight: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ECSTarget {
    #[serde(rename = "deploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(rename = "lifecycleEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_events: Option<Vec<LifecycleEvent>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "targetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
    #[serde(rename = "targetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    #[serde(rename = "taskSetsInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_sets_info: Option<Vec<ECSTaskSet>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ECSTaskSet {
    #[serde(rename = "desiredCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_count: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifer: Option<String>,
    #[serde(rename = "pendingCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_count: Option<i64>,
    #[serde(rename = "runningCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_count: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "targetGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_group: Option<TargetGroupInfo>,
    #[serde(rename = "taskSetLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_set_label: Option<String>,
    #[serde(rename = "trafficWeight")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_weight: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceTarget {
    #[serde(rename = "deploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(rename = "instanceLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_label: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(rename = "lifecycleEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_events: Option<Vec<LifecycleEvent>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "targetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
    #[serde(rename = "targetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LambdaTarget {
    #[serde(rename = "deploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(rename = "lambdaFunctionInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_info: Option<LambdaFunctionInfo>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(rename = "lifecycleEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_events: Option<Vec<LifecycleEvent>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "targetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
    #[serde(rename = "targetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LambdaFunctionInfo {
    #[serde(rename = "currentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
    #[serde(rename = "functionAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_alias: Option<String>,
    #[serde(rename = "functionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
    #[serde(rename = "targetVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_version: Option<String>,
    #[serde(rename = "targetVersionWeight")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_version_weight: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetDeploymentsInput {
    #[serde(rename = "deploymentIds")]
    #[serde(default)]
    pub deployment_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetDeploymentsOutput {
    #[serde(rename = "deploymentsInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployments_info: Option<Vec<DeploymentInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeploymentInfo {
    #[serde(rename = "additionalDeploymentStatusInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_deployment_status_info: Option<String>,
    #[serde(rename = "applicationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(rename = "autoRollbackConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_rollback_configuration: Option<AutoRollbackConfiguration>,
    #[serde(rename = "blueGreenDeploymentConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue_green_deployment_configuration: Option<BlueGreenDeploymentConfiguration>,
    #[serde(rename = "completeTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete_time: Option<f64>,
    #[serde(rename = "computePlatform")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_platform: Option<String>,
    #[serde(rename = "createTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    #[serde(rename = "deploymentConfigName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_config_name: Option<String>,
    #[serde(rename = "deploymentGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_group_name: Option<String>,
    #[serde(rename = "deploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(rename = "deploymentOverview")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_overview: Option<DeploymentOverview>,
    #[serde(rename = "deploymentStatusMessages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_status_messages: Option<Vec<String>>,
    #[serde(rename = "deploymentStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_style: Option<DeploymentStyle>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "errorInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_information: Option<ErrorInformation>,
    #[serde(rename = "externalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "fileExistsBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_exists_behavior: Option<String>,
    #[serde(rename = "ignoreApplicationStopFailures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_application_stop_failures: Option<bool>,
    #[serde(rename = "instanceTerminationWaitTimeStarted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_termination_wait_time_started: Option<bool>,
    #[serde(rename = "loadBalancerInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_info: Option<LoadBalancerInfo>,
    #[serde(rename = "overrideAlarmConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_alarm_configuration: Option<AlarmConfiguration>,
    #[serde(rename = "previousRevision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_revision: Option<RevisionLocation>,
    #[serde(rename = "relatedDeployments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_deployments: Option<RelatedDeployments>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionLocation>,
    #[serde(rename = "rollbackInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_info: Option<RollbackInfo>,
    #[serde(rename = "startTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "targetInstances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_instances: Option<TargetInstances>,
    #[serde(rename = "updateOutdatedInstancesOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_outdated_instances_only: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeploymentOverview {
    #[serde(rename = "Failed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<i64>,
    #[serde(rename = "InProgress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_progress: Option<i64>,
    #[serde(rename = "Pending")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<i64>,
    #[serde(rename = "Ready")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready: Option<i64>,
    #[serde(rename = "Skipped")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipped: Option<i64>,
    #[serde(rename = "Succeeded")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub succeeded: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ErrorInformation {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RelatedDeployments {
    #[serde(rename = "autoUpdateOutdatedInstancesDeploymentIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_update_outdated_instances_deployment_ids: Option<Vec<String>>,
    #[serde(rename = "autoUpdateOutdatedInstancesRootDeploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_update_outdated_instances_root_deployment_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RollbackInfo {
    #[serde(rename = "rollbackDeploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_deployment_id: Option<String>,
    #[serde(rename = "rollbackMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_message: Option<String>,
    #[serde(rename = "rollbackTriggeringDeploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_triggering_deployment_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetInstances {
    #[serde(rename = "autoScalingGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_groups: Option<Vec<String>>,
    #[serde(rename = "ec2TagSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_tag_set: Option<EC2TagSet>,
    #[serde(rename = "tagFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filters: Option<Vec<EC2TagFilter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetOnPremisesInstancesInput {
    #[serde(rename = "instanceNames")]
    #[serde(default)]
    pub instance_names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetOnPremisesInstancesOutput {
    #[serde(rename = "instanceInfos")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_infos: Option<Vec<InstanceInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceInfo {
    #[serde(rename = "deregisterTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deregister_time: Option<f64>,
    #[serde(rename = "iamSessionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_session_arn: Option<String>,
    #[serde(rename = "iamUserArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_user_arn: Option<String>,
    #[serde(rename = "instanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arn: Option<String>,
    #[serde(rename = "instanceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    #[serde(rename = "registerTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContinueDeploymentInput {
    #[serde(rename = "deploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(rename = "deploymentWaitType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_wait_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApplicationInput {
    #[serde(rename = "applicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "computePlatform")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_platform: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApplicationOutput {
    #[serde(rename = "applicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDeploymentConfigInput {
    #[serde(rename = "computePlatform")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_platform: Option<String>,
    #[serde(rename = "deploymentConfigName")]
    #[serde(default)]
    pub deployment_config_name: String,
    #[serde(rename = "minimumHealthyHosts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_healthy_hosts: Option<MinimumHealthyHosts>,
    #[serde(rename = "trafficRoutingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_routing_config: Option<TrafficRoutingConfig>,
    #[serde(rename = "zonalConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zonal_config: Option<ZonalConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MinimumHealthyHosts {
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrafficRoutingConfig {
    #[serde(rename = "timeBasedCanary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_based_canary: Option<TimeBasedCanary>,
    #[serde(rename = "timeBasedLinear")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_based_linear: Option<TimeBasedLinear>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimeBasedCanary {
    #[serde(rename = "canaryInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canary_interval: Option<i32>,
    #[serde(rename = "canaryPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canary_percentage: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimeBasedLinear {
    #[serde(rename = "linearInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linear_interval: Option<i32>,
    #[serde(rename = "linearPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linear_percentage: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ZonalConfig {
    #[serde(rename = "firstZoneMonitorDurationInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_zone_monitor_duration_in_seconds: Option<i64>,
    #[serde(rename = "minimumHealthyHostsPerZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_healthy_hosts_per_zone: Option<MinimumHealthyHostsPerZone>,
    #[serde(rename = "monitorDurationInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_duration_in_seconds: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MinimumHealthyHostsPerZone {
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDeploymentConfigOutput {
    #[serde(rename = "deploymentConfigId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_config_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDeploymentGroupInput {
    #[serde(rename = "alarmConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_configuration: Option<AlarmConfiguration>,
    #[serde(rename = "applicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "autoRollbackConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_rollback_configuration: Option<AutoRollbackConfiguration>,
    #[serde(rename = "autoScalingGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_groups: Option<Vec<String>>,
    #[serde(rename = "blueGreenDeploymentConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue_green_deployment_configuration: Option<BlueGreenDeploymentConfiguration>,
    #[serde(rename = "deploymentConfigName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_config_name: Option<String>,
    #[serde(rename = "deploymentGroupName")]
    #[serde(default)]
    pub deployment_group_name: String,
    #[serde(rename = "deploymentStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_style: Option<DeploymentStyle>,
    #[serde(rename = "ec2TagFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_tag_filters: Option<Vec<EC2TagFilter>>,
    #[serde(rename = "ec2TagSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_tag_set: Option<EC2TagSet>,
    #[serde(rename = "ecsServices")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_services: Option<Vec<ECSService>>,
    #[serde(rename = "loadBalancerInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_info: Option<LoadBalancerInfo>,
    #[serde(rename = "onPremisesInstanceTagFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_premises_instance_tag_filters: Option<Vec<TagFilter>>,
    #[serde(rename = "onPremisesTagSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_premises_tag_set: Option<OnPremisesTagSet>,
    #[serde(rename = "outdatedInstancesStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outdated_instances_strategy: Option<String>,
    #[serde(rename = "serviceRoleArn")]
    #[serde(default)]
    pub service_role_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "terminationHookEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_hook_enabled: Option<bool>,
    #[serde(rename = "triggerConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_configurations: Option<Vec<TriggerConfig>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDeploymentGroupOutput {
    #[serde(rename = "deploymentGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_group_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDeploymentInput {
    #[serde(rename = "applicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "autoRollbackConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_rollback_configuration: Option<AutoRollbackConfiguration>,
    #[serde(rename = "deploymentConfigName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_config_name: Option<String>,
    #[serde(rename = "deploymentGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_group_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "fileExistsBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_exists_behavior: Option<String>,
    #[serde(rename = "ignoreApplicationStopFailures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_application_stop_failures: Option<bool>,
    #[serde(rename = "overrideAlarmConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_alarm_configuration: Option<AlarmConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionLocation>,
    #[serde(rename = "targetInstances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_instances: Option<TargetInstances>,
    #[serde(rename = "updateOutdatedInstancesOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_outdated_instances_only: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDeploymentOutput {
    #[serde(rename = "deploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApplicationInput {
    #[serde(rename = "applicationName")]
    #[serde(default)]
    pub application_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDeploymentConfigInput {
    #[serde(rename = "deploymentConfigName")]
    #[serde(default)]
    pub deployment_config_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDeploymentGroupInput {
    #[serde(rename = "applicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "deploymentGroupName")]
    #[serde(default)]
    pub deployment_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDeploymentGroupOutput {
    #[serde(rename = "hooksNotCleanedUp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hooks_not_cleaned_up: Option<Vec<AutoScalingGroup>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteGitHubAccountTokenInput {
    #[serde(rename = "tokenName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteGitHubAccountTokenOutput {
    #[serde(rename = "tokenName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourcesByExternalIdInput {
    #[serde(rename = "externalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourcesByExternalIdOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterOnPremisesInstanceInput {
    #[serde(rename = "instanceName")]
    #[serde(default)]
    pub instance_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApplicationInput {
    #[serde(rename = "applicationName")]
    #[serde(default)]
    pub application_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApplicationOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<ApplicationInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApplicationRevisionInput {
    #[serde(rename = "applicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(default)]
    pub revision: RevisionLocation,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApplicationRevisionOutput {
    #[serde(rename = "applicationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<RevisionLocation>,
    #[serde(rename = "revisionInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_info: Option<GenericRevisionInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeploymentConfigInput {
    #[serde(rename = "deploymentConfigName")]
    #[serde(default)]
    pub deployment_config_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeploymentConfigOutput {
    #[serde(rename = "deploymentConfigInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_config_info: Option<DeploymentConfigInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeploymentConfigInfo {
    #[serde(rename = "computePlatform")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_platform: Option<String>,
    #[serde(rename = "createTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    #[serde(rename = "deploymentConfigId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_config_id: Option<String>,
    #[serde(rename = "deploymentConfigName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_config_name: Option<String>,
    #[serde(rename = "minimumHealthyHosts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_healthy_hosts: Option<MinimumHealthyHosts>,
    #[serde(rename = "trafficRoutingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic_routing_config: Option<TrafficRoutingConfig>,
    #[serde(rename = "zonalConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zonal_config: Option<ZonalConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeploymentGroupInput {
    #[serde(rename = "applicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "deploymentGroupName")]
    #[serde(default)]
    pub deployment_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeploymentGroupOutput {
    #[serde(rename = "deploymentGroupInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_group_info: Option<DeploymentGroupInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeploymentInput {
    #[serde(rename = "deploymentId")]
    #[serde(default)]
    pub deployment_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeploymentInstanceInput {
    #[serde(rename = "deploymentId")]
    #[serde(default)]
    pub deployment_id: String,
    #[serde(rename = "instanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeploymentInstanceOutput {
    #[serde(rename = "instanceSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_summary: Option<InstanceSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeploymentOutput {
    #[serde(rename = "deploymentInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_info: Option<DeploymentInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeploymentTargetInput {
    #[serde(rename = "deploymentId")]
    #[serde(default)]
    pub deployment_id: String,
    #[serde(rename = "targetId")]
    #[serde(default)]
    pub target_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeploymentTargetOutput {
    #[serde(rename = "deploymentTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_target: Option<DeploymentTarget>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOnPremisesInstanceInput {
    #[serde(rename = "instanceName")]
    #[serde(default)]
    pub instance_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOnPremisesInstanceOutput {
    #[serde(rename = "instanceInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_info: Option<InstanceInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationRevisionsInput {
    #[serde(rename = "applicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployed: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "s3Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket: Option<String>,
    #[serde(rename = "s3KeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationRevisionsOutput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revisions: Option<Vec<RevisionLocation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationsInput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationsOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applications: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDeploymentConfigsInput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDeploymentConfigsOutput {
    #[serde(rename = "deploymentConfigsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_configs_list: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDeploymentGroupsInput {
    #[serde(rename = "applicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDeploymentGroupsOutput {
    #[serde(rename = "applicationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(rename = "deploymentGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_groups: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDeploymentInstancesInput {
    #[serde(rename = "deploymentId")]
    #[serde(default)]
    pub deployment_id: String,
    #[serde(rename = "instanceStatusFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_status_filter: Option<Vec<String>>,
    #[serde(rename = "instanceTypeFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type_filter: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDeploymentInstancesOutput {
    #[serde(rename = "instancesList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_list: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDeploymentTargetsInput {
    #[serde(rename = "deploymentId")]
    #[serde(default)]
    pub deployment_id: String,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "targetFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_filters: Option<std::collections::HashMap<String, Vec<String>>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDeploymentTargetsOutput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "targetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDeploymentsInput {
    #[serde(rename = "applicationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(rename = "createTimeRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time_range: Option<TimeRange>,
    #[serde(rename = "deploymentGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_group_name: Option<String>,
    #[serde(rename = "externalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "includeOnlyStatuses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_only_statuses: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimeRange {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDeploymentsOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployments: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListGitHubAccountTokenNamesInput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListGitHubAccountTokenNamesOutput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "tokenNameList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_name_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOnPremisesInstancesInput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "registrationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_status: Option<String>,
    #[serde(rename = "tagFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_filters: Option<Vec<TagFilter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOnPremisesInstancesOutput {
    #[serde(rename = "instanceNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_names: Option<Vec<String>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceInput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutLifecycleEventHookExecutionStatusInput {
    #[serde(rename = "deploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    #[serde(rename = "lifecycleEventHookExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_event_hook_execution_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutLifecycleEventHookExecutionStatusOutput {
    #[serde(rename = "lifecycleEventHookExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_event_hook_execution_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterApplicationRevisionInput {
    #[serde(rename = "applicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub revision: RevisionLocation,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterOnPremisesInstanceInput {
    #[serde(rename = "iamSessionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_session_arn: Option<String>,
    #[serde(rename = "iamUserArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_user_arn: Option<String>,
    #[serde(rename = "instanceName")]
    #[serde(default)]
    pub instance_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveTagsFromOnPremisesInstancesInput {
    #[serde(rename = "instanceNames")]
    #[serde(default)]
    pub instance_names: Vec<String>,
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SkipWaitTimeForInstanceTerminationInput {
    #[serde(rename = "deploymentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopDeploymentInput {
    #[serde(rename = "autoRollbackEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_rollback_enabled: Option<bool>,
    #[serde(rename = "deploymentId")]
    #[serde(default)]
    pub deployment_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopDeploymentOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceInput {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceInput {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApplicationInput {
    #[serde(rename = "applicationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(rename = "newApplicationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_application_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDeploymentGroupInput {
    #[serde(rename = "alarmConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alarm_configuration: Option<AlarmConfiguration>,
    #[serde(rename = "applicationName")]
    #[serde(default)]
    pub application_name: String,
    #[serde(rename = "autoRollbackConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_rollback_configuration: Option<AutoRollbackConfiguration>,
    #[serde(rename = "autoScalingGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_groups: Option<Vec<String>>,
    #[serde(rename = "blueGreenDeploymentConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue_green_deployment_configuration: Option<BlueGreenDeploymentConfiguration>,
    #[serde(rename = "currentDeploymentGroupName")]
    #[serde(default)]
    pub current_deployment_group_name: String,
    #[serde(rename = "deploymentConfigName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_config_name: Option<String>,
    #[serde(rename = "deploymentStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_style: Option<DeploymentStyle>,
    #[serde(rename = "ec2TagFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_tag_filters: Option<Vec<EC2TagFilter>>,
    #[serde(rename = "ec2TagSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_tag_set: Option<EC2TagSet>,
    #[serde(rename = "ecsServices")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_services: Option<Vec<ECSService>>,
    #[serde(rename = "loadBalancerInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_info: Option<LoadBalancerInfo>,
    #[serde(rename = "newDeploymentGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_deployment_group_name: Option<String>,
    #[serde(rename = "onPremisesInstanceTagFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_premises_instance_tag_filters: Option<Vec<TagFilter>>,
    #[serde(rename = "onPremisesTagSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_premises_tag_set: Option<OnPremisesTagSet>,
    #[serde(rename = "outdatedInstancesStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outdated_instances_strategy: Option<String>,
    #[serde(rename = "serviceRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_arn: Option<String>,
    #[serde(rename = "terminationHookEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_hook_enabled: Option<bool>,
    #[serde(rename = "triggerConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_configurations: Option<Vec<TriggerConfig>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDeploymentGroupOutput {
    #[serde(rename = "hooksNotCleanedUp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hooks_not_cleaned_up: Option<Vec<AutoScalingGroup>>,
}
