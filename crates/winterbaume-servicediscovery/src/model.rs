//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-servicediscovery

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateHttpNamespaceRequest {
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
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
pub struct CreateHttpNamespaceResponse {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePrivateDnsNamespaceRequest {
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Properties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateDnsNamespaceProperties>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Vpc")]
    #[serde(default)]
    pub vpc: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PrivateDnsNamespaceProperties {
    #[serde(rename = "DnsProperties")]
    #[serde(default)]
    pub dns_properties: PrivateDnsPropertiesMutable,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PrivateDnsPropertiesMutable {
    #[serde(rename = "SOA")]
    #[serde(default)]
    pub s_o_a: SOA,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SOA {
    #[serde(rename = "TTL")]
    #[serde(default)]
    pub t_t_l: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePrivateDnsNamespaceResponse {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePublicDnsNamespaceRequest {
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Properties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PublicDnsNamespaceProperties>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PublicDnsNamespaceProperties {
    #[serde(rename = "DnsProperties")]
    #[serde(default)]
    pub dns_properties: PublicDnsPropertiesMutable,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PublicDnsPropertiesMutable {
    #[serde(rename = "SOA")]
    #[serde(default)]
    pub s_o_a: SOA,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePublicDnsNamespaceResponse {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateServiceRequest {
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DnsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_config: Option<DnsConfig>,
    #[serde(rename = "HealthCheckConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_config: Option<HealthCheckConfig>,
    #[serde(rename = "HealthCheckCustomConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_custom_config: Option<HealthCheckCustomConfig>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "NamespaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DnsConfig {
    #[serde(rename = "DnsRecords")]
    #[serde(default)]
    pub dns_records: Vec<DnsRecord>,
    #[serde(rename = "NamespaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<String>,
    #[serde(rename = "RoutingPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DnsRecord {
    #[serde(rename = "TTL")]
    #[serde(default)]
    pub t_t_l: i64,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HealthCheckConfig {
    #[serde(rename = "FailureThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_threshold: Option<i32>,
    #[serde(rename = "ResourcePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_path: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HealthCheckCustomConfig {
    #[serde(rename = "FailureThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_threshold: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateServiceResponse {
    #[serde(rename = "Service")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Service {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<f64>,
    #[serde(rename = "CreatedByAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_account: Option<String>,
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DnsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_config: Option<DnsConfig>,
    #[serde(rename = "HealthCheckConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_config: Option<HealthCheckConfig>,
    #[serde(rename = "HealthCheckCustomConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_custom_config: Option<HealthCheckCustomConfig>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "InstanceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_count: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NamespaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_id: Option<String>,
    #[serde(rename = "ResourceOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteNamespaceRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteNamespaceResponse {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteServiceAttributesRequest {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    pub attributes: Vec<String>,
    #[serde(rename = "ServiceId")]
    #[serde(default)]
    pub service_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteServiceAttributesResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteServiceRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteServiceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterInstanceRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "ServiceId")]
    #[serde(default)]
    pub service_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterInstanceResponse {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DiscoverInstancesRequest {
    #[serde(rename = "HealthStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_status: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NamespaceName")]
    #[serde(default)]
    pub namespace_name: String,
    #[serde(rename = "OptionalParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "OwnerAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account: Option<String>,
    #[serde(rename = "QueryParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ServiceName")]
    #[serde(default)]
    pub service_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DiscoverInstancesResponse {
    #[serde(rename = "Instances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<HttpInstanceSummary>>,
    #[serde(rename = "InstancesRevision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_revision: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpInstanceSummary {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "HealthStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_status: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "NamespaceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_name: Option<String>,
    #[serde(rename = "ServiceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DiscoverInstancesRevisionRequest {
    #[serde(rename = "NamespaceName")]
    #[serde(default)]
    pub namespace_name: String,
    #[serde(rename = "OwnerAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account: Option<String>,
    #[serde(rename = "ServiceName")]
    #[serde(default)]
    pub service_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DiscoverInstancesRevisionResponse {
    #[serde(rename = "InstancesRevision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_revision: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInstanceRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "ServiceId")]
    #[serde(default)]
    pub service_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInstanceResponse {
    #[serde(rename = "Instance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<Instance>,
    #[serde(rename = "ResourceOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Instance {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "CreatedByAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_account: Option<String>,
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInstancesHealthStatusRequest {
    #[serde(rename = "Instances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<String>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ServiceId")]
    #[serde(default)]
    pub service_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInstancesHealthStatusResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetNamespaceRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetNamespaceResponse {
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<Namespace>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Namespace {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<f64>,
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Properties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<NamespaceProperties>,
    #[serde(rename = "ResourceOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner: Option<String>,
    #[serde(rename = "ServiceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_count: Option<i32>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NamespaceProperties {
    #[serde(rename = "DnsProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_properties: Option<DnsProperties>,
    #[serde(rename = "HttpProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_properties: Option<HttpProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DnsProperties {
    #[serde(rename = "HostedZoneId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id: Option<String>,
    #[serde(rename = "SOA")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_o_a: Option<SOA>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpProperties {
    #[serde(rename = "HttpName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOperationRequest {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    pub operation_id: String,
    #[serde(rename = "OwnerAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOperationResponse {
    #[serde(rename = "Operation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Operation {
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<f64>,
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "OwnerAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "UpdateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetServiceAttributesRequest {
    #[serde(rename = "ServiceId")]
    #[serde(default)]
    pub service_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetServiceAttributesResponse {
    #[serde(rename = "ServiceAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_attributes: Option<ServiceAttributes>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceAttributes {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ResourceOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner: Option<String>,
    #[serde(rename = "ServiceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetServiceRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetServiceResponse {
    #[serde(rename = "Service")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListInstancesRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ServiceId")]
    #[serde(default)]
    pub service_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListInstancesResponse {
    #[serde(rename = "Instances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<InstanceSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceSummary {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "CreatedByAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_account: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListNamespacesRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<NamespaceFilter>>,
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
pub struct NamespaceFilter {
    #[serde(rename = "Condition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListNamespacesResponse {
    #[serde(rename = "Namespaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<NamespaceSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NamespaceSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Properties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<NamespaceProperties>,
    #[serde(rename = "ResourceOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner: Option<String>,
    #[serde(rename = "ServiceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_count: Option<i32>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOperationsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<OperationFilter>>,
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
pub struct OperationFilter {
    #[serde(rename = "Condition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOperationsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Operations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<OperationSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OperationSummary {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServicesRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ServiceFilter>>,
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
pub struct ServiceFilter {
    #[serde(rename = "Condition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServicesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Services")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<ServiceSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<f64>,
    #[serde(rename = "CreatedByAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_account: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DnsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_config: Option<DnsConfig>,
    #[serde(rename = "HealthCheckConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_config: Option<HealthCheckConfig>,
    #[serde(rename = "HealthCheckCustomConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_custom_config: Option<HealthCheckCustomConfig>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "InstanceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_count: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ResourceOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterInstanceRequest {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    pub attributes: std::collections::HashMap<String, String>,
    #[serde(rename = "CreatorRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "ServiceId")]
    #[serde(default)]
    pub service_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterInstanceResponse {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateHttpNamespaceRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: HttpNamespaceChange,
    #[serde(rename = "UpdaterRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updater_request_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpNamespaceChange {
    #[serde(rename = "Description")]
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateHttpNamespaceResponse {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateInstanceCustomHealthStatusRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "ServiceId")]
    #[serde(default)]
    pub service_id: String,
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePrivateDnsNamespaceRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: PrivateDnsNamespaceChange,
    #[serde(rename = "UpdaterRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updater_request_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PrivateDnsNamespaceChange {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Properties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PrivateDnsNamespacePropertiesChange>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PrivateDnsNamespacePropertiesChange {
    #[serde(rename = "DnsProperties")]
    #[serde(default)]
    pub dns_properties: PrivateDnsPropertiesMutableChange,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PrivateDnsPropertiesMutableChange {
    #[serde(rename = "SOA")]
    #[serde(default)]
    pub s_o_a: SOAChange,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SOAChange {
    #[serde(rename = "TTL")]
    #[serde(default)]
    pub t_t_l: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePrivateDnsNamespaceResponse {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePublicDnsNamespaceRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: PublicDnsNamespaceChange,
    #[serde(rename = "UpdaterRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updater_request_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PublicDnsNamespaceChange {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Properties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<PublicDnsNamespacePropertiesChange>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PublicDnsNamespacePropertiesChange {
    #[serde(rename = "DnsProperties")]
    #[serde(default)]
    pub dns_properties: PublicDnsPropertiesMutableChange,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PublicDnsPropertiesMutableChange {
    #[serde(rename = "SOA")]
    #[serde(default)]
    pub s_o_a: SOAChange,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePublicDnsNamespaceResponse {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateServiceAttributesRequest {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    pub attributes: std::collections::HashMap<String, String>,
    #[serde(rename = "ServiceId")]
    #[serde(default)]
    pub service_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateServiceAttributesResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateServiceRequest {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Service")]
    #[serde(default)]
    pub service: ServiceChange,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceChange {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DnsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_config: Option<DnsConfigChange>,
    #[serde(rename = "HealthCheckConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_config: Option<HealthCheckConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DnsConfigChange {
    #[serde(rename = "DnsRecords")]
    #[serde(default)]
    pub dns_records: Vec<DnsRecord>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateServiceResponse {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}
