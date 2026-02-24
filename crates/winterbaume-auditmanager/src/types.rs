use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Control {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub control_type: String,
    pub created_at: f64,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Framework {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub compliance_type: Option<String>,
    pub framework_type: String,
    pub created_at: f64,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Assessment {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub framework_id: String,
    pub status: String,
    pub created_at: f64,
    pub tags: HashMap<String, String>,
}
