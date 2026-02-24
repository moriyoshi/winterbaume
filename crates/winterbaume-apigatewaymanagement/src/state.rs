use std::collections::HashMap;

use chrono::Utc;

use crate::types::Connection;

#[derive(Debug, Default)]
pub struct ApiGatewayManagementApiState {
    pub connections: HashMap<String, Connection>,
}

#[derive(Debug, thiserror::Error)]
pub enum ApiGatewayManagementApiError {
    #[error("Not found")]
    NotFound,
}

impl ApiGatewayManagementApiState {
    pub fn get_connection(&mut self, connection_id: &str) -> &Connection {
        self.connections
            .entry(connection_id.to_string())
            .or_default()
    }

    pub fn delete_connection(&mut self, connection_id: &str) {
        self.connections.remove(connection_id);
    }

    pub fn post_to_connection(&mut self, connection_id: &str, data: Vec<u8>) {
        let conn = self
            .connections
            .entry(connection_id.to_string())
            .or_default();
        conn.data.extend_from_slice(&data);
        conn.last_active_at = Utc::now();
    }
}
