//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-ec2instanceconnect

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendSerialConsoleSSHPublicKeyRequest {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "SSHPublicKey")]
    #[serde(default)]
    pub s_s_h_public_key: String,
    #[serde(rename = "SerialPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_port: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendSSHPublicKeyResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Success")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendSSHPublicKeyRequest {
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
    #[serde(rename = "InstanceOSUser")]
    #[serde(default)]
    pub instance_o_s_user: String,
    #[serde(rename = "SSHPublicKey")]
    #[serde(default)]
    pub s_s_h_public_key: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendSerialConsoleSSHPublicKeyResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Success")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}
