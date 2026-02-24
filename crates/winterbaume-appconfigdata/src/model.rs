//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-appconfigdata

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLatestConfigurationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLatestConfigurationResponse {
    #[serde(rename = "Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
    #[serde(rename = "ContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "NextPollConfigurationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_poll_configuration_token: Option<String>,
    #[serde(rename = "NextPollIntervalInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_poll_interval_in_seconds: Option<i32>,
    #[serde(rename = "VersionLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_label: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartConfigurationSessionRequest {
    #[serde(rename = "ApplicationIdentifier")]
    #[serde(default)]
    pub application_identifier: String,
    #[serde(rename = "ConfigurationProfileIdentifier")]
    #[serde(default)]
    pub configuration_profile_identifier: String,
    #[serde(rename = "EnvironmentIdentifier")]
    #[serde(default)]
    pub environment_identifier: String,
    #[serde(rename = "RequiredMinimumPollIntervalInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_minimum_poll_interval_in_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartConfigurationSessionResponse {
    #[serde(rename = "InitialConfigurationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_configuration_token: Option<String>,
}
