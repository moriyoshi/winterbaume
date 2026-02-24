use std::collections::HashMap;

use serde_json::Value;

#[derive(Debug, Default)]
pub struct ConnectContactLensState {
    pub segments: HashMap<(String, String), Vec<Value>>,
}

impl ConnectContactLensState {
    pub fn put_segments(&mut self, instance_id: String, contact_id: String, segments: Vec<Value>) {
        self.segments.insert((instance_id, contact_id), segments);
    }

    pub fn get_segments(&self, instance_id: &str, contact_id: &str) -> Vec<Value> {
        self.segments
            .get(&(instance_id.to_string(), contact_id.to_string()))
            .cloned()
            .unwrap_or_default()
    }
}
