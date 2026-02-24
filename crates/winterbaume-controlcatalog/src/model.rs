//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-controlcatalog

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetControlRequest {
    #[serde(rename = "ControlArn")]
    #[serde(default)]
    pub control_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetControlResponse {
    #[serde(rename = "Aliases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Behavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior: Option<String>,
    #[serde(rename = "CreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "GovernedResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub governed_resources: Option<Vec<String>>,
    #[serde(rename = "Implementation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implementation: Option<ImplementationDetails>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ControlParameter>>,
    #[serde(rename = "RegionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_configuration: Option<RegionConfiguration>,
    #[serde(rename = "Severity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImplementationDetails {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ControlParameter {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegionConfiguration {
    #[serde(rename = "DeployableRegions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployable_regions: Option<Vec<String>>,
    #[serde(rename = "Scope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCommonControlsRequest {
    #[serde(rename = "CommonControlFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_control_filter: Option<CommonControlFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CommonControlFilter {
    #[serde(rename = "Objectives")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objectives: Option<Vec<ObjectiveResourceFilter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ObjectiveResourceFilter {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCommonControlsResponse {
    #[serde(rename = "CommonControls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_controls: Option<Vec<CommonControlSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CommonControlSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<AssociatedDomainSummary>,
    #[serde(rename = "LastUpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Objective")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objective: Option<AssociatedObjectiveSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociatedDomainSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociatedObjectiveSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListControlMappingsRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ControlMappingFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ControlMappingFilter {
    #[serde(rename = "CommonControlArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_control_arns: Option<Vec<String>>,
    #[serde(rename = "ControlArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_arns: Option<Vec<String>>,
    #[serde(rename = "MappingTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapping_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListControlMappingsResponse {
    #[serde(rename = "ControlMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_mappings: Option<Vec<ControlMapping>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ControlMapping {
    #[serde(rename = "ControlArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_arn: Option<String>,
    #[serde(rename = "Mapping")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapping: Option<Mapping>,
    #[serde(rename = "MappingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapping_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Mapping {
    #[serde(rename = "CommonControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_control: Option<CommonControlMappingDetails>,
    #[serde(rename = "Framework")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework: Option<FrameworkMappingDetails>,
    #[serde(rename = "RelatedControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_control: Option<RelatedControlMappingDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CommonControlMappingDetails {
    #[serde(rename = "CommonControlArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_control_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FrameworkMappingDetails {
    #[serde(rename = "Item")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RelatedControlMappingDetails {
    #[serde(rename = "ControlArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_arn: Option<String>,
    #[serde(rename = "RelationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListControlsRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ControlFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ControlFilter {
    #[serde(rename = "Implementations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implementations: Option<ImplementationFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImplementationFilter {
    #[serde(rename = "Identifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<Vec<String>>,
    #[serde(rename = "Types")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListControlsResponse {
    #[serde(rename = "Controls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controls: Option<Vec<ControlSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ControlSummary {
    #[serde(rename = "Aliases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Behavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior: Option<String>,
    #[serde(rename = "CreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "GovernedResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub governed_resources: Option<Vec<String>>,
    #[serde(rename = "Implementation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implementation: Option<ImplementationSummary>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Severity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImplementationSummary {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDomainsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDomainsResponse {
    #[serde(rename = "Domains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<DomainSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LastUpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListObjectivesRequest {
    #[serde(rename = "ObjectiveFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objective_filter: Option<ObjectiveFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ObjectiveFilter {
    #[serde(rename = "Domains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<DomainResourceFilter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainResourceFilter {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListObjectivesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Objectives")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objectives: Option<Vec<ObjectiveSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ObjectiveSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<AssociatedDomainSummary>,
    #[serde(rename = "LastUpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
