use std::collections::HashMap;

use thiserror::Error;

use crate::types::*;

/// In-memory state for the Direct Connect service.
#[derive(Debug, Default)]
pub struct DirectConnectState {
    /// Connections keyed by connection ID.
    pub connections: HashMap<String, Connection>,
}

/// Error type for Direct Connect operations.
#[derive(Debug, Error)]
pub enum DirectConnectError {
    #[error("Connection name must not be empty")]
    ConnectionNameEmpty,

    #[error("Location must not be empty")]
    LocationEmpty,

    #[error("Bandwidth must not be empty")]
    BandwidthEmpty,

    #[error("Connection ID '{connection_id}' does not exist.")]
    ConnectionNotFound { connection_id: String },

    #[error("Resource ARN '{arn}' is not a valid Direct Connect resource ARN.")]
    InvalidResourceArn { arn: String },
}

impl DirectConnectState {
    pub fn create_connection(
        &mut self,
        connection_name: &str,
        location: &str,
        bandwidth: &str,
        owner_account: &str,
        region: &str,
        tags: HashMap<String, String>,
    ) -> Result<&Connection, DirectConnectError> {
        // Validate required fields
        if connection_name.is_empty() {
            return Err(DirectConnectError::ConnectionNameEmpty);
        }
        if location.is_empty() {
            return Err(DirectConnectError::LocationEmpty);
        }
        if bandwidth.is_empty() {
            return Err(DirectConnectError::BandwidthEmpty);
        }

        let connection_id = format!("dxcon-{}", &uuid::Uuid::new_v4().to_string()[..8]);
        let vlan = 0;

        let connection = Connection {
            connection_id: connection_id.clone(),
            connection_name: connection_name.to_string(),
            connection_state: ConnectionState::Requested,
            region: region.to_string(),
            location: location.to_string(),
            bandwidth: bandwidth.to_string(),
            owner_account: owner_account.to_string(),
            vlan,
            partner_name: None,
            loa_issue_time: None,
            tags,
        };

        self.connections.insert(connection_id.clone(), connection);
        Ok(self.connections.get(&connection_id).unwrap())
    }

    pub fn describe_connections(
        &self,
        connection_id: Option<&str>,
    ) -> Result<Vec<&Connection>, DirectConnectError> {
        match connection_id {
            Some(id) => {
                if let Some(conn) = self.connections.get(id) {
                    Ok(vec![conn])
                } else {
                    Err(DirectConnectError::ConnectionNotFound {
                        connection_id: id.to_string(),
                    })
                }
            }
            None => Ok(self.connections.values().collect()),
        }
    }

    pub fn delete_connection(
        &mut self,
        connection_id: &str,
    ) -> Result<Connection, DirectConnectError> {
        match self.connections.remove(connection_id) {
            Some(mut conn) => {
                conn.connection_state = ConnectionState::Deleted;
                Ok(conn)
            }
            None => Err(DirectConnectError::ConnectionNotFound {
                connection_id: connection_id.to_string(),
            }),
        }
    }

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), DirectConnectError> {
        let connection_id = parse_connection_arn(resource_arn)?;
        match self.connections.get_mut(&connection_id) {
            Some(conn) => {
                for (k, v) in tags {
                    conn.tags.insert(k, v);
                }
                Ok(())
            }
            None => Err(DirectConnectError::ConnectionNotFound { connection_id }),
        }
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), DirectConnectError> {
        let connection_id = parse_connection_arn(resource_arn)?;
        match self.connections.get_mut(&connection_id) {
            Some(conn) => {
                for k in tag_keys {
                    conn.tags.remove(k);
                }
                Ok(())
            }
            None => Err(DirectConnectError::ConnectionNotFound { connection_id }),
        }
    }

    /// Look up tags for each requested ARN. Entries for ARNs that do not
    /// resolve to a known connection are returned with `None`.
    pub fn describe_tags(
        &self,
        resource_arns: &[String],
    ) -> Vec<(String, Option<HashMap<String, String>>)> {
        resource_arns
            .iter()
            .map(|arn| {
                let tags = parse_connection_arn(arn)
                    .ok()
                    .and_then(|cid| self.connections.get(&cid))
                    .map(|conn| conn.tags.clone());
                (arn.clone(), tags)
            })
            .collect()
    }

    pub fn describe_locations(&self, region: &str) -> Vec<Location> {
        // Return a set of mock locations for the region
        vec![
            Location {
                location_code: "EqDC2".to_string(),
                location_name: "Equinix DC2, Ashburn, VA".to_string(),
                region: region.to_string(),
                available_port_speeds: vec!["1Gbps".to_string(), "10Gbps".to_string()],
                available_providers: vec!["Equinix".to_string()],
                available_mac_sec_port_speeds: vec!["10Gbps".to_string()],
            },
            Location {
                location_code: "EqSe2".to_string(),
                location_name: "Equinix SE2, Seattle, WA".to_string(),
                region: region.to_string(),
                available_port_speeds: vec!["1Gbps".to_string(), "10Gbps".to_string()],
                available_providers: vec!["Equinix".to_string()],
                available_mac_sec_port_speeds: vec!["10Gbps".to_string()],
            },
        ]
    }
}

/// Parse `arn:aws:directconnect:<region>:<account>:dxcon/<connection-id>` and
/// return the connection id portion.
fn parse_connection_arn(arn: &str) -> Result<String, DirectConnectError> {
    let parts: Vec<&str> = arn.splitn(6, ':').collect();
    if parts.len() != 6 || parts[0] != "arn" || parts[2] != "directconnect" {
        return Err(DirectConnectError::InvalidResourceArn {
            arn: arn.to_string(),
        });
    }
    if let Some(cid) = parts[5].strip_prefix("dxcon/") {
        if cid.is_empty() {
            Err(DirectConnectError::InvalidResourceArn {
                arn: arn.to_string(),
            })
        } else {
            Ok(cid.to_string())
        }
    } else {
        Err(DirectConnectError::InvalidResourceArn {
            arn: arn.to_string(),
        })
    }
}
