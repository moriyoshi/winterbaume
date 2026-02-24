#[derive(Debug, Clone)]
pub struct ConfigurationSession {
    pub token: String,
    pub application_id: String,
    pub environment_id: String,
    pub configuration_profile_id: String,
    pub required_minimum_poll_interval_in_seconds: Option<i32>,
}
