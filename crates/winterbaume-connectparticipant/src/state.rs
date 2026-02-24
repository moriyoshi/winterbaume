use std::collections::HashMap;

use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Default)]
pub struct ConnectParticipantState {
    /// Per-participant connection record keyed by ParticipantToken.
    pub connections: HashMap<String, ConnectionRecord>,
    /// Transcript items per ConnectionToken in arrival order.
    pub transcripts: HashMap<String, Vec<Value>>,
    /// Attachment metadata keyed by AttachmentId.
    pub attachments: HashMap<String, AttachmentRecord>,
}

#[derive(Debug, Clone)]
pub struct ConnectionRecord {
    pub participant_token: String,
    pub connection_token: String,
    pub disconnected: bool,
}

#[derive(Debug, Clone)]
pub struct AttachmentRecord {
    pub attachment_id: String,
    pub connection_token: String,
    pub name: String,
    pub size_in_bytes: i64,
    pub content_type: String,
    pub status: String,
}

#[derive(Debug, Error)]
pub enum ConnectParticipantError {
    #[error("Participant connection for token {token} not found.")]
    ConnectionNotFound { token: String },

    #[error("Attachment {id} not found.")]
    AttachmentNotFound { id: String },

    #[error("{message}")]
    Validation { message: String },
}

impl ConnectParticipantState {
    pub fn create_connection(&mut self, participant_token: String) -> &ConnectionRecord {
        let connection_token = format!("conn-{}", uuid::Uuid::new_v4().simple());
        let record = ConnectionRecord {
            participant_token: participant_token.clone(),
            connection_token: connection_token.clone(),
            disconnected: false,
        };
        self.connections.insert(participant_token.clone(), record);
        self.connections.get(&participant_token).unwrap()
    }

    pub fn find_by_connection_token(&self, connection_token: &str) -> Option<&ConnectionRecord> {
        self.connections
            .values()
            .find(|c| c.connection_token == connection_token)
    }

    pub fn require_active(
        &self,
        connection_token: &str,
    ) -> Result<&ConnectionRecord, ConnectParticipantError> {
        match self.find_by_connection_token(connection_token) {
            Some(c) if !c.disconnected => Ok(c),
            Some(_) | None => Err(ConnectParticipantError::ConnectionNotFound {
                token: connection_token.to_string(),
            }),
        }
    }

    pub fn disconnect(&mut self, connection_token: &str) -> Result<(), ConnectParticipantError> {
        let participant = match self.find_by_connection_token(connection_token) {
            Some(c) => c.participant_token.clone(),
            None => {
                return Err(ConnectParticipantError::ConnectionNotFound {
                    token: connection_token.to_string(),
                });
            }
        };
        if let Some(c) = self.connections.get_mut(&participant) {
            c.disconnected = true;
        }
        Ok(())
    }

    pub fn append_transcript(&mut self, connection_token: &str, item: Value) {
        self.transcripts
            .entry(connection_token.to_string())
            .or_default()
            .push(item);
    }

    pub fn transcript(&self, connection_token: &str) -> &[Value] {
        self.transcripts
            .get(connection_token)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn upsert_attachment(&mut self, record: AttachmentRecord) {
        self.attachments
            .insert(record.attachment_id.clone(), record);
    }

    pub fn complete_attachments(&mut self, ids: &[String]) -> Result<(), ConnectParticipantError> {
        for id in ids {
            let r = self
                .attachments
                .get_mut(id)
                .ok_or(ConnectParticipantError::AttachmentNotFound { id: id.clone() })?;
            r.status = "APPROVED".to_string();
        }
        Ok(())
    }
}
