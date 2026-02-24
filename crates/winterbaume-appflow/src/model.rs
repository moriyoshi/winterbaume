//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-appflow

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelFlowExecutionsRequest {
    #[serde(rename = "executionIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_ids: Option<Vec<String>>,
    #[serde(rename = "flowName")]
    #[serde(default)]
    pub flow_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelFlowExecutionsResponse {
    #[serde(rename = "invalidExecutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_executions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConnectorProfileRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "connectionMode")]
    #[serde(default)]
    pub connection_mode: String,
    #[serde(rename = "connectorLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_label: Option<String>,
    #[serde(rename = "connectorProfileConfig")]
    #[serde(default)]
    pub connector_profile_config: ConnectorProfileConfig,
    #[serde(rename = "connectorProfileName")]
    #[serde(default)]
    pub connector_profile_name: String,
    #[serde(rename = "connectorType")]
    #[serde(default)]
    pub connector_type: String,
    #[serde(rename = "kmsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectorProfileConfig {
    #[serde(rename = "connectorProfileCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_profile_credentials: Option<ConnectorProfileCredentials>,
    #[serde(rename = "connectorProfileProperties")]
    #[serde(default)]
    pub connector_profile_properties: ConnectorProfileProperties,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectorProfileCredentials {
    #[serde(rename = "Amplitude")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amplitude: Option<AmplitudeConnectorProfileCredentials>,
    #[serde(rename = "CustomConnector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_connector: Option<CustomConnectorProfileCredentials>,
    #[serde(rename = "Datadog")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datadog: Option<DatadogConnectorProfileCredentials>,
    #[serde(rename = "Dynatrace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynatrace: Option<DynatraceConnectorProfileCredentials>,
    #[serde(rename = "GoogleAnalytics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_analytics: Option<GoogleAnalyticsConnectorProfileCredentials>,
    #[serde(rename = "Honeycode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honeycode: Option<HoneycodeConnectorProfileCredentials>,
    #[serde(rename = "InforNexus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infor_nexus: Option<InforNexusConnectorProfileCredentials>,
    #[serde(rename = "Marketo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketo: Option<MarketoConnectorProfileCredentials>,
    #[serde(rename = "Pardot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pardot: Option<PardotConnectorProfileCredentials>,
    #[serde(rename = "Redshift")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift: Option<RedshiftConnectorProfileCredentials>,
    #[serde(rename = "SAPOData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_a_p_o_data: Option<SAPODataConnectorProfileCredentials>,
    #[serde(rename = "Salesforce")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salesforce: Option<SalesforceConnectorProfileCredentials>,
    #[serde(rename = "ServiceNow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_now: Option<ServiceNowConnectorProfileCredentials>,
    #[serde(rename = "Singular")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub singular: Option<SingularConnectorProfileCredentials>,
    #[serde(rename = "Slack")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slack: Option<SlackConnectorProfileCredentials>,
    #[serde(rename = "Snowflake")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowflake: Option<SnowflakeConnectorProfileCredentials>,
    #[serde(rename = "Trendmicro")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trendmicro: Option<TrendmicroConnectorProfileCredentials>,
    #[serde(rename = "Veeva")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub veeva: Option<VeevaConnectorProfileCredentials>,
    #[serde(rename = "Zendesk")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zendesk: Option<ZendeskConnectorProfileCredentials>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AmplitudeConnectorProfileCredentials {
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    #[serde(rename = "secretKey")]
    #[serde(default)]
    pub secret_key: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomConnectorProfileCredentials {
    #[serde(rename = "apiKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<ApiKeyCredentials>,
    #[serde(rename = "authenticationType")]
    #[serde(default)]
    pub authentication_type: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic: Option<BasicAuthCredentials>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<CustomAuthCredentials>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth2: Option<OAuth2Credentials>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApiKeyCredentials {
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    #[serde(rename = "apiSecretKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_secret_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BasicAuthCredentials {
    #[serde(default)]
    pub password: String,
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomAuthCredentials {
    #[serde(rename = "credentialsMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials_map: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "customAuthenticationType")]
    #[serde(default)]
    pub custom_authentication_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OAuth2Credentials {
    #[serde(rename = "accessToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "clientId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "clientSecret")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(rename = "oAuthRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth_request: Option<ConnectorOAuthRequest>,
    #[serde(rename = "refreshToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectorOAuthRequest {
    #[serde(rename = "authCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_code: Option<String>,
    #[serde(rename = "redirectUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatadogConnectorProfileCredentials {
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    #[serde(rename = "applicationKey")]
    #[serde(default)]
    pub application_key: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DynatraceConnectorProfileCredentials {
    #[serde(rename = "apiToken")]
    #[serde(default)]
    pub api_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GoogleAnalyticsConnectorProfileCredentials {
    #[serde(rename = "accessToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "clientId")]
    #[serde(default)]
    pub client_id: String,
    #[serde(rename = "clientSecret")]
    #[serde(default)]
    pub client_secret: String,
    #[serde(rename = "oAuthRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth_request: Option<ConnectorOAuthRequest>,
    #[serde(rename = "refreshToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HoneycodeConnectorProfileCredentials {
    #[serde(rename = "accessToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "oAuthRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth_request: Option<ConnectorOAuthRequest>,
    #[serde(rename = "refreshToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InforNexusConnectorProfileCredentials {
    #[serde(rename = "accessKeyId")]
    #[serde(default)]
    pub access_key_id: String,
    #[serde(default)]
    pub datakey: String,
    #[serde(rename = "secretAccessKey")]
    #[serde(default)]
    pub secret_access_key: String,
    #[serde(rename = "userId")]
    #[serde(default)]
    pub user_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MarketoConnectorProfileCredentials {
    #[serde(rename = "accessToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "clientId")]
    #[serde(default)]
    pub client_id: String,
    #[serde(rename = "clientSecret")]
    #[serde(default)]
    pub client_secret: String,
    #[serde(rename = "oAuthRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth_request: Option<ConnectorOAuthRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PardotConnectorProfileCredentials {
    #[serde(rename = "accessToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "clientCredentialsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_credentials_arn: Option<String>,
    #[serde(rename = "oAuthRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth_request: Option<ConnectorOAuthRequest>,
    #[serde(rename = "refreshToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RedshiftConnectorProfileCredentials {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SAPODataConnectorProfileCredentials {
    #[serde(rename = "basicAuthCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth_credentials: Option<BasicAuthCredentials>,
    #[serde(rename = "oAuthCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth_credentials: Option<OAuthCredentials>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OAuthCredentials {
    #[serde(rename = "accessToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "clientId")]
    #[serde(default)]
    pub client_id: String,
    #[serde(rename = "clientSecret")]
    #[serde(default)]
    pub client_secret: String,
    #[serde(rename = "oAuthRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth_request: Option<ConnectorOAuthRequest>,
    #[serde(rename = "refreshToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SalesforceConnectorProfileCredentials {
    #[serde(rename = "accessToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "clientCredentialsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_credentials_arn: Option<String>,
    #[serde(rename = "jwtToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt_token: Option<String>,
    #[serde(rename = "oAuth2GrantType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth2_grant_type: Option<String>,
    #[serde(rename = "oAuthRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth_request: Option<ConnectorOAuthRequest>,
    #[serde(rename = "refreshToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceNowConnectorProfileCredentials {
    #[serde(rename = "oAuth2Credentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth2_credentials: Option<OAuth2Credentials>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SingularConnectorProfileCredentials {
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SlackConnectorProfileCredentials {
    #[serde(rename = "accessToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "clientId")]
    #[serde(default)]
    pub client_id: String,
    #[serde(rename = "clientSecret")]
    #[serde(default)]
    pub client_secret: String,
    #[serde(rename = "oAuthRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth_request: Option<ConnectorOAuthRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnowflakeConnectorProfileCredentials {
    #[serde(default)]
    pub password: String,
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrendmicroConnectorProfileCredentials {
    #[serde(rename = "apiSecretKey")]
    #[serde(default)]
    pub api_secret_key: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VeevaConnectorProfileCredentials {
    #[serde(default)]
    pub password: String,
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ZendeskConnectorProfileCredentials {
    #[serde(rename = "accessToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "clientId")]
    #[serde(default)]
    pub client_id: String,
    #[serde(rename = "clientSecret")]
    #[serde(default)]
    pub client_secret: String,
    #[serde(rename = "oAuthRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth_request: Option<ConnectorOAuthRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectorProfileProperties {
    #[serde(rename = "Amplitude")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amplitude: Option<AmplitudeConnectorProfileProperties>,
    #[serde(rename = "CustomConnector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_connector: Option<CustomConnectorProfileProperties>,
    #[serde(rename = "Datadog")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datadog: Option<DatadogConnectorProfileProperties>,
    #[serde(rename = "Dynatrace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynatrace: Option<DynatraceConnectorProfileProperties>,
    #[serde(rename = "GoogleAnalytics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_analytics: Option<GoogleAnalyticsConnectorProfileProperties>,
    #[serde(rename = "Honeycode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honeycode: Option<HoneycodeConnectorProfileProperties>,
    #[serde(rename = "InforNexus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infor_nexus: Option<InforNexusConnectorProfileProperties>,
    #[serde(rename = "Marketo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketo: Option<MarketoConnectorProfileProperties>,
    #[serde(rename = "Pardot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pardot: Option<PardotConnectorProfileProperties>,
    #[serde(rename = "Redshift")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift: Option<RedshiftConnectorProfileProperties>,
    #[serde(rename = "SAPOData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_a_p_o_data: Option<SAPODataConnectorProfileProperties>,
    #[serde(rename = "Salesforce")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salesforce: Option<SalesforceConnectorProfileProperties>,
    #[serde(rename = "ServiceNow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_now: Option<ServiceNowConnectorProfileProperties>,
    #[serde(rename = "Singular")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub singular: Option<SingularConnectorProfileProperties>,
    #[serde(rename = "Slack")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slack: Option<SlackConnectorProfileProperties>,
    #[serde(rename = "Snowflake")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowflake: Option<SnowflakeConnectorProfileProperties>,
    #[serde(rename = "Trendmicro")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trendmicro: Option<TrendmicroConnectorProfileProperties>,
    #[serde(rename = "Veeva")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub veeva: Option<VeevaConnectorProfileProperties>,
    #[serde(rename = "Zendesk")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zendesk: Option<ZendeskConnectorProfileProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AmplitudeConnectorProfileProperties {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomConnectorProfileProperties {
    #[serde(rename = "oAuth2Properties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth2_properties: Option<OAuth2Properties>,
    #[serde(rename = "profileProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_properties: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OAuth2Properties {
    #[serde(rename = "oAuth2GrantType")]
    #[serde(default)]
    pub o_auth2_grant_type: String,
    #[serde(rename = "tokenUrl")]
    #[serde(default)]
    pub token_url: String,
    #[serde(rename = "tokenUrlCustomProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_url_custom_properties: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatadogConnectorProfileProperties {
    #[serde(rename = "instanceUrl")]
    #[serde(default)]
    pub instance_url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DynatraceConnectorProfileProperties {
    #[serde(rename = "instanceUrl")]
    #[serde(default)]
    pub instance_url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GoogleAnalyticsConnectorProfileProperties {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HoneycodeConnectorProfileProperties {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InforNexusConnectorProfileProperties {
    #[serde(rename = "instanceUrl")]
    #[serde(default)]
    pub instance_url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MarketoConnectorProfileProperties {
    #[serde(rename = "instanceUrl")]
    #[serde(default)]
    pub instance_url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PardotConnectorProfileProperties {
    #[serde(rename = "businessUnitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_unit_id: Option<String>,
    #[serde(rename = "instanceUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_url: Option<String>,
    #[serde(rename = "isSandboxEnvironment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_sandbox_environment: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RedshiftConnectorProfileProperties {
    #[serde(rename = "bucketName")]
    #[serde(default)]
    pub bucket_name: String,
    #[serde(rename = "bucketPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_prefix: Option<String>,
    #[serde(rename = "clusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "dataApiRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_api_role_arn: Option<String>,
    #[serde(rename = "databaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "databaseUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_url: Option<String>,
    #[serde(rename = "isRedshiftServerless")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_redshift_serverless: Option<bool>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "workgroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workgroup_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SAPODataConnectorProfileProperties {
    #[serde(rename = "applicationHostUrl")]
    #[serde(default)]
    pub application_host_url: String,
    #[serde(rename = "applicationServicePath")]
    #[serde(default)]
    pub application_service_path: String,
    #[serde(rename = "clientNumber")]
    #[serde(default)]
    pub client_number: String,
    #[serde(rename = "disableSSO")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_s_s_o: Option<bool>,
    #[serde(rename = "logonLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logon_language: Option<String>,
    #[serde(rename = "oAuthProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth_properties: Option<OAuthProperties>,
    #[serde(rename = "portNumber")]
    #[serde(default)]
    pub port_number: i32,
    #[serde(rename = "privateLinkServiceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_link_service_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OAuthProperties {
    #[serde(rename = "authCodeUrl")]
    #[serde(default)]
    pub auth_code_url: String,
    #[serde(rename = "oAuthScopes")]
    #[serde(default)]
    pub o_auth_scopes: Vec<String>,
    #[serde(rename = "tokenUrl")]
    #[serde(default)]
    pub token_url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SalesforceConnectorProfileProperties {
    #[serde(rename = "instanceUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_url: Option<String>,
    #[serde(rename = "isSandboxEnvironment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_sandbox_environment: Option<bool>,
    #[serde(rename = "usePrivateLinkForMetadataAndAuthorization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_private_link_for_metadata_and_authorization: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceNowConnectorProfileProperties {
    #[serde(rename = "instanceUrl")]
    #[serde(default)]
    pub instance_url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SingularConnectorProfileProperties {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SlackConnectorProfileProperties {
    #[serde(rename = "instanceUrl")]
    #[serde(default)]
    pub instance_url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnowflakeConnectorProfileProperties {
    #[serde(rename = "accountName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[serde(rename = "bucketName")]
    #[serde(default)]
    pub bucket_name: String,
    #[serde(rename = "bucketPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_prefix: Option<String>,
    #[serde(rename = "privateLinkServiceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_link_service_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(default)]
    pub stage: String,
    #[serde(default)]
    pub warehouse: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrendmicroConnectorProfileProperties {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VeevaConnectorProfileProperties {
    #[serde(rename = "instanceUrl")]
    #[serde(default)]
    pub instance_url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ZendeskConnectorProfileProperties {
    #[serde(rename = "instanceUrl")]
    #[serde(default)]
    pub instance_url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConnectorProfileResponse {
    #[serde(rename = "connectorProfileArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_profile_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFlowRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "destinationFlowConfigList")]
    #[serde(default)]
    pub destination_flow_config_list: Vec<DestinationFlowConfig>,
    #[serde(rename = "flowName")]
    #[serde(default)]
    pub flow_name: String,
    #[serde(rename = "kmsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_arn: Option<String>,
    #[serde(rename = "metadataCatalogConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_catalog_config: Option<MetadataCatalogConfig>,
    #[serde(rename = "sourceFlowConfig")]
    #[serde(default)]
    pub source_flow_config: SourceFlowConfig,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    pub tasks: Vec<Task>,
    #[serde(rename = "triggerConfig")]
    #[serde(default)]
    pub trigger_config: TriggerConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DestinationFlowConfig {
    #[serde(rename = "apiVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    #[serde(rename = "connectorProfileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_profile_name: Option<String>,
    #[serde(rename = "connectorType")]
    #[serde(default)]
    pub connector_type: String,
    #[serde(rename = "destinationConnectorProperties")]
    #[serde(default)]
    pub destination_connector_properties: DestinationConnectorProperties,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DestinationConnectorProperties {
    #[serde(rename = "CustomConnector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_connector: Option<CustomConnectorDestinationProperties>,
    #[serde(rename = "CustomerProfiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_profiles: Option<CustomerProfilesDestinationProperties>,
    #[serde(rename = "EventBridge")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bridge: Option<EventBridgeDestinationProperties>,
    #[serde(rename = "Honeycode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honeycode: Option<HoneycodeDestinationProperties>,
    #[serde(rename = "LookoutMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookout_metrics: Option<LookoutMetricsDestinationProperties>,
    #[serde(rename = "Marketo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketo: Option<MarketoDestinationProperties>,
    #[serde(rename = "Redshift")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift: Option<RedshiftDestinationProperties>,
    #[serde(rename = "S3")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3: Option<S3DestinationProperties>,
    #[serde(rename = "SAPOData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_a_p_o_data: Option<SAPODataDestinationProperties>,
    #[serde(rename = "Salesforce")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salesforce: Option<SalesforceDestinationProperties>,
    #[serde(rename = "Snowflake")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowflake: Option<SnowflakeDestinationProperties>,
    #[serde(rename = "Upsolver")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upsolver: Option<UpsolverDestinationProperties>,
    #[serde(rename = "Zendesk")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zendesk: Option<ZendeskDestinationProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomConnectorDestinationProperties {
    #[serde(rename = "customProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "entityName")]
    #[serde(default)]
    pub entity_name: String,
    #[serde(rename = "errorHandlingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_handling_config: Option<ErrorHandlingConfig>,
    #[serde(rename = "idFieldNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_field_names: Option<Vec<String>>,
    #[serde(rename = "writeOperationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_operation_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ErrorHandlingConfig {
    #[serde(rename = "bucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    #[serde(rename = "bucketPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_prefix: Option<String>,
    #[serde(rename = "failOnFirstDestinationError")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_on_first_destination_error: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomerProfilesDestinationProperties {
    #[serde(rename = "domainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "objectTypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_type_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventBridgeDestinationProperties {
    #[serde(rename = "errorHandlingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_handling_config: Option<ErrorHandlingConfig>,
    #[serde(default)]
    pub object: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HoneycodeDestinationProperties {
    #[serde(rename = "errorHandlingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_handling_config: Option<ErrorHandlingConfig>,
    #[serde(default)]
    pub object: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LookoutMetricsDestinationProperties {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MarketoDestinationProperties {
    #[serde(rename = "errorHandlingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_handling_config: Option<ErrorHandlingConfig>,
    #[serde(default)]
    pub object: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RedshiftDestinationProperties {
    #[serde(rename = "bucketPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_prefix: Option<String>,
    #[serde(rename = "errorHandlingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_handling_config: Option<ErrorHandlingConfig>,
    #[serde(rename = "intermediateBucketName")]
    #[serde(default)]
    pub intermediate_bucket_name: String,
    #[serde(default)]
    pub object: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3DestinationProperties {
    #[serde(rename = "bucketName")]
    #[serde(default)]
    pub bucket_name: String,
    #[serde(rename = "bucketPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_prefix: Option<String>,
    #[serde(rename = "s3OutputFormatConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_output_format_config: Option<S3OutputFormatConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3OutputFormatConfig {
    #[serde(rename = "aggregationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_config: Option<AggregationConfig>,
    #[serde(rename = "fileType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    #[serde(rename = "prefixConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_config: Option<PrefixConfig>,
    #[serde(rename = "preserveSourceDataTyping")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_source_data_typing: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AggregationConfig {
    #[serde(rename = "aggregationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[serde(rename = "targetFileSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_file_size: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PrefixConfig {
    #[serde(rename = "pathPrefixHierarchy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_prefix_hierarchy: Option<Vec<String>>,
    #[serde(rename = "prefixFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_format: Option<String>,
    #[serde(rename = "prefixType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SAPODataDestinationProperties {
    #[serde(rename = "errorHandlingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_handling_config: Option<ErrorHandlingConfig>,
    #[serde(rename = "idFieldNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_field_names: Option<Vec<String>>,
    #[serde(rename = "objectPath")]
    #[serde(default)]
    pub object_path: String,
    #[serde(rename = "successResponseHandlingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_response_handling_config: Option<SuccessResponseHandlingConfig>,
    #[serde(rename = "writeOperationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_operation_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SuccessResponseHandlingConfig {
    #[serde(rename = "bucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    #[serde(rename = "bucketPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SalesforceDestinationProperties {
    #[serde(rename = "dataTransferApi")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_transfer_api: Option<String>,
    #[serde(rename = "errorHandlingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_handling_config: Option<ErrorHandlingConfig>,
    #[serde(rename = "idFieldNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_field_names: Option<Vec<String>>,
    #[serde(default)]
    pub object: String,
    #[serde(rename = "writeOperationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_operation_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnowflakeDestinationProperties {
    #[serde(rename = "bucketPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_prefix: Option<String>,
    #[serde(rename = "errorHandlingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_handling_config: Option<ErrorHandlingConfig>,
    #[serde(rename = "intermediateBucketName")]
    #[serde(default)]
    pub intermediate_bucket_name: String,
    #[serde(default)]
    pub object: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpsolverDestinationProperties {
    #[serde(rename = "bucketName")]
    #[serde(default)]
    pub bucket_name: String,
    #[serde(rename = "bucketPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_prefix: Option<String>,
    #[serde(rename = "s3OutputFormatConfig")]
    #[serde(default)]
    pub s3_output_format_config: UpsolverS3OutputFormatConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpsolverS3OutputFormatConfig {
    #[serde(rename = "aggregationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_config: Option<AggregationConfig>,
    #[serde(rename = "fileType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    #[serde(rename = "prefixConfig")]
    #[serde(default)]
    pub prefix_config: PrefixConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ZendeskDestinationProperties {
    #[serde(rename = "errorHandlingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_handling_config: Option<ErrorHandlingConfig>,
    #[serde(rename = "idFieldNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_field_names: Option<Vec<String>>,
    #[serde(default)]
    pub object: String,
    #[serde(rename = "writeOperationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_operation_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetadataCatalogConfig {
    #[serde(rename = "glueDataCatalog")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glue_data_catalog: Option<GlueDataCatalogConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlueDataCatalogConfig {
    #[serde(rename = "databaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "tablePrefix")]
    #[serde(default)]
    pub table_prefix: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceFlowConfig {
    #[serde(rename = "apiVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    #[serde(rename = "connectorProfileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_profile_name: Option<String>,
    #[serde(rename = "connectorType")]
    #[serde(default)]
    pub connector_type: String,
    #[serde(rename = "incrementalPullConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incremental_pull_config: Option<IncrementalPullConfig>,
    #[serde(rename = "sourceConnectorProperties")]
    #[serde(default)]
    pub source_connector_properties: SourceConnectorProperties,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IncrementalPullConfig {
    #[serde(rename = "datetimeTypeFieldName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datetime_type_field_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceConnectorProperties {
    #[serde(rename = "Amplitude")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amplitude: Option<AmplitudeSourceProperties>,
    #[serde(rename = "CustomConnector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_connector: Option<CustomConnectorSourceProperties>,
    #[serde(rename = "Datadog")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datadog: Option<DatadogSourceProperties>,
    #[serde(rename = "Dynatrace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynatrace: Option<DynatraceSourceProperties>,
    #[serde(rename = "GoogleAnalytics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_analytics: Option<GoogleAnalyticsSourceProperties>,
    #[serde(rename = "InforNexus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infor_nexus: Option<InforNexusSourceProperties>,
    #[serde(rename = "Marketo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketo: Option<MarketoSourceProperties>,
    #[serde(rename = "Pardot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pardot: Option<PardotSourceProperties>,
    #[serde(rename = "S3")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3: Option<S3SourceProperties>,
    #[serde(rename = "SAPOData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_a_p_o_data: Option<SAPODataSourceProperties>,
    #[serde(rename = "Salesforce")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salesforce: Option<SalesforceSourceProperties>,
    #[serde(rename = "ServiceNow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_now: Option<ServiceNowSourceProperties>,
    #[serde(rename = "Singular")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub singular: Option<SingularSourceProperties>,
    #[serde(rename = "Slack")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slack: Option<SlackSourceProperties>,
    #[serde(rename = "Trendmicro")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trendmicro: Option<TrendmicroSourceProperties>,
    #[serde(rename = "Veeva")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub veeva: Option<VeevaSourceProperties>,
    #[serde(rename = "Zendesk")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zendesk: Option<ZendeskSourceProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AmplitudeSourceProperties {
    #[serde(default)]
    pub object: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomConnectorSourceProperties {
    #[serde(rename = "customProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "dataTransferApi")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_transfer_api: Option<DataTransferApi>,
    #[serde(rename = "entityName")]
    #[serde(default)]
    pub entity_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataTransferApi {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatadogSourceProperties {
    #[serde(default)]
    pub object: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DynatraceSourceProperties {
    #[serde(default)]
    pub object: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GoogleAnalyticsSourceProperties {
    #[serde(default)]
    pub object: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InforNexusSourceProperties {
    #[serde(default)]
    pub object: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MarketoSourceProperties {
    #[serde(default)]
    pub object: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PardotSourceProperties {
    #[serde(default)]
    pub object: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3SourceProperties {
    #[serde(rename = "bucketName")]
    #[serde(default)]
    pub bucket_name: String,
    #[serde(rename = "bucketPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_prefix: Option<String>,
    #[serde(rename = "s3InputFormatConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_input_format_config: Option<S3InputFormatConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3InputFormatConfig {
    #[serde(rename = "s3InputFileType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_input_file_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SAPODataSourceProperties {
    #[serde(rename = "objectPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_path: Option<String>,
    #[serde(rename = "paginationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_config: Option<SAPODataPaginationConfig>,
    #[serde(rename = "parallelismConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelism_config: Option<SAPODataParallelismConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SAPODataPaginationConfig {
    #[serde(rename = "maxPageSize")]
    #[serde(default)]
    pub max_page_size: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SAPODataParallelismConfig {
    #[serde(rename = "maxParallelism")]
    #[serde(default)]
    pub max_parallelism: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SalesforceSourceProperties {
    #[serde(rename = "dataTransferApi")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_transfer_api: Option<String>,
    #[serde(rename = "enableDynamicFieldUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_dynamic_field_update: Option<bool>,
    #[serde(rename = "includeDeletedRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_deleted_records: Option<bool>,
    #[serde(default)]
    pub object: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceNowSourceProperties {
    #[serde(default)]
    pub object: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SingularSourceProperties {
    #[serde(default)]
    pub object: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SlackSourceProperties {
    #[serde(default)]
    pub object: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrendmicroSourceProperties {
    #[serde(default)]
    pub object: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VeevaSourceProperties {
    #[serde(rename = "documentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_type: Option<String>,
    #[serde(rename = "includeAllVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_all_versions: Option<bool>,
    #[serde(rename = "includeRenditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_renditions: Option<bool>,
    #[serde(rename = "includeSourceFiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_source_files: Option<bool>,
    #[serde(default)]
    pub object: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ZendeskSourceProperties {
    #[serde(default)]
    pub object: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Task {
    #[serde(rename = "connectorOperator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_operator: Option<ConnectorOperator>,
    #[serde(rename = "destinationField")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_field: Option<String>,
    #[serde(rename = "sourceFields")]
    #[serde(default)]
    pub source_fields: Vec<String>,
    #[serde(rename = "taskProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_properties: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "taskType")]
    #[serde(default)]
    pub task_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectorOperator {
    #[serde(rename = "Amplitude")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amplitude: Option<String>,
    #[serde(rename = "CustomConnector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_connector: Option<String>,
    #[serde(rename = "Datadog")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datadog: Option<String>,
    #[serde(rename = "Dynatrace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynatrace: Option<String>,
    #[serde(rename = "GoogleAnalytics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_analytics: Option<String>,
    #[serde(rename = "InforNexus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infor_nexus: Option<String>,
    #[serde(rename = "Marketo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketo: Option<String>,
    #[serde(rename = "Pardot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pardot: Option<String>,
    #[serde(rename = "S3")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3: Option<String>,
    #[serde(rename = "SAPOData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_a_p_o_data: Option<String>,
    #[serde(rename = "Salesforce")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salesforce: Option<String>,
    #[serde(rename = "ServiceNow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_now: Option<String>,
    #[serde(rename = "Singular")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub singular: Option<String>,
    #[serde(rename = "Slack")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slack: Option<String>,
    #[serde(rename = "Trendmicro")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trendmicro: Option<String>,
    #[serde(rename = "Veeva")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub veeva: Option<String>,
    #[serde(rename = "Zendesk")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zendesk: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TriggerConfig {
    #[serde(rename = "triggerProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_properties: Option<TriggerProperties>,
    #[serde(rename = "triggerType")]
    #[serde(default)]
    pub trigger_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TriggerProperties {
    #[serde(rename = "Scheduled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled: Option<ScheduledTriggerProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduledTriggerProperties {
    #[serde(rename = "dataPullMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_pull_mode: Option<String>,
    #[serde(rename = "firstExecutionFrom")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_execution_from: Option<f64>,
    #[serde(rename = "flowErrorDeactivationThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_error_deactivation_threshold: Option<i32>,
    #[serde(rename = "scheduleEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_end_time: Option<f64>,
    #[serde(rename = "scheduleExpression")]
    #[serde(default)]
    pub schedule_expression: String,
    #[serde(rename = "scheduleOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_offset: Option<i64>,
    #[serde(rename = "scheduleStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_start_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFlowResponse {
    #[serde(rename = "flowArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    #[serde(rename = "flowStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConnectorProfileRequest {
    #[serde(rename = "connectorProfileName")]
    #[serde(default)]
    pub connector_profile_name: String,
    #[serde(rename = "forceDelete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_delete: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConnectorProfileResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFlowRequest {
    #[serde(rename = "flowName")]
    #[serde(default)]
    pub flow_name: String,
    #[serde(rename = "forceDelete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_delete: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFlowResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConnectorEntityRequest {
    #[serde(rename = "apiVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    #[serde(rename = "connectorEntityName")]
    #[serde(default)]
    pub connector_entity_name: String,
    #[serde(rename = "connectorProfileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_profile_name: Option<String>,
    #[serde(rename = "connectorType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConnectorEntityResponse {
    #[serde(rename = "connectorEntityFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_entity_fields: Option<Vec<ConnectorEntityField>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectorEntityField {
    #[serde(rename = "customProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_properties: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "defaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "destinationProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_properties: Option<DestinationFieldProperties>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(rename = "isDeprecated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_deprecated: Option<bool>,
    #[serde(rename = "isPrimaryKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_primary_key: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "parentIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_identifier: Option<String>,
    #[serde(rename = "sourceProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_properties: Option<SourceFieldProperties>,
    #[serde(rename = "supportedFieldTypeDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_field_type_details: Option<SupportedFieldTypeDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DestinationFieldProperties {
    #[serde(rename = "isCreatable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_creatable: Option<bool>,
    #[serde(rename = "isDefaultedOnCreate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_defaulted_on_create: Option<bool>,
    #[serde(rename = "isNullable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_nullable: Option<bool>,
    #[serde(rename = "isUpdatable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_updatable: Option<bool>,
    #[serde(rename = "isUpsertable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_upsertable: Option<bool>,
    #[serde(rename = "supportedWriteOperations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_write_operations: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceFieldProperties {
    #[serde(rename = "isQueryable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_queryable: Option<bool>,
    #[serde(rename = "isRetrievable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_retrievable: Option<bool>,
    #[serde(rename = "isTimestampFieldForIncrementalQueries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_timestamp_field_for_incremental_queries: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SupportedFieldTypeDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v1: Option<FieldTypeDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FieldTypeDetails {
    #[serde(rename = "fieldLengthRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_length_range: Option<Range>,
    #[serde(rename = "fieldType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_type: Option<String>,
    #[serde(rename = "fieldValueRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_value_range: Option<Range>,
    #[serde(rename = "filterOperators")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_operators: Option<Vec<String>>,
    #[serde(rename = "supportedDateFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_date_format: Option<String>,
    #[serde(rename = "supportedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_values: Option<Vec<String>>,
    #[serde(rename = "valueRegexPattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_regex_pattern: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Range {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConnectorProfilesRequest {
    #[serde(rename = "connectorLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_label: Option<String>,
    #[serde(rename = "connectorProfileNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_profile_names: Option<Vec<String>>,
    #[serde(rename = "connectorType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_type: Option<String>,
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
pub struct DescribeConnectorProfilesResponse {
    #[serde(rename = "connectorProfileDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_profile_details: Option<Vec<ConnectorProfile>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectorProfile {
    #[serde(rename = "connectionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_mode: Option<String>,
    #[serde(rename = "connectorLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_label: Option<String>,
    #[serde(rename = "connectorProfileArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_profile_arn: Option<String>,
    #[serde(rename = "connectorProfileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_profile_name: Option<String>,
    #[serde(rename = "connectorProfileProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_profile_properties: Option<ConnectorProfileProperties>,
    #[serde(rename = "connectorType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_type: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "credentialsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials_arn: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(rename = "privateConnectionProvisioningState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_connection_provisioning_state: Option<PrivateConnectionProvisioningState>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PrivateConnectionProvisioningState {
    #[serde(rename = "failureCause")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_cause: Option<String>,
    #[serde(rename = "failureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConnectorRequest {
    #[serde(rename = "connectorLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_label: Option<String>,
    #[serde(rename = "connectorType")]
    #[serde(default)]
    pub connector_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConnectorResponse {
    #[serde(rename = "connectorConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_configuration: Option<ConnectorConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectorConfiguration {
    #[serde(rename = "authenticationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_config: Option<AuthenticationConfig>,
    #[serde(rename = "canUseAsDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_use_as_destination: Option<bool>,
    #[serde(rename = "canUseAsSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_use_as_source: Option<bool>,
    #[serde(rename = "connectorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_arn: Option<String>,
    #[serde(rename = "connectorDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_description: Option<String>,
    #[serde(rename = "connectorLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_label: Option<String>,
    #[serde(rename = "connectorMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_metadata: Option<ConnectorMetadata>,
    #[serde(rename = "connectorModes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_modes: Option<Vec<String>>,
    #[serde(rename = "connectorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_name: Option<String>,
    #[serde(rename = "connectorOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_owner: Option<String>,
    #[serde(rename = "connectorProvisioningConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_provisioning_config: Option<ConnectorProvisioningConfig>,
    #[serde(rename = "connectorProvisioningType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_provisioning_type: Option<String>,
    #[serde(rename = "connectorRuntimeSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_runtime_settings: Option<Vec<ConnectorRuntimeSetting>>,
    #[serde(rename = "connectorType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_type: Option<String>,
    #[serde(rename = "connectorVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_version: Option<String>,
    #[serde(rename = "isPrivateLinkEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_private_link_enabled: Option<bool>,
    #[serde(rename = "isPrivateLinkEndpointUrlRequired")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_private_link_endpoint_url_required: Option<bool>,
    #[serde(rename = "logoURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_u_r_l: Option<String>,
    #[serde(rename = "registeredAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_at: Option<f64>,
    #[serde(rename = "registeredBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_by: Option<String>,
    #[serde(rename = "supportedApiVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_api_versions: Option<Vec<String>>,
    #[serde(rename = "supportedDataTransferApis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_data_transfer_apis: Option<Vec<DataTransferApi>>,
    #[serde(rename = "supportedDataTransferTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_data_transfer_types: Option<Vec<String>>,
    #[serde(rename = "supportedDestinationConnectors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_destination_connectors: Option<Vec<String>>,
    #[serde(rename = "supportedOperators")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_operators: Option<Vec<String>>,
    #[serde(rename = "supportedSchedulingFrequencies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_scheduling_frequencies: Option<Vec<String>>,
    #[serde(rename = "supportedTriggerTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_trigger_types: Option<Vec<String>>,
    #[serde(rename = "supportedWriteOperations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_write_operations: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthenticationConfig {
    #[serde(rename = "customAuthConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_auth_configs: Option<Vec<CustomAuthConfig>>,
    #[serde(rename = "isApiKeyAuthSupported")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_api_key_auth_supported: Option<bool>,
    #[serde(rename = "isBasicAuthSupported")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_basic_auth_supported: Option<bool>,
    #[serde(rename = "isCustomAuthSupported")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_custom_auth_supported: Option<bool>,
    #[serde(rename = "isOAuth2Supported")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_o_auth2_supported: Option<bool>,
    #[serde(rename = "oAuth2Defaults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth2_defaults: Option<OAuth2Defaults>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomAuthConfig {
    #[serde(rename = "authParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_parameters: Option<Vec<AuthParameter>>,
    #[serde(rename = "customAuthenticationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_authentication_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthParameter {
    #[serde(rename = "connectorSuppliedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_supplied_values: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "isRequired")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    #[serde(rename = "isSensitiveField")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_sensitive_field: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OAuth2Defaults {
    #[serde(rename = "authCodeUrls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_code_urls: Option<Vec<String>>,
    #[serde(rename = "oauth2CustomProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth2_custom_properties: Option<Vec<OAuth2CustomParameter>>,
    #[serde(rename = "oauth2GrantTypesSupported")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth2_grant_types_supported: Option<Vec<String>>,
    #[serde(rename = "oauthScopes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_scopes: Option<Vec<String>>,
    #[serde(rename = "tokenUrls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_urls: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OAuth2CustomParameter {
    #[serde(rename = "connectorSuppliedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_supplied_values: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "isRequired")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    #[serde(rename = "isSensitiveField")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_sensitive_field: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectorMetadata {
    #[serde(rename = "Amplitude")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amplitude: Option<AmplitudeMetadata>,
    #[serde(rename = "CustomerProfiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_profiles: Option<CustomerProfilesMetadata>,
    #[serde(rename = "Datadog")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datadog: Option<DatadogMetadata>,
    #[serde(rename = "Dynatrace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynatrace: Option<DynatraceMetadata>,
    #[serde(rename = "EventBridge")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bridge: Option<EventBridgeMetadata>,
    #[serde(rename = "GoogleAnalytics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_analytics: Option<GoogleAnalyticsMetadata>,
    #[serde(rename = "Honeycode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub honeycode: Option<HoneycodeMetadata>,
    #[serde(rename = "InforNexus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub infor_nexus: Option<InforNexusMetadata>,
    #[serde(rename = "Marketo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketo: Option<MarketoMetadata>,
    #[serde(rename = "Pardot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pardot: Option<PardotMetadata>,
    #[serde(rename = "Redshift")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift: Option<RedshiftMetadata>,
    #[serde(rename = "S3")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3: Option<S3Metadata>,
    #[serde(rename = "SAPOData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_a_p_o_data: Option<SAPODataMetadata>,
    #[serde(rename = "Salesforce")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salesforce: Option<SalesforceMetadata>,
    #[serde(rename = "ServiceNow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_now: Option<ServiceNowMetadata>,
    #[serde(rename = "Singular")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub singular: Option<SingularMetadata>,
    #[serde(rename = "Slack")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slack: Option<SlackMetadata>,
    #[serde(rename = "Snowflake")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowflake: Option<SnowflakeMetadata>,
    #[serde(rename = "Trendmicro")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trendmicro: Option<TrendmicroMetadata>,
    #[serde(rename = "Upsolver")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upsolver: Option<UpsolverMetadata>,
    #[serde(rename = "Veeva")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub veeva: Option<VeevaMetadata>,
    #[serde(rename = "Zendesk")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zendesk: Option<ZendeskMetadata>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AmplitudeMetadata {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomerProfilesMetadata {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatadogMetadata {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DynatraceMetadata {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventBridgeMetadata {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GoogleAnalyticsMetadata {
    #[serde(rename = "oAuthScopes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth_scopes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HoneycodeMetadata {
    #[serde(rename = "oAuthScopes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth_scopes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InforNexusMetadata {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MarketoMetadata {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PardotMetadata {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RedshiftMetadata {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Metadata {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SAPODataMetadata {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SalesforceMetadata {
    #[serde(rename = "dataTransferApis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_transfer_apis: Option<Vec<String>>,
    #[serde(rename = "oAuthScopes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth_scopes: Option<Vec<String>>,
    #[serde(rename = "oauth2GrantTypesSupported")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth2_grant_types_supported: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceNowMetadata {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SingularMetadata {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SlackMetadata {
    #[serde(rename = "oAuthScopes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth_scopes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnowflakeMetadata {
    #[serde(rename = "supportedRegions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_regions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrendmicroMetadata {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpsolverMetadata {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VeevaMetadata {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ZendeskMetadata {
    #[serde(rename = "oAuthScopes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth_scopes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectorProvisioningConfig {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda: Option<LambdaConnectorProvisioningConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LambdaConnectorProvisioningConfig {
    #[serde(rename = "lambdaArn")]
    #[serde(default)]
    pub lambda_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectorRuntimeSetting {
    #[serde(rename = "connectorSuppliedValueOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_supplied_value_options: Option<Vec<String>>,
    #[serde(rename = "dataType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "isRequired")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConnectorsRequest {
    #[serde(rename = "connectorTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_types: Option<Vec<String>>,
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
pub struct DescribeConnectorsResponse {
    #[serde(rename = "connectorConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_configurations: Option<std::collections::HashMap<String, ConnectorConfiguration>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectors: Option<Vec<ConnectorDetail>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectorDetail {
    #[serde(rename = "applicationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_type: Option<String>,
    #[serde(rename = "connectorDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_description: Option<String>,
    #[serde(rename = "connectorLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_label: Option<String>,
    #[serde(rename = "connectorModes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_modes: Option<Vec<String>>,
    #[serde(rename = "connectorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_name: Option<String>,
    #[serde(rename = "connectorOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_owner: Option<String>,
    #[serde(rename = "connectorProvisioningType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_provisioning_type: Option<String>,
    #[serde(rename = "connectorType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_type: Option<String>,
    #[serde(rename = "connectorVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_version: Option<String>,
    #[serde(rename = "registeredAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_at: Option<f64>,
    #[serde(rename = "registeredBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_by: Option<String>,
    #[serde(rename = "supportedDataTransferTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_data_transfer_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFlowExecutionRecordsRequest {
    #[serde(rename = "flowName")]
    #[serde(default)]
    pub flow_name: String,
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
pub struct DescribeFlowExecutionRecordsResponse {
    #[serde(rename = "flowExecutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_executions: Option<Vec<ExecutionRecord>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutionRecord {
    #[serde(rename = "dataPullEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_pull_end_time: Option<f64>,
    #[serde(rename = "dataPullStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_pull_start_time: Option<f64>,
    #[serde(rename = "executionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    #[serde(rename = "executionResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_result: Option<ExecutionResult>,
    #[serde(rename = "executionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_status: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(rename = "metadataCatalogDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_catalog_details: Option<Vec<MetadataCatalogDetail>>,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutionResult {
    #[serde(rename = "bytesProcessed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_processed: Option<i64>,
    #[serde(rename = "bytesWritten")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_written: Option<i64>,
    #[serde(rename = "errorInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_info: Option<ErrorInfo>,
    #[serde(rename = "maxPageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_page_size: Option<i64>,
    #[serde(rename = "numParallelProcesses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_parallel_processes: Option<i64>,
    #[serde(rename = "recordsProcessed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records_processed: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ErrorInfo {
    #[serde(rename = "executionMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_message: Option<String>,
    #[serde(rename = "putFailuresCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put_failures_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetadataCatalogDetail {
    #[serde(rename = "catalogType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_type: Option<String>,
    #[serde(rename = "partitionRegistrationOutput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_registration_output: Option<RegistrationOutput>,
    #[serde(rename = "tableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    #[serde(rename = "tableRegistrationOutput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_registration_output: Option<RegistrationOutput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegistrationOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFlowRequest {
    #[serde(rename = "flowName")]
    #[serde(default)]
    pub flow_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFlowResponse {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "createdBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "destinationFlowConfigList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_flow_config_list: Option<Vec<DestinationFlowConfig>>,
    #[serde(rename = "flowArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    #[serde(rename = "flowName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_name: Option<String>,
    #[serde(rename = "flowStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_status: Option<String>,
    #[serde(rename = "flowStatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_status_message: Option<String>,
    #[serde(rename = "kmsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_arn: Option<String>,
    #[serde(rename = "lastRunExecutionDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_execution_details: Option<ExecutionDetails>,
    #[serde(rename = "lastRunMetadataCatalogDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_metadata_catalog_details: Option<Vec<MetadataCatalogDetail>>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(rename = "lastUpdatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by: Option<String>,
    #[serde(rename = "metadataCatalogConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_catalog_config: Option<MetadataCatalogConfig>,
    #[serde(rename = "schemaVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<i64>,
    #[serde(rename = "sourceFlowConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_flow_config: Option<SourceFlowConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<Task>>,
    #[serde(rename = "triggerConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_config: Option<TriggerConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutionDetails {
    #[serde(rename = "mostRecentExecutionMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub most_recent_execution_message: Option<String>,
    #[serde(rename = "mostRecentExecutionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub most_recent_execution_status: Option<String>,
    #[serde(rename = "mostRecentExecutionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub most_recent_execution_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConnectorEntitiesRequest {
    #[serde(rename = "apiVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    #[serde(rename = "connectorProfileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_profile_name: Option<String>,
    #[serde(rename = "connectorType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_type: Option<String>,
    #[serde(rename = "entitiesPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities_path: Option<String>,
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
pub struct ListConnectorEntitiesResponse {
    #[serde(rename = "connectorEntityMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_entity_map: Option<std::collections::HashMap<String, Vec<ConnectorEntity>>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectorEntity {
    #[serde(rename = "hasNestedEntities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_nested_entities: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConnectorsRequest {
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
pub struct ListConnectorsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectors: Option<Vec<ConnectorDetail>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFlowsRequest {
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
pub struct ListFlowsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flows: Option<Vec<FlowDefinition>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlowDefinition {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "createdBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "destinationConnectorLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_connector_label: Option<String>,
    #[serde(rename = "destinationConnectorType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_connector_type: Option<String>,
    #[serde(rename = "flowArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    #[serde(rename = "flowName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_name: Option<String>,
    #[serde(rename = "flowStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_status: Option<String>,
    #[serde(rename = "lastRunExecutionDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_execution_details: Option<ExecutionDetails>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(rename = "lastUpdatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by: Option<String>,
    #[serde(rename = "sourceConnectorLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_connector_label: Option<String>,
    #[serde(rename = "sourceConnectorType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_connector_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "triggerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_type: Option<String>,
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
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterConnectorRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "connectorLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_label: Option<String>,
    #[serde(rename = "connectorProvisioningConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_provisioning_config: Option<ConnectorProvisioningConfig>,
    #[serde(rename = "connectorProvisioningType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_provisioning_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterConnectorResponse {
    #[serde(rename = "connectorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResetConnectorMetadataCacheRequest {
    #[serde(rename = "apiVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    #[serde(rename = "connectorEntityName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_entity_name: Option<String>,
    #[serde(rename = "connectorProfileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_profile_name: Option<String>,
    #[serde(rename = "connectorType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_type: Option<String>,
    #[serde(rename = "entitiesPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities_path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResetConnectorMetadataCacheResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartFlowRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "flowName")]
    #[serde(default)]
    pub flow_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartFlowResponse {
    #[serde(rename = "executionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    #[serde(rename = "flowArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    #[serde(rename = "flowStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopFlowRequest {
    #[serde(rename = "flowName")]
    #[serde(default)]
    pub flow_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopFlowResponse {
    #[serde(rename = "flowArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
    #[serde(rename = "flowStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnregisterConnectorRequest {
    #[serde(rename = "connectorLabel")]
    #[serde(default)]
    pub connector_label: String,
    #[serde(rename = "forceDelete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_delete: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnregisterConnectorResponse {}

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
pub struct UpdateConnectorProfileRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "connectionMode")]
    #[serde(default)]
    pub connection_mode: String,
    #[serde(rename = "connectorProfileConfig")]
    #[serde(default)]
    pub connector_profile_config: ConnectorProfileConfig,
    #[serde(rename = "connectorProfileName")]
    #[serde(default)]
    pub connector_profile_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConnectorProfileResponse {
    #[serde(rename = "connectorProfileArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_profile_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConnectorRegistrationRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "connectorLabel")]
    #[serde(default)]
    pub connector_label: String,
    #[serde(rename = "connectorProvisioningConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_provisioning_config: Option<ConnectorProvisioningConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConnectorRegistrationResponse {
    #[serde(rename = "connectorArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFlowRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "destinationFlowConfigList")]
    #[serde(default)]
    pub destination_flow_config_list: Vec<DestinationFlowConfig>,
    #[serde(rename = "flowName")]
    #[serde(default)]
    pub flow_name: String,
    #[serde(rename = "metadataCatalogConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_catalog_config: Option<MetadataCatalogConfig>,
    #[serde(rename = "sourceFlowConfig")]
    #[serde(default)]
    pub source_flow_config: SourceFlowConfig,
    #[serde(default)]
    pub tasks: Vec<Task>,
    #[serde(rename = "triggerConfig")]
    #[serde(default)]
    pub trigger_config: TriggerConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFlowResponse {
    #[serde(rename = "flowStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_status: Option<String>,
}
