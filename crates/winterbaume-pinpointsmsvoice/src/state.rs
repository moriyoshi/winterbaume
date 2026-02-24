use std::collections::HashMap;

use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Default)]
pub struct PinpointSmsVoiceState {
    pub configuration_sets: HashMap<String, ConfigurationSet>,
    /// Voice messages sent through SendVoiceMessage, keyed by message_id.
    pub messages: HashMap<String, Value>,
}

#[derive(Debug, Clone, Default)]
pub struct ConfigurationSet {
    pub name: String,
    /// Event destinations keyed by destination name.
    pub event_destinations: HashMap<String, Value>,
}

#[derive(Debug, Error)]
pub enum PinpointSmsVoiceError {
    #[error("Configuration set {name} not found.")]
    NotFound { name: String },

    #[error("Configuration set {name} already exists.")]
    AlreadyExists { name: String },

    #[error("Event destination {name} not found.")]
    DestinationNotFound { name: String },

    #[error("{message}")]
    Validation { message: String },
}

impl PinpointSmsVoiceState {
    pub fn create_configuration_set(
        &mut self,
        name: &str,
    ) -> Result<&ConfigurationSet, PinpointSmsVoiceError> {
        if self.configuration_sets.contains_key(name) {
            return Err(PinpointSmsVoiceError::AlreadyExists {
                name: name.to_string(),
            });
        }
        let cs = ConfigurationSet {
            name: name.to_string(),
            event_destinations: HashMap::new(),
        };
        self.configuration_sets.insert(name.to_string(), cs);
        Ok(self.configuration_sets.get(name).unwrap())
    }

    pub fn delete_configuration_set(&mut self, name: &str) -> Result<(), PinpointSmsVoiceError> {
        self.configuration_sets
            .remove(name)
            .ok_or(PinpointSmsVoiceError::NotFound {
                name: name.to_string(),
            })?;
        Ok(())
    }

    pub fn list_configuration_sets(&self) -> Vec<&str> {
        let mut names: Vec<&str> = self.configuration_sets.keys().map(String::as_str).collect();
        names.sort();
        names
    }

    pub fn put_event_destination(
        &mut self,
        cs_name: &str,
        dest_name: &str,
        destination: Value,
    ) -> Result<(), PinpointSmsVoiceError> {
        let cs =
            self.configuration_sets
                .get_mut(cs_name)
                .ok_or(PinpointSmsVoiceError::NotFound {
                    name: cs_name.to_string(),
                })?;
        cs.event_destinations
            .insert(dest_name.to_string(), destination);
        Ok(())
    }

    pub fn delete_event_destination(
        &mut self,
        cs_name: &str,
        dest_name: &str,
    ) -> Result<(), PinpointSmsVoiceError> {
        let cs =
            self.configuration_sets
                .get_mut(cs_name)
                .ok_or(PinpointSmsVoiceError::NotFound {
                    name: cs_name.to_string(),
                })?;
        cs.event_destinations.remove(dest_name).ok_or(
            PinpointSmsVoiceError::DestinationNotFound {
                name: dest_name.to_string(),
            },
        )?;
        Ok(())
    }

    pub fn get_event_destinations(
        &self,
        cs_name: &str,
    ) -> Result<&HashMap<String, Value>, PinpointSmsVoiceError> {
        self.configuration_sets
            .get(cs_name)
            .map(|cs| &cs.event_destinations)
            .ok_or(PinpointSmsVoiceError::NotFound {
                name: cs_name.to_string(),
            })
    }

    pub fn record_message(&mut self, message_id: String, payload: Value) {
        self.messages.insert(message_id, payload);
    }
}
