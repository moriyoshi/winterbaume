//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-opensearchserverless

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetCollectionGroupRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetCollectionGroupResponse {
    #[serde(rename = "collectionGroupDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_group_details: Option<Vec<CollectionGroupDetail>>,
    #[serde(rename = "collectionGroupErrorDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_group_error_details: Option<Vec<CollectionGroupErrorDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CollectionGroupDetail {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "capacityLimits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_limits: Option<CollectionGroupCapacityLimits>,
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "numberOfCollections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_collections: Option<i32>,
    #[serde(rename = "standbyReplicas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standby_replicas: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CollectionGroupCapacityLimits {
    #[serde(rename = "maxIndexingCapacityInOCU")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_indexing_capacity_in_o_c_u: Option<f32>,
    #[serde(rename = "maxSearchCapacityInOCU")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_search_capacity_in_o_c_u: Option<f32>,
    #[serde(rename = "minIndexingCapacityInOCU")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_indexing_capacity_in_o_c_u: Option<f32>,
    #[serde(rename = "minSearchCapacityInOCU")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_search_capacity_in_o_c_u: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CollectionGroupErrorDetail {
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetCollectionRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetCollectionResponse {
    #[serde(rename = "collectionDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_details: Option<Vec<CollectionDetail>>,
    #[serde(rename = "collectionErrorDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_error_details: Option<Vec<CollectionErrorDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CollectionDetail {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "collectionEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_endpoint: Option<String>,
    #[serde(rename = "collectionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_group_name: Option<String>,
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i64>,
    #[serde(rename = "dashboardEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_endpoint: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "failureCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    #[serde(rename = "failureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    #[serde(rename = "fipsEndpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fips_endpoints: Option<FipsEndpoints>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "kmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "standbyReplicas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standby_replicas: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "vectorOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_options: Option<VectorOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FipsEndpoints {
    #[serde(rename = "collectionEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_endpoint: Option<String>,
    #[serde(rename = "dashboardEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_endpoint: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VectorOptions {
    #[serde(rename = "ServerlessVectorAcceleration")]
    #[serde(default)]
    pub serverless_vector_acceleration: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CollectionErrorDetail {
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetEffectiveLifecyclePolicyRequest {
    #[serde(rename = "resourceIdentifiers")]
    #[serde(default)]
    pub resource_identifiers: Vec<LifecyclePolicyResourceIdentifier>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LifecyclePolicyResourceIdentifier {
    #[serde(default)]
    pub resource: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetEffectiveLifecyclePolicyResponse {
    #[serde(rename = "effectiveLifecyclePolicyDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_lifecycle_policy_details: Option<Vec<EffectiveLifecyclePolicyDetail>>,
    #[serde(rename = "effectiveLifecyclePolicyErrorDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_lifecycle_policy_error_details: Option<Vec<EffectiveLifecyclePolicyErrorDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EffectiveLifecyclePolicyDetail {
    #[serde(rename = "noMinRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_min_retention_period: Option<bool>,
    #[serde(rename = "policyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "retentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EffectiveLifecyclePolicyErrorDetail {
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetLifecyclePolicyRequest {
    #[serde(default)]
    pub identifiers: Vec<LifecyclePolicyIdentifier>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LifecyclePolicyIdentifier {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetLifecyclePolicyResponse {
    #[serde(rename = "lifecyclePolicyDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy_details: Option<Vec<LifecyclePolicyDetail>>,
    #[serde(rename = "lifecyclePolicyErrorDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy_error_details: Option<Vec<LifecyclePolicyErrorDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LifecyclePolicyDetail {
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<serde_json::Value>,
    #[serde(rename = "policyVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_version: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LifecyclePolicyErrorDetail {
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetVpcEndpointRequest {
    #[serde(default)]
    pub ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetVpcEndpointResponse {
    #[serde(rename = "vpcEndpointDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_details: Option<Vec<VpcEndpointDetail>>,
    #[serde(rename = "vpcEndpointErrorDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_error_details: Option<Vec<VpcEndpointErrorDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcEndpointDetail {
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i64>,
    #[serde(rename = "failureCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    #[serde(rename = "failureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "subnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(rename = "vpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcEndpointErrorDetail {
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAccessPolicyRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub policy: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAccessPolicyResponse {
    #[serde(rename = "accessPolicyDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_policy_detail: Option<AccessPolicyDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessPolicyDetail {
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<serde_json::Value>,
    #[serde(rename = "policyVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_version: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCollectionGroupRequest {
    #[serde(rename = "capacityLimits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_limits: Option<CollectionGroupCapacityLimits>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "standbyReplicas")]
    #[serde(default)]
    pub standby_replicas: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCollectionGroupResponse {
    #[serde(rename = "createCollectionGroupDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_collection_group_detail: Option<CreateCollectionGroupDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCollectionGroupDetail {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "capacityLimits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_limits: Option<CollectionGroupCapacityLimits>,
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "standbyReplicas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standby_replicas: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCollectionRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "collectionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_group_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "encryptionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_config: Option<EncryptionConfig>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "standbyReplicas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standby_replicas: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "vectorOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_options: Option<VectorOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptionConfig {
    #[serde(rename = "aWSOwnedKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_w_s_owned_key: Option<bool>,
    #[serde(rename = "kmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCollectionResponse {
    #[serde(rename = "createCollectionDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_collection_detail: Option<CreateCollectionDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCollectionDetail {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "collectionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_group_name: Option<String>,
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "kmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "standbyReplicas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standby_replicas: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "vectorOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_options: Option<VectorOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIndexRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "indexName")]
    #[serde(default)]
    pub index_name: String,
    #[serde(rename = "indexSchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_schema: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIndexResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLifecyclePolicyRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub policy: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLifecyclePolicyResponse {
    #[serde(rename = "lifecyclePolicyDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy_detail: Option<LifecyclePolicyDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSecurityConfigRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "iamFederationOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_federation_options: Option<IamFederationConfigOptions>,
    #[serde(rename = "iamIdentityCenterOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_identity_center_options: Option<CreateIamIdentityCenterConfigOptions>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "samlOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saml_options: Option<SamlConfigOptions>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IamFederationConfigOptions {
    #[serde(rename = "groupAttribute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_attribute: Option<String>,
    #[serde(rename = "userAttribute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_attribute: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIamIdentityCenterConfigOptions {
    #[serde(rename = "groupAttribute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_attribute: Option<String>,
    #[serde(rename = "instanceArn")]
    #[serde(default)]
    pub instance_arn: String,
    #[serde(rename = "userAttribute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_attribute: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SamlConfigOptions {
    #[serde(rename = "groupAttribute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_attribute: Option<String>,
    #[serde(default)]
    pub metadata: String,
    #[serde(rename = "openSearchServerlessEntityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_search_serverless_entity_id: Option<String>,
    #[serde(rename = "sessionTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_timeout: Option<i32>,
    #[serde(rename = "userAttribute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_attribute: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSecurityConfigResponse {
    #[serde(rename = "securityConfigDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_config_detail: Option<SecurityConfigDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecurityConfigDetail {
    #[serde(rename = "configVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_version: Option<String>,
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "iamFederationOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_federation_options: Option<IamFederationConfigOptions>,
    #[serde(rename = "iamIdentityCenterOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_identity_center_options: Option<IamIdentityCenterConfigOptions>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<i64>,
    #[serde(rename = "samlOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saml_options: Option<SamlConfigOptions>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IamIdentityCenterConfigOptions {
    #[serde(rename = "applicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
    #[serde(rename = "applicationDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_description: Option<String>,
    #[serde(rename = "applicationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
    #[serde(rename = "groupAttribute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_attribute: Option<String>,
    #[serde(rename = "instanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_arn: Option<String>,
    #[serde(rename = "userAttribute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_attribute: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSecurityPolicyRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub policy: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSecurityPolicyResponse {
    #[serde(rename = "securityPolicyDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_policy_detail: Option<SecurityPolicyDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecurityPolicyDetail {
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<serde_json::Value>,
    #[serde(rename = "policyVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_version: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVpcEndpointRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "subnetIds")]
    #[serde(default)]
    pub subnet_ids: Vec<String>,
    #[serde(rename = "vpcId")]
    #[serde(default)]
    pub vpc_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVpcEndpointResponse {
    #[serde(rename = "createVpcEndpointDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_vpc_endpoint_detail: Option<CreateVpcEndpointDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVpcEndpointDetail {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAccessPolicyRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAccessPolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCollectionGroupRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCollectionGroupResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCollectionRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCollectionResponse {
    #[serde(rename = "deleteCollectionDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_collection_detail: Option<DeleteCollectionDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCollectionDetail {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIndexRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "indexName")]
    #[serde(default)]
    pub index_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIndexResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLifecyclePolicyRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLifecyclePolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSecurityConfigRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSecurityConfigResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSecurityPolicyRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSecurityPolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVpcEndpointRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVpcEndpointResponse {
    #[serde(rename = "deleteVpcEndpointDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_vpc_endpoint_detail: Option<DeleteVpcEndpointDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVpcEndpointDetail {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccessPolicyRequest {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccessPolicyResponse {
    #[serde(rename = "accessPolicyDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_policy_detail: Option<AccessPolicyDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccountSettingsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccountSettingsResponse {
    #[serde(rename = "accountSettingsDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_settings_detail: Option<AccountSettingsDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountSettingsDetail {
    #[serde(rename = "capacityLimits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_limits: Option<CapacityLimits>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CapacityLimits {
    #[serde(rename = "maxIndexingCapacityInOCU")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_indexing_capacity_in_o_c_u: Option<i32>,
    #[serde(rename = "maxSearchCapacityInOCU")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_search_capacity_in_o_c_u: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIndexRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "indexName")]
    #[serde(default)]
    pub index_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIndexResponse {
    #[serde(rename = "indexSchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_schema: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPoliciesStatsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPoliciesStatsResponse {
    #[serde(rename = "AccessPolicyStats")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_policy_stats: Option<AccessPolicyStats>,
    #[serde(rename = "LifecyclePolicyStats")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy_stats: Option<LifecyclePolicyStats>,
    #[serde(rename = "SecurityConfigStats")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_config_stats: Option<SecurityConfigStats>,
    #[serde(rename = "SecurityPolicyStats")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_policy_stats: Option<SecurityPolicyStats>,
    #[serde(rename = "TotalPolicyCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_policy_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessPolicyStats {
    #[serde(rename = "DataPolicyCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_policy_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LifecyclePolicyStats {
    #[serde(rename = "RetentionPolicyCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_policy_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecurityConfigStats {
    #[serde(rename = "SamlConfigCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saml_config_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecurityPolicyStats {
    #[serde(rename = "EncryptionPolicyCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_policy_count: Option<i64>,
    #[serde(rename = "NetworkPolicyCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_policy_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSecurityConfigRequest {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSecurityConfigResponse {
    #[serde(rename = "securityConfigDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_config_detail: Option<SecurityConfigDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSecurityPolicyRequest {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSecurityPolicyResponse {
    #[serde(rename = "securityPolicyDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_policy_detail: Option<SecurityPolicyDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccessPoliciesRequest {
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
    pub resource: Option<Vec<String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAccessPoliciesResponse {
    #[serde(rename = "accessPolicySummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_policy_summaries: Option<Vec<AccessPolicySummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessPolicySummary {
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "policyVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_version: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCollectionGroupsRequest {
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
pub struct ListCollectionGroupsResponse {
    #[serde(rename = "collectionGroupSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_group_summaries: Option<Vec<CollectionGroupSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CollectionGroupSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "capacityLimits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_limits: Option<CollectionGroupCapacityLimits>,
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "numberOfCollections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_collections: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCollectionsRequest {
    #[serde(rename = "collectionFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_filters: Option<CollectionFilters>,
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
pub struct CollectionFilters {
    #[serde(rename = "collectionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_group_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCollectionsResponse {
    #[serde(rename = "collectionSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_summaries: Option<Vec<CollectionSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CollectionSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "collectionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_group_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "kmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLifecyclePoliciesRequest {
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
    pub resources: Option<Vec<String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLifecyclePoliciesResponse {
    #[serde(rename = "lifecyclePolicySummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy_summaries: Option<Vec<LifecyclePolicySummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LifecyclePolicySummary {
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "policyVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_version: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSecurityConfigsRequest {
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
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSecurityConfigsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "securityConfigSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_config_summaries: Option<Vec<SecurityConfigSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecurityConfigSummary {
    #[serde(rename = "configVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_version: Option<String>,
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<i64>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSecurityPoliciesRequest {
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
    pub resource: Option<Vec<String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSecurityPoliciesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "securityPolicySummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_policy_summaries: Option<Vec<SecurityPolicySummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecurityPolicySummary {
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "policyVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_version: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
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
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVpcEndpointsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "vpcEndpointFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_filters: Option<VpcEndpointFilters>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcEndpointFilters {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVpcEndpointsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "vpcEndpointSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_summaries: Option<Vec<VpcEndpointSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcEndpointSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    pub tags: Vec<Tag>,
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
pub struct UpdateAccessPolicyRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "policyVersion")]
    #[serde(default)]
    pub policy_version: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAccessPolicyResponse {
    #[serde(rename = "accessPolicyDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_policy_detail: Option<AccessPolicyDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAccountSettingsRequest {
    #[serde(rename = "capacityLimits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_limits: Option<CapacityLimits>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAccountSettingsResponse {
    #[serde(rename = "accountSettingsDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_settings_detail: Option<AccountSettingsDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCollectionGroupRequest {
    #[serde(rename = "capacityLimits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_limits: Option<CollectionGroupCapacityLimits>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCollectionGroupResponse {
    #[serde(rename = "updateCollectionGroupDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_collection_group_detail: Option<UpdateCollectionGroupDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCollectionGroupDetail {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "capacityLimits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_limits: Option<CollectionGroupCapacityLimits>,
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCollectionRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "vectorOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_options: Option<VectorOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCollectionResponse {
    #[serde(rename = "updateCollectionDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_collection_detail: Option<UpdateCollectionDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCollectionDetail {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "vectorOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_options: Option<VectorOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIndexRequest {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "indexName")]
    #[serde(default)]
    pub index_name: String,
    #[serde(rename = "indexSchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_schema: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIndexResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLifecyclePolicyRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "policyVersion")]
    #[serde(default)]
    pub policy_version: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLifecyclePolicyResponse {
    #[serde(rename = "lifecyclePolicyDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_policy_detail: Option<LifecyclePolicyDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSecurityConfigRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "configVersion")]
    #[serde(default)]
    pub config_version: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "iamFederationOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_federation_options: Option<IamFederationConfigOptions>,
    #[serde(rename = "iamIdentityCenterOptionsUpdates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_identity_center_options_updates: Option<UpdateIamIdentityCenterConfigOptions>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "samlOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saml_options: Option<SamlConfigOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIamIdentityCenterConfigOptions {
    #[serde(rename = "groupAttribute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_attribute: Option<String>,
    #[serde(rename = "userAttribute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_attribute: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSecurityConfigResponse {
    #[serde(rename = "securityConfigDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_config_detail: Option<SecurityConfigDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSecurityPolicyRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "policyVersion")]
    #[serde(default)]
    pub policy_version: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSecurityPolicyResponse {
    #[serde(rename = "securityPolicyDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_policy_detail: Option<SecurityPolicyDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateVpcEndpointRequest {
    #[serde(rename = "addSecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_security_group_ids: Option<Vec<String>>,
    #[serde(rename = "addSubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_subnet_ids: Option<Vec<String>>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "removeSecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_security_group_ids: Option<Vec<String>>,
    #[serde(rename = "removeSubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_subnet_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateVpcEndpointResponse {
    #[serde(rename = "UpdateVpcEndpointDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_vpc_endpoint_detail: Option<UpdateVpcEndpointDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateVpcEndpointDetail {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "subnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
}
