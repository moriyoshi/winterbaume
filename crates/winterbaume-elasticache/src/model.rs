//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-elasticache

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TestFailoverMessage")]
pub struct TestFailoverMessage {
    #[serde(rename = "NodeGroupId")]
    #[serde(default)]
    pub node_group_id: String,
    #[serde(rename = "ReplicationGroupId")]
    #[serde(default)]
    pub replication_group_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeUsersResult")]
pub struct DescribeUsersResult {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Users")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<UserList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<User>,
}
impl From<Vec<User>> for UserList {
    fn from(v: Vec<User>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<User> for UserList {
    fn from_iter<I: IntoIterator<Item = User>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<User>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlUserList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<User>,
}

impl From<Vec<User>> for XmlUserList {
    fn from(v: Vec<User>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<User> for XmlUserList {
    fn from_iter<I: IntoIterator<Item = User>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyUserResult")]
pub struct User {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "AccessString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_string: Option<String>,
    #[serde(rename = "Authentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication: Option<Authentication>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "MinimumEngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_engine_version: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "UserGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_group_ids: Option<UserGroupIdList>,
    #[serde(rename = "UserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Authentication")]
pub struct Authentication {
    #[serde(rename = "PasswordCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_count: Option<i32>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserGroupIdList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for UserGroupIdList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for UserGroupIdList {
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
#[serde(rename = "DescribeReservedCacheNodesOfferingsMessage")]
pub struct DescribeReservedCacheNodesOfferingsMessage {
    #[serde(rename = "CacheNodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_type: Option<String>,
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "OfferingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_type: Option<String>,
    #[serde(rename = "ProductDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    #[serde(rename = "ReservedCacheNodesOfferingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_cache_nodes_offering_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "FailoverGlobalReplicationGroupResult")]
pub struct FailoverGlobalReplicationGroupResult {
    #[serde(rename = "GlobalReplicationGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_replication_group: Option<GlobalReplicationGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GlobalReplicationGroup")]
pub struct GlobalReplicationGroup {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "AtRestEncryptionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_rest_encryption_enabled: Option<bool>,
    #[serde(rename = "AuthTokenEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token_enabled: Option<bool>,
    #[serde(rename = "CacheNodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_type: Option<String>,
    #[serde(rename = "ClusterEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_enabled: Option<bool>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "GlobalNodeGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_node_groups: Option<GlobalNodeGroupList>,
    #[serde(rename = "GlobalReplicationGroupDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_replication_group_description: Option<String>,
    #[serde(rename = "GlobalReplicationGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_replication_group_id: Option<String>,
    #[serde(rename = "Members")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<GlobalReplicationGroupMemberList>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TransitEncryptionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_encryption_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlobalNodeGroupList {
    #[serde(
        rename = "GlobalNodeGroup",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<GlobalNodeGroup>,
}
impl From<Vec<GlobalNodeGroup>> for GlobalNodeGroupList {
    fn from(v: Vec<GlobalNodeGroup>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<GlobalNodeGroup> for GlobalNodeGroupList {
    fn from_iter<I: IntoIterator<Item = GlobalNodeGroup>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<GlobalNodeGroup>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlGlobalNodeGroupList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<GlobalNodeGroup>,
}

impl From<Vec<GlobalNodeGroup>> for XmlGlobalNodeGroupList {
    fn from(v: Vec<GlobalNodeGroup>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<GlobalNodeGroup> for XmlGlobalNodeGroupList {
    fn from_iter<I: IntoIterator<Item = GlobalNodeGroup>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GlobalNodeGroup")]
pub struct GlobalNodeGroup {
    #[serde(rename = "GlobalNodeGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_node_group_id: Option<String>,
    #[serde(rename = "Slots")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slots: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlobalReplicationGroupMemberList {
    #[serde(
        rename = "GlobalReplicationGroupMember",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<GlobalReplicationGroupMember>,
}
impl From<Vec<GlobalReplicationGroupMember>> for GlobalReplicationGroupMemberList {
    fn from(v: Vec<GlobalReplicationGroupMember>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<GlobalReplicationGroupMember> for GlobalReplicationGroupMemberList {
    fn from_iter<I: IntoIterator<Item = GlobalReplicationGroupMember>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<GlobalReplicationGroupMember>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlGlobalReplicationGroupMemberList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<GlobalReplicationGroupMember>,
}

impl From<Vec<GlobalReplicationGroupMember>> for XmlGlobalReplicationGroupMemberList {
    fn from(v: Vec<GlobalReplicationGroupMember>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<GlobalReplicationGroupMember> for XmlGlobalReplicationGroupMemberList {
    fn from_iter<I: IntoIterator<Item = GlobalReplicationGroupMember>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GlobalReplicationGroupMember")]
pub struct GlobalReplicationGroupMember {
    #[serde(rename = "AutomaticFailover")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_failover: Option<String>,
    #[serde(rename = "ReplicationGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group_id: Option<String>,
    #[serde(rename = "ReplicationGroupRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group_region: Option<String>,
    #[serde(rename = "Role")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateCacheSubnetGroupMessage")]
pub struct CreateCacheSubnetGroupMessage {
    #[serde(rename = "CacheSubnetGroupDescription")]
    #[serde(default)]
    pub cache_subnet_group_description: String,
    #[serde(rename = "CacheSubnetGroupName")]
    #[serde(default)]
    pub cache_subnet_group_name: String,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    pub subnet_ids: SubnetIdentifierList,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubnetIdentifierList {
    #[serde(
        rename = "SubnetIdentifier",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for SubnetIdentifierList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for SubnetIdentifierList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagList {
    #[serde(rename = "Tag", default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyCacheParameterGroupMessage")]
pub struct ModifyCacheParameterGroupMessage {
    #[serde(rename = "CacheParameterGroupName")]
    #[serde(default)]
    pub cache_parameter_group_name: String,
    #[serde(rename = "ParameterNameValues")]
    #[serde(default)]
    pub parameter_name_values: ParameterNameValueList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParameterNameValueList {
    #[serde(
        rename = "ParameterNameValue",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ParameterNameValue>,
}
impl From<Vec<ParameterNameValue>> for ParameterNameValueList {
    fn from(v: Vec<ParameterNameValue>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ParameterNameValue> for ParameterNameValueList {
    fn from_iter<I: IntoIterator<Item = ParameterNameValue>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ParameterNameValue>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlParameterNameValueList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ParameterNameValue>,
}

impl From<Vec<ParameterNameValue>> for XmlParameterNameValueList {
    fn from(v: Vec<ParameterNameValue>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ParameterNameValue> for XmlParameterNameValueList {
    fn from_iter<I: IntoIterator<Item = ParameterNameValue>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ParameterNameValue")]
pub struct ParameterNameValue {
    #[serde(rename = "ParameterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_name: Option<String>,
    #[serde(rename = "ParameterValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyCacheSubnetGroupResult")]
pub struct ModifyCacheSubnetGroupResult {
    #[serde(rename = "CacheSubnetGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_subnet_group: Option<CacheSubnetGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CacheSubnetGroup")]
pub struct CacheSubnetGroup {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "CacheSubnetGroupDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_subnet_group_description: Option<String>,
    #[serde(rename = "CacheSubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_subnet_group_name: Option<String>,
    #[serde(rename = "Subnets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<SubnetList>,
    #[serde(rename = "SupportedNetworkTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_network_types: Option<NetworkTypeList>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubnetList {
    #[serde(rename = "Subnet", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Subnet>,
}
impl From<Vec<Subnet>> for SubnetList {
    fn from(v: Vec<Subnet>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Subnet> for SubnetList {
    fn from_iter<I: IntoIterator<Item = Subnet>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Subnet>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlSubnetList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Subnet>,
}

impl From<Vec<Subnet>> for XmlSubnetList {
    fn from(v: Vec<Subnet>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Subnet> for XmlSubnetList {
    fn from_iter<I: IntoIterator<Item = Subnet>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Subnet")]
pub struct Subnet {
    #[serde(rename = "SubnetAvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_availability_zone: Option<AvailabilityZone>,
    #[serde(rename = "SubnetIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_identifier: Option<String>,
    #[serde(rename = "SubnetOutpost")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_outpost: Option<SubnetOutpost>,
    #[serde(rename = "SupportedNetworkTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_network_types: Option<NetworkTypeList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AvailabilityZone")]
pub struct AvailabilityZone {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SubnetOutpost")]
pub struct SubnetOutpost {
    #[serde(rename = "SubnetOutpostArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_outpost_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkTypeList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for NetworkTypeList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for NetworkTypeList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEngineDefaultParametersResult")]
pub struct DescribeEngineDefaultParametersResult {
    #[serde(rename = "EngineDefaults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_defaults: Option<EngineDefaults>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EngineDefaults")]
pub struct EngineDefaults {
    #[serde(rename = "CacheNodeTypeSpecificParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_type_specific_parameters: Option<CacheNodeTypeSpecificParametersList>,
    #[serde(rename = "CacheParameterGroupFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_parameter_group_family: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParametersList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CacheNodeTypeSpecificParametersList {
    #[serde(
        rename = "CacheNodeTypeSpecificParameter",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<CacheNodeTypeSpecificParameter>,
}
impl From<Vec<CacheNodeTypeSpecificParameter>> for CacheNodeTypeSpecificParametersList {
    fn from(v: Vec<CacheNodeTypeSpecificParameter>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<CacheNodeTypeSpecificParameter> for CacheNodeTypeSpecificParametersList {
    fn from_iter<I: IntoIterator<Item = CacheNodeTypeSpecificParameter>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<CacheNodeTypeSpecificParameter>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlCacheNodeTypeSpecificParameterList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<CacheNodeTypeSpecificParameter>,
}

impl From<Vec<CacheNodeTypeSpecificParameter>> for XmlCacheNodeTypeSpecificParameterList {
    fn from(v: Vec<CacheNodeTypeSpecificParameter>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<CacheNodeTypeSpecificParameter> for XmlCacheNodeTypeSpecificParameterList {
    fn from_iter<I: IntoIterator<Item = CacheNodeTypeSpecificParameter>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CacheNodeTypeSpecificParameter")]
pub struct CacheNodeTypeSpecificParameter {
    #[serde(rename = "AllowedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<String>,
    #[serde(rename = "CacheNodeTypeSpecificValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_type_specific_values: Option<CacheNodeTypeSpecificValueList>,
    #[serde(rename = "ChangeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_type: Option<String>,
    #[serde(rename = "DataType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "IsModifiable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_modifiable: Option<bool>,
    #[serde(rename = "MinimumEngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_engine_version: Option<String>,
    #[serde(rename = "ParameterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_name: Option<String>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CacheNodeTypeSpecificValueList {
    #[serde(
        rename = "CacheNodeTypeSpecificValue",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<CacheNodeTypeSpecificValue>,
}
impl From<Vec<CacheNodeTypeSpecificValue>> for CacheNodeTypeSpecificValueList {
    fn from(v: Vec<CacheNodeTypeSpecificValue>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<CacheNodeTypeSpecificValue> for CacheNodeTypeSpecificValueList {
    fn from_iter<I: IntoIterator<Item = CacheNodeTypeSpecificValue>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<CacheNodeTypeSpecificValue>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlCacheNodeTypeSpecificValueList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<CacheNodeTypeSpecificValue>,
}

impl From<Vec<CacheNodeTypeSpecificValue>> for XmlCacheNodeTypeSpecificValueList {
    fn from(v: Vec<CacheNodeTypeSpecificValue>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<CacheNodeTypeSpecificValue> for XmlCacheNodeTypeSpecificValueList {
    fn from_iter<I: IntoIterator<Item = CacheNodeTypeSpecificValue>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CacheNodeTypeSpecificValue")]
pub struct CacheNodeTypeSpecificValue {
    #[serde(rename = "CacheNodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_type: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParametersList {
    #[serde(rename = "Parameter", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Parameter>,
}
impl From<Vec<Parameter>> for ParametersList {
    fn from(v: Vec<Parameter>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Parameter> for ParametersList {
    fn from_iter<I: IntoIterator<Item = Parameter>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Parameter>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlParameterList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Parameter>,
}

impl From<Vec<Parameter>> for XmlParameterList {
    fn from(v: Vec<Parameter>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Parameter> for XmlParameterList {
    fn from_iter<I: IntoIterator<Item = Parameter>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Parameter")]
pub struct Parameter {
    #[serde(rename = "AllowedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<String>,
    #[serde(rename = "ChangeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_type: Option<String>,
    #[serde(rename = "DataType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "IsModifiable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_modifiable: Option<bool>,
    #[serde(rename = "MinimumEngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_engine_version: Option<String>,
    #[serde(rename = "ParameterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_name: Option<String>,
    #[serde(rename = "ParameterValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_value: Option<String>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyCacheSubnetGroupMessage")]
pub struct ModifyCacheSubnetGroupMessage {
    #[serde(rename = "CacheSubnetGroupDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_subnet_group_description: Option<String>,
    #[serde(rename = "CacheSubnetGroupName")]
    #[serde(default)]
    pub cache_subnet_group_name: String,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<SubnetIdentifierList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeCacheParametersMessage")]
pub struct DescribeCacheParametersMessage {
    #[serde(rename = "CacheParameterGroupName")]
    #[serde(default)]
    pub cache_parameter_group_name: String,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateCacheClusterResult")]
pub struct CreateCacheClusterResult {
    #[serde(rename = "CacheCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster: Option<CacheCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CacheCluster")]
pub struct CacheCluster {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "AtRestEncryptionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_rest_encryption_enabled: Option<bool>,
    #[serde(rename = "AuthTokenEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token_enabled: Option<bool>,
    #[serde(rename = "AuthTokenLastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token_last_modified_date: Option<String>,
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    #[serde(rename = "CacheClusterCreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_create_time: Option<String>,
    #[serde(rename = "CacheClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_id: Option<String>,
    #[serde(rename = "CacheClusterStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_status: Option<String>,
    #[serde(rename = "CacheNodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_type: Option<String>,
    #[serde(rename = "CacheNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_nodes: Option<CacheNodeList>,
    #[serde(rename = "CacheParameterGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_parameter_group: Option<CacheParameterGroupStatus>,
    #[serde(rename = "CacheSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_security_groups: Option<CacheSecurityGroupMembershipList>,
    #[serde(rename = "CacheSubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_subnet_group_name: Option<String>,
    #[serde(rename = "ClientDownloadLandingPage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_download_landing_page: Option<String>,
    #[serde(rename = "ConfigurationEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_endpoint: Option<Endpoint>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "IpDiscovery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_discovery: Option<String>,
    #[serde(rename = "LogDeliveryConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_delivery_configurations: Option<LogDeliveryConfigurationList>,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "NotificationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_configuration: Option<NotificationConfiguration>,
    #[serde(rename = "NumCacheNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_cache_nodes: Option<i32>,
    #[serde(rename = "PendingModifiedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_modified_values: Option<PendingModifiedValues>,
    #[serde(rename = "PreferredAvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_availability_zone: Option<String>,
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[serde(rename = "PreferredOutpostArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_outpost_arn: Option<String>,
    #[serde(rename = "ReplicationGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group_id: Option<String>,
    #[serde(rename = "ReplicationGroupLogDeliveryEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group_log_delivery_enabled: Option<bool>,
    #[serde(rename = "SecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<SecurityGroupMembershipList>,
    #[serde(rename = "SnapshotRetentionLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_retention_limit: Option<i32>,
    #[serde(rename = "SnapshotWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_window: Option<String>,
    #[serde(rename = "TransitEncryptionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_encryption_enabled: Option<bool>,
    #[serde(rename = "TransitEncryptionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_encryption_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CacheNodeList {
    #[serde(rename = "CacheNode", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<CacheNode>,
}
impl From<Vec<CacheNode>> for CacheNodeList {
    fn from(v: Vec<CacheNode>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<CacheNode> for CacheNodeList {
    fn from_iter<I: IntoIterator<Item = CacheNode>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<CacheNode>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlCacheNodeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<CacheNode>,
}

impl From<Vec<CacheNode>> for XmlCacheNodeList {
    fn from(v: Vec<CacheNode>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<CacheNode> for XmlCacheNodeList {
    fn from_iter<I: IntoIterator<Item = CacheNode>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CacheNode")]
pub struct CacheNode {
    #[serde(rename = "CacheNodeCreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_create_time: Option<String>,
    #[serde(rename = "CacheNodeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_id: Option<String>,
    #[serde(rename = "CacheNodeStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_status: Option<String>,
    #[serde(rename = "CustomerAvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_availability_zone: Option<String>,
    #[serde(rename = "CustomerOutpostArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_outpost_arn: Option<String>,
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<Endpoint>,
    #[serde(rename = "ParameterGroupStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_status: Option<String>,
    #[serde(rename = "SourceCacheNodeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_cache_node_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Endpoint")]
pub struct Endpoint {
    #[serde(rename = "Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CacheParameterGroupStatus")]
pub struct CacheParameterGroupStatus {
    #[serde(rename = "CacheNodeIdsToReboot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_ids_to_reboot: Option<CacheNodeIdsList>,
    #[serde(rename = "CacheParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_parameter_group_name: Option<String>,
    #[serde(rename = "ParameterApplyStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_apply_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CacheNodeIdsList {
    #[serde(rename = "CacheNodeId", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for CacheNodeIdsList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for CacheNodeIdsList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CacheSecurityGroupMembershipList {
    #[serde(
        rename = "CacheSecurityGroup",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<CacheSecurityGroupMembership>,
}
impl From<Vec<CacheSecurityGroupMembership>> for CacheSecurityGroupMembershipList {
    fn from(v: Vec<CacheSecurityGroupMembership>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<CacheSecurityGroupMembership> for CacheSecurityGroupMembershipList {
    fn from_iter<I: IntoIterator<Item = CacheSecurityGroupMembership>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<CacheSecurityGroupMembership>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlCacheSecurityGroupMembershipList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<CacheSecurityGroupMembership>,
}

impl From<Vec<CacheSecurityGroupMembership>> for XmlCacheSecurityGroupMembershipList {
    fn from(v: Vec<CacheSecurityGroupMembership>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<CacheSecurityGroupMembership> for XmlCacheSecurityGroupMembershipList {
    fn from_iter<I: IntoIterator<Item = CacheSecurityGroupMembership>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CacheSecurityGroupMembership")]
pub struct CacheSecurityGroupMembership {
    #[serde(rename = "CacheSecurityGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_security_group_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogDeliveryConfigurationList {
    #[serde(
        rename = "LogDeliveryConfiguration",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<LogDeliveryConfiguration>,
}
impl From<Vec<LogDeliveryConfiguration>> for LogDeliveryConfigurationList {
    fn from(v: Vec<LogDeliveryConfiguration>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<LogDeliveryConfiguration> for LogDeliveryConfigurationList {
    fn from_iter<I: IntoIterator<Item = LogDeliveryConfiguration>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<LogDeliveryConfiguration>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlLogDeliveryConfigurationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<LogDeliveryConfiguration>,
}

impl From<Vec<LogDeliveryConfiguration>> for XmlLogDeliveryConfigurationList {
    fn from(v: Vec<LogDeliveryConfiguration>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<LogDeliveryConfiguration> for XmlLogDeliveryConfigurationList {
    fn from_iter<I: IntoIterator<Item = LogDeliveryConfiguration>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LogDeliveryConfiguration")]
pub struct LogDeliveryConfiguration {
    #[serde(rename = "DestinationDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_details: Option<DestinationDetails>,
    #[serde(rename = "DestinationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_type: Option<String>,
    #[serde(rename = "LogFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_format: Option<String>,
    #[serde(rename = "LogType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_type: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DestinationDetails")]
pub struct DestinationDetails {
    #[serde(rename = "CloudWatchLogsDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_details: Option<CloudWatchLogsDestinationDetails>,
    #[serde(rename = "KinesisFirehoseDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_firehose_details: Option<KinesisFirehoseDestinationDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CloudWatchLogsDestinationDetails")]
pub struct CloudWatchLogsDestinationDetails {
    #[serde(rename = "LogGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "KinesisFirehoseDestinationDetails")]
pub struct KinesisFirehoseDestinationDetails {
    #[serde(rename = "DeliveryStream")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_stream: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "NotificationConfiguration")]
pub struct NotificationConfiguration {
    #[serde(rename = "TopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
    #[serde(rename = "TopicStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PendingModifiedValues")]
pub struct PendingModifiedValues {
    #[serde(rename = "AuthTokenStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token_status: Option<String>,
    #[serde(rename = "CacheNodeIdsToRemove")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_ids_to_remove: Option<CacheNodeIdsList>,
    #[serde(rename = "CacheNodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_type: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "LogDeliveryConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_delivery_configurations: Option<PendingLogDeliveryConfigurationList>,
    #[serde(rename = "NumCacheNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_cache_nodes: Option<i32>,
    #[serde(rename = "ScaleConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_config: Option<ScaleConfig>,
    #[serde(rename = "TransitEncryptionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_encryption_enabled: Option<bool>,
    #[serde(rename = "TransitEncryptionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_encryption_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PendingLogDeliveryConfigurationList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<PendingLogDeliveryConfiguration>,
}
impl From<Vec<PendingLogDeliveryConfiguration>> for PendingLogDeliveryConfigurationList {
    fn from(v: Vec<PendingLogDeliveryConfiguration>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<PendingLogDeliveryConfiguration> for PendingLogDeliveryConfigurationList {
    fn from_iter<I: IntoIterator<Item = PendingLogDeliveryConfiguration>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<PendingLogDeliveryConfiguration>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlPendingLogDeliveryConfigurationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<PendingLogDeliveryConfiguration>,
}

impl From<Vec<PendingLogDeliveryConfiguration>> for XmlPendingLogDeliveryConfigurationList {
    fn from(v: Vec<PendingLogDeliveryConfiguration>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<PendingLogDeliveryConfiguration> for XmlPendingLogDeliveryConfigurationList {
    fn from_iter<I: IntoIterator<Item = PendingLogDeliveryConfiguration>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PendingLogDeliveryConfiguration")]
pub struct PendingLogDeliveryConfiguration {
    #[serde(rename = "DestinationDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_details: Option<DestinationDetails>,
    #[serde(rename = "DestinationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_type: Option<String>,
    #[serde(rename = "LogFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_format: Option<String>,
    #[serde(rename = "LogType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ScaleConfig")]
pub struct ScaleConfig {
    #[serde(rename = "ScaleIntervalMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_interval_minutes: Option<i32>,
    #[serde(rename = "ScalePercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_percentage: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecurityGroupMembershipList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<SecurityGroupMembership>,
}
impl From<Vec<SecurityGroupMembership>> for SecurityGroupMembershipList {
    fn from(v: Vec<SecurityGroupMembership>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<SecurityGroupMembership> for SecurityGroupMembershipList {
    fn from_iter<I: IntoIterator<Item = SecurityGroupMembership>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<SecurityGroupMembership>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlSecurityGroupMembershipList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<SecurityGroupMembership>,
}

impl From<Vec<SecurityGroupMembership>> for XmlSecurityGroupMembershipList {
    fn from(v: Vec<SecurityGroupMembership>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<SecurityGroupMembership> for XmlSecurityGroupMembershipList {
    fn from_iter<I: IntoIterator<Item = SecurityGroupMembership>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SecurityGroupMembership")]
pub struct SecurityGroupMembership {
    #[serde(rename = "SecurityGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "BatchStopUpdateActionResult")]
pub struct UpdateActionResultsMessage {
    #[serde(rename = "ProcessedUpdateActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_update_actions: Option<ProcessedUpdateActionList>,
    #[serde(rename = "UnprocessedUpdateActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_update_actions: Option<UnprocessedUpdateActionList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProcessedUpdateActionList {
    #[serde(
        rename = "ProcessedUpdateAction",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ProcessedUpdateAction>,
}
impl From<Vec<ProcessedUpdateAction>> for ProcessedUpdateActionList {
    fn from(v: Vec<ProcessedUpdateAction>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ProcessedUpdateAction> for ProcessedUpdateActionList {
    fn from_iter<I: IntoIterator<Item = ProcessedUpdateAction>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ProcessedUpdateAction>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlProcessedUpdateActionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ProcessedUpdateAction>,
}

impl From<Vec<ProcessedUpdateAction>> for XmlProcessedUpdateActionList {
    fn from(v: Vec<ProcessedUpdateAction>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ProcessedUpdateAction> for XmlProcessedUpdateActionList {
    fn from_iter<I: IntoIterator<Item = ProcessedUpdateAction>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ProcessedUpdateAction")]
pub struct ProcessedUpdateAction {
    #[serde(rename = "CacheClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_id: Option<String>,
    #[serde(rename = "ReplicationGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group_id: Option<String>,
    #[serde(rename = "ServiceUpdateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_update_name: Option<String>,
    #[serde(rename = "UpdateActionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_action_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnprocessedUpdateActionList {
    #[serde(
        rename = "UnprocessedUpdateAction",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<UnprocessedUpdateAction>,
}
impl From<Vec<UnprocessedUpdateAction>> for UnprocessedUpdateActionList {
    fn from(v: Vec<UnprocessedUpdateAction>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<UnprocessedUpdateAction> for UnprocessedUpdateActionList {
    fn from_iter<I: IntoIterator<Item = UnprocessedUpdateAction>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<UnprocessedUpdateAction>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlUnprocessedUpdateActionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<UnprocessedUpdateAction>,
}

impl From<Vec<UnprocessedUpdateAction>> for XmlUnprocessedUpdateActionList {
    fn from(v: Vec<UnprocessedUpdateAction>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<UnprocessedUpdateAction> for XmlUnprocessedUpdateActionList {
    fn from_iter<I: IntoIterator<Item = UnprocessedUpdateAction>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UnprocessedUpdateAction")]
pub struct UnprocessedUpdateAction {
    #[serde(rename = "CacheClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_id: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "ErrorType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,
    #[serde(rename = "ReplicationGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group_id: Option<String>,
    #[serde(rename = "ServiceUpdateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_update_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeServiceUpdatesResult")]
pub struct ServiceUpdatesMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "ServiceUpdates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_updates: Option<ServiceUpdateList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceUpdateList {
    #[serde(
        rename = "ServiceUpdate",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ServiceUpdate>,
}
impl From<Vec<ServiceUpdate>> for ServiceUpdateList {
    fn from(v: Vec<ServiceUpdate>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ServiceUpdate> for ServiceUpdateList {
    fn from_iter<I: IntoIterator<Item = ServiceUpdate>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ServiceUpdate>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlServiceUpdateList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ServiceUpdate>,
}

impl From<Vec<ServiceUpdate>> for XmlServiceUpdateList {
    fn from(v: Vec<ServiceUpdate>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ServiceUpdate> for XmlServiceUpdateList {
    fn from_iter<I: IntoIterator<Item = ServiceUpdate>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ServiceUpdate")]
pub struct ServiceUpdate {
    #[serde(rename = "AutoUpdateAfterRecommendedApplyByDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_update_after_recommended_apply_by_date: Option<bool>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "EstimatedUpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_update_time: Option<String>,
    #[serde(rename = "ServiceUpdateDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_update_description: Option<String>,
    #[serde(rename = "ServiceUpdateEndDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_update_end_date: Option<String>,
    #[serde(rename = "ServiceUpdateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_update_name: Option<String>,
    #[serde(rename = "ServiceUpdateRecommendedApplyByDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_update_recommended_apply_by_date: Option<String>,
    #[serde(rename = "ServiceUpdateReleaseDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_update_release_date: Option<String>,
    #[serde(rename = "ServiceUpdateSeverity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_update_severity: Option<String>,
    #[serde(rename = "ServiceUpdateStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_update_status: Option<String>,
    #[serde(rename = "ServiceUpdateType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_update_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyGlobalReplicationGroupResult")]
pub struct ModifyGlobalReplicationGroupResult {
    #[serde(rename = "GlobalReplicationGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_replication_group: Option<GlobalReplicationGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TestMigrationMessage")]
pub struct TestMigrationMessage {
    #[serde(rename = "CustomerNodeEndpointList")]
    #[serde(default)]
    pub customer_node_endpoint_list: CustomerNodeEndpointList,
    #[serde(rename = "ReplicationGroupId")]
    #[serde(default)]
    pub replication_group_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomerNodeEndpointList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<CustomerNodeEndpoint>,
}
impl From<Vec<CustomerNodeEndpoint>> for CustomerNodeEndpointList {
    fn from(v: Vec<CustomerNodeEndpoint>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<CustomerNodeEndpoint> for CustomerNodeEndpointList {
    fn from_iter<I: IntoIterator<Item = CustomerNodeEndpoint>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<CustomerNodeEndpoint>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlCustomerNodeEndpointList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<CustomerNodeEndpoint>,
}

impl From<Vec<CustomerNodeEndpoint>> for XmlCustomerNodeEndpointList {
    fn from(v: Vec<CustomerNodeEndpoint>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<CustomerNodeEndpoint> for XmlCustomerNodeEndpointList {
    fn from_iter<I: IntoIterator<Item = CustomerNodeEndpoint>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CustomerNodeEndpoint")]
pub struct CustomerNodeEndpoint {
    #[serde(rename = "Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DisassociateGlobalReplicationGroupMessage")]
pub struct DisassociateGlobalReplicationGroupMessage {
    #[serde(rename = "GlobalReplicationGroupId")]
    #[serde(default)]
    pub global_replication_group_id: String,
    #[serde(rename = "ReplicationGroupId")]
    #[serde(default)]
    pub replication_group_id: String,
    #[serde(rename = "ReplicationGroupRegion")]
    #[serde(default)]
    pub replication_group_region: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteCacheParameterGroupMessage")]
pub struct DeleteCacheParameterGroupMessage {
    #[serde(rename = "CacheParameterGroupName")]
    #[serde(default)]
    pub cache_parameter_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeReplicationGroupsResult")]
pub struct ReplicationGroupMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "ReplicationGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_groups: Option<ReplicationGroupList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationGroupList {
    #[serde(
        rename = "ReplicationGroup",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ReplicationGroup>,
}
impl From<Vec<ReplicationGroup>> for ReplicationGroupList {
    fn from(v: Vec<ReplicationGroup>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ReplicationGroup> for ReplicationGroupList {
    fn from_iter<I: IntoIterator<Item = ReplicationGroup>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ReplicationGroup>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlReplicationGroupList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ReplicationGroup>,
}

impl From<Vec<ReplicationGroup>> for XmlReplicationGroupList {
    fn from(v: Vec<ReplicationGroup>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ReplicationGroup> for XmlReplicationGroupList {
    fn from_iter<I: IntoIterator<Item = ReplicationGroup>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ReplicationGroup")]
pub struct ReplicationGroup {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "AtRestEncryptionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_rest_encryption_enabled: Option<bool>,
    #[serde(rename = "AuthTokenEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token_enabled: Option<bool>,
    #[serde(rename = "AuthTokenLastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token_last_modified_date: Option<String>,
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    #[serde(rename = "AutomaticFailover")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_failover: Option<String>,
    #[serde(rename = "CacheNodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_type: Option<String>,
    #[serde(rename = "ClusterEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_enabled: Option<bool>,
    #[serde(rename = "ClusterMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_mode: Option<String>,
    #[serde(rename = "ConfigurationEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_endpoint: Option<Endpoint>,
    #[serde(rename = "DataTiering")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_tiering: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "GlobalReplicationGroupInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_replication_group_info: Option<GlobalReplicationGroupInfo>,
    #[serde(rename = "IpDiscovery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_discovery: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "LogDeliveryConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_delivery_configurations: Option<LogDeliveryConfigurationList>,
    #[serde(rename = "MemberClusters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_clusters: Option<ClusterIdList>,
    #[serde(rename = "MemberClustersOutpostArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_clusters_outpost_arns: Option<ReplicationGroupOutpostArnList>,
    #[serde(rename = "MultiAZ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_a_z: Option<String>,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "NodeGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_groups: Option<NodeGroupList>,
    #[serde(rename = "PendingModifiedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_modified_values: Option<ReplicationGroupPendingModifiedValues>,
    #[serde(rename = "ReplicationGroupCreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group_create_time: Option<String>,
    #[serde(rename = "ReplicationGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group_id: Option<String>,
    #[serde(rename = "SnapshotRetentionLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_retention_limit: Option<i32>,
    #[serde(rename = "SnapshotWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_window: Option<String>,
    #[serde(rename = "SnapshottingClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshotting_cluster_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TransitEncryptionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_encryption_enabled: Option<bool>,
    #[serde(rename = "TransitEncryptionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_encryption_mode: Option<String>,
    #[serde(rename = "UserGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_group_ids: Option<UserGroupIdList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GlobalReplicationGroupInfo")]
pub struct GlobalReplicationGroupInfo {
    #[serde(rename = "GlobalReplicationGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_replication_group_id: Option<String>,
    #[serde(rename = "GlobalReplicationGroupMemberRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_replication_group_member_role: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterIdList {
    #[serde(rename = "ClusterId", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ClusterIdList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ClusterIdList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationGroupOutpostArnList {
    #[serde(
        rename = "ReplicationGroupOutpostArn",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ReplicationGroupOutpostArnList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ReplicationGroupOutpostArnList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeGroupList {
    #[serde(rename = "NodeGroup", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<NodeGroup>,
}
impl From<Vec<NodeGroup>> for NodeGroupList {
    fn from(v: Vec<NodeGroup>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<NodeGroup> for NodeGroupList {
    fn from_iter<I: IntoIterator<Item = NodeGroup>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<NodeGroup>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlNodeGroupList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<NodeGroup>,
}

impl From<Vec<NodeGroup>> for XmlNodeGroupList {
    fn from(v: Vec<NodeGroup>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<NodeGroup> for XmlNodeGroupList {
    fn from_iter<I: IntoIterator<Item = NodeGroup>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "NodeGroup")]
pub struct NodeGroup {
    #[serde(rename = "NodeGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_group_id: Option<String>,
    #[serde(rename = "NodeGroupMembers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_group_members: Option<NodeGroupMemberList>,
    #[serde(rename = "PrimaryEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_endpoint: Option<Endpoint>,
    #[serde(rename = "ReaderEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reader_endpoint: Option<Endpoint>,
    #[serde(rename = "Slots")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slots: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeGroupMemberList {
    #[serde(
        rename = "NodeGroupMember",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<NodeGroupMember>,
}
impl From<Vec<NodeGroupMember>> for NodeGroupMemberList {
    fn from(v: Vec<NodeGroupMember>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<NodeGroupMember> for NodeGroupMemberList {
    fn from_iter<I: IntoIterator<Item = NodeGroupMember>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<NodeGroupMember>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlNodeGroupMemberList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<NodeGroupMember>,
}

impl From<Vec<NodeGroupMember>> for XmlNodeGroupMemberList {
    fn from(v: Vec<NodeGroupMember>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<NodeGroupMember> for XmlNodeGroupMemberList {
    fn from_iter<I: IntoIterator<Item = NodeGroupMember>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "NodeGroupMember")]
pub struct NodeGroupMember {
    #[serde(rename = "CacheClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_id: Option<String>,
    #[serde(rename = "CacheNodeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_id: Option<String>,
    #[serde(rename = "CurrentRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_role: Option<String>,
    #[serde(rename = "PreferredAvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_availability_zone: Option<String>,
    #[serde(rename = "PreferredOutpostArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_outpost_arn: Option<String>,
    #[serde(rename = "ReadEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_endpoint: Option<Endpoint>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ReplicationGroupPendingModifiedValues")]
pub struct ReplicationGroupPendingModifiedValues {
    #[serde(rename = "AuthTokenStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token_status: Option<String>,
    #[serde(rename = "AutomaticFailoverStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_failover_status: Option<String>,
    #[serde(rename = "ClusterMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_mode: Option<String>,
    #[serde(rename = "LogDeliveryConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_delivery_configurations: Option<PendingLogDeliveryConfigurationList>,
    #[serde(rename = "PrimaryClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_cluster_id: Option<String>,
    #[serde(rename = "Resharding")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resharding: Option<ReshardingStatus>,
    #[serde(rename = "TransitEncryptionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_encryption_enabled: Option<bool>,
    #[serde(rename = "TransitEncryptionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_encryption_mode: Option<String>,
    #[serde(rename = "UserGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_groups: Option<UserGroupsUpdateStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ReshardingStatus")]
pub struct ReshardingStatus {
    #[serde(rename = "SlotMigration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_migration: Option<SlotMigration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SlotMigration")]
pub struct SlotMigration {
    #[serde(rename = "ProgressPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_percentage: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UserGroupsUpdateStatus")]
pub struct UserGroupsUpdateStatus {
    #[serde(rename = "UserGroupIdsToAdd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_group_ids_to_add: Option<UserGroupIdList>,
    #[serde(rename = "UserGroupIdsToRemove")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_group_ids_to_remove: Option<UserGroupIdList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyServerlessCacheRequest")]
pub struct ModifyServerlessCacheRequest {
    #[serde(rename = "CacheUsageLimits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_usage_limits: Option<CacheUsageLimits>,
    #[serde(rename = "DailySnapshotTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_snapshot_time: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "MajorEngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major_engine_version: Option<String>,
    #[serde(rename = "RemoveUserGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_user_group: Option<bool>,
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<SecurityGroupIdsList>,
    #[serde(rename = "ServerlessCacheName")]
    #[serde(default)]
    pub serverless_cache_name: String,
    #[serde(rename = "SnapshotRetentionLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_retention_limit: Option<i32>,
    #[serde(rename = "UserGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_group_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CacheUsageLimits")]
pub struct CacheUsageLimits {
    #[serde(rename = "DataStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_storage: Option<DataStorage>,
    #[serde(rename = "ECPUPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_c_p_u_per_second: Option<ECPUPerSecond>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DataStorage")]
pub struct DataStorage {
    #[serde(rename = "Maximum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i32>,
    #[serde(rename = "Minimum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i32>,
    #[serde(rename = "Unit")]
    #[serde(default)]
    pub unit: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ECPUPerSecond")]
pub struct ECPUPerSecond {
    #[serde(rename = "Maximum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i32>,
    #[serde(rename = "Minimum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecurityGroupIdsList {
    #[serde(
        rename = "SecurityGroupId",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for SecurityGroupIdsList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for SecurityGroupIdsList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResetCacheParameterGroupResult")]
pub struct CacheParameterGroupNameMessage {
    #[serde(rename = "CacheParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_parameter_group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEventsResult")]
pub struct EventsMessage {
    #[serde(rename = "Events")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<EventList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventList {
    #[serde(rename = "Event", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Event>,
}
impl From<Vec<Event>> for EventList {
    fn from(v: Vec<Event>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Event> for EventList {
    fn from_iter<I: IntoIterator<Item = Event>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Event>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlEventList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Event>,
}

impl From<Vec<Event>> for XmlEventList {
    fn from(v: Vec<Event>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Event> for XmlEventList {
    fn from_iter<I: IntoIterator<Item = Event>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Event")]
pub struct Event {
    #[serde(rename = "Date")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "SourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_identifier: Option<String>,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteCacheSubnetGroupMessage")]
pub struct DeleteCacheSubnetGroupMessage {
    #[serde(rename = "CacheSubnetGroupName")]
    #[serde(default)]
    pub cache_subnet_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateServerlessCacheSnapshotRequest")]
pub struct CreateServerlessCacheSnapshotRequest {
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "ServerlessCacheName")]
    #[serde(default)]
    pub serverless_cache_name: String,
    #[serde(rename = "ServerlessCacheSnapshotName")]
    #[serde(default)]
    pub serverless_cache_snapshot_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteServerlessCacheSnapshotRequest")]
pub struct DeleteServerlessCacheSnapshotRequest {
    #[serde(rename = "ServerlessCacheSnapshotName")]
    #[serde(default)]
    pub serverless_cache_snapshot_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEngineDefaultParametersMessage")]
pub struct DescribeEngineDefaultParametersMessage {
    #[serde(rename = "CacheParameterGroupFamily")]
    #[serde(default)]
    pub cache_parameter_group_family: String,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEventsMessage")]
pub struct DescribeEventsMessage {
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "SourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_identifier: Option<String>,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RebalanceSlotsInGlobalReplicationGroupResult")]
pub struct RebalanceSlotsInGlobalReplicationGroupResult {
    #[serde(rename = "GlobalReplicationGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_replication_group: Option<GlobalReplicationGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateGlobalReplicationGroupMessage")]
pub struct CreateGlobalReplicationGroupMessage {
    #[serde(rename = "GlobalReplicationGroupDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_replication_group_description: Option<String>,
    #[serde(rename = "GlobalReplicationGroupIdSuffix")]
    #[serde(default)]
    pub global_replication_group_id_suffix: String,
    #[serde(rename = "PrimaryReplicationGroupId")]
    #[serde(default)]
    pub primary_replication_group_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteCacheSecurityGroupMessage")]
pub struct DeleteCacheSecurityGroupMessage {
    #[serde(rename = "CacheSecurityGroupName")]
    #[serde(default)]
    pub cache_security_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateCacheClusterMessage")]
pub struct CreateCacheClusterMessage {
    #[serde(rename = "AZMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_z_mode: Option<String>,
    #[serde(rename = "AuthToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    #[serde(rename = "CacheClusterId")]
    #[serde(default)]
    pub cache_cluster_id: String,
    #[serde(rename = "CacheNodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_type: Option<String>,
    #[serde(rename = "CacheParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_parameter_group_name: Option<String>,
    #[serde(rename = "CacheSecurityGroupNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_security_group_names: Option<CacheSecurityGroupNameList>,
    #[serde(rename = "CacheSubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_subnet_group_name: Option<String>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "IpDiscovery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_discovery: Option<String>,
    #[serde(rename = "LogDeliveryConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_delivery_configurations: Option<LogDeliveryConfigurationRequestList>,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "NotificationTopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_topic_arn: Option<String>,
    #[serde(rename = "NumCacheNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_cache_nodes: Option<i32>,
    #[serde(rename = "OutpostMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_mode: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "PreferredAvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_availability_zone: Option<String>,
    #[serde(rename = "PreferredAvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_availability_zones: Option<PreferredAvailabilityZoneList>,
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[serde(rename = "PreferredOutpostArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_outpost_arn: Option<String>,
    #[serde(rename = "PreferredOutpostArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_outpost_arns: Option<PreferredOutpostArnList>,
    #[serde(rename = "ReplicationGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group_id: Option<String>,
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<SecurityGroupIdsList>,
    #[serde(rename = "SnapshotArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_arns: Option<SnapshotArnsList>,
    #[serde(rename = "SnapshotName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_name: Option<String>,
    #[serde(rename = "SnapshotRetentionLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_retention_limit: Option<i32>,
    #[serde(rename = "SnapshotWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_window: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "TransitEncryptionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_encryption_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CacheSecurityGroupNameList {
    #[serde(
        rename = "CacheSecurityGroupName",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for CacheSecurityGroupNameList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for CacheSecurityGroupNameList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogDeliveryConfigurationRequestList {
    #[serde(
        rename = "LogDeliveryConfigurationRequest",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<LogDeliveryConfigurationRequest>,
}
impl From<Vec<LogDeliveryConfigurationRequest>> for LogDeliveryConfigurationRequestList {
    fn from(v: Vec<LogDeliveryConfigurationRequest>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<LogDeliveryConfigurationRequest> for LogDeliveryConfigurationRequestList {
    fn from_iter<I: IntoIterator<Item = LogDeliveryConfigurationRequest>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<LogDeliveryConfigurationRequest>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlLogDeliveryConfigurationRequestList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<LogDeliveryConfigurationRequest>,
}

impl From<Vec<LogDeliveryConfigurationRequest>> for XmlLogDeliveryConfigurationRequestList {
    fn from(v: Vec<LogDeliveryConfigurationRequest>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<LogDeliveryConfigurationRequest> for XmlLogDeliveryConfigurationRequestList {
    fn from_iter<I: IntoIterator<Item = LogDeliveryConfigurationRequest>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LogDeliveryConfigurationRequest")]
pub struct LogDeliveryConfigurationRequest {
    #[serde(rename = "DestinationDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_details: Option<DestinationDetails>,
    #[serde(rename = "DestinationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_type: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "LogFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_format: Option<String>,
    #[serde(rename = "LogType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PreferredAvailabilityZoneList {
    #[serde(
        rename = "PreferredAvailabilityZone",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for PreferredAvailabilityZoneList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for PreferredAvailabilityZoneList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PreferredOutpostArnList {
    #[serde(
        rename = "PreferredOutpostArn",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for PreferredOutpostArnList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for PreferredOutpostArnList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnapshotArnsList {
    #[serde(rename = "SnapshotArn", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for SnapshotArnsList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for SnapshotArnsList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyUserGroupResult")]
pub struct UserGroup {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "MinimumEngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_engine_version: Option<String>,
    #[serde(rename = "PendingChanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_changes: Option<UserGroupPendingChanges>,
    #[serde(rename = "ReplicationGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_groups: Option<UGReplicationGroupIdList>,
    #[serde(rename = "ServerlessCaches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless_caches: Option<UGServerlessCacheIdList>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "UserGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_group_id: Option<String>,
    #[serde(rename = "UserIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<UserIdList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UserGroupPendingChanges")]
pub struct UserGroupPendingChanges {
    #[serde(rename = "UserIdsToAdd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids_to_add: Option<UserIdList>,
    #[serde(rename = "UserIdsToRemove")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids_to_remove: Option<UserIdList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserIdList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for UserIdList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for UserIdList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UGReplicationGroupIdList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for UGReplicationGroupIdList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for UGReplicationGroupIdList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UGServerlessCacheIdList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for UGServerlessCacheIdList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for UGServerlessCacheIdList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteServerlessCacheSnapshotResult")]
pub struct DeleteServerlessCacheSnapshotResponse {
    #[serde(rename = "ServerlessCacheSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless_cache_snapshot: Option<ServerlessCacheSnapshot>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ServerlessCacheSnapshot")]
pub struct ServerlessCacheSnapshot {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "BytesUsedForCache")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_used_for_cache: Option<String>,
    #[serde(rename = "CreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(rename = "ExpiryTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "ServerlessCacheConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless_cache_configuration: Option<ServerlessCacheConfiguration>,
    #[serde(rename = "ServerlessCacheSnapshotName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless_cache_snapshot_name: Option<String>,
    #[serde(rename = "SnapshotType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ServerlessCacheConfiguration")]
pub struct ServerlessCacheConfiguration {
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "MajorEngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major_engine_version: Option<String>,
    #[serde(rename = "ServerlessCacheName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless_cache_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeServerlessCachesResult")]
pub struct DescribeServerlessCachesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ServerlessCaches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless_caches: Option<ServerlessCacheList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServerlessCacheList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ServerlessCache>,
}
impl From<Vec<ServerlessCache>> for ServerlessCacheList {
    fn from(v: Vec<ServerlessCache>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ServerlessCache> for ServerlessCacheList {
    fn from_iter<I: IntoIterator<Item = ServerlessCache>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ServerlessCache>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlServerlessCacheList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ServerlessCache>,
}

impl From<Vec<ServerlessCache>> for XmlServerlessCacheList {
    fn from(v: Vec<ServerlessCache>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ServerlessCache> for XmlServerlessCacheList {
    fn from_iter<I: IntoIterator<Item = ServerlessCache>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ServerlessCache")]
pub struct ServerlessCache {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "CacheUsageLimits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_usage_limits: Option<CacheUsageLimits>,
    #[serde(rename = "CreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(rename = "DailySnapshotTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_snapshot_time: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<Endpoint>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "FullEngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_engine_version: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "MajorEngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major_engine_version: Option<String>,
    #[serde(rename = "ReaderEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reader_endpoint: Option<Endpoint>,
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<SecurityGroupIdsList>,
    #[serde(rename = "ServerlessCacheName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless_cache_name: Option<String>,
    #[serde(rename = "SnapshotRetentionLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_retention_limit: Option<i32>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<SubnetIdsList>,
    #[serde(rename = "UserGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_group_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubnetIdsList {
    #[serde(rename = "SubnetId", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for SubnetIdsList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for SubnetIdsList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeSnapshotsMessage")]
pub struct DescribeSnapshotsMessage {
    #[serde(rename = "CacheClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_id: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "ReplicationGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group_id: Option<String>,
    #[serde(rename = "ShowNodeGroupConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_node_group_config: Option<bool>,
    #[serde(rename = "SnapshotName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_name: Option<String>,
    #[serde(rename = "SnapshotSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PurchaseReservedCacheNodesOfferingResult")]
pub struct PurchaseReservedCacheNodesOfferingResult {
    #[serde(rename = "ReservedCacheNode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_cache_node: Option<ReservedCacheNode>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ReservedCacheNode")]
pub struct ReservedCacheNode {
    #[serde(rename = "CacheNodeCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_count: Option<i32>,
    #[serde(rename = "CacheNodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_type: Option<String>,
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "FixedPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_price: Option<f64>,
    #[serde(rename = "OfferingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_type: Option<String>,
    #[serde(rename = "ProductDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    #[serde(rename = "RecurringCharges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_charges: Option<RecurringChargeList>,
    #[serde(rename = "ReservationARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_a_r_n: Option<String>,
    #[serde(rename = "ReservedCacheNodeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_cache_node_id: Option<String>,
    #[serde(rename = "ReservedCacheNodesOfferingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_cache_nodes_offering_id: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "UsagePrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_price: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecurringChargeList {
    #[serde(
        rename = "RecurringCharge",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<RecurringCharge>,
}
impl From<Vec<RecurringCharge>> for RecurringChargeList {
    fn from(v: Vec<RecurringCharge>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<RecurringCharge> for RecurringChargeList {
    fn from_iter<I: IntoIterator<Item = RecurringCharge>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<RecurringCharge>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlRecurringChargeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<RecurringCharge>,
}

impl From<Vec<RecurringCharge>> for XmlRecurringChargeList {
    fn from(v: Vec<RecurringCharge>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<RecurringCharge> for XmlRecurringChargeList {
    fn from_iter<I: IntoIterator<Item = RecurringCharge>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RecurringCharge")]
pub struct RecurringCharge {
    #[serde(rename = "RecurringChargeAmount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_charge_amount: Option<f64>,
    #[serde(rename = "RecurringChargeFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_charge_frequency: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateUserMessage")]
pub struct CreateUserMessage {
    #[serde(rename = "AccessString")]
    #[serde(default)]
    pub access_string: String,
    #[serde(rename = "AuthenticationMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_mode: Option<AuthenticationMode>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    pub engine: String,
    #[serde(rename = "NoPasswordRequired")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_password_required: Option<bool>,
    #[serde(rename = "Passwords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passwords: Option<PasswordListInput>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "UserId")]
    #[serde(default)]
    pub user_id: String,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AuthenticationMode")]
pub struct AuthenticationMode {
    #[serde(rename = "Passwords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passwords: Option<PasswordListInput>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PasswordListInput {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for PasswordListInput {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for PasswordListInput {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "BatchStopUpdateActionMessage")]
pub struct BatchStopUpdateActionMessage {
    #[serde(rename = "CacheClusterIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_ids: Option<CacheClusterIdList>,
    #[serde(rename = "ReplicationGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group_ids: Option<ReplicationGroupIdList>,
    #[serde(rename = "ServiceUpdateName")]
    #[serde(default)]
    pub service_update_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CacheClusterIdList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for CacheClusterIdList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for CacheClusterIdList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationGroupIdList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ReplicationGroupIdList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ReplicationGroupIdList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateCacheSecurityGroupResult")]
pub struct CreateCacheSecurityGroupResult {
    #[serde(rename = "CacheSecurityGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_security_group: Option<CacheSecurityGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CacheSecurityGroup")]
pub struct CacheSecurityGroup {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "CacheSecurityGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_security_group_name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EC2SecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_c2_security_groups: Option<EC2SecurityGroupList>,
    #[serde(rename = "OwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EC2SecurityGroupList {
    #[serde(
        rename = "EC2SecurityGroup",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<EC2SecurityGroup>,
}
impl From<Vec<EC2SecurityGroup>> for EC2SecurityGroupList {
    fn from(v: Vec<EC2SecurityGroup>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<EC2SecurityGroup> for EC2SecurityGroupList {
    fn from_iter<I: IntoIterator<Item = EC2SecurityGroup>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<EC2SecurityGroup>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlEC2SecurityGroupList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<EC2SecurityGroup>,
}

impl From<Vec<EC2SecurityGroup>> for XmlEC2SecurityGroupList {
    fn from(v: Vec<EC2SecurityGroup>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<EC2SecurityGroup> for XmlEC2SecurityGroupList {
    fn from_iter<I: IntoIterator<Item = EC2SecurityGroup>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EC2SecurityGroup")]
pub struct EC2SecurityGroup {
    #[serde(rename = "EC2SecurityGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_c2_security_group_name: Option<String>,
    #[serde(rename = "EC2SecurityGroupOwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_c2_security_group_owner_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CopySnapshotMessage")]
pub struct CopySnapshotMessage {
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "SourceSnapshotName")]
    #[serde(default)]
    pub source_snapshot_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "TargetBucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_bucket: Option<String>,
    #[serde(rename = "TargetSnapshotName")]
    #[serde(default)]
    pub target_snapshot_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "FailoverGlobalReplicationGroupMessage")]
pub struct FailoverGlobalReplicationGroupMessage {
    #[serde(rename = "GlobalReplicationGroupId")]
    #[serde(default)]
    pub global_replication_group_id: String,
    #[serde(rename = "PrimaryRegion")]
    #[serde(default)]
    pub primary_region: String,
    #[serde(rename = "PrimaryReplicationGroupId")]
    #[serde(default)]
    pub primary_replication_group_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateServerlessCacheResult")]
pub struct CreateServerlessCacheResponse {
    #[serde(rename = "ServerlessCache")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless_cache: Option<ServerlessCache>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DecreaseNodeGroupsInGlobalReplicationGroupMessage")]
pub struct DecreaseNodeGroupsInGlobalReplicationGroupMessage {
    #[serde(rename = "ApplyImmediately")]
    #[serde(default)]
    pub apply_immediately: bool,
    #[serde(rename = "GlobalNodeGroupsToRemove")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_node_groups_to_remove: Option<GlobalNodeGroupIdList>,
    #[serde(rename = "GlobalNodeGroupsToRetain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_node_groups_to_retain: Option<GlobalNodeGroupIdList>,
    #[serde(rename = "GlobalReplicationGroupId")]
    #[serde(default)]
    pub global_replication_group_id: String,
    #[serde(rename = "NodeGroupCount")]
    #[serde(default)]
    pub node_group_count: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlobalNodeGroupIdList {
    #[serde(
        rename = "GlobalNodeGroupId",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for GlobalNodeGroupIdList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for GlobalNodeGroupIdList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AuthorizeCacheSecurityGroupIngressMessage")]
pub struct AuthorizeCacheSecurityGroupIngressMessage {
    #[serde(rename = "CacheSecurityGroupName")]
    #[serde(default)]
    pub cache_security_group_name: String,
    #[serde(rename = "EC2SecurityGroupName")]
    #[serde(default)]
    pub e_c2_security_group_name: String,
    #[serde(rename = "EC2SecurityGroupOwnerId")]
    #[serde(default)]
    pub e_c2_security_group_owner_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTagsForResourceMessage")]
pub struct ListTagsForResourceMessage {
    #[serde(rename = "ResourceName")]
    #[serde(default)]
    pub resource_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeCacheClustersResult")]
pub struct CacheClusterMessage {
    #[serde(rename = "CacheClusters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_clusters: Option<CacheClusterList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CacheClusterList {
    #[serde(
        rename = "CacheCluster",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<CacheCluster>,
}
impl From<Vec<CacheCluster>> for CacheClusterList {
    fn from(v: Vec<CacheCluster>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<CacheCluster> for CacheClusterList {
    fn from_iter<I: IntoIterator<Item = CacheCluster>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<CacheCluster>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlCacheClusterList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<CacheCluster>,
}

impl From<Vec<CacheCluster>> for XmlCacheClusterList {
    fn from(v: Vec<CacheCluster>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<CacheCluster> for XmlCacheClusterList {
    fn from_iter<I: IntoIterator<Item = CacheCluster>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeUsersMessage")]
pub struct DescribeUsersMessage {
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FilterList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "UserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Filter>,
}
impl From<Vec<Filter>> for FilterList {
    fn from(v: Vec<Filter>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Filter> for FilterList {
    fn from_iter<I: IntoIterator<Item = Filter>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Filter>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlFilterList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Filter>,
}

impl From<Vec<Filter>> for XmlFilterList {
    fn from(v: Vec<Filter>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Filter> for XmlFilterList {
    fn from_iter<I: IntoIterator<Item = Filter>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Filter")]
pub struct Filter {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: FilterValueList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterValueList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for FilterValueList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for FilterValueList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeCacheSecurityGroupsResult")]
pub struct CacheSecurityGroupMessage {
    #[serde(rename = "CacheSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_security_groups: Option<CacheSecurityGroups>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CacheSecurityGroups {
    #[serde(
        rename = "CacheSecurityGroup",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<CacheSecurityGroup>,
}
impl From<Vec<CacheSecurityGroup>> for CacheSecurityGroups {
    fn from(v: Vec<CacheSecurityGroup>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<CacheSecurityGroup> for CacheSecurityGroups {
    fn from_iter<I: IntoIterator<Item = CacheSecurityGroup>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<CacheSecurityGroup>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlCacheSecurityGroupList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<CacheSecurityGroup>,
}

impl From<Vec<CacheSecurityGroup>> for XmlCacheSecurityGroupList {
    fn from(v: Vec<CacheSecurityGroup>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<CacheSecurityGroup> for XmlCacheSecurityGroupList {
    fn from_iter<I: IntoIterator<Item = CacheSecurityGroup>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CompleteMigrationMessage")]
pub struct CompleteMigrationMessage {
    #[serde(rename = "Force")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[serde(rename = "ReplicationGroupId")]
    #[serde(default)]
    pub replication_group_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeUpdateActionsResult")]
pub struct UpdateActionsMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "UpdateActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_actions: Option<UpdateActionList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateActionList {
    #[serde(
        rename = "UpdateAction",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<UpdateAction>,
}
impl From<Vec<UpdateAction>> for UpdateActionList {
    fn from(v: Vec<UpdateAction>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<UpdateAction> for UpdateActionList {
    fn from_iter<I: IntoIterator<Item = UpdateAction>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<UpdateAction>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlUpdateActionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<UpdateAction>,
}

impl From<Vec<UpdateAction>> for XmlUpdateActionList {
    fn from(v: Vec<UpdateAction>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<UpdateAction> for XmlUpdateActionList {
    fn from_iter<I: IntoIterator<Item = UpdateAction>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateAction")]
pub struct UpdateAction {
    #[serde(rename = "CacheClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_id: Option<String>,
    #[serde(rename = "CacheNodeUpdateStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_update_status: Option<CacheNodeUpdateStatusList>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EstimatedUpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_update_time: Option<String>,
    #[serde(rename = "NodeGroupUpdateStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_group_update_status: Option<NodeGroupUpdateStatusList>,
    #[serde(rename = "NodesUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes_updated: Option<String>,
    #[serde(rename = "ReplicationGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group_id: Option<String>,
    #[serde(rename = "ServiceUpdateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_update_name: Option<String>,
    #[serde(rename = "ServiceUpdateRecommendedApplyByDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_update_recommended_apply_by_date: Option<String>,
    #[serde(rename = "ServiceUpdateReleaseDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_update_release_date: Option<String>,
    #[serde(rename = "ServiceUpdateSeverity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_update_severity: Option<String>,
    #[serde(rename = "ServiceUpdateStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_update_status: Option<String>,
    #[serde(rename = "ServiceUpdateType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_update_type: Option<String>,
    #[serde(rename = "SlaMet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sla_met: Option<String>,
    #[serde(rename = "UpdateActionAvailableDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_action_available_date: Option<String>,
    #[serde(rename = "UpdateActionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_action_status: Option<String>,
    #[serde(rename = "UpdateActionStatusModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_action_status_modified_date: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CacheNodeUpdateStatusList {
    #[serde(
        rename = "CacheNodeUpdateStatus",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<CacheNodeUpdateStatus>,
}
impl From<Vec<CacheNodeUpdateStatus>> for CacheNodeUpdateStatusList {
    fn from(v: Vec<CacheNodeUpdateStatus>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<CacheNodeUpdateStatus> for CacheNodeUpdateStatusList {
    fn from_iter<I: IntoIterator<Item = CacheNodeUpdateStatus>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<CacheNodeUpdateStatus>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlCacheNodeUpdateStatusList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<CacheNodeUpdateStatus>,
}

impl From<Vec<CacheNodeUpdateStatus>> for XmlCacheNodeUpdateStatusList {
    fn from(v: Vec<CacheNodeUpdateStatus>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<CacheNodeUpdateStatus> for XmlCacheNodeUpdateStatusList {
    fn from_iter<I: IntoIterator<Item = CacheNodeUpdateStatus>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CacheNodeUpdateStatus")]
pub struct CacheNodeUpdateStatus {
    #[serde(rename = "CacheNodeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_id: Option<String>,
    #[serde(rename = "NodeDeletionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_deletion_date: Option<String>,
    #[serde(rename = "NodeUpdateEndDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_update_end_date: Option<String>,
    #[serde(rename = "NodeUpdateInitiatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_update_initiated_by: Option<String>,
    #[serde(rename = "NodeUpdateInitiatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_update_initiated_date: Option<String>,
    #[serde(rename = "NodeUpdateStartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_update_start_date: Option<String>,
    #[serde(rename = "NodeUpdateStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_update_status: Option<String>,
    #[serde(rename = "NodeUpdateStatusModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_update_status_modified_date: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeGroupUpdateStatusList {
    #[serde(
        rename = "NodeGroupUpdateStatus",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<NodeGroupUpdateStatus>,
}
impl From<Vec<NodeGroupUpdateStatus>> for NodeGroupUpdateStatusList {
    fn from(v: Vec<NodeGroupUpdateStatus>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<NodeGroupUpdateStatus> for NodeGroupUpdateStatusList {
    fn from_iter<I: IntoIterator<Item = NodeGroupUpdateStatus>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<NodeGroupUpdateStatus>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlNodeGroupUpdateStatusList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<NodeGroupUpdateStatus>,
}

impl From<Vec<NodeGroupUpdateStatus>> for XmlNodeGroupUpdateStatusList {
    fn from(v: Vec<NodeGroupUpdateStatus>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<NodeGroupUpdateStatus> for XmlNodeGroupUpdateStatusList {
    fn from_iter<I: IntoIterator<Item = NodeGroupUpdateStatus>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "NodeGroupUpdateStatus")]
pub struct NodeGroupUpdateStatus {
    #[serde(rename = "NodeGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_group_id: Option<String>,
    #[serde(rename = "NodeGroupMemberUpdateStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_group_member_update_status: Option<NodeGroupMemberUpdateStatusList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeGroupMemberUpdateStatusList {
    #[serde(
        rename = "NodeGroupMemberUpdateStatus",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<NodeGroupMemberUpdateStatus>,
}
impl From<Vec<NodeGroupMemberUpdateStatus>> for NodeGroupMemberUpdateStatusList {
    fn from(v: Vec<NodeGroupMemberUpdateStatus>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<NodeGroupMemberUpdateStatus> for NodeGroupMemberUpdateStatusList {
    fn from_iter<I: IntoIterator<Item = NodeGroupMemberUpdateStatus>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<NodeGroupMemberUpdateStatus>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlNodeGroupMemberUpdateStatusList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<NodeGroupMemberUpdateStatus>,
}

impl From<Vec<NodeGroupMemberUpdateStatus>> for XmlNodeGroupMemberUpdateStatusList {
    fn from(v: Vec<NodeGroupMemberUpdateStatus>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<NodeGroupMemberUpdateStatus> for XmlNodeGroupMemberUpdateStatusList {
    fn from_iter<I: IntoIterator<Item = NodeGroupMemberUpdateStatus>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "NodeGroupMemberUpdateStatus")]
pub struct NodeGroupMemberUpdateStatus {
    #[serde(rename = "CacheClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_id: Option<String>,
    #[serde(rename = "CacheNodeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_id: Option<String>,
    #[serde(rename = "NodeDeletionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_deletion_date: Option<String>,
    #[serde(rename = "NodeUpdateEndDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_update_end_date: Option<String>,
    #[serde(rename = "NodeUpdateInitiatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_update_initiated_by: Option<String>,
    #[serde(rename = "NodeUpdateInitiatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_update_initiated_date: Option<String>,
    #[serde(rename = "NodeUpdateStartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_update_start_date: Option<String>,
    #[serde(rename = "NodeUpdateStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_update_status: Option<String>,
    #[serde(rename = "NodeUpdateStatusModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_update_status_modified_date: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RemoveTagsFromResourceResult")]
pub struct TagListMessage {
    #[serde(rename = "TagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeServerlessCacheSnapshotsResult")]
pub struct DescribeServerlessCacheSnapshotsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ServerlessCacheSnapshots")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless_cache_snapshots: Option<ServerlessCacheSnapshotList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServerlessCacheSnapshotList {
    #[serde(
        rename = "ServerlessCacheSnapshot",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ServerlessCacheSnapshot>,
}
impl From<Vec<ServerlessCacheSnapshot>> for ServerlessCacheSnapshotList {
    fn from(v: Vec<ServerlessCacheSnapshot>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ServerlessCacheSnapshot> for ServerlessCacheSnapshotList {
    fn from_iter<I: IntoIterator<Item = ServerlessCacheSnapshot>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ServerlessCacheSnapshot>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlServerlessCacheSnapshotList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ServerlessCacheSnapshot>,
}

impl From<Vec<ServerlessCacheSnapshot>> for XmlServerlessCacheSnapshotList {
    fn from(v: Vec<ServerlessCacheSnapshot>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ServerlessCacheSnapshot> for XmlServerlessCacheSnapshotList {
    fn from_iter<I: IntoIterator<Item = ServerlessCacheSnapshot>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateCacheParameterGroupResult")]
pub struct CreateCacheParameterGroupResult {
    #[serde(rename = "CacheParameterGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_parameter_group: Option<CacheParameterGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CacheParameterGroup")]
pub struct CacheParameterGroup {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "CacheParameterGroupFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_parameter_group_family: Option<String>,
    #[serde(rename = "CacheParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_parameter_group_name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "IsGlobal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_global: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteSnapshotMessage")]
pub struct DeleteSnapshotMessage {
    #[serde(rename = "SnapshotName")]
    #[serde(default)]
    pub snapshot_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyCacheClusterResult")]
pub struct ModifyCacheClusterResult {
    #[serde(rename = "CacheCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster: Option<CacheCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "IncreaseNodeGroupsInGlobalReplicationGroupMessage")]
pub struct IncreaseNodeGroupsInGlobalReplicationGroupMessage {
    #[serde(rename = "ApplyImmediately")]
    #[serde(default)]
    pub apply_immediately: bool,
    #[serde(rename = "GlobalReplicationGroupId")]
    #[serde(default)]
    pub global_replication_group_id: String,
    #[serde(rename = "NodeGroupCount")]
    #[serde(default)]
    pub node_group_count: i32,
    #[serde(rename = "RegionalConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regional_configurations: Option<RegionalConfigurationList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegionalConfigurationList {
    #[serde(
        rename = "RegionalConfiguration",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<RegionalConfiguration>,
}
impl From<Vec<RegionalConfiguration>> for RegionalConfigurationList {
    fn from(v: Vec<RegionalConfiguration>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<RegionalConfiguration> for RegionalConfigurationList {
    fn from_iter<I: IntoIterator<Item = RegionalConfiguration>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<RegionalConfiguration>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlRegionalConfigurationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<RegionalConfiguration>,
}

impl From<Vec<RegionalConfiguration>> for XmlRegionalConfigurationList {
    fn from(v: Vec<RegionalConfiguration>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<RegionalConfiguration> for XmlRegionalConfigurationList {
    fn from_iter<I: IntoIterator<Item = RegionalConfiguration>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RegionalConfiguration")]
pub struct RegionalConfiguration {
    #[serde(rename = "ReplicationGroupId")]
    #[serde(default)]
    pub replication_group_id: String,
    #[serde(rename = "ReplicationGroupRegion")]
    #[serde(default)]
    pub replication_group_region: String,
    #[serde(rename = "ReshardingConfiguration")]
    #[serde(default)]
    pub resharding_configuration: ReshardingConfigurationList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReshardingConfigurationList {
    #[serde(
        rename = "ReshardingConfiguration",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ReshardingConfiguration>,
}
impl From<Vec<ReshardingConfiguration>> for ReshardingConfigurationList {
    fn from(v: Vec<ReshardingConfiguration>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ReshardingConfiguration> for ReshardingConfigurationList {
    fn from_iter<I: IntoIterator<Item = ReshardingConfiguration>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ReshardingConfiguration>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlReshardingConfigurationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ReshardingConfiguration>,
}

impl From<Vec<ReshardingConfiguration>> for XmlReshardingConfigurationList {
    fn from(v: Vec<ReshardingConfiguration>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ReshardingConfiguration> for XmlReshardingConfigurationList {
    fn from_iter<I: IntoIterator<Item = ReshardingConfiguration>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ReshardingConfiguration")]
pub struct ReshardingConfiguration {
    #[serde(rename = "NodeGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_group_id: Option<String>,
    #[serde(rename = "PreferredAvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_availability_zones: Option<AvailabilityZonesList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AvailabilityZonesList {
    #[serde(
        rename = "AvailabilityZone",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for AvailabilityZonesList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for AvailabilityZonesList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PurchaseReservedCacheNodesOfferingMessage")]
pub struct PurchaseReservedCacheNodesOfferingMessage {
    #[serde(rename = "CacheNodeCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_count: Option<i32>,
    #[serde(rename = "ReservedCacheNodeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_cache_node_id: Option<String>,
    #[serde(rename = "ReservedCacheNodesOfferingId")]
    #[serde(default)]
    pub reserved_cache_nodes_offering_id: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListAllowedNodeTypeModificationsMessage")]
pub struct ListAllowedNodeTypeModificationsMessage {
    #[serde(rename = "CacheClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_id: Option<String>,
    #[serde(rename = "ReplicationGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateCacheSubnetGroupResult")]
pub struct CreateCacheSubnetGroupResult {
    #[serde(rename = "CacheSubnetGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_subnet_group: Option<CacheSubnetGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RebootCacheClusterResult")]
pub struct RebootCacheClusterResult {
    #[serde(rename = "CacheCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster: Option<CacheCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyReplicationGroupShardConfigurationResult")]
pub struct ModifyReplicationGroupShardConfigurationResult {
    #[serde(rename = "ReplicationGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group: Option<ReplicationGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RebootCacheClusterMessage")]
pub struct RebootCacheClusterMessage {
    #[serde(rename = "CacheClusterId")]
    #[serde(default)]
    pub cache_cluster_id: String,
    #[serde(rename = "CacheNodeIdsToReboot")]
    #[serde(default)]
    pub cache_node_ids_to_reboot: CacheNodeIdsList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteUserMessage")]
pub struct DeleteUserMessage {
    #[serde(rename = "UserId")]
    #[serde(default)]
    pub user_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteSnapshotResult")]
pub struct DeleteSnapshotResult {
    #[serde(rename = "Snapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<Snapshot>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Snapshot")]
pub struct Snapshot {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    #[serde(rename = "AutomaticFailover")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_failover: Option<String>,
    #[serde(rename = "CacheClusterCreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_create_time: Option<String>,
    #[serde(rename = "CacheClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_id: Option<String>,
    #[serde(rename = "CacheNodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_type: Option<String>,
    #[serde(rename = "CacheParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_parameter_group_name: Option<String>,
    #[serde(rename = "CacheSubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_subnet_group_name: Option<String>,
    #[serde(rename = "DataTiering")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_tiering: Option<String>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "NodeSnapshots")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_snapshots: Option<NodeSnapshotList>,
    #[serde(rename = "NumCacheNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_cache_nodes: Option<i32>,
    #[serde(rename = "NumNodeGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_node_groups: Option<i32>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "PreferredAvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_availability_zone: Option<String>,
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[serde(rename = "PreferredOutpostArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_outpost_arn: Option<String>,
    #[serde(rename = "ReplicationGroupDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group_description: Option<String>,
    #[serde(rename = "ReplicationGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group_id: Option<String>,
    #[serde(rename = "SnapshotName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_name: Option<String>,
    #[serde(rename = "SnapshotRetentionLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_retention_limit: Option<i32>,
    #[serde(rename = "SnapshotSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_source: Option<String>,
    #[serde(rename = "SnapshotStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_status: Option<String>,
    #[serde(rename = "SnapshotWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_window: Option<String>,
    #[serde(rename = "TopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeSnapshotList {
    #[serde(
        rename = "NodeSnapshot",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<NodeSnapshot>,
}
impl From<Vec<NodeSnapshot>> for NodeSnapshotList {
    fn from(v: Vec<NodeSnapshot>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<NodeSnapshot> for NodeSnapshotList {
    fn from_iter<I: IntoIterator<Item = NodeSnapshot>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<NodeSnapshot>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlNodeSnapshotList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<NodeSnapshot>,
}

impl From<Vec<NodeSnapshot>> for XmlNodeSnapshotList {
    fn from(v: Vec<NodeSnapshot>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<NodeSnapshot> for XmlNodeSnapshotList {
    fn from_iter<I: IntoIterator<Item = NodeSnapshot>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "NodeSnapshot")]
pub struct NodeSnapshot {
    #[serde(rename = "CacheClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_id: Option<String>,
    #[serde(rename = "CacheNodeCreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_create_time: Option<String>,
    #[serde(rename = "CacheNodeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_id: Option<String>,
    #[serde(rename = "CacheSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_size: Option<String>,
    #[serde(rename = "NodeGroupConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_group_configuration: Option<NodeGroupConfiguration>,
    #[serde(rename = "NodeGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_group_id: Option<String>,
    #[serde(rename = "SnapshotCreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_create_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "NodeGroupConfiguration")]
pub struct NodeGroupConfiguration {
    #[serde(rename = "NodeGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_group_id: Option<String>,
    #[serde(rename = "PrimaryAvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_availability_zone: Option<String>,
    #[serde(rename = "PrimaryOutpostArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_outpost_arn: Option<String>,
    #[serde(rename = "ReplicaAvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_availability_zones: Option<AvailabilityZonesList>,
    #[serde(rename = "ReplicaCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_count: Option<i32>,
    #[serde(rename = "ReplicaOutpostArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_outpost_arns: Option<OutpostArnsList>,
    #[serde(rename = "Slots")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slots: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutpostArnsList {
    #[serde(rename = "OutpostArn", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for OutpostArnsList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for OutpostArnsList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DecreaseReplicaCountMessage")]
pub struct DecreaseReplicaCountMessage {
    #[serde(rename = "ApplyImmediately")]
    #[serde(default)]
    pub apply_immediately: bool,
    #[serde(rename = "NewReplicaCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_replica_count: Option<i32>,
    #[serde(rename = "ReplicaConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_configuration: Option<ReplicaConfigurationList>,
    #[serde(rename = "ReplicasToRemove")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas_to_remove: Option<RemoveReplicasList>,
    #[serde(rename = "ReplicationGroupId")]
    #[serde(default)]
    pub replication_group_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicaConfigurationList {
    #[serde(
        rename = "ConfigureShard",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ConfigureShard>,
}
impl From<Vec<ConfigureShard>> for ReplicaConfigurationList {
    fn from(v: Vec<ConfigureShard>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ConfigureShard> for ReplicaConfigurationList {
    fn from_iter<I: IntoIterator<Item = ConfigureShard>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ConfigureShard>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlConfigureShardList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ConfigureShard>,
}

impl From<Vec<ConfigureShard>> for XmlConfigureShardList {
    fn from(v: Vec<ConfigureShard>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ConfigureShard> for XmlConfigureShardList {
    fn from_iter<I: IntoIterator<Item = ConfigureShard>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ConfigureShard")]
pub struct ConfigureShard {
    #[serde(rename = "NewReplicaCount")]
    #[serde(default)]
    pub new_replica_count: i32,
    #[serde(rename = "NodeGroupId")]
    #[serde(default)]
    pub node_group_id: String,
    #[serde(rename = "PreferredAvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_availability_zones: Option<PreferredAvailabilityZoneList>,
    #[serde(rename = "PreferredOutpostArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_outpost_arns: Option<PreferredOutpostArnList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveReplicasList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for RemoveReplicasList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for RemoveReplicasList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "BatchApplyUpdateActionMessage")]
pub struct BatchApplyUpdateActionMessage {
    #[serde(rename = "CacheClusterIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_ids: Option<CacheClusterIdList>,
    #[serde(rename = "ReplicationGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group_ids: Option<ReplicationGroupIdList>,
    #[serde(rename = "ServiceUpdateName")]
    #[serde(default)]
    pub service_update_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeCacheParameterGroupsMessage")]
pub struct DescribeCacheParameterGroupsMessage {
    #[serde(rename = "CacheParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_parameter_group_name: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AuthorizeCacheSecurityGroupIngressResult")]
pub struct AuthorizeCacheSecurityGroupIngressResult {
    #[serde(rename = "CacheSecurityGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_security_group: Option<CacheSecurityGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateCacheSecurityGroupMessage")]
pub struct CreateCacheSecurityGroupMessage {
    #[serde(rename = "CacheSecurityGroupName")]
    #[serde(default)]
    pub cache_security_group_name: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    pub description: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeServerlessCacheSnapshotsRequest")]
pub struct DescribeServerlessCacheSnapshotsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ServerlessCacheName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless_cache_name: Option<String>,
    #[serde(rename = "ServerlessCacheSnapshotName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless_cache_snapshot_name: Option<String>,
    #[serde(rename = "SnapshotType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyCacheClusterMessage")]
pub struct ModifyCacheClusterMessage {
    #[serde(rename = "AZMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_z_mode: Option<String>,
    #[serde(rename = "ApplyImmediately")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_immediately: Option<bool>,
    #[serde(rename = "AuthToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    #[serde(rename = "AuthTokenUpdateStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token_update_strategy: Option<String>,
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    #[serde(rename = "CacheClusterId")]
    #[serde(default)]
    pub cache_cluster_id: String,
    #[serde(rename = "CacheNodeIdsToRemove")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_ids_to_remove: Option<CacheNodeIdsList>,
    #[serde(rename = "CacheNodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_type: Option<String>,
    #[serde(rename = "CacheParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_parameter_group_name: Option<String>,
    #[serde(rename = "CacheSecurityGroupNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_security_group_names: Option<CacheSecurityGroupNameList>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "IpDiscovery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_discovery: Option<String>,
    #[serde(rename = "LogDeliveryConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_delivery_configurations: Option<LogDeliveryConfigurationRequestList>,
    #[serde(rename = "NewAvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_availability_zones: Option<PreferredAvailabilityZoneList>,
    #[serde(rename = "NotificationTopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_topic_arn: Option<String>,
    #[serde(rename = "NotificationTopicStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_topic_status: Option<String>,
    #[serde(rename = "NumCacheNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_cache_nodes: Option<i32>,
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[serde(rename = "ScaleConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_config: Option<ScaleConfig>,
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<SecurityGroupIdsList>,
    #[serde(rename = "SnapshotRetentionLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_retention_limit: Option<i32>,
    #[serde(rename = "SnapshotWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_window: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeCacheSecurityGroupsMessage")]
pub struct DescribeCacheSecurityGroupsMessage {
    #[serde(rename = "CacheSecurityGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_security_group_name: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateServerlessCacheRequest")]
pub struct CreateServerlessCacheRequest {
    #[serde(rename = "CacheUsageLimits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_usage_limits: Option<CacheUsageLimits>,
    #[serde(rename = "DailySnapshotTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_snapshot_time: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    pub engine: String,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "MajorEngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major_engine_version: Option<String>,
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<SecurityGroupIdsList>,
    #[serde(rename = "ServerlessCacheName")]
    #[serde(default)]
    pub serverless_cache_name: String,
    #[serde(rename = "SnapshotArnsToRestore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_arns_to_restore: Option<SnapshotArnsList>,
    #[serde(rename = "SnapshotRetentionLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_retention_limit: Option<i32>,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<SubnetIdsList>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "UserGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_group_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteCacheClusterMessage")]
pub struct DeleteCacheClusterMessage {
    #[serde(rename = "CacheClusterId")]
    #[serde(default)]
    pub cache_cluster_id: String,
    #[serde(rename = "FinalSnapshotIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_snapshot_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateServerlessCacheSnapshotResult")]
pub struct CreateServerlessCacheSnapshotResponse {
    #[serde(rename = "ServerlessCacheSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless_cache_snapshot: Option<ServerlessCacheSnapshot>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteReplicationGroupResult")]
pub struct DeleteReplicationGroupResult {
    #[serde(rename = "ReplicationGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group: Option<ReplicationGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeCacheParametersResult")]
pub struct CacheParameterGroupDetails {
    #[serde(rename = "CacheNodeTypeSpecificParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_type_specific_parameters: Option<CacheNodeTypeSpecificParametersList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParametersList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ExportServerlessCacheSnapshotRequest")]
pub struct ExportServerlessCacheSnapshotRequest {
    #[serde(rename = "S3BucketName")]
    #[serde(default)]
    pub s3_bucket_name: String,
    #[serde(rename = "ServerlessCacheSnapshotName")]
    #[serde(default)]
    pub serverless_cache_snapshot_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyUserMessage")]
pub struct ModifyUserMessage {
    #[serde(rename = "AccessString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_string: Option<String>,
    #[serde(rename = "AppendAccessString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub append_access_string: Option<String>,
    #[serde(rename = "AuthenticationMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_mode: Option<AuthenticationMode>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "NoPasswordRequired")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_password_required: Option<bool>,
    #[serde(rename = "Passwords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passwords: Option<PasswordListInput>,
    #[serde(rename = "UserId")]
    #[serde(default)]
    pub user_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteReplicationGroupMessage")]
pub struct DeleteReplicationGroupMessage {
    #[serde(rename = "FinalSnapshotIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_snapshot_identifier: Option<String>,
    #[serde(rename = "ReplicationGroupId")]
    #[serde(default)]
    pub replication_group_id: String,
    #[serde(rename = "RetainPrimaryCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain_primary_cluster: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeCacheClustersMessage")]
pub struct DescribeCacheClustersMessage {
    #[serde(rename = "CacheClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_id: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "ShowCacheClustersNotInReplicationGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_cache_clusters_not_in_replication_groups: Option<bool>,
    #[serde(rename = "ShowCacheNodeInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_cache_node_info: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeCacheSubnetGroupsResult")]
pub struct CacheSubnetGroupMessage {
    #[serde(rename = "CacheSubnetGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_subnet_groups: Option<CacheSubnetGroups>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CacheSubnetGroups {
    #[serde(
        rename = "CacheSubnetGroup",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<CacheSubnetGroup>,
}
impl From<Vec<CacheSubnetGroup>> for CacheSubnetGroups {
    fn from(v: Vec<CacheSubnetGroup>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<CacheSubnetGroup> for CacheSubnetGroups {
    fn from_iter<I: IntoIterator<Item = CacheSubnetGroup>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<CacheSubnetGroup>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlCacheSubnetGroupList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<CacheSubnetGroup>,
}

impl From<Vec<CacheSubnetGroup>> for XmlCacheSubnetGroupList {
    fn from(v: Vec<CacheSubnetGroup>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<CacheSubnetGroup> for XmlCacheSubnetGroupList {
    fn from_iter<I: IntoIterator<Item = CacheSubnetGroup>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RevokeCacheSecurityGroupIngressResult")]
pub struct RevokeCacheSecurityGroupIngressResult {
    #[serde(rename = "CacheSecurityGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_security_group: Option<CacheSecurityGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeReplicationGroupsMessage")]
pub struct DescribeReplicationGroupsMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "ReplicationGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RevokeCacheSecurityGroupIngressMessage")]
pub struct RevokeCacheSecurityGroupIngressMessage {
    #[serde(rename = "CacheSecurityGroupName")]
    #[serde(default)]
    pub cache_security_group_name: String,
    #[serde(rename = "EC2SecurityGroupName")]
    #[serde(default)]
    pub e_c2_security_group_name: String,
    #[serde(rename = "EC2SecurityGroupOwnerId")]
    #[serde(default)]
    pub e_c2_security_group_owner_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeCacheEngineVersionsMessage")]
pub struct DescribeCacheEngineVersionsMessage {
    #[serde(rename = "CacheParameterGroupFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_parameter_group_family: Option<String>,
    #[serde(rename = "DefaultOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_only: Option<bool>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyServerlessCacheResult")]
pub struct ModifyServerlessCacheResponse {
    #[serde(rename = "ServerlessCache")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless_cache: Option<ServerlessCache>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeReservedCacheNodesResult")]
pub struct ReservedCacheNodeMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "ReservedCacheNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_cache_nodes: Option<ReservedCacheNodeList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReservedCacheNodeList {
    #[serde(
        rename = "ReservedCacheNode",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ReservedCacheNode>,
}
impl From<Vec<ReservedCacheNode>> for ReservedCacheNodeList {
    fn from(v: Vec<ReservedCacheNode>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ReservedCacheNode> for ReservedCacheNodeList {
    fn from_iter<I: IntoIterator<Item = ReservedCacheNode>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ReservedCacheNode>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlReservedCacheNodeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ReservedCacheNode>,
}

impl From<Vec<ReservedCacheNode>> for XmlReservedCacheNodeList {
    fn from(v: Vec<ReservedCacheNode>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ReservedCacheNode> for XmlReservedCacheNodeList {
    fn from_iter<I: IntoIterator<Item = ReservedCacheNode>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RemoveTagsFromResourceMessage")]
pub struct RemoveTagsFromResourceMessage {
    #[serde(rename = "ResourceName")]
    #[serde(default)]
    pub resource_name: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: KeyList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KeyList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for KeyList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for KeyList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CopySnapshotResult")]
pub struct CopySnapshotResult {
    #[serde(rename = "Snapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<Snapshot>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateSnapshotMessage")]
pub struct CreateSnapshotMessage {
    #[serde(rename = "CacheClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_id: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "ReplicationGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group_id: Option<String>,
    #[serde(rename = "SnapshotName")]
    #[serde(default)]
    pub snapshot_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "IncreaseReplicaCountResult")]
pub struct IncreaseReplicaCountResult {
    #[serde(rename = "ReplicationGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group: Option<ReplicationGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RebalanceSlotsInGlobalReplicationGroupMessage")]
pub struct RebalanceSlotsInGlobalReplicationGroupMessage {
    #[serde(rename = "ApplyImmediately")]
    #[serde(default)]
    pub apply_immediately: bool,
    #[serde(rename = "GlobalReplicationGroupId")]
    #[serde(default)]
    pub global_replication_group_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResetCacheParameterGroupMessage")]
pub struct ResetCacheParameterGroupMessage {
    #[serde(rename = "CacheParameterGroupName")]
    #[serde(default)]
    pub cache_parameter_group_name: String,
    #[serde(rename = "ParameterNameValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_name_values: Option<ParameterNameValueList>,
    #[serde(rename = "ResetAllParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_all_parameters: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteServerlessCacheResult")]
pub struct DeleteServerlessCacheResponse {
    #[serde(rename = "ServerlessCache")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless_cache: Option<ServerlessCache>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateCacheParameterGroupMessage")]
pub struct CreateCacheParameterGroupMessage {
    #[serde(rename = "CacheParameterGroupFamily")]
    #[serde(default)]
    pub cache_parameter_group_family: String,
    #[serde(rename = "CacheParameterGroupName")]
    #[serde(default)]
    pub cache_parameter_group_name: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    pub description: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateReplicationGroupMessage")]
pub struct CreateReplicationGroupMessage {
    #[serde(rename = "AtRestEncryptionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_rest_encryption_enabled: Option<bool>,
    #[serde(rename = "AuthToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    #[serde(rename = "AutomaticFailoverEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_failover_enabled: Option<bool>,
    #[serde(rename = "CacheNodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_type: Option<String>,
    #[serde(rename = "CacheParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_parameter_group_name: Option<String>,
    #[serde(rename = "CacheSecurityGroupNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_security_group_names: Option<CacheSecurityGroupNameList>,
    #[serde(rename = "CacheSubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_subnet_group_name: Option<String>,
    #[serde(rename = "ClusterMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_mode: Option<String>,
    #[serde(rename = "DataTieringEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_tiering_enabled: Option<bool>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "GlobalReplicationGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_replication_group_id: Option<String>,
    #[serde(rename = "IpDiscovery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_discovery: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "LogDeliveryConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_delivery_configurations: Option<LogDeliveryConfigurationRequestList>,
    #[serde(rename = "MultiAZEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_a_z_enabled: Option<bool>,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "NodeGroupConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_group_configuration: Option<NodeGroupConfigurationList>,
    #[serde(rename = "NotificationTopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_topic_arn: Option<String>,
    #[serde(rename = "NumCacheClusters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_cache_clusters: Option<i32>,
    #[serde(rename = "NumNodeGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_node_groups: Option<i32>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "PreferredCacheClusterAZs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_cache_cluster_a_zs: Option<AvailabilityZonesList>,
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[serde(rename = "PrimaryClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_cluster_id: Option<String>,
    #[serde(rename = "ReplicasPerNodeGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replicas_per_node_group: Option<i32>,
    #[serde(rename = "ReplicationGroupDescription")]
    #[serde(default)]
    pub replication_group_description: String,
    #[serde(rename = "ReplicationGroupId")]
    #[serde(default)]
    pub replication_group_id: String,
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<SecurityGroupIdsList>,
    #[serde(rename = "ServerlessCacheSnapshotName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless_cache_snapshot_name: Option<String>,
    #[serde(rename = "SnapshotArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_arns: Option<SnapshotArnsList>,
    #[serde(rename = "SnapshotName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_name: Option<String>,
    #[serde(rename = "SnapshotRetentionLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_retention_limit: Option<i32>,
    #[serde(rename = "SnapshotWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_window: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "TransitEncryptionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_encryption_enabled: Option<bool>,
    #[serde(rename = "TransitEncryptionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_encryption_mode: Option<String>,
    #[serde(rename = "UserGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_group_ids: Option<UserGroupIdListInput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeGroupConfigurationList {
    #[serde(
        rename = "NodeGroupConfiguration",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<NodeGroupConfiguration>,
}
impl From<Vec<NodeGroupConfiguration>> for NodeGroupConfigurationList {
    fn from(v: Vec<NodeGroupConfiguration>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<NodeGroupConfiguration> for NodeGroupConfigurationList {
    fn from_iter<I: IntoIterator<Item = NodeGroupConfiguration>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<NodeGroupConfiguration>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlNodeGroupConfigurationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<NodeGroupConfiguration>,
}

impl From<Vec<NodeGroupConfiguration>> for XmlNodeGroupConfigurationList {
    fn from(v: Vec<NodeGroupConfiguration>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<NodeGroupConfiguration> for XmlNodeGroupConfigurationList {
    fn from_iter<I: IntoIterator<Item = NodeGroupConfiguration>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserGroupIdListInput {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for UserGroupIdListInput {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for UserGroupIdListInput {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeCacheSubnetGroupsMessage")]
pub struct DescribeCacheSubnetGroupsMessage {
    #[serde(rename = "CacheSubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_subnet_group_name: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeUserGroupsResult")]
pub struct DescribeUserGroupsResult {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "UserGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_groups: Option<UserGroupList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserGroupList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<UserGroup>,
}
impl From<Vec<UserGroup>> for UserGroupList {
    fn from(v: Vec<UserGroup>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<UserGroup> for UserGroupList {
    fn from_iter<I: IntoIterator<Item = UserGroup>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<UserGroup>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlUserGroupList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<UserGroup>,
}

impl From<Vec<UserGroup>> for XmlUserGroupList {
    fn from(v: Vec<UserGroup>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<UserGroup> for XmlUserGroupList {
    fn from_iter<I: IntoIterator<Item = UserGroup>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteCacheClusterResult")]
pub struct DeleteCacheClusterResult {
    #[serde(rename = "CacheCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster: Option<CacheCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TestFailoverResult")]
pub struct TestFailoverResult {
    #[serde(rename = "ReplicationGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group: Option<ReplicationGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AddTagsToResourceMessage")]
pub struct AddTagsToResourceMessage {
    #[serde(rename = "ResourceName")]
    #[serde(default)]
    pub resource_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: TagList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteGlobalReplicationGroupResult")]
pub struct DeleteGlobalReplicationGroupResult {
    #[serde(rename = "GlobalReplicationGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_replication_group: Option<GlobalReplicationGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeGlobalReplicationGroupsMessage")]
pub struct DescribeGlobalReplicationGroupsMessage {
    #[serde(rename = "GlobalReplicationGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_replication_group_id: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "ShowMemberInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_member_info: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ExportServerlessCacheSnapshotResult")]
pub struct ExportServerlessCacheSnapshotResponse {
    #[serde(rename = "ServerlessCacheSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless_cache_snapshot: Option<ServerlessCacheSnapshot>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "IncreaseReplicaCountMessage")]
pub struct IncreaseReplicaCountMessage {
    #[serde(rename = "ApplyImmediately")]
    #[serde(default)]
    pub apply_immediately: bool,
    #[serde(rename = "NewReplicaCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_replica_count: Option<i32>,
    #[serde(rename = "ReplicaConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_configuration: Option<ReplicaConfigurationList>,
    #[serde(rename = "ReplicationGroupId")]
    #[serde(default)]
    pub replication_group_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateSnapshotResult")]
pub struct CreateSnapshotResult {
    #[serde(rename = "Snapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<Snapshot>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateUserGroupMessage")]
pub struct CreateUserGroupMessage {
    #[serde(rename = "Engine")]
    #[serde(default)]
    pub engine: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "UserGroupId")]
    #[serde(default)]
    pub user_group_id: String,
    #[serde(rename = "UserIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<UserIdListInput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserIdListInput {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for UserIdListInput {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for UserIdListInput {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyReplicationGroupShardConfigurationMessage")]
pub struct ModifyReplicationGroupShardConfigurationMessage {
    #[serde(rename = "ApplyImmediately")]
    #[serde(default)]
    pub apply_immediately: bool,
    #[serde(rename = "NodeGroupCount")]
    #[serde(default)]
    pub node_group_count: i32,
    #[serde(rename = "NodeGroupsToRemove")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_groups_to_remove: Option<NodeGroupsToRemoveList>,
    #[serde(rename = "NodeGroupsToRetain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_groups_to_retain: Option<NodeGroupsToRetainList>,
    #[serde(rename = "ReplicationGroupId")]
    #[serde(default)]
    pub replication_group_id: String,
    #[serde(rename = "ReshardingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resharding_configuration: Option<ReshardingConfigurationList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeGroupsToRemoveList {
    #[serde(
        rename = "NodeGroupToRemove",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for NodeGroupsToRemoveList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for NodeGroupsToRemoveList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeGroupsToRetainList {
    #[serde(
        rename = "NodeGroupToRetain",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for NodeGroupsToRetainList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for NodeGroupsToRetainList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeReservedCacheNodesOfferingsResult")]
pub struct ReservedCacheNodesOfferingMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "ReservedCacheNodesOfferings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_cache_nodes_offerings: Option<ReservedCacheNodesOfferingList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReservedCacheNodesOfferingList {
    #[serde(
        rename = "ReservedCacheNodesOffering",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ReservedCacheNodesOffering>,
}
impl From<Vec<ReservedCacheNodesOffering>> for ReservedCacheNodesOfferingList {
    fn from(v: Vec<ReservedCacheNodesOffering>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ReservedCacheNodesOffering> for ReservedCacheNodesOfferingList {
    fn from_iter<I: IntoIterator<Item = ReservedCacheNodesOffering>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ReservedCacheNodesOffering>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlReservedCacheNodesOfferingList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ReservedCacheNodesOffering>,
}

impl From<Vec<ReservedCacheNodesOffering>> for XmlReservedCacheNodesOfferingList {
    fn from(v: Vec<ReservedCacheNodesOffering>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ReservedCacheNodesOffering> for XmlReservedCacheNodesOfferingList {
    fn from_iter<I: IntoIterator<Item = ReservedCacheNodesOffering>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ReservedCacheNodesOffering")]
pub struct ReservedCacheNodesOffering {
    #[serde(rename = "CacheNodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_type: Option<String>,
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "FixedPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_price: Option<f64>,
    #[serde(rename = "OfferingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_type: Option<String>,
    #[serde(rename = "ProductDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    #[serde(rename = "RecurringCharges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_charges: Option<RecurringChargeList>,
    #[serde(rename = "ReservedCacheNodesOfferingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_cache_nodes_offering_id: Option<String>,
    #[serde(rename = "UsagePrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_price: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DecreaseNodeGroupsInGlobalReplicationGroupResult")]
pub struct DecreaseNodeGroupsInGlobalReplicationGroupResult {
    #[serde(rename = "GlobalReplicationGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_replication_group: Option<GlobalReplicationGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CopyServerlessCacheSnapshotRequest")]
pub struct CopyServerlessCacheSnapshotRequest {
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "SourceServerlessCacheSnapshotName")]
    #[serde(default)]
    pub source_serverless_cache_snapshot_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "TargetServerlessCacheSnapshotName")]
    #[serde(default)]
    pub target_serverless_cache_snapshot_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateReplicationGroupResult")]
pub struct CreateReplicationGroupResult {
    #[serde(rename = "ReplicationGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group: Option<ReplicationGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteUserGroupMessage")]
pub struct DeleteUserGroupMessage {
    #[serde(rename = "UserGroupId")]
    #[serde(default)]
    pub user_group_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeReservedCacheNodesMessage")]
pub struct DescribeReservedCacheNodesMessage {
    #[serde(rename = "CacheNodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_type: Option<String>,
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "OfferingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_type: Option<String>,
    #[serde(rename = "ProductDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    #[serde(rename = "ReservedCacheNodeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_cache_node_id: Option<String>,
    #[serde(rename = "ReservedCacheNodesOfferingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_cache_nodes_offering_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeUserGroupsMessage")]
pub struct DescribeUserGroupsMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "UserGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_group_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StartMigrationMessage")]
pub struct StartMigrationMessage {
    #[serde(rename = "CustomerNodeEndpointList")]
    #[serde(default)]
    pub customer_node_endpoint_list: CustomerNodeEndpointList,
    #[serde(rename = "ReplicationGroupId")]
    #[serde(default)]
    pub replication_group_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeServiceUpdatesMessage")]
pub struct DescribeServiceUpdatesMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "ServiceUpdateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_update_name: Option<String>,
    #[serde(rename = "ServiceUpdateStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_update_status: Option<ServiceUpdateStatusList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceUpdateStatusList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ServiceUpdateStatusList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ServiceUpdateStatusList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CopyServerlessCacheSnapshotResult")]
pub struct CopyServerlessCacheSnapshotResponse {
    #[serde(rename = "ServerlessCacheSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless_cache_snapshot: Option<ServerlessCacheSnapshot>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeServerlessCachesRequest")]
pub struct DescribeServerlessCachesRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ServerlessCacheName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless_cache_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeSnapshotsResult")]
pub struct DescribeSnapshotsListMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Snapshots")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshots: Option<SnapshotList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnapshotList {
    #[serde(rename = "Snapshot", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Snapshot>,
}
impl From<Vec<Snapshot>> for SnapshotList {
    fn from(v: Vec<Snapshot>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Snapshot> for SnapshotList {
    fn from_iter<I: IntoIterator<Item = Snapshot>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Snapshot>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlSnapshotList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Snapshot>,
}

impl From<Vec<Snapshot>> for XmlSnapshotList {
    fn from(v: Vec<Snapshot>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Snapshot> for XmlSnapshotList {
    fn from_iter<I: IntoIterator<Item = Snapshot>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeUpdateActionsMessage")]
pub struct DescribeUpdateActionsMessage {
    #[serde(rename = "CacheClusterIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_ids: Option<CacheClusterIdList>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "ReplicationGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group_ids: Option<ReplicationGroupIdList>,
    #[serde(rename = "ServiceUpdateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_update_name: Option<String>,
    #[serde(rename = "ServiceUpdateStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_update_status: Option<ServiceUpdateStatusList>,
    #[serde(rename = "ServiceUpdateTimeRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_update_time_range: Option<TimeRangeFilter>,
    #[serde(rename = "ShowNodeLevelUpdateStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_node_level_update_status: Option<bool>,
    #[serde(rename = "UpdateActionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_action_status: Option<UpdateActionStatusList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TimeRangeFilter")]
pub struct TimeRangeFilter {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateActionStatusList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for UpdateActionStatusList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for UpdateActionStatusList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyReplicationGroupMessage")]
pub struct ModifyReplicationGroupMessage {
    #[serde(rename = "ApplyImmediately")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_immediately: Option<bool>,
    #[serde(rename = "AuthToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    #[serde(rename = "AuthTokenUpdateStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token_update_strategy: Option<String>,
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    #[serde(rename = "AutomaticFailoverEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_failover_enabled: Option<bool>,
    #[serde(rename = "CacheNodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_type: Option<String>,
    #[serde(rename = "CacheParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_parameter_group_name: Option<String>,
    #[serde(rename = "CacheSecurityGroupNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_security_group_names: Option<CacheSecurityGroupNameList>,
    #[serde(rename = "ClusterMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_mode: Option<String>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "IpDiscovery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_discovery: Option<String>,
    #[serde(rename = "LogDeliveryConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_delivery_configurations: Option<LogDeliveryConfigurationRequestList>,
    #[serde(rename = "MultiAZEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_a_z_enabled: Option<bool>,
    #[serde(rename = "NodeGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_group_id: Option<String>,
    #[serde(rename = "NotificationTopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_topic_arn: Option<String>,
    #[serde(rename = "NotificationTopicStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_topic_status: Option<String>,
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[serde(rename = "PrimaryClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_cluster_id: Option<String>,
    #[serde(rename = "RemoveUserGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_user_groups: Option<bool>,
    #[serde(rename = "ReplicationGroupDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group_description: Option<String>,
    #[serde(rename = "ReplicationGroupId")]
    #[serde(default)]
    pub replication_group_id: String,
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<SecurityGroupIdsList>,
    #[serde(rename = "SnapshotRetentionLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_retention_limit: Option<i32>,
    #[serde(rename = "SnapshotWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_window: Option<String>,
    #[serde(rename = "SnapshottingClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshotting_cluster_id: Option<String>,
    #[serde(rename = "TransitEncryptionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_encryption_enabled: Option<bool>,
    #[serde(rename = "TransitEncryptionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_encryption_mode: Option<String>,
    #[serde(rename = "UserGroupIdsToAdd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_group_ids_to_add: Option<UserGroupIdList>,
    #[serde(rename = "UserGroupIdsToRemove")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_group_ids_to_remove: Option<UserGroupIdList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeCacheParameterGroupsResult")]
pub struct CacheParameterGroupsMessage {
    #[serde(rename = "CacheParameterGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_parameter_groups: Option<CacheParameterGroupList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CacheParameterGroupList {
    #[serde(
        rename = "CacheParameterGroup",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<CacheParameterGroup>,
}
impl From<Vec<CacheParameterGroup>> for CacheParameterGroupList {
    fn from(v: Vec<CacheParameterGroup>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<CacheParameterGroup> for CacheParameterGroupList {
    fn from_iter<I: IntoIterator<Item = CacheParameterGroup>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<CacheParameterGroup>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlCacheParameterGroupList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<CacheParameterGroup>,
}

impl From<Vec<CacheParameterGroup>> for XmlCacheParameterGroupList {
    fn from(v: Vec<CacheParameterGroup>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<CacheParameterGroup> for XmlCacheParameterGroupList {
    fn from_iter<I: IntoIterator<Item = CacheParameterGroup>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StartMigrationResult")]
pub struct StartMigrationResponse {
    #[serde(rename = "ReplicationGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group: Option<ReplicationGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteServerlessCacheRequest")]
pub struct DeleteServerlessCacheRequest {
    #[serde(rename = "FinalSnapshotName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_snapshot_name: Option<String>,
    #[serde(rename = "ServerlessCacheName")]
    #[serde(default)]
    pub serverless_cache_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DisassociateGlobalReplicationGroupResult")]
pub struct DisassociateGlobalReplicationGroupResult {
    #[serde(rename = "GlobalReplicationGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_replication_group: Option<GlobalReplicationGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeGlobalReplicationGroupsResult")]
pub struct DescribeGlobalReplicationGroupsResult {
    #[serde(rename = "GlobalReplicationGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_replication_groups: Option<GlobalReplicationGroupList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlobalReplicationGroupList {
    #[serde(
        rename = "GlobalReplicationGroup",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<GlobalReplicationGroup>,
}
impl From<Vec<GlobalReplicationGroup>> for GlobalReplicationGroupList {
    fn from(v: Vec<GlobalReplicationGroup>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<GlobalReplicationGroup> for GlobalReplicationGroupList {
    fn from_iter<I: IntoIterator<Item = GlobalReplicationGroup>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<GlobalReplicationGroup>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlGlobalReplicationGroupList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<GlobalReplicationGroup>,
}

impl From<Vec<GlobalReplicationGroup>> for XmlGlobalReplicationGroupList {
    fn from(v: Vec<GlobalReplicationGroup>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<GlobalReplicationGroup> for XmlGlobalReplicationGroupList {
    fn from_iter<I: IntoIterator<Item = GlobalReplicationGroup>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeCacheEngineVersionsResult")]
pub struct CacheEngineVersionMessage {
    #[serde(rename = "CacheEngineVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_engine_versions: Option<CacheEngineVersionList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CacheEngineVersionList {
    #[serde(
        rename = "CacheEngineVersion",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<CacheEngineVersion>,
}
impl From<Vec<CacheEngineVersion>> for CacheEngineVersionList {
    fn from(v: Vec<CacheEngineVersion>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<CacheEngineVersion> for CacheEngineVersionList {
    fn from_iter<I: IntoIterator<Item = CacheEngineVersion>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<CacheEngineVersion>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlCacheEngineVersionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<CacheEngineVersion>,
}

impl From<Vec<CacheEngineVersion>> for XmlCacheEngineVersionList {
    fn from(v: Vec<CacheEngineVersion>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<CacheEngineVersion> for XmlCacheEngineVersionList {
    fn from_iter<I: IntoIterator<Item = CacheEngineVersion>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CacheEngineVersion")]
pub struct CacheEngineVersion {
    #[serde(rename = "CacheEngineDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_engine_description: Option<String>,
    #[serde(rename = "CacheEngineVersionDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_engine_version_description: Option<String>,
    #[serde(rename = "CacheParameterGroupFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_parameter_group_family: Option<String>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "IncreaseNodeGroupsInGlobalReplicationGroupResult")]
pub struct IncreaseNodeGroupsInGlobalReplicationGroupResult {
    #[serde(rename = "GlobalReplicationGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_replication_group: Option<GlobalReplicationGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListAllowedNodeTypeModificationsResult")]
pub struct AllowedNodeTypeModificationsMessage {
    #[serde(rename = "ScaleDownModifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_down_modifications: Option<NodeTypeList>,
    #[serde(rename = "ScaleUpModifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_up_modifications: Option<NodeTypeList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeTypeList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for NodeTypeList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for NodeTypeList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyUserGroupMessage")]
pub struct ModifyUserGroupMessage {
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "UserGroupId")]
    #[serde(default)]
    pub user_group_id: String,
    #[serde(rename = "UserIdsToAdd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids_to_add: Option<UserIdListInput>,
    #[serde(rename = "UserIdsToRemove")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids_to_remove: Option<UserIdListInput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteGlobalReplicationGroupMessage")]
pub struct DeleteGlobalReplicationGroupMessage {
    #[serde(rename = "GlobalReplicationGroupId")]
    #[serde(default)]
    pub global_replication_group_id: String,
    #[serde(rename = "RetainPrimaryReplicationGroup")]
    #[serde(default)]
    pub retain_primary_replication_group: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyGlobalReplicationGroupMessage")]
pub struct ModifyGlobalReplicationGroupMessage {
    #[serde(rename = "ApplyImmediately")]
    #[serde(default)]
    pub apply_immediately: bool,
    #[serde(rename = "AutomaticFailoverEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_failover_enabled: Option<bool>,
    #[serde(rename = "CacheNodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_node_type: Option<String>,
    #[serde(rename = "CacheParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_parameter_group_name: Option<String>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "GlobalReplicationGroupDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_replication_group_description: Option<String>,
    #[serde(rename = "GlobalReplicationGroupId")]
    #[serde(default)]
    pub global_replication_group_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TestMigrationResult")]
pub struct TestMigrationResponse {
    #[serde(rename = "ReplicationGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group: Option<ReplicationGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateGlobalReplicationGroupResult")]
pub struct CreateGlobalReplicationGroupResult {
    #[serde(rename = "GlobalReplicationGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_replication_group: Option<GlobalReplicationGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CompleteMigrationResult")]
pub struct CompleteMigrationResponse {
    #[serde(rename = "ReplicationGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group: Option<ReplicationGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DecreaseReplicaCountResult")]
pub struct DecreaseReplicaCountResult {
    #[serde(rename = "ReplicationGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group: Option<ReplicationGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyReplicationGroupResult")]
pub struct ModifyReplicationGroupResult {
    #[serde(rename = "ReplicationGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_group: Option<ReplicationGroup>,
}
