use serde_json::Value;

#[derive(Debug, Clone)]
pub struct ScalingPlan {
    pub scaling_plan_name: String,
    pub scaling_plan_version: i64,
    /// Stored as opaque JSON because it's a discriminated union over
    /// CloudFormationStackARN / TagFilters.
    pub application_source: Value,
    /// Stored opaquely; each instruction has many connector-specific fields.
    pub scaling_instructions: Vec<Value>,
    pub status_code: String,
    pub status_message: Option<String>,
    pub status_start_time: i64,
    pub creation_time: i64,
}
