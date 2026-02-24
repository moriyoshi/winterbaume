use std::collections::HashMap;

use chrono::{DateTime, Utc};

/// A Direct Connect connection.
#[derive(Debug, Clone)]
pub struct Connection {
    pub connection_id: String,
    pub connection_name: String,
    pub connection_state: ConnectionState,
    pub region: String,
    pub location: String,
    pub bandwidth: String,
    pub owner_account: String,
    pub vlan: i32,
    pub partner_name: Option<String>,
    pub loa_issue_time: Option<DateTime<Utc>>,
    pub tags: HashMap<String, String>,
}

/// Connection state enum matching AWS Direct Connect states.
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionState {
    Ordering,
    Requested,
    Pending,
    Available,
    Down,
    Deleting,
    Deleted,
    Rejected,
    Unknown,
}

impl ConnectionState {
    pub fn as_str(&self) -> &str {
        match self {
            ConnectionState::Ordering => "ordering",
            ConnectionState::Requested => "requested",
            ConnectionState::Pending => "pending",
            ConnectionState::Available => "available",
            ConnectionState::Down => "down",
            ConnectionState::Deleting => "deleting",
            ConnectionState::Deleted => "deleted",
            ConnectionState::Rejected => "rejected",
            ConnectionState::Unknown => "unknown",
        }
    }
}

/// A Direct Connect location.
#[derive(Debug, Clone)]
pub struct Location {
    pub location_code: String,
    pub location_name: String,
    pub region: String,
    pub available_port_speeds: Vec<String>,
    pub available_providers: Vec<String>,
    pub available_mac_sec_port_speeds: Vec<String>,
}
