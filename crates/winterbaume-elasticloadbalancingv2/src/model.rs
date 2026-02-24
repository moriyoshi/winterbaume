//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-elbv2

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateTargetGroupResult")]
pub struct CreateTargetGroupOutput {
    #[serde(rename = "TargetGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_groups: Option<TargetGroups>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetGroups {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<TargetGroup>,
}
impl From<Vec<TargetGroup>> for TargetGroups {
    fn from(v: Vec<TargetGroup>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<TargetGroup> for TargetGroups {
    fn from_iter<I: IntoIterator<Item = TargetGroup>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<TargetGroup>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTargetGroupList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<TargetGroup>,
}

impl From<Vec<TargetGroup>> for XmlTargetGroupList {
    fn from(v: Vec<TargetGroup>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<TargetGroup> for XmlTargetGroupList {
    fn from_iter<I: IntoIterator<Item = TargetGroup>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TargetGroup")]
pub struct TargetGroup {
    #[serde(rename = "HealthCheckEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_enabled: Option<bool>,
    #[serde(rename = "HealthCheckIntervalSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_interval_seconds: Option<i32>,
    #[serde(rename = "HealthCheckPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_path: Option<String>,
    #[serde(rename = "HealthCheckPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_port: Option<String>,
    #[serde(rename = "HealthCheckProtocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_protocol: Option<String>,
    #[serde(rename = "HealthCheckTimeoutSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_timeout_seconds: Option<i32>,
    #[serde(rename = "HealthyThresholdCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthy_threshold_count: Option<i32>,
    #[serde(rename = "IpAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(rename = "LoadBalancerArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_arns: Option<LoadBalancerArns>,
    #[serde(rename = "Matcher")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matcher: Option<Matcher>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "Protocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "ProtocolVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_version: Option<String>,
    #[serde(rename = "TargetControlPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_control_port: Option<i32>,
    #[serde(rename = "TargetGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_group_arn: Option<String>,
    #[serde(rename = "TargetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_group_name: Option<String>,
    #[serde(rename = "TargetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    #[serde(rename = "UnhealthyThresholdCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhealthy_threshold_count: Option<i32>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoadBalancerArns {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for LoadBalancerArns {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for LoadBalancerArns {
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
#[serde(rename = "Matcher")]
pub struct Matcher {
    #[serde(rename = "GrpcCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grpc_code: Option<String>,
    #[serde(rename = "HttpCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeTargetHealthInput")]
pub struct DescribeTargetHealthInput {
    #[serde(rename = "Include")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<ListOfDescribeTargetHealthIncludeOptions>,
    #[serde(rename = "TargetGroupArn")]
    #[serde(default)]
    pub target_group_arn: String,
    #[serde(rename = "Targets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<TargetDescriptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOfDescribeTargetHealthIncludeOptions {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ListOfDescribeTargetHealthIncludeOptions {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ListOfDescribeTargetHealthIncludeOptions {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetDescriptions {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<TargetDescription>,
}
impl From<Vec<TargetDescription>> for TargetDescriptions {
    fn from(v: Vec<TargetDescription>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<TargetDescription> for TargetDescriptions {
    fn from_iter<I: IntoIterator<Item = TargetDescription>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<TargetDescription>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTargetDescriptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<TargetDescription>,
}

impl From<Vec<TargetDescription>> for XmlTargetDescriptionList {
    fn from(v: Vec<TargetDescription>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<TargetDescription> for XmlTargetDescriptionList {
    fn from_iter<I: IntoIterator<Item = TargetDescription>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TargetDescription")]
pub struct TargetDescription {
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "QuicServerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quic_server_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyListenerResult")]
pub struct ModifyListenerOutput {
    #[serde(rename = "Listeners")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listeners: Option<Listeners>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Listeners {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Listener>,
}
impl From<Vec<Listener>> for Listeners {
    fn from(v: Vec<Listener>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Listener> for Listeners {
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
    #[serde(rename = "AlpnPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpn_policy: Option<AlpnPolicyName>,
    #[serde(rename = "Certificates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates: Option<CertificateList>,
    #[serde(rename = "DefaultActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_actions: Option<Actions>,
    #[serde(rename = "ListenerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_arn: Option<String>,
    #[serde(rename = "LoadBalancerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_arn: Option<String>,
    #[serde(rename = "MutualAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutual_authentication: Option<MutualAuthenticationAttributes>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "Protocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "SslPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AlpnPolicyName {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for AlpnPolicyName {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for AlpnPolicyName {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CertificateList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Certificate>,
}
impl From<Vec<Certificate>> for CertificateList {
    fn from(v: Vec<Certificate>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Certificate> for CertificateList {
    fn from_iter<I: IntoIterator<Item = Certificate>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Certificate>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlCertificateList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Certificate>,
}

impl From<Vec<Certificate>> for XmlCertificateList {
    fn from(v: Vec<Certificate>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Certificate> for XmlCertificateList {
    fn from_iter<I: IntoIterator<Item = Certificate>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Certificate")]
pub struct Certificate {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "IsDefault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Actions {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Action>,
}
impl From<Vec<Action>> for Actions {
    fn from(v: Vec<Action>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Action> for Actions {
    fn from_iter<I: IntoIterator<Item = Action>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Action>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlActionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Action>,
}

impl From<Vec<Action>> for XmlActionList {
    fn from(v: Vec<Action>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Action> for XmlActionList {
    fn from_iter<I: IntoIterator<Item = Action>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Action")]
pub struct Action {
    #[serde(rename = "AuthenticateCognitoConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticate_cognito_config: Option<AuthenticateCognitoActionConfig>,
    #[serde(rename = "AuthenticateOidcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticate_oidc_config: Option<AuthenticateOidcActionConfig>,
    #[serde(rename = "FixedResponseConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_response_config: Option<FixedResponseActionConfig>,
    #[serde(rename = "ForwardConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_config: Option<ForwardActionConfig>,
    #[serde(rename = "JwtValidationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt_validation_config: Option<JwtValidationActionConfig>,
    #[serde(rename = "Order")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    #[serde(rename = "RedirectConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_config: Option<RedirectActionConfig>,
    #[serde(rename = "TargetGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_group_arn: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AuthenticateCognitoActionConfig")]
pub struct AuthenticateCognitoActionConfig {
    #[serde(rename = "AuthenticationRequestExtraParams")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_request_extra_params: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "OnUnauthenticatedRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_unauthenticated_request: Option<String>,
    #[serde(rename = "Scope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "SessionCookieName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_cookie_name: Option<String>,
    #[serde(rename = "SessionTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_timeout: Option<i64>,
    #[serde(rename = "UserPoolArn")]
    #[serde(default)]
    pub user_pool_arn: String,
    #[serde(rename = "UserPoolClientId")]
    #[serde(default)]
    pub user_pool_client_id: String,
    #[serde(rename = "UserPoolDomain")]
    #[serde(default)]
    pub user_pool_domain: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AuthenticateOidcActionConfig")]
pub struct AuthenticateOidcActionConfig {
    #[serde(rename = "AuthenticationRequestExtraParams")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_request_extra_params: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "AuthorizationEndpoint")]
    #[serde(default)]
    pub authorization_endpoint: String,
    #[serde(rename = "ClientId")]
    #[serde(default)]
    pub client_id: String,
    #[serde(rename = "ClientSecret")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(rename = "Issuer")]
    #[serde(default)]
    pub issuer: String,
    #[serde(rename = "OnUnauthenticatedRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_unauthenticated_request: Option<String>,
    #[serde(rename = "Scope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "SessionCookieName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_cookie_name: Option<String>,
    #[serde(rename = "SessionTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_timeout: Option<i64>,
    #[serde(rename = "TokenEndpoint")]
    #[serde(default)]
    pub token_endpoint: String,
    #[serde(rename = "UseExistingClientSecret")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_existing_client_secret: Option<bool>,
    #[serde(rename = "UserInfoEndpoint")]
    #[serde(default)]
    pub user_info_endpoint: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "FixedResponseActionConfig")]
pub struct FixedResponseActionConfig {
    #[serde(rename = "ContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "MessageBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_body: Option<String>,
    #[serde(rename = "StatusCode")]
    #[serde(default)]
    pub status_code: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ForwardActionConfig")]
pub struct ForwardActionConfig {
    #[serde(rename = "TargetGroupStickinessConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_group_stickiness_config: Option<TargetGroupStickinessConfig>,
    #[serde(rename = "TargetGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_groups: Option<TargetGroupList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TargetGroupStickinessConfig")]
pub struct TargetGroupStickinessConfig {
    #[serde(rename = "DurationSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i32>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetGroupList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<TargetGroupTuple>,
}
impl From<Vec<TargetGroupTuple>> for TargetGroupList {
    fn from(v: Vec<TargetGroupTuple>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<TargetGroupTuple> for TargetGroupList {
    fn from_iter<I: IntoIterator<Item = TargetGroupTuple>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<TargetGroupTuple>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTargetGroupTupleList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<TargetGroupTuple>,
}

impl From<Vec<TargetGroupTuple>> for XmlTargetGroupTupleList {
    fn from(v: Vec<TargetGroupTuple>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<TargetGroupTuple> for XmlTargetGroupTupleList {
    fn from_iter<I: IntoIterator<Item = TargetGroupTuple>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TargetGroupTuple")]
pub struct TargetGroupTuple {
    #[serde(rename = "TargetGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_group_arn: Option<String>,
    #[serde(rename = "Weight")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "JwtValidationActionConfig")]
pub struct JwtValidationActionConfig {
    #[serde(rename = "AdditionalClaims")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_claims: Option<JwtValidationActionAdditionalClaims>,
    #[serde(rename = "Issuer")]
    #[serde(default)]
    pub issuer: String,
    #[serde(rename = "JwksEndpoint")]
    #[serde(default)]
    pub jwks_endpoint: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JwtValidationActionAdditionalClaims {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<JwtValidationActionAdditionalClaim>,
}
impl From<Vec<JwtValidationActionAdditionalClaim>> for JwtValidationActionAdditionalClaims {
    fn from(v: Vec<JwtValidationActionAdditionalClaim>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<JwtValidationActionAdditionalClaim> for JwtValidationActionAdditionalClaims {
    fn from_iter<I: IntoIterator<Item = JwtValidationActionAdditionalClaim>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<JwtValidationActionAdditionalClaim>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlJwtValidationActionAdditionalClaimList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<JwtValidationActionAdditionalClaim>,
}

impl From<Vec<JwtValidationActionAdditionalClaim>> for XmlJwtValidationActionAdditionalClaimList {
    fn from(v: Vec<JwtValidationActionAdditionalClaim>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<JwtValidationActionAdditionalClaim>
    for XmlJwtValidationActionAdditionalClaimList
{
    fn from_iter<I: IntoIterator<Item = JwtValidationActionAdditionalClaim>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "JwtValidationActionAdditionalClaim")]
pub struct JwtValidationActionAdditionalClaim {
    #[serde(rename = "Format")]
    #[serde(default)]
    pub format: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: JwtValidationActionAdditionalClaimValues,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JwtValidationActionAdditionalClaimValues {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for JwtValidationActionAdditionalClaimValues {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for JwtValidationActionAdditionalClaimValues {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RedirectActionConfig")]
pub struct RedirectActionConfig {
    #[serde(rename = "Host")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    #[serde(rename = "Protocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "Query")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(rename = "StatusCode")]
    #[serde(default)]
    pub status_code: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MutualAuthenticationAttributes")]
pub struct MutualAuthenticationAttributes {
    #[serde(rename = "AdvertiseTrustStoreCaNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advertise_trust_store_ca_names: Option<String>,
    #[serde(rename = "IgnoreClientCertificateExpiry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_client_certificate_expiry: Option<bool>,
    #[serde(rename = "Mode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "TrustStoreArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_store_arn: Option<String>,
    #[serde(rename = "TrustStoreAssociationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_store_association_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyTargetGroupInput")]
pub struct ModifyTargetGroupInput {
    #[serde(rename = "HealthCheckEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_enabled: Option<bool>,
    #[serde(rename = "HealthCheckIntervalSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_interval_seconds: Option<i32>,
    #[serde(rename = "HealthCheckPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_path: Option<String>,
    #[serde(rename = "HealthCheckPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_port: Option<String>,
    #[serde(rename = "HealthCheckProtocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_protocol: Option<String>,
    #[serde(rename = "HealthCheckTimeoutSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_timeout_seconds: Option<i32>,
    #[serde(rename = "HealthyThresholdCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthy_threshold_count: Option<i32>,
    #[serde(rename = "Matcher")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matcher: Option<Matcher>,
    #[serde(rename = "TargetGroupArn")]
    #[serde(default)]
    pub target_group_arn: String,
    #[serde(rename = "UnhealthyThresholdCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhealthy_threshold_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeTagsResult")]
pub struct DescribeTagsOutput {
    #[serde(rename = "TagDescriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_descriptions: Option<TagDescriptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagDescriptions {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<TagDescription>,
}
impl From<Vec<TagDescription>> for TagDescriptions {
    fn from(v: Vec<TagDescription>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<TagDescription> for TagDescriptions {
    fn from_iter<I: IntoIterator<Item = TagDescription>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<TagDescription>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTagDescriptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<TagDescription>,
}

impl From<Vec<TagDescription>> for XmlTagDescriptionList {
    fn from(v: Vec<TagDescription>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<TagDescription> for XmlTagDescriptionList {
    fn from_iter<I: IntoIterator<Item = TagDescription>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TagDescription")]
pub struct TagDescription {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
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
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSharedTrustStoreAssociationOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeAccountLimitsResult")]
pub struct DescribeAccountLimitsOutput {
    #[serde(rename = "Limits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<Limits>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Limits {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Limit>,
}
impl From<Vec<Limit>> for Limits {
    fn from(v: Vec<Limit>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Limit> for Limits {
    fn from_iter<I: IntoIterator<Item = Limit>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Limit>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlLimitList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Limit>,
}

impl From<Vec<Limit>> for XmlLimitList {
    fn from(v: Vec<Limit>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Limit> for XmlLimitList {
    fn from_iter<I: IntoIterator<Item = Limit>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Limit")]
pub struct Limit {
    #[serde(rename = "Max")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeTargetGroupAttributesResult")]
pub struct DescribeTargetGroupAttributesOutput {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<TargetGroupAttributes>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetGroupAttributes {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<TargetGroupAttribute>,
}
impl From<Vec<TargetGroupAttribute>> for TargetGroupAttributes {
    fn from(v: Vec<TargetGroupAttribute>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<TargetGroupAttribute> for TargetGroupAttributes {
    fn from_iter<I: IntoIterator<Item = TargetGroupAttribute>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<TargetGroupAttribute>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTargetGroupAttributeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<TargetGroupAttribute>,
}

impl From<Vec<TargetGroupAttribute>> for XmlTargetGroupAttributeList {
    fn from(v: Vec<TargetGroupAttribute>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<TargetGroupAttribute> for XmlTargetGroupAttributeList {
    fn from_iter<I: IntoIterator<Item = TargetGroupAttribute>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TargetGroupAttribute")]
pub struct TargetGroupAttribute {
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
#[serde(rename = "RemoveListenerCertificatesInput")]
pub struct RemoveListenerCertificatesInput {
    #[serde(rename = "Certificates")]
    #[serde(default)]
    pub certificates: CertificateList,
    #[serde(rename = "ListenerArn")]
    #[serde(default)]
    pub listener_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteListenerOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveListenerCertificatesOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SetSubnetsResult")]
pub struct SetSubnetsOutput {
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<AvailabilityZones>,
    #[serde(rename = "EnablePrefixForIpv6SourceNat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_prefix_for_ipv6_source_nat: Option<String>,
    #[serde(rename = "IpAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AvailabilityZones {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<AvailabilityZone>,
}
impl From<Vec<AvailabilityZone>> for AvailabilityZones {
    fn from(v: Vec<AvailabilityZone>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<AvailabilityZone> for AvailabilityZones {
    fn from_iter<I: IntoIterator<Item = AvailabilityZone>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<AvailabilityZone>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlAvailabilityZoneList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<AvailabilityZone>,
}

impl From<Vec<AvailabilityZone>> for XmlAvailabilityZoneList {
    fn from(v: Vec<AvailabilityZone>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<AvailabilityZone> for XmlAvailabilityZoneList {
    fn from_iter<I: IntoIterator<Item = AvailabilityZone>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AvailabilityZone")]
pub struct AvailabilityZone {
    #[serde(rename = "LoadBalancerAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_addresses: Option<LoadBalancerAddresses>,
    #[serde(rename = "OutpostId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_id: Option<String>,
    #[serde(rename = "SourceNatIpv6Prefixes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_nat_ipv6_prefixes: Option<SourceNatIpv6Prefixes>,
    #[serde(rename = "SubnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[serde(rename = "ZoneName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoadBalancerAddresses {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<LoadBalancerAddress>,
}
impl From<Vec<LoadBalancerAddress>> for LoadBalancerAddresses {
    fn from(v: Vec<LoadBalancerAddress>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<LoadBalancerAddress> for LoadBalancerAddresses {
    fn from_iter<I: IntoIterator<Item = LoadBalancerAddress>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<LoadBalancerAddress>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlLoadBalancerAddressList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<LoadBalancerAddress>,
}

impl From<Vec<LoadBalancerAddress>> for XmlLoadBalancerAddressList {
    fn from(v: Vec<LoadBalancerAddress>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<LoadBalancerAddress> for XmlLoadBalancerAddressList {
    fn from_iter<I: IntoIterator<Item = LoadBalancerAddress>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LoadBalancerAddress")]
pub struct LoadBalancerAddress {
    #[serde(rename = "AllocationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation_id: Option<String>,
    #[serde(rename = "IPv6Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_pv6_address: Option<String>,
    #[serde(rename = "IpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "PrivateIPv4Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_i_pv4_address: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceNatIpv6Prefixes {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for SourceNatIpv6Prefixes {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for SourceNatIpv6Prefixes {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeTrustStoresResult")]
pub struct DescribeTrustStoresOutput {
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "TrustStores")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_stores: Option<TrustStores>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrustStores {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<TrustStore>,
}
impl From<Vec<TrustStore>> for TrustStores {
    fn from(v: Vec<TrustStore>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<TrustStore> for TrustStores {
    fn from_iter<I: IntoIterator<Item = TrustStore>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<TrustStore>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTrustStoreList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<TrustStore>,
}

impl From<Vec<TrustStore>> for XmlTrustStoreList {
    fn from(v: Vec<TrustStore>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<TrustStore> for XmlTrustStoreList {
    fn from_iter<I: IntoIterator<Item = TrustStore>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TrustStore")]
pub struct TrustStore {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NumberOfCaCertificates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_ca_certificates: Option<i32>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TotalRevokedEntries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_revoked_entries: Option<i64>,
    #[serde(rename = "TrustStoreArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_store_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SetIpAddressTypeResult")]
pub struct SetIpAddressTypeOutput {
    #[serde(rename = "IpAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetTrustStoreCaCertificatesBundleInput")]
pub struct GetTrustStoreCaCertificatesBundleInput {
    #[serde(rename = "TrustStoreArn")]
    #[serde(default)]
    pub trust_store_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRuleOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyTargetGroupResult")]
pub struct ModifyTargetGroupOutput {
    #[serde(rename = "TargetGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_groups: Option<TargetGroups>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteLoadBalancerInput")]
pub struct DeleteLoadBalancerInput {
    #[serde(rename = "LoadBalancerArn")]
    #[serde(default)]
    pub load_balancer_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddTagsOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteTargetGroupInput")]
pub struct DeleteTargetGroupInput {
    #[serde(rename = "TargetGroupArn")]
    #[serde(default)]
    pub target_group_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyTrustStoreInput")]
pub struct ModifyTrustStoreInput {
    #[serde(rename = "CaCertificatesBundleS3Bucket")]
    #[serde(default)]
    pub ca_certificates_bundle_s3_bucket: String,
    #[serde(rename = "CaCertificatesBundleS3Key")]
    #[serde(default)]
    pub ca_certificates_bundle_s3_key: String,
    #[serde(rename = "CaCertificatesBundleS3ObjectVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_certificates_bundle_s3_object_version: Option<String>,
    #[serde(rename = "TrustStoreArn")]
    #[serde(default)]
    pub trust_store_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeListenersInput")]
pub struct DescribeListenersInput {
    #[serde(rename = "ListenerArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_arns: Option<ListenerArns>,
    #[serde(rename = "LoadBalancerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_arn: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListenerArns {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ListenerArns {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ListenerArns {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetResourcePolicyInput")]
pub struct GetResourcePolicyInput {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeSSLPoliciesResult")]
pub struct DescribeSSLPoliciesOutput {
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "SslPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_policies: Option<SslPolicies>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SslPolicies {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<SslPolicy>,
}
impl From<Vec<SslPolicy>> for SslPolicies {
    fn from(v: Vec<SslPolicy>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<SslPolicy> for SslPolicies {
    fn from_iter<I: IntoIterator<Item = SslPolicy>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<SslPolicy>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlSslPolicyList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<SslPolicy>,
}

impl From<Vec<SslPolicy>> for XmlSslPolicyList {
    fn from(v: Vec<SslPolicy>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<SslPolicy> for XmlSslPolicyList {
    fn from_iter<I: IntoIterator<Item = SslPolicy>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SslPolicy")]
pub struct SslPolicy {
    #[serde(rename = "Ciphers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphers: Option<Ciphers>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SslProtocols")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_protocols: Option<SslProtocols>,
    #[serde(rename = "SupportedLoadBalancerTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_load_balancer_types: Option<ListOfString>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Ciphers {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Cipher>,
}
impl From<Vec<Cipher>> for Ciphers {
    fn from(v: Vec<Cipher>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Cipher> for Ciphers {
    fn from_iter<I: IntoIterator<Item = Cipher>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Cipher>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlCipherList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Cipher>,
}

impl From<Vec<Cipher>> for XmlCipherList {
    fn from(v: Vec<Cipher>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Cipher> for XmlCipherList {
    fn from_iter<I: IntoIterator<Item = Cipher>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Cipher")]
pub struct Cipher {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SslProtocols {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for SslProtocols {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for SslProtocols {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOfString {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ListOfString {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ListOfString {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AddTagsInput")]
pub struct AddTagsInput {
    #[serde(rename = "ResourceArns")]
    #[serde(default)]
    pub resource_arns: ResourceArns,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: TagList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceArns {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ResourceArns {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ResourceArns {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AddTrustStoreRevocationsInput")]
pub struct AddTrustStoreRevocationsInput {
    #[serde(rename = "RevocationContents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_contents: Option<RevocationContents>,
    #[serde(rename = "TrustStoreArn")]
    #[serde(default)]
    pub trust_store_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RevocationContents {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<RevocationContent>,
}
impl From<Vec<RevocationContent>> for RevocationContents {
    fn from(v: Vec<RevocationContent>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<RevocationContent> for RevocationContents {
    fn from_iter<I: IntoIterator<Item = RevocationContent>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<RevocationContent>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlRevocationContentList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<RevocationContent>,
}

impl From<Vec<RevocationContent>> for XmlRevocationContentList {
    fn from(v: Vec<RevocationContent>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<RevocationContent> for XmlRevocationContentList {
    fn from_iter<I: IntoIterator<Item = RevocationContent>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RevocationContent")]
pub struct RevocationContent {
    #[serde(rename = "RevocationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_type: Option<String>,
    #[serde(rename = "S3Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket: Option<String>,
    #[serde(rename = "S3Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key: Option<String>,
    #[serde(rename = "S3ObjectVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeLoadBalancersInput")]
pub struct DescribeLoadBalancersInput {
    #[serde(rename = "LoadBalancerArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_arns: Option<LoadBalancerArns>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Names")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<LoadBalancerNames>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoadBalancerNames {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for LoadBalancerNames {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for LoadBalancerNames {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyIpPoolsInput")]
pub struct ModifyIpPoolsInput {
    #[serde(rename = "IpamPools")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipam_pools: Option<IpamPools>,
    #[serde(rename = "LoadBalancerArn")]
    #[serde(default)]
    pub load_balancer_arn: String,
    #[serde(rename = "RemoveIpamPools")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_ipam_pools: Option<RemoveIpamPools>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "IpamPools")]
pub struct IpamPools {
    #[serde(rename = "Ipv4IpamPoolId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv4_ipam_pool_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveIpamPools {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for RemoveIpamPools {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for RemoveIpamPools {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyListenerAttributesResult")]
pub struct ModifyListenerAttributesOutput {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<ListenerAttributes>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListenerAttributes {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ListenerAttribute>,
}
impl From<Vec<ListenerAttribute>> for ListenerAttributes {
    fn from(v: Vec<ListenerAttribute>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ListenerAttribute> for ListenerAttributes {
    fn from_iter<I: IntoIterator<Item = ListenerAttribute>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ListenerAttribute>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlListenerAttributeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ListenerAttribute>,
}

impl From<Vec<ListenerAttribute>> for XmlListenerAttributeList {
    fn from(v: Vec<ListenerAttribute>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ListenerAttribute> for XmlListenerAttributeList {
    fn from_iter<I: IntoIterator<Item = ListenerAttribute>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListenerAttribute")]
pub struct ListenerAttribute {
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
#[serde(rename = "CreateTrustStoreInput")]
pub struct CreateTrustStoreInput {
    #[serde(rename = "CaCertificatesBundleS3Bucket")]
    #[serde(default)]
    pub ca_certificates_bundle_s3_bucket: String,
    #[serde(rename = "CaCertificatesBundleS3Key")]
    #[serde(default)]
    pub ca_certificates_bundle_s3_key: String,
    #[serde(rename = "CaCertificatesBundleS3ObjectVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca_certificates_bundle_s3_object_version: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeLoadBalancersResult")]
pub struct DescribeLoadBalancersOutput {
    #[serde(rename = "LoadBalancers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancers: Option<LoadBalancers>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoadBalancers {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<LoadBalancer>,
}
impl From<Vec<LoadBalancer>> for LoadBalancers {
    fn from(v: Vec<LoadBalancer>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<LoadBalancer> for LoadBalancers {
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
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<AvailabilityZones>,
    #[serde(rename = "CanonicalHostedZoneId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_hosted_zone_id: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "CustomerOwnedIpv4Pool")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_owned_ipv4_pool: Option<String>,
    #[serde(rename = "DNSName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_n_s_name: Option<String>,
    #[serde(rename = "EnablePrefixForIpv6SourceNat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_prefix_for_ipv6_source_nat: Option<String>,
    #[serde(rename = "EnforceSecurityGroupInboundRulesOnPrivateLinkTraffic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce_security_group_inbound_rules_on_private_link_traffic: Option<String>,
    #[serde(rename = "IpAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(rename = "IpamPools")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipam_pools: Option<IpamPools>,
    #[serde(rename = "LoadBalancerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_arn: Option<String>,
    #[serde(rename = "LoadBalancerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_name: Option<String>,
    #[serde(rename = "Scheme")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    #[serde(rename = "SecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<SecurityGroups>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<LoadBalancerState>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecurityGroups {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for SecurityGroups {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for SecurityGroups {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LoadBalancerState")]
pub struct LoadBalancerState {
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "Reason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeTrustStoreRevocationsInput")]
pub struct DescribeTrustStoreRevocationsInput {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "RevocationIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_ids: Option<RevocationIds>,
    #[serde(rename = "TrustStoreArn")]
    #[serde(default)]
    pub trust_store_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RevocationIds {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<i64>,
}
impl From<Vec<i64>> for RevocationIds {
    fn from(v: Vec<i64>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<i64> for RevocationIds {
    fn from_iter<I: IntoIterator<Item = i64>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<i64>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Xmli64List {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<i64>,
}

impl From<Vec<i64>> for Xmli64List {
    fn from(v: Vec<i64>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<i64> for Xmli64List {
    fn from_iter<I: IntoIterator<Item = i64>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeCapacityReservationInput")]
pub struct DescribeCapacityReservationInput {
    #[serde(rename = "LoadBalancerArn")]
    #[serde(default)]
    pub load_balancer_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeListenerAttributesResult")]
pub struct DescribeListenerAttributesOutput {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<ListenerAttributes>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeTrustStoreRevocationsResult")]
pub struct DescribeTrustStoreRevocationsOutput {
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "TrustStoreRevocations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_store_revocations: Option<DescribeTrustStoreRevocationResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTrustStoreRevocationResponse {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<DescribeTrustStoreRevocation>,
}
impl From<Vec<DescribeTrustStoreRevocation>> for DescribeTrustStoreRevocationResponse {
    fn from(v: Vec<DescribeTrustStoreRevocation>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DescribeTrustStoreRevocation> for DescribeTrustStoreRevocationResponse {
    fn from_iter<I: IntoIterator<Item = DescribeTrustStoreRevocation>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DescribeTrustStoreRevocation>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDescribeTrustStoreRevocationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DescribeTrustStoreRevocation>,
}

impl From<Vec<DescribeTrustStoreRevocation>> for XmlDescribeTrustStoreRevocationList {
    fn from(v: Vec<DescribeTrustStoreRevocation>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DescribeTrustStoreRevocation> for XmlDescribeTrustStoreRevocationList {
    fn from_iter<I: IntoIterator<Item = DescribeTrustStoreRevocation>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeTrustStoreRevocation")]
pub struct DescribeTrustStoreRevocation {
    #[serde(rename = "NumberOfRevokedEntries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_revoked_entries: Option<i64>,
    #[serde(rename = "RevocationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_id: Option<i64>,
    #[serde(rename = "RevocationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_type: Option<String>,
    #[serde(rename = "TrustStoreArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_store_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateListenerResult")]
pub struct CreateListenerOutput {
    #[serde(rename = "Listeners")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listeners: Option<Listeners>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyLoadBalancerAttributesResult")]
pub struct ModifyLoadBalancerAttributesOutput {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<LoadBalancerAttributes>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoadBalancerAttributes {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<LoadBalancerAttribute>,
}
impl From<Vec<LoadBalancerAttribute>> for LoadBalancerAttributes {
    fn from(v: Vec<LoadBalancerAttribute>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<LoadBalancerAttribute> for LoadBalancerAttributes {
    fn from_iter<I: IntoIterator<Item = LoadBalancerAttribute>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<LoadBalancerAttribute>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlLoadBalancerAttributeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<LoadBalancerAttribute>,
}

impl From<Vec<LoadBalancerAttribute>> for XmlLoadBalancerAttributeList {
    fn from(v: Vec<LoadBalancerAttribute>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<LoadBalancerAttribute> for XmlLoadBalancerAttributeList {
    fn from_iter<I: IntoIterator<Item = LoadBalancerAttribute>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LoadBalancerAttribute")]
pub struct LoadBalancerAttribute {
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
pub struct DeregisterTargetsOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveTrustStoreRevocationsOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SetRulePrioritiesResult")]
pub struct SetRulePrioritiesOutput {
    #[serde(rename = "Rules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Rules>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Rules {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Rule>,
}
impl From<Vec<Rule>> for Rules {
    fn from(v: Vec<Rule>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Rule> for Rules {
    fn from_iter<I: IntoIterator<Item = Rule>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Rule>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlRuleList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Rule>,
}

impl From<Vec<Rule>> for XmlRuleList {
    fn from(v: Vec<Rule>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Rule> for XmlRuleList {
    fn from_iter<I: IntoIterator<Item = Rule>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Rule")]
pub struct Rule {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Actions>,
    #[serde(rename = "Conditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<RuleConditionList>,
    #[serde(rename = "IsDefault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[serde(rename = "RuleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_arn: Option<String>,
    #[serde(rename = "Transforms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transforms: Option<RuleTransformList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleConditionList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<RuleCondition>,
}
impl From<Vec<RuleCondition>> for RuleConditionList {
    fn from(v: Vec<RuleCondition>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<RuleCondition> for RuleConditionList {
    fn from_iter<I: IntoIterator<Item = RuleCondition>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<RuleCondition>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlRuleConditionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<RuleCondition>,
}

impl From<Vec<RuleCondition>> for XmlRuleConditionList {
    fn from(v: Vec<RuleCondition>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<RuleCondition> for XmlRuleConditionList {
    fn from_iter<I: IntoIterator<Item = RuleCondition>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RuleCondition")]
pub struct RuleCondition {
    #[serde(rename = "Field")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(rename = "HostHeaderConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_header_config: Option<HostHeaderConditionConfig>,
    #[serde(rename = "HttpHeaderConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_header_config: Option<HttpHeaderConditionConfig>,
    #[serde(rename = "HttpRequestMethodConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_request_method_config: Option<HttpRequestMethodConditionConfig>,
    #[serde(rename = "PathPatternConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_pattern_config: Option<PathPatternConditionConfig>,
    #[serde(rename = "QueryStringConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string_config: Option<QueryStringConditionConfig>,
    #[serde(rename = "RegexValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_values: Option<ListOfString>,
    #[serde(rename = "SourceIpConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip_config: Option<SourceIpConditionConfig>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<ListOfString>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "HostHeaderConditionConfig")]
pub struct HostHeaderConditionConfig {
    #[serde(rename = "RegexValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_values: Option<ListOfString>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<ListOfString>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "HttpHeaderConditionConfig")]
pub struct HttpHeaderConditionConfig {
    #[serde(rename = "HttpHeaderName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_header_name: Option<String>,
    #[serde(rename = "RegexValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_values: Option<ListOfString>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<ListOfString>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "HttpRequestMethodConditionConfig")]
pub struct HttpRequestMethodConditionConfig {
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<ListOfString>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PathPatternConditionConfig")]
pub struct PathPatternConditionConfig {
    #[serde(rename = "RegexValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_values: Option<ListOfString>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<ListOfString>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "QueryStringConditionConfig")]
pub struct QueryStringConditionConfig {
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<QueryStringKeyValuePairList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryStringKeyValuePairList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<QueryStringKeyValuePair>,
}
impl From<Vec<QueryStringKeyValuePair>> for QueryStringKeyValuePairList {
    fn from(v: Vec<QueryStringKeyValuePair>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<QueryStringKeyValuePair> for QueryStringKeyValuePairList {
    fn from_iter<I: IntoIterator<Item = QueryStringKeyValuePair>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<QueryStringKeyValuePair>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlQueryStringKeyValuePairList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<QueryStringKeyValuePair>,
}

impl From<Vec<QueryStringKeyValuePair>> for XmlQueryStringKeyValuePairList {
    fn from(v: Vec<QueryStringKeyValuePair>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<QueryStringKeyValuePair> for XmlQueryStringKeyValuePairList {
    fn from_iter<I: IntoIterator<Item = QueryStringKeyValuePair>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "QueryStringKeyValuePair")]
pub struct QueryStringKeyValuePair {
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
#[serde(rename = "SourceIpConditionConfig")]
pub struct SourceIpConditionConfig {
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<ListOfString>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleTransformList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<RuleTransform>,
}
impl From<Vec<RuleTransform>> for RuleTransformList {
    fn from(v: Vec<RuleTransform>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<RuleTransform> for RuleTransformList {
    fn from_iter<I: IntoIterator<Item = RuleTransform>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<RuleTransform>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlRuleTransformList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<RuleTransform>,
}

impl From<Vec<RuleTransform>> for XmlRuleTransformList {
    fn from(v: Vec<RuleTransform>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<RuleTransform> for XmlRuleTransformList {
    fn from_iter<I: IntoIterator<Item = RuleTransform>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RuleTransform")]
pub struct RuleTransform {
    #[serde(rename = "HostHeaderRewriteConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_header_rewrite_config: Option<HostHeaderRewriteConfig>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "UrlRewriteConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_rewrite_config: Option<UrlRewriteConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "HostHeaderRewriteConfig")]
pub struct HostHeaderRewriteConfig {
    #[serde(rename = "Rewrites")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewrites: Option<RewriteConfigList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RewriteConfigList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<RewriteConfig>,
}
impl From<Vec<RewriteConfig>> for RewriteConfigList {
    fn from(v: Vec<RewriteConfig>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<RewriteConfig> for RewriteConfigList {
    fn from_iter<I: IntoIterator<Item = RewriteConfig>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<RewriteConfig>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlRewriteConfigList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<RewriteConfig>,
}

impl From<Vec<RewriteConfig>> for XmlRewriteConfigList {
    fn from(v: Vec<RewriteConfig>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<RewriteConfig> for XmlRewriteConfigList {
    fn from_iter<I: IntoIterator<Item = RewriteConfig>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RewriteConfig")]
pub struct RewriteConfig {
    #[serde(rename = "Regex")]
    #[serde(default)]
    pub regex: String,
    #[serde(rename = "Replace")]
    #[serde(default)]
    pub replace: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UrlRewriteConfig")]
pub struct UrlRewriteConfig {
    #[serde(rename = "Rewrites")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewrites: Option<RewriteConfigList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetTrustStoreCaCertificatesBundleResult")]
pub struct GetTrustStoreCaCertificatesBundleOutput {
    #[serde(rename = "Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SetSecurityGroupsInput")]
pub struct SetSecurityGroupsInput {
    #[serde(rename = "EnforceSecurityGroupInboundRulesOnPrivateLinkTraffic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce_security_group_inbound_rules_on_private_link_traffic: Option<String>,
    #[serde(rename = "LoadBalancerArn")]
    #[serde(default)]
    pub load_balancer_arn: String,
    #[serde(rename = "SecurityGroups")]
    #[serde(default)]
    pub security_groups: SecurityGroups,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SetRulePrioritiesInput")]
pub struct SetRulePrioritiesInput {
    #[serde(rename = "RulePriorities")]
    #[serde(default)]
    pub rule_priorities: RulePriorityList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RulePriorityList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<RulePriorityPair>,
}
impl From<Vec<RulePriorityPair>> for RulePriorityList {
    fn from(v: Vec<RulePriorityPair>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<RulePriorityPair> for RulePriorityList {
    fn from_iter<I: IntoIterator<Item = RulePriorityPair>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<RulePriorityPair>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlRulePriorityPairList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<RulePriorityPair>,
}

impl From<Vec<RulePriorityPair>> for XmlRulePriorityPairList {
    fn from(v: Vec<RulePriorityPair>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<RulePriorityPair> for XmlRulePriorityPairList {
    fn from_iter<I: IntoIterator<Item = RulePriorityPair>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RulePriorityPair")]
pub struct RulePriorityPair {
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "RuleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeTrustStoresInput")]
pub struct DescribeTrustStoresInput {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Names")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<TrustStoreNames>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "TrustStoreArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_store_arns: Option<TrustStoreArns>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrustStoreNames {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for TrustStoreNames {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for TrustStoreNames {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrustStoreArns {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for TrustStoreArns {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for TrustStoreArns {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetTrustStoreRevocationContentResult")]
pub struct GetTrustStoreRevocationContentOutput {
    #[serde(rename = "Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateTargetGroupInput")]
pub struct CreateTargetGroupInput {
    #[serde(rename = "HealthCheckEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_enabled: Option<bool>,
    #[serde(rename = "HealthCheckIntervalSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_interval_seconds: Option<i32>,
    #[serde(rename = "HealthCheckPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_path: Option<String>,
    #[serde(rename = "HealthCheckPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_port: Option<String>,
    #[serde(rename = "HealthCheckProtocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_protocol: Option<String>,
    #[serde(rename = "HealthCheckTimeoutSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_timeout_seconds: Option<i32>,
    #[serde(rename = "HealthyThresholdCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healthy_threshold_count: Option<i32>,
    #[serde(rename = "IpAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(rename = "Matcher")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matcher: Option<Matcher>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "Protocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "ProtocolVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_version: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "TargetControlPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_control_port: Option<i32>,
    #[serde(rename = "TargetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    #[serde(rename = "UnhealthyThresholdCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhealthy_threshold_count: Option<i32>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLoadBalancerOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateListenerInput")]
pub struct CreateListenerInput {
    #[serde(rename = "AlpnPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpn_policy: Option<AlpnPolicyName>,
    #[serde(rename = "Certificates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates: Option<CertificateList>,
    #[serde(rename = "DefaultActions")]
    #[serde(default)]
    pub default_actions: Actions,
    #[serde(rename = "LoadBalancerArn")]
    #[serde(default)]
    pub load_balancer_arn: String,
    #[serde(rename = "MutualAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutual_authentication: Option<MutualAuthenticationAttributes>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "Protocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "SslPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_policy: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeTargetHealthResult")]
pub struct DescribeTargetHealthOutput {
    #[serde(rename = "TargetHealthDescriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_health_descriptions: Option<TargetHealthDescriptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetHealthDescriptions {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<TargetHealthDescription>,
}
impl From<Vec<TargetHealthDescription>> for TargetHealthDescriptions {
    fn from(v: Vec<TargetHealthDescription>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<TargetHealthDescription> for TargetHealthDescriptions {
    fn from_iter<I: IntoIterator<Item = TargetHealthDescription>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<TargetHealthDescription>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTargetHealthDescriptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<TargetHealthDescription>,
}

impl From<Vec<TargetHealthDescription>> for XmlTargetHealthDescriptionList {
    fn from(v: Vec<TargetHealthDescription>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<TargetHealthDescription> for XmlTargetHealthDescriptionList {
    fn from_iter<I: IntoIterator<Item = TargetHealthDescription>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TargetHealthDescription")]
pub struct TargetHealthDescription {
    #[serde(rename = "AdministrativeOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrative_override: Option<AdministrativeOverride>,
    #[serde(rename = "AnomalyDetection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anomaly_detection: Option<AnomalyDetection>,
    #[serde(rename = "HealthCheckPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_port: Option<String>,
    #[serde(rename = "Target")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<TargetDescription>,
    #[serde(rename = "TargetHealth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_health: Option<TargetHealth>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AdministrativeOverride")]
pub struct AdministrativeOverride {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Reason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AnomalyDetection")]
pub struct AnomalyDetection {
    #[serde(rename = "MitigationInEffect")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mitigation_in_effect: Option<String>,
    #[serde(rename = "Result")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TargetHealth")]
pub struct TargetHealth {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Reason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteRuleInput")]
pub struct DeleteRuleInput {
    #[serde(rename = "RuleArn")]
    #[serde(default)]
    pub rule_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateLoadBalancerResult")]
pub struct CreateLoadBalancerOutput {
    #[serde(rename = "LoadBalancers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancers: Option<LoadBalancers>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeAccountLimitsInput")]
pub struct DescribeAccountLimitsInput {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyCapacityReservationInput")]
pub struct ModifyCapacityReservationInput {
    #[serde(rename = "LoadBalancerArn")]
    #[serde(default)]
    pub load_balancer_arn: String,
    #[serde(rename = "MinimumLoadBalancerCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_load_balancer_capacity: Option<MinimumLoadBalancerCapacity>,
    #[serde(rename = "ResetCapacityReservation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_capacity_reservation: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MinimumLoadBalancerCapacity")]
pub struct MinimumLoadBalancerCapacity {
    #[serde(rename = "CapacityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_units: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeregisterTargetsInput")]
pub struct DeregisterTargetsInput {
    #[serde(rename = "TargetGroupArn")]
    #[serde(default)]
    pub target_group_arn: String,
    #[serde(rename = "Targets")]
    #[serde(default)]
    pub targets: TargetDescriptions,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyCapacityReservationResult")]
pub struct ModifyCapacityReservationOutput {
    #[serde(rename = "CapacityReservationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_reservation_state: Option<ZonalCapacityReservationStates>,
    #[serde(rename = "DecreaseRequestsRemaining")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decrease_requests_remaining: Option<i32>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(rename = "MinimumLoadBalancerCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_load_balancer_capacity: Option<MinimumLoadBalancerCapacity>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ZonalCapacityReservationStates {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ZonalCapacityReservationState>,
}
impl From<Vec<ZonalCapacityReservationState>> for ZonalCapacityReservationStates {
    fn from(v: Vec<ZonalCapacityReservationState>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ZonalCapacityReservationState> for ZonalCapacityReservationStates {
    fn from_iter<I: IntoIterator<Item = ZonalCapacityReservationState>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ZonalCapacityReservationState>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlZonalCapacityReservationStateList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ZonalCapacityReservationState>,
}

impl From<Vec<ZonalCapacityReservationState>> for XmlZonalCapacityReservationStateList {
    fn from(v: Vec<ZonalCapacityReservationState>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ZonalCapacityReservationState> for XmlZonalCapacityReservationStateList {
    fn from_iter<I: IntoIterator<Item = ZonalCapacityReservationState>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ZonalCapacityReservationState")]
pub struct ZonalCapacityReservationState {
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "EffectiveCapacityUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_capacity_units: Option<f64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<CapacityReservationStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CapacityReservationStatus")]
pub struct CapacityReservationStatus {
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "Reason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeListenerCertificatesResult")]
pub struct DescribeListenerCertificatesOutput {
    #[serde(rename = "Certificates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates: Option<CertificateList>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeLoadBalancerAttributesResult")]
pub struct DescribeLoadBalancerAttributesOutput {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<LoadBalancerAttributes>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyListenerAttributesInput")]
pub struct ModifyListenerAttributesInput {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    pub attributes: ListenerAttributes,
    #[serde(rename = "ListenerArn")]
    #[serde(default)]
    pub listener_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyRuleInput")]
pub struct ModifyRuleInput {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Actions>,
    #[serde(rename = "Conditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<RuleConditionList>,
    #[serde(rename = "ResetTransforms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_transforms: Option<bool>,
    #[serde(rename = "RuleArn")]
    #[serde(default)]
    pub rule_arn: String,
    #[serde(rename = "Transforms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transforms: Option<RuleTransformList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RemoveTagsInput")]
pub struct RemoveTagsInput {
    #[serde(rename = "ResourceArns")]
    #[serde(default)]
    pub resource_arns: ResourceArns,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: TagKeys,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagKeys {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for TagKeys {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for TagKeys {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RemoveTrustStoreRevocationsInput")]
pub struct RemoveTrustStoreRevocationsInput {
    #[serde(rename = "RevocationIds")]
    #[serde(default)]
    pub revocation_ids: RevocationIds,
    #[serde(rename = "TrustStoreArn")]
    #[serde(default)]
    pub trust_store_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SetSubnetsInput")]
pub struct SetSubnetsInput {
    #[serde(rename = "EnablePrefixForIpv6SourceNat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_prefix_for_ipv6_source_nat: Option<String>,
    #[serde(rename = "IpAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(rename = "LoadBalancerArn")]
    #[serde(default)]
    pub load_balancer_arn: String,
    #[serde(rename = "SubnetMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_mappings: Option<SubnetMappings>,
    #[serde(rename = "Subnets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Subnets>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubnetMappings {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<SubnetMapping>,
}
impl From<Vec<SubnetMapping>> for SubnetMappings {
    fn from(v: Vec<SubnetMapping>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<SubnetMapping> for SubnetMappings {
    fn from_iter<I: IntoIterator<Item = SubnetMapping>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<SubnetMapping>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlSubnetMappingList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<SubnetMapping>,
}

impl From<Vec<SubnetMapping>> for XmlSubnetMappingList {
    fn from(v: Vec<SubnetMapping>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<SubnetMapping> for XmlSubnetMappingList {
    fn from_iter<I: IntoIterator<Item = SubnetMapping>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SubnetMapping")]
pub struct SubnetMapping {
    #[serde(rename = "AllocationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation_id: Option<String>,
    #[serde(rename = "IPv6Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_pv6_address: Option<String>,
    #[serde(rename = "PrivateIPv4Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_i_pv4_address: Option<String>,
    #[serde(rename = "SourceNatIpv6Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_nat_ipv6_prefix: Option<String>,
    #[serde(rename = "SubnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Subnets {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for Subnets {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for Subnets {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateLoadBalancerInput")]
pub struct CreateLoadBalancerInput {
    #[serde(rename = "CustomerOwnedIpv4Pool")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_owned_ipv4_pool: Option<String>,
    #[serde(rename = "EnablePrefixForIpv6SourceNat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_prefix_for_ipv6_source_nat: Option<String>,
    #[serde(rename = "IpAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(rename = "IpamPools")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipam_pools: Option<IpamPools>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Scheme")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    #[serde(rename = "SecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<SecurityGroups>,
    #[serde(rename = "SubnetMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_mappings: Option<SubnetMappings>,
    #[serde(rename = "Subnets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Subnets>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeRulesInput")]
pub struct DescribeRulesInput {
    #[serde(rename = "ListenerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_arn: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "RuleArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_arns: Option<RuleArns>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuleArns {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for RuleArns {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for RuleArns {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeLoadBalancerAttributesInput")]
pub struct DescribeLoadBalancerAttributesInput {
    #[serde(rename = "LoadBalancerArn")]
    #[serde(default)]
    pub load_balancer_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateRuleResult")]
pub struct CreateRuleOutput {
    #[serde(rename = "Rules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Rules>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteTrustStoreInput")]
pub struct DeleteTrustStoreInput {
    #[serde(rename = "TrustStoreArn")]
    #[serde(default)]
    pub trust_store_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeRulesResult")]
pub struct DescribeRulesOutput {
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "Rules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Rules>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeSSLPoliciesInput")]
pub struct DescribeSSLPoliciesInput {
    #[serde(rename = "LoadBalancerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_type: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Names")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<SslPolicyNames>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SslPolicyNames {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for SslPolicyNames {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for SslPolicyNames {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteListenerInput")]
pub struct DeleteListenerInput {
    #[serde(rename = "ListenerArn")]
    #[serde(default)]
    pub listener_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetTrustStoreRevocationContentInput")]
pub struct GetTrustStoreRevocationContentInput {
    #[serde(rename = "RevocationId")]
    #[serde(default)]
    pub revocation_id: i64,
    #[serde(rename = "TrustStoreArn")]
    #[serde(default)]
    pub trust_store_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTargetGroupOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyLoadBalancerAttributesInput")]
pub struct ModifyLoadBalancerAttributesInput {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    pub attributes: LoadBalancerAttributes,
    #[serde(rename = "LoadBalancerArn")]
    #[serde(default)]
    pub load_balancer_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RegisterTargetsInput")]
pub struct RegisterTargetsInput {
    #[serde(rename = "TargetGroupArn")]
    #[serde(default)]
    pub target_group_arn: String,
    #[serde(rename = "Targets")]
    #[serde(default)]
    pub targets: TargetDescriptions,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SetIpAddressTypeInput")]
pub struct SetIpAddressTypeInput {
    #[serde(rename = "IpAddressType")]
    #[serde(default)]
    pub ip_address_type: String,
    #[serde(rename = "LoadBalancerArn")]
    #[serde(default)]
    pub load_balancer_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeTagsInput")]
pub struct DescribeTagsInput {
    #[serde(rename = "ResourceArns")]
    #[serde(default)]
    pub resource_arns: ResourceArns,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeTargetGroupsInput")]
pub struct DescribeTargetGroupsInput {
    #[serde(rename = "LoadBalancerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_arn: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Names")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<TargetGroupNames>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "TargetGroupArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_group_arns: Option<TargetGroupArns>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetGroupNames {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for TargetGroupNames {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for TargetGroupNames {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetGroupArns {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for TargetGroupArns {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for TargetGroupArns {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetResourcePolicyResult")]
pub struct GetResourcePolicyOutput {
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyListenerInput")]
pub struct ModifyListenerInput {
    #[serde(rename = "AlpnPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpn_policy: Option<AlpnPolicyName>,
    #[serde(rename = "Certificates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates: Option<CertificateList>,
    #[serde(rename = "DefaultActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_actions: Option<Actions>,
    #[serde(rename = "ListenerArn")]
    #[serde(default)]
    pub listener_arn: String,
    #[serde(rename = "MutualAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutual_authentication: Option<MutualAuthenticationAttributes>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "Protocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(rename = "SslPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyTrustStoreResult")]
pub struct ModifyTrustStoreOutput {
    #[serde(rename = "TrustStores")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_stores: Option<TrustStores>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTrustStoreOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AddListenerCertificatesResult")]
pub struct AddListenerCertificatesOutput {
    #[serde(rename = "Certificates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates: Option<CertificateList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AddListenerCertificatesInput")]
pub struct AddListenerCertificatesInput {
    #[serde(rename = "Certificates")]
    #[serde(default)]
    pub certificates: CertificateList,
    #[serde(rename = "ListenerArn")]
    #[serde(default)]
    pub listener_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterTargetsOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyIpPoolsResult")]
pub struct ModifyIpPoolsOutput {
    #[serde(rename = "IpamPools")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipam_pools: Option<IpamPools>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyTargetGroupAttributesResult")]
pub struct ModifyTargetGroupAttributesOutput {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<TargetGroupAttributes>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveTagsOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeTargetGroupAttributesInput")]
pub struct DescribeTargetGroupAttributesInput {
    #[serde(rename = "TargetGroupArn")]
    #[serde(default)]
    pub target_group_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeListenersResult")]
pub struct DescribeListenersOutput {
    #[serde(rename = "Listeners")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listeners: Option<Listeners>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeListenerCertificatesInput")]
pub struct DescribeListenerCertificatesInput {
    #[serde(rename = "ListenerArn")]
    #[serde(default)]
    pub listener_arn: String,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyTargetGroupAttributesInput")]
pub struct ModifyTargetGroupAttributesInput {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    pub attributes: TargetGroupAttributes,
    #[serde(rename = "TargetGroupArn")]
    #[serde(default)]
    pub target_group_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateTrustStoreResult")]
pub struct CreateTrustStoreOutput {
    #[serde(rename = "TrustStores")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_stores: Option<TrustStores>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateRuleInput")]
pub struct CreateRuleInput {
    #[serde(rename = "Actions")]
    #[serde(default)]
    pub actions: Actions,
    #[serde(rename = "Conditions")]
    #[serde(default)]
    pub conditions: RuleConditionList,
    #[serde(rename = "ListenerArn")]
    #[serde(default)]
    pub listener_arn: String,
    #[serde(rename = "Priority")]
    #[serde(default)]
    pub priority: i32,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "Transforms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transforms: Option<RuleTransformList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeCapacityReservationResult")]
pub struct DescribeCapacityReservationOutput {
    #[serde(rename = "CapacityReservationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_reservation_state: Option<ZonalCapacityReservationStates>,
    #[serde(rename = "DecreaseRequestsRemaining")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decrease_requests_remaining: Option<i32>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<String>,
    #[serde(rename = "MinimumLoadBalancerCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_load_balancer_capacity: Option<MinimumLoadBalancerCapacity>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteSharedTrustStoreAssociationInput")]
pub struct DeleteSharedTrustStoreAssociationInput {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "TrustStoreArn")]
    #[serde(default)]
    pub trust_store_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeListenerAttributesInput")]
pub struct DescribeListenerAttributesInput {
    #[serde(rename = "ListenerArn")]
    #[serde(default)]
    pub listener_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyRuleResult")]
pub struct ModifyRuleOutput {
    #[serde(rename = "Rules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Rules>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeTrustStoreAssociationsResult")]
pub struct DescribeTrustStoreAssociationsOutput {
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "TrustStoreAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_store_associations: Option<TrustStoreAssociations>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrustStoreAssociations {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<TrustStoreAssociation>,
}
impl From<Vec<TrustStoreAssociation>> for TrustStoreAssociations {
    fn from(v: Vec<TrustStoreAssociation>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<TrustStoreAssociation> for TrustStoreAssociations {
    fn from_iter<I: IntoIterator<Item = TrustStoreAssociation>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<TrustStoreAssociation>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTrustStoreAssociationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<TrustStoreAssociation>,
}

impl From<Vec<TrustStoreAssociation>> for XmlTrustStoreAssociationList {
    fn from(v: Vec<TrustStoreAssociation>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<TrustStoreAssociation> for XmlTrustStoreAssociationList {
    fn from_iter<I: IntoIterator<Item = TrustStoreAssociation>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TrustStoreAssociation")]
pub struct TrustStoreAssociation {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeTrustStoreAssociationsInput")]
pub struct DescribeTrustStoreAssociationsInput {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "TrustStoreArn")]
    #[serde(default)]
    pub trust_store_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SetSecurityGroupsResult")]
pub struct SetSecurityGroupsOutput {
    #[serde(rename = "EnforceSecurityGroupInboundRulesOnPrivateLinkTraffic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce_security_group_inbound_rules_on_private_link_traffic: Option<String>,
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<SecurityGroups>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeTargetGroupsResult")]
pub struct DescribeTargetGroupsOutput {
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    #[serde(rename = "TargetGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_groups: Option<TargetGroups>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AddTrustStoreRevocationsResult")]
pub struct AddTrustStoreRevocationsOutput {
    #[serde(rename = "TrustStoreRevocations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_store_revocations: Option<TrustStoreRevocations>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrustStoreRevocations {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<TrustStoreRevocation>,
}
impl From<Vec<TrustStoreRevocation>> for TrustStoreRevocations {
    fn from(v: Vec<TrustStoreRevocation>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<TrustStoreRevocation> for TrustStoreRevocations {
    fn from_iter<I: IntoIterator<Item = TrustStoreRevocation>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<TrustStoreRevocation>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTrustStoreRevocationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<TrustStoreRevocation>,
}

impl From<Vec<TrustStoreRevocation>> for XmlTrustStoreRevocationList {
    fn from(v: Vec<TrustStoreRevocation>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<TrustStoreRevocation> for XmlTrustStoreRevocationList {
    fn from_iter<I: IntoIterator<Item = TrustStoreRevocation>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TrustStoreRevocation")]
pub struct TrustStoreRevocation {
    #[serde(rename = "NumberOfRevokedEntries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_revoked_entries: Option<i64>,
    #[serde(rename = "RevocationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_id: Option<i64>,
    #[serde(rename = "RevocationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_type: Option<String>,
    #[serde(rename = "TrustStoreArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_store_arn: Option<String>,
}
