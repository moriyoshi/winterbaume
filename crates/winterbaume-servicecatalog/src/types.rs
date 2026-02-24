use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct PortfolioDetail {
    pub id: String,
    pub arn: String,
    pub display_name: String,
    pub description: String,
    pub created_time: DateTime<Utc>,
    pub provider_name: String,
    pub tags: Vec<PortfolioTag>,
}

#[derive(Debug, Clone)]
pub struct ProductDetail {
    pub product_id: String,
    pub product_arn: String,
    pub name: String,
    pub owner: String,
    pub description: Option<String>,
    pub product_type: String,
    pub distributor: Option<String>,
    pub support_description: Option<String>,
    pub support_email: Option<String>,
    pub support_url: Option<String>,
    pub created_time: DateTime<Utc>,
    pub tags: Vec<PortfolioTag>,
}

#[derive(Debug, Clone)]
pub struct PortfolioTag {
    pub key: String,
    pub value: String,
}
