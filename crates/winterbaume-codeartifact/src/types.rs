use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Domain {
    pub name: String,
    pub owner: String,
    pub arn: String,
    pub encryption_key: Option<String>,
    pub status: String,
    pub created_time: f64,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Repository {
    pub name: String,
    pub domain_name: String,
    pub domain_owner: String,
    pub arn: String,
    pub description: Option<String>,
    pub created_time: f64,
    pub tags: HashMap<String, String>,
}
