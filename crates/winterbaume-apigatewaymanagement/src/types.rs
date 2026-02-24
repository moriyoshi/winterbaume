use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Connection {
    pub connected_at: DateTime<Utc>,
    pub last_active_at: DateTime<Utc>,
    pub source_ip: String,
    pub user_agent: String,
    pub data: Vec<u8>,
}

impl Default for Connection {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            connected_at: now,
            last_active_at: now,
            source_ip: "192.168.0.1".to_string(),
            user_agent: "winterbaume".to_string(),
            data: Vec::new(),
        }
    }
}
