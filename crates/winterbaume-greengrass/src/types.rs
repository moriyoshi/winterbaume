/// A Greengrass group.
#[derive(Debug, Clone)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub arn: String,
    pub creation_timestamp: String,
    pub last_updated_timestamp: String,
    pub latest_version: String,
    pub latest_version_arn: String,
}

/// A generic Greengrass definition (core, device, function, resource, subscription).
#[derive(Debug, Clone)]
pub struct Definition {
    pub id: String,
    pub name: String,
    pub arn: String,
    pub creation_timestamp: String,
    pub last_updated_timestamp: String,
    pub latest_version: String,
    pub latest_version_arn: String,
    pub def_type: DefinitionType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DefinitionType {
    Connector,
    Core,
    Device,
    Function,
    Logger,
    Resource,
    Subscription,
}

impl DefinitionType {
    pub fn path_segment(&self) -> &'static str {
        match self {
            DefinitionType::Connector => "connectors",
            DefinitionType::Core => "cores",
            DefinitionType::Device => "devices",
            DefinitionType::Function => "functions",
            DefinitionType::Logger => "loggers",
            DefinitionType::Resource => "resources",
            DefinitionType::Subscription => "subscriptions",
        }
    }

    /// Display name for error messages (lowercase, used in "That {name} definition does not exist.")
    pub fn display_name(&self) -> &'static str {
        match self {
            DefinitionType::Connector => "connectors",
            DefinitionType::Core => "cores",
            DefinitionType::Device => "devices",
            DefinitionType::Function => "lambdas",
            DefinitionType::Logger => "loggers",
            DefinitionType::Resource => "resources",
            DefinitionType::Subscription => "subscriptions",
        }
    }

    /// Display name for version not found errors (title case, used in "Version X of {Name} Definition Y does not exist.")
    pub fn version_display_name(&self) -> &'static str {
        match self {
            DefinitionType::Connector => "Connector List",
            DefinitionType::Core => "Core List",
            DefinitionType::Device => "Device List",
            DefinitionType::Function => "Lambda List",
            DefinitionType::Logger => "Logger List",
            DefinitionType::Resource => "Resource List",
            DefinitionType::Subscription => "Subscription List",
        }
    }

    /// Display name for get_definition not found errors (title case with "List" suffix)
    pub fn get_display_name(&self) -> &'static str {
        match self {
            DefinitionType::Connector => "Connector List",
            DefinitionType::Core => "cores",
            DefinitionType::Device => "Device List",
            DefinitionType::Function => "Lambda List",
            DefinitionType::Logger => "Logger List",
            DefinitionType::Resource => "Resource List",
            DefinitionType::Subscription => "Subscription List",
        }
    }
}

/// A version of a definition.
#[derive(Debug, Clone)]
pub struct DefinitionVersion {
    pub arn: String,
    pub id: String,
    pub version: String,
    pub creation_timestamp: String,
    pub definition_id: String,
}

/// A group version.
#[derive(Debug, Clone)]
pub struct GroupVersionInfo {
    pub arn: String,
    pub id: String,
    pub version: String,
    pub creation_timestamp: String,
    pub group_id: String,
    pub core_definition_version_arn: Option<String>,
    pub device_definition_version_arn: Option<String>,
    pub function_definition_version_arn: Option<String>,
    pub resource_definition_version_arn: Option<String>,
    pub subscription_definition_version_arn: Option<String>,
}

/// A deployment.
#[derive(Debug, Clone)]
pub struct DeploymentInfo {
    pub deployment_id: String,
    pub deployment_arn: String,
    pub deployment_type: String,
    pub group_arn: String,
    pub created_at: String,
    pub deployment_status: String,
    pub updated_at: String,
}

/// Role association for a group.
#[derive(Debug, Clone)]
pub struct RoleAssociation {
    pub role_arn: String,
    pub associated_at: String,
}
