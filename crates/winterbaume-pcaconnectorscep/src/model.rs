//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-pcaconnectorscep

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateChallengeRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "ConnectorArn")]
    #[serde(default)]
    pub connector_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateChallengeResponse {
    #[serde(rename = "Challenge")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge: Option<Challenge>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Challenge {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ConnectorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_arn: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "Password")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConnectorRequest {
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(default)]
    pub certificate_authority_arn: String,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "MobileDeviceManagement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_device_management: Option<MobileDeviceManagement>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "VpcEndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MobileDeviceManagement {
    #[serde(rename = "Intune")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intune: Option<IntuneConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntuneConfiguration {
    #[serde(rename = "AzureApplicationId")]
    #[serde(default)]
    pub azure_application_id: String,
    #[serde(rename = "Domain")]
    #[serde(default)]
    pub domain: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConnectorResponse {
    #[serde(rename = "ConnectorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteChallengeRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConnectorRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetChallengeMetadataRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetChallengeMetadataResponse {
    #[serde(rename = "ChallengeMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge_metadata: Option<ChallengeMetadata>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChallengeMetadata {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ConnectorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_arn: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetChallengePasswordRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetChallengePasswordResponse {
    #[serde(rename = "Password")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConnectorRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConnectorResponse {
    #[serde(rename = "Connector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector: Option<Connector>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Connector {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority_arn: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "MobileDeviceManagement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_device_management: Option<MobileDeviceManagement>,
    #[serde(rename = "OpenIdConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id_configuration: Option<OpenIdConfiguration>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpenIdConfiguration {
    #[serde(rename = "Audience")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,
    #[serde(rename = "Issuer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(rename = "Subject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListChallengeMetadataRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListChallengeMetadataResponse {
    #[serde(rename = "Challenges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenges: Option<Vec<ChallengeMetadataSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChallengeMetadataSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ConnectorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_arn: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConnectorsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConnectorsResponse {
    #[serde(rename = "Connectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectors: Option<Vec<ConnectorSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectorSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority_arn: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "MobileDeviceManagement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_device_management: Option<MobileDeviceManagement>,
    #[serde(rename = "OpenIdConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id_configuration: Option<OpenIdConfiguration>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {}
