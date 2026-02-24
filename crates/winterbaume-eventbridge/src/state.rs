use std::collections::HashMap;

use crate::types::*;

/// In-memory state for the EventBridge service.
#[derive(Debug)]
pub struct EventsState {
    /// Rules keyed by rule name.
    pub rules: HashMap<String, Rule>,
    /// Event buses keyed by name.
    pub event_buses: HashMap<String, EventBus>,
    /// Archives keyed by name.
    pub archives: HashMap<String, Archive>,
    /// Connections keyed by name.
    pub connections: HashMap<String, Connection>,
    /// API destinations keyed by name.
    pub api_destinations: HashMap<String, ApiDestination>,
    /// Replays keyed by name.
    pub replays: HashMap<String, Replay>,
    /// Partner event sources keyed by name.
    pub partner_event_sources: HashMap<String, PartnerEventSource>,
    /// Tags keyed by resource ARN.
    pub tags: HashMap<String, Vec<Tag>>,
    /// Endpoints keyed by name.
    pub endpoints: HashMap<String, Endpoint>,
}

impl Default for EventsState {
    fn default() -> Self {
        let mut event_buses = HashMap::new();
        event_buses.insert(
            "default".to_string(),
            EventBus {
                name: "default".to_string(),
                arn: String::new(), // Will be set with proper account/region on first access
                policy: None,
                description: None,
                kms_key_identifier: None,
            },
        );
        Self {
            rules: HashMap::new(),
            event_buses,
            archives: HashMap::new(),
            connections: HashMap::new(),
            api_destinations: HashMap::new(),
            replays: HashMap::new(),
            partner_event_sources: HashMap::new(),
            tags: HashMap::new(),
            endpoints: HashMap::new(),
        }
    }
}

/// Error type for EventBridge operations.
#[derive(Debug, thiserror::Error)]
pub enum EventsError {
    #[error("Rule can't be deleted since it has targets.")]
    RuleHasTargets,
    #[error("Event bus {0} already exists.")]
    EventBusAlreadyExists(String),
    #[error("Cannot delete event bus default.")]
    CannotDeleteDefaultBus,
    #[error("Provided value in parameter 'action' is not supported.")]
    InvalidPermissionAction,
    #[error("EventBus does not have a policy.")]
    EventBusHasNoPolicy,
    #[error("Statement with the provided id does not exist.")]
    StatementNotFound,
    #[error("Archive {0} already exists.")]
    ArchiveAlreadyExists(String),
    #[error("Connection {0} already exists.")]
    ConnectionAlreadyExists(String),
    #[error("Connection '{0}' does not exist.")]
    ConnectionNotFound(String),
    #[error("Api destination {0} already exists.")]
    ApiDestinationAlreadyExists(String),
    #[error("An api-destination '{0}' does not exist.")]
    ApiDestinationNotFound(String),
    #[error("Replay {0} already exists.")]
    ReplayAlreadyExists(String),
    #[error("Replay {0} is not in a valid state for this operation.")]
    ReplayInvalidState(String),
    #[error("Partner event source {0} already exists.")]
    PartnerEventSourceAlreadyExists(String),
    #[error("Endpoint {0} already exists.")]
    EndpointAlreadyExists(String),
    #[error("{0} {1} does not exist.")]
    ResourceNotFound(String, String),
}

impl EventsState {
    pub fn ensure_default_bus(&mut self, account_id: &str, region: &str) {
        if let Some(bus) = self.event_buses.get_mut("default")
            && bus.arn.is_empty()
        {
            bus.arn = format!("arn:aws:events:{region}:{account_id}:event-bus/default");
        }
    }

    pub fn put_rule(
        &mut self,
        name: &str,
        account_id: &str,
        region: &str,
        event_pattern: Option<&str>,
        schedule_expression: Option<&str>,
        state: Option<&str>,
        description: Option<&str>,
        event_bus_name: Option<&str>,
    ) -> Result<&Rule, EventsError> {
        let arn = format!("arn:aws:events:{region}:{account_id}:rule/{name}");
        let rule_state = state.map(RuleState::parse).unwrap_or(RuleState::Enabled);
        let bus_name = event_bus_name.unwrap_or("default").to_string();

        // PutRule is idempotent - update if exists, create if not
        let rule = self.rules.entry(name.to_string()).or_insert_with(|| Rule {
            name: name.to_string(),
            arn: arn.clone(),
            event_pattern: None,
            schedule_expression: None,
            state: RuleState::Enabled,
            description: None,
            event_bus_name: bus_name.clone(),
            targets: Vec::new(),
            tags: Vec::new(),
        });

        rule.arn = arn;
        rule.event_pattern = event_pattern.map(|s| s.to_string());
        rule.schedule_expression = schedule_expression.map(|s| s.to_string());
        rule.state = rule_state;
        rule.description = description.map(|s| s.to_string());
        rule.event_bus_name = bus_name;

        Ok(self.rules.get(name).unwrap())
    }

    pub fn delete_rule(&mut self, name: &str) -> Result<(), EventsError> {
        if let Some(rule) = self.rules.get(name)
            && !rule.targets.is_empty()
        {
            return Err(EventsError::RuleHasTargets);
        }
        // Deleting an unknown rule succeeds silently (verified against AWS)
        self.rules.remove(name);
        // Also remove tags for this rule
        let arn_suffix = format!("rule/{name}");
        let keys_to_remove: Vec<String> = self
            .tags
            .keys()
            .filter(|k| k.ends_with(&arn_suffix))
            .cloned()
            .collect();
        for key in keys_to_remove {
            self.tags.remove(&key);
        }
        Ok(())
    }

    pub fn describe_rule(&self, name: &str) -> Result<&Rule, EventsError> {
        self.rules
            .get(name)
            .ok_or_else(|| not_found_error("Rule", name))
    }

    pub fn list_rules(&self, name_prefix: Option<&str>) -> Vec<&Rule> {
        self.rules
            .values()
            .filter(|r| match name_prefix {
                Some(prefix) => r.name.starts_with(prefix),
                None => true,
            })
            .collect()
    }

    pub fn put_targets(
        &mut self,
        rule_name: &str,
        targets: Vec<Target>,
    ) -> Result<(), EventsError> {
        let rule = self
            .rules
            .get_mut(rule_name)
            .ok_or_else(|| not_found_error("Rule", rule_name))?;

        for target in targets {
            // Replace existing target with same ID, or add new
            if let Some(existing) = rule.targets.iter_mut().find(|t| t.id == target.id) {
                *existing = target;
            } else {
                rule.targets.push(target);
            }
        }
        Ok(())
    }

    pub fn remove_targets(&mut self, rule_name: &str, ids: &[String]) -> Result<(), EventsError> {
        let rule = self
            .rules
            .get_mut(rule_name)
            .ok_or_else(|| not_found_error("Rule", rule_name))?;

        rule.targets.retain(|t| !ids.contains(&t.id));
        Ok(())
    }

    pub fn list_targets_by_rule(&self, rule_name: &str) -> Result<Vec<&Target>, EventsError> {
        let rule = self
            .rules
            .get(rule_name)
            .ok_or_else(|| not_found_error("Rule", rule_name))?;
        Ok(rule.targets.iter().collect())
    }

    pub fn put_events(&self, entries: &[PutEventsEntry]) -> Vec<PutEventsResultEntry> {
        entries
            .iter()
            .map(|_| PutEventsResultEntry {
                event_id: uuid::Uuid::new_v4().to_string(),
            })
            .collect()
    }

    // --- EventBus operations ---

    pub fn create_event_bus(
        &mut self,
        name: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&EventBus, EventsError> {
        if self.event_buses.contains_key(name) {
            return Err(EventsError::EventBusAlreadyExists(name.to_string()));
        }
        let arn = format!("arn:aws:events:{region}:{account_id}:event-bus/{name}");
        self.event_buses.insert(
            name.to_string(),
            EventBus {
                name: name.to_string(),
                arn,
                policy: None,
                description: None,
                kms_key_identifier: None,
            },
        );
        Ok(self.event_buses.get(name).unwrap())
    }

    pub fn delete_event_bus(&mut self, name: &str) -> Result<(), EventsError> {
        if name == "default" {
            return Err(EventsError::CannotDeleteDefaultBus);
        }
        // Deleting a non-existing event bus succeeds silently (verified against AWS)
        self.event_buses.remove(name);
        Ok(())
    }

    pub fn describe_event_bus(&self, name: &str) -> Result<&EventBus, EventsError> {
        self.event_buses
            .get(name)
            .ok_or_else(|| not_found_error("Event bus", name))
    }

    pub fn list_event_buses(&self, name_prefix: Option<&str>) -> Vec<&EventBus> {
        self.event_buses
            .values()
            .filter(|b| match name_prefix {
                Some(prefix) => b.name.starts_with(prefix),
                None => true,
            })
            .collect()
    }

    pub fn put_permission(
        &mut self,
        event_bus_name: &str,
        policy: Option<&str>,
        action: Option<&str>,
        principal: Option<&str>,
        statement_id: Option<&str>,
    ) -> Result<(), EventsError> {
        let bus = self
            .event_buses
            .get_mut(event_bus_name)
            .ok_or_else(|| not_found_error("Event bus", event_bus_name))?;

        if let Some(policy_str) = policy {
            // Direct policy mode
            bus.policy = Some(policy_str.to_string());
        } else if let (Some(action), Some(principal), Some(sid)) = (action, principal, statement_id)
        {
            // Validate action
            if action != "events:PutEvents" {
                return Err(EventsError::InvalidPermissionAction);
            }

            let bus_arn = bus.arn.clone();
            let statement = serde_json::json!({
                "Sid": sid,
                "Effect": "Allow",
                "Principal": {"AWS": format!("arn:aws:iam::{principal}:root")},
                "Action": action,
                "Resource": bus_arn,
            });

            // Parse existing policy or create new one
            let mut policy_doc = if let Some(ref existing) = bus.policy {
                serde_json::from_str::<serde_json::Value>(existing).unwrap_or_else(|_| {
                    serde_json::json!({
                        "Version": "2012-10-17",
                        "Statement": []
                    })
                })
            } else {
                serde_json::json!({
                    "Version": "2012-10-17",
                    "Statement": []
                })
            };

            if let Some(statements) = policy_doc
                .get_mut("Statement")
                .and_then(|v| v.as_array_mut())
            {
                // Replace existing statement with same Sid, or add new
                if let Some(pos) = statements
                    .iter()
                    .position(|s| s.get("Sid").and_then(|v| v.as_str()) == Some(sid))
                {
                    statements[pos] = statement;
                } else {
                    statements.push(statement);
                }
            }

            bus.policy = Some(serde_json::to_string(&policy_doc).unwrap());
        }

        Ok(())
    }

    pub fn remove_permission(
        &mut self,
        event_bus_name: &str,
        statement_id: Option<&str>,
        remove_all: bool,
    ) -> Result<(), EventsError> {
        let bus = self
            .event_buses
            .get_mut(event_bus_name)
            .ok_or_else(|| not_found_error("Event bus", event_bus_name))?;

        if remove_all {
            bus.policy = None;
            return Ok(());
        }

        if let Some(sid) = statement_id {
            let policy_str = bus
                .policy
                .as_ref()
                .ok_or(EventsError::EventBusHasNoPolicy)?;

            let mut policy_doc: serde_json::Value = serde_json::from_str(policy_str)
                .unwrap_or_else(|_| serde_json::json!({"Version": "2012-10-17", "Statement": []}));

            if let Some(statements) = policy_doc
                .get_mut("Statement")
                .and_then(|v| v.as_array_mut())
            {
                let before_len = statements.len();
                statements.retain(|s| s.get("Sid").and_then(|v| v.as_str()) != Some(sid));
                if statements.len() == before_len {
                    return Err(EventsError::StatementNotFound);
                }
                if statements.is_empty() {
                    bus.policy = None;
                } else {
                    bus.policy = Some(serde_json::to_string(&policy_doc).unwrap());
                }
            }
        } else {
            bus.policy = None;
        }

        Ok(())
    }

    // --- Archive operations ---

    pub fn create_archive(
        &mut self,
        name: &str,
        account_id: &str,
        region: &str,
        event_source_arn: &str,
        description: Option<&str>,
        event_pattern: Option<&str>,
        retention_days: i64,
    ) -> Result<&Archive, EventsError> {
        if self.archives.contains_key(name) {
            return Err(EventsError::ArchiveAlreadyExists(name.to_string()));
        }
        let arn = format!("arn:aws:events:{region}:{account_id}:archive/{name}");
        let now = chrono::Utc::now().timestamp() as f64;
        self.archives.insert(
            name.to_string(),
            Archive {
                name: name.to_string(),
                arn,
                event_source_arn: event_source_arn.to_string(),
                description: description.map(|s| s.to_string()),
                event_pattern: event_pattern.map(|s| s.to_string()),
                retention_days,
                state: "ENABLED".to_string(),
                creation_time: now,
                size_bytes: 0,
            },
        );
        Ok(self.archives.get(name).unwrap())
    }

    pub fn delete_archive(&mut self, name: &str) -> Result<(), EventsError> {
        self.archives
            .remove(name)
            .ok_or_else(|| not_found_error("Archive", name))?;
        Ok(())
    }

    pub fn describe_archive(&self, name: &str) -> Result<&Archive, EventsError> {
        self.archives
            .get(name)
            .ok_or_else(|| not_found_error("Archive", name))
    }

    pub fn update_archive(
        &mut self,
        name: &str,
        description: Option<&str>,
        event_pattern: Option<&str>,
        retention_days: Option<i64>,
    ) -> Result<&Archive, EventsError> {
        let archive = self
            .archives
            .get_mut(name)
            .ok_or_else(|| not_found_error("Archive", name))?;
        if let Some(d) = description {
            archive.description = Some(d.to_string());
        }
        if let Some(ep) = event_pattern {
            archive.event_pattern = Some(ep.to_string());
        }
        if let Some(rd) = retention_days {
            archive.retention_days = rd;
        }
        Ok(self.archives.get(name).unwrap())
    }

    pub fn list_archives(&self, name_prefix: Option<&str>) -> Vec<&Archive> {
        self.archives
            .values()
            .filter(|a| match name_prefix {
                Some(prefix) => a.name.starts_with(prefix),
                None => true,
            })
            .collect()
    }

    // --- Connection operations ---

    pub fn create_connection(
        &mut self,
        name: &str,
        account_id: &str,
        region: &str,
        description: Option<&str>,
        authorization_type: &str,
        auth_parameters: &str,
    ) -> Result<&Connection, EventsError> {
        if self.connections.contains_key(name) {
            return Err(EventsError::ConnectionAlreadyExists(name.to_string()));
        }
        let uuid = uuid::Uuid::new_v4();
        let arn = format!("arn:aws:events:{region}:{account_id}:connection/{name}/{uuid}");
        let now = chrono::Utc::now().timestamp() as f64;
        self.connections.insert(
            name.to_string(),
            Connection {
                name: name.to_string(),
                arn,
                state: "AUTHORIZED".to_string(),
                description: description.map(|s| s.to_string()),
                authorization_type: authorization_type.to_string(),
                auth_parameters: auth_parameters.to_string(),
                creation_time: now,
            },
        );
        Ok(self.connections.get(name).unwrap())
    }

    pub fn remove_connection(&mut self, name: &str) -> Result<Connection, EventsError> {
        self.connections
            .remove(name)
            .ok_or_else(|| not_found_error("Connection", name))
    }

    pub fn describe_connection(&self, name: &str) -> Result<&Connection, EventsError> {
        self.connections
            .get(name)
            .ok_or_else(|| not_found_error("Connection", name))
    }

    pub fn update_connection(
        &mut self,
        name: &str,
        description: Option<&str>,
        authorization_type: Option<&str>,
        auth_parameters: Option<&str>,
    ) -> Result<&Connection, EventsError> {
        let conn = self
            .connections
            .get_mut(name)
            .ok_or_else(|| EventsError::ConnectionNotFound(name.to_string()))?;
        if let Some(d) = description {
            conn.description = Some(d.to_string());
        }
        if let Some(at) = authorization_type {
            conn.authorization_type = at.to_string();
        }
        if let Some(ap) = auth_parameters {
            conn.auth_parameters = ap.to_string();
        }
        Ok(self.connections.get(name).unwrap())
    }

    pub fn list_connections(&self, name_prefix: Option<&str>) -> Vec<&Connection> {
        self.connections
            .values()
            .filter(|c| match name_prefix {
                Some(prefix) => c.name.starts_with(prefix),
                None => true,
            })
            .collect()
    }

    // --- ApiDestination operations ---

    pub fn create_api_destination(
        &mut self,
        name: &str,
        account_id: &str,
        region: &str,
        description: Option<&str>,
        connection_arn: &str,
        invocation_endpoint: &str,
        http_method: &str,
        invocation_rate_limit: Option<i64>,
    ) -> Result<&ApiDestination, EventsError> {
        if self.api_destinations.contains_key(name) {
            return Err(EventsError::ApiDestinationAlreadyExists(name.to_string()));
        }
        let uuid = uuid::Uuid::new_v4();
        let arn = format!("arn:aws:events:{region}:{account_id}:api-destination/{name}/{uuid}");
        let now = chrono::Utc::now().timestamp() as f64;
        self.api_destinations.insert(
            name.to_string(),
            ApiDestination {
                name: name.to_string(),
                arn,
                description: description.map(|s| s.to_string()),
                connection_arn: connection_arn.to_string(),
                invocation_endpoint: invocation_endpoint.to_string(),
                http_method: http_method.to_string(),
                invocation_rate_limit: invocation_rate_limit.unwrap_or(300),
                state: "ACTIVE".to_string(),
                creation_time: now,
            },
        );
        Ok(self.api_destinations.get(name).unwrap())
    }

    pub fn delete_api_destination(&mut self, name: &str) -> Result<(), EventsError> {
        self.api_destinations
            .remove(name)
            .ok_or_else(|| EventsError::ApiDestinationNotFound(name.to_string()))?;
        Ok(())
    }

    pub fn describe_api_destination(&self, name: &str) -> Result<&ApiDestination, EventsError> {
        self.api_destinations
            .get(name)
            .ok_or_else(|| EventsError::ApiDestinationNotFound(name.to_string()))
    }

    pub fn update_api_destination(
        &mut self,
        name: &str,
        description: Option<&str>,
        connection_arn: Option<&str>,
        invocation_endpoint: Option<&str>,
        http_method: Option<&str>,
        invocation_rate_limit: Option<i64>,
    ) -> Result<&ApiDestination, EventsError> {
        let dest = self
            .api_destinations
            .get_mut(name)
            .ok_or_else(|| EventsError::ApiDestinationNotFound(name.to_string()))?;
        if let Some(d) = description {
            dest.description = Some(d.to_string());
        }
        if let Some(ca) = connection_arn {
            dest.connection_arn = ca.to_string();
        }
        if let Some(ie) = invocation_endpoint {
            dest.invocation_endpoint = ie.to_string();
        }
        if let Some(hm) = http_method {
            dest.http_method = hm.to_string();
        }
        if let Some(irl) = invocation_rate_limit {
            dest.invocation_rate_limit = irl;
        }
        Ok(self.api_destinations.get(name).unwrap())
    }

    pub fn list_api_destinations(&self, name_prefix: Option<&str>) -> Vec<&ApiDestination> {
        self.api_destinations
            .values()
            .filter(|d| match name_prefix {
                Some(prefix) => d.name.starts_with(prefix),
                None => true,
            })
            .collect()
    }

    // --- Replay operations ---

    pub fn start_replay(
        &mut self,
        name: &str,
        account_id: &str,
        region: &str,
        event_source_arn: &str,
        description: Option<&str>,
        start_time: f64,
        end_time: f64,
        destination: &str,
    ) -> Result<&Replay, EventsError> {
        if self.replays.contains_key(name) {
            return Err(EventsError::ReplayAlreadyExists(name.to_string()));
        }
        let arn = format!("arn:aws:events:{region}:{account_id}:replay/{name}");
        let now = chrono::Utc::now().timestamp() as f64;
        // Moto immediately transitions to COMPLETED after synchronous replay_events
        self.replays.insert(
            name.to_string(),
            Replay {
                name: name.to_string(),
                arn,
                event_source_arn: event_source_arn.to_string(),
                description: description.map(|s| s.to_string()),
                state: "COMPLETED".to_string(),
                start_time,
                end_time,
                destination: destination.to_string(),
                replay_start_time: now,
                replay_end_time: now,
            },
        );
        Ok(self.replays.get(name).unwrap())
    }

    /// Returns the replay with state "CANCELLING" for the response, then sets state to "CANCELLED".
    pub fn cancel_replay(&mut self, name: &str) -> Result<CancelReplayResult, EventsError> {
        let replay = self
            .replays
            .get_mut(name)
            .ok_or_else(|| not_found_error("Replay", name))?;

        // Can only cancel replays that are STARTING, RUNNING, or COMPLETED
        // (COMPLETED is allowed because our implementation is synchronous)
        if replay.state == "CANCELLED" || replay.state == "CANCELLING" || replay.state == "FAILED" {
            return Err(EventsError::ReplayInvalidState(name.to_string()));
        }

        let arn = replay.arn.clone();
        replay.state = "CANCELLED".to_string();
        Ok(CancelReplayResult {
            replay_arn: arn,
            state: "CANCELLING".to_string(),
        })
    }

    pub fn describe_replay(&self, name: &str) -> Result<&Replay, EventsError> {
        self.replays
            .get(name)
            .ok_or_else(|| not_found_error("Replay", name))
    }

    pub fn list_replays(&self, name_prefix: Option<&str>) -> Vec<&Replay> {
        self.replays
            .values()
            .filter(|r| match name_prefix {
                Some(prefix) => r.name.starts_with(prefix),
                None => true,
            })
            .collect()
    }

    // --- PartnerEventSource operations ---

    pub fn create_partner_event_source(
        &mut self,
        name: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&PartnerEventSource, EventsError> {
        if self.partner_event_sources.contains_key(name) {
            return Err(EventsError::PartnerEventSourceAlreadyExists(
                name.to_string(),
            ));
        }
        let arn = format!("arn:aws:events:{region}:{account_id}:event-source/{name}");
        self.partner_event_sources.insert(
            name.to_string(),
            PartnerEventSource {
                name: name.to_string(),
                arn,
                state: "ACTIVE".to_string(),
            },
        );
        Ok(self.partner_event_sources.get(name).unwrap())
    }

    pub fn delete_partner_event_source(&mut self, name: &str) -> Result<(), EventsError> {
        self.partner_event_sources
            .remove(name)
            .ok_or_else(|| not_found_error("Partner event source", name))?;
        Ok(())
    }

    pub fn describe_event_source(&self, name: &str) -> Result<&PartnerEventSource, EventsError> {
        self.partner_event_sources
            .get(name)
            .ok_or_else(|| not_found_error("Event source", name))
    }

    pub fn describe_partner_event_source(
        &self,
        name: &str,
    ) -> Result<&PartnerEventSource, EventsError> {
        self.partner_event_sources
            .get(name)
            .ok_or_else(|| not_found_error("Partner event source", name))
    }

    // --- Event source activation ---

    pub fn activate_event_source(&mut self, name: &str) -> Result<(), EventsError> {
        let source = self
            .partner_event_sources
            .get_mut(name)
            .ok_or_else(|| not_found_error("Event source", name))?;
        source.state = "ACTIVE".to_string();
        Ok(())
    }

    pub fn deactivate_event_source(&mut self, name: &str) -> Result<(), EventsError> {
        let source = self
            .partner_event_sources
            .get_mut(name)
            .ok_or_else(|| not_found_error("Event source", name))?;
        source.state = "PENDING".to_string();
        Ok(())
    }

    pub fn list_event_sources(&self, name_prefix: Option<&str>) -> Vec<&PartnerEventSource> {
        self.partner_event_sources
            .values()
            .filter(|s| match name_prefix {
                Some(prefix) => s.name.starts_with(prefix),
                None => true,
            })
            .collect()
    }

    pub fn list_partner_event_sources(
        &self,
        name_prefix: Option<&str>,
    ) -> Vec<&PartnerEventSource> {
        self.partner_event_sources
            .values()
            .filter(|s| match name_prefix {
                Some(prefix) => s.name.starts_with(prefix),
                None => true,
            })
            .collect()
    }

    // --- Rule enable/disable ---

    pub fn disable_rule(&mut self, name: &str) -> Result<(), EventsError> {
        let rule = self
            .rules
            .get_mut(name)
            .ok_or_else(|| not_found_error("Rule", name))?;
        rule.state = RuleState::Disabled;
        Ok(())
    }

    pub fn enable_rule(&mut self, name: &str) -> Result<(), EventsError> {
        let rule = self
            .rules
            .get_mut(name)
            .ok_or_else(|| not_found_error("Rule", name))?;
        rule.state = RuleState::Enabled;
        Ok(())
    }

    // --- ListRuleNamesByTarget ---

    pub fn list_rule_names_by_target(&self, target_arn: &str) -> Vec<String> {
        self.rules
            .values()
            .filter(|r| r.targets.iter().any(|t| t.arn == target_arn))
            .map(|r| r.name.clone())
            .collect()
    }

    // --- Tag operations ---

    pub fn tag_resource(&mut self, arn: &str, tags: Vec<Tag>) {
        let existing = self.tags.entry(arn.to_string()).or_default();
        for tag in tags {
            if let Some(pos) = existing.iter().position(|t| t.key == tag.key) {
                existing[pos] = tag;
            } else {
                existing.push(tag);
            }
        }
    }

    pub fn untag_resource(&mut self, arn: &str, tag_keys: &[String]) {
        if let Some(existing) = self.tags.get_mut(arn) {
            existing.retain(|t| !tag_keys.contains(&t.key));
        }
    }

    pub fn list_tags_for_resource(&self, arn: &str) -> Vec<&Tag> {
        self.tags
            .get(arn)
            .map(|tags| tags.iter().collect())
            .unwrap_or_default()
    }

    // --- TestEventPattern ---

    pub fn test_event_pattern(&self, event_pattern: &str, event: &str) -> bool {
        match_event_pattern(event_pattern, event)
    }

    // --- Endpoint operations ---

    pub fn create_endpoint(
        &mut self,
        name: &str,
        account_id: &str,
        region: &str,
        description: Option<&str>,
        routing_config: Option<&str>,
        replication_config: Option<&str>,
        event_buses: Vec<String>,
        role_arn: Option<&str>,
    ) -> Result<&Endpoint, EventsError> {
        if self.endpoints.contains_key(name) {
            return Err(EventsError::EndpointAlreadyExists(name.to_string()));
        }
        let uuid = uuid::Uuid::new_v4();
        let arn = format!("arn:aws:events:{region}:{account_id}:endpoint/{name}");
        let endpoint_id = format!("{name}.{uuid}");
        let endpoint_url = format!("{endpoint_id}.endpoint.events.amazonaws.com");
        let now = chrono::Utc::now().timestamp() as f64;
        self.endpoints.insert(
            name.to_string(),
            Endpoint {
                name: name.to_string(),
                arn,
                description: description.map(|s| s.to_string()),
                routing_config: routing_config.map(|s| s.to_string()),
                replication_config: replication_config.map(|s| s.to_string()),
                event_buses,
                role_arn: role_arn.map(|s| s.to_string()),
                state: "ACTIVE".to_string(),
                endpoint_id,
                endpoint_url,
                creation_time: now,
                last_modified_time: now,
            },
        );
        Ok(self.endpoints.get(name).unwrap())
    }

    pub fn delete_endpoint(&mut self, name: &str) -> Result<(), EventsError> {
        self.endpoints
            .remove(name)
            .ok_or_else(|| not_found_error("Endpoint", name))?;
        Ok(())
    }

    pub fn describe_endpoint(&self, name: &str) -> Result<&Endpoint, EventsError> {
        self.endpoints
            .get(name)
            .ok_or_else(|| not_found_error("Endpoint", name))
    }

    pub fn list_endpoints(&self, name_prefix: Option<&str>) -> Vec<&Endpoint> {
        self.endpoints
            .values()
            .filter(|e| match name_prefix {
                Some(prefix) => e.name.starts_with(prefix),
                None => true,
            })
            .collect()
    }

    pub fn update_endpoint(
        &mut self,
        name: &str,
        description: Option<&str>,
        routing_config: Option<&str>,
        replication_config: Option<&str>,
        event_buses: Option<Vec<String>>,
        role_arn: Option<&str>,
    ) -> Result<&Endpoint, EventsError> {
        let ep = self
            .endpoints
            .get_mut(name)
            .ok_or_else(|| not_found_error("Endpoint", name))?;
        if let Some(d) = description {
            ep.description = Some(d.to_string());
        }
        if let Some(rc) = routing_config {
            ep.routing_config = Some(rc.to_string());
        }
        if let Some(rc) = replication_config {
            ep.replication_config = Some(rc.to_string());
        }
        if let Some(buses) = event_buses {
            ep.event_buses = buses;
        }
        if let Some(ra) = role_arn {
            ep.role_arn = Some(ra.to_string());
        }
        ep.last_modified_time = chrono::Utc::now().timestamp() as f64;
        Ok(self.endpoints.get(name).unwrap())
    }

    // --- UpdateEventBus ---

    pub fn update_event_bus(
        &mut self,
        name: &str,
        description: Option<&str>,
        kms_key_identifier: Option<&str>,
    ) -> Result<&EventBus, EventsError> {
        let bus = self
            .event_buses
            .get_mut(name)
            .ok_or_else(|| not_found_error("Event bus", name))?;
        if let Some(d) = description {
            bus.description = Some(d.to_string());
        }
        if let Some(k) = kms_key_identifier {
            bus.kms_key_identifier = Some(k.to_string());
        }
        Ok(self.event_buses.get(name).unwrap())
    }

    // --- DeauthorizeConnection ---

    pub fn deauthorize_connection(&mut self, name: &str) -> Result<&Connection, EventsError> {
        let conn = self
            .connections
            .get_mut(name)
            .ok_or_else(|| EventsError::ConnectionNotFound(name.to_string()))?;
        conn.state = "DEAUTHORIZED".to_string();
        Ok(self.connections.get(name).unwrap())
    }

    // --- PutPartnerEvents ---

    pub fn put_partner_events(&self, entries: &[PutEventsEntry]) -> Vec<PutEventsResultEntry> {
        entries
            .iter()
            .map(|_| PutEventsResultEntry {
                event_id: uuid::Uuid::new_v4().to_string(),
            })
            .collect()
    }
}

/// Result from cancel_replay: the response state (CANCELLING) differs from stored state (CANCELLED).
pub struct CancelReplayResult {
    pub replay_arn: String,
    pub state: String,
}

/// Input entry for PutEvents.
pub struct PutEventsEntry {
    pub source: Option<String>,
    pub detail_type: Option<String>,
    pub detail: Option<String>,
}

/// Result entry for PutEvents.
pub struct PutEventsResultEntry {
    pub event_id: String,
}

fn not_found_error(resource_type: &str, name: &str) -> EventsError {
    EventsError::ResourceNotFound(resource_type.to_string(), name.to_string())
}

/// Match an event against an event pattern (simplified implementation).
/// Pattern values at leaf level must be arrays; each pattern key must match.
fn match_event_pattern(pattern_str: &str, event_str: &str) -> bool {
    let pattern: serde_json::Value = match serde_json::from_str(pattern_str) {
        Ok(v) => v,
        Err(_) => return false,
    };
    let event: serde_json::Value = match serde_json::from_str(event_str) {
        Ok(v) => v,
        Err(_) => return false,
    };
    matches_value(&pattern, &event)
}

fn matches_value(pattern: &serde_json::Value, event: &serde_json::Value) -> bool {
    match (pattern, event) {
        (serde_json::Value::Object(pat_map), serde_json::Value::Object(evt_map)) => {
            // Every key in pattern must match
            for (key, pat_val) in pat_map {
                match evt_map.get(key) {
                    Some(evt_val) => {
                        if !matches_value(pat_val, evt_val) {
                            return false;
                        }
                    }
                    None => {
                        // Key not present in event - check if pattern is testing for absence (exists: false)
                        if let serde_json::Value::Array(arr) = pat_val {
                            let all_not_exists = !arr.is_empty()
                                && arr.iter().all(|item| {
                                    if let serde_json::Value::Object(m) = item {
                                        m.get("exists")
                                            .and_then(|v| v.as_bool())
                                            .is_some_and(|b| !b)
                                    } else {
                                        false
                                    }
                                });
                            if all_not_exists {
                                continue;
                            }
                        }
                        return false;
                    }
                }
            }
            true
        }
        (serde_json::Value::Object(pat_map), _event_val) => {
            // Pattern is an object but event value is a scalar - check for nested match
            // This handles cases where pattern has nested objects but event has a leaf value
            if let Some((_key, _pat_val)) = pat_map.into_iter().next() {
                return false;
            }
            true
        }
        (serde_json::Value::Array(pat_arr), event_val) => {
            // Pattern array: event value must match at least one pattern element
            // Event value could be a scalar or an array
            let event_values: Vec<&serde_json::Value> =
                if let serde_json::Value::Array(arr) = event_val {
                    arr.iter().collect()
                } else {
                    vec![event_val]
                };

            for evt_v in &event_values {
                for pat_item in pat_arr {
                    if matches_pattern_item(pat_item, evt_v) {
                        return true;
                    }
                }
            }
            false
        }
        _ => false,
    }
}

fn matches_pattern_item(pattern_item: &serde_json::Value, event_value: &serde_json::Value) -> bool {
    match pattern_item {
        serde_json::Value::String(s) => {
            if let serde_json::Value::String(ev) = event_value {
                s == ev
            } else {
                false
            }
        }
        serde_json::Value::Number(n) => {
            if let serde_json::Value::Number(ev) = event_value {
                n.as_f64() == ev.as_f64()
            } else {
                false
            }
        }
        serde_json::Value::Bool(b) => {
            if let serde_json::Value::Bool(ev) = event_value {
                b == ev
            } else {
                false
            }
        }
        serde_json::Value::Null => event_value.is_null(),
        serde_json::Value::Object(m) => {
            // Special pattern matchers: prefix, exists, numeric, etc.
            if let Some(prefix) = m.get("prefix").and_then(|v| v.as_str()) {
                if let serde_json::Value::String(ev) = event_value {
                    return ev.starts_with(prefix);
                }
                return false;
            }
            if let Some(exists) = m.get("exists").and_then(|v| v.as_bool()) {
                // exists: true means the key must exist (already at leaf level)
                // exists: false means the key must NOT exist
                // At this point the key exists if we got here, so:
                return exists;
            }
            if let Some(numeric) = m.get("numeric").and_then(|v| v.as_array()) {
                let ev_num = match event_value.as_f64() {
                    Some(n) => n,
                    None => return false,
                };
                return matches_numeric(numeric, ev_num);
            }
            false
        }
        _ => false,
    }
}

fn matches_numeric(conditions: &[serde_json::Value], value: f64) -> bool {
    let mut i = 0;
    while i < conditions.len() {
        let op = match conditions[i].as_str() {
            Some(s) => s,
            None => return false,
        };
        if i + 1 >= conditions.len() {
            return false;
        }
        let cmp_val = match conditions[i + 1].as_f64() {
            Some(n) => n,
            None => return false,
        };
        let ok = match op {
            "<" => value < cmp_val,
            "<=" => value <= cmp_val,
            "=" => (value - cmp_val).abs() < f64::EPSILON,
            ">" => value > cmp_val,
            ">=" => value >= cmp_val,
            _ => return false,
        };
        if !ok {
            return false;
        }
        i += 2;
    }
    true
}
