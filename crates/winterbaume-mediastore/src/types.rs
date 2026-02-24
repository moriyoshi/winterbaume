use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Container {
    pub arn: String,
    pub name: String,
    pub endpoint: String,
    pub status: String,
    pub creation_time: String,
    pub lifecycle_policy: Option<String>,
    pub policy: Option<String>,
    pub metric_policy: Option<Value>,
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone)]
pub struct Tag {
    pub key: String,
    pub value: Option<String>,
}
