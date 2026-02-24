use std::collections::HashMap;

use serde_json::Value;

#[derive(Debug, Default)]
pub struct PersonalizeEventsState {
    /// Events ingested via PutEvents, keyed by tracking_id.
    pub events: HashMap<String, Vec<Value>>,
    /// Action interactions ingested via PutActionInteractions, keyed by tracking_id.
    pub action_interactions: HashMap<String, Vec<Value>>,
    /// Actions ingested via PutActions, keyed by dataset ARN.
    pub actions: HashMap<String, Vec<Value>>,
    /// Items ingested via PutItems, keyed by dataset ARN.
    pub items: HashMap<String, Vec<Value>>,
    /// Users ingested via PutUsers, keyed by dataset ARN.
    pub users: HashMap<String, Vec<Value>>,
}

impl PersonalizeEventsState {
    pub fn put_events(&mut self, tracking_id: &str, events: Vec<Value>) {
        self.events
            .entry(tracking_id.to_string())
            .or_default()
            .extend(events);
    }

    pub fn put_action_interactions(&mut self, tracking_id: &str, interactions: Vec<Value>) {
        self.action_interactions
            .entry(tracking_id.to_string())
            .or_default()
            .extend(interactions);
    }

    pub fn put_actions(&mut self, dataset_arn: &str, actions: Vec<Value>) {
        self.actions
            .entry(dataset_arn.to_string())
            .or_default()
            .extend(actions);
    }

    pub fn put_items(&mut self, dataset_arn: &str, items: Vec<Value>) {
        self.items
            .entry(dataset_arn.to_string())
            .or_default()
            .extend(items);
    }

    pub fn put_users(&mut self, dataset_arn: &str, users: Vec<Value>) {
        self.users
            .entry(dataset_arn.to_string())
            .or_default()
            .extend(users);
    }
}
