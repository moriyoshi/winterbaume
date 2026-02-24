use std::collections::HashMap;

use chrono::{DateTime, Utc};

/// The resource types that Inspector2 can scan.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Ec2,
    Ecr,
    Lambda,
    LambdaCode,
    CodeRepository,
}

impl ResourceType {
    pub fn from_str_value(s: &str) -> Option<Self> {
        match s {
            "EC2" => Some(Self::Ec2),
            "ECR" => Some(Self::Ecr),
            "LAMBDA" => Some(Self::Lambda),
            "LAMBDA_CODE" => Some(Self::LambdaCode),
            "CODE_REPOSITORY" => Some(Self::CodeRepository),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Ec2 => "EC2",
            Self::Ecr => "ECR",
            Self::Lambda => "LAMBDA",
            Self::LambdaCode => "LAMBDA_CODE",
            Self::CodeRepository => "CODE_REPOSITORY",
        }
    }
}

/// Status of a resource type scan.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceScanStatus {
    Enabled,
    Disabled,
}

impl ResourceScanStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Enabled => "ENABLED",
            Self::Disabled => "DISABLED",
        }
    }
}

/// Overall account status for Inspector2.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountStatus {
    Enabled,
    Disabled,
    Disabling,
    Enabling,
    Suspended,
}

impl AccountStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Enabled => "ENABLED",
            Self::Disabled => "DISABLED",
            Self::Disabling => "DISABLING",
            Self::Enabling => "ENABLING",
            Self::Suspended => "SUSPENDED",
        }
    }
}

/// A finding produced by Inspector2.
#[derive(Debug, Clone)]
pub struct Finding {
    pub finding_arn: String,
    pub aws_account_id: String,
    pub description: String,
    pub first_observed_at: DateTime<Utc>,
    pub last_observed_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub severity: String,
    pub status: String,
    pub title: String,
    pub finding_type: String,
}

/// A member account record.
#[derive(Debug, Clone)]
pub struct MemberRecord {
    pub account_id: String,
    pub relationship_status: String,
    pub updated_at: DateTime<Utc>,
}

/// An Inspector2 filter.
#[derive(Debug, Clone)]
pub struct InspectorFilter {
    pub arn: String,
    pub name: String,
    pub action: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub owner_id: String,
    pub tags: Option<HashMap<String, String>>,
}

/// A delegated admin account record.
#[derive(Debug, Clone)]
pub struct DelegatedAdminRecord {
    pub account_id: String,
    pub status: String,
}

/// Organization auto-enable configuration.
#[derive(Debug, Clone)]
pub struct AutoEnableConfig {
    pub ec2: bool,
    pub ecr: bool,
    pub lambda: Option<bool>,
    pub lambda_code: Option<bool>,
    pub code_repository: Option<bool>,
}

/// Organization configuration result.
#[derive(Debug, Clone)]
pub struct OrganizationConfig {
    pub auto_enable: Option<AutoEnableConfig>,
    pub max_account_limit_reached: bool,
}

/// A code security integration record.
#[derive(Debug, Clone)]
pub struct CodeSecurityIntegration {
    pub integration_arn: String,
    pub name: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A code security scan configuration record.
#[derive(Debug, Clone)]
pub struct CodeSecurityScanConfig {
    pub scan_configuration_arn: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// An association between a code security scan configuration and a resource.
#[derive(Debug, Clone)]
pub struct CodeSecurityScanConfigAssociation {
    pub scan_configuration_arn: String,
    pub resource_id: String,
    pub status: String,
    pub updated_at: DateTime<Utc>,
}

/// Per-member EC2 deep inspection status record.
#[derive(Debug, Clone)]
pub struct MemberEc2DeepInspectionStatus {
    pub account_id: String,
    pub activate_deep_inspection: bool,
    pub status: String,
}
