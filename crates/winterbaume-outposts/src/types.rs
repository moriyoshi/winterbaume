use std::collections::HashMap;

/// An AWS Outpost.
#[derive(Debug, Clone)]
pub struct Outpost {
    pub outpost_id: String,
    pub outpost_arn: String,
    pub owner_id: String,
    pub name: String,
    pub description: Option<String>,
    pub site_id: String,
    pub site_arn: String,
    pub availability_zone: Option<String>,
    pub availability_zone_id: Option<String>,
    pub life_cycle_status: String,
    pub supported_hardware_type: Option<String>,
    pub tags: HashMap<String, String>,
}

/// An AWS Outposts site.
#[derive(Debug, Clone)]
pub struct Site {
    pub site_id: String,
    pub site_arn: String,
    pub account_id: String,
    pub name: String,
    pub description: Option<String>,
    pub notes: Option<String>,
    pub operating_address_country_code: Option<String>,
    pub operating_address_state_or_region: Option<String>,
    pub operating_address_city: Option<String>,
    pub tags: HashMap<String, String>,
}
