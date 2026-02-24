#[derive(Debug, Clone)]
pub struct AlternateContact {
    pub alternate_contact_type: String,
    pub email_address: String,
    pub name: String,
    pub phone_number: String,
    pub title: String,
}

#[derive(Debug, Clone, Default)]
pub struct ContactInformation {
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub address_line3: Option<String>,
    pub city: String,
    pub company_name: Option<String>,
    pub country_code: String,
    pub district_or_county: Option<String>,
    pub full_name: String,
    pub phone_number: String,
    pub postal_code: String,
    pub state_or_region: Option<String>,
    pub website_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RegionOptStatus {
    Enabled,
    EnabledByDefault,
    Disabled,
    Enabling,
    Disabling,
}

impl RegionOptStatus {
    pub fn as_str(&self) -> &str {
        match self {
            RegionOptStatus::Enabled => "ENABLED",
            RegionOptStatus::EnabledByDefault => "ENABLED_BY_DEFAULT",
            RegionOptStatus::Disabled => "DISABLED",
            RegionOptStatus::Enabling => "ENABLING",
            RegionOptStatus::Disabling => "DISABLING",
        }
    }
}

/// Regions that are enabled by default (opt-out model). All other regions are opt-in.
pub const DEFAULT_ENABLED_REGIONS: &[&str] = &[
    "us-east-1",
    "us-east-2",
    "us-west-1",
    "us-west-2",
    "ap-northeast-1",
    "ap-northeast-2",
    "ap-northeast-3",
    "ap-south-1",
    "ap-southeast-1",
    "ap-southeast-2",
    "ca-central-1",
    "eu-central-1",
    "eu-north-1",
    "eu-west-1",
    "eu-west-2",
    "eu-west-3",
    "sa-east-1",
];

/// All AWS regions (opt-in regions plus default-enabled regions).
pub const ALL_REGIONS: &[&str] = &[
    "af-south-1",
    "ap-east-1",
    "ap-northeast-1",
    "ap-northeast-2",
    "ap-northeast-3",
    "ap-south-1",
    "ap-south-2",
    "ap-southeast-1",
    "ap-southeast-2",
    "ap-southeast-3",
    "ap-southeast-4",
    "ca-central-1",
    "ca-west-1",
    "eu-central-1",
    "eu-central-2",
    "eu-north-1",
    "eu-south-1",
    "eu-south-2",
    "eu-west-1",
    "eu-west-2",
    "eu-west-3",
    "il-central-1",
    "me-central-1",
    "me-south-1",
    "mx-central-1",
    "sa-east-1",
    "us-east-1",
    "us-east-2",
    "us-west-1",
    "us-west-2",
];
