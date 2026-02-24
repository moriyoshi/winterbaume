use std::collections::HashMap;

use chrono::{DateTime, Utc};

/// An application registered in Service Catalog App Registry.
#[derive(Debug, Clone)]
pub struct Application {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub description: Option<String>,
    pub creation_time: DateTime<Utc>,
    pub last_update_time: DateTime<Utc>,
    pub tags: HashMap<String, String>,
}

/// An attribute group registered in Service Catalog App Registry.
#[derive(Debug, Clone)]
pub struct AttributeGroup {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub description: Option<String>,
    pub attributes: String,
    pub creation_time: DateTime<Utc>,
    pub last_update_time: DateTime<Utc>,
    pub tags: HashMap<String, String>,
}

/// An association between an application and an attribute group.
#[derive(Debug, Clone)]
pub struct AttributeGroupAssociation {
    pub application_id: String,
    pub attribute_group_id: String,
}

/// An association between an application and a resource.
#[derive(Debug, Clone)]
pub struct ResourceAssociation {
    pub application_id: String,
    pub resource_type: String,
    pub resource: String,
    pub resource_arn: Option<String>,
    pub options: Vec<String>,
}

/// Global AppRegistry configuration (account-level, not per-region).
#[derive(Debug, Clone, Default)]
pub struct AppRegistryConfig {
    pub tag_key: Option<String>,
}
