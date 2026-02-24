use std::collections::HashMap;

use chrono::Utc;
use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct IotDataPlaneState {
    pub shadows: HashMap<ShadowKey, ThingShadow>,
    pub published_messages: Vec<PublishedMessage>,
    pub retained_messages: HashMap<String, RetainedMessage>,
}

#[derive(Debug, Error)]
pub enum IotDataPlaneError {
    #[error("No shadow exists with name: '{shadow_name}'")]
    ShadowNotFound { shadow_name: String },
    #[error("No retained message found for topic: '{topic}'")]
    RetainedMessageNotFound { topic: String },
    #[error("Invalid JSON in shadow document")]
    InvalidShadowDocument,
}

impl IotDataPlaneState {
    pub fn get_thing_shadow(
        &self,
        thing_name: &str,
        shadow_name: Option<&str>,
    ) -> Result<&ThingShadow, IotDataPlaneError> {
        let key = match shadow_name {
            Some(name) => ShadowKey::named(thing_name, name),
            None => ShadowKey::classic(thing_name),
        };
        self.shadows
            .get(&key)
            .ok_or_else(|| IotDataPlaneError::ShadowNotFound {
                shadow_name: shadow_name.unwrap_or("classic").to_string(),
            })
    }

    pub fn update_thing_shadow(
        &mut self,
        thing_name: &str,
        shadow_name: Option<&str>,
        payload: Vec<u8>,
    ) -> Result<&ThingShadow, IotDataPlaneError> {
        let incoming: serde_json::Value = serde_json::from_slice(&payload)
            .map_err(|_| IotDataPlaneError::InvalidShadowDocument)?;

        let new_state = incoming
            .get("state")
            .cloned()
            .unwrap_or(serde_json::Value::Object(Default::default()));

        let key = match shadow_name {
            Some(name) => ShadowKey::named(thing_name, name),
            None => ShadowKey::classic(thing_name),
        };

        let version = self.shadows.get(&key).map(|s| s.version + 1).unwrap_or(1);
        let now = Utc::now();

        let full_doc = serde_json::json!({
            "state": new_state,
            "metadata": {},
            "version": version,
            "timestamp": now.timestamp(),
        });

        let shadow = ThingShadow {
            thing_name: thing_name.to_string(),
            shadow_name: shadow_name.map(|s| s.to_string()),
            payload: full_doc.to_string().into_bytes(),
            version,
            last_modified: now,
        };

        self.shadows.insert(key.clone(), shadow);
        Ok(self.shadows.get(&key).unwrap())
    }

    pub fn delete_thing_shadow(
        &mut self,
        thing_name: &str,
        shadow_name: Option<&str>,
    ) -> Result<ThingShadow, IotDataPlaneError> {
        let key = match shadow_name {
            Some(name) => ShadowKey::named(thing_name, name),
            None => ShadowKey::classic(thing_name),
        };
        self.shadows
            .remove(&key)
            .ok_or_else(|| IotDataPlaneError::ShadowNotFound {
                shadow_name: shadow_name.unwrap_or("classic").to_string(),
            })
    }

    pub fn list_named_shadows_for_thing(&self, thing_name: &str) -> Vec<String> {
        self.shadows
            .keys()
            .filter(|k| k.thing_name == thing_name && k.shadow_name.is_some())
            .filter_map(|k| k.shadow_name.clone())
            .collect()
    }

    pub fn publish(&mut self, topic: &str, payload: Vec<u8>, qos: i32, retain: bool) {
        let now = Utc::now();
        if retain {
            let retained = RetainedMessage {
                topic: topic.to_string(),
                payload: payload.clone(),
                qos,
                last_modified: now,
            };
            self.retained_messages.insert(topic.to_string(), retained);
        }
        let msg = PublishedMessage {
            topic: topic.to_string(),
            payload,
            qos,
            retain,
            published_at: now,
        };
        self.published_messages.push(msg);
    }

    pub fn get_retained_message(&self, topic: &str) -> Result<&RetainedMessage, IotDataPlaneError> {
        self.retained_messages.get(topic).ok_or_else(|| {
            IotDataPlaneError::RetainedMessageNotFound {
                topic: topic.to_string(),
            }
        })
    }

    pub fn list_retained_messages(&self) -> Vec<&RetainedMessage> {
        self.retained_messages.values().collect()
    }
}
