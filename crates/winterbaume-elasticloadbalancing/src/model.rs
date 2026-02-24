//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-elastic-load-balancing

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AddAvailabilityZonesInput")]
pub struct AddAvailabilityZonesInput {
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    pub availability_zones: AvailabilityZones,
    #[serde(rename = "LoadBalancerName")]
    #[serde(default)]
    pub load_balancer_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AvailabilityZones {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for AvailabilityZones {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for AvailabilityZones {
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
#[serde(rename = "EnableAvailabilityZonesForLoadBalancerResult")]
pub struct AddAvailabilityZonesOutput {
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<AvailabilityZones>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AddTagsInput")]
pub struct AddTagsInput {
    #[serde(rename = "LoadBalancerNames")]
    #[serde(default)]
    pub load_balancer_names: LoadBalancerNames,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: TagList,
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
pub struct AddTagsOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ApplySecurityGroupsToLoadBalancerInput")]
pub struct ApplySecurityGroupsToLoadBalancerInput {
    #[serde(rename = "LoadBalancerName")]
    #[serde(default)]
    pub load_balancer_name: String,
    #[serde(rename = "SecurityGroups")]
    #[serde(default)]
    pub security_groups: SecurityGroups,
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
#[serde(rename = "ApplySecurityGroupsToLoadBalancerResult")]
pub struct ApplySecurityGroupsToLoadBalancerOutput {
    #[serde(rename = "SecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<SecurityGroups>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AttachLoadBalancerToSubnetsInput")]
pub struct AttachLoadBalancerToSubnetsInput {
    #[serde(rename = "LoadBalancerName")]
    #[serde(default)]
    pub load_balancer_name: String,
    #[serde(rename = "Subnets")]
    #[serde(default)]
    pub subnets: Subnets,
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
#[serde(rename = "AttachLoadBalancerToSubnetsResult")]
pub struct AttachLoadBalancerToSubnetsOutput {
    #[serde(rename = "Subnets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Subnets>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ConfigureHealthCheckInput")]
pub struct ConfigureHealthCheckInput {
    #[serde(rename = "HealthCheck")]
    #[serde(default)]
    pub health_check: HealthCheck,
    #[serde(rename = "LoadBalancerName")]
    #[serde(default)]
    pub load_balancer_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "HealthCheck")]
pub struct HealthCheck {
    #[serde(rename = "HealthyThreshold")]
    #[serde(default)]
    pub healthy_threshold: i32,
    #[serde(rename = "Interval")]
    #[serde(default)]
    pub interval: i32,
    #[serde(rename = "Target")]
    #[serde(default)]
    pub target: String,
    #[serde(rename = "Timeout")]
    #[serde(default)]
    pub timeout: i32,
    #[serde(rename = "UnhealthyThreshold")]
    #[serde(default)]
    pub unhealthy_threshold: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ConfigureHealthCheckResult")]
pub struct ConfigureHealthCheckOutput {
    #[serde(rename = "HealthCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<HealthCheck>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateAccessPointInput")]
pub struct CreateAccessPointInput {
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<AvailabilityZones>,
    #[serde(rename = "Listeners")]
    #[serde(default)]
    pub listeners: Listeners,
    #[serde(rename = "LoadBalancerName")]
    #[serde(default)]
    pub load_balancer_name: String,
    #[serde(rename = "Scheme")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    #[serde(rename = "SecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<SecurityGroups>,
    #[serde(rename = "Subnets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Subnets>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
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
    #[serde(rename = "InstancePort")]
    #[serde(default)]
    pub instance_port: i32,
    #[serde(rename = "InstanceProtocol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_protocol: Option<String>,
    #[serde(rename = "LoadBalancerPort")]
    #[serde(default)]
    pub load_balancer_port: i32,
    #[serde(rename = "Protocol")]
    #[serde(default)]
    pub protocol: String,
    #[serde(rename = "SSLCertificateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_l_certificate_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateLoadBalancerResult")]
pub struct CreateAccessPointOutput {
    #[serde(rename = "DNSName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_n_s_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateAppCookieStickinessPolicyInput")]
pub struct CreateAppCookieStickinessPolicyInput {
    #[serde(rename = "CookieName")]
    #[serde(default)]
    pub cookie_name: String,
    #[serde(rename = "LoadBalancerName")]
    #[serde(default)]
    pub load_balancer_name: String,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    pub policy_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAppCookieStickinessPolicyOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateLBCookieStickinessPolicyInput")]
pub struct CreateLBCookieStickinessPolicyInput {
    #[serde(rename = "CookieExpirationPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_expiration_period: Option<i64>,
    #[serde(rename = "LoadBalancerName")]
    #[serde(default)]
    pub load_balancer_name: String,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    pub policy_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLBCookieStickinessPolicyOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateLoadBalancerListenerInput")]
pub struct CreateLoadBalancerListenerInput {
    #[serde(rename = "Listeners")]
    #[serde(default)]
    pub listeners: Listeners,
    #[serde(rename = "LoadBalancerName")]
    #[serde(default)]
    pub load_balancer_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLoadBalancerListenerOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateLoadBalancerPolicyInput")]
pub struct CreateLoadBalancerPolicyInput {
    #[serde(rename = "LoadBalancerName")]
    #[serde(default)]
    pub load_balancer_name: String,
    #[serde(rename = "PolicyAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_attributes: Option<PolicyAttributes>,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    pub policy_name: String,
    #[serde(rename = "PolicyTypeName")]
    #[serde(default)]
    pub policy_type_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PolicyAttributes {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PolicyAttribute>,
}
impl From<Vec<PolicyAttribute>> for PolicyAttributes {
    fn from(v: Vec<PolicyAttribute>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<PolicyAttribute> for PolicyAttributes {
    fn from_iter<I: IntoIterator<Item = PolicyAttribute>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<PolicyAttribute>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlPolicyAttributeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<PolicyAttribute>,
}

impl From<Vec<PolicyAttribute>> for XmlPolicyAttributeList {
    fn from(v: Vec<PolicyAttribute>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<PolicyAttribute> for XmlPolicyAttributeList {
    fn from_iter<I: IntoIterator<Item = PolicyAttribute>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PolicyAttribute")]
pub struct PolicyAttribute {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[serde(rename = "AttributeValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLoadBalancerPolicyOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteAccessPointInput")]
pub struct DeleteAccessPointInput {
    #[serde(rename = "LoadBalancerName")]
    #[serde(default)]
    pub load_balancer_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAccessPointOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteLoadBalancerListenerInput")]
pub struct DeleteLoadBalancerListenerInput {
    #[serde(rename = "LoadBalancerName")]
    #[serde(default)]
    pub load_balancer_name: String,
    #[serde(rename = "LoadBalancerPorts")]
    #[serde(default)]
    pub load_balancer_ports: Ports,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Ports {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<i32>,
}
impl From<Vec<i32>> for Ports {
    fn from(v: Vec<i32>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<i32> for Ports {
    fn from_iter<I: IntoIterator<Item = i32>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<i32>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Xmli32List {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<i32>,
}

impl From<Vec<i32>> for Xmli32List {
    fn from(v: Vec<i32>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<i32> for Xmli32List {
    fn from_iter<I: IntoIterator<Item = i32>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLoadBalancerListenerOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteLoadBalancerPolicyInput")]
pub struct DeleteLoadBalancerPolicyInput {
    #[serde(rename = "LoadBalancerName")]
    #[serde(default)]
    pub load_balancer_name: String,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    pub policy_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLoadBalancerPolicyOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeregisterEndPointsInput")]
pub struct DeregisterEndPointsInput {
    #[serde(rename = "Instances")]
    #[serde(default)]
    pub instances: Instances,
    #[serde(rename = "LoadBalancerName")]
    #[serde(default)]
    pub load_balancer_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Instances {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Instance>,
}
impl From<Vec<Instance>> for Instances {
    fn from(v: Vec<Instance>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Instance> for Instances {
    fn from_iter<I: IntoIterator<Item = Instance>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Instance>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlInstanceList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Instance>,
}

impl From<Vec<Instance>> for XmlInstanceList {
    fn from(v: Vec<Instance>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Instance> for XmlInstanceList {
    fn from_iter<I: IntoIterator<Item = Instance>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Instance")]
pub struct Instance {
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeregisterInstancesFromLoadBalancerResult")]
pub struct DeregisterEndPointsOutput {
    #[serde(rename = "Instances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Instances>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeAccessPointsInput")]
pub struct DescribeAccessPointsInput {
    #[serde(rename = "LoadBalancerNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_names: Option<LoadBalancerNames>,
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
#[serde(rename = "DescribeLoadBalancersResult")]
pub struct DescribeAccessPointsOutput {
    #[serde(rename = "LoadBalancerDescriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_descriptions: Option<LoadBalancerDescriptions>,
    #[serde(rename = "NextMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoadBalancerDescriptions {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<LoadBalancerDescription>,
}
impl From<Vec<LoadBalancerDescription>> for LoadBalancerDescriptions {
    fn from(v: Vec<LoadBalancerDescription>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<LoadBalancerDescription> for LoadBalancerDescriptions {
    fn from_iter<I: IntoIterator<Item = LoadBalancerDescription>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<LoadBalancerDescription>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlLoadBalancerDescriptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<LoadBalancerDescription>,
}

impl From<Vec<LoadBalancerDescription>> for XmlLoadBalancerDescriptionList {
    fn from(v: Vec<LoadBalancerDescription>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<LoadBalancerDescription> for XmlLoadBalancerDescriptionList {
    fn from_iter<I: IntoIterator<Item = LoadBalancerDescription>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LoadBalancerDescription")]
pub struct LoadBalancerDescription {
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<AvailabilityZones>,
    #[serde(rename = "BackendServerDescriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backend_server_descriptions: Option<BackendServerDescriptions>,
    #[serde(rename = "CanonicalHostedZoneName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_hosted_zone_name: Option<String>,
    #[serde(rename = "CanonicalHostedZoneNameID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical_hosted_zone_name_i_d: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "DNSName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_n_s_name: Option<String>,
    #[serde(rename = "HealthCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<HealthCheck>,
    #[serde(rename = "Instances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Instances>,
    #[serde(rename = "ListenerDescriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_descriptions: Option<ListenerDescriptions>,
    #[serde(rename = "LoadBalancerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_name: Option<String>,
    #[serde(rename = "Policies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Policies>,
    #[serde(rename = "Scheme")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    #[serde(rename = "SecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<SecurityGroups>,
    #[serde(rename = "SourceSecurityGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_security_group: Option<SourceSecurityGroup>,
    #[serde(rename = "Subnets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Subnets>,
    #[serde(rename = "VPCId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_p_c_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BackendServerDescriptions {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<BackendServerDescription>,
}
impl From<Vec<BackendServerDescription>> for BackendServerDescriptions {
    fn from(v: Vec<BackendServerDescription>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<BackendServerDescription> for BackendServerDescriptions {
    fn from_iter<I: IntoIterator<Item = BackendServerDescription>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<BackendServerDescription>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlBackendServerDescriptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<BackendServerDescription>,
}

impl From<Vec<BackendServerDescription>> for XmlBackendServerDescriptionList {
    fn from(v: Vec<BackendServerDescription>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<BackendServerDescription> for XmlBackendServerDescriptionList {
    fn from_iter<I: IntoIterator<Item = BackendServerDescription>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "BackendServerDescription")]
pub struct BackendServerDescription {
    #[serde(rename = "InstancePort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_port: Option<i32>,
    #[serde(rename = "PolicyNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_names: Option<PolicyNames>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PolicyNames {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for PolicyNames {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for PolicyNames {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListenerDescriptions {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ListenerDescription>,
}
impl From<Vec<ListenerDescription>> for ListenerDescriptions {
    fn from(v: Vec<ListenerDescription>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ListenerDescription> for ListenerDescriptions {
    fn from_iter<I: IntoIterator<Item = ListenerDescription>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ListenerDescription>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlListenerDescriptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ListenerDescription>,
}

impl From<Vec<ListenerDescription>> for XmlListenerDescriptionList {
    fn from(v: Vec<ListenerDescription>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ListenerDescription> for XmlListenerDescriptionList {
    fn from_iter<I: IntoIterator<Item = ListenerDescription>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListenerDescription")]
pub struct ListenerDescription {
    #[serde(rename = "Listener")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener: Option<Listener>,
    #[serde(rename = "PolicyNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_names: Option<PolicyNames>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Policies")]
pub struct Policies {
    #[serde(rename = "AppCookieStickinessPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_cookie_stickiness_policies: Option<AppCookieStickinessPolicies>,
    #[serde(rename = "LBCookieStickinessPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l_b_cookie_stickiness_policies: Option<LBCookieStickinessPolicies>,
    #[serde(rename = "OtherPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_policies: Option<PolicyNames>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AppCookieStickinessPolicies {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<AppCookieStickinessPolicy>,
}
impl From<Vec<AppCookieStickinessPolicy>> for AppCookieStickinessPolicies {
    fn from(v: Vec<AppCookieStickinessPolicy>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<AppCookieStickinessPolicy> for AppCookieStickinessPolicies {
    fn from_iter<I: IntoIterator<Item = AppCookieStickinessPolicy>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<AppCookieStickinessPolicy>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlAppCookieStickinessPolicyList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<AppCookieStickinessPolicy>,
}

impl From<Vec<AppCookieStickinessPolicy>> for XmlAppCookieStickinessPolicyList {
    fn from(v: Vec<AppCookieStickinessPolicy>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<AppCookieStickinessPolicy> for XmlAppCookieStickinessPolicyList {
    fn from_iter<I: IntoIterator<Item = AppCookieStickinessPolicy>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AppCookieStickinessPolicy")]
pub struct AppCookieStickinessPolicy {
    #[serde(rename = "CookieName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_name: Option<String>,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LBCookieStickinessPolicies {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<LBCookieStickinessPolicy>,
}
impl From<Vec<LBCookieStickinessPolicy>> for LBCookieStickinessPolicies {
    fn from(v: Vec<LBCookieStickinessPolicy>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<LBCookieStickinessPolicy> for LBCookieStickinessPolicies {
    fn from_iter<I: IntoIterator<Item = LBCookieStickinessPolicy>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<LBCookieStickinessPolicy>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlLBCookieStickinessPolicyList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<LBCookieStickinessPolicy>,
}

impl From<Vec<LBCookieStickinessPolicy>> for XmlLBCookieStickinessPolicyList {
    fn from(v: Vec<LBCookieStickinessPolicy>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<LBCookieStickinessPolicy> for XmlLBCookieStickinessPolicyList {
    fn from_iter<I: IntoIterator<Item = LBCookieStickinessPolicy>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LBCookieStickinessPolicy")]
pub struct LBCookieStickinessPolicy {
    #[serde(rename = "CookieExpirationPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cookie_expiration_period: Option<i64>,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SourceSecurityGroup")]
pub struct SourceSecurityGroup {
    #[serde(rename = "GroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "OwnerAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_alias: Option<String>,
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
#[serde(rename = "DescribeEndPointStateInput")]
pub struct DescribeEndPointStateInput {
    #[serde(rename = "Instances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Instances>,
    #[serde(rename = "LoadBalancerName")]
    #[serde(default)]
    pub load_balancer_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeInstanceHealthResult")]
pub struct DescribeEndPointStateOutput {
    #[serde(rename = "InstanceStates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_states: Option<InstanceStates>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceStates {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<InstanceState>,
}
impl From<Vec<InstanceState>> for InstanceStates {
    fn from(v: Vec<InstanceState>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<InstanceState> for InstanceStates {
    fn from_iter<I: IntoIterator<Item = InstanceState>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<InstanceState>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlInstanceStateList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<InstanceState>,
}

impl From<Vec<InstanceState>> for XmlInstanceStateList {
    fn from(v: Vec<InstanceState>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<InstanceState> for XmlInstanceStateList {
    fn from_iter<I: IntoIterator<Item = InstanceState>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "InstanceState")]
pub struct InstanceState {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "ReasonCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeLoadBalancerAttributesInput")]
pub struct DescribeLoadBalancerAttributesInput {
    #[serde(rename = "LoadBalancerName")]
    #[serde(default)]
    pub load_balancer_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeLoadBalancerAttributesResult")]
pub struct DescribeLoadBalancerAttributesOutput {
    #[serde(rename = "LoadBalancerAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_attributes: Option<LoadBalancerAttributes>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LoadBalancerAttributes")]
pub struct LoadBalancerAttributes {
    #[serde(rename = "AccessLog")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_log: Option<AccessLog>,
    #[serde(rename = "AdditionalAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_attributes: Option<AdditionalAttributes>,
    #[serde(rename = "ConnectionDraining")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_draining: Option<ConnectionDraining>,
    #[serde(rename = "ConnectionSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_settings: Option<ConnectionSettings>,
    #[serde(rename = "CrossZoneLoadBalancing")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_zone_load_balancing: Option<CrossZoneLoadBalancing>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AccessLog")]
pub struct AccessLog {
    #[serde(rename = "EmitInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emit_interval: Option<i32>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "S3BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,
    #[serde(rename = "S3BucketPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdditionalAttributes {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<AdditionalAttribute>,
}
impl From<Vec<AdditionalAttribute>> for AdditionalAttributes {
    fn from(v: Vec<AdditionalAttribute>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<AdditionalAttribute> for AdditionalAttributes {
    fn from_iter<I: IntoIterator<Item = AdditionalAttribute>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<AdditionalAttribute>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlAdditionalAttributeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<AdditionalAttribute>,
}

impl From<Vec<AdditionalAttribute>> for XmlAdditionalAttributeList {
    fn from(v: Vec<AdditionalAttribute>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<AdditionalAttribute> for XmlAdditionalAttributeList {
    fn from_iter<I: IntoIterator<Item = AdditionalAttribute>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AdditionalAttribute")]
pub struct AdditionalAttribute {
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
#[serde(rename = "ConnectionDraining")]
pub struct ConnectionDraining {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "Timeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ConnectionSettings")]
pub struct ConnectionSettings {
    #[serde(rename = "IdleTimeout")]
    #[serde(default)]
    pub idle_timeout: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CrossZoneLoadBalancing")]
pub struct CrossZoneLoadBalancing {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeLoadBalancerPoliciesInput")]
pub struct DescribeLoadBalancerPoliciesInput {
    #[serde(rename = "LoadBalancerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_name: Option<String>,
    #[serde(rename = "PolicyNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_names: Option<PolicyNames>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeLoadBalancerPoliciesResult")]
pub struct DescribeLoadBalancerPoliciesOutput {
    #[serde(rename = "PolicyDescriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_descriptions: Option<PolicyDescriptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PolicyDescriptions {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PolicyDescription>,
}
impl From<Vec<PolicyDescription>> for PolicyDescriptions {
    fn from(v: Vec<PolicyDescription>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<PolicyDescription> for PolicyDescriptions {
    fn from_iter<I: IntoIterator<Item = PolicyDescription>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<PolicyDescription>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlPolicyDescriptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<PolicyDescription>,
}

impl From<Vec<PolicyDescription>> for XmlPolicyDescriptionList {
    fn from(v: Vec<PolicyDescription>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<PolicyDescription> for XmlPolicyDescriptionList {
    fn from_iter<I: IntoIterator<Item = PolicyDescription>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PolicyDescription")]
pub struct PolicyDescription {
    #[serde(rename = "PolicyAttributeDescriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_attribute_descriptions: Option<PolicyAttributeDescriptions>,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
    #[serde(rename = "PolicyTypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PolicyAttributeDescriptions {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PolicyAttributeDescription>,
}
impl From<Vec<PolicyAttributeDescription>> for PolicyAttributeDescriptions {
    fn from(v: Vec<PolicyAttributeDescription>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<PolicyAttributeDescription> for PolicyAttributeDescriptions {
    fn from_iter<I: IntoIterator<Item = PolicyAttributeDescription>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<PolicyAttributeDescription>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlPolicyAttributeDescriptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<PolicyAttributeDescription>,
}

impl From<Vec<PolicyAttributeDescription>> for XmlPolicyAttributeDescriptionList {
    fn from(v: Vec<PolicyAttributeDescription>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<PolicyAttributeDescription> for XmlPolicyAttributeDescriptionList {
    fn from_iter<I: IntoIterator<Item = PolicyAttributeDescription>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PolicyAttributeDescription")]
pub struct PolicyAttributeDescription {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[serde(rename = "AttributeValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeLoadBalancerPolicyTypesInput")]
pub struct DescribeLoadBalancerPolicyTypesInput {
    #[serde(rename = "PolicyTypeNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type_names: Option<PolicyTypeNames>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PolicyTypeNames {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for PolicyTypeNames {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for PolicyTypeNames {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeLoadBalancerPolicyTypesResult")]
pub struct DescribeLoadBalancerPolicyTypesOutput {
    #[serde(rename = "PolicyTypeDescriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type_descriptions: Option<PolicyTypeDescriptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PolicyTypeDescriptions {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PolicyTypeDescription>,
}
impl From<Vec<PolicyTypeDescription>> for PolicyTypeDescriptions {
    fn from(v: Vec<PolicyTypeDescription>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<PolicyTypeDescription> for PolicyTypeDescriptions {
    fn from_iter<I: IntoIterator<Item = PolicyTypeDescription>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<PolicyTypeDescription>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlPolicyTypeDescriptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<PolicyTypeDescription>,
}

impl From<Vec<PolicyTypeDescription>> for XmlPolicyTypeDescriptionList {
    fn from(v: Vec<PolicyTypeDescription>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<PolicyTypeDescription> for XmlPolicyTypeDescriptionList {
    fn from_iter<I: IntoIterator<Item = PolicyTypeDescription>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PolicyTypeDescription")]
pub struct PolicyTypeDescription {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "PolicyAttributeTypeDescriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_attribute_type_descriptions: Option<PolicyAttributeTypeDescriptions>,
    #[serde(rename = "PolicyTypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PolicyAttributeTypeDescriptions {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PolicyAttributeTypeDescription>,
}
impl From<Vec<PolicyAttributeTypeDescription>> for PolicyAttributeTypeDescriptions {
    fn from(v: Vec<PolicyAttributeTypeDescription>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<PolicyAttributeTypeDescription> for PolicyAttributeTypeDescriptions {
    fn from_iter<I: IntoIterator<Item = PolicyAttributeTypeDescription>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<PolicyAttributeTypeDescription>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlPolicyAttributeTypeDescriptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<PolicyAttributeTypeDescription>,
}

impl From<Vec<PolicyAttributeTypeDescription>> for XmlPolicyAttributeTypeDescriptionList {
    fn from(v: Vec<PolicyAttributeTypeDescription>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<PolicyAttributeTypeDescription> for XmlPolicyAttributeTypeDescriptionList {
    fn from_iter<I: IntoIterator<Item = PolicyAttributeTypeDescription>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PolicyAttributeTypeDescription")]
pub struct PolicyAttributeTypeDescription {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[serde(rename = "AttributeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<String>,
    #[serde(rename = "Cardinality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardinality: Option<String>,
    #[serde(rename = "DefaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeTagsInput")]
pub struct DescribeTagsInput {
    #[serde(rename = "LoadBalancerNames")]
    #[serde(default)]
    pub load_balancer_names: LoadBalancerNamesMax20,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoadBalancerNamesMax20 {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for LoadBalancerNamesMax20 {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for LoadBalancerNamesMax20 {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
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
    #[serde(rename = "LoadBalancerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_name: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DetachLoadBalancerFromSubnetsInput")]
pub struct DetachLoadBalancerFromSubnetsInput {
    #[serde(rename = "LoadBalancerName")]
    #[serde(default)]
    pub load_balancer_name: String,
    #[serde(rename = "Subnets")]
    #[serde(default)]
    pub subnets: Subnets,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DetachLoadBalancerFromSubnetsResult")]
pub struct DetachLoadBalancerFromSubnetsOutput {
    #[serde(rename = "Subnets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Subnets>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyLoadBalancerAttributesInput")]
pub struct ModifyLoadBalancerAttributesInput {
    #[serde(rename = "LoadBalancerAttributes")]
    #[serde(default)]
    pub load_balancer_attributes: LoadBalancerAttributes,
    #[serde(rename = "LoadBalancerName")]
    #[serde(default)]
    pub load_balancer_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyLoadBalancerAttributesResult")]
pub struct ModifyLoadBalancerAttributesOutput {
    #[serde(rename = "LoadBalancerAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_attributes: Option<LoadBalancerAttributes>,
    #[serde(rename = "LoadBalancerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RegisterEndPointsInput")]
pub struct RegisterEndPointsInput {
    #[serde(rename = "Instances")]
    #[serde(default)]
    pub instances: Instances,
    #[serde(rename = "LoadBalancerName")]
    #[serde(default)]
    pub load_balancer_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RegisterInstancesWithLoadBalancerResult")]
pub struct RegisterEndPointsOutput {
    #[serde(rename = "Instances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Instances>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RemoveAvailabilityZonesInput")]
pub struct RemoveAvailabilityZonesInput {
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    pub availability_zones: AvailabilityZones,
    #[serde(rename = "LoadBalancerName")]
    #[serde(default)]
    pub load_balancer_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DisableAvailabilityZonesForLoadBalancerResult")]
pub struct RemoveAvailabilityZonesOutput {
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<AvailabilityZones>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RemoveTagsInput")]
pub struct RemoveTagsInput {
    #[serde(rename = "LoadBalancerNames")]
    #[serde(default)]
    pub load_balancer_names: LoadBalancerNames,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: TagKeyList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagKeyList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<TagKeyOnly>,
}
impl From<Vec<TagKeyOnly>> for TagKeyList {
    fn from(v: Vec<TagKeyOnly>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<TagKeyOnly> for TagKeyList {
    fn from_iter<I: IntoIterator<Item = TagKeyOnly>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<TagKeyOnly>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTagKeyOnlyList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<TagKeyOnly>,
}

impl From<Vec<TagKeyOnly>> for XmlTagKeyOnlyList {
    fn from(v: Vec<TagKeyOnly>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<TagKeyOnly> for XmlTagKeyOnlyList {
    fn from_iter<I: IntoIterator<Item = TagKeyOnly>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TagKeyOnly")]
pub struct TagKeyOnly {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveTagsOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SetLoadBalancerListenerSSLCertificateInput")]
pub struct SetLoadBalancerListenerSSLCertificateInput {
    #[serde(rename = "LoadBalancerName")]
    #[serde(default)]
    pub load_balancer_name: String,
    #[serde(rename = "LoadBalancerPort")]
    #[serde(default)]
    pub load_balancer_port: i32,
    #[serde(rename = "SSLCertificateId")]
    #[serde(default)]
    pub s_s_l_certificate_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetLoadBalancerListenerSSLCertificateOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SetLoadBalancerPoliciesForBackendServerInput")]
pub struct SetLoadBalancerPoliciesForBackendServerInput {
    #[serde(rename = "InstancePort")]
    #[serde(default)]
    pub instance_port: i32,
    #[serde(rename = "LoadBalancerName")]
    #[serde(default)]
    pub load_balancer_name: String,
    #[serde(rename = "PolicyNames")]
    #[serde(default)]
    pub policy_names: PolicyNames,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetLoadBalancerPoliciesForBackendServerOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SetLoadBalancerPoliciesOfListenerInput")]
pub struct SetLoadBalancerPoliciesOfListenerInput {
    #[serde(rename = "LoadBalancerName")]
    #[serde(default)]
    pub load_balancer_name: String,
    #[serde(rename = "LoadBalancerPort")]
    #[serde(default)]
    pub load_balancer_port: i32,
    #[serde(rename = "PolicyNames")]
    #[serde(default)]
    pub policy_names: PolicyNames,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetLoadBalancerPoliciesOfListenerOutput {}
