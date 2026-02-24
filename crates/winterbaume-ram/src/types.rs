/// A RAM resource share.
#[derive(Debug, Clone)]
pub struct ResourceShare {
    pub resource_share_arn: String,
    pub name: String,
    pub owning_account_id: String,
    pub allow_external_principals: bool,
    pub status: String,
    pub creation_time: f64,
    pub last_updated_time: f64,
    pub tags: Vec<Tag>,
}

/// A resource associated with a resource share.
#[derive(Debug, Clone)]
pub struct Resource {
    pub arn: String,
    pub r#type: String,
    pub resource_share_arn: String,
    pub status: String,
    pub creation_time: f64,
    pub last_updated_time: f64,
}

/// A tag key-value pair.
#[derive(Debug, Clone)]
pub struct Tag {
    pub key: String,
    pub value: String,
}

/// A resource share association (RESOURCE or PRINCIPAL).
#[derive(Debug, Clone)]
pub struct ResourceShareAssociation {
    pub resource_share_arn: String,
    pub resource_share_name: String,
    pub associated_entity: String,
    pub association_type: String,
    pub status: String,
    pub creation_time: f64,
    pub last_updated_time: f64,
    pub external: bool,
}

/// A RAM permission summary (static/built-in permissions).
#[derive(Debug, Clone)]
pub struct RamPermission {
    pub arn: String,
    pub name: String,
    pub resource_type: String,
    pub default_version: bool,
    pub is_resource_type_default: bool,
    pub version: String,
    pub status: String,
    pub creation_time: f64,
    pub last_updated_time: f64,
    pub permission_type: String,
}

/// A resource type available for sharing.
#[derive(Debug, Clone)]
pub struct ShareableResourceType {
    pub resource_type: String,
    pub service_name: String,
    pub resource_region_scope: String,
}

/// A customer-managed RAM permission.
#[derive(Debug, Clone)]
pub struct CustomerManagedPermission {
    pub arn: String,
    pub name: String,
    pub resource_type: String,
    pub policy_template: String,
    pub default_version: bool,
    pub version: String,
    pub status: String,
    pub creation_time: f64,
    pub last_updated_time: f64,
    pub permission_type: String,
    pub tags: Vec<Tag>,
}

/// A permission associated with a resource share.
#[derive(Debug, Clone)]
pub struct ResourceSharePermissionEntry {
    pub permission_arn: String,
    pub permission_version: String,
    pub resource_share_arn: String,
    pub resource_type: String,
    pub default_version: bool,
    pub last_updated_time: f64,
    pub status: String,
}

/// A replace-permission-associations work item.
#[derive(Debug, Clone)]
pub struct ReplacePermissionWork {
    pub id: String,
    pub from_permission_arn: String,
    pub from_permission_version: String,
    pub to_permission_arn: String,
    pub to_permission_version: String,
    pub status: String,
    pub creation_time: f64,
    pub last_updated_time: f64,
}

/// An invitation to join a resource share.
#[derive(Debug, Clone)]
pub struct ResourceShareInvitation {
    pub invitation_arn: String,
    pub resource_share_arn: String,
    pub resource_share_name: String,
    pub sender_account_id: String,
    pub receiver_account_id: String,
    pub status: String, // PENDING, ACCEPTED, REJECTED, EXPIRED
    pub invitation_timestamp: f64,
}
