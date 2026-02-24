use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AppBundle {
    pub arn: String,
    pub customer_managed_key_arn: Option<String>,
    pub tags: HashMap<String, String>,
}
