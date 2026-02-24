//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-s3outposts

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEndpointRequest {
    #[serde(rename = "AccessType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_type: Option<String>,
    #[serde(rename = "CustomerOwnedIpv4Pool")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_owned_ipv4_pool: Option<String>,
    #[serde(rename = "OutpostId")]
    #[serde(default)]
    pub outpost_id: String,
    #[serde(rename = "SecurityGroupId")]
    #[serde(default)]
    pub security_group_id: String,
    #[serde(rename = "SubnetId")]
    #[serde(default)]
    pub subnet_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEndpointResult {
    #[serde(rename = "EndpointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEndpointRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEndpointsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEndpointsResult {
    #[serde(rename = "Endpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<Endpoint>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Endpoint {
    #[serde(rename = "AccessType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_type: Option<String>,
    #[serde(rename = "CidrBlock")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr_block: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "CustomerOwnedIpv4Pool")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_owned_ipv4_pool: Option<String>,
    #[serde(rename = "EndpointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_arn: Option<String>,
    #[serde(rename = "FailedReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_reason: Option<FailedReason>,
    #[serde(rename = "NetworkInterfaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interfaces: Option<Vec<NetworkInterface>>,
    #[serde(rename = "OutpostsId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outposts_id: Option<String>,
    #[serde(rename = "SecurityGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "SubnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailedReason {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkInterface {
    #[serde(rename = "NetworkInterfaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOutpostsWithS3Request {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOutpostsWithS3Result {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Outposts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outposts: Option<Vec<Outpost>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Outpost {
    #[serde(rename = "CapacityInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_in_bytes: Option<i64>,
    #[serde(rename = "OutpostArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_arn: Option<String>,
    #[serde(rename = "OutpostId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_id: Option<String>,
    #[serde(rename = "OwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "S3OutpostArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_outpost_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSharedEndpointsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSharedEndpointsResult {
    #[serde(rename = "Endpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<Endpoint>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}
