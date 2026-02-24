use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Directory {
    pub directory_arn: String,
    pub name: String,
    pub schema_arn: String,
    pub state: String,
    pub created_date_time: f64,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Schema {
    pub schema_arn: String,
    pub name: String,
    pub type_: SchemaType,
    pub published_arns: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SchemaType {
    Development,
    Published,
}
