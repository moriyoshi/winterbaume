use std::collections::HashMap;

use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Dashboard {
    pub arn: String,
    pub name: String,
    pub description: Option<String>,
    pub r#type: String,
    pub widgets: Vec<Value>,
    pub created_at: i64,
    pub updated_at: i64,
    pub tags: HashMap<String, String>,
}
