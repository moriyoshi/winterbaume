use std::collections::HashMap;

use thiserror::Error;

#[derive(Debug, Default)]
pub struct PcaConnectorScepState {
    /// Connectors keyed by ARN.
    pub connectors: HashMap<String, ConnectorRecord>,
    /// Challenges keyed by ARN.
    pub challenges: HashMap<String, ChallengeRecord>,
    /// Tags keyed by ARN.
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone)]
pub struct ConnectorRecord {
    pub arn: String,
    pub certificate_authority_arn: String,
    pub created_at: f64,
    pub updated_at: f64,
    pub status: String,
    pub connector_type: String,
    pub endpoint: String,
}

#[derive(Debug, Clone)]
pub struct ChallengeRecord {
    pub arn: String,
    pub connector_arn: String,
    pub password: String,
    pub created_at: f64,
    pub updated_at: f64,
}

#[derive(Debug, Error)]
pub enum PcaConnectorScepError {
    #[error("Connector {arn} not found.")]
    ConnectorNotFound { arn: String },

    #[error("Challenge {arn} not found.")]
    ChallengeNotFound { arn: String },

    #[error("{message}")]
    Validation { message: String },
}

impl PcaConnectorScepState {
    pub fn create_connector(&mut self, c: ConnectorRecord) -> &ConnectorRecord {
        let arn = c.arn.clone();
        self.connectors.insert(arn.clone(), c);
        self.connectors.get(&arn).unwrap()
    }

    pub fn get_connector(&self, arn: &str) -> Result<&ConnectorRecord, PcaConnectorScepError> {
        self.connectors
            .get(arn)
            .ok_or(PcaConnectorScepError::ConnectorNotFound {
                arn: arn.to_string(),
            })
    }

    pub fn delete_connector(&mut self, arn: &str) -> Result<(), PcaConnectorScepError> {
        self.connectors
            .remove(arn)
            .ok_or(PcaConnectorScepError::ConnectorNotFound {
                arn: arn.to_string(),
            })?;
        // Cascade delete challenges that reference this connector.
        self.challenges.retain(|_, c| c.connector_arn != arn);
        self.tags.remove(arn);
        Ok(())
    }

    pub fn list_connectors(&self) -> Vec<&ConnectorRecord> {
        let mut v: Vec<&ConnectorRecord> = self.connectors.values().collect();
        v.sort_by(|a, b| a.arn.cmp(&b.arn));
        v
    }

    pub fn create_challenge(
        &mut self,
        c: ChallengeRecord,
    ) -> Result<&ChallengeRecord, PcaConnectorScepError> {
        if !self.connectors.contains_key(&c.connector_arn) {
            return Err(PcaConnectorScepError::ConnectorNotFound {
                arn: c.connector_arn.clone(),
            });
        }
        let arn = c.arn.clone();
        self.challenges.insert(arn.clone(), c);
        Ok(self.challenges.get(&arn).unwrap())
    }

    pub fn get_challenge(&self, arn: &str) -> Result<&ChallengeRecord, PcaConnectorScepError> {
        self.challenges
            .get(arn)
            .ok_or(PcaConnectorScepError::ChallengeNotFound {
                arn: arn.to_string(),
            })
    }

    pub fn delete_challenge(&mut self, arn: &str) -> Result<(), PcaConnectorScepError> {
        self.challenges
            .remove(arn)
            .ok_or(PcaConnectorScepError::ChallengeNotFound {
                arn: arn.to_string(),
            })?;
        self.tags.remove(arn);
        Ok(())
    }

    pub fn list_challenges<'a>(
        &'a self,
        connector_arn: Option<&'a str>,
    ) -> Vec<&'a ChallengeRecord> {
        let mut v: Vec<&ChallengeRecord> = self
            .challenges
            .values()
            .filter(|c| connector_arn.is_none_or(|a| a == c.connector_arn))
            .collect();
        v.sort_by(|a, b| a.arn.cmp(&b.arn));
        v
    }

    pub fn tag_resource(&mut self, arn: &str, tags: HashMap<String, String>) {
        let entry = self.tags.entry(arn.to_string()).or_default();
        for (k, v) in tags {
            entry.insert(k, v);
        }
    }

    pub fn untag_resource(&mut self, arn: &str, keys: &[String]) {
        if let Some(entry) = self.tags.get_mut(arn) {
            for k in keys {
                entry.remove(k);
            }
        }
    }

    pub fn list_tags(&self, arn: &str) -> HashMap<String, String> {
        self.tags.get(arn).cloned().unwrap_or_default()
    }
}
