//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-rds

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RevokeDBSecurityGroupIngressMessage")]
pub struct RevokeDBSecurityGroupIngressMessage {
    #[serde(rename = "CIDRIP")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_i_d_r_i_p: Option<String>,
    #[serde(rename = "DBSecurityGroupName")]
    #[serde(default)]
    pub d_b_security_group_name: String,
    #[serde(rename = "EC2SecurityGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_c2_security_group_id: Option<String>,
    #[serde(rename = "EC2SecurityGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_c2_security_group_name: Option<String>,
    #[serde(rename = "EC2SecurityGroupOwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_c2_security_group_owner_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyTenantDatabaseResult")]
pub struct ModifyTenantDatabaseResult {
    #[serde(rename = "TenantDatabase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_database: Option<TenantDatabase>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TenantDatabase")]
pub struct TenantDatabase {
    #[serde(rename = "CharacterSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_set_name: Option<String>,
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_identifier: Option<String>,
    #[serde(rename = "DbiResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbi_resource_id: Option<String>,
    #[serde(rename = "DeletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    #[serde(rename = "MasterUserSecret")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_secret: Option<MasterUserSecret>,
    #[serde(rename = "MasterUsername")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_username: Option<String>,
    #[serde(rename = "NcharCharacterSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nchar_character_set_name: Option<String>,
    #[serde(rename = "PendingModifiedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_modified_values: Option<TenantDatabasePendingModifiedValues>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<TagList>,
    #[serde(rename = "TenantDBName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_d_b_name: Option<String>,
    #[serde(rename = "TenantDatabaseARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_database_a_r_n: Option<String>,
    #[serde(rename = "TenantDatabaseCreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_database_create_time: Option<String>,
    #[serde(rename = "TenantDatabaseResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_database_resource_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MasterUserSecret")]
pub struct MasterUserSecret {
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "SecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
    #[serde(rename = "SecretStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TenantDatabasePendingModifiedValues")]
pub struct TenantDatabasePendingModifiedValues {
    #[serde(rename = "MasterUserPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<String>,
    #[serde(rename = "TenantDBName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_d_b_name: Option<String>,
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
#[serde(rename = "CreateDBClusterParameterGroupMessage")]
pub struct CreateDBClusterParameterGroupMessage {
    #[serde(rename = "DBClusterParameterGroupName")]
    #[serde(default)]
    pub d_b_cluster_parameter_group_name: String,
    #[serde(rename = "DBParameterGroupFamily")]
    #[serde(default)]
    pub d_b_parameter_group_family: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    pub description: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SwitchoverBlueGreenDeploymentRequest")]
pub struct SwitchoverBlueGreenDeploymentRequest {
    #[serde(rename = "BlueGreenDeploymentIdentifier")]
    #[serde(default)]
    pub blue_green_deployment_identifier: String,
    #[serde(rename = "SwitchoverTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switchover_timeout: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBClusterSnapshotsMessage")]
pub struct DescribeDBClusterSnapshotsMessage {
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_identifier: Option<String>,
    #[serde(rename = "DBClusterSnapshotIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_snapshot_identifier: Option<String>,
    #[serde(rename = "DbClusterResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_resource_id: Option<String>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FilterList>,
    #[serde(rename = "IncludePublic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_public: Option<bool>,
    #[serde(rename = "IncludeShared")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_shared: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "SnapshotType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterList {
    #[serde(rename = "Filter", default, skip_serializing_if = "Vec::is_empty")]
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
    #[serde(rename = "Value", default, skip_serializing_if = "Vec::is_empty")]
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
#[serde(rename = "DescribeReservedDBInstancesResult")]
pub struct ReservedDBInstanceMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "ReservedDBInstances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_d_b_instances: Option<ReservedDBInstanceList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReservedDBInstanceList {
    #[serde(
        rename = "ReservedDBInstance",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ReservedDBInstance>,
}
impl From<Vec<ReservedDBInstance>> for ReservedDBInstanceList {
    fn from(v: Vec<ReservedDBInstance>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ReservedDBInstance> for ReservedDBInstanceList {
    fn from_iter<I: IntoIterator<Item = ReservedDBInstance>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ReservedDBInstance>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlReservedDBInstanceList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ReservedDBInstance>,
}

impl From<Vec<ReservedDBInstance>> for XmlReservedDBInstanceList {
    fn from(v: Vec<ReservedDBInstance>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ReservedDBInstance> for XmlReservedDBInstanceList {
    fn from_iter<I: IntoIterator<Item = ReservedDBInstance>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ReservedDBInstance")]
pub struct ReservedDBInstance {
    #[serde(rename = "CurrencyCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(rename = "DBInstanceClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_class: Option<String>,
    #[serde(rename = "DBInstanceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_count: Option<i32>,
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "FixedPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_price: Option<f64>,
    #[serde(rename = "LeaseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lease_id: Option<String>,
    #[serde(rename = "MultiAZ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_a_z: Option<bool>,
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
    #[serde(rename = "ReservedDBInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_d_b_instance_arn: Option<String>,
    #[serde(rename = "ReservedDBInstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_d_b_instance_id: Option<String>,
    #[serde(rename = "ReservedDBInstancesOfferingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_d_b_instances_offering_id: Option<String>,
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
#[serde(rename = "CreateDBProxyRequest")]
pub struct CreateDBProxyRequest {
    #[serde(rename = "Auth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<UserAuthConfigList>,
    #[serde(rename = "DBProxyName")]
    #[serde(default)]
    pub d_b_proxy_name: String,
    #[serde(rename = "DebugLogging")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_logging: Option<bool>,
    #[serde(rename = "DefaultAuthScheme")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_auth_scheme: Option<String>,
    #[serde(rename = "EndpointNetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_network_type: Option<String>,
    #[serde(rename = "EngineFamily")]
    #[serde(default)]
    pub engine_family: String,
    #[serde(rename = "IdleClientTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_client_timeout: Option<i32>,
    #[serde(rename = "RequireTLS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_t_l_s: Option<bool>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "TargetConnectionNetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_connection_network_type: Option<String>,
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<StringList>,
    #[serde(rename = "VpcSubnetIds")]
    #[serde(default)]
    pub vpc_subnet_ids: StringList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserAuthConfigList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<UserAuthConfig>,
}
impl From<Vec<UserAuthConfig>> for UserAuthConfigList {
    fn from(v: Vec<UserAuthConfig>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<UserAuthConfig> for UserAuthConfigList {
    fn from_iter<I: IntoIterator<Item = UserAuthConfig>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<UserAuthConfig>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlUserAuthConfigList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<UserAuthConfig>,
}

impl From<Vec<UserAuthConfig>> for XmlUserAuthConfigList {
    fn from(v: Vec<UserAuthConfig>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<UserAuthConfig> for XmlUserAuthConfigList {
    fn from_iter<I: IntoIterator<Item = UserAuthConfig>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UserAuthConfig")]
pub struct UserAuthConfig {
    #[serde(rename = "AuthScheme")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_scheme: Option<String>,
    #[serde(rename = "ClientPasswordAuthType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_password_auth_type: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "IAMAuth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_a_m_auth: Option<String>,
    #[serde(rename = "SecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StringList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for StringList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for StringList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateOptionGroupMessage")]
pub struct CreateOptionGroupMessage {
    #[serde(rename = "EngineName")]
    #[serde(default)]
    pub engine_name: String,
    #[serde(rename = "MajorEngineVersion")]
    #[serde(default)]
    pub major_engine_version: String,
    #[serde(rename = "OptionGroupDescription")]
    #[serde(default)]
    pub option_group_description: String,
    #[serde(rename = "OptionGroupName")]
    #[serde(default)]
    pub option_group_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StartDBInstanceAutomatedBackupsReplicationResult")]
pub struct StartDBInstanceAutomatedBackupsReplicationResult {
    #[serde(rename = "DBInstanceAutomatedBackup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_automated_backup: Option<DBInstanceAutomatedBackup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBInstanceAutomatedBackup")]
pub struct DBInstanceAutomatedBackup {
    #[serde(rename = "AdditionalStorageVolumes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_storage_volumes: Option<AdditionalStorageVolumesList>,
    #[serde(rename = "AllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "AwsBackupRecoveryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_backup_recovery_point_arn: Option<String>,
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i32>,
    #[serde(rename = "BackupTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_target: Option<String>,
    #[serde(rename = "DBInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_arn: Option<String>,
    #[serde(rename = "DBInstanceAutomatedBackupsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_automated_backups_arn: Option<String>,
    #[serde(rename = "DBInstanceAutomatedBackupsReplications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_automated_backups_replications:
        Option<DBInstanceAutomatedBackupsReplicationList>,
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_identifier: Option<String>,
    #[serde(rename = "DbiResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbi_resource_id: Option<String>,
    #[serde(rename = "DedicatedLogVolume")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_log_volume: Option<bool>,
    #[serde(rename = "Encrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "IAMDatabaseAuthenticationEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_a_m_database_authentication_enabled: Option<bool>,
    #[serde(rename = "InstanceCreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_create_time: Option<String>,
    #[serde(rename = "Iops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "LicenseModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_model: Option<String>,
    #[serde(rename = "MasterUsername")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_username: Option<String>,
    #[serde(rename = "MultiTenant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_tenant: Option<bool>,
    #[serde(rename = "OptionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_name: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "PreferredBackupWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "RestoreWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_window: Option<RestoreWindow>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StorageEncryptionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encryption_type: Option<String>,
    #[serde(rename = "StorageThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_throughput: Option<i32>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "TagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<TagList>,
    #[serde(rename = "TdeCredentialArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tde_credential_arn: Option<String>,
    #[serde(rename = "Timezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdditionalStorageVolumesList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<AdditionalStorageVolume>,
}
impl From<Vec<AdditionalStorageVolume>> for AdditionalStorageVolumesList {
    fn from(v: Vec<AdditionalStorageVolume>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<AdditionalStorageVolume> for AdditionalStorageVolumesList {
    fn from_iter<I: IntoIterator<Item = AdditionalStorageVolume>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<AdditionalStorageVolume>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlAdditionalStorageVolumeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<AdditionalStorageVolume>,
}

impl From<Vec<AdditionalStorageVolume>> for XmlAdditionalStorageVolumeList {
    fn from(v: Vec<AdditionalStorageVolume>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<AdditionalStorageVolume> for XmlAdditionalStorageVolumeList {
    fn from_iter<I: IntoIterator<Item = AdditionalStorageVolume>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AdditionalStorageVolume")]
pub struct AdditionalStorageVolume {
    #[serde(rename = "AllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(rename = "IOPS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_o_p_s: Option<i32>,
    #[serde(rename = "MaxAllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_allocated_storage: Option<i32>,
    #[serde(rename = "StorageThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_throughput: Option<i32>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "VolumeName")]
    #[serde(default)]
    pub volume_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBInstanceAutomatedBackupsReplicationList {
    #[serde(
        rename = "DBInstanceAutomatedBackupsReplication",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DBInstanceAutomatedBackupsReplication>,
}
impl From<Vec<DBInstanceAutomatedBackupsReplication>>
    for DBInstanceAutomatedBackupsReplicationList
{
    fn from(v: Vec<DBInstanceAutomatedBackupsReplication>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBInstanceAutomatedBackupsReplication>
    for DBInstanceAutomatedBackupsReplicationList
{
    fn from_iter<I: IntoIterator<Item = DBInstanceAutomatedBackupsReplication>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBInstanceAutomatedBackupsReplication>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBInstanceAutomatedBackupsReplicationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBInstanceAutomatedBackupsReplication>,
}

impl From<Vec<DBInstanceAutomatedBackupsReplication>>
    for XmlDBInstanceAutomatedBackupsReplicationList
{
    fn from(v: Vec<DBInstanceAutomatedBackupsReplication>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBInstanceAutomatedBackupsReplication>
    for XmlDBInstanceAutomatedBackupsReplicationList
{
    fn from_iter<I: IntoIterator<Item = DBInstanceAutomatedBackupsReplication>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBInstanceAutomatedBackupsReplication")]
pub struct DBInstanceAutomatedBackupsReplication {
    #[serde(rename = "DBInstanceAutomatedBackupsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_automated_backups_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RestoreWindow")]
pub struct RestoreWindow {
    #[serde(rename = "EarliestTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earliest_time: Option<String>,
    #[serde(rename = "LatestTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeOrderableDBInstanceOptionsResult")]
pub struct OrderableDBInstanceOptionsMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "OrderableDBInstanceOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orderable_d_b_instance_options: Option<OrderableDBInstanceOptionsList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrderableDBInstanceOptionsList {
    #[serde(
        rename = "OrderableDBInstanceOption",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<OrderableDBInstanceOption>,
}
impl From<Vec<OrderableDBInstanceOption>> for OrderableDBInstanceOptionsList {
    fn from(v: Vec<OrderableDBInstanceOption>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<OrderableDBInstanceOption> for OrderableDBInstanceOptionsList {
    fn from_iter<I: IntoIterator<Item = OrderableDBInstanceOption>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<OrderableDBInstanceOption>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlOrderableDBInstanceOptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<OrderableDBInstanceOption>,
}

impl From<Vec<OrderableDBInstanceOption>> for XmlOrderableDBInstanceOptionList {
    fn from(v: Vec<OrderableDBInstanceOption>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<OrderableDBInstanceOption> for XmlOrderableDBInstanceOptionList {
    fn from_iter<I: IntoIterator<Item = OrderableDBInstanceOption>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OrderableDBInstanceOption")]
pub struct OrderableDBInstanceOption {
    #[serde(rename = "AvailabilityZoneGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_group: Option<String>,
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<AvailabilityZoneList>,
    #[serde(rename = "AvailableAdditionalStorageVolumesOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_additional_storage_volumes_options:
        Option<AvailableAdditionalStorageVolumesOptionList>,
    #[serde(rename = "AvailableProcessorFeatures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_processor_features: Option<AvailableProcessorFeatureList>,
    #[serde(rename = "DBInstanceClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_class: Option<String>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "LicenseModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_model: Option<String>,
    #[serde(rename = "MaxIopsPerDbInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_iops_per_db_instance: Option<i32>,
    #[serde(rename = "MaxIopsPerGib")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_iops_per_gib: Option<f64>,
    #[serde(rename = "MaxStorageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_size: Option<i32>,
    #[serde(rename = "MaxStorageThroughputPerDbInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_throughput_per_db_instance: Option<i32>,
    #[serde(rename = "MaxStorageThroughputPerIops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_throughput_per_iops: Option<f64>,
    #[serde(rename = "MinIopsPerDbInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_iops_per_db_instance: Option<i32>,
    #[serde(rename = "MinIopsPerGib")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_iops_per_gib: Option<f64>,
    #[serde(rename = "MinStorageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_storage_size: Option<i32>,
    #[serde(rename = "MinStorageThroughputPerDbInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_storage_throughput_per_db_instance: Option<i32>,
    #[serde(rename = "MinStorageThroughputPerIops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_storage_throughput_per_iops: Option<f64>,
    #[serde(rename = "MultiAZCapable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_a_z_capable: Option<bool>,
    #[serde(rename = "OutpostCapable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_capable: Option<bool>,
    #[serde(rename = "ReadReplicaCapable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_replica_capable: Option<bool>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "SupportedActivityStreamModes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_activity_stream_modes: Option<ActivityStreamModeList>,
    #[serde(rename = "SupportedEngineModes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_engine_modes: Option<EngineModeList>,
    #[serde(rename = "SupportedNetworkTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_network_types: Option<StringList>,
    #[serde(rename = "SupportsAdditionalStorageVolumes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_additional_storage_volumes: Option<bool>,
    #[serde(rename = "SupportsClusters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_clusters: Option<bool>,
    #[serde(rename = "SupportsDedicatedLogVolume")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_dedicated_log_volume: Option<bool>,
    #[serde(rename = "SupportsEnhancedMonitoring")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_enhanced_monitoring: Option<bool>,
    #[serde(rename = "SupportsGlobalDatabases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_global_databases: Option<bool>,
    #[serde(rename = "SupportsHttpEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_http_endpoint: Option<bool>,
    #[serde(rename = "SupportsIAMDatabaseAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_i_a_m_database_authentication: Option<bool>,
    #[serde(rename = "SupportsIops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_iops: Option<bool>,
    #[serde(rename = "SupportsKerberosAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_kerberos_authentication: Option<bool>,
    #[serde(rename = "SupportsPerformanceInsights")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_performance_insights: Option<bool>,
    #[serde(rename = "SupportsStorageAutoscaling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_storage_autoscaling: Option<bool>,
    #[serde(rename = "SupportsStorageEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_storage_encryption: Option<bool>,
    #[serde(rename = "SupportsStorageThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_storage_throughput: Option<bool>,
    #[serde(rename = "Vpc")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AvailabilityZoneList {
    #[serde(
        rename = "AvailabilityZone",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<AvailabilityZone>,
}
impl From<Vec<AvailabilityZone>> for AvailabilityZoneList {
    fn from(v: Vec<AvailabilityZone>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<AvailabilityZone> for AvailabilityZoneList {
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
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AvailableAdditionalStorageVolumesOptionList {
    #[serde(
        rename = "AvailableAdditionalStorageVolumesOption",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<AvailableAdditionalStorageVolumesOption>,
}
impl From<Vec<AvailableAdditionalStorageVolumesOption>>
    for AvailableAdditionalStorageVolumesOptionList
{
    fn from(v: Vec<AvailableAdditionalStorageVolumesOption>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<AvailableAdditionalStorageVolumesOption>
    for AvailableAdditionalStorageVolumesOptionList
{
    fn from_iter<I: IntoIterator<Item = AvailableAdditionalStorageVolumesOption>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<AvailableAdditionalStorageVolumesOption>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlAvailableAdditionalStorageVolumesOptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<AvailableAdditionalStorageVolumesOption>,
}

impl From<Vec<AvailableAdditionalStorageVolumesOption>>
    for XmlAvailableAdditionalStorageVolumesOptionList
{
    fn from(v: Vec<AvailableAdditionalStorageVolumesOption>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<AvailableAdditionalStorageVolumesOption>
    for XmlAvailableAdditionalStorageVolumesOptionList
{
    fn from_iter<I: IntoIterator<Item = AvailableAdditionalStorageVolumesOption>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AvailableAdditionalStorageVolumesOption")]
pub struct AvailableAdditionalStorageVolumesOption {
    #[serde(rename = "MaxIops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_iops: Option<i32>,
    #[serde(rename = "MaxIopsPerGib")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_iops_per_gib: Option<f64>,
    #[serde(rename = "MaxStorageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_size: Option<i32>,
    #[serde(rename = "MaxStorageThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_throughput: Option<i32>,
    #[serde(rename = "MinIops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_iops: Option<i32>,
    #[serde(rename = "MinIopsPerGib")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_iops_per_gib: Option<f64>,
    #[serde(rename = "MinStorageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_storage_size: Option<i32>,
    #[serde(rename = "MinStorageThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_storage_throughput: Option<i32>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "SupportsIops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_iops: Option<bool>,
    #[serde(rename = "SupportsStorageAutoscaling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_storage_autoscaling: Option<bool>,
    #[serde(rename = "SupportsStorageThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_storage_throughput: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AvailableProcessorFeatureList {
    #[serde(
        rename = "AvailableProcessorFeature",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<AvailableProcessorFeature>,
}
impl From<Vec<AvailableProcessorFeature>> for AvailableProcessorFeatureList {
    fn from(v: Vec<AvailableProcessorFeature>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<AvailableProcessorFeature> for AvailableProcessorFeatureList {
    fn from_iter<I: IntoIterator<Item = AvailableProcessorFeature>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<AvailableProcessorFeature>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlAvailableProcessorFeatureList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<AvailableProcessorFeature>,
}

impl From<Vec<AvailableProcessorFeature>> for XmlAvailableProcessorFeatureList {
    fn from(v: Vec<AvailableProcessorFeature>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<AvailableProcessorFeature> for XmlAvailableProcessorFeatureList {
    fn from_iter<I: IntoIterator<Item = AvailableProcessorFeature>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AvailableProcessorFeature")]
pub struct AvailableProcessorFeature {
    #[serde(rename = "AllowedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<String>,
    #[serde(rename = "DefaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActivityStreamModeList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ActivityStreamModeList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ActivityStreamModeList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EngineModeList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for EngineModeList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for EngineModeList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StartDBInstanceMessage")]
pub struct StartDBInstanceMessage {
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    pub d_b_instance_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SwitchoverReadReplicaResult")]
pub struct SwitchoverReadReplicaResult {
    #[serde(rename = "DBInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance: Option<DBInstance>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBInstance")]
pub struct DBInstance {
    #[serde(rename = "ActivityStreamEngineNativeAuditFieldsIncluded")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_stream_engine_native_audit_fields_included: Option<bool>,
    #[serde(rename = "ActivityStreamKinesisStreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_stream_kinesis_stream_name: Option<String>,
    #[serde(rename = "ActivityStreamKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_stream_kms_key_id: Option<String>,
    #[serde(rename = "ActivityStreamMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_stream_mode: Option<String>,
    #[serde(rename = "ActivityStreamPolicyStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_stream_policy_status: Option<String>,
    #[serde(rename = "ActivityStreamStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_stream_status: Option<String>,
    #[serde(rename = "AdditionalStorageVolumes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_storage_volumes: Option<AdditionalStorageVolumesOutputList>,
    #[serde(rename = "AllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(rename = "AssociatedRoles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_roles: Option<DBInstanceRoles>,
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    #[serde(rename = "AutomaticRestartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_restart_time: Option<String>,
    #[serde(rename = "AutomationMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_mode: Option<String>,
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "AwsBackupRecoveryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_backup_recovery_point_arn: Option<String>,
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i32>,
    #[serde(rename = "BackupTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_target: Option<String>,
    #[serde(rename = "CACertificateIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_a_certificate_identifier: Option<String>,
    #[serde(rename = "CertificateDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_details: Option<CertificateDetails>,
    #[serde(rename = "CharacterSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_set_name: Option<String>,
    #[serde(rename = "CopyTagsToSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_snapshot: Option<bool>,
    #[serde(rename = "CustomIamInstanceProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_iam_instance_profile: Option<String>,
    #[serde(rename = "CustomerOwnedIpEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_owned_ip_enabled: Option<bool>,
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_identifier: Option<String>,
    #[serde(rename = "DBInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_arn: Option<String>,
    #[serde(rename = "DBInstanceAutomatedBackupsReplications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_automated_backups_replications:
        Option<DBInstanceAutomatedBackupsReplicationList>,
    #[serde(rename = "DBInstanceClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_class: Option<String>,
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_identifier: Option<String>,
    #[serde(rename = "DBInstanceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_status: Option<String>,
    #[serde(rename = "DBName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_name: Option<String>,
    #[serde(rename = "DBParameterGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_parameter_groups: Option<DBParameterGroupStatusList>,
    #[serde(rename = "DBSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_security_groups: Option<DBSecurityGroupMembershipList>,
    #[serde(rename = "DBSubnetGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_subnet_group: Option<DBSubnetGroup>,
    #[serde(rename = "DBSystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_system_id: Option<String>,
    #[serde(rename = "DatabaseInsightsMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_insights_mode: Option<String>,
    #[serde(rename = "DbInstancePort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_port: Option<i32>,
    #[serde(rename = "DbiResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbi_resource_id: Option<String>,
    #[serde(rename = "DedicatedLogVolume")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_log_volume: Option<bool>,
    #[serde(rename = "DeletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    #[serde(rename = "DomainMemberships")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_memberships: Option<DomainMembershipList>,
    #[serde(rename = "EnabledCloudwatchLogsExports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_cloudwatch_logs_exports: Option<LogTypeList>,
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<Endpoint>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineLifecycleSupport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_lifecycle_support: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "EnhancedMonitoringResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_monitoring_resource_arn: Option<String>,
    #[serde(rename = "IAMDatabaseAuthenticationEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_a_m_database_authentication_enabled: Option<bool>,
    #[serde(rename = "InstanceCreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_create_time: Option<String>,
    #[serde(rename = "Iops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    #[serde(rename = "IsStorageConfigUpgradeAvailable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_storage_config_upgrade_available: Option<bool>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "LatestRestorableTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_restorable_time: Option<String>,
    #[serde(rename = "LicenseModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_model: Option<String>,
    #[serde(rename = "ListenerEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listener_endpoint: Option<Endpoint>,
    #[serde(rename = "MasterUserSecret")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_secret: Option<MasterUserSecret>,
    #[serde(rename = "MasterUsername")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_username: Option<String>,
    #[serde(rename = "MaxAllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_allocated_storage: Option<i32>,
    #[serde(rename = "MonitoringInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_interval: Option<i32>,
    #[serde(rename = "MonitoringRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_role_arn: Option<String>,
    #[serde(rename = "MultiAZ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_a_z: Option<bool>,
    #[serde(rename = "MultiTenant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_tenant: Option<bool>,
    #[serde(rename = "NcharCharacterSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nchar_character_set_name: Option<String>,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "OptionGroupMemberships")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_memberships: Option<OptionGroupMembershipList>,
    #[serde(rename = "PendingModifiedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_modified_values: Option<PendingModifiedValues>,
    #[serde(rename = "PercentProgress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_progress: Option<String>,
    #[serde(rename = "PerformanceInsightsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_insights_enabled: Option<bool>,
    #[serde(rename = "PerformanceInsightsKMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_insights_k_m_s_key_id: Option<String>,
    #[serde(rename = "PerformanceInsightsRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_insights_retention_period: Option<i32>,
    #[serde(rename = "PreferredBackupWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[serde(rename = "ProcessorFeatures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_features: Option<ProcessorFeatureList>,
    #[serde(rename = "PromotionTier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_tier: Option<i32>,
    #[serde(rename = "PubliclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    #[serde(rename = "ReadReplicaDBClusterIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_replica_d_b_cluster_identifiers: Option<ReadReplicaDBClusterIdentifierList>,
    #[serde(rename = "ReadReplicaDBInstanceIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_replica_d_b_instance_identifiers: Option<ReadReplicaDBInstanceIdentifierList>,
    #[serde(rename = "ReadReplicaSourceDBClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_replica_source_d_b_cluster_identifier: Option<String>,
    #[serde(rename = "ReadReplicaSourceDBInstanceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_replica_source_d_b_instance_identifier: Option<String>,
    #[serde(rename = "ReplicaMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_mode: Option<String>,
    #[serde(rename = "ResumeFullAutomationModeTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resume_full_automation_mode_time: Option<String>,
    #[serde(rename = "SecondaryAvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_availability_zone: Option<String>,
    #[serde(rename = "StatusInfos")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_infos: Option<DBInstanceStatusInfoList>,
    #[serde(rename = "StorageEncrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encrypted: Option<bool>,
    #[serde(rename = "StorageEncryptionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encryption_type: Option<String>,
    #[serde(rename = "StorageThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_throughput: Option<i32>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "StorageVolumeStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_volume_status: Option<String>,
    #[serde(rename = "TagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<TagList>,
    #[serde(rename = "TdeCredentialArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tde_credential_arn: Option<String>,
    #[serde(rename = "Timezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(rename = "UpgradeRolloutOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_rollout_order: Option<String>,
    #[serde(rename = "VpcSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_groups: Option<VpcSecurityGroupMembershipList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdditionalStorageVolumesOutputList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<AdditionalStorageVolumeOutput>,
}
impl From<Vec<AdditionalStorageVolumeOutput>> for AdditionalStorageVolumesOutputList {
    fn from(v: Vec<AdditionalStorageVolumeOutput>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<AdditionalStorageVolumeOutput> for AdditionalStorageVolumesOutputList {
    fn from_iter<I: IntoIterator<Item = AdditionalStorageVolumeOutput>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<AdditionalStorageVolumeOutput>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlAdditionalStorageVolumeOutputList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<AdditionalStorageVolumeOutput>,
}

impl From<Vec<AdditionalStorageVolumeOutput>> for XmlAdditionalStorageVolumeOutputList {
    fn from(v: Vec<AdditionalStorageVolumeOutput>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<AdditionalStorageVolumeOutput> for XmlAdditionalStorageVolumeOutputList {
    fn from_iter<I: IntoIterator<Item = AdditionalStorageVolumeOutput>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AdditionalStorageVolumeOutput")]
pub struct AdditionalStorageVolumeOutput {
    #[serde(rename = "AllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(rename = "IOPS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_o_p_s: Option<i32>,
    #[serde(rename = "MaxAllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_allocated_storage: Option<i32>,
    #[serde(rename = "StorageThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_throughput: Option<i32>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "StorageVolumeStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_volume_status: Option<String>,
    #[serde(rename = "VolumeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBInstanceRoles {
    #[serde(
        rename = "DBInstanceRole",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DBInstanceRole>,
}
impl From<Vec<DBInstanceRole>> for DBInstanceRoles {
    fn from(v: Vec<DBInstanceRole>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBInstanceRole> for DBInstanceRoles {
    fn from_iter<I: IntoIterator<Item = DBInstanceRole>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBInstanceRole>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBInstanceRoleList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBInstanceRole>,
}

impl From<Vec<DBInstanceRole>> for XmlDBInstanceRoleList {
    fn from(v: Vec<DBInstanceRole>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBInstanceRole> for XmlDBInstanceRoleList {
    fn from_iter<I: IntoIterator<Item = DBInstanceRole>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBInstanceRole")]
pub struct DBInstanceRole {
    #[serde(rename = "FeatureName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_name: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CertificateDetails")]
pub struct CertificateDetails {
    #[serde(rename = "CAIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_a_identifier: Option<String>,
    #[serde(rename = "ValidTill")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_till: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBParameterGroupStatusList {
    #[serde(
        rename = "DBParameterGroup",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DBParameterGroupStatus>,
}
impl From<Vec<DBParameterGroupStatus>> for DBParameterGroupStatusList {
    fn from(v: Vec<DBParameterGroupStatus>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBParameterGroupStatus> for DBParameterGroupStatusList {
    fn from_iter<I: IntoIterator<Item = DBParameterGroupStatus>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBParameterGroupStatus>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBParameterGroupStatusList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBParameterGroupStatus>,
}

impl From<Vec<DBParameterGroupStatus>> for XmlDBParameterGroupStatusList {
    fn from(v: Vec<DBParameterGroupStatus>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBParameterGroupStatus> for XmlDBParameterGroupStatusList {
    fn from_iter<I: IntoIterator<Item = DBParameterGroupStatus>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBParameterGroupStatus")]
pub struct DBParameterGroupStatus {
    #[serde(rename = "DBParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_parameter_group_name: Option<String>,
    #[serde(rename = "ParameterApplyStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_apply_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBSecurityGroupMembershipList {
    #[serde(
        rename = "DBSecurityGroup",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DBSecurityGroupMembership>,
}
impl From<Vec<DBSecurityGroupMembership>> for DBSecurityGroupMembershipList {
    fn from(v: Vec<DBSecurityGroupMembership>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBSecurityGroupMembership> for DBSecurityGroupMembershipList {
    fn from_iter<I: IntoIterator<Item = DBSecurityGroupMembership>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBSecurityGroupMembership>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBSecurityGroupMembershipList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBSecurityGroupMembership>,
}

impl From<Vec<DBSecurityGroupMembership>> for XmlDBSecurityGroupMembershipList {
    fn from(v: Vec<DBSecurityGroupMembership>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBSecurityGroupMembership> for XmlDBSecurityGroupMembershipList {
    fn from_iter<I: IntoIterator<Item = DBSecurityGroupMembership>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBSecurityGroupMembership")]
pub struct DBSecurityGroupMembership {
    #[serde(rename = "DBSecurityGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_security_group_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBSubnetGroup")]
pub struct DBSubnetGroup {
    #[serde(rename = "DBSubnetGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_subnet_group_arn: Option<String>,
    #[serde(rename = "DBSubnetGroupDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_subnet_group_description: Option<String>,
    #[serde(rename = "DBSubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_subnet_group_name: Option<String>,
    #[serde(rename = "SubnetGroupStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_group_status: Option<String>,
    #[serde(rename = "Subnets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<SubnetList>,
    #[serde(rename = "SupportedNetworkTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_network_types: Option<StringList>,
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
    pub subnet_outpost: Option<Outpost>,
    #[serde(rename = "SubnetStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Outpost")]
pub struct Outpost {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainMembershipList {
    #[serde(
        rename = "DomainMembership",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DomainMembership>,
}
impl From<Vec<DomainMembership>> for DomainMembershipList {
    fn from(v: Vec<DomainMembership>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DomainMembership> for DomainMembershipList {
    fn from_iter<I: IntoIterator<Item = DomainMembership>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DomainMembership>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDomainMembershipList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DomainMembership>,
}

impl From<Vec<DomainMembership>> for XmlDomainMembershipList {
    fn from(v: Vec<DomainMembership>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DomainMembership> for XmlDomainMembershipList {
    fn from_iter<I: IntoIterator<Item = DomainMembership>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DomainMembership")]
pub struct DomainMembership {
    #[serde(rename = "AuthSecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_secret_arn: Option<String>,
    #[serde(rename = "DnsIps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_ips: Option<StringList>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "FQDN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f_q_d_n: Option<String>,
    #[serde(rename = "IAMRoleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_a_m_role_name: Option<String>,
    #[serde(rename = "OU")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_u: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogTypeList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for LogTypeList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for LogTypeList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Endpoint")]
pub struct Endpoint {
    #[serde(rename = "Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "HostedZoneId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OptionGroupMembershipList {
    #[serde(
        rename = "OptionGroupMembership",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<OptionGroupMembership>,
}
impl From<Vec<OptionGroupMembership>> for OptionGroupMembershipList {
    fn from(v: Vec<OptionGroupMembership>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<OptionGroupMembership> for OptionGroupMembershipList {
    fn from_iter<I: IntoIterator<Item = OptionGroupMembership>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<OptionGroupMembership>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlOptionGroupMembershipList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<OptionGroupMembership>,
}

impl From<Vec<OptionGroupMembership>> for XmlOptionGroupMembershipList {
    fn from(v: Vec<OptionGroupMembership>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<OptionGroupMembership> for XmlOptionGroupMembershipList {
    fn from_iter<I: IntoIterator<Item = OptionGroupMembership>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OptionGroupMembership")]
pub struct OptionGroupMembership {
    #[serde(rename = "OptionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PendingModifiedValues")]
pub struct PendingModifiedValues {
    #[serde(rename = "AdditionalStorageVolumes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_storage_volumes: Option<AdditionalStorageVolumesList>,
    #[serde(rename = "AllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(rename = "AutomationMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_mode: Option<String>,
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i32>,
    #[serde(rename = "CACertificateIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_a_certificate_identifier: Option<String>,
    #[serde(rename = "DBInstanceClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_class: Option<String>,
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_identifier: Option<String>,
    #[serde(rename = "DBSubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_subnet_group_name: Option<String>,
    #[serde(rename = "DedicatedLogVolume")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_log_volume: Option<bool>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "IAMDatabaseAuthenticationEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_a_m_database_authentication_enabled: Option<bool>,
    #[serde(rename = "Iops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    #[serde(rename = "LicenseModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_model: Option<String>,
    #[serde(rename = "MasterUserPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<String>,
    #[serde(rename = "MultiAZ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_a_z: Option<bool>,
    #[serde(rename = "MultiTenant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_tenant: Option<bool>,
    #[serde(rename = "PendingCloudwatchLogsExports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_cloudwatch_logs_exports: Option<PendingCloudwatchLogsExports>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "ProcessorFeatures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_features: Option<ProcessorFeatureList>,
    #[serde(rename = "ResumeFullAutomationModeTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resume_full_automation_mode_time: Option<String>,
    #[serde(rename = "StorageThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_throughput: Option<i32>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PendingCloudwatchLogsExports")]
pub struct PendingCloudwatchLogsExports {
    #[serde(rename = "LogTypesToDisable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_types_to_disable: Option<LogTypeList>,
    #[serde(rename = "LogTypesToEnable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_types_to_enable: Option<LogTypeList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProcessorFeatureList {
    #[serde(
        rename = "ProcessorFeature",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ProcessorFeature>,
}
impl From<Vec<ProcessorFeature>> for ProcessorFeatureList {
    fn from(v: Vec<ProcessorFeature>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ProcessorFeature> for ProcessorFeatureList {
    fn from_iter<I: IntoIterator<Item = ProcessorFeature>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ProcessorFeature>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlProcessorFeatureList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ProcessorFeature>,
}

impl From<Vec<ProcessorFeature>> for XmlProcessorFeatureList {
    fn from(v: Vec<ProcessorFeature>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ProcessorFeature> for XmlProcessorFeatureList {
    fn from_iter<I: IntoIterator<Item = ProcessorFeature>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ProcessorFeature")]
pub struct ProcessorFeature {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReadReplicaDBClusterIdentifierList {
    #[serde(
        rename = "ReadReplicaDBClusterIdentifier",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ReadReplicaDBClusterIdentifierList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ReadReplicaDBClusterIdentifierList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReadReplicaDBInstanceIdentifierList {
    #[serde(
        rename = "ReadReplicaDBInstanceIdentifier",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ReadReplicaDBInstanceIdentifierList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ReadReplicaDBInstanceIdentifierList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBInstanceStatusInfoList {
    #[serde(
        rename = "DBInstanceStatusInfo",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DBInstanceStatusInfo>,
}
impl From<Vec<DBInstanceStatusInfo>> for DBInstanceStatusInfoList {
    fn from(v: Vec<DBInstanceStatusInfo>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBInstanceStatusInfo> for DBInstanceStatusInfoList {
    fn from_iter<I: IntoIterator<Item = DBInstanceStatusInfo>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBInstanceStatusInfo>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBInstanceStatusInfoList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBInstanceStatusInfo>,
}

impl From<Vec<DBInstanceStatusInfo>> for XmlDBInstanceStatusInfoList {
    fn from(v: Vec<DBInstanceStatusInfo>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBInstanceStatusInfo> for XmlDBInstanceStatusInfoList {
    fn from_iter<I: IntoIterator<Item = DBInstanceStatusInfo>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBInstanceStatusInfo")]
pub struct DBInstanceStatusInfo {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Normal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal: Option<bool>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcSecurityGroupMembershipList {
    #[serde(
        rename = "VpcSecurityGroupMembership",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<VpcSecurityGroupMembership>,
}
impl From<Vec<VpcSecurityGroupMembership>> for VpcSecurityGroupMembershipList {
    fn from(v: Vec<VpcSecurityGroupMembership>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<VpcSecurityGroupMembership> for VpcSecurityGroupMembershipList {
    fn from_iter<I: IntoIterator<Item = VpcSecurityGroupMembership>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<VpcSecurityGroupMembership>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlVpcSecurityGroupMembershipList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<VpcSecurityGroupMembership>,
}

impl From<Vec<VpcSecurityGroupMembership>> for XmlVpcSecurityGroupMembershipList {
    fn from(v: Vec<VpcSecurityGroupMembership>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<VpcSecurityGroupMembership> for XmlVpcSecurityGroupMembershipList {
    fn from_iter<I: IntoIterator<Item = VpcSecurityGroupMembership>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "VpcSecurityGroupMembership")]
pub struct VpcSecurityGroupMembership {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "VpcSecurityGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBClusterParametersMessage")]
pub struct DescribeDBClusterParametersMessage {
    #[serde(rename = "DBClusterParameterGroupName")]
    #[serde(default)]
    pub d_b_cluster_parameter_group_name: String,
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
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeCertificatesMessage")]
pub struct DescribeCertificatesMessage {
    #[serde(rename = "CertificateIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_identifier: Option<String>,
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
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyDBProxyResult")]
pub struct ModifyDBProxyResponse {
    #[serde(rename = "DBProxy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_proxy: Option<DBProxy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBProxy")]
pub struct DBProxy {
    #[serde(rename = "Auth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<UserAuthConfigInfoList>,
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(rename = "DBProxyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_proxy_arn: Option<String>,
    #[serde(rename = "DBProxyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_proxy_name: Option<String>,
    #[serde(rename = "DebugLogging")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_logging: Option<bool>,
    #[serde(rename = "DefaultAuthScheme")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_auth_scheme: Option<String>,
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "EndpointNetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_network_type: Option<String>,
    #[serde(rename = "EngineFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_family: Option<String>,
    #[serde(rename = "IdleClientTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_client_timeout: Option<i32>,
    #[serde(rename = "RequireTLS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_t_l_s: Option<bool>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TargetConnectionNetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_connection_network_type: Option<String>,
    #[serde(rename = "UpdatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<StringList>,
    #[serde(rename = "VpcSubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_subnet_ids: Option<StringList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserAuthConfigInfoList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<UserAuthConfigInfo>,
}
impl From<Vec<UserAuthConfigInfo>> for UserAuthConfigInfoList {
    fn from(v: Vec<UserAuthConfigInfo>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<UserAuthConfigInfo> for UserAuthConfigInfoList {
    fn from_iter<I: IntoIterator<Item = UserAuthConfigInfo>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<UserAuthConfigInfo>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlUserAuthConfigInfoList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<UserAuthConfigInfo>,
}

impl From<Vec<UserAuthConfigInfo>> for XmlUserAuthConfigInfoList {
    fn from(v: Vec<UserAuthConfigInfo>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<UserAuthConfigInfo> for XmlUserAuthConfigInfoList {
    fn from_iter<I: IntoIterator<Item = UserAuthConfigInfo>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UserAuthConfigInfo")]
pub struct UserAuthConfigInfo {
    #[serde(rename = "AuthScheme")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_scheme: Option<String>,
    #[serde(rename = "ClientPasswordAuthType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_password_auth_type: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "IAMAuth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_a_m_auth: Option<String>,
    #[serde(rename = "SecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBSnapshotTenantDatabasesMessage")]
pub struct DescribeDBSnapshotTenantDatabasesMessage {
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_identifier: Option<String>,
    #[serde(rename = "DBSnapshotIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_snapshot_identifier: Option<String>,
    #[serde(rename = "DbiResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbi_resource_id: Option<String>,
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
    #[serde(rename = "SnapshotType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RemoveRoleFromDBClusterMessage")]
pub struct RemoveRoleFromDBClusterMessage {
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    pub d_b_cluster_identifier: String,
    #[serde(rename = "FeatureName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_name: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResetDBParameterGroupMessage")]
pub struct ResetDBParameterGroupMessage {
    #[serde(rename = "DBParameterGroupName")]
    #[serde(default)]
    pub d_b_parameter_group_name: String,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParametersList>,
    #[serde(rename = "ResetAllParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_all_parameters: Option<bool>,
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
    #[serde(rename = "ApplyMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_method: Option<String>,
    #[serde(rename = "ApplyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_type: Option<String>,
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
    #[serde(rename = "SupportedEngineModes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_engine_modes: Option<EngineModeList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBSnapshotsResult")]
pub struct DBSnapshotMessage {
    #[serde(rename = "DBSnapshots")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_snapshots: Option<DBSnapshotList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBSnapshotList {
    #[serde(rename = "DBSnapshot", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<DBSnapshot>,
}
impl From<Vec<DBSnapshot>> for DBSnapshotList {
    fn from(v: Vec<DBSnapshot>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBSnapshot> for DBSnapshotList {
    fn from_iter<I: IntoIterator<Item = DBSnapshot>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBSnapshot>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBSnapshotList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBSnapshot>,
}

impl From<Vec<DBSnapshot>> for XmlDBSnapshotList {
    fn from(v: Vec<DBSnapshot>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBSnapshot> for XmlDBSnapshotList {
    fn from_iter<I: IntoIterator<Item = DBSnapshot>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBSnapshot")]
pub struct DBSnapshot {
    #[serde(rename = "AdditionalStorageVolumes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_storage_volumes: Option<AdditionalStorageVolumesList>,
    #[serde(rename = "AllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i32>,
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_identifier: Option<String>,
    #[serde(rename = "DBSnapshotArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_snapshot_arn: Option<String>,
    #[serde(rename = "DBSnapshotIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_snapshot_identifier: Option<String>,
    #[serde(rename = "DBSystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_system_id: Option<String>,
    #[serde(rename = "DbiResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbi_resource_id: Option<String>,
    #[serde(rename = "DedicatedLogVolume")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_log_volume: Option<bool>,
    #[serde(rename = "Encrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "IAMDatabaseAuthenticationEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_a_m_database_authentication_enabled: Option<bool>,
    #[serde(rename = "InstanceCreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_create_time: Option<String>,
    #[serde(rename = "Iops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "LicenseModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_model: Option<String>,
    #[serde(rename = "MasterUsername")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_username: Option<String>,
    #[serde(rename = "MultiTenant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_tenant: Option<bool>,
    #[serde(rename = "OptionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_name: Option<String>,
    #[serde(rename = "OriginalSnapshotCreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_snapshot_create_time: Option<String>,
    #[serde(rename = "PercentProgress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_progress: Option<i32>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "PreferredBackupWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,
    #[serde(rename = "ProcessorFeatures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_features: Option<ProcessorFeatureList>,
    #[serde(rename = "SnapshotAvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_availability_zone: Option<String>,
    #[serde(rename = "SnapshotCreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_create_time: Option<String>,
    #[serde(rename = "SnapshotDatabaseTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_database_time: Option<String>,
    #[serde(rename = "SnapshotTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_target: Option<String>,
    #[serde(rename = "SnapshotType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_type: Option<String>,
    #[serde(rename = "SourceDBSnapshotIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_d_b_snapshot_identifier: Option<String>,
    #[serde(rename = "SourceRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_region: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StorageEncryptionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encryption_type: Option<String>,
    #[serde(rename = "StorageThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_throughput: Option<i32>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "TagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<TagList>,
    #[serde(rename = "TdeCredentialArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tde_credential_arn: Option<String>,
    #[serde(rename = "Timezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyTenantDatabaseMessage")]
pub struct ModifyTenantDatabaseMessage {
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    pub d_b_instance_identifier: String,
    #[serde(rename = "ManageMasterUserPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manage_master_user_password: Option<bool>,
    #[serde(rename = "MasterUserPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<String>,
    #[serde(rename = "MasterUserSecretKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_secret_kms_key_id: Option<String>,
    #[serde(rename = "NewTenantDBName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_tenant_d_b_name: Option<String>,
    #[serde(rename = "RotateMasterUserPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate_master_user_password: Option<bool>,
    #[serde(rename = "TenantDBName")]
    #[serde(default)]
    pub tenant_d_b_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RemoveRoleFromDBInstanceMessage")]
pub struct RemoveRoleFromDBInstanceMessage {
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    pub d_b_instance_identifier: String,
    #[serde(rename = "FeatureName")]
    #[serde(default)]
    pub feature_name: String,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteIntegrationMessage")]
pub struct DeleteIntegrationMessage {
    #[serde(rename = "IntegrationIdentifier")]
    #[serde(default)]
    pub integration_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyDBSnapshotAttributeMessage")]
pub struct ModifyDBSnapshotAttributeMessage {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    pub attribute_name: String,
    #[serde(rename = "DBSnapshotIdentifier")]
    #[serde(default)]
    pub d_b_snapshot_identifier: String,
    #[serde(rename = "ValuesToAdd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values_to_add: Option<AttributeValueList>,
    #[serde(rename = "ValuesToRemove")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values_to_remove: Option<AttributeValueList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttributeValueList {
    #[serde(
        rename = "AttributeValue",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for AttributeValueList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for AttributeValueList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CopyDBSnapshotMessage")]
pub struct CopyDBSnapshotMessage {
    #[serde(rename = "CopyOptionGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_option_group: Option<bool>,
    #[serde(rename = "CopyTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags: Option<bool>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "OptionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_name: Option<String>,
    #[serde(rename = "PreSignedUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_signed_url: Option<String>,
    #[serde(rename = "SnapshotAvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_availability_zone: Option<String>,
    #[serde(rename = "SnapshotTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_target: Option<String>,
    #[serde(rename = "SourceDBSnapshotIdentifier")]
    #[serde(default)]
    pub source_d_b_snapshot_identifier: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "TargetCustomAvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_custom_availability_zone: Option<String>,
    #[serde(rename = "TargetDBSnapshotIdentifier")]
    #[serde(default)]
    pub target_d_b_snapshot_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AuthorizeDBSecurityGroupIngressResult")]
pub struct AuthorizeDBSecurityGroupIngressResult {
    #[serde(rename = "DBSecurityGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_security_group: Option<DBSecurityGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBSecurityGroup")]
pub struct DBSecurityGroup {
    #[serde(rename = "DBSecurityGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_security_group_arn: Option<String>,
    #[serde(rename = "DBSecurityGroupDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_security_group_description: Option<String>,
    #[serde(rename = "DBSecurityGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_security_group_name: Option<String>,
    #[serde(rename = "EC2SecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_c2_security_groups: Option<EC2SecurityGroupList>,
    #[serde(rename = "IPRanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_p_ranges: Option<IPRangeList>,
    #[serde(rename = "OwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
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
    #[serde(rename = "EC2SecurityGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_c2_security_group_id: Option<String>,
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
pub struct IPRangeList {
    #[serde(rename = "IPRange", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<IPRange>,
}
impl From<Vec<IPRange>> for IPRangeList {
    fn from(v: Vec<IPRange>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<IPRange> for IPRangeList {
    fn from_iter<I: IntoIterator<Item = IPRange>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<IPRange>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlIPRangeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<IPRange>,
}

impl From<Vec<IPRange>> for XmlIPRangeList {
    fn from(v: Vec<IPRange>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<IPRange> for XmlIPRangeList {
    fn from_iter<I: IntoIterator<Item = IPRange>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "IPRange")]
pub struct IPRange {
    #[serde(rename = "CIDRIP")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_i_d_r_i_p: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteDBSecurityGroupMessage")]
pub struct DeleteDBSecurityGroupMessage {
    #[serde(rename = "DBSecurityGroupName")]
    #[serde(default)]
    pub d_b_security_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBParametersMessage")]
pub struct DescribeDBParametersMessage {
    #[serde(rename = "DBParameterGroupName")]
    #[serde(default)]
    pub d_b_parameter_group_name: String,
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
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "FailoverGlobalClusterResult")]
pub struct FailoverGlobalClusterResult {
    #[serde(rename = "GlobalCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_cluster: Option<GlobalCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GlobalCluster")]
pub struct GlobalCluster {
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "DeletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineLifecycleSupport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_lifecycle_support: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "FailoverState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failover_state: Option<FailoverState>,
    #[serde(rename = "GlobalClusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_cluster_arn: Option<String>,
    #[serde(rename = "GlobalClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_cluster_identifier: Option<String>,
    #[serde(rename = "GlobalClusterMembers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_cluster_members: Option<GlobalClusterMemberList>,
    #[serde(rename = "GlobalClusterResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_cluster_resource_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StorageEncrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encrypted: Option<bool>,
    #[serde(rename = "StorageEncryptionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encryption_type: Option<String>,
    #[serde(rename = "TagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "FailoverState")]
pub struct FailoverState {
    #[serde(rename = "FromDbClusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_db_cluster_arn: Option<String>,
    #[serde(rename = "IsDataLossAllowed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_data_loss_allowed: Option<bool>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "ToDbClusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_db_cluster_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlobalClusterMemberList {
    #[serde(
        rename = "GlobalClusterMember",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<GlobalClusterMember>,
}
impl From<Vec<GlobalClusterMember>> for GlobalClusterMemberList {
    fn from(v: Vec<GlobalClusterMember>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<GlobalClusterMember> for GlobalClusterMemberList {
    fn from_iter<I: IntoIterator<Item = GlobalClusterMember>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<GlobalClusterMember>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlGlobalClusterMemberList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<GlobalClusterMember>,
}

impl From<Vec<GlobalClusterMember>> for XmlGlobalClusterMemberList {
    fn from(v: Vec<GlobalClusterMember>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<GlobalClusterMember> for XmlGlobalClusterMemberList {
    fn from_iter<I: IntoIterator<Item = GlobalClusterMember>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GlobalClusterMember")]
pub struct GlobalClusterMember {
    #[serde(rename = "DBClusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_arn: Option<String>,
    #[serde(rename = "GlobalWriteForwardingStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_write_forwarding_status: Option<String>,
    #[serde(rename = "IsWriter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_writer: Option<bool>,
    #[serde(rename = "Readers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readers: Option<ReadersArnList>,
    #[serde(rename = "SynchronizationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synchronization_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReadersArnList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ReadersArnList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ReadersArnList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyDBInstanceResult")]
pub struct ModifyDBInstanceResult {
    #[serde(rename = "DBInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance: Option<DBInstance>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEngineDefaultClusterParametersResult")]
pub struct DescribeEngineDefaultClusterParametersResult {
    #[serde(rename = "EngineDefaults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_defaults: Option<EngineDefaults>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EngineDefaults")]
pub struct EngineDefaults {
    #[serde(rename = "DBParameterGroupFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_parameter_group_family: Option<String>,
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
#[serde(rename = "RegisterDBProxyTargetsResult")]
pub struct RegisterDBProxyTargetsResponse {
    #[serde(rename = "DBProxyTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_proxy_targets: Option<TargetList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<DBProxyTarget>,
}
impl From<Vec<DBProxyTarget>> for TargetList {
    fn from(v: Vec<DBProxyTarget>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBProxyTarget> for TargetList {
    fn from_iter<I: IntoIterator<Item = DBProxyTarget>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBProxyTarget>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBProxyTargetList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBProxyTarget>,
}

impl From<Vec<DBProxyTarget>> for XmlDBProxyTargetList {
    fn from(v: Vec<DBProxyTarget>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBProxyTarget> for XmlDBProxyTargetList {
    fn from_iter<I: IntoIterator<Item = DBProxyTarget>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBProxyTarget")]
pub struct DBProxyTarget {
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "RdsResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_resource_id: Option<String>,
    #[serde(rename = "Role")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "TargetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
    #[serde(rename = "TargetHealth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_health: Option<TargetHealth>,
    #[serde(rename = "TrackedClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracked_cluster_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
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
#[serde(rename = "FailoverDBClusterResult")]
pub struct FailoverDBClusterResult {
    #[serde(rename = "DBCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster: Option<DBCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBCluster")]
pub struct DBCluster {
    #[serde(rename = "ActivityStreamKinesisStreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_stream_kinesis_stream_name: Option<String>,
    #[serde(rename = "ActivityStreamKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_stream_kms_key_id: Option<String>,
    #[serde(rename = "ActivityStreamMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_stream_mode: Option<String>,
    #[serde(rename = "ActivityStreamStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_stream_status: Option<String>,
    #[serde(rename = "AllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(rename = "AssociatedRoles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_roles: Option<DBClusterRoles>,
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    #[serde(rename = "AutomaticRestartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_restart_time: Option<String>,
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<AvailabilityZones>,
    #[serde(rename = "AwsBackupRecoveryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_backup_recovery_point_arn: Option<String>,
    #[serde(rename = "BacktrackConsumedChangeRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backtrack_consumed_change_records: Option<i64>,
    #[serde(rename = "BacktrackWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backtrack_window: Option<i64>,
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i32>,
    #[serde(rename = "Capacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    #[serde(rename = "CertificateDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_details: Option<CertificateDetails>,
    #[serde(rename = "CharacterSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_set_name: Option<String>,
    #[serde(rename = "CloneGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clone_group_id: Option<String>,
    #[serde(rename = "ClusterCreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_create_time: Option<String>,
    #[serde(rename = "ClusterScalabilityType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_scalability_type: Option<String>,
    #[serde(rename = "CopyTagsToSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_snapshot: Option<bool>,
    #[serde(rename = "CrossAccountClone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_account_clone: Option<bool>,
    #[serde(rename = "CustomEndpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_endpoints: Option<StringList>,
    #[serde(rename = "DBClusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_arn: Option<String>,
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_identifier: Option<String>,
    #[serde(rename = "DBClusterInstanceClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_instance_class: Option<String>,
    #[serde(rename = "DBClusterMembers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_members: Option<DBClusterMemberList>,
    #[serde(rename = "DBClusterOptionGroupMemberships")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_option_group_memberships: Option<DBClusterOptionGroupMemberships>,
    #[serde(rename = "DBClusterParameterGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_parameter_group: Option<String>,
    #[serde(rename = "DBSubnetGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_subnet_group: Option<String>,
    #[serde(rename = "DBSystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_system_id: Option<String>,
    #[serde(rename = "DatabaseInsightsMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_insights_mode: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "DbClusterResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_resource_id: Option<String>,
    #[serde(rename = "DeletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    #[serde(rename = "DomainMemberships")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_memberships: Option<DomainMembershipList>,
    #[serde(rename = "EarliestBacktrackTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earliest_backtrack_time: Option<String>,
    #[serde(rename = "EarliestRestorableTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earliest_restorable_time: Option<String>,
    #[serde(rename = "EnabledCloudwatchLogsExports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_cloudwatch_logs_exports: Option<LogTypeList>,
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineLifecycleSupport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_lifecycle_support: Option<String>,
    #[serde(rename = "EngineMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_mode: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "GlobalClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_cluster_identifier: Option<String>,
    #[serde(rename = "GlobalWriteForwardingRequested")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_write_forwarding_requested: Option<bool>,
    #[serde(rename = "GlobalWriteForwardingStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_write_forwarding_status: Option<String>,
    #[serde(rename = "HostedZoneId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id: Option<String>,
    #[serde(rename = "HttpEndpointEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_endpoint_enabled: Option<bool>,
    #[serde(rename = "IAMDatabaseAuthenticationEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_a_m_database_authentication_enabled: Option<bool>,
    #[serde(rename = "IOOptimizedNextAllowedModificationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_o_optimized_next_allowed_modification_time: Option<String>,
    #[serde(rename = "Iops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "LatestRestorableTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_restorable_time: Option<String>,
    #[serde(rename = "LimitlessDatabase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limitless_database: Option<LimitlessDatabase>,
    #[serde(rename = "LocalWriteForwardingStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_write_forwarding_status: Option<String>,
    #[serde(rename = "MasterUserSecret")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_secret: Option<MasterUserSecret>,
    #[serde(rename = "MasterUsername")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_username: Option<String>,
    #[serde(rename = "MonitoringInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_interval: Option<i32>,
    #[serde(rename = "MonitoringRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_role_arn: Option<String>,
    #[serde(rename = "MultiAZ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_a_z: Option<bool>,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "PendingModifiedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_modified_values: Option<ClusterPendingModifiedValues>,
    #[serde(rename = "PercentProgress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_progress: Option<String>,
    #[serde(rename = "PerformanceInsightsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_insights_enabled: Option<bool>,
    #[serde(rename = "PerformanceInsightsKMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_insights_k_m_s_key_id: Option<String>,
    #[serde(rename = "PerformanceInsightsRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_insights_retention_period: Option<i32>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "PreferredBackupWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[serde(rename = "PubliclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    #[serde(rename = "RdsCustomClusterConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_custom_cluster_configuration: Option<RdsCustomClusterConfiguration>,
    #[serde(rename = "ReadReplicaIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_replica_identifiers: Option<ReadReplicaIdentifierList>,
    #[serde(rename = "ReaderEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reader_endpoint: Option<String>,
    #[serde(rename = "ReplicationSourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_source_identifier: Option<String>,
    #[serde(rename = "ScalingConfigurationInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_configuration_info: Option<ScalingConfigurationInfo>,
    #[serde(rename = "ServerlessV2PlatformVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless_v2_platform_version: Option<String>,
    #[serde(rename = "ServerlessV2ScalingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless_v2_scaling_configuration: Option<ServerlessV2ScalingConfigurationInfo>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusInfos")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_infos: Option<DBClusterStatusInfoList>,
    #[serde(rename = "StorageEncrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encrypted: Option<bool>,
    #[serde(rename = "StorageEncryptionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encryption_type: Option<String>,
    #[serde(rename = "StorageThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_throughput: Option<i32>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "TagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<TagList>,
    #[serde(rename = "UpgradeRolloutOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_rollout_order: Option<String>,
    #[serde(rename = "VpcSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_groups: Option<VpcSecurityGroupMembershipList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBClusterRoles {
    #[serde(
        rename = "DBClusterRole",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DBClusterRole>,
}
impl From<Vec<DBClusterRole>> for DBClusterRoles {
    fn from(v: Vec<DBClusterRole>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBClusterRole> for DBClusterRoles {
    fn from_iter<I: IntoIterator<Item = DBClusterRole>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBClusterRole>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBClusterRoleList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBClusterRole>,
}

impl From<Vec<DBClusterRole>> for XmlDBClusterRoleList {
    fn from(v: Vec<DBClusterRole>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBClusterRole> for XmlDBClusterRoleList {
    fn from_iter<I: IntoIterator<Item = DBClusterRole>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBClusterRole")]
pub struct DBClusterRole {
    #[serde(rename = "FeatureName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_name: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AvailabilityZones {
    #[serde(
        rename = "AvailabilityZone",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
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

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBClusterMemberList {
    #[serde(
        rename = "DBClusterMember",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DBClusterMember>,
}
impl From<Vec<DBClusterMember>> for DBClusterMemberList {
    fn from(v: Vec<DBClusterMember>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBClusterMember> for DBClusterMemberList {
    fn from_iter<I: IntoIterator<Item = DBClusterMember>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBClusterMember>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBClusterMemberList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBClusterMember>,
}

impl From<Vec<DBClusterMember>> for XmlDBClusterMemberList {
    fn from(v: Vec<DBClusterMember>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBClusterMember> for XmlDBClusterMemberList {
    fn from_iter<I: IntoIterator<Item = DBClusterMember>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBClusterMember")]
pub struct DBClusterMember {
    #[serde(rename = "DBClusterParameterGroupStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_parameter_group_status: Option<String>,
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_identifier: Option<String>,
    #[serde(rename = "IsClusterWriter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_cluster_writer: Option<bool>,
    #[serde(rename = "PromotionTier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_tier: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBClusterOptionGroupMemberships {
    #[serde(
        rename = "DBClusterOptionGroup",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DBClusterOptionGroupStatus>,
}
impl From<Vec<DBClusterOptionGroupStatus>> for DBClusterOptionGroupMemberships {
    fn from(v: Vec<DBClusterOptionGroupStatus>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBClusterOptionGroupStatus> for DBClusterOptionGroupMemberships {
    fn from_iter<I: IntoIterator<Item = DBClusterOptionGroupStatus>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBClusterOptionGroupStatus>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBClusterOptionGroupStatusList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBClusterOptionGroupStatus>,
}

impl From<Vec<DBClusterOptionGroupStatus>> for XmlDBClusterOptionGroupStatusList {
    fn from(v: Vec<DBClusterOptionGroupStatus>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBClusterOptionGroupStatus> for XmlDBClusterOptionGroupStatusList {
    fn from_iter<I: IntoIterator<Item = DBClusterOptionGroupStatus>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBClusterOptionGroupStatus")]
pub struct DBClusterOptionGroupStatus {
    #[serde(rename = "DBClusterOptionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_option_group_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LimitlessDatabase")]
pub struct LimitlessDatabase {
    #[serde(rename = "MinRequiredACU")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_required_a_c_u: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ClusterPendingModifiedValues")]
pub struct ClusterPendingModifiedValues {
    #[serde(rename = "AllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i32>,
    #[serde(rename = "CertificateDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_details: Option<CertificateDetails>,
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_identifier: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "IAMDatabaseAuthenticationEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_a_m_database_authentication_enabled: Option<bool>,
    #[serde(rename = "Iops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    #[serde(rename = "MasterUserPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<String>,
    #[serde(rename = "PendingCloudwatchLogsExports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_cloudwatch_logs_exports: Option<PendingCloudwatchLogsExports>,
    #[serde(rename = "RdsCustomClusterConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_custom_cluster_configuration: Option<RdsCustomClusterConfiguration>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RdsCustomClusterConfiguration")]
pub struct RdsCustomClusterConfiguration {
    #[serde(rename = "InterconnectSubnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interconnect_subnet_id: Option<String>,
    #[serde(rename = "ReplicaMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_mode: Option<String>,
    #[serde(rename = "TransitGatewayMulticastDomainId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_gateway_multicast_domain_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReadReplicaIdentifierList {
    #[serde(
        rename = "ReadReplicaIdentifier",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ReadReplicaIdentifierList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ReadReplicaIdentifierList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ScalingConfigurationInfo")]
pub struct ScalingConfigurationInfo {
    #[serde(rename = "AutoPause")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_pause: Option<bool>,
    #[serde(rename = "MaxCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<i32>,
    #[serde(rename = "MinCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_capacity: Option<i32>,
    #[serde(rename = "SecondsBeforeTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds_before_timeout: Option<i32>,
    #[serde(rename = "SecondsUntilAutoPause")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds_until_auto_pause: Option<i32>,
    #[serde(rename = "TimeoutAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_action: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ServerlessV2ScalingConfigurationInfo")]
pub struct ServerlessV2ScalingConfigurationInfo {
    #[serde(rename = "MaxCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<f64>,
    #[serde(rename = "MinCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_capacity: Option<f64>,
    #[serde(rename = "SecondsUntilAutoPause")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds_until_auto_pause: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBClusterStatusInfoList {
    #[serde(
        rename = "DBClusterStatusInfo",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DBClusterStatusInfo>,
}
impl From<Vec<DBClusterStatusInfo>> for DBClusterStatusInfoList {
    fn from(v: Vec<DBClusterStatusInfo>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBClusterStatusInfo> for DBClusterStatusInfoList {
    fn from_iter<I: IntoIterator<Item = DBClusterStatusInfo>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBClusterStatusInfo>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBClusterStatusInfoList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBClusterStatusInfo>,
}

impl From<Vec<DBClusterStatusInfo>> for XmlDBClusterStatusInfoList {
    fn from(v: Vec<DBClusterStatusInfo>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBClusterStatusInfo> for XmlDBClusterStatusInfoList {
    fn from_iter<I: IntoIterator<Item = DBClusterStatusInfo>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBClusterStatusInfo")]
pub struct DBClusterStatusInfo {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Normal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal: Option<bool>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyDBProxyEndpointRequest")]
pub struct ModifyDBProxyEndpointRequest {
    #[serde(rename = "DBProxyEndpointName")]
    #[serde(default)]
    pub d_b_proxy_endpoint_name: String,
    #[serde(rename = "NewDBProxyEndpointName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_d_b_proxy_endpoint_name: Option<String>,
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<StringList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RestoreDBClusterToPointInTimeMessage")]
pub struct RestoreDBClusterToPointInTimeMessage {
    #[serde(rename = "BacktrackWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backtrack_window: Option<i64>,
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i32>,
    #[serde(rename = "CopyTagsToSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_snapshot: Option<bool>,
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    pub d_b_cluster_identifier: String,
    #[serde(rename = "DBClusterInstanceClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_instance_class: Option<String>,
    #[serde(rename = "DBClusterParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_parameter_group_name: Option<String>,
    #[serde(rename = "DBSubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_subnet_group_name: Option<String>,
    #[serde(rename = "DeletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "DomainIAMRoleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_i_a_m_role_name: Option<String>,
    #[serde(rename = "EnableCloudwatchLogsExports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_cloudwatch_logs_exports: Option<LogTypeList>,
    #[serde(rename = "EnableIAMDatabaseAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_i_a_m_database_authentication: Option<bool>,
    #[serde(rename = "EnablePerformanceInsights")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_performance_insights: Option<bool>,
    #[serde(rename = "EngineLifecycleSupport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_lifecycle_support: Option<String>,
    #[serde(rename = "EngineMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_mode: Option<String>,
    #[serde(rename = "Iops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "MonitoringInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_interval: Option<i32>,
    #[serde(rename = "MonitoringRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_role_arn: Option<String>,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "OptionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_name: Option<String>,
    #[serde(rename = "PerformanceInsightsKMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_insights_k_m_s_key_id: Option<String>,
    #[serde(rename = "PerformanceInsightsRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_insights_retention_period: Option<i32>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "PreferredBackupWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,
    #[serde(rename = "PubliclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    #[serde(rename = "RdsCustomClusterConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_custom_cluster_configuration: Option<RdsCustomClusterConfiguration>,
    #[serde(rename = "RestoreToTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_to_time: Option<String>,
    #[serde(rename = "RestoreType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_type: Option<String>,
    #[serde(rename = "ScalingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_configuration: Option<ScalingConfiguration>,
    #[serde(rename = "ServerlessV2ScalingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless_v2_scaling_configuration: Option<ServerlessV2ScalingConfiguration>,
    #[serde(rename = "SourceDBClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_d_b_cluster_identifier: Option<String>,
    #[serde(rename = "SourceDbClusterResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_db_cluster_resource_id: Option<String>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "TagSpecifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_specifications: Option<TagSpecificationList>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "UseLatestRestorableTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_latest_restorable_time: Option<bool>,
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<VpcSecurityGroupIdList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ScalingConfiguration")]
pub struct ScalingConfiguration {
    #[serde(rename = "AutoPause")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_pause: Option<bool>,
    #[serde(rename = "MaxCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<i32>,
    #[serde(rename = "MinCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_capacity: Option<i32>,
    #[serde(rename = "SecondsBeforeTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds_before_timeout: Option<i32>,
    #[serde(rename = "SecondsUntilAutoPause")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds_until_auto_pause: Option<i32>,
    #[serde(rename = "TimeoutAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_action: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ServerlessV2ScalingConfiguration")]
pub struct ServerlessV2ScalingConfiguration {
    #[serde(rename = "MaxCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<f64>,
    #[serde(rename = "MinCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_capacity: Option<f64>,
    #[serde(rename = "SecondsUntilAutoPause")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds_until_auto_pause: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagSpecificationList {
    #[serde(rename = "item", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<TagSpecification>,
}
impl From<Vec<TagSpecification>> for TagSpecificationList {
    fn from(v: Vec<TagSpecification>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<TagSpecification> for TagSpecificationList {
    fn from_iter<I: IntoIterator<Item = TagSpecification>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<TagSpecification>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTagSpecificationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<TagSpecification>,
}

impl From<Vec<TagSpecification>> for XmlTagSpecificationList {
    fn from(v: Vec<TagSpecification>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<TagSpecification> for XmlTagSpecificationList {
    fn from_iter<I: IntoIterator<Item = TagSpecification>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TagSpecification")]
pub struct TagSpecification {
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcSecurityGroupIdList {
    #[serde(
        rename = "VpcSecurityGroupId",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for VpcSecurityGroupIdList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for VpcSecurityGroupIdList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SwitchoverBlueGreenDeploymentResult")]
pub struct SwitchoverBlueGreenDeploymentResponse {
    #[serde(rename = "BlueGreenDeployment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue_green_deployment: Option<BlueGreenDeployment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "BlueGreenDeployment")]
pub struct BlueGreenDeployment {
    #[serde(rename = "BlueGreenDeploymentIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue_green_deployment_identifier: Option<String>,
    #[serde(rename = "BlueGreenDeploymentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue_green_deployment_name: Option<String>,
    #[serde(rename = "CreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(rename = "DeleteTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_time: Option<String>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<String>,
    #[serde(rename = "SwitchoverDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switchover_details: Option<SwitchoverDetailList>,
    #[serde(rename = "TagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<TagList>,
    #[serde(rename = "Target")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(rename = "Tasks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<BlueGreenDeploymentTaskList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SwitchoverDetailList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<SwitchoverDetail>,
}
impl From<Vec<SwitchoverDetail>> for SwitchoverDetailList {
    fn from(v: Vec<SwitchoverDetail>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<SwitchoverDetail> for SwitchoverDetailList {
    fn from_iter<I: IntoIterator<Item = SwitchoverDetail>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<SwitchoverDetail>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlSwitchoverDetailList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<SwitchoverDetail>,
}

impl From<Vec<SwitchoverDetail>> for XmlSwitchoverDetailList {
    fn from(v: Vec<SwitchoverDetail>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<SwitchoverDetail> for XmlSwitchoverDetailList {
    fn from_iter<I: IntoIterator<Item = SwitchoverDetail>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SwitchoverDetail")]
pub struct SwitchoverDetail {
    #[serde(rename = "SourceMember")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_member: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TargetMember")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_member: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BlueGreenDeploymentTaskList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<BlueGreenDeploymentTask>,
}
impl From<Vec<BlueGreenDeploymentTask>> for BlueGreenDeploymentTaskList {
    fn from(v: Vec<BlueGreenDeploymentTask>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<BlueGreenDeploymentTask> for BlueGreenDeploymentTaskList {
    fn from_iter<I: IntoIterator<Item = BlueGreenDeploymentTask>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<BlueGreenDeploymentTask>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlBlueGreenDeploymentTaskList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<BlueGreenDeploymentTask>,
}

impl From<Vec<BlueGreenDeploymentTask>> for XmlBlueGreenDeploymentTaskList {
    fn from(v: Vec<BlueGreenDeploymentTask>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<BlueGreenDeploymentTask> for XmlBlueGreenDeploymentTaskList {
    fn from_iter<I: IntoIterator<Item = BlueGreenDeploymentTask>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "BlueGreenDeploymentTask")]
pub struct BlueGreenDeploymentTask {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeOptionGroupsResult")]
pub struct OptionGroups {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "OptionGroupsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_groups_list: Option<OptionGroupsList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OptionGroupsList {
    #[serde(rename = "OptionGroup", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<OptionGroup>,
}
impl From<Vec<OptionGroup>> for OptionGroupsList {
    fn from(v: Vec<OptionGroup>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<OptionGroup> for OptionGroupsList {
    fn from_iter<I: IntoIterator<Item = OptionGroup>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<OptionGroup>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlOptionGroupList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<OptionGroup>,
}

impl From<Vec<OptionGroup>> for XmlOptionGroupList {
    fn from(v: Vec<OptionGroup>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<OptionGroup> for XmlOptionGroupList {
    fn from_iter<I: IntoIterator<Item = OptionGroup>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OptionGroup")]
pub struct OptionGroup {
    #[serde(rename = "AllowsVpcAndNonVpcInstanceMemberships")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_vpc_and_non_vpc_instance_memberships: Option<bool>,
    #[serde(rename = "CopyTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_timestamp: Option<String>,
    #[serde(rename = "EngineName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_name: Option<String>,
    #[serde(rename = "MajorEngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major_engine_version: Option<String>,
    #[serde(rename = "OptionGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_arn: Option<String>,
    #[serde(rename = "OptionGroupDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_description: Option<String>,
    #[serde(rename = "OptionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_name: Option<String>,
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<OptionsList>,
    #[serde(rename = "SourceAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_account_id: Option<String>,
    #[serde(rename = "SourceOptionGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_option_group: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OptionsList {
    #[serde(rename = "Option", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Option_>,
}
impl From<Vec<Option_>> for OptionsList {
    fn from(v: Vec<Option_>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Option_> for OptionsList {
    fn from_iter<I: IntoIterator<Item = Option_>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Option_>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlOption_List {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Option_>,
}

impl From<Vec<Option_>> for XmlOption_List {
    fn from(v: Vec<Option_>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Option_> for XmlOption_List {
    fn from_iter<I: IntoIterator<Item = Option_>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Option_")]
pub struct Option_ {
    #[serde(rename = "DBSecurityGroupMemberships")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_security_group_memberships: Option<DBSecurityGroupMembershipList>,
    #[serde(rename = "OptionDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_description: Option<String>,
    #[serde(rename = "OptionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_name: Option<String>,
    #[serde(rename = "OptionSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_settings: Option<OptionSettingConfigurationList>,
    #[serde(rename = "OptionVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_version: Option<String>,
    #[serde(rename = "Permanent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permanent: Option<bool>,
    #[serde(rename = "Persistent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent: Option<bool>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "VpcSecurityGroupMemberships")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_memberships: Option<VpcSecurityGroupMembershipList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OptionSettingConfigurationList {
    #[serde(
        rename = "OptionSetting",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<OptionSetting>,
}
impl From<Vec<OptionSetting>> for OptionSettingConfigurationList {
    fn from(v: Vec<OptionSetting>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<OptionSetting> for OptionSettingConfigurationList {
    fn from_iter<I: IntoIterator<Item = OptionSetting>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<OptionSetting>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlOptionSettingList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<OptionSetting>,
}

impl From<Vec<OptionSetting>> for XmlOptionSettingList {
    fn from(v: Vec<OptionSetting>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<OptionSetting> for XmlOptionSettingList {
    fn from_iter<I: IntoIterator<Item = OptionSetting>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OptionSetting")]
pub struct OptionSetting {
    #[serde(rename = "AllowedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<String>,
    #[serde(rename = "ApplyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_type: Option<String>,
    #[serde(rename = "DataType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(rename = "DefaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "IsCollection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_collection: Option<bool>,
    #[serde(rename = "IsModifiable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_modifiable: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "BacktrackDBClusterMessage")]
pub struct BacktrackDBClusterMessage {
    #[serde(rename = "BacktrackTo")]
    #[serde(default)]
    pub backtrack_to: String,
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    pub d_b_cluster_identifier: String,
    #[serde(rename = "Force")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[serde(rename = "UseEarliestTimeOnPointInTimeUnavailable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_earliest_time_on_point_in_time_unavailable: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyDBClusterSnapshotAttributeMessage")]
pub struct ModifyDBClusterSnapshotAttributeMessage {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    pub attribute_name: String,
    #[serde(rename = "DBClusterSnapshotIdentifier")]
    #[serde(default)]
    pub d_b_cluster_snapshot_identifier: String,
    #[serde(rename = "ValuesToAdd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values_to_add: Option<AttributeValueList>,
    #[serde(rename = "ValuesToRemove")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values_to_remove: Option<AttributeValueList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteDBInstanceAutomatedBackupResult")]
pub struct DeleteDBInstanceAutomatedBackupResult {
    #[serde(rename = "DBInstanceAutomatedBackup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_automated_backup: Option<DBInstanceAutomatedBackup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RebootDBInstanceMessage")]
pub struct RebootDBInstanceMessage {
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    pub d_b_instance_identifier: String,
    #[serde(rename = "ForceFailover")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_failover: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SwitchoverReadReplicaMessage")]
pub struct SwitchoverReadReplicaMessage {
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    pub d_b_instance_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CancelExportTaskMessage")]
pub struct CancelExportTaskMessage {
    #[serde(rename = "ExportTaskIdentifier")]
    #[serde(default)]
    pub export_task_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteBlueGreenDeploymentResult")]
pub struct DeleteBlueGreenDeploymentResponse {
    #[serde(rename = "BlueGreenDeployment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue_green_deployment: Option<BlueGreenDeployment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDBParameterGroupMessage")]
pub struct CreateDBParameterGroupMessage {
    #[serde(rename = "DBParameterGroupFamily")]
    #[serde(default)]
    pub d_b_parameter_group_family: String,
    #[serde(rename = "DBParameterGroupName")]
    #[serde(default)]
    pub d_b_parameter_group_name: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    pub description: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteDBClusterSnapshotMessage")]
pub struct DeleteDBClusterSnapshotMessage {
    #[serde(rename = "DBClusterSnapshotIdentifier")]
    #[serde(default)]
    pub d_b_cluster_snapshot_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBClusterSnapshotsResult")]
pub struct DBClusterSnapshotMessage {
    #[serde(rename = "DBClusterSnapshots")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_snapshots: Option<DBClusterSnapshotList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBClusterSnapshotList {
    #[serde(
        rename = "DBClusterSnapshot",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DBClusterSnapshot>,
}
impl From<Vec<DBClusterSnapshot>> for DBClusterSnapshotList {
    fn from(v: Vec<DBClusterSnapshot>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBClusterSnapshot> for DBClusterSnapshotList {
    fn from_iter<I: IntoIterator<Item = DBClusterSnapshot>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBClusterSnapshot>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBClusterSnapshotList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBClusterSnapshot>,
}

impl From<Vec<DBClusterSnapshot>> for XmlDBClusterSnapshotList {
    fn from(v: Vec<DBClusterSnapshot>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBClusterSnapshot> for XmlDBClusterSnapshotList {
    fn from_iter<I: IntoIterator<Item = DBClusterSnapshot>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBClusterSnapshot")]
pub struct DBClusterSnapshot {
    #[serde(rename = "AllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<AvailabilityZones>,
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i32>,
    #[serde(rename = "ClusterCreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_create_time: Option<String>,
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_identifier: Option<String>,
    #[serde(rename = "DBClusterSnapshotArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_snapshot_arn: Option<String>,
    #[serde(rename = "DBClusterSnapshotIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_snapshot_identifier: Option<String>,
    #[serde(rename = "DBSystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_system_id: Option<String>,
    #[serde(rename = "DbClusterResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_resource_id: Option<String>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_mode: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "IAMDatabaseAuthenticationEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_a_m_database_authentication_enabled: Option<bool>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "LicenseModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_model: Option<String>,
    #[serde(rename = "MasterUsername")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_username: Option<String>,
    #[serde(rename = "PercentProgress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_progress: Option<i32>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "PreferredBackupWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,
    #[serde(rename = "SnapshotCreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_create_time: Option<String>,
    #[serde(rename = "SnapshotType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_type: Option<String>,
    #[serde(rename = "SourceDBClusterSnapshotArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_d_b_cluster_snapshot_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StorageEncrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encrypted: Option<bool>,
    #[serde(rename = "StorageEncryptionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encryption_type: Option<String>,
    #[serde(rename = "StorageThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_throughput: Option<i32>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "TagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<TagList>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyEventSubscriptionResult")]
pub struct ModifyEventSubscriptionResult {
    #[serde(rename = "EventSubscription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_subscription: Option<EventSubscription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EventSubscription")]
pub struct EventSubscription {
    #[serde(rename = "CustSubscriptionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cust_subscription_id: Option<String>,
    #[serde(rename = "CustomerAwsId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_aws_id: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "EventCategoriesList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_categories_list: Option<EventCategoriesList>,
    #[serde(rename = "EventSubscriptionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_subscription_arn: Option<String>,
    #[serde(rename = "SnsTopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
    #[serde(rename = "SourceIdsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ids_list: Option<SourceIdsList>,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "SubscriptionCreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_creation_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventCategoriesList {
    #[serde(
        rename = "EventCategory",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for EventCategoriesList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for EventCategoriesList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceIdsList {
    #[serde(rename = "SourceId", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for SourceIdsList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for SourceIdsList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBProxyTargetGroupsResult")]
pub struct DescribeDBProxyTargetGroupsResponse {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "TargetGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_groups: Option<TargetGroupList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetGroupList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<DBProxyTargetGroup>,
}
impl From<Vec<DBProxyTargetGroup>> for TargetGroupList {
    fn from(v: Vec<DBProxyTargetGroup>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBProxyTargetGroup> for TargetGroupList {
    fn from_iter<I: IntoIterator<Item = DBProxyTargetGroup>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBProxyTargetGroup>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBProxyTargetGroupList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBProxyTargetGroup>,
}

impl From<Vec<DBProxyTargetGroup>> for XmlDBProxyTargetGroupList {
    fn from(v: Vec<DBProxyTargetGroup>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBProxyTargetGroup> for XmlDBProxyTargetGroupList {
    fn from_iter<I: IntoIterator<Item = DBProxyTargetGroup>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBProxyTargetGroup")]
pub struct DBProxyTargetGroup {
    #[serde(rename = "ConnectionPoolConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_pool_config: Option<ConnectionPoolConfigurationInfo>,
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(rename = "DBProxyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_proxy_name: Option<String>,
    #[serde(rename = "IsDefault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TargetGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_group_arn: Option<String>,
    #[serde(rename = "TargetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_group_name: Option<String>,
    #[serde(rename = "UpdatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ConnectionPoolConfigurationInfo")]
pub struct ConnectionPoolConfigurationInfo {
    #[serde(rename = "ConnectionBorrowTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_borrow_timeout: Option<i32>,
    #[serde(rename = "InitQuery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_query: Option<String>,
    #[serde(rename = "MaxConnectionsPercent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections_percent: Option<i32>,
    #[serde(rename = "MaxIdleConnectionsPercent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_idle_connections_percent: Option<i32>,
    #[serde(rename = "SessionPinningFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_pinning_filters: Option<StringList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteCustomDBEngineVersionMessage")]
pub struct DeleteCustomDBEngineVersionMessage {
    #[serde(rename = "Engine")]
    #[serde(default)]
    pub engine: String,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    pub engine_version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CopyDBSnapshotResult")]
pub struct CopyDBSnapshotResult {
    #[serde(rename = "DBSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_snapshot: Option<DBSnapshot>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "FailoverDBClusterMessage")]
pub struct FailoverDBClusterMessage {
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    pub d_b_cluster_identifier: String,
    #[serde(rename = "TargetDBInstanceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_d_b_instance_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeBlueGreenDeploymentsResult")]
pub struct DescribeBlueGreenDeploymentsResponse {
    #[serde(rename = "BlueGreenDeployments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue_green_deployments: Option<BlueGreenDeploymentList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BlueGreenDeploymentList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<BlueGreenDeployment>,
}
impl From<Vec<BlueGreenDeployment>> for BlueGreenDeploymentList {
    fn from(v: Vec<BlueGreenDeployment>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<BlueGreenDeployment> for BlueGreenDeploymentList {
    fn from_iter<I: IntoIterator<Item = BlueGreenDeployment>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<BlueGreenDeployment>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlBlueGreenDeploymentList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<BlueGreenDeployment>,
}

impl From<Vec<BlueGreenDeployment>> for XmlBlueGreenDeploymentList {
    fn from(v: Vec<BlueGreenDeployment>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<BlueGreenDeployment> for XmlBlueGreenDeploymentList {
    fn from_iter<I: IntoIterator<Item = BlueGreenDeployment>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AddSourceIdentifierToSubscriptionMessage")]
pub struct AddSourceIdentifierToSubscriptionMessage {
    #[serde(rename = "SourceIdentifier")]
    #[serde(default)]
    pub source_identifier: String,
    #[serde(rename = "SubscriptionName")]
    #[serde(default)]
    pub subscription_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeValidDBInstanceModificationsMessage")]
pub struct DescribeValidDBInstanceModificationsMessage {
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    pub d_b_instance_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SwitchoverGlobalClusterMessage")]
pub struct SwitchoverGlobalClusterMessage {
    #[serde(rename = "GlobalClusterIdentifier")]
    #[serde(default)]
    pub global_cluster_identifier: String,
    #[serde(rename = "TargetDbClusterIdentifier")]
    #[serde(default)]
    pub target_db_cluster_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBInstancesMessage")]
pub struct DescribeDBInstancesMessage {
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_identifier: Option<String>,
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
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDBShardGroupMessage")]
pub struct CreateDBShardGroupMessage {
    #[serde(rename = "ComputeRedundancy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_redundancy: Option<i32>,
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    pub d_b_cluster_identifier: String,
    #[serde(rename = "DBShardGroupIdentifier")]
    #[serde(default)]
    pub d_b_shard_group_identifier: String,
    #[serde(rename = "MaxACU")]
    #[serde(default)]
    pub max_a_c_u: f64,
    #[serde(rename = "MinACU")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_a_c_u: Option<f64>,
    #[serde(rename = "PubliclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeReservedDBInstancesOfferingsMessage")]
pub struct DescribeReservedDBInstancesOfferingsMessage {
    #[serde(rename = "DBInstanceClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_class: Option<String>,
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
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
    #[serde(rename = "MultiAZ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_a_z: Option<bool>,
    #[serde(rename = "OfferingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_type: Option<String>,
    #[serde(rename = "ProductDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    #[serde(rename = "ReservedDBInstancesOfferingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_d_b_instances_offering_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEngineDefaultParametersMessage")]
pub struct DescribeEngineDefaultParametersMessage {
    #[serde(rename = "DBParameterGroupFamily")]
    #[serde(default)]
    pub d_b_parameter_group_family: String,
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
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEventCategoriesResult")]
pub struct EventCategoriesMessage {
    #[serde(rename = "EventCategoriesMapList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_categories_map_list: Option<EventCategoriesMapList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventCategoriesMapList {
    #[serde(
        rename = "EventCategoriesMap",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<EventCategoriesMap>,
}
impl From<Vec<EventCategoriesMap>> for EventCategoriesMapList {
    fn from(v: Vec<EventCategoriesMap>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<EventCategoriesMap> for EventCategoriesMapList {
    fn from_iter<I: IntoIterator<Item = EventCategoriesMap>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<EventCategoriesMap>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlEventCategoriesMapList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<EventCategoriesMap>,
}

impl From<Vec<EventCategoriesMap>> for XmlEventCategoriesMapList {
    fn from(v: Vec<EventCategoriesMap>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<EventCategoriesMap> for XmlEventCategoriesMapList {
    fn from_iter<I: IntoIterator<Item = EventCategoriesMap>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EventCategoriesMap")]
pub struct EventCategoriesMap {
    #[serde(rename = "EventCategories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_categories: Option<EventCategoriesList>,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyDBClusterMessage")]
pub struct ModifyDBClusterMessage {
    #[serde(rename = "AllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(rename = "AllowEngineModeChange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_engine_mode_change: Option<bool>,
    #[serde(rename = "AllowMajorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_major_version_upgrade: Option<bool>,
    #[serde(rename = "ApplyImmediately")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_immediately: Option<bool>,
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    #[serde(rename = "AwsBackupRecoveryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_backup_recovery_point_arn: Option<String>,
    #[serde(rename = "BacktrackWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backtrack_window: Option<i64>,
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i32>,
    #[serde(rename = "CACertificateIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_a_certificate_identifier: Option<String>,
    #[serde(rename = "CloudwatchLogsExportConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudwatch_logs_export_configuration: Option<CloudwatchLogsExportConfiguration>,
    #[serde(rename = "CopyTagsToSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_snapshot: Option<bool>,
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    pub d_b_cluster_identifier: String,
    #[serde(rename = "DBClusterInstanceClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_instance_class: Option<String>,
    #[serde(rename = "DBClusterParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_parameter_group_name: Option<String>,
    #[serde(rename = "DBInstanceParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_parameter_group_name: Option<String>,
    #[serde(rename = "DatabaseInsightsMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_insights_mode: Option<String>,
    #[serde(rename = "DeletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "DomainIAMRoleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_i_a_m_role_name: Option<String>,
    #[serde(rename = "EnableGlobalWriteForwarding")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_global_write_forwarding: Option<bool>,
    #[serde(rename = "EnableHttpEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_http_endpoint: Option<bool>,
    #[serde(rename = "EnableIAMDatabaseAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_i_a_m_database_authentication: Option<bool>,
    #[serde(rename = "EnableLimitlessDatabase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_limitless_database: Option<bool>,
    #[serde(rename = "EnableLocalWriteForwarding")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_local_write_forwarding: Option<bool>,
    #[serde(rename = "EnablePerformanceInsights")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_performance_insights: Option<bool>,
    #[serde(rename = "EngineMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_mode: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "Iops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    #[serde(rename = "ManageMasterUserPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manage_master_user_password: Option<bool>,
    #[serde(rename = "MasterUserAuthenticationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_authentication_type: Option<String>,
    #[serde(rename = "MasterUserPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<String>,
    #[serde(rename = "MasterUserSecretKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_secret_kms_key_id: Option<String>,
    #[serde(rename = "MonitoringInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_interval: Option<i32>,
    #[serde(rename = "MonitoringRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_role_arn: Option<String>,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "NewDBClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_d_b_cluster_identifier: Option<String>,
    #[serde(rename = "OptionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_name: Option<String>,
    #[serde(rename = "PerformanceInsightsKMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_insights_k_m_s_key_id: Option<String>,
    #[serde(rename = "PerformanceInsightsRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_insights_retention_period: Option<i32>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "PreferredBackupWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[serde(rename = "RotateMasterUserPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate_master_user_password: Option<bool>,
    #[serde(rename = "ScalingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_configuration: Option<ScalingConfiguration>,
    #[serde(rename = "ServerlessV2ScalingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless_v2_scaling_configuration: Option<ServerlessV2ScalingConfiguration>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<VpcSecurityGroupIdList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CloudwatchLogsExportConfiguration")]
pub struct CloudwatchLogsExportConfiguration {
    #[serde(rename = "DisableLogTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_log_types: Option<LogTypeList>,
    #[serde(rename = "EnableLogTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_log_types: Option<LogTypeList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeGlobalClustersMessage")]
pub struct DescribeGlobalClustersMessage {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FilterList>,
    #[serde(rename = "GlobalClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_cluster_identifier: Option<String>,
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
#[serde(rename = "ModifyIntegrationMessage")]
pub struct ModifyIntegrationMessage {
    #[serde(rename = "DataFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_filter: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "IntegrationIdentifier")]
    #[serde(default)]
    pub integration_identifier: String,
    #[serde(rename = "IntegrationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyDBShardGroupMessage")]
pub struct ModifyDBShardGroupMessage {
    #[serde(rename = "ComputeRedundancy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_redundancy: Option<i32>,
    #[serde(rename = "DBShardGroupIdentifier")]
    #[serde(default)]
    pub d_b_shard_group_identifier: String,
    #[serde(rename = "MaxACU")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_a_c_u: Option<f64>,
    #[serde(rename = "MinACU")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_a_c_u: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyIntegrationResult")]
pub struct Integration {
    #[serde(rename = "AdditionalEncryptionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_encryption_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "CreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(rename = "DataFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_filter: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<IntegrationErrorList>,
    #[serde(rename = "IntegrationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_arn: Option<String>,
    #[serde(rename = "IntegrationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_name: Option<String>,
    #[serde(rename = "KMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_key_id: Option<String>,
    #[serde(rename = "SourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "TargetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntegrationErrorList {
    #[serde(
        rename = "IntegrationError",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<IntegrationError>,
}
impl From<Vec<IntegrationError>> for IntegrationErrorList {
    fn from(v: Vec<IntegrationError>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<IntegrationError> for IntegrationErrorList {
    fn from_iter<I: IntoIterator<Item = IntegrationError>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<IntegrationError>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlIntegrationErrorList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<IntegrationError>,
}

impl From<Vec<IntegrationError>> for XmlIntegrationErrorList {
    fn from(v: Vec<IntegrationError>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<IntegrationError> for XmlIntegrationErrorList {
    fn from_iter<I: IntoIterator<Item = IntegrationError>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "IntegrationError")]
pub struct IntegrationError {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RestoreDBClusterFromS3Message")]
pub struct RestoreDBClusterFromS3Message {
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<AvailabilityZones>,
    #[serde(rename = "BacktrackWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backtrack_window: Option<i64>,
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i32>,
    #[serde(rename = "CharacterSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_set_name: Option<String>,
    #[serde(rename = "CopyTagsToSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_snapshot: Option<bool>,
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    pub d_b_cluster_identifier: String,
    #[serde(rename = "DBClusterParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_parameter_group_name: Option<String>,
    #[serde(rename = "DBSubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_subnet_group_name: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "DeletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "DomainIAMRoleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_i_a_m_role_name: Option<String>,
    #[serde(rename = "EnableCloudwatchLogsExports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_cloudwatch_logs_exports: Option<LogTypeList>,
    #[serde(rename = "EnableIAMDatabaseAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_i_a_m_database_authentication: Option<bool>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    pub engine: String,
    #[serde(rename = "EngineLifecycleSupport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_lifecycle_support: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "ManageMasterUserPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manage_master_user_password: Option<bool>,
    #[serde(rename = "MasterUserPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<String>,
    #[serde(rename = "MasterUserSecretKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_secret_kms_key_id: Option<String>,
    #[serde(rename = "MasterUsername")]
    #[serde(default)]
    pub master_username: String,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "OptionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_name: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "PreferredBackupWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[serde(rename = "S3BucketName")]
    #[serde(default)]
    pub s3_bucket_name: String,
    #[serde(rename = "S3IngestionRoleArn")]
    #[serde(default)]
    pub s3_ingestion_role_arn: String,
    #[serde(rename = "S3Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_prefix: Option<String>,
    #[serde(rename = "ServerlessV2ScalingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless_v2_scaling_configuration: Option<ServerlessV2ScalingConfiguration>,
    #[serde(rename = "SourceEngine")]
    #[serde(default)]
    pub source_engine: String,
    #[serde(rename = "SourceEngineVersion")]
    #[serde(default)]
    pub source_engine_version: String,
    #[serde(rename = "StorageEncrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encrypted: Option<bool>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "TagSpecifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_specifications: Option<TagSpecificationList>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<VpcSecurityGroupIdList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PromoteReadReplicaResult")]
pub struct PromoteReadReplicaResult {
    #[serde(rename = "DBInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance: Option<DBInstance>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RestoreDBInstanceToPointInTimeResult")]
pub struct RestoreDBInstanceToPointInTimeResult {
    #[serde(rename = "DBInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance: Option<DBInstance>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StopDBClusterResult")]
pub struct StopDBClusterResult {
    #[serde(rename = "DBCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster: Option<DBCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StopDBInstanceMessage")]
pub struct StopDBInstanceMessage {
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    pub d_b_instance_identifier: String,
    #[serde(rename = "DBSnapshotIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_snapshot_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PromoteReadReplicaMessage")]
pub struct PromoteReadReplicaMessage {
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i32>,
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    pub d_b_instance_identifier: String,
    #[serde(rename = "PreferredBackupWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,
    #[serde(rename = "TagSpecifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_specifications: Option<TagSpecificationList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBInstancesResult")]
pub struct DBInstanceMessage {
    #[serde(rename = "DBInstances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instances: Option<DBInstanceList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBInstanceList {
    #[serde(rename = "DBInstance", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<DBInstance>,
}
impl From<Vec<DBInstance>> for DBInstanceList {
    fn from(v: Vec<DBInstance>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBInstance> for DBInstanceList {
    fn from_iter<I: IntoIterator<Item = DBInstance>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBInstance>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBInstanceList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBInstance>,
}

impl From<Vec<DBInstance>> for XmlDBInstanceList {
    fn from(v: Vec<DBInstance>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBInstance> for XmlDBInstanceList {
    fn from_iter<I: IntoIterator<Item = DBInstance>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RestoreDBInstanceFromDBSnapshotMessage")]
pub struct RestoreDBInstanceFromDBSnapshotMessage {
    #[serde(rename = "AdditionalStorageVolumes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_storage_volumes: Option<AdditionalStorageVolumesList>,
    #[serde(rename = "AllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i32>,
    #[serde(rename = "BackupTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_target: Option<String>,
    #[serde(rename = "CACertificateIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_a_certificate_identifier: Option<String>,
    #[serde(rename = "CopyTagsToSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_snapshot: Option<bool>,
    #[serde(rename = "CustomIamInstanceProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_iam_instance_profile: Option<String>,
    #[serde(rename = "DBClusterSnapshotIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_snapshot_identifier: Option<String>,
    #[serde(rename = "DBInstanceClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_class: Option<String>,
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    pub d_b_instance_identifier: String,
    #[serde(rename = "DBName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_name: Option<String>,
    #[serde(rename = "DBParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_parameter_group_name: Option<String>,
    #[serde(rename = "DBSnapshotIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_snapshot_identifier: Option<String>,
    #[serde(rename = "DBSubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_subnet_group_name: Option<String>,
    #[serde(rename = "DedicatedLogVolume")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_log_volume: Option<bool>,
    #[serde(rename = "DeletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "DomainAuthSecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_auth_secret_arn: Option<String>,
    #[serde(rename = "DomainDnsIps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_dns_ips: Option<StringList>,
    #[serde(rename = "DomainFqdn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_fqdn: Option<String>,
    #[serde(rename = "DomainIAMRoleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_i_a_m_role_name: Option<String>,
    #[serde(rename = "DomainOu")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_ou: Option<String>,
    #[serde(rename = "EnableCloudwatchLogsExports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_cloudwatch_logs_exports: Option<LogTypeList>,
    #[serde(rename = "EnableCustomerOwnedIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_customer_owned_ip: Option<bool>,
    #[serde(rename = "EnableIAMDatabaseAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_i_a_m_database_authentication: Option<bool>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineLifecycleSupport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_lifecycle_support: Option<String>,
    #[serde(rename = "Iops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    #[serde(rename = "LicenseModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_model: Option<String>,
    #[serde(rename = "ManageMasterUserPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manage_master_user_password: Option<bool>,
    #[serde(rename = "MasterUserSecretKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_secret_kms_key_id: Option<String>,
    #[serde(rename = "MultiAZ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_a_z: Option<bool>,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "OptionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_name: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "PreferredBackupWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,
    #[serde(rename = "ProcessorFeatures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_features: Option<ProcessorFeatureList>,
    #[serde(rename = "PubliclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    #[serde(rename = "StorageThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_throughput: Option<i32>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "TagSpecifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_specifications: Option<TagSpecificationList>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "TdeCredentialArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tde_credential_arn: Option<String>,
    #[serde(rename = "TdeCredentialPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tde_credential_password: Option<String>,
    #[serde(rename = "UseDefaultProcessorFeatures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_default_processor_features: Option<bool>,
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<VpcSecurityGroupIdList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RebootDBShardGroupResult")]
pub struct DBShardGroup {
    #[serde(rename = "ComputeRedundancy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_redundancy: Option<i32>,
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_identifier: Option<String>,
    #[serde(rename = "DBShardGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_shard_group_arn: Option<String>,
    #[serde(rename = "DBShardGroupIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_shard_group_identifier: Option<String>,
    #[serde(rename = "DBShardGroupResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_shard_group_resource_id: Option<String>,
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "MaxACU")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_a_c_u: Option<f64>,
    #[serde(rename = "MinACU")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_a_c_u: Option<f64>,
    #[serde(rename = "PubliclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBProxyTargetsResult")]
pub struct DescribeDBProxyTargetsResponse {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<TargetList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyDBProxyEndpointResult")]
pub struct ModifyDBProxyEndpointResponse {
    #[serde(rename = "DBProxyEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_proxy_endpoint: Option<DBProxyEndpoint>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBProxyEndpoint")]
pub struct DBProxyEndpoint {
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(rename = "DBProxyEndpointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_proxy_endpoint_arn: Option<String>,
    #[serde(rename = "DBProxyEndpointName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_proxy_endpoint_name: Option<String>,
    #[serde(rename = "DBProxyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_proxy_name: Option<String>,
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "EndpointNetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_network_type: Option<String>,
    #[serde(rename = "IsDefault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TargetRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_role: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<StringList>,
    #[serde(rename = "VpcSubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_subnet_ids: Option<StringList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RestoreDBClusterFromS3Result")]
pub struct RestoreDBClusterFromS3Result {
    #[serde(rename = "DBCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster: Option<DBCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEventCategoriesMessage")]
pub struct DescribeEventCategoriesMessage {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FilterList>,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteBlueGreenDeploymentRequest")]
pub struct DeleteBlueGreenDeploymentRequest {
    #[serde(rename = "BlueGreenDeploymentIdentifier")]
    #[serde(default)]
    pub blue_green_deployment_identifier: String,
    #[serde(rename = "DeleteTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_target: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RemoveSourceIdentifierFromSubscriptionMessage")]
pub struct RemoveSourceIdentifierFromSubscriptionMessage {
    #[serde(rename = "SourceIdentifier")]
    #[serde(default)]
    pub source_identifier: String,
    #[serde(rename = "SubscriptionName")]
    #[serde(default)]
    pub subscription_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateEventSubscriptionMessage")]
pub struct CreateEventSubscriptionMessage {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "EventCategories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_categories: Option<EventCategoriesList>,
    #[serde(rename = "SnsTopicArn")]
    #[serde(default)]
    pub sns_topic_arn: String,
    #[serde(rename = "SourceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ids: Option<SourceIdsList>,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(rename = "SubscriptionName")]
    #[serde(default)]
    pub subscription_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RestoreDBClusterToPointInTimeResult")]
pub struct RestoreDBClusterToPointInTimeResult {
    #[serde(rename = "DBCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster: Option<DBCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CopyDBParameterGroupResult")]
pub struct CopyDBParameterGroupResult {
    #[serde(rename = "DBParameterGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_parameter_group: Option<DBParameterGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBParameterGroup")]
pub struct DBParameterGroup {
    #[serde(rename = "DBParameterGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_parameter_group_arn: Option<String>,
    #[serde(rename = "DBParameterGroupFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_parameter_group_family: Option<String>,
    #[serde(rename = "DBParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_parameter_group_name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDBProxyResult")]
pub struct CreateDBProxyResponse {
    #[serde(rename = "DBProxy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_proxy: Option<DBProxy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBParameterGroupsResult")]
pub struct DBParameterGroupsMessage {
    #[serde(rename = "DBParameterGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_parameter_groups: Option<DBParameterGroupList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBParameterGroupList {
    #[serde(
        rename = "DBParameterGroup",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DBParameterGroup>,
}
impl From<Vec<DBParameterGroup>> for DBParameterGroupList {
    fn from(v: Vec<DBParameterGroup>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBParameterGroup> for DBParameterGroupList {
    fn from_iter<I: IntoIterator<Item = DBParameterGroup>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBParameterGroup>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBParameterGroupList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBParameterGroup>,
}

impl From<Vec<DBParameterGroup>> for XmlDBParameterGroupList {
    fn from(v: Vec<DBParameterGroup>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBParameterGroup> for XmlDBParameterGroupList {
    fn from_iter<I: IntoIterator<Item = DBParameterGroup>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBSnapshotTenantDatabasesResult")]
pub struct DBSnapshotTenantDatabasesMessage {
    #[serde(rename = "DBSnapshotTenantDatabases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_snapshot_tenant_databases: Option<DBSnapshotTenantDatabasesList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBSnapshotTenantDatabasesList {
    #[serde(
        rename = "DBSnapshotTenantDatabase",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DBSnapshotTenantDatabase>,
}
impl From<Vec<DBSnapshotTenantDatabase>> for DBSnapshotTenantDatabasesList {
    fn from(v: Vec<DBSnapshotTenantDatabase>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBSnapshotTenantDatabase> for DBSnapshotTenantDatabasesList {
    fn from_iter<I: IntoIterator<Item = DBSnapshotTenantDatabase>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBSnapshotTenantDatabase>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBSnapshotTenantDatabaseList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBSnapshotTenantDatabase>,
}

impl From<Vec<DBSnapshotTenantDatabase>> for XmlDBSnapshotTenantDatabaseList {
    fn from(v: Vec<DBSnapshotTenantDatabase>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBSnapshotTenantDatabase> for XmlDBSnapshotTenantDatabaseList {
    fn from_iter<I: IntoIterator<Item = DBSnapshotTenantDatabase>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBSnapshotTenantDatabase")]
pub struct DBSnapshotTenantDatabase {
    #[serde(rename = "CharacterSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_set_name: Option<String>,
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_identifier: Option<String>,
    #[serde(rename = "DBSnapshotIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_snapshot_identifier: Option<String>,
    #[serde(rename = "DBSnapshotTenantDatabaseARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_snapshot_tenant_database_a_r_n: Option<String>,
    #[serde(rename = "DbiResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbi_resource_id: Option<String>,
    #[serde(rename = "EngineName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_name: Option<String>,
    #[serde(rename = "MasterUsername")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_username: Option<String>,
    #[serde(rename = "NcharCharacterSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nchar_character_set_name: Option<String>,
    #[serde(rename = "SnapshotType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_type: Option<String>,
    #[serde(rename = "TagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<TagList>,
    #[serde(rename = "TenantDBName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_d_b_name: Option<String>,
    #[serde(rename = "TenantDatabaseCreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_database_create_time: Option<String>,
    #[serde(rename = "TenantDatabaseResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_database_resource_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeReservedDBInstancesMessage")]
pub struct DescribeReservedDBInstancesMessage {
    #[serde(rename = "DBInstanceClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_class: Option<String>,
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FilterList>,
    #[serde(rename = "LeaseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lease_id: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "MultiAZ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_a_z: Option<bool>,
    #[serde(rename = "OfferingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_type: Option<String>,
    #[serde(rename = "ProductDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    #[serde(rename = "ReservedDBInstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_d_b_instance_id: Option<String>,
    #[serde(rename = "ReservedDBInstancesOfferingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_d_b_instances_offering_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StopDBClusterMessage")]
pub struct StopDBClusterMessage {
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    pub d_b_cluster_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDBSubnetGroupMessage")]
pub struct CreateDBSubnetGroupMessage {
    #[serde(rename = "DBSubnetGroupDescription")]
    #[serde(default)]
    pub d_b_subnet_group_description: String,
    #[serde(rename = "DBSubnetGroupName")]
    #[serde(default)]
    pub d_b_subnet_group_name: String,
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
#[serde(rename = "DeleteDBInstanceResult")]
pub struct DeleteDBInstanceResult {
    #[serde(rename = "DBInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance: Option<DBInstance>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDBInstanceReadReplicaResult")]
pub struct CreateDBInstanceReadReplicaResult {
    #[serde(rename = "DBInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance: Option<DBInstance>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteGlobalClusterMessage")]
pub struct DeleteGlobalClusterMessage {
    #[serde(rename = "GlobalClusterIdentifier")]
    #[serde(default)]
    pub global_cluster_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteDBProxyEndpointRequest")]
pub struct DeleteDBProxyEndpointRequest {
    #[serde(rename = "DBProxyEndpointName")]
    #[serde(default)]
    pub d_b_proxy_endpoint_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDBParameterGroupResult")]
pub struct CreateDBParameterGroupResult {
    #[serde(rename = "DBParameterGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_parameter_group: Option<DBParameterGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDBClusterEndpointMessage")]
pub struct CreateDBClusterEndpointMessage {
    #[serde(rename = "DBClusterEndpointIdentifier")]
    #[serde(default)]
    pub d_b_cluster_endpoint_identifier: String,
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    pub d_b_cluster_identifier: String,
    #[serde(rename = "EndpointType")]
    #[serde(default)]
    pub endpoint_type: String,
    #[serde(rename = "ExcludedMembers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_members: Option<StringList>,
    #[serde(rename = "StaticMembers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_members: Option<StringList>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EnableHttpEndpointRequest")]
pub struct EnableHttpEndpointRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EnableHttpEndpointResult")]
pub struct EnableHttpEndpointResponse {
    #[serde(rename = "HttpEndpointEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_endpoint_enabled: Option<bool>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyDBSubnetGroupResult")]
pub struct ModifyDBSubnetGroupResult {
    #[serde(rename = "DBSubnetGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_subnet_group: Option<DBSubnetGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RebootDBInstanceResult")]
pub struct RebootDBInstanceResult {
    #[serde(rename = "DBInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance: Option<DBInstance>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeExportTasksMessage")]
pub struct DescribeExportTasksMessage {
    #[serde(rename = "ExportTaskIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_task_identifier: Option<String>,
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
    #[serde(rename = "SourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SwitchoverGlobalClusterResult")]
pub struct SwitchoverGlobalClusterResult {
    #[serde(rename = "GlobalCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_cluster: Option<GlobalCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBClusterSnapshotAttributesResult")]
pub struct DescribeDBClusterSnapshotAttributesResult {
    #[serde(rename = "DBClusterSnapshotAttributesResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_snapshot_attributes_result: Option<DBClusterSnapshotAttributesResult>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBClusterSnapshotAttributesResult")]
pub struct DBClusterSnapshotAttributesResult {
    #[serde(rename = "DBClusterSnapshotAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_snapshot_attributes: Option<DBClusterSnapshotAttributeList>,
    #[serde(rename = "DBClusterSnapshotIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_snapshot_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBClusterSnapshotAttributeList {
    #[serde(
        rename = "DBClusterSnapshotAttribute",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DBClusterSnapshotAttribute>,
}
impl From<Vec<DBClusterSnapshotAttribute>> for DBClusterSnapshotAttributeList {
    fn from(v: Vec<DBClusterSnapshotAttribute>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBClusterSnapshotAttribute> for DBClusterSnapshotAttributeList {
    fn from_iter<I: IntoIterator<Item = DBClusterSnapshotAttribute>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBClusterSnapshotAttribute>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBClusterSnapshotAttributeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBClusterSnapshotAttribute>,
}

impl From<Vec<DBClusterSnapshotAttribute>> for XmlDBClusterSnapshotAttributeList {
    fn from(v: Vec<DBClusterSnapshotAttribute>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBClusterSnapshotAttribute> for XmlDBClusterSnapshotAttributeList {
    fn from_iter<I: IntoIterator<Item = DBClusterSnapshotAttribute>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBClusterSnapshotAttribute")]
pub struct DBClusterSnapshotAttribute {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[serde(rename = "AttributeValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_values: Option<AttributeValueList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBLogFilesMessage")]
pub struct DescribeDBLogFilesMessage {
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    pub d_b_instance_identifier: String,
    #[serde(rename = "FileLastWritten")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_last_written: Option<i64>,
    #[serde(rename = "FileSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    #[serde(rename = "FilenameContains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename_contains: Option<String>,
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
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBProxiesResult")]
pub struct DescribeDBProxiesResponse {
    #[serde(rename = "DBProxies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_proxies: Option<DBProxyList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBProxyList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<DBProxy>,
}
impl From<Vec<DBProxy>> for DBProxyList {
    fn from(v: Vec<DBProxy>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBProxy> for DBProxyList {
    fn from_iter<I: IntoIterator<Item = DBProxy>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBProxy>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBProxyList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBProxy>,
}

impl From<Vec<DBProxy>> for XmlDBProxyList {
    fn from(v: Vec<DBProxy>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBProxy> for XmlDBProxyList {
    fn from_iter<I: IntoIterator<Item = DBProxy>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBClusterEndpointsResult")]
pub struct DBClusterEndpointMessage {
    #[serde(rename = "DBClusterEndpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_endpoints: Option<DBClusterEndpointList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBClusterEndpointList {
    #[serde(
        rename = "DBClusterEndpointList",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DBClusterEndpoint>,
}
impl From<Vec<DBClusterEndpoint>> for DBClusterEndpointList {
    fn from(v: Vec<DBClusterEndpoint>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBClusterEndpoint> for DBClusterEndpointList {
    fn from_iter<I: IntoIterator<Item = DBClusterEndpoint>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBClusterEndpoint>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBClusterEndpointList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBClusterEndpoint>,
}

impl From<Vec<DBClusterEndpoint>> for XmlDBClusterEndpointList {
    fn from(v: Vec<DBClusterEndpoint>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBClusterEndpoint> for XmlDBClusterEndpointList {
    fn from_iter<I: IntoIterator<Item = DBClusterEndpoint>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyDBClusterEndpointResult")]
pub struct DBClusterEndpoint {
    #[serde(rename = "CustomEndpointType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_endpoint_type: Option<String>,
    #[serde(rename = "DBClusterEndpointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_endpoint_arn: Option<String>,
    #[serde(rename = "DBClusterEndpointIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_endpoint_identifier: Option<String>,
    #[serde(rename = "DBClusterEndpointResourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_endpoint_resource_identifier: Option<String>,
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_identifier: Option<String>,
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "EndpointType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
    #[serde(rename = "ExcludedMembers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_members: Option<StringList>,
    #[serde(rename = "StaticMembers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_members: Option<StringList>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBProxyEndpointsRequest")]
pub struct DescribeDBProxyEndpointsRequest {
    #[serde(rename = "DBProxyEndpointName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_proxy_endpoint_name: Option<String>,
    #[serde(rename = "DBProxyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_proxy_name: Option<String>,
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
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CopyOptionGroupResult")]
pub struct CopyOptionGroupResult {
    #[serde(rename = "OptionGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group: Option<OptionGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDBProxyEndpointRequest")]
pub struct CreateDBProxyEndpointRequest {
    #[serde(rename = "DBProxyEndpointName")]
    #[serde(default)]
    pub d_b_proxy_endpoint_name: String,
    #[serde(rename = "DBProxyName")]
    #[serde(default)]
    pub d_b_proxy_name: String,
    #[serde(rename = "EndpointNetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_network_type: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "TargetRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_role: Option<String>,
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<StringList>,
    #[serde(rename = "VpcSubnetIds")]
    #[serde(default)]
    pub vpc_subnet_ids: StringList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBClusterAutomatedBackupsResult")]
pub struct DBClusterAutomatedBackupMessage {
    #[serde(rename = "DBClusterAutomatedBackups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_automated_backups: Option<DBClusterAutomatedBackupList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBClusterAutomatedBackupList {
    #[serde(
        rename = "DBClusterAutomatedBackup",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DBClusterAutomatedBackup>,
}
impl From<Vec<DBClusterAutomatedBackup>> for DBClusterAutomatedBackupList {
    fn from(v: Vec<DBClusterAutomatedBackup>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBClusterAutomatedBackup> for DBClusterAutomatedBackupList {
    fn from_iter<I: IntoIterator<Item = DBClusterAutomatedBackup>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBClusterAutomatedBackup>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBClusterAutomatedBackupList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBClusterAutomatedBackup>,
}

impl From<Vec<DBClusterAutomatedBackup>> for XmlDBClusterAutomatedBackupList {
    fn from(v: Vec<DBClusterAutomatedBackup>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBClusterAutomatedBackup> for XmlDBClusterAutomatedBackupList {
    fn from_iter<I: IntoIterator<Item = DBClusterAutomatedBackup>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBClusterAutomatedBackup")]
pub struct DBClusterAutomatedBackup {
    #[serde(rename = "AllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<AvailabilityZones>,
    #[serde(rename = "AwsBackupRecoveryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_backup_recovery_point_arn: Option<String>,
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i32>,
    #[serde(rename = "ClusterCreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_create_time: Option<String>,
    #[serde(rename = "DBClusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_arn: Option<String>,
    #[serde(rename = "DBClusterAutomatedBackupsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_automated_backups_arn: Option<String>,
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_identifier: Option<String>,
    #[serde(rename = "DbClusterResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_resource_id: Option<String>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_mode: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "IAMDatabaseAuthenticationEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_a_m_database_authentication_enabled: Option<bool>,
    #[serde(rename = "Iops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "LicenseModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_model: Option<String>,
    #[serde(rename = "MasterUsername")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_username: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "PreferredBackupWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "RestoreWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_window: Option<RestoreWindow>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StorageEncrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encrypted: Option<bool>,
    #[serde(rename = "StorageEncryptionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encryption_type: Option<String>,
    #[serde(rename = "StorageThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_throughput: Option<i32>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "TagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<TagList>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PromoteReadReplicaDBClusterMessage")]
pub struct PromoteReadReplicaDBClusterMessage {
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    pub d_b_cluster_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteDBShardGroupMessage")]
pub struct DeleteDBShardGroupMessage {
    #[serde(rename = "DBShardGroupIdentifier")]
    #[serde(default)]
    pub d_b_shard_group_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RebootDBShardGroupMessage")]
pub struct RebootDBShardGroupMessage {
    #[serde(rename = "DBShardGroupIdentifier")]
    #[serde(default)]
    pub d_b_shard_group_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteDBClusterMessage")]
pub struct DeleteDBClusterMessage {
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    pub d_b_cluster_identifier: String,
    #[serde(rename = "DeleteAutomatedBackups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_automated_backups: Option<bool>,
    #[serde(rename = "FinalDBSnapshotIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_d_b_snapshot_identifier: Option<String>,
    #[serde(rename = "SkipFinalSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_final_snapshot: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StartActivityStreamResult")]
pub struct StartActivityStreamResponse {
    #[serde(rename = "ApplyImmediately")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_immediately: Option<bool>,
    #[serde(rename = "EngineNativeAuditFieldsIncluded")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_native_audit_fields_included: Option<bool>,
    #[serde(rename = "KinesisStreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_stream_name: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "Mode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeIntegrationsResult")]
pub struct DescribeIntegrationsResponse {
    #[serde(rename = "Integrations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<IntegrationList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntegrationList {
    #[serde(rename = "Integration", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Integration>,
}
impl From<Vec<Integration>> for IntegrationList {
    fn from(v: Vec<Integration>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Integration> for IntegrationList {
    fn from_iter<I: IntoIterator<Item = Integration>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Integration>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlIntegrationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Integration>,
}

impl From<Vec<Integration>> for XmlIntegrationList {
    fn from(v: Vec<Integration>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Integration> for XmlIntegrationList {
    fn from_iter<I: IntoIterator<Item = Integration>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AddRoleToDBInstanceMessage")]
pub struct AddRoleToDBInstanceMessage {
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    pub d_b_instance_identifier: String,
    #[serde(rename = "FeatureName")]
    #[serde(default)]
    pub feature_name: String,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyDBProxyTargetGroupResult")]
pub struct ModifyDBProxyTargetGroupResponse {
    #[serde(rename = "DBProxyTargetGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_proxy_target_group: Option<DBProxyTargetGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteDBInstanceMessage")]
pub struct DeleteDBInstanceMessage {
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    pub d_b_instance_identifier: String,
    #[serde(rename = "DeleteAutomatedBackups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_automated_backups: Option<bool>,
    #[serde(rename = "FinalDBSnapshotIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_d_b_snapshot_identifier: Option<String>,
    #[serde(rename = "SkipFinalSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_final_snapshot: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeAccountAttributesResult")]
pub struct AccountAttributesMessage {
    #[serde(rename = "AccountQuotas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_quotas: Option<AccountQuotaList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountQuotaList {
    #[serde(
        rename = "AccountQuota",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<AccountQuota>,
}
impl From<Vec<AccountQuota>> for AccountQuotaList {
    fn from(v: Vec<AccountQuota>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<AccountQuota> for AccountQuotaList {
    fn from_iter<I: IntoIterator<Item = AccountQuota>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<AccountQuota>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlAccountQuotaList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<AccountQuota>,
}

impl From<Vec<AccountQuota>> for XmlAccountQuotaList {
    fn from(v: Vec<AccountQuota>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<AccountQuota> for XmlAccountQuotaList {
    fn from_iter<I: IntoIterator<Item = AccountQuota>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AccountQuota")]
pub struct AccountQuota {
    #[serde(rename = "AccountQuotaName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_quota_name: Option<String>,
    #[serde(rename = "Max")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i64>,
    #[serde(rename = "Used")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteDBSubnetGroupMessage")]
pub struct DeleteDBSubnetGroupMessage {
    #[serde(rename = "DBSubnetGroupName")]
    #[serde(default)]
    pub d_b_subnet_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateCustomDBEngineVersionMessage")]
pub struct CreateCustomDBEngineVersionMessage {
    #[serde(rename = "DatabaseInstallationFiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_installation_files: Option<StringList>,
    #[serde(rename = "DatabaseInstallationFilesS3BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_installation_files_s3_bucket_name: Option<String>,
    #[serde(rename = "DatabaseInstallationFilesS3Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_installation_files_s3_prefix: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    pub engine: String,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    pub engine_version: String,
    #[serde(rename = "ImageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "KMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_key_id: Option<String>,
    #[serde(rename = "Manifest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest: Option<String>,
    #[serde(rename = "SourceCustomDbEngineVersionIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_custom_db_engine_version_identifier: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "UseAwsProvidedLatestImage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_aws_provided_latest_image: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteDBProxyResult")]
pub struct DeleteDBProxyResponse {
    #[serde(rename = "DBProxy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_proxy: Option<DBProxy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyDBSubnetGroupMessage")]
pub struct ModifyDBSubnetGroupMessage {
    #[serde(rename = "DBSubnetGroupDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_subnet_group_description: Option<String>,
    #[serde(rename = "DBSubnetGroupName")]
    #[serde(default)]
    pub d_b_subnet_group_name: String,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    pub subnet_ids: SubnetIdentifierList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBRecommendationsMessage")]
pub struct DescribeDBRecommendationsMessage {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FilterList>,
    #[serde(rename = "LastUpdatedAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_after: Option<String>,
    #[serde(rename = "LastUpdatedBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_before: Option<String>,
    #[serde(rename = "Locale")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
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
#[serde(rename = "DescribeDBClustersMessage")]
pub struct DescribeDBClustersMessage {
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_identifier: Option<String>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FilterList>,
    #[serde(rename = "IncludeShared")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_shared: Option<bool>,
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
#[serde(rename = "ModifyDBClusterEndpointMessage")]
pub struct ModifyDBClusterEndpointMessage {
    #[serde(rename = "DBClusterEndpointIdentifier")]
    #[serde(default)]
    pub d_b_cluster_endpoint_identifier: String,
    #[serde(rename = "EndpointType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
    #[serde(rename = "ExcludedMembers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_members: Option<StringList>,
    #[serde(rename = "StaticMembers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_members: Option<StringList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyGlobalClusterResult")]
pub struct ModifyGlobalClusterResult {
    #[serde(rename = "GlobalCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_cluster: Option<GlobalCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyOptionGroupResult")]
pub struct ModifyOptionGroupResult {
    #[serde(rename = "OptionGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group: Option<OptionGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RestoreDBClusterFromSnapshotResult")]
pub struct RestoreDBClusterFromSnapshotResult {
    #[serde(rename = "DBCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster: Option<DBCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBClusterBacktracksMessage")]
pub struct DescribeDBClusterBacktracksMessage {
    #[serde(rename = "BacktrackIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backtrack_identifier: Option<String>,
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    pub d_b_cluster_identifier: String,
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
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBSecurityGroupsResult")]
pub struct DBSecurityGroupMessage {
    #[serde(rename = "DBSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_security_groups: Option<DBSecurityGroups>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBSecurityGroups {
    #[serde(
        rename = "DBSecurityGroup",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DBSecurityGroup>,
}
impl From<Vec<DBSecurityGroup>> for DBSecurityGroups {
    fn from(v: Vec<DBSecurityGroup>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBSecurityGroup> for DBSecurityGroups {
    fn from_iter<I: IntoIterator<Item = DBSecurityGroup>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBSecurityGroup>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBSecurityGroupList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBSecurityGroup>,
}

impl From<Vec<DBSecurityGroup>> for XmlDBSecurityGroupList {
    fn from(v: Vec<DBSecurityGroup>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBSecurityGroup> for XmlDBSecurityGroupList {
    fn from_iter<I: IntoIterator<Item = DBSecurityGroup>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBClusterParameterGroupsResult")]
pub struct DBClusterParameterGroupsMessage {
    #[serde(rename = "DBClusterParameterGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_parameter_groups: Option<DBClusterParameterGroupList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBClusterParameterGroupList {
    #[serde(
        rename = "DBClusterParameterGroup",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DBClusterParameterGroup>,
}
impl From<Vec<DBClusterParameterGroup>> for DBClusterParameterGroupList {
    fn from(v: Vec<DBClusterParameterGroup>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBClusterParameterGroup> for DBClusterParameterGroupList {
    fn from_iter<I: IntoIterator<Item = DBClusterParameterGroup>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBClusterParameterGroup>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBClusterParameterGroupList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBClusterParameterGroup>,
}

impl From<Vec<DBClusterParameterGroup>> for XmlDBClusterParameterGroupList {
    fn from(v: Vec<DBClusterParameterGroup>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBClusterParameterGroup> for XmlDBClusterParameterGroupList {
    fn from_iter<I: IntoIterator<Item = DBClusterParameterGroup>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBClusterParameterGroup")]
pub struct DBClusterParameterGroup {
    #[serde(rename = "DBClusterParameterGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_parameter_group_arn: Option<String>,
    #[serde(rename = "DBClusterParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_parameter_group_name: Option<String>,
    #[serde(rename = "DBParameterGroupFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_parameter_group_family: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyDBSnapshotAttributeResult")]
pub struct ModifyDBSnapshotAttributeResult {
    #[serde(rename = "DBSnapshotAttributesResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_snapshot_attributes_result: Option<DBSnapshotAttributesResult>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBSnapshotAttributesResult")]
pub struct DBSnapshotAttributesResult {
    #[serde(rename = "DBSnapshotAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_snapshot_attributes: Option<DBSnapshotAttributeList>,
    #[serde(rename = "DBSnapshotIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_snapshot_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBSnapshotAttributeList {
    #[serde(
        rename = "DBSnapshotAttribute",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DBSnapshotAttribute>,
}
impl From<Vec<DBSnapshotAttribute>> for DBSnapshotAttributeList {
    fn from(v: Vec<DBSnapshotAttribute>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBSnapshotAttribute> for DBSnapshotAttributeList {
    fn from_iter<I: IntoIterator<Item = DBSnapshotAttribute>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBSnapshotAttribute>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBSnapshotAttributeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBSnapshotAttribute>,
}

impl From<Vec<DBSnapshotAttribute>> for XmlDBSnapshotAttributeList {
    fn from(v: Vec<DBSnapshotAttribute>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBSnapshotAttribute> for XmlDBSnapshotAttributeList {
    fn from_iter<I: IntoIterator<Item = DBSnapshotAttribute>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBSnapshotAttribute")]
pub struct DBSnapshotAttribute {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[serde(rename = "AttributeValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_values: Option<AttributeValueList>,
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
#[serde(rename = "CreateDBInstanceResult")]
pub struct CreateDBInstanceResult {
    #[serde(rename = "DBInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance: Option<DBInstance>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBMajorEngineVersionsResult")]
pub struct DescribeDBMajorEngineVersionsResponse {
    #[serde(rename = "DBMajorEngineVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_major_engine_versions: Option<DBMajorEngineVersionsList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBMajorEngineVersionsList {
    #[serde(
        rename = "DBMajorEngineVersion",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DBMajorEngineVersion>,
}
impl From<Vec<DBMajorEngineVersion>> for DBMajorEngineVersionsList {
    fn from(v: Vec<DBMajorEngineVersion>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBMajorEngineVersion> for DBMajorEngineVersionsList {
    fn from_iter<I: IntoIterator<Item = DBMajorEngineVersion>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBMajorEngineVersion>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBMajorEngineVersionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBMajorEngineVersion>,
}

impl From<Vec<DBMajorEngineVersion>> for XmlDBMajorEngineVersionList {
    fn from(v: Vec<DBMajorEngineVersion>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBMajorEngineVersion> for XmlDBMajorEngineVersionList {
    fn from_iter<I: IntoIterator<Item = DBMajorEngineVersion>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBMajorEngineVersion")]
pub struct DBMajorEngineVersion {
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "MajorEngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major_engine_version: Option<String>,
    #[serde(rename = "SupportedEngineLifecycles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_engine_lifecycles: Option<SupportedEngineLifecycleList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SupportedEngineLifecycleList {
    #[serde(
        rename = "SupportedEngineLifecycle",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<SupportedEngineLifecycle>,
}
impl From<Vec<SupportedEngineLifecycle>> for SupportedEngineLifecycleList {
    fn from(v: Vec<SupportedEngineLifecycle>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<SupportedEngineLifecycle> for SupportedEngineLifecycleList {
    fn from_iter<I: IntoIterator<Item = SupportedEngineLifecycle>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<SupportedEngineLifecycle>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlSupportedEngineLifecycleList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<SupportedEngineLifecycle>,
}

impl From<Vec<SupportedEngineLifecycle>> for XmlSupportedEngineLifecycleList {
    fn from(v: Vec<SupportedEngineLifecycle>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<SupportedEngineLifecycle> for XmlSupportedEngineLifecycleList {
    fn from_iter<I: IntoIterator<Item = SupportedEngineLifecycle>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SupportedEngineLifecycle")]
pub struct SupportedEngineLifecycle {
    #[serde(rename = "LifecycleSupportEndDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_support_end_date: Option<String>,
    #[serde(rename = "LifecycleSupportName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_support_name: Option<String>,
    #[serde(rename = "LifecycleSupportStartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_support_start_date: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AddRoleToDBClusterMessage")]
pub struct AddRoleToDBClusterMessage {
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    pub d_b_cluster_identifier: String,
    #[serde(rename = "FeatureName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_name: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBProxyTargetsRequest")]
pub struct DescribeDBProxyTargetsRequest {
    #[serde(rename = "DBProxyName")]
    #[serde(default)]
    pub d_b_proxy_name: String,
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
    #[serde(rename = "TargetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyCustomDBEngineVersionResult")]
pub struct DBEngineVersion {
    #[serde(rename = "CreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(rename = "CustomDBEngineVersionManifest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_d_b_engine_version_manifest: Option<String>,
    #[serde(rename = "DBEngineDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_engine_description: Option<String>,
    #[serde(rename = "DBEngineMediaType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_engine_media_type: Option<String>,
    #[serde(rename = "DBEngineVersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_engine_version_arn: Option<String>,
    #[serde(rename = "DBEngineVersionDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_engine_version_description: Option<String>,
    #[serde(rename = "DBParameterGroupFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_parameter_group_family: Option<String>,
    #[serde(rename = "DatabaseInstallationFiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_installation_files: Option<StringList>,
    #[serde(rename = "DatabaseInstallationFilesS3BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_installation_files_s3_bucket_name: Option<String>,
    #[serde(rename = "DatabaseInstallationFilesS3Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_installation_files_s3_prefix: Option<String>,
    #[serde(rename = "DefaultCharacterSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_character_set: Option<CharacterSet>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "ExportableLogTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exportable_log_types: Option<LogTypeList>,
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "Image")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<CustomDBEngineVersionAMI>,
    #[serde(rename = "KMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_key_id: Option<String>,
    #[serde(rename = "MajorEngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major_engine_version: Option<String>,
    #[serde(rename = "ServerlessV2FeaturesSupport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless_v2_features_support: Option<ServerlessV2FeaturesSupport>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "SupportedCACertificateIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_c_a_certificate_identifiers: Option<CACertificateIdentifiersList>,
    #[serde(rename = "SupportedCharacterSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_character_sets: Option<SupportedCharacterSetsList>,
    #[serde(rename = "SupportedEngineModes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_engine_modes: Option<EngineModeList>,
    #[serde(rename = "SupportedFeatureNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_feature_names: Option<FeatureNameList>,
    #[serde(rename = "SupportedNcharCharacterSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_nchar_character_sets: Option<SupportedCharacterSetsList>,
    #[serde(rename = "SupportedTimezones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_timezones: Option<SupportedTimezonesList>,
    #[serde(rename = "SupportsBabelfish")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_babelfish: Option<bool>,
    #[serde(rename = "SupportsCertificateRotationWithoutRestart")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_certificate_rotation_without_restart: Option<bool>,
    #[serde(rename = "SupportsGlobalDatabases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_global_databases: Option<bool>,
    #[serde(rename = "SupportsIntegrations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_integrations: Option<bool>,
    #[serde(rename = "SupportsLimitlessDatabase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_limitless_database: Option<bool>,
    #[serde(rename = "SupportsLocalWriteForwarding")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_local_write_forwarding: Option<bool>,
    #[serde(rename = "SupportsLogExportsToCloudwatchLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_log_exports_to_cloudwatch_logs: Option<bool>,
    #[serde(rename = "SupportsParallelQuery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_parallel_query: Option<bool>,
    #[serde(rename = "SupportsReadReplica")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_read_replica: Option<bool>,
    #[serde(rename = "TagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<TagList>,
    #[serde(rename = "ValidUpgradeTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_upgrade_target: Option<ValidUpgradeTargetList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CharacterSet")]
pub struct CharacterSet {
    #[serde(rename = "CharacterSetDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_set_description: Option<String>,
    #[serde(rename = "CharacterSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_set_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CustomDBEngineVersionAMI")]
pub struct CustomDBEngineVersionAMI {
    #[serde(rename = "ImageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ServerlessV2FeaturesSupport")]
pub struct ServerlessV2FeaturesSupport {
    #[serde(rename = "MaxCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<f64>,
    #[serde(rename = "MinCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_capacity: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CACertificateIdentifiersList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for CACertificateIdentifiersList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for CACertificateIdentifiersList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SupportedCharacterSetsList {
    #[serde(
        rename = "CharacterSet",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<CharacterSet>,
}
impl From<Vec<CharacterSet>> for SupportedCharacterSetsList {
    fn from(v: Vec<CharacterSet>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<CharacterSet> for SupportedCharacterSetsList {
    fn from_iter<I: IntoIterator<Item = CharacterSet>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<CharacterSet>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlCharacterSetList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<CharacterSet>,
}

impl From<Vec<CharacterSet>> for XmlCharacterSetList {
    fn from(v: Vec<CharacterSet>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<CharacterSet> for XmlCharacterSetList {
    fn from_iter<I: IntoIterator<Item = CharacterSet>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FeatureNameList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for FeatureNameList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for FeatureNameList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SupportedTimezonesList {
    #[serde(rename = "Timezone", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Timezone>,
}
impl From<Vec<Timezone>> for SupportedTimezonesList {
    fn from(v: Vec<Timezone>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Timezone> for SupportedTimezonesList {
    fn from_iter<I: IntoIterator<Item = Timezone>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Timezone>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTimezoneList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Timezone>,
}

impl From<Vec<Timezone>> for XmlTimezoneList {
    fn from(v: Vec<Timezone>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Timezone> for XmlTimezoneList {
    fn from_iter<I: IntoIterator<Item = Timezone>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Timezone")]
pub struct Timezone {
    #[serde(rename = "TimezoneName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidUpgradeTargetList {
    #[serde(
        rename = "UpgradeTarget",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<UpgradeTarget>,
}
impl From<Vec<UpgradeTarget>> for ValidUpgradeTargetList {
    fn from(v: Vec<UpgradeTarget>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<UpgradeTarget> for ValidUpgradeTargetList {
    fn from_iter<I: IntoIterator<Item = UpgradeTarget>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<UpgradeTarget>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlUpgradeTargetList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<UpgradeTarget>,
}

impl From<Vec<UpgradeTarget>> for XmlUpgradeTargetList {
    fn from(v: Vec<UpgradeTarget>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<UpgradeTarget> for XmlUpgradeTargetList {
    fn from_iter<I: IntoIterator<Item = UpgradeTarget>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpgradeTarget")]
pub struct UpgradeTarget {
    #[serde(rename = "AutoUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_upgrade: Option<bool>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "IsMajorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_major_version_upgrade: Option<bool>,
    #[serde(rename = "SupportedEngineModes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_engine_modes: Option<EngineModeList>,
    #[serde(rename = "SupportsBabelfish")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_babelfish: Option<bool>,
    #[serde(rename = "SupportsGlobalDatabases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_global_databases: Option<bool>,
    #[serde(rename = "SupportsIntegrations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_integrations: Option<bool>,
    #[serde(rename = "SupportsLimitlessDatabase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_limitless_database: Option<bool>,
    #[serde(rename = "SupportsLocalWriteForwarding")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_local_write_forwarding: Option<bool>,
    #[serde(rename = "SupportsParallelQuery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_parallel_query: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBLogFilesResult")]
pub struct DescribeDBLogFilesResponse {
    #[serde(rename = "DescribeDBLogFiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub describe_d_b_log_files: Option<DescribeDBLogFilesList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDBLogFilesList {
    #[serde(
        rename = "DescribeDBLogFilesDetails",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DescribeDBLogFilesDetails>,
}
impl From<Vec<DescribeDBLogFilesDetails>> for DescribeDBLogFilesList {
    fn from(v: Vec<DescribeDBLogFilesDetails>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DescribeDBLogFilesDetails> for DescribeDBLogFilesList {
    fn from_iter<I: IntoIterator<Item = DescribeDBLogFilesDetails>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DescribeDBLogFilesDetails>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDescribeDBLogFilesDetailsList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DescribeDBLogFilesDetails>,
}

impl From<Vec<DescribeDBLogFilesDetails>> for XmlDescribeDBLogFilesDetailsList {
    fn from(v: Vec<DescribeDBLogFilesDetails>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DescribeDBLogFilesDetails> for XmlDescribeDBLogFilesDetailsList {
    fn from_iter<I: IntoIterator<Item = DescribeDBLogFilesDetails>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBLogFilesDetails")]
pub struct DescribeDBLogFilesDetails {
    #[serde(rename = "LastWritten")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_written: Option<i64>,
    #[serde(rename = "LogFileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_file_name: Option<String>,
    #[serde(rename = "Size")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyActivityStreamResult")]
pub struct ModifyActivityStreamResponse {
    #[serde(rename = "EngineNativeAuditFieldsIncluded")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_native_audit_fields_included: Option<bool>,
    #[serde(rename = "KinesisStreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_stream_name: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "Mode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "PolicyStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_status: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RebootDBClusterMessage")]
pub struct RebootDBClusterMessage {
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    pub d_b_cluster_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBClusterParametersResult")]
pub struct DBClusterParameterGroupDetails {
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
#[serde(rename = "CreateDBClusterSnapshotMessage")]
pub struct CreateDBClusterSnapshotMessage {
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    pub d_b_cluster_identifier: String,
    #[serde(rename = "DBClusterSnapshotIdentifier")]
    #[serde(default)]
    pub d_b_cluster_snapshot_identifier: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CopyDBClusterSnapshotResult")]
pub struct CopyDBClusterSnapshotResult {
    #[serde(rename = "DBClusterSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_snapshot: Option<DBClusterSnapshot>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyActivityStreamRequest")]
pub struct ModifyActivityStreamRequest {
    #[serde(rename = "AuditPolicyState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_policy_state: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StartActivityStreamRequest")]
pub struct StartActivityStreamRequest {
    #[serde(rename = "ApplyImmediately")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_immediately: Option<bool>,
    #[serde(rename = "EngineNativeAuditFieldsIncluded")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_native_audit_fields_included: Option<bool>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    pub kms_key_id: String,
    #[serde(rename = "Mode")]
    #[serde(default)]
    pub mode: String,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyDBClusterParameterGroupMessage")]
pub struct ModifyDBClusterParameterGroupMessage {
    #[serde(rename = "DBClusterParameterGroupName")]
    #[serde(default)]
    pub d_b_cluster_parameter_group_name: String,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    pub parameters: ParametersList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StartDBClusterMessage")]
pub struct StartDBClusterMessage {
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    pub d_b_cluster_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteDBInstanceAutomatedBackupMessage")]
pub struct DeleteDBInstanceAutomatedBackupMessage {
    #[serde(rename = "DBInstanceAutomatedBackupsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_automated_backups_arn: Option<String>,
    #[serde(rename = "DbiResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbi_resource_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeValidDBInstanceModificationsResult")]
pub struct DescribeValidDBInstanceModificationsResult {
    #[serde(rename = "ValidDBInstanceModificationsMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_d_b_instance_modifications_message: Option<ValidDBInstanceModificationsMessage>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ValidDBInstanceModificationsMessage")]
pub struct ValidDBInstanceModificationsMessage {
    #[serde(rename = "AdditionalStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_storage: Option<ValidAdditionalStorageOptions>,
    #[serde(rename = "Storage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<ValidStorageOptionsList>,
    #[serde(rename = "SupportsDedicatedLogVolume")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_dedicated_log_volume: Option<bool>,
    #[serde(rename = "ValidProcessorFeatures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_processor_features: Option<AvailableProcessorFeatureList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ValidAdditionalStorageOptions")]
pub struct ValidAdditionalStorageOptions {
    #[serde(rename = "SupportsAdditionalStorageVolumes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_additional_storage_volumes: Option<bool>,
    #[serde(rename = "Volumes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<ValidVolumeOptionsList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidVolumeOptionsList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ValidVolumeOptions>,
}
impl From<Vec<ValidVolumeOptions>> for ValidVolumeOptionsList {
    fn from(v: Vec<ValidVolumeOptions>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ValidVolumeOptions> for ValidVolumeOptionsList {
    fn from_iter<I: IntoIterator<Item = ValidVolumeOptions>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ValidVolumeOptions>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlValidVolumeOptionsList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ValidVolumeOptions>,
}

impl From<Vec<ValidVolumeOptions>> for XmlValidVolumeOptionsList {
    fn from(v: Vec<ValidVolumeOptions>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ValidVolumeOptions> for XmlValidVolumeOptionsList {
    fn from_iter<I: IntoIterator<Item = ValidVolumeOptions>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ValidVolumeOptions")]
pub struct ValidVolumeOptions {
    #[serde(rename = "Storage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<ValidStorageOptionsList>,
    #[serde(rename = "VolumeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidStorageOptionsList {
    #[serde(
        rename = "ValidStorageOptions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ValidStorageOptions>,
}
impl From<Vec<ValidStorageOptions>> for ValidStorageOptionsList {
    fn from(v: Vec<ValidStorageOptions>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ValidStorageOptions> for ValidStorageOptionsList {
    fn from_iter<I: IntoIterator<Item = ValidStorageOptions>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ValidStorageOptions>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlValidStorageOptionsList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ValidStorageOptions>,
}

impl From<Vec<ValidStorageOptions>> for XmlValidStorageOptionsList {
    fn from(v: Vec<ValidStorageOptions>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ValidStorageOptions> for XmlValidStorageOptionsList {
    fn from_iter<I: IntoIterator<Item = ValidStorageOptions>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ValidStorageOptions")]
pub struct ValidStorageOptions {
    #[serde(rename = "IopsToStorageRatio")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops_to_storage_ratio: Option<DoubleRangeList>,
    #[serde(rename = "ProvisionedIops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_iops: Option<RangeList>,
    #[serde(rename = "ProvisionedStorageThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_storage_throughput: Option<RangeList>,
    #[serde(rename = "StorageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_size: Option<RangeList>,
    #[serde(rename = "StorageThroughputToIopsRatio")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_throughput_to_iops_ratio: Option<DoubleRangeList>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "SupportsStorageAutoscaling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_storage_autoscaling: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DoubleRangeList {
    #[serde(rename = "DoubleRange", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<DoubleRange>,
}
impl From<Vec<DoubleRange>> for DoubleRangeList {
    fn from(v: Vec<DoubleRange>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DoubleRange> for DoubleRangeList {
    fn from_iter<I: IntoIterator<Item = DoubleRange>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DoubleRange>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDoubleRangeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DoubleRange>,
}

impl From<Vec<DoubleRange>> for XmlDoubleRangeList {
    fn from(v: Vec<DoubleRange>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DoubleRange> for XmlDoubleRangeList {
    fn from_iter<I: IntoIterator<Item = DoubleRange>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DoubleRange")]
pub struct DoubleRange {
    #[serde(rename = "From")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<f64>,
    #[serde(rename = "To")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RangeList {
    #[serde(rename = "Range", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Range>,
}
impl From<Vec<Range>> for RangeList {
    fn from(v: Vec<Range>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Range> for RangeList {
    fn from_iter<I: IntoIterator<Item = Range>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Range>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlRangeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Range>,
}

impl From<Vec<Range>> for XmlRangeList {
    fn from(v: Vec<Range>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Range> for XmlRangeList {
    fn from_iter<I: IntoIterator<Item = Range>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Range")]
pub struct Range {
    #[serde(rename = "From")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i32>,
    #[serde(rename = "Step")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<i32>,
    #[serde(rename = "To")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDBInstanceMessage")]
pub struct CreateDBInstanceMessage {
    #[serde(rename = "AdditionalStorageVolumes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_storage_volumes: Option<AdditionalStorageVolumesList>,
    #[serde(rename = "AllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i32>,
    #[serde(rename = "BackupTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_target: Option<String>,
    #[serde(rename = "CACertificateIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_a_certificate_identifier: Option<String>,
    #[serde(rename = "CharacterSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_set_name: Option<String>,
    #[serde(rename = "CopyTagsToSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_snapshot: Option<bool>,
    #[serde(rename = "CustomIamInstanceProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_iam_instance_profile: Option<String>,
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_identifier: Option<String>,
    #[serde(rename = "DBInstanceClass")]
    #[serde(default)]
    pub d_b_instance_class: String,
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    pub d_b_instance_identifier: String,
    #[serde(rename = "DBName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_name: Option<String>,
    #[serde(rename = "DBParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_parameter_group_name: Option<String>,
    #[serde(rename = "DBSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_security_groups: Option<DBSecurityGroupNameList>,
    #[serde(rename = "DBSubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_subnet_group_name: Option<String>,
    #[serde(rename = "DBSystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_system_id: Option<String>,
    #[serde(rename = "DatabaseInsightsMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_insights_mode: Option<String>,
    #[serde(rename = "DedicatedLogVolume")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_log_volume: Option<bool>,
    #[serde(rename = "DeletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "DomainAuthSecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_auth_secret_arn: Option<String>,
    #[serde(rename = "DomainDnsIps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_dns_ips: Option<StringList>,
    #[serde(rename = "DomainFqdn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_fqdn: Option<String>,
    #[serde(rename = "DomainIAMRoleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_i_a_m_role_name: Option<String>,
    #[serde(rename = "DomainOu")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_ou: Option<String>,
    #[serde(rename = "EnableCloudwatchLogsExports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_cloudwatch_logs_exports: Option<LogTypeList>,
    #[serde(rename = "EnableCustomerOwnedIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_customer_owned_ip: Option<bool>,
    #[serde(rename = "EnableIAMDatabaseAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_i_a_m_database_authentication: Option<bool>,
    #[serde(rename = "EnablePerformanceInsights")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_performance_insights: Option<bool>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    pub engine: String,
    #[serde(rename = "EngineLifecycleSupport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_lifecycle_support: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "Iops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "LicenseModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_model: Option<String>,
    #[serde(rename = "ManageMasterUserPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manage_master_user_password: Option<bool>,
    #[serde(rename = "MasterUserAuthenticationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_authentication_type: Option<String>,
    #[serde(rename = "MasterUserPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<String>,
    #[serde(rename = "MasterUserSecretKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_secret_kms_key_id: Option<String>,
    #[serde(rename = "MasterUsername")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_username: Option<String>,
    #[serde(rename = "MaxAllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_allocated_storage: Option<i32>,
    #[serde(rename = "MonitoringInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_interval: Option<i32>,
    #[serde(rename = "MonitoringRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_role_arn: Option<String>,
    #[serde(rename = "MultiAZ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_a_z: Option<bool>,
    #[serde(rename = "MultiTenant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_tenant: Option<bool>,
    #[serde(rename = "NcharCharacterSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nchar_character_set_name: Option<String>,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "OptionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_name: Option<String>,
    #[serde(rename = "PerformanceInsightsKMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_insights_k_m_s_key_id: Option<String>,
    #[serde(rename = "PerformanceInsightsRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_insights_retention_period: Option<i32>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "PreferredBackupWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[serde(rename = "ProcessorFeatures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_features: Option<ProcessorFeatureList>,
    #[serde(rename = "PromotionTier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_tier: Option<i32>,
    #[serde(rename = "PubliclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    #[serde(rename = "StorageEncrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encrypted: Option<bool>,
    #[serde(rename = "StorageThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_throughput: Option<i32>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "TagSpecifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_specifications: Option<TagSpecificationList>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "TdeCredentialArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tde_credential_arn: Option<String>,
    #[serde(rename = "TdeCredentialPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tde_credential_password: Option<String>,
    #[serde(rename = "Timezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<VpcSecurityGroupIdList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBSecurityGroupNameList {
    #[serde(
        rename = "DBSecurityGroupName",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for DBSecurityGroupNameList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for DBSecurityGroupNameList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyCertificatesMessage")]
pub struct ModifyCertificatesMessage {
    #[serde(rename = "CertificateIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_identifier: Option<String>,
    #[serde(rename = "RemoveCustomerOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_customer_override: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RestoreDBInstanceFromDBSnapshotResult")]
pub struct RestoreDBInstanceFromDBSnapshotResult {
    #[serde(rename = "DBInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance: Option<DBInstance>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEngineDefaultClusterParametersMessage")]
pub struct DescribeEngineDefaultClusterParametersMessage {
    #[serde(rename = "DBParameterGroupFamily")]
    #[serde(default)]
    pub d_b_parameter_group_family: String,
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
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResetDBClusterParameterGroupResult")]
pub struct DBClusterParameterGroupNameMessage {
    #[serde(rename = "DBClusterParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_parameter_group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeOrderableDBInstanceOptionsMessage")]
pub struct DescribeOrderableDBInstanceOptionsMessage {
    #[serde(rename = "AvailabilityZoneGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_group: Option<String>,
    #[serde(rename = "DBInstanceClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_class: Option<String>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    pub engine: String,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FilterList>,
    #[serde(rename = "LicenseModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_model: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "Vpc")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ApplyPendingMaintenanceActionResult")]
pub struct ApplyPendingMaintenanceActionResult {
    #[serde(rename = "ResourcePendingMaintenanceActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_pending_maintenance_actions: Option<ResourcePendingMaintenanceActions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResourcePendingMaintenanceActions")]
pub struct ResourcePendingMaintenanceActions {
    #[serde(rename = "PendingMaintenanceActionDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_maintenance_action_details: Option<PendingMaintenanceActionDetails>,
    #[serde(rename = "ResourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PendingMaintenanceActionDetails {
    #[serde(
        rename = "PendingMaintenanceAction",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<PendingMaintenanceAction>,
}
impl From<Vec<PendingMaintenanceAction>> for PendingMaintenanceActionDetails {
    fn from(v: Vec<PendingMaintenanceAction>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<PendingMaintenanceAction> for PendingMaintenanceActionDetails {
    fn from_iter<I: IntoIterator<Item = PendingMaintenanceAction>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<PendingMaintenanceAction>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlPendingMaintenanceActionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<PendingMaintenanceAction>,
}

impl From<Vec<PendingMaintenanceAction>> for XmlPendingMaintenanceActionList {
    fn from(v: Vec<PendingMaintenanceAction>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<PendingMaintenanceAction> for XmlPendingMaintenanceActionList {
    fn from_iter<I: IntoIterator<Item = PendingMaintenanceAction>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PendingMaintenanceAction")]
pub struct PendingMaintenanceAction {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "AutoAppliedAfterDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_applied_after_date: Option<String>,
    #[serde(rename = "CurrentApplyDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_apply_date: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ForcedApplyDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forced_apply_date: Option<String>,
    #[serde(rename = "OptInStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_in_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateEventSubscriptionResult")]
pub struct CreateEventSubscriptionResult {
    #[serde(rename = "EventSubscription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_subscription: Option<EventSubscription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateOptionGroupResult")]
pub struct CreateOptionGroupResult {
    #[serde(rename = "OptionGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group: Option<OptionGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeOptionGroupOptionsResult")]
pub struct OptionGroupOptionsMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "OptionGroupOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_options: Option<OptionGroupOptionsList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OptionGroupOptionsList {
    #[serde(
        rename = "OptionGroupOption",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<OptionGroupOption>,
}
impl From<Vec<OptionGroupOption>> for OptionGroupOptionsList {
    fn from(v: Vec<OptionGroupOption>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<OptionGroupOption> for OptionGroupOptionsList {
    fn from_iter<I: IntoIterator<Item = OptionGroupOption>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<OptionGroupOption>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlOptionGroupOptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<OptionGroupOption>,
}

impl From<Vec<OptionGroupOption>> for XmlOptionGroupOptionList {
    fn from(v: Vec<OptionGroupOption>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<OptionGroupOption> for XmlOptionGroupOptionList {
    fn from_iter<I: IntoIterator<Item = OptionGroupOption>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OptionGroupOption")]
pub struct OptionGroupOption {
    #[serde(rename = "CopyableCrossAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyable_cross_account: Option<bool>,
    #[serde(rename = "DefaultPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_port: Option<i32>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EngineName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_name: Option<String>,
    #[serde(rename = "MajorEngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major_engine_version: Option<String>,
    #[serde(rename = "MinimumRequiredMinorEngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_required_minor_engine_version: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OptionGroupOptionSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_option_settings: Option<OptionGroupOptionSettingsList>,
    #[serde(rename = "OptionGroupOptionVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_option_versions: Option<OptionGroupOptionVersionsList>,
    #[serde(rename = "OptionsConflictsWith")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options_conflicts_with: Option<OptionsConflictsWith>,
    #[serde(rename = "OptionsDependedOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options_depended_on: Option<OptionsDependedOn>,
    #[serde(rename = "Permanent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permanent: Option<bool>,
    #[serde(rename = "Persistent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent: Option<bool>,
    #[serde(rename = "PortRequired")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_required: Option<bool>,
    #[serde(rename = "RequiresAutoMinorEngineVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_auto_minor_engine_version_upgrade: Option<bool>,
    #[serde(rename = "SupportsOptionVersionDowngrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_option_version_downgrade: Option<bool>,
    #[serde(rename = "VpcOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_only: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OptionGroupOptionSettingsList {
    #[serde(
        rename = "OptionGroupOptionSetting",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<OptionGroupOptionSetting>,
}
impl From<Vec<OptionGroupOptionSetting>> for OptionGroupOptionSettingsList {
    fn from(v: Vec<OptionGroupOptionSetting>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<OptionGroupOptionSetting> for OptionGroupOptionSettingsList {
    fn from_iter<I: IntoIterator<Item = OptionGroupOptionSetting>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<OptionGroupOptionSetting>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlOptionGroupOptionSettingList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<OptionGroupOptionSetting>,
}

impl From<Vec<OptionGroupOptionSetting>> for XmlOptionGroupOptionSettingList {
    fn from(v: Vec<OptionGroupOptionSetting>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<OptionGroupOptionSetting> for XmlOptionGroupOptionSettingList {
    fn from_iter<I: IntoIterator<Item = OptionGroupOptionSetting>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OptionGroupOptionSetting")]
pub struct OptionGroupOptionSetting {
    #[serde(rename = "AllowedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<String>,
    #[serde(rename = "ApplyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_type: Option<String>,
    #[serde(rename = "DefaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "IsModifiable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_modifiable: Option<bool>,
    #[serde(rename = "IsRequired")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,
    #[serde(rename = "MinimumEngineVersionPerAllowedValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_engine_version_per_allowed_value: Option<MinimumEngineVersionPerAllowedValueList>,
    #[serde(rename = "SettingDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting_description: Option<String>,
    #[serde(rename = "SettingName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MinimumEngineVersionPerAllowedValueList {
    #[serde(
        rename = "MinimumEngineVersionPerAllowedValue",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<MinimumEngineVersionPerAllowedValue>,
}
impl From<Vec<MinimumEngineVersionPerAllowedValue>> for MinimumEngineVersionPerAllowedValueList {
    fn from(v: Vec<MinimumEngineVersionPerAllowedValue>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<MinimumEngineVersionPerAllowedValue> for MinimumEngineVersionPerAllowedValueList {
    fn from_iter<I: IntoIterator<Item = MinimumEngineVersionPerAllowedValue>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<MinimumEngineVersionPerAllowedValue>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlMinimumEngineVersionPerAllowedValueList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<MinimumEngineVersionPerAllowedValue>,
}

impl From<Vec<MinimumEngineVersionPerAllowedValue>> for XmlMinimumEngineVersionPerAllowedValueList {
    fn from(v: Vec<MinimumEngineVersionPerAllowedValue>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<MinimumEngineVersionPerAllowedValue>
    for XmlMinimumEngineVersionPerAllowedValueList
{
    fn from_iter<I: IntoIterator<Item = MinimumEngineVersionPerAllowedValue>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MinimumEngineVersionPerAllowedValue")]
pub struct MinimumEngineVersionPerAllowedValue {
    #[serde(rename = "AllowedValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_value: Option<String>,
    #[serde(rename = "MinimumEngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_engine_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OptionGroupOptionVersionsList {
    #[serde(
        rename = "OptionVersion",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<OptionVersion>,
}
impl From<Vec<OptionVersion>> for OptionGroupOptionVersionsList {
    fn from(v: Vec<OptionVersion>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<OptionVersion> for OptionGroupOptionVersionsList {
    fn from_iter<I: IntoIterator<Item = OptionVersion>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<OptionVersion>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlOptionVersionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<OptionVersion>,
}

impl From<Vec<OptionVersion>> for XmlOptionVersionList {
    fn from(v: Vec<OptionVersion>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<OptionVersion> for XmlOptionVersionList {
    fn from_iter<I: IntoIterator<Item = OptionVersion>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OptionVersion")]
pub struct OptionVersion {
    #[serde(rename = "IsDefault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OptionsConflictsWith {
    #[serde(
        rename = "OptionConflictName",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for OptionsConflictsWith {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for OptionsConflictsWith {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OptionsDependedOn {
    #[serde(rename = "OptionName", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for OptionsDependedOn {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for OptionsDependedOn {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyEventSubscriptionMessage")]
pub struct ModifyEventSubscriptionMessage {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "EventCategories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_categories: Option<EventCategoriesList>,
    #[serde(rename = "SnsTopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(rename = "SubscriptionName")]
    #[serde(default)]
    pub subscription_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteTenantDatabaseMessage")]
pub struct DeleteTenantDatabaseMessage {
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    pub d_b_instance_identifier: String,
    #[serde(rename = "FinalDBSnapshotIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_d_b_snapshot_identifier: Option<String>,
    #[serde(rename = "SkipFinalSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_final_snapshot: Option<bool>,
    #[serde(rename = "TenantDBName")]
    #[serde(default)]
    pub tenant_d_b_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDBClusterMessage")]
pub struct CreateDBClusterMessage {
    #[serde(rename = "AllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<AvailabilityZones>,
    #[serde(rename = "BacktrackWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backtrack_window: Option<i64>,
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i32>,
    #[serde(rename = "CACertificateIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_a_certificate_identifier: Option<String>,
    #[serde(rename = "CharacterSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_set_name: Option<String>,
    #[serde(rename = "ClusterScalabilityType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_scalability_type: Option<String>,
    #[serde(rename = "CopyTagsToSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_snapshot: Option<bool>,
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    pub d_b_cluster_identifier: String,
    #[serde(rename = "DBClusterInstanceClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_instance_class: Option<String>,
    #[serde(rename = "DBClusterParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_parameter_group_name: Option<String>,
    #[serde(rename = "DBSubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_subnet_group_name: Option<String>,
    #[serde(rename = "DBSystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_system_id: Option<String>,
    #[serde(rename = "DatabaseInsightsMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_insights_mode: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "DeletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "DomainIAMRoleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_i_a_m_role_name: Option<String>,
    #[serde(rename = "EnableCloudwatchLogsExports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_cloudwatch_logs_exports: Option<LogTypeList>,
    #[serde(rename = "EnableGlobalWriteForwarding")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_global_write_forwarding: Option<bool>,
    #[serde(rename = "EnableHttpEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_http_endpoint: Option<bool>,
    #[serde(rename = "EnableIAMDatabaseAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_i_a_m_database_authentication: Option<bool>,
    #[serde(rename = "EnableLimitlessDatabase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_limitless_database: Option<bool>,
    #[serde(rename = "EnableLocalWriteForwarding")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_local_write_forwarding: Option<bool>,
    #[serde(rename = "EnablePerformanceInsights")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_performance_insights: Option<bool>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    pub engine: String,
    #[serde(rename = "EngineLifecycleSupport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_lifecycle_support: Option<String>,
    #[serde(rename = "EngineMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_mode: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "GlobalClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_cluster_identifier: Option<String>,
    #[serde(rename = "Iops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "ManageMasterUserPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manage_master_user_password: Option<bool>,
    #[serde(rename = "MasterUserAuthenticationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_authentication_type: Option<String>,
    #[serde(rename = "MasterUserPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<String>,
    #[serde(rename = "MasterUserSecretKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_secret_kms_key_id: Option<String>,
    #[serde(rename = "MasterUsername")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_username: Option<String>,
    #[serde(rename = "MonitoringInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_interval: Option<i32>,
    #[serde(rename = "MonitoringRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_role_arn: Option<String>,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "OptionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_name: Option<String>,
    #[serde(rename = "PerformanceInsightsKMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_insights_k_m_s_key_id: Option<String>,
    #[serde(rename = "PerformanceInsightsRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_insights_retention_period: Option<i32>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "PreSignedUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_signed_url: Option<String>,
    #[serde(rename = "PreferredBackupWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[serde(rename = "PubliclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    #[serde(rename = "RdsCustomClusterConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_custom_cluster_configuration: Option<RdsCustomClusterConfiguration>,
    #[serde(rename = "ReplicationSourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_source_identifier: Option<String>,
    #[serde(rename = "ScalingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_configuration: Option<ScalingConfiguration>,
    #[serde(rename = "ServerlessV2ScalingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless_v2_scaling_configuration: Option<ServerlessV2ScalingConfiguration>,
    #[serde(rename = "StorageEncrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encrypted: Option<bool>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "TagSpecifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_specifications: Option<TagSpecificationList>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<VpcSecurityGroupIdList>,
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
#[serde(rename = "DescribeDBEngineVersionsMessage")]
pub struct DescribeDBEngineVersionsMessage {
    #[serde(rename = "DBParameterGroupFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_parameter_group_family: Option<String>,
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
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FilterList>,
    #[serde(rename = "IncludeAll")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_all: Option<bool>,
    #[serde(rename = "ListSupportedCharacterSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_supported_character_sets: Option<bool>,
    #[serde(rename = "ListSupportedTimezones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_supported_timezones: Option<bool>,
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
#[serde(rename = "ModifyDBInstanceMessage")]
pub struct ModifyDBInstanceMessage {
    #[serde(rename = "AdditionalStorageVolumes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_storage_volumes: Option<ModifyAdditionalStorageVolumesList>,
    #[serde(rename = "AllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(rename = "AllowMajorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_major_version_upgrade: Option<bool>,
    #[serde(rename = "ApplyImmediately")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_immediately: Option<bool>,
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    #[serde(rename = "AutomationMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation_mode: Option<String>,
    #[serde(rename = "AwsBackupRecoveryPointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_backup_recovery_point_arn: Option<String>,
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i32>,
    #[serde(rename = "CACertificateIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_a_certificate_identifier: Option<String>,
    #[serde(rename = "CertificateRotationRestart")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_rotation_restart: Option<bool>,
    #[serde(rename = "CloudwatchLogsExportConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudwatch_logs_export_configuration: Option<CloudwatchLogsExportConfiguration>,
    #[serde(rename = "CopyTagsToSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_snapshot: Option<bool>,
    #[serde(rename = "DBInstanceClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_class: Option<String>,
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    pub d_b_instance_identifier: String,
    #[serde(rename = "DBParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_parameter_group_name: Option<String>,
    #[serde(rename = "DBPortNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_port_number: Option<i32>,
    #[serde(rename = "DBSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_security_groups: Option<DBSecurityGroupNameList>,
    #[serde(rename = "DBSubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_subnet_group_name: Option<String>,
    #[serde(rename = "DatabaseInsightsMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_insights_mode: Option<String>,
    #[serde(rename = "DedicatedLogVolume")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_log_volume: Option<bool>,
    #[serde(rename = "DeletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    #[serde(rename = "DisableDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_domain: Option<bool>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "DomainAuthSecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_auth_secret_arn: Option<String>,
    #[serde(rename = "DomainDnsIps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_dns_ips: Option<StringList>,
    #[serde(rename = "DomainFqdn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_fqdn: Option<String>,
    #[serde(rename = "DomainIAMRoleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_i_a_m_role_name: Option<String>,
    #[serde(rename = "DomainOu")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_ou: Option<String>,
    #[serde(rename = "EnableCustomerOwnedIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_customer_owned_ip: Option<bool>,
    #[serde(rename = "EnableIAMDatabaseAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_i_a_m_database_authentication: Option<bool>,
    #[serde(rename = "EnablePerformanceInsights")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_performance_insights: Option<bool>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "Iops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    #[serde(rename = "LicenseModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_model: Option<String>,
    #[serde(rename = "ManageMasterUserPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manage_master_user_password: Option<bool>,
    #[serde(rename = "MasterUserAuthenticationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_authentication_type: Option<String>,
    #[serde(rename = "MasterUserPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<String>,
    #[serde(rename = "MasterUserSecretKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_secret_kms_key_id: Option<String>,
    #[serde(rename = "MaxAllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_allocated_storage: Option<i32>,
    #[serde(rename = "MonitoringInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_interval: Option<i32>,
    #[serde(rename = "MonitoringRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_role_arn: Option<String>,
    #[serde(rename = "MultiAZ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_a_z: Option<bool>,
    #[serde(rename = "MultiTenant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_tenant: Option<bool>,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "NewDBInstanceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_d_b_instance_identifier: Option<String>,
    #[serde(rename = "OptionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_name: Option<String>,
    #[serde(rename = "PerformanceInsightsKMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_insights_k_m_s_key_id: Option<String>,
    #[serde(rename = "PerformanceInsightsRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_insights_retention_period: Option<i32>,
    #[serde(rename = "PreferredBackupWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[serde(rename = "ProcessorFeatures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_features: Option<ProcessorFeatureList>,
    #[serde(rename = "PromotionTier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_tier: Option<i32>,
    #[serde(rename = "PubliclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    #[serde(rename = "ReplicaMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_mode: Option<String>,
    #[serde(rename = "ResumeFullAutomationModeMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resume_full_automation_mode_minutes: Option<i32>,
    #[serde(rename = "RotateMasterUserPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate_master_user_password: Option<bool>,
    #[serde(rename = "StorageThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_throughput: Option<i32>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "TagSpecifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_specifications: Option<TagSpecificationList>,
    #[serde(rename = "TdeCredentialArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tde_credential_arn: Option<String>,
    #[serde(rename = "TdeCredentialPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tde_credential_password: Option<String>,
    #[serde(rename = "UseDefaultProcessorFeatures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_default_processor_features: Option<bool>,
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<VpcSecurityGroupIdList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyAdditionalStorageVolumesList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ModifyAdditionalStorageVolume>,
}
impl From<Vec<ModifyAdditionalStorageVolume>> for ModifyAdditionalStorageVolumesList {
    fn from(v: Vec<ModifyAdditionalStorageVolume>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ModifyAdditionalStorageVolume> for ModifyAdditionalStorageVolumesList {
    fn from_iter<I: IntoIterator<Item = ModifyAdditionalStorageVolume>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ModifyAdditionalStorageVolume>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlModifyAdditionalStorageVolumeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ModifyAdditionalStorageVolume>,
}

impl From<Vec<ModifyAdditionalStorageVolume>> for XmlModifyAdditionalStorageVolumeList {
    fn from(v: Vec<ModifyAdditionalStorageVolume>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ModifyAdditionalStorageVolume> for XmlModifyAdditionalStorageVolumeList {
    fn from_iter<I: IntoIterator<Item = ModifyAdditionalStorageVolume>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyAdditionalStorageVolume")]
pub struct ModifyAdditionalStorageVolume {
    #[serde(rename = "AllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(rename = "IOPS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_o_p_s: Option<i32>,
    #[serde(rename = "MaxAllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_allocated_storage: Option<i32>,
    #[serde(rename = "SetForDelete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_for_delete: Option<bool>,
    #[serde(rename = "StorageThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_throughput: Option<i32>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "VolumeName")]
    #[serde(default)]
    pub volume_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StopActivityStreamResult")]
pub struct StopActivityStreamResponse {
    #[serde(rename = "KinesisStreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_stream_name: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyCertificatesResult")]
pub struct ModifyCertificatesResult {
    #[serde(rename = "Certificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<Certificate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Certificate")]
pub struct Certificate {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "CertificateIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_identifier: Option<String>,
    #[serde(rename = "CertificateType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_type: Option<String>,
    #[serde(rename = "CustomerOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_override: Option<bool>,
    #[serde(rename = "CustomerOverrideValidTill")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_override_valid_till: Option<String>,
    #[serde(rename = "Thumbprint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbprint: Option<String>,
    #[serde(rename = "ValidFrom")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<String>,
    #[serde(rename = "ValidTill")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_till: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteDBClusterAutomatedBackupMessage")]
pub struct DeleteDBClusterAutomatedBackupMessage {
    #[serde(rename = "DbClusterResourceId")]
    #[serde(default)]
    pub db_cluster_resource_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribePendingMaintenanceActionsMessage")]
pub struct DescribePendingMaintenanceActionsMessage {
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
    #[serde(rename = "ResourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeBlueGreenDeploymentsRequest")]
pub struct DescribeBlueGreenDeploymentsRequest {
    #[serde(rename = "BlueGreenDeploymentIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue_green_deployment_identifier: Option<String>,
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
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ApplyPendingMaintenanceActionMessage")]
pub struct ApplyPendingMaintenanceActionMessage {
    #[serde(rename = "ApplyAction")]
    #[serde(default)]
    pub apply_action: String,
    #[serde(rename = "OptInType")]
    #[serde(default)]
    pub opt_in_type: String,
    #[serde(rename = "ResourceIdentifier")]
    #[serde(default)]
    pub resource_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DisableHttpEndpointRequest")]
pub struct DisableHttpEndpointRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CopyDBParameterGroupMessage")]
pub struct CopyDBParameterGroupMessage {
    #[serde(rename = "SourceDBParameterGroupIdentifier")]
    #[serde(default)]
    pub source_d_b_parameter_group_identifier: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "TargetDBParameterGroupDescription")]
    #[serde(default)]
    pub target_d_b_parameter_group_description: String,
    #[serde(rename = "TargetDBParameterGroupIdentifier")]
    #[serde(default)]
    pub target_d_b_parameter_group_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeOptionGroupOptionsMessage")]
pub struct DescribeOptionGroupOptionsMessage {
    #[serde(rename = "EngineName")]
    #[serde(default)]
    pub engine_name: String,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FilterList>,
    #[serde(rename = "MajorEngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major_engine_version: Option<String>,
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
#[serde(rename = "ModifyCurrentDBClusterCapacityResult")]
pub struct DBClusterCapacityInfo {
    #[serde(rename = "CurrentCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_capacity: Option<i32>,
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_identifier: Option<String>,
    #[serde(rename = "PendingCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_capacity: Option<i32>,
    #[serde(rename = "SecondsBeforeTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds_before_timeout: Option<i32>,
    #[serde(rename = "TimeoutAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_action: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateTenantDatabaseResult")]
pub struct CreateTenantDatabaseResult {
    #[serde(rename = "TenantDatabase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_database: Option<TenantDatabase>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RemoveSourceIdentifierFromSubscriptionResult")]
pub struct RemoveSourceIdentifierFromSubscriptionResult {
    #[serde(rename = "EventSubscription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_subscription: Option<EventSubscription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBSecurityGroupsMessage")]
pub struct DescribeDBSecurityGroupsMessage {
    #[serde(rename = "DBSecurityGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_security_group_name: Option<String>,
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
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBClusterParameterGroupsMessage")]
pub struct DescribeDBClusterParameterGroupsMessage {
    #[serde(rename = "DBClusterParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_parameter_group_name: Option<String>,
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
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDBSecurityGroupMessage")]
pub struct CreateDBSecurityGroupMessage {
    #[serde(rename = "DBSecurityGroupDescription")]
    #[serde(default)]
    pub d_b_security_group_description: String,
    #[serde(rename = "DBSecurityGroupName")]
    #[serde(default)]
    pub d_b_security_group_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteDBProxyEndpointResult")]
pub struct DeleteDBProxyEndpointResponse {
    #[serde(rename = "DBProxyEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_proxy_endpoint: Option<DBProxyEndpoint>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBSubnetGroupsResult")]
pub struct DBSubnetGroupMessage {
    #[serde(rename = "DBSubnetGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_subnet_groups: Option<DBSubnetGroups>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBSubnetGroups {
    #[serde(
        rename = "DBSubnetGroup",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DBSubnetGroup>,
}
impl From<Vec<DBSubnetGroup>> for DBSubnetGroups {
    fn from(v: Vec<DBSubnetGroup>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBSubnetGroup> for DBSubnetGroups {
    fn from_iter<I: IntoIterator<Item = DBSubnetGroup>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBSubnetGroup>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBSubnetGroupList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBSubnetGroup>,
}

impl From<Vec<DBSubnetGroup>> for XmlDBSubnetGroupList {
    fn from(v: Vec<DBSubnetGroup>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBSubnetGroup> for XmlDBSubnetGroupList {
    fn from_iter<I: IntoIterator<Item = DBSubnetGroup>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyDBParameterGroupMessage")]
pub struct ModifyDBParameterGroupMessage {
    #[serde(rename = "DBParameterGroupName")]
    #[serde(default)]
    pub d_b_parameter_group_name: String,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    pub parameters: ParametersList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RevokeDBSecurityGroupIngressResult")]
pub struct RevokeDBSecurityGroupIngressResult {
    #[serde(rename = "DBSecurityGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_security_group: Option<DBSecurityGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBSnapshotsMessage")]
pub struct DescribeDBSnapshotsMessage {
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_identifier: Option<String>,
    #[serde(rename = "DBSnapshotIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_snapshot_identifier: Option<String>,
    #[serde(rename = "DbiResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbi_resource_id: Option<String>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FilterList>,
    #[serde(rename = "IncludePublic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_public: Option<bool>,
    #[serde(rename = "IncludeShared")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_shared: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "SnapshotType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBParameterGroupsMessage")]
pub struct DescribeDBParameterGroupsMessage {
    #[serde(rename = "DBParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_parameter_group_name: Option<String>,
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
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeCertificatesResult")]
pub struct CertificateMessage {
    #[serde(rename = "Certificates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates: Option<CertificateList>,
    #[serde(rename = "DefaultCertificateForNewLaunches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_certificate_for_new_launches: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CertificateList {
    #[serde(rename = "Certificate", default, skip_serializing_if = "Vec::is_empty")]
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
#[serde(rename = "ModifyDBRecommendationMessage")]
pub struct ModifyDBRecommendationMessage {
    #[serde(rename = "Locale")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(rename = "RecommendationId")]
    #[serde(default)]
    pub recommendation_id: String,
    #[serde(rename = "RecommendedActionUpdates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_action_updates: Option<RecommendedActionUpdateList>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecommendedActionUpdateList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<RecommendedActionUpdate>,
}
impl From<Vec<RecommendedActionUpdate>> for RecommendedActionUpdateList {
    fn from(v: Vec<RecommendedActionUpdate>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<RecommendedActionUpdate> for RecommendedActionUpdateList {
    fn from_iter<I: IntoIterator<Item = RecommendedActionUpdate>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<RecommendedActionUpdate>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlRecommendedActionUpdateList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<RecommendedActionUpdate>,
}

impl From<Vec<RecommendedActionUpdate>> for XmlRecommendedActionUpdateList {
    fn from(v: Vec<RecommendedActionUpdate>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<RecommendedActionUpdate> for XmlRecommendedActionUpdateList {
    fn from_iter<I: IntoIterator<Item = RecommendedActionUpdate>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RecommendedActionUpdate")]
pub struct RecommendedActionUpdate {
    #[serde(rename = "ActionId")]
    #[serde(default)]
    pub action_id: String,
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateGlobalClusterMessage")]
pub struct CreateGlobalClusterMessage {
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "DeletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineLifecycleSupport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_lifecycle_support: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "GlobalClusterIdentifier")]
    #[serde(default)]
    pub global_cluster_identifier: String,
    #[serde(rename = "SourceDBClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_d_b_cluster_identifier: Option<String>,
    #[serde(rename = "StorageEncrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encrypted: Option<bool>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDBInstanceReadReplicaMessage")]
pub struct CreateDBInstanceReadReplicaMessage {
    #[serde(rename = "AdditionalStorageVolumes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_storage_volumes: Option<AdditionalStorageVolumesList>,
    #[serde(rename = "AllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "BackupTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_target: Option<String>,
    #[serde(rename = "CACertificateIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_a_certificate_identifier: Option<String>,
    #[serde(rename = "CopyTagsToSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_snapshot: Option<bool>,
    #[serde(rename = "CustomIamInstanceProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_iam_instance_profile: Option<String>,
    #[serde(rename = "DBInstanceClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_class: Option<String>,
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    pub d_b_instance_identifier: String,
    #[serde(rename = "DBParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_parameter_group_name: Option<String>,
    #[serde(rename = "DBSubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_subnet_group_name: Option<String>,
    #[serde(rename = "DatabaseInsightsMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_insights_mode: Option<String>,
    #[serde(rename = "DedicatedLogVolume")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_log_volume: Option<bool>,
    #[serde(rename = "DeletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "DomainAuthSecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_auth_secret_arn: Option<String>,
    #[serde(rename = "DomainDnsIps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_dns_ips: Option<StringList>,
    #[serde(rename = "DomainFqdn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_fqdn: Option<String>,
    #[serde(rename = "DomainIAMRoleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_i_a_m_role_name: Option<String>,
    #[serde(rename = "DomainOu")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_ou: Option<String>,
    #[serde(rename = "EnableCloudwatchLogsExports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_cloudwatch_logs_exports: Option<LogTypeList>,
    #[serde(rename = "EnableCustomerOwnedIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_customer_owned_ip: Option<bool>,
    #[serde(rename = "EnableIAMDatabaseAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_i_a_m_database_authentication: Option<bool>,
    #[serde(rename = "EnablePerformanceInsights")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_performance_insights: Option<bool>,
    #[serde(rename = "Iops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "MaxAllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_allocated_storage: Option<i32>,
    #[serde(rename = "MonitoringInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_interval: Option<i32>,
    #[serde(rename = "MonitoringRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_role_arn: Option<String>,
    #[serde(rename = "MultiAZ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_a_z: Option<bool>,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "OptionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_name: Option<String>,
    #[serde(rename = "PerformanceInsightsKMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_insights_k_m_s_key_id: Option<String>,
    #[serde(rename = "PerformanceInsightsRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_insights_retention_period: Option<i32>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "PreSignedUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_signed_url: Option<String>,
    #[serde(rename = "ProcessorFeatures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_features: Option<ProcessorFeatureList>,
    #[serde(rename = "PubliclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    #[serde(rename = "ReplicaMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_mode: Option<String>,
    #[serde(rename = "SourceDBClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_d_b_cluster_identifier: Option<String>,
    #[serde(rename = "SourceDBInstanceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_d_b_instance_identifier: Option<String>,
    #[serde(rename = "StorageThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_throughput: Option<i32>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "TagSpecifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_specifications: Option<TagSpecificationList>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "UpgradeStorageConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_storage_config: Option<bool>,
    #[serde(rename = "UseDefaultProcessorFeatures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_default_processor_features: Option<bool>,
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<VpcSecurityGroupIdList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBSnapshotAttributesMessage")]
pub struct DescribeDBSnapshotAttributesMessage {
    #[serde(rename = "DBSnapshotIdentifier")]
    #[serde(default)]
    pub d_b_snapshot_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StopDBInstanceAutomatedBackupsReplicationMessage")]
pub struct StopDBInstanceAutomatedBackupsReplicationMessage {
    #[serde(rename = "SourceDBInstanceArn")]
    #[serde(default)]
    pub source_d_b_instance_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PurchaseReservedDBInstancesOfferingMessage")]
pub struct PurchaseReservedDBInstancesOfferingMessage {
    #[serde(rename = "DBInstanceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_count: Option<i32>,
    #[serde(rename = "ReservedDBInstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_d_b_instance_id: Option<String>,
    #[serde(rename = "ReservedDBInstancesOfferingId")]
    #[serde(default)]
    pub reserved_d_b_instances_offering_id: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDBSubnetGroupResult")]
pub struct CreateDBSubnetGroupResult {
    #[serde(rename = "DBSubnetGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_subnet_group: Option<DBSubnetGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeExportTasksResult")]
pub struct ExportTasksMessage {
    #[serde(rename = "ExportTasks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_tasks: Option<ExportTasksList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportTasksList {
    #[serde(rename = "ExportTask", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ExportTask>,
}
impl From<Vec<ExportTask>> for ExportTasksList {
    fn from(v: Vec<ExportTask>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ExportTask> for ExportTasksList {
    fn from_iter<I: IntoIterator<Item = ExportTask>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ExportTask>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlExportTaskList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ExportTask>,
}

impl From<Vec<ExportTask>> for XmlExportTaskList {
    fn from(v: Vec<ExportTask>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ExportTask> for XmlExportTaskList {
    fn from_iter<I: IntoIterator<Item = ExportTask>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StartExportTaskResult")]
pub struct ExportTask {
    #[serde(rename = "ExportOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_only: Option<StringList>,
    #[serde(rename = "ExportTaskIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_task_identifier: Option<String>,
    #[serde(rename = "FailureCause")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_cause: Option<String>,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "PercentProgress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_progress: Option<i32>,
    #[serde(rename = "S3Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket: Option<String>,
    #[serde(rename = "S3Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_prefix: Option<String>,
    #[serde(rename = "SnapshotTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_time: Option<String>,
    #[serde(rename = "SourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TaskEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_end_time: Option<String>,
    #[serde(rename = "TaskStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_start_time: Option<String>,
    #[serde(rename = "TotalExtractedDataInGB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_extracted_data_in_g_b: Option<i32>,
    #[serde(rename = "WarningMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeSourceRegionsResult")]
pub struct SourceRegionMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "SourceRegions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_regions: Option<SourceRegionList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceRegionList {
    #[serde(
        rename = "SourceRegion",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<SourceRegion>,
}
impl From<Vec<SourceRegion>> for SourceRegionList {
    fn from(v: Vec<SourceRegion>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<SourceRegion> for SourceRegionList {
    fn from_iter<I: IntoIterator<Item = SourceRegion>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<SourceRegion>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlSourceRegionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<SourceRegion>,
}

impl From<Vec<SourceRegion>> for XmlSourceRegionList {
    fn from(v: Vec<SourceRegion>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<SourceRegion> for XmlSourceRegionList {
    fn from_iter<I: IntoIterator<Item = SourceRegion>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SourceRegion")]
pub struct SourceRegion {
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "RegionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "SupportsDBInstanceAutomatedBackupsReplication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_d_b_instance_automated_backups_replication: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DisableHttpEndpointResult")]
pub struct DisableHttpEndpointResponse {
    #[serde(rename = "HttpEndpointEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_endpoint_enabled: Option<bool>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBSubnetGroupsMessage")]
pub struct DescribeDBSubnetGroupsMessage {
    #[serde(rename = "DBSubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_subnet_group_name: Option<String>,
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
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AddSourceIdentifierToSubscriptionResult")]
pub struct AddSourceIdentifierToSubscriptionResult {
    #[serde(rename = "EventSubscription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_subscription: Option<EventSubscription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyDBSnapshotResult")]
pub struct ModifyDBSnapshotResult {
    #[serde(rename = "DBSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_snapshot: Option<DBSnapshot>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBClusterBacktracksResult")]
pub struct DBClusterBacktrackMessage {
    #[serde(rename = "DBClusterBacktracks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_backtracks: Option<DBClusterBacktrackList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBClusterBacktrackList {
    #[serde(
        rename = "DBClusterBacktrack",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DBClusterBacktrack>,
}
impl From<Vec<DBClusterBacktrack>> for DBClusterBacktrackList {
    fn from(v: Vec<DBClusterBacktrack>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBClusterBacktrack> for DBClusterBacktrackList {
    fn from_iter<I: IntoIterator<Item = DBClusterBacktrack>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBClusterBacktrack>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBClusterBacktrackList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBClusterBacktrack>,
}

impl From<Vec<DBClusterBacktrack>> for XmlDBClusterBacktrackList {
    fn from(v: Vec<DBClusterBacktrack>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBClusterBacktrack> for XmlDBClusterBacktrackList {
    fn from_iter<I: IntoIterator<Item = DBClusterBacktrack>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "BacktrackDBClusterResult")]
pub struct DBClusterBacktrack {
    #[serde(rename = "BacktrackIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backtrack_identifier: Option<String>,
    #[serde(rename = "BacktrackRequestCreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backtrack_request_creation_time: Option<String>,
    #[serde(rename = "BacktrackTo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backtrack_to: Option<String>,
    #[serde(rename = "BacktrackedFrom")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backtracked_from: Option<String>,
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_identifier: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
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
#[serde(rename = "CreateDBProxyEndpointResult")]
pub struct CreateDBProxyEndpointResponse {
    #[serde(rename = "DBProxyEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_proxy_endpoint: Option<DBProxyEndpoint>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteDBClusterEndpointMessage")]
pub struct DeleteDBClusterEndpointMessage {
    #[serde(rename = "DBClusterEndpointIdentifier")]
    #[serde(default)]
    pub d_b_cluster_endpoint_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteEventSubscriptionMessage")]
pub struct DeleteEventSubscriptionMessage {
    #[serde(rename = "SubscriptionName")]
    #[serde(default)]
    pub subscription_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDBClusterParameterGroupResult")]
pub struct CreateDBClusterParameterGroupResult {
    #[serde(rename = "DBClusterParameterGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_parameter_group: Option<DBClusterParameterGroup>,
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
    #[serde(rename = "EventCategories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_categories: Option<EventCategoriesList>,
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
#[serde(rename = "DownloadDBLogFilePortionResult")]
pub struct DownloadDBLogFilePortionDetails {
    #[serde(rename = "AdditionalDataPending")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data_pending: Option<bool>,
    #[serde(rename = "LogFileData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_file_data: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyCurrentDBClusterCapacityMessage")]
pub struct ModifyCurrentDBClusterCapacityMessage {
    #[serde(rename = "Capacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<i32>,
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    pub d_b_cluster_identifier: String,
    #[serde(rename = "SecondsBeforeTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds_before_timeout: Option<i32>,
    #[serde(rename = "TimeoutAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_action: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CopyDBClusterParameterGroupMessage")]
pub struct CopyDBClusterParameterGroupMessage {
    #[serde(rename = "SourceDBClusterParameterGroupIdentifier")]
    #[serde(default)]
    pub source_d_b_cluster_parameter_group_identifier: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "TargetDBClusterParameterGroupDescription")]
    #[serde(default)]
    pub target_d_b_cluster_parameter_group_description: String,
    #[serde(rename = "TargetDBClusterParameterGroupIdentifier")]
    #[serde(default)]
    pub target_d_b_cluster_parameter_group_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PromoteReadReplicaDBClusterResult")]
pub struct PromoteReadReplicaDBClusterResult {
    #[serde(rename = "DBCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster: Option<DBCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBClusterEndpointsMessage")]
pub struct DescribeDBClusterEndpointsMessage {
    #[serde(rename = "DBClusterEndpointIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_endpoint_identifier: Option<String>,
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_identifier: Option<String>,
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
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteTenantDatabaseResult")]
pub struct DeleteTenantDatabaseResult {
    #[serde(rename = "TenantDatabase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_database: Option<TenantDatabase>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyDBProxyRequest")]
pub struct ModifyDBProxyRequest {
    #[serde(rename = "Auth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<UserAuthConfigList>,
    #[serde(rename = "DBProxyName")]
    #[serde(default)]
    pub d_b_proxy_name: String,
    #[serde(rename = "DebugLogging")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_logging: Option<bool>,
    #[serde(rename = "DefaultAuthScheme")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_auth_scheme: Option<String>,
    #[serde(rename = "IdleClientTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_client_timeout: Option<i32>,
    #[serde(rename = "NewDBProxyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_d_b_proxy_name: Option<String>,
    #[serde(rename = "RequireTLS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_t_l_s: Option<bool>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "SecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<StringList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBProxyTargetGroupsRequest")]
pub struct DescribeDBProxyTargetGroupsRequest {
    #[serde(rename = "DBProxyName")]
    #[serde(default)]
    pub d_b_proxy_name: String,
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
    #[serde(rename = "TargetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDBClusterSnapshotResult")]
pub struct CreateDBClusterSnapshotResult {
    #[serde(rename = "DBClusterSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_snapshot: Option<DBClusterSnapshot>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteDBProxyRequest")]
pub struct DeleteDBProxyRequest {
    #[serde(rename = "DBProxyName")]
    #[serde(default)]
    pub d_b_proxy_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyOptionGroupMessage")]
pub struct ModifyOptionGroupMessage {
    #[serde(rename = "ApplyImmediately")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_immediately: Option<bool>,
    #[serde(rename = "OptionGroupName")]
    #[serde(default)]
    pub option_group_name: String,
    #[serde(rename = "OptionsToInclude")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options_to_include: Option<OptionConfigurationList>,
    #[serde(rename = "OptionsToRemove")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options_to_remove: Option<OptionNamesList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OptionConfigurationList {
    #[serde(
        rename = "OptionConfiguration",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<OptionConfiguration>,
}
impl From<Vec<OptionConfiguration>> for OptionConfigurationList {
    fn from(v: Vec<OptionConfiguration>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<OptionConfiguration> for OptionConfigurationList {
    fn from_iter<I: IntoIterator<Item = OptionConfiguration>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<OptionConfiguration>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlOptionConfigurationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<OptionConfiguration>,
}

impl From<Vec<OptionConfiguration>> for XmlOptionConfigurationList {
    fn from(v: Vec<OptionConfiguration>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<OptionConfiguration> for XmlOptionConfigurationList {
    fn from_iter<I: IntoIterator<Item = OptionConfiguration>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OptionConfiguration")]
pub struct OptionConfiguration {
    #[serde(rename = "DBSecurityGroupMemberships")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_security_group_memberships: Option<DBSecurityGroupNameList>,
    #[serde(rename = "OptionName")]
    #[serde(default)]
    pub option_name: String,
    #[serde(rename = "OptionSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_settings: Option<OptionSettingsList>,
    #[serde(rename = "OptionVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_version: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "VpcSecurityGroupMemberships")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_memberships: Option<VpcSecurityGroupIdList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OptionSettingsList {
    #[serde(
        rename = "OptionSetting",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<OptionSetting>,
}
impl From<Vec<OptionSetting>> for OptionSettingsList {
    fn from(v: Vec<OptionSetting>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<OptionSetting> for OptionSettingsList {
    fn from_iter<I: IntoIterator<Item = OptionSetting>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OptionNamesList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for OptionNamesList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for OptionNamesList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeregisterDBProxyTargetsRequest")]
pub struct DeregisterDBProxyTargetsRequest {
    #[serde(rename = "DBClusterIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_identifiers: Option<StringList>,
    #[serde(rename = "DBInstanceIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_identifiers: Option<StringList>,
    #[serde(rename = "DBProxyName")]
    #[serde(default)]
    pub d_b_proxy_name: String,
    #[serde(rename = "TargetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEventSubscriptionsResult")]
pub struct EventSubscriptionsMessage {
    #[serde(rename = "EventSubscriptionsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_subscriptions_list: Option<EventSubscriptionsList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventSubscriptionsList {
    #[serde(
        rename = "EventSubscription",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<EventSubscription>,
}
impl From<Vec<EventSubscription>> for EventSubscriptionsList {
    fn from(v: Vec<EventSubscription>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<EventSubscription> for EventSubscriptionsList {
    fn from_iter<I: IntoIterator<Item = EventSubscription>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<EventSubscription>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlEventSubscriptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<EventSubscription>,
}

impl From<Vec<EventSubscription>> for XmlEventSubscriptionList {
    fn from(v: Vec<EventSubscription>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<EventSubscription> for XmlEventSubscriptionList {
    fn from_iter<I: IntoIterator<Item = EventSubscription>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StartDBInstanceResult")]
pub struct StartDBInstanceResult {
    #[serde(rename = "DBInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance: Option<DBInstance>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBShardGroupsResult")]
pub struct DescribeDBShardGroupsResponse {
    #[serde(rename = "DBShardGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_shard_groups: Option<DBShardGroupsList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBShardGroupsList {
    #[serde(
        rename = "DBShardGroup",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DBShardGroup>,
}
impl From<Vec<DBShardGroup>> for DBShardGroupsList {
    fn from(v: Vec<DBShardGroup>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBShardGroup> for DBShardGroupsList {
    fn from_iter<I: IntoIterator<Item = DBShardGroup>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBShardGroup>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBShardGroupList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBShardGroup>,
}

impl From<Vec<DBShardGroup>> for XmlDBShardGroupList {
    fn from(v: Vec<DBShardGroup>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBShardGroup> for XmlDBShardGroupList {
    fn from_iter<I: IntoIterator<Item = DBShardGroup>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteDBParameterGroupMessage")]
pub struct DeleteDBParameterGroupMessage {
    #[serde(rename = "DBParameterGroupName")]
    #[serde(default)]
    pub d_b_parameter_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteGlobalClusterResult")]
pub struct DeleteGlobalClusterResult {
    #[serde(rename = "GlobalCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_cluster: Option<GlobalCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CopyOptionGroupMessage")]
pub struct CopyOptionGroupMessage {
    #[serde(rename = "SourceOptionGroupIdentifier")]
    #[serde(default)]
    pub source_option_group_identifier: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "TargetOptionGroupDescription")]
    #[serde(default)]
    pub target_option_group_description: String,
    #[serde(rename = "TargetOptionGroupIdentifier")]
    #[serde(default)]
    pub target_option_group_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterDBProxyTargetsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBClusterAutomatedBackupsMessage")]
pub struct DescribeDBClusterAutomatedBackupsMessage {
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_identifier: Option<String>,
    #[serde(rename = "DbClusterResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_resource_id: Option<String>,
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
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBEngineVersionsResult")]
pub struct DBEngineVersionMessage {
    #[serde(rename = "DBEngineVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_engine_versions: Option<DBEngineVersionList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBEngineVersionList {
    #[serde(
        rename = "DBEngineVersion",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DBEngineVersion>,
}
impl From<Vec<DBEngineVersion>> for DBEngineVersionList {
    fn from(v: Vec<DBEngineVersion>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBEngineVersion> for DBEngineVersionList {
    fn from_iter<I: IntoIterator<Item = DBEngineVersion>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBEngineVersion>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBEngineVersionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBEngineVersion>,
}

impl From<Vec<DBEngineVersion>> for XmlDBEngineVersionList {
    fn from(v: Vec<DBEngineVersion>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBEngineVersion> for XmlDBEngineVersionList {
    fn from_iter<I: IntoIterator<Item = DBEngineVersion>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBInstanceAutomatedBackupsResult")]
pub struct DBInstanceAutomatedBackupMessage {
    #[serde(rename = "DBInstanceAutomatedBackups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_automated_backups: Option<DBInstanceAutomatedBackupList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBInstanceAutomatedBackupList {
    #[serde(
        rename = "DBInstanceAutomatedBackup",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DBInstanceAutomatedBackup>,
}
impl From<Vec<DBInstanceAutomatedBackup>> for DBInstanceAutomatedBackupList {
    fn from(v: Vec<DBInstanceAutomatedBackup>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBInstanceAutomatedBackup> for DBInstanceAutomatedBackupList {
    fn from_iter<I: IntoIterator<Item = DBInstanceAutomatedBackup>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBInstanceAutomatedBackup>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBInstanceAutomatedBackupList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBInstanceAutomatedBackup>,
}

impl From<Vec<DBInstanceAutomatedBackup>> for XmlDBInstanceAutomatedBackupList {
    fn from(v: Vec<DBInstanceAutomatedBackup>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBInstanceAutomatedBackup> for XmlDBInstanceAutomatedBackupList {
    fn from_iter<I: IntoIterator<Item = DBInstanceAutomatedBackup>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteDBClusterParameterGroupMessage")]
pub struct DeleteDBClusterParameterGroupMessage {
    #[serde(rename = "DBClusterParameterGroupName")]
    #[serde(default)]
    pub d_b_cluster_parameter_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBClustersResult")]
pub struct DBClusterMessage {
    #[serde(rename = "DBClusters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_clusters: Option<DBClusterList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBClusterList {
    #[serde(rename = "DBCluster", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<DBCluster>,
}
impl From<Vec<DBCluster>> for DBClusterList {
    fn from(v: Vec<DBCluster>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBCluster> for DBClusterList {
    fn from_iter<I: IntoIterator<Item = DBCluster>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBCluster>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBClusterList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBCluster>,
}

impl From<Vec<DBCluster>> for XmlDBClusterList {
    fn from(v: Vec<DBCluster>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBCluster> for XmlDBClusterList {
    fn from_iter<I: IntoIterator<Item = DBCluster>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyDBClusterSnapshotAttributeResult")]
pub struct ModifyDBClusterSnapshotAttributeResult {
    #[serde(rename = "DBClusterSnapshotAttributesResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_snapshot_attributes_result: Option<DBClusterSnapshotAttributesResult>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResetDBParameterGroupResult")]
pub struct DBParameterGroupNameMessage {
    #[serde(rename = "DBParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_parameter_group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyDBClusterResult")]
pub struct ModifyDBClusterResult {
    #[serde(rename = "DBCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster: Option<DBCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResetDBClusterParameterGroupMessage")]
pub struct ResetDBClusterParameterGroupMessage {
    #[serde(rename = "DBClusterParameterGroupName")]
    #[serde(default)]
    pub d_b_cluster_parameter_group_name: String,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParametersList>,
    #[serde(rename = "ResetAllParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_all_parameters: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteDBSnapshotMessage")]
pub struct DeleteDBSnapshotMessage {
    #[serde(rename = "DBSnapshotIdentifier")]
    #[serde(default)]
    pub d_b_snapshot_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RestoreDBClusterFromSnapshotMessage")]
pub struct RestoreDBClusterFromSnapshotMessage {
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<AvailabilityZones>,
    #[serde(rename = "BacktrackWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backtrack_window: Option<i64>,
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i32>,
    #[serde(rename = "CopyTagsToSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_snapshot: Option<bool>,
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    pub d_b_cluster_identifier: String,
    #[serde(rename = "DBClusterInstanceClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_instance_class: Option<String>,
    #[serde(rename = "DBClusterParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_parameter_group_name: Option<String>,
    #[serde(rename = "DBSubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_subnet_group_name: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "DeletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "DomainIAMRoleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_i_a_m_role_name: Option<String>,
    #[serde(rename = "EnableCloudwatchLogsExports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_cloudwatch_logs_exports: Option<LogTypeList>,
    #[serde(rename = "EnableIAMDatabaseAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_i_a_m_database_authentication: Option<bool>,
    #[serde(rename = "EnablePerformanceInsights")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_performance_insights: Option<bool>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    pub engine: String,
    #[serde(rename = "EngineLifecycleSupport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_lifecycle_support: Option<String>,
    #[serde(rename = "EngineMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_mode: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "Iops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "MonitoringInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_interval: Option<i32>,
    #[serde(rename = "MonitoringRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_role_arn: Option<String>,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "OptionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_name: Option<String>,
    #[serde(rename = "PerformanceInsightsKMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_insights_k_m_s_key_id: Option<String>,
    #[serde(rename = "PerformanceInsightsRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_insights_retention_period: Option<i32>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "PreferredBackupWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,
    #[serde(rename = "PubliclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    #[serde(rename = "RdsCustomClusterConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_custom_cluster_configuration: Option<RdsCustomClusterConfiguration>,
    #[serde(rename = "ScalingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_configuration: Option<ScalingConfiguration>,
    #[serde(rename = "ServerlessV2ScalingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless_v2_scaling_configuration: Option<ServerlessV2ScalingConfiguration>,
    #[serde(rename = "SnapshotIdentifier")]
    #[serde(default)]
    pub snapshot_identifier: String,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "TagSpecifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_specifications: Option<TagSpecificationList>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<VpcSecurityGroupIdList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeIntegrationsMessage")]
pub struct DescribeIntegrationsMessage {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FilterList>,
    #[serde(rename = "IntegrationIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_identifier: Option<String>,
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
#[serde(rename = "CreateDBSnapshotMessage")]
pub struct CreateDBSnapshotMessage {
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    pub d_b_instance_identifier: String,
    #[serde(rename = "DBSnapshotIdentifier")]
    #[serde(default)]
    pub d_b_snapshot_identifier: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteDBSnapshotResult")]
pub struct DeleteDBSnapshotResult {
    #[serde(rename = "DBSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_snapshot: Option<DBSnapshot>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AuthorizeDBSecurityGroupIngressMessage")]
pub struct AuthorizeDBSecurityGroupIngressMessage {
    #[serde(rename = "CIDRIP")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_i_d_r_i_p: Option<String>,
    #[serde(rename = "DBSecurityGroupName")]
    #[serde(default)]
    pub d_b_security_group_name: String,
    #[serde(rename = "EC2SecurityGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_c2_security_group_id: Option<String>,
    #[serde(rename = "EC2SecurityGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_c2_security_group_name: Option<String>,
    #[serde(rename = "EC2SecurityGroupOwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_c2_security_group_owner_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTagsForResourceResult")]
pub struct TagListMessage {
    #[serde(rename = "TagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyDBProxyTargetGroupRequest")]
pub struct ModifyDBProxyTargetGroupRequest {
    #[serde(rename = "ConnectionPoolConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_pool_config: Option<ConnectionPoolConfiguration>,
    #[serde(rename = "DBProxyName")]
    #[serde(default)]
    pub d_b_proxy_name: String,
    #[serde(rename = "NewName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_name: Option<String>,
    #[serde(rename = "TargetGroupName")]
    #[serde(default)]
    pub target_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ConnectionPoolConfiguration")]
pub struct ConnectionPoolConfiguration {
    #[serde(rename = "ConnectionBorrowTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_borrow_timeout: Option<i32>,
    #[serde(rename = "InitQuery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_query: Option<String>,
    #[serde(rename = "MaxConnectionsPercent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections_percent: Option<i32>,
    #[serde(rename = "MaxIdleConnectionsPercent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_idle_connections_percent: Option<i32>,
    #[serde(rename = "SessionPinningFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_pinning_filters: Option<StringList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PurchaseReservedDBInstancesOfferingResult")]
pub struct PurchaseReservedDBInstancesOfferingResult {
    #[serde(rename = "ReservedDBInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_d_b_instance: Option<ReservedDBInstance>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RegisterDBProxyTargetsRequest")]
pub struct RegisterDBProxyTargetsRequest {
    #[serde(rename = "DBClusterIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_identifiers: Option<StringList>,
    #[serde(rename = "DBInstanceIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_identifiers: Option<StringList>,
    #[serde(rename = "DBProxyName")]
    #[serde(default)]
    pub d_b_proxy_name: String,
    #[serde(rename = "TargetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateIntegrationMessage")]
pub struct CreateIntegrationMessage {
    #[serde(rename = "AdditionalEncryptionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_encryption_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "DataFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_filter: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "IntegrationName")]
    #[serde(default)]
    pub integration_name: String,
    #[serde(rename = "KMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_key_id: Option<String>,
    #[serde(rename = "SourceArn")]
    #[serde(default)]
    pub source_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "TargetArn")]
    #[serde(default)]
    pub target_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StopDBInstanceAutomatedBackupsReplicationResult")]
pub struct StopDBInstanceAutomatedBackupsReplicationResult {
    #[serde(rename = "DBInstanceAutomatedBackup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_automated_backup: Option<DBInstanceAutomatedBackup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBMajorEngineVersionsRequest")]
pub struct DescribeDBMajorEngineVersionsRequest {
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "MajorEngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major_engine_version: Option<String>,
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
#[serde(rename = "CreateDBSecurityGroupResult")]
pub struct CreateDBSecurityGroupResult {
    #[serde(rename = "DBSecurityGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_security_group: Option<DBSecurityGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBInstanceAutomatedBackupsMessage")]
pub struct DescribeDBInstanceAutomatedBackupsMessage {
    #[serde(rename = "DBInstanceAutomatedBackupsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_automated_backups_arn: Option<String>,
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_identifier: Option<String>,
    #[serde(rename = "DbiResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbi_resource_id: Option<String>,
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
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBSnapshotAttributesResult")]
pub struct DescribeDBSnapshotAttributesResult {
    #[serde(rename = "DBSnapshotAttributesResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_snapshot_attributes_result: Option<DBSnapshotAttributesResult>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeOptionGroupsMessage")]
pub struct DescribeOptionGroupsMessage {
    #[serde(rename = "EngineName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_name: Option<String>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FilterList>,
    #[serde(rename = "MajorEngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major_engine_version: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "OptionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTagsForResourceMessage")]
pub struct ListTagsForResourceMessage {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FilterList>,
    #[serde(rename = "ResourceName")]
    #[serde(default)]
    pub resource_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyGlobalClusterMessage")]
pub struct ModifyGlobalClusterMessage {
    #[serde(rename = "AllowMajorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_major_version_upgrade: Option<bool>,
    #[serde(rename = "DeletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "GlobalClusterIdentifier")]
    #[serde(default)]
    pub global_cluster_identifier: String,
    #[serde(rename = "NewGlobalClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_global_cluster_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBShardGroupsMessage")]
pub struct DescribeDBShardGroupsMessage {
    #[serde(rename = "DBShardGroupIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_shard_group_identifier: Option<String>,
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
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RemoveFromGlobalClusterResult")]
pub struct RemoveFromGlobalClusterResult {
    #[serde(rename = "GlobalCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_cluster: Option<GlobalCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateBlueGreenDeploymentRequest")]
pub struct CreateBlueGreenDeploymentRequest {
    #[serde(rename = "BlueGreenDeploymentName")]
    #[serde(default)]
    pub blue_green_deployment_name: String,
    #[serde(rename = "Source")]
    #[serde(default)]
    pub source: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "TargetAllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_allocated_storage: Option<i32>,
    #[serde(rename = "TargetDBClusterParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_d_b_cluster_parameter_group_name: Option<String>,
    #[serde(rename = "TargetDBInstanceClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_d_b_instance_class: Option<String>,
    #[serde(rename = "TargetDBParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_d_b_parameter_group_name: Option<String>,
    #[serde(rename = "TargetEngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_engine_version: Option<String>,
    #[serde(rename = "TargetIops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_iops: Option<i32>,
    #[serde(rename = "TargetStorageThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_storage_throughput: Option<i32>,
    #[serde(rename = "TargetStorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_storage_type: Option<String>,
    #[serde(rename = "UpgradeTargetStorageConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_target_storage_config: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeSourceRegionsMessage")]
pub struct DescribeSourceRegionsMessage {
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
    #[serde(rename = "RegionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RestoreDBInstanceFromS3Result")]
pub struct RestoreDBInstanceFromS3Result {
    #[serde(rename = "DBInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance: Option<DBInstance>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StopActivityStreamRequest")]
pub struct StopActivityStreamRequest {
    #[serde(rename = "ApplyImmediately")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_immediately: Option<bool>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDBClusterResult")]
pub struct CreateDBClusterResult {
    #[serde(rename = "DBCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster: Option<DBCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateBlueGreenDeploymentResult")]
pub struct CreateBlueGreenDeploymentResponse {
    #[serde(rename = "BlueGreenDeployment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue_green_deployment: Option<BlueGreenDeployment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAccountAttributesMessage {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateGlobalClusterResult")]
pub struct CreateGlobalClusterResult {
    #[serde(rename = "GlobalCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_cluster: Option<GlobalCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CopyDBClusterSnapshotMessage")]
pub struct CopyDBClusterSnapshotMessage {
    #[serde(rename = "CopyTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags: Option<bool>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "PreSignedUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_signed_url: Option<String>,
    #[serde(rename = "SourceDBClusterSnapshotIdentifier")]
    #[serde(default)]
    pub source_d_b_cluster_snapshot_identifier: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "TargetDBClusterSnapshotIdentifier")]
    #[serde(default)]
    pub target_d_b_cluster_snapshot_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribePendingMaintenanceActionsResult")]
pub struct PendingMaintenanceActionsMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "PendingMaintenanceActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_maintenance_actions: Option<PendingMaintenanceActions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PendingMaintenanceActions {
    #[serde(
        rename = "ResourcePendingMaintenanceActions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ResourcePendingMaintenanceActions>,
}
impl From<Vec<ResourcePendingMaintenanceActions>> for PendingMaintenanceActions {
    fn from(v: Vec<ResourcePendingMaintenanceActions>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ResourcePendingMaintenanceActions> for PendingMaintenanceActions {
    fn from_iter<I: IntoIterator<Item = ResourcePendingMaintenanceActions>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ResourcePendingMaintenanceActions>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlResourcePendingMaintenanceActionsList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ResourcePendingMaintenanceActions>,
}

impl From<Vec<ResourcePendingMaintenanceActions>> for XmlResourcePendingMaintenanceActionsList {
    fn from(v: Vec<ResourcePendingMaintenanceActions>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ResourcePendingMaintenanceActions> for XmlResourcePendingMaintenanceActionsList {
    fn from_iter<I: IntoIterator<Item = ResourcePendingMaintenanceActions>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeTenantDatabasesResult")]
pub struct TenantDatabasesMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "TenantDatabases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_databases: Option<TenantDatabasesList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TenantDatabasesList {
    #[serde(
        rename = "TenantDatabase",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<TenantDatabase>,
}
impl From<Vec<TenantDatabase>> for TenantDatabasesList {
    fn from(v: Vec<TenantDatabase>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<TenantDatabase> for TenantDatabasesList {
    fn from_iter<I: IntoIterator<Item = TenantDatabase>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<TenantDatabase>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTenantDatabaseList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<TenantDatabase>,
}

impl From<Vec<TenantDatabase>> for XmlTenantDatabaseList {
    fn from(v: Vec<TenantDatabase>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<TenantDatabase> for XmlTenantDatabaseList {
    fn from_iter<I: IntoIterator<Item = TenantDatabase>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteDBClusterSnapshotResult")]
pub struct DeleteDBClusterSnapshotResult {
    #[serde(rename = "DBClusterSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_snapshot: Option<DBClusterSnapshot>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteDBClusterResult")]
pub struct DeleteDBClusterResult {
    #[serde(rename = "DBCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster: Option<DBCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RemoveFromGlobalClusterMessage")]
pub struct RemoveFromGlobalClusterMessage {
    #[serde(rename = "DbClusterIdentifier")]
    #[serde(default)]
    pub db_cluster_identifier: String,
    #[serde(rename = "GlobalClusterIdentifier")]
    #[serde(default)]
    pub global_cluster_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBRecommendationsResult")]
pub struct DBRecommendationsMessage {
    #[serde(rename = "DBRecommendations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_recommendations: Option<DBRecommendationList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBRecommendationList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<DBRecommendation>,
}
impl From<Vec<DBRecommendation>> for DBRecommendationList {
    fn from(v: Vec<DBRecommendation>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBRecommendation> for DBRecommendationList {
    fn from_iter<I: IntoIterator<Item = DBRecommendation>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBRecommendation>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBRecommendationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBRecommendation>,
}

impl From<Vec<DBRecommendation>> for XmlDBRecommendationList {
    fn from(v: Vec<DBRecommendation>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBRecommendation> for XmlDBRecommendationList {
    fn from_iter<I: IntoIterator<Item = DBRecommendation>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBRecommendation")]
pub struct DBRecommendation {
    #[serde(rename = "AdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
    #[serde(rename = "Category")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Detection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detection: Option<String>,
    #[serde(rename = "Impact")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact: Option<String>,
    #[serde(rename = "IssueDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_details: Option<IssueDetails>,
    #[serde(rename = "Links")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<DocLinkList>,
    #[serde(rename = "Reason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "Recommendation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation: Option<String>,
    #[serde(rename = "RecommendationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_id: Option<String>,
    #[serde(rename = "RecommendedActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_actions: Option<RecommendedActionList>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "Severity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TypeDetection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_detection: Option<String>,
    #[serde(rename = "TypeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_id: Option<String>,
    #[serde(rename = "TypeRecommendation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_recommendation: Option<String>,
    #[serde(rename = "UpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "IssueDetails")]
pub struct IssueDetails {
    #[serde(rename = "PerformanceIssueDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_issue_details: Option<PerformanceIssueDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PerformanceIssueDetails")]
pub struct PerformanceIssueDetails {
    #[serde(rename = "Analysis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "Metrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<MetricList>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Metric>,
}
impl From<Vec<Metric>> for MetricList {
    fn from(v: Vec<Metric>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Metric> for MetricList {
    fn from_iter<I: IntoIterator<Item = Metric>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Metric>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlMetricList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Metric>,
}

impl From<Vec<Metric>> for XmlMetricList {
    fn from(v: Vec<Metric>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Metric> for XmlMetricList {
    fn from_iter<I: IntoIterator<Item = Metric>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Metric")]
pub struct Metric {
    #[serde(rename = "MetricQuery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_query: Option<MetricQuery>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "References")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub references: Option<MetricReferenceList>,
    #[serde(rename = "StatisticsDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics_details: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MetricQuery")]
pub struct MetricQuery {
    #[serde(rename = "PerformanceInsightsMetricQuery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_insights_metric_query: Option<PerformanceInsightsMetricQuery>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PerformanceInsightsMetricQuery")]
pub struct PerformanceInsightsMetricQuery {
    #[serde(rename = "GroupBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<PerformanceInsightsMetricDimensionGroup>,
    #[serde(rename = "Metric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PerformanceInsightsMetricDimensionGroup")]
pub struct PerformanceInsightsMetricDimensionGroup {
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<StringList>,
    #[serde(rename = "Group")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricReferenceList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<MetricReference>,
}
impl From<Vec<MetricReference>> for MetricReferenceList {
    fn from(v: Vec<MetricReference>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<MetricReference> for MetricReferenceList {
    fn from_iter<I: IntoIterator<Item = MetricReference>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<MetricReference>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlMetricReferenceList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<MetricReference>,
}

impl From<Vec<MetricReference>> for XmlMetricReferenceList {
    fn from(v: Vec<MetricReference>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<MetricReference> for XmlMetricReferenceList {
    fn from_iter<I: IntoIterator<Item = MetricReference>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MetricReference")]
pub struct MetricReference {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ReferenceDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_details: Option<ReferenceDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ReferenceDetails")]
pub struct ReferenceDetails {
    #[serde(rename = "ScalarReferenceDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalar_reference_details: Option<ScalarReferenceDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ScalarReferenceDetails")]
pub struct ScalarReferenceDetails {
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocLinkList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<DocLink>,
}
impl From<Vec<DocLink>> for DocLinkList {
    fn from(v: Vec<DocLink>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DocLink> for DocLinkList {
    fn from_iter<I: IntoIterator<Item = DocLink>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DocLink>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDocLinkList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DocLink>,
}

impl From<Vec<DocLink>> for XmlDocLinkList {
    fn from(v: Vec<DocLink>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DocLink> for XmlDocLinkList {
    fn from_iter<I: IntoIterator<Item = DocLink>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DocLink")]
pub struct DocLink {
    #[serde(rename = "Text")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecommendedActionList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<RecommendedAction>,
}
impl From<Vec<RecommendedAction>> for RecommendedActionList {
    fn from(v: Vec<RecommendedAction>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<RecommendedAction> for RecommendedActionList {
    fn from_iter<I: IntoIterator<Item = RecommendedAction>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<RecommendedAction>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlRecommendedActionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<RecommendedAction>,
}

impl From<Vec<RecommendedAction>> for XmlRecommendedActionList {
    fn from(v: Vec<RecommendedAction>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<RecommendedAction> for XmlRecommendedActionList {
    fn from_iter<I: IntoIterator<Item = RecommendedAction>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RecommendedAction")]
pub struct RecommendedAction {
    #[serde(rename = "ActionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(rename = "ApplyModes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_modes: Option<StringList>,
    #[serde(rename = "ContextAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_attributes: Option<ContextAttributeList>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "IssueDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_details: Option<IssueDetails>,
    #[serde(rename = "Operation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<RecommendedActionParameterList>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContextAttributeList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ContextAttribute>,
}
impl From<Vec<ContextAttribute>> for ContextAttributeList {
    fn from(v: Vec<ContextAttribute>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ContextAttribute> for ContextAttributeList {
    fn from_iter<I: IntoIterator<Item = ContextAttribute>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ContextAttribute>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlContextAttributeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ContextAttribute>,
}

impl From<Vec<ContextAttribute>> for XmlContextAttributeList {
    fn from(v: Vec<ContextAttribute>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ContextAttribute> for XmlContextAttributeList {
    fn from_iter<I: IntoIterator<Item = ContextAttribute>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ContextAttribute")]
pub struct ContextAttribute {
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
pub struct RecommendedActionParameterList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<RecommendedActionParameter>,
}
impl From<Vec<RecommendedActionParameter>> for RecommendedActionParameterList {
    fn from(v: Vec<RecommendedActionParameter>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<RecommendedActionParameter> for RecommendedActionParameterList {
    fn from_iter<I: IntoIterator<Item = RecommendedActionParameter>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<RecommendedActionParameter>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlRecommendedActionParameterList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<RecommendedActionParameter>,
}

impl From<Vec<RecommendedActionParameter>> for XmlRecommendedActionParameterList {
    fn from(v: Vec<RecommendedActionParameter>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<RecommendedActionParameter> for XmlRecommendedActionParameterList {
    fn from_iter<I: IntoIterator<Item = RecommendedActionParameter>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RecommendedActionParameter")]
pub struct RecommendedActionParameter {
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
#[serde(rename = "StartDBInstanceAutomatedBackupsReplicationMessage")]
pub struct StartDBInstanceAutomatedBackupsReplicationMessage {
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i32>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "PreSignedUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_signed_url: Option<String>,
    #[serde(rename = "SourceDBInstanceArn")]
    #[serde(default)]
    pub source_d_b_instance_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StopDBInstanceResult")]
pub struct StopDBInstanceResult {
    #[serde(rename = "DBInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance: Option<DBInstance>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEventSubscriptionsMessage")]
pub struct DescribeEventSubscriptionsMessage {
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
    #[serde(rename = "SubscriptionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CopyDBClusterParameterGroupResult")]
pub struct CopyDBClusterParameterGroupResult {
    #[serde(rename = "DBClusterParameterGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_parameter_group: Option<DBClusterParameterGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RestoreDBInstanceFromS3Message")]
pub struct RestoreDBInstanceFromS3Message {
    #[serde(rename = "AdditionalStorageVolumes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_storage_volumes: Option<AdditionalStorageVolumesList>,
    #[serde(rename = "AllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i32>,
    #[serde(rename = "CACertificateIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_a_certificate_identifier: Option<String>,
    #[serde(rename = "CopyTagsToSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_snapshot: Option<bool>,
    #[serde(rename = "DBInstanceClass")]
    #[serde(default)]
    pub d_b_instance_class: String,
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    pub d_b_instance_identifier: String,
    #[serde(rename = "DBName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_name: Option<String>,
    #[serde(rename = "DBParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_parameter_group_name: Option<String>,
    #[serde(rename = "DBSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_security_groups: Option<DBSecurityGroupNameList>,
    #[serde(rename = "DBSubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_subnet_group_name: Option<String>,
    #[serde(rename = "DatabaseInsightsMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_insights_mode: Option<String>,
    #[serde(rename = "DedicatedLogVolume")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_log_volume: Option<bool>,
    #[serde(rename = "DeletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    #[serde(rename = "EnableCloudwatchLogsExports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_cloudwatch_logs_exports: Option<LogTypeList>,
    #[serde(rename = "EnableIAMDatabaseAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_i_a_m_database_authentication: Option<bool>,
    #[serde(rename = "EnablePerformanceInsights")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_performance_insights: Option<bool>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    pub engine: String,
    #[serde(rename = "EngineLifecycleSupport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_lifecycle_support: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "Iops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "LicenseModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_model: Option<String>,
    #[serde(rename = "ManageMasterUserPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manage_master_user_password: Option<bool>,
    #[serde(rename = "MasterUserPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<String>,
    #[serde(rename = "MasterUserSecretKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_secret_kms_key_id: Option<String>,
    #[serde(rename = "MasterUsername")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_username: Option<String>,
    #[serde(rename = "MaxAllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_allocated_storage: Option<i32>,
    #[serde(rename = "MonitoringInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_interval: Option<i32>,
    #[serde(rename = "MonitoringRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_role_arn: Option<String>,
    #[serde(rename = "MultiAZ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_a_z: Option<bool>,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "OptionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_name: Option<String>,
    #[serde(rename = "PerformanceInsightsKMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_insights_k_m_s_key_id: Option<String>,
    #[serde(rename = "PerformanceInsightsRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_insights_retention_period: Option<i32>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "PreferredBackupWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[serde(rename = "ProcessorFeatures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_features: Option<ProcessorFeatureList>,
    #[serde(rename = "PubliclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    #[serde(rename = "S3BucketName")]
    #[serde(default)]
    pub s3_bucket_name: String,
    #[serde(rename = "S3IngestionRoleArn")]
    #[serde(default)]
    pub s3_ingestion_role_arn: String,
    #[serde(rename = "S3Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_prefix: Option<String>,
    #[serde(rename = "SourceEngine")]
    #[serde(default)]
    pub source_engine: String,
    #[serde(rename = "SourceEngineVersion")]
    #[serde(default)]
    pub source_engine_version: String,
    #[serde(rename = "StorageEncrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encrypted: Option<bool>,
    #[serde(rename = "StorageThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_throughput: Option<i32>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "TagSpecifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_specifications: Option<TagSpecificationList>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "UseDefaultProcessorFeatures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_default_processor_features: Option<bool>,
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<VpcSecurityGroupIdList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StartDBClusterResult")]
pub struct StartDBClusterResult {
    #[serde(rename = "DBCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster: Option<DBCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StartExportTaskMessage")]
pub struct StartExportTaskMessage {
    #[serde(rename = "ExportOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_only: Option<StringList>,
    #[serde(rename = "ExportTaskIdentifier")]
    #[serde(default)]
    pub export_task_identifier: String,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    pub iam_role_arn: String,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    pub kms_key_id: String,
    #[serde(rename = "S3BucketName")]
    #[serde(default)]
    pub s3_bucket_name: String,
    #[serde(rename = "S3Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_prefix: Option<String>,
    #[serde(rename = "SourceArn")]
    #[serde(default)]
    pub source_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteOptionGroupMessage")]
pub struct DeleteOptionGroupMessage {
    #[serde(rename = "OptionGroupName")]
    #[serde(default)]
    pub option_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "FailoverGlobalClusterMessage")]
pub struct FailoverGlobalClusterMessage {
    #[serde(rename = "AllowDataLoss")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_data_loss: Option<bool>,
    #[serde(rename = "GlobalClusterIdentifier")]
    #[serde(default)]
    pub global_cluster_identifier: String,
    #[serde(rename = "Switchover")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switchover: Option<bool>,
    #[serde(rename = "TargetDbClusterIdentifier")]
    #[serde(default)]
    pub target_db_cluster_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateTenantDatabaseMessage")]
pub struct CreateTenantDatabaseMessage {
    #[serde(rename = "CharacterSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_set_name: Option<String>,
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    pub d_b_instance_identifier: String,
    #[serde(rename = "ManageMasterUserPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manage_master_user_password: Option<bool>,
    #[serde(rename = "MasterUserPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<String>,
    #[serde(rename = "MasterUserSecretKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_secret_kms_key_id: Option<String>,
    #[serde(rename = "MasterUsername")]
    #[serde(default)]
    pub master_username: String,
    #[serde(rename = "NcharCharacterSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nchar_character_set_name: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "TenantDBName")]
    #[serde(default)]
    pub tenant_d_b_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeReservedDBInstancesOfferingsResult")]
pub struct ReservedDBInstancesOfferingMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "ReservedDBInstancesOfferings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_d_b_instances_offerings: Option<ReservedDBInstancesOfferingList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReservedDBInstancesOfferingList {
    #[serde(
        rename = "ReservedDBInstancesOffering",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ReservedDBInstancesOffering>,
}
impl From<Vec<ReservedDBInstancesOffering>> for ReservedDBInstancesOfferingList {
    fn from(v: Vec<ReservedDBInstancesOffering>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ReservedDBInstancesOffering> for ReservedDBInstancesOfferingList {
    fn from_iter<I: IntoIterator<Item = ReservedDBInstancesOffering>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ReservedDBInstancesOffering>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlReservedDBInstancesOfferingList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ReservedDBInstancesOffering>,
}

impl From<Vec<ReservedDBInstancesOffering>> for XmlReservedDBInstancesOfferingList {
    fn from(v: Vec<ReservedDBInstancesOffering>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ReservedDBInstancesOffering> for XmlReservedDBInstancesOfferingList {
    fn from_iter<I: IntoIterator<Item = ReservedDBInstancesOffering>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ReservedDBInstancesOffering")]
pub struct ReservedDBInstancesOffering {
    #[serde(rename = "CurrencyCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(rename = "DBInstanceClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_class: Option<String>,
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "FixedPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_price: Option<f64>,
    #[serde(rename = "MultiAZ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_a_z: Option<bool>,
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
    #[serde(rename = "ReservedDBInstancesOfferingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_d_b_instances_offering_id: Option<String>,
    #[serde(rename = "UsagePrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_price: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBProxyEndpointsResult")]
pub struct DescribeDBProxyEndpointsResponse {
    #[serde(rename = "DBProxyEndpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_proxy_endpoints: Option<DBProxyEndpointList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBProxyEndpointList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<DBProxyEndpoint>,
}
impl From<Vec<DBProxyEndpoint>> for DBProxyEndpointList {
    fn from(v: Vec<DBProxyEndpoint>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBProxyEndpoint> for DBProxyEndpointList {
    fn from_iter<I: IntoIterator<Item = DBProxyEndpoint>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBProxyEndpoint>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBProxyEndpointList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBProxyEndpoint>,
}

impl From<Vec<DBProxyEndpoint>> for XmlDBProxyEndpointList {
    fn from(v: Vec<DBProxyEndpoint>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBProxyEndpoint> for XmlDBProxyEndpointList {
    fn from_iter<I: IntoIterator<Item = DBProxyEndpoint>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyDBRecommendationResult")]
pub struct DBRecommendationMessage {
    #[serde(rename = "DBRecommendation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_recommendation: Option<DBRecommendation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyDBSnapshotMessage")]
pub struct ModifyDBSnapshotMessage {
    #[serde(rename = "DBSnapshotIdentifier")]
    #[serde(default)]
    pub d_b_snapshot_identifier: String,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "OptionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_name: Option<String>,
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
    #[serde(rename = "EventCategories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_categories: Option<EventCategoriesList>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "SourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
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
#[serde(rename = "RebootDBClusterResult")]
pub struct RebootDBClusterResult {
    #[serde(rename = "DBCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster: Option<DBCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDBSnapshotResult")]
pub struct CreateDBSnapshotResult {
    #[serde(rename = "DBSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_snapshot: Option<DBSnapshot>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBProxiesRequest")]
pub struct DescribeDBProxiesRequest {
    #[serde(rename = "DBProxyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_proxy_name: Option<String>,
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
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteEventSubscriptionResult")]
pub struct DeleteEventSubscriptionResult {
    #[serde(rename = "EventSubscription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_subscription: Option<EventSubscription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeGlobalClustersResult")]
pub struct GlobalClustersMessage {
    #[serde(rename = "GlobalClusters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_clusters: Option<GlobalClusterList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlobalClusterList {
    #[serde(
        rename = "GlobalClusterMember",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<GlobalCluster>,
}
impl From<Vec<GlobalCluster>> for GlobalClusterList {
    fn from(v: Vec<GlobalCluster>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<GlobalCluster> for GlobalClusterList {
    fn from_iter<I: IntoIterator<Item = GlobalCluster>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<GlobalCluster>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlGlobalClusterList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<GlobalCluster>,
}

impl From<Vec<GlobalCluster>> for XmlGlobalClusterList {
    fn from(v: Vec<GlobalCluster>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<GlobalCluster> for XmlGlobalClusterList {
    fn from_iter<I: IntoIterator<Item = GlobalCluster>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBClusterSnapshotAttributesMessage")]
pub struct DescribeDBClusterSnapshotAttributesMessage {
    #[serde(rename = "DBClusterSnapshotIdentifier")]
    #[serde(default)]
    pub d_b_cluster_snapshot_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteDBClusterAutomatedBackupResult")]
pub struct DeleteDBClusterAutomatedBackupResult {
    #[serde(rename = "DBClusterAutomatedBackup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_automated_backup: Option<DBClusterAutomatedBackup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DownloadDBLogFilePortionMessage")]
pub struct DownloadDBLogFilePortionMessage {
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    pub d_b_instance_identifier: String,
    #[serde(rename = "LogFileName")]
    #[serde(default)]
    pub log_file_name: String,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "NumberOfLines")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_lines: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBParametersResult")]
pub struct DBParameterGroupDetails {
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
#[serde(rename = "RestoreDBInstanceToPointInTimeMessage")]
pub struct RestoreDBInstanceToPointInTimeMessage {
    #[serde(rename = "AdditionalStorageVolumes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_storage_volumes: Option<AdditionalStorageVolumesList>,
    #[serde(rename = "AllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i32>,
    #[serde(rename = "BackupTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_target: Option<String>,
    #[serde(rename = "CACertificateIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_a_certificate_identifier: Option<String>,
    #[serde(rename = "CopyTagsToSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_snapshot: Option<bool>,
    #[serde(rename = "CustomIamInstanceProfile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_iam_instance_profile: Option<String>,
    #[serde(rename = "DBInstanceClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_class: Option<String>,
    #[serde(rename = "DBName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_name: Option<String>,
    #[serde(rename = "DBParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_parameter_group_name: Option<String>,
    #[serde(rename = "DBSubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_subnet_group_name: Option<String>,
    #[serde(rename = "DedicatedLogVolume")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_log_volume: Option<bool>,
    #[serde(rename = "DeletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "DomainAuthSecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_auth_secret_arn: Option<String>,
    #[serde(rename = "DomainDnsIps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_dns_ips: Option<StringList>,
    #[serde(rename = "DomainFqdn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_fqdn: Option<String>,
    #[serde(rename = "DomainIAMRoleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_i_a_m_role_name: Option<String>,
    #[serde(rename = "DomainOu")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_ou: Option<String>,
    #[serde(rename = "EnableCloudwatchLogsExports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_cloudwatch_logs_exports: Option<LogTypeList>,
    #[serde(rename = "EnableCustomerOwnedIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_customer_owned_ip: Option<bool>,
    #[serde(rename = "EnableIAMDatabaseAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_i_a_m_database_authentication: Option<bool>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineLifecycleSupport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_lifecycle_support: Option<String>,
    #[serde(rename = "Iops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    #[serde(rename = "LicenseModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_model: Option<String>,
    #[serde(rename = "ManageMasterUserPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manage_master_user_password: Option<bool>,
    #[serde(rename = "MasterUserSecretKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_secret_kms_key_id: Option<String>,
    #[serde(rename = "MaxAllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_allocated_storage: Option<i32>,
    #[serde(rename = "MultiAZ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_a_z: Option<bool>,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "OptionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_name: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "PreferredBackupWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,
    #[serde(rename = "ProcessorFeatures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor_features: Option<ProcessorFeatureList>,
    #[serde(rename = "PubliclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    #[serde(rename = "RestoreTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_time: Option<String>,
    #[serde(rename = "SourceDBInstanceAutomatedBackupsArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_d_b_instance_automated_backups_arn: Option<String>,
    #[serde(rename = "SourceDBInstanceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_d_b_instance_identifier: Option<String>,
    #[serde(rename = "SourceDbiResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_dbi_resource_id: Option<String>,
    #[serde(rename = "StorageThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_throughput: Option<i32>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "TagSpecifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_specifications: Option<TagSpecificationList>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "TargetDBInstanceIdentifier")]
    #[serde(default)]
    pub target_d_b_instance_identifier: String,
    #[serde(rename = "TdeCredentialArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tde_credential_arn: Option<String>,
    #[serde(rename = "TdeCredentialPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tde_credential_password: Option<String>,
    #[serde(rename = "UseDefaultProcessorFeatures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_default_processor_features: Option<bool>,
    #[serde(rename = "UseLatestRestorableTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_latest_restorable_time: Option<bool>,
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<VpcSecurityGroupIdList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyCustomDBEngineVersionMessage")]
pub struct ModifyCustomDBEngineVersionMessage {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    pub engine: String,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    pub engine_version: String,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeTenantDatabasesMessage")]
pub struct DescribeTenantDatabasesMessage {
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_identifier: Option<String>,
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
    #[serde(rename = "TenantDBName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_d_b_name: Option<String>,
}
