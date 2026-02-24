use chrono::{DateTime, Utc};

/// Represents a thing shadow document stored in the IoT Data Plane.
#[derive(Debug, Clone)]
pub struct ThingShadow {
    pub thing_name: String,
    pub shadow_name: Option<String>,
    pub payload: Vec<u8>,
    pub version: i64,
    pub last_modified: DateTime<Utc>,
}

/// Key for identifying a shadow: (thing_name, shadow_name).
/// Named shadows use Some(name), classic shadow uses None.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ShadowKey {
    pub thing_name: String,
    pub shadow_name: Option<String>,
}

impl ShadowKey {
    pub fn classic(thing_name: &str) -> Self {
        Self {
            thing_name: thing_name.to_string(),
            shadow_name: None,
        }
    }

    pub fn named(thing_name: &str, shadow_name: &str) -> Self {
        Self {
            thing_name: thing_name.to_string(),
            shadow_name: Some(shadow_name.to_string()),
        }
    }
}

/// Represents an MQTT message published via the Publish API.
#[derive(Debug, Clone)]
pub struct PublishedMessage {
    pub topic: String,
    pub payload: Vec<u8>,
    pub qos: i32,
    pub retain: bool,
    pub published_at: DateTime<Utc>,
}

/// Represents a retained MQTT message stored by topic.
#[derive(Debug, Clone)]
pub struct RetainedMessage {
    pub topic: String,
    pub payload: Vec<u8>,
    pub qos: i32,
    pub last_modified: DateTime<Utc>,
}
