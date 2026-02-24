use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Dataset {
    pub name: String,
    pub account_id: String,
    pub created_by: String,
    pub create_date: DateTime<Utc>,
    pub last_modified_by: String,
    pub last_modified_date: DateTime<Utc>,
    pub source: String,
    pub format: Option<String>,
    pub format_options: Option<Value>,
    pub input: Value,
    pub tags: Option<std::collections::HashMap<String, String>>,
    pub resource_arn: String,
    pub path_options: Option<Value>,
}

#[derive(Debug, Clone)]
pub struct RecipeVersion {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub steps: Vec<Value>,
    pub project_name: Option<String>,
    pub tags: Option<std::collections::HashMap<String, String>>,
    pub account_id: String,
    pub created_by: String,
    pub create_date: DateTime<Utc>,
    pub last_modified_by: String,
    pub last_modified_date: DateTime<Utc>,
    pub resource_arn: String,
    pub published_by: Option<String>,
    pub published_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct Recipe {
    pub name: String,
    pub description: Option<String>,
    pub steps: Vec<Value>,
    pub project_name: Option<String>,
    pub tags: Option<std::collections::HashMap<String, String>>,
    pub account_id: String,
    pub created_by: String,
    pub create_date: DateTime<Utc>,
    pub last_modified_by: String,
    pub last_modified_date: DateTime<Utc>,
    pub resource_arn: String,
    /// All versions: key is version string ("1.0", "1.1", etc.), value is the version data
    pub versions: std::collections::BTreeMap<String, RecipeVersion>,
    /// The latest working version number (incremented on update)
    pub latest_working: i64,
    /// The latest published version number (set on publish), None if never published
    pub latest_published: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct Ruleset {
    pub name: String,
    pub description: Option<String>,
    pub target_arn: String,
    pub rules: Vec<Value>,
    pub tags: Option<std::collections::HashMap<String, String>>,
    pub account_id: String,
    pub created_by: String,
    pub create_date: DateTime<Utc>,
    pub last_modified_by: String,
    pub last_modified_date: DateTime<Utc>,
    pub resource_arn: String,
}

#[derive(Debug, Clone)]
pub struct Schedule {
    pub name: String,
    pub cron_expression: Option<String>,
    pub job_names: Option<Vec<String>>,
    pub tags: Option<std::collections::HashMap<String, String>>,
    pub account_id: String,
    pub created_by: String,
    pub create_date: DateTime<Utc>,
    pub last_modified_by: String,
    pub last_modified_date: DateTime<Utc>,
    pub resource_arn: String,
}

#[derive(Debug, Clone)]
pub struct Job {
    pub name: String,
    pub job_type: String, // "PROFILE" or "RECIPE"
    pub dataset_name: Option<String>,
    pub project_name: Option<String>,
    pub role_arn: String,
    pub encryption_mode: Option<String>,
    pub encryption_key_arn: Option<String>,
    pub log_subscription: Option<String>,
    pub output_location: Option<Value>,
    pub outputs: Option<Value>,
    pub tags: Option<std::collections::HashMap<String, String>>,
    pub account_id: String,
    pub created_by: String,
    pub create_date: DateTime<Utc>,
    pub last_modified_by: String,
    pub last_modified_date: DateTime<Utc>,
    pub resource_arn: String,
    pub max_capacity: Option<i32>,
    pub max_retries: Option<i32>,
    pub timeout: Option<i32>,
}
