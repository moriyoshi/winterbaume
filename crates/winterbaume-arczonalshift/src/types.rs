use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ZonalShift {
    pub zonal_shift_id: String,
    pub resource_identifier: String,
    pub away_from: String,
    pub comment: String,
    pub start_time: i64,
    pub expiry_time: i64,
    pub status: String,
    pub shift_type: String,
    pub practice_run_outcome: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct PracticeRunConfiguration {
    pub resource_identifier: String,
    pub blocking_alarms: Vec<ControlCondition>,
    pub outcome_alarms: Vec<ControlCondition>,
    pub blocked_windows: Vec<String>,
    pub allowed_windows: Vec<String>,
    pub blocked_dates: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ControlCondition {
    pub alarm_identifier: String,
    pub r#type: String,
}

#[derive(Debug, Clone)]
pub struct ManagedResource {
    pub resource_identifier: String,
    pub name: String,
    pub arn: String,
    pub availability_zones: Vec<String>,
    pub applied_weights: HashMap<String, f32>,
    pub zonal_autoshift_status: String,
}

impl ManagedResource {
    pub fn new(identifier: &str) -> Self {
        let (name, arn) = if identifier.starts_with("arn:") {
            let parsed_name = identifier
                .rsplit_once('/')
                .map(|(_, n)| n.to_string())
                .unwrap_or_else(|| identifier.to_string());
            (parsed_name, identifier.to_string())
        } else {
            (identifier.to_string(), identifier.to_string())
        };
        Self {
            resource_identifier: identifier.to_string(),
            name,
            arn,
            availability_zones: vec![
                "use1-az1".to_string(),
                "use1-az2".to_string(),
                "use1-az3".to_string(),
            ],
            applied_weights: HashMap::new(),
            zonal_autoshift_status: "DISABLED".to_string(),
        }
    }
}
