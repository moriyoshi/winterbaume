/// A resource registered with Lake Formation.
#[derive(Debug, Clone)]
pub struct RegisteredResource {
    /// The ARN of the resource (e.g., S3 path).
    pub resource_arn: String,
    /// The IAM role ARN used by Lake Formation to access the resource.
    pub role_arn: Option<String>,
    /// Whether Lake Formation manages the resource using service-linked role.
    pub use_service_linked_role: bool,
}

/// Data lake settings for an account/region.
#[derive(Debug, Clone)]
pub struct DataLakeSettings {
    /// List of data lake administrator principals.
    pub data_lake_admins: Vec<DataLakePrincipal>,
    /// Whether to allow external data filtering.
    pub allow_external_data_filtering: bool,
    /// Authorized session tag value list.
    pub authorized_session_tag_value_list: Vec<String>,
    /// Create database default permissions.
    pub create_database_default_permissions: Vec<PrincipalPermissions>,
    /// Create table default permissions.
    pub create_table_default_permissions: Vec<PrincipalPermissions>,
}

impl Default for DataLakeSettings {
    fn default() -> Self {
        Self {
            data_lake_admins: Vec::new(),
            allow_external_data_filtering: false,
            authorized_session_tag_value_list: Vec::new(),
            create_database_default_permissions: vec![PrincipalPermissions {
                principal: DataLakePrincipal {
                    data_lake_principal_identifier: "IAM_ALLOWED_PRINCIPALS".to_string(),
                },
                permissions: vec!["ALL".to_string()],
            }],
            create_table_default_permissions: vec![PrincipalPermissions {
                principal: DataLakePrincipal {
                    data_lake_principal_identifier: "IAM_ALLOWED_PRINCIPALS".to_string(),
                },
                permissions: vec!["ALL".to_string()],
            }],
        }
    }
}

/// A data lake principal.
#[derive(Debug, Clone)]
pub struct DataLakePrincipal {
    pub data_lake_principal_identifier: String,
}

/// Principal permissions pair.
#[derive(Debug, Clone)]
pub struct PrincipalPermissions {
    pub principal: DataLakePrincipal,
    pub permissions: Vec<String>,
}

/// An LF-Tag definition stored in the catalog.
#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub struct LFTag {
    pub tag_key: String,
    pub tag_values: Vec<String>,
    pub catalog_id: Option<String>,
}

/// An LF-Tag assignment on a resource.
#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub struct LFTagAssignment {
    pub resource: ResourceIdentifier,
    pub lf_tags: Vec<LFTagPair>,
}

/// Simplified resource identifier for tag assignments.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResourceIdentifier {
    Database {
        catalog_id: Option<String>,
        name: String,
    },
    Table {
        catalog_id: Option<String>,
        database_name: String,
        name: String,
    },
    Column {
        catalog_id: Option<String>,
        database_name: String,
        table_name: String,
        column_names: Vec<String>,
    },
}

/// An LF-Tag key-value pair for assignments.
#[derive(Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub struct LFTagPair {
    pub catalog_id: Option<String>,
    pub tag_key: String,
    pub tag_values: Vec<String>,
}

/// A permission grant entry stored in state.
#[derive(Debug, Clone)]
pub struct PermissionGrant {
    pub principal: String,
    pub resource: serde_json::Value,
    pub permissions: Vec<String>,
    pub permissions_with_grant_option: Vec<String>,
}
