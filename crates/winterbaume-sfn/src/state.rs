use std::collections::HashMap;

use chrono::Utc;

use crate::types::*;

/// In-memory state for the Step Functions service.
#[derive(Debug, Default)]
pub struct StepFunctionsState {
    /// State machines keyed by ARN.
    pub state_machines: HashMap<String, StateMachine>,
    /// Activities keyed by ARN.
    pub activities: HashMap<String, Activity>,
    /// Map runs keyed by map run ARN.
    pub map_runs: HashMap<String, MapRun>,
    /// Task tokens keyed by token string.
    pub task_tokens: HashMap<String, TaskToken>,
    /// History events keyed by execution ARN.
    pub execution_history: HashMap<String, Vec<HistoryEvent>>,
    /// State machine aliases keyed by alias ARN.
    pub aliases: HashMap<String, StateMachineAlias>,
    /// Published versions keyed by version ARN.
    pub versions: HashMap<String, StateMachineVersion>,
    /// Next version number per base state machine ARN.
    pub next_version: HashMap<String, u64>,
}

/// Error type for Step Functions operations.
#[derive(Debug, thiserror::Error)]
pub enum SfnError {
    #[error("State Machine Already Exists: '{0}'")]
    StateMachineAlreadyExists(String),
    #[error("State Machine Does Not Exist: '{0}'")]
    StateMachineDoesNotExist(String),
    #[error("Execution Already Exists: '{0}'")]
    ExecutionAlreadyExists(String),
    #[error("Execution Does Not Exist: '{0}'")]
    ExecutionDoesNotExist(String),
    #[error(
        "Execution cannot be redriven unless it is in FAILED, TIMED_OUT, or ABORTED state: '{0}'"
    )]
    ExecutionNotAbortedOrFailed(String),
    #[error("Activity Already Exists: '{0}'")]
    ActivityAlreadyExists(String),
    #[error("Activity Does Not Exist: '{0}'")]
    ActivityDoesNotExist(String),
    #[error("Task Does Not Exist: '{0}'")]
    TaskDoesNotExist(String),
    #[error("Resource Not Found: '{0}'")]
    ResourceNotFound(String),
    #[error("Map Run Does Not Exist: '{0}'")]
    MapRunDoesNotExist(String),
    #[error("State Machine Alias Already Exists: '{0}'")]
    StateMachineAliasMustBeUnique(String),
    #[error("State Machine Alias Does Not Exist: '{0}'")]
    StateMachineAliasDoesNotExist(String),
}

impl StepFunctionsState {
    pub fn create_state_machine(
        &mut self,
        name: &str,
        definition: &str,
        role_arn: &str,
        account_id: &str,
        region: &str,
        sm_type: Option<&str>,
        tags: Vec<Tag>,
        logging_configuration: Option<LoggingConfiguration>,
        tracing_configuration: Option<TracingConfiguration>,
        encryption_configuration: Option<EncryptionConfiguration>,
    ) -> Result<&StateMachine, SfnError> {
        let arn = format!("arn:aws:states:{region}:{account_id}:stateMachine:{name}");

        // Check for duplicate name
        if self.state_machines.values().any(|sm| sm.name == name) {
            return Err(SfnError::StateMachineAlreadyExists(arn.clone()));
        }

        let sm = StateMachine {
            name: name.to_string(),
            arn: arn.clone(),
            definition: definition.to_string(),
            role_arn: role_arn.to_string(),
            status: StateMachineStatus::Active,
            creation_date: Utc::now(),
            r#type: sm_type.unwrap_or("STANDARD").to_string(),
            executions: Vec::new(),
            tags,
            logging_configuration,
            tracing_configuration,
            encryption_configuration,
        };

        self.state_machines.insert(arn.clone(), sm);
        Ok(self.state_machines.get(&arn).unwrap())
    }

    pub fn delete_state_machine(&mut self, arn: &str) -> Result<(), SfnError> {
        // DeleteStateMachine is idempotent - does not error on non-existent
        self.state_machines.remove(arn);
        Ok(())
    }

    pub fn describe_state_machine(&self, arn: &str) -> Result<&StateMachine, SfnError> {
        self.state_machines
            .get(arn)
            .ok_or_else(|| not_found_error(arn))
    }

    pub fn list_state_machines(&self) -> Vec<&StateMachine> {
        self.state_machines.values().collect()
    }

    pub fn update_state_machine(
        &mut self,
        arn: &str,
        definition: Option<&str>,
        role_arn: Option<&str>,
        logging_configuration: Option<LoggingConfiguration>,
        tracing_configuration: Option<TracingConfiguration>,
        encryption_configuration: Option<EncryptionConfiguration>,
    ) -> Result<&StateMachine, SfnError> {
        let sm = self
            .state_machines
            .get_mut(arn)
            .ok_or_else(|| not_found_error(arn))?;

        if let Some(def) = definition {
            sm.definition = def.to_string();
        }
        if let Some(role) = role_arn {
            sm.role_arn = role.to_string();
        }
        if let Some(lc) = logging_configuration {
            sm.logging_configuration = Some(lc);
        }
        if let Some(tc) = tracing_configuration {
            sm.tracing_configuration = Some(tc);
        }
        if let Some(ec) = encryption_configuration {
            sm.encryption_configuration = Some(ec);
        }

        Ok(sm)
    }

    pub fn start_execution(
        &mut self,
        state_machine_arn: &str,
        name: Option<&str>,
        input: Option<&str>,
        account_id: &str,
        region: &str,
    ) -> Result<&Execution, SfnError> {
        // Verify state machine exists and get its name
        let sm_name = match self.state_machines.get(state_machine_arn) {
            Some(sm) => sm.name.clone(),
            None => return Err(not_found_error(state_machine_arn)),
        };

        let exec_name = name
            .map(|n| n.to_string())
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        // Default input to "{}" when not specified
        let exec_input = input.unwrap_or("{}").to_string();

        let execution_arn =
            format!("arn:aws:states:{region}:{account_id}:execution:{sm_name}:{exec_name}",);

        // Check for duplicate execution name
        let sm = self.state_machines.get(state_machine_arn).unwrap();
        if let Some(existing) = sm.executions.iter().find(|e| e.name == exec_name) {
            // If same input, return existing (idempotent)
            if existing.input.as_deref() == Some(&exec_input) {
                let exec_arn = existing.execution_arn.clone();
                // Return reference to existing execution
                let sm = self.state_machines.get(state_machine_arn).unwrap();
                return Ok(sm
                    .executions
                    .iter()
                    .find(|e| e.execution_arn == exec_arn)
                    .unwrap());
            } else {
                // Different input with same name: error
                return Err(SfnError::ExecutionAlreadyExists(
                    existing.execution_arn.clone(),
                ));
            }
        }

        let execution = Execution {
            execution_arn,
            name: exec_name,
            status: ExecutionStatus::Running,
            start_date: Utc::now(),
            stop_date: None,
            input: Some(exec_input),
            output: None,
            state_machine_arn: state_machine_arn.to_string(),
        };

        let sm = self.state_machines.get_mut(state_machine_arn).unwrap();
        sm.executions.push(execution);
        let exec = sm.executions.last().unwrap();
        let exec_arn = exec.execution_arn.clone();
        let _exec_input = exec.input.clone().unwrap_or_else(|| "{}".to_string());
        let _role_arn = sm.role_arn.clone();

        // Generate default execution history events (matching moto behavior)
        let now = Utc::now();
        let events = vec![
            HistoryEvent {
                id: 1,
                event_type: "ExecutionStarted".to_string(),
                timestamp: now,
                previous_event_id: Some(0),
            },
            HistoryEvent {
                id: 2,
                event_type: "PassStateEntered".to_string(),
                timestamp: now,
                previous_event_id: Some(0),
            },
            HistoryEvent {
                id: 3,
                event_type: "PassStateExited".to_string(),
                timestamp: now,
                previous_event_id: Some(2),
            },
            HistoryEvent {
                id: 4,
                event_type: "ExecutionSucceeded".to_string(),
                timestamp: now,
                previous_event_id: Some(3),
            },
        ];
        self.execution_history.insert(exec_arn.clone(), events);

        let sm = self.state_machines.get(state_machine_arn).unwrap();
        Ok(sm
            .executions
            .iter()
            .find(|e| e.execution_arn == exec_arn)
            .unwrap())
    }

    pub fn stop_execution(&mut self, execution_arn: &str) -> Result<&Execution, SfnError> {
        for sm in self.state_machines.values_mut() {
            if let Some(exec) = sm
                .executions
                .iter_mut()
                .find(|e| e.execution_arn == execution_arn)
            {
                exec.status = ExecutionStatus::Aborted;
                exec.stop_date = Some(Utc::now());
                let exec_arn = exec.execution_arn.clone();
                return Ok(sm
                    .executions
                    .iter()
                    .find(|e| e.execution_arn == exec_arn)
                    .unwrap());
            }
        }
        Err(SfnError::ExecutionDoesNotExist(execution_arn.to_string()))
    }

    pub fn describe_execution(&self, execution_arn: &str) -> Result<&Execution, SfnError> {
        for sm in self.state_machines.values() {
            if let Some(exec) = sm
                .executions
                .iter()
                .find(|e| e.execution_arn == execution_arn)
            {
                return Ok(exec);
            }
        }
        Err(SfnError::ExecutionDoesNotExist(execution_arn.to_string()))
    }

    pub fn list_executions(&self, state_machine_arn: &str) -> Result<Vec<&Execution>, SfnError> {
        let sm = self
            .state_machines
            .get(state_machine_arn)
            .ok_or_else(|| not_found_error(state_machine_arn))?;
        Ok(sm.executions.iter().collect())
    }

    pub fn tag_resource(&mut self, arn: &str, new_tags: Vec<Tag>) -> Result<(), SfnError> {
        // Try state machine first, then activity
        let tags = if let Some(sm) = self.state_machines.get_mut(arn) {
            &mut sm.tags
        } else if let Some(activity) = self.activities.get_mut(arn) {
            &mut activity.tags
        } else {
            return Err(SfnError::ResourceNotFound(arn.to_string()));
        };

        for new_tag in new_tags {
            if let Some(existing) = tags.iter_mut().find(|t| t.key == new_tag.key) {
                existing.value = new_tag.value;
            } else {
                tags.push(new_tag);
            }
        }
        Ok(())
    }

    pub fn untag_resource(&mut self, arn: &str, tag_keys: &[String]) -> Result<(), SfnError> {
        let tags = if let Some(sm) = self.state_machines.get_mut(arn) {
            &mut sm.tags
        } else if let Some(activity) = self.activities.get_mut(arn) {
            &mut activity.tags
        } else {
            return Err(SfnError::ResourceNotFound(arn.to_string()));
        };

        tags.retain(|t| !tag_keys.contains(&t.key));
        Ok(())
    }

    pub fn list_tags_for_resource(&self, arn: &str) -> Vec<&Tag> {
        // Returns empty list for non-existent resources (per AWS behavior)
        if let Some(sm) = self.state_machines.get(arn) {
            sm.tags.iter().collect()
        } else if let Some(activity) = self.activities.get(arn) {
            activity.tags.iter().collect()
        } else {
            Vec::new()
        }
    }

    // --- Activity operations ---

    pub fn create_activity(
        &mut self,
        name: &str,
        account_id: &str,
        region: &str,
        tags: Vec<Tag>,
    ) -> Result<&Activity, SfnError> {
        // Check for duplicate name
        if self.activities.values().any(|a| a.name == name) {
            return Err(SfnError::ActivityAlreadyExists(name.to_string()));
        }

        let arn = format!("arn:aws:states:{region}:{account_id}:activity:{name}");
        let activity = Activity {
            name: name.to_string(),
            arn: arn.clone(),
            creation_date: Utc::now(),
            tags,
        };

        self.activities.insert(arn.clone(), activity);
        Ok(self.activities.get(&arn).unwrap())
    }

    pub fn describe_activity(&self, arn: &str) -> Result<&Activity, SfnError> {
        self.activities
            .get(arn)
            .ok_or_else(|| SfnError::ActivityDoesNotExist(arn.to_string()))
    }

    pub fn delete_activity(&mut self, arn: &str) -> Result<(), SfnError> {
        // DeleteActivity is idempotent
        self.activities.remove(arn);
        Ok(())
    }

    pub fn list_activities(&self) -> Vec<&Activity> {
        self.activities.values().collect()
    }

    // --- Task token operations ---

    pub fn register_task_token(&mut self, token: &str, execution_arn: &str) {
        self.task_tokens.insert(
            token.to_string(),
            TaskToken {
                token: token.to_string(),
                execution_arn: execution_arn.to_string(),
            },
        );
    }

    pub fn send_task_success(&mut self, task_token: &str, output: &str) -> Result<(), SfnError> {
        let token = self
            .task_tokens
            .remove(task_token)
            .ok_or_else(|| SfnError::TaskDoesNotExist(task_token.to_string()))?;

        // Find execution and mark it succeeded
        for sm in self.state_machines.values_mut() {
            if let Some(exec) = sm
                .executions
                .iter_mut()
                .find(|e| e.execution_arn == token.execution_arn)
            {
                exec.status = ExecutionStatus::Succeeded;
                exec.stop_date = Some(Utc::now());
                exec.output = Some(output.to_string());
                return Ok(());
            }
        }
        Ok(())
    }

    pub fn send_task_failure(
        &mut self,
        task_token: &str,
        _error: Option<&str>,
        _cause: Option<&str>,
    ) -> Result<(), SfnError> {
        let token = self
            .task_tokens
            .remove(task_token)
            .ok_or_else(|| SfnError::TaskDoesNotExist(task_token.to_string()))?;

        // Find execution and mark it failed
        for sm in self.state_machines.values_mut() {
            if let Some(exec) = sm
                .executions
                .iter_mut()
                .find(|e| e.execution_arn == token.execution_arn)
            {
                exec.status = ExecutionStatus::Failed;
                exec.stop_date = Some(Utc::now());
                return Ok(());
            }
        }
        Ok(())
    }

    pub fn send_task_heartbeat(&mut self, task_token: &str) -> Result<(), SfnError> {
        if !self.task_tokens.contains_key(task_token) {
            return Err(SfnError::TaskDoesNotExist(task_token.to_string()));
        }
        // Heartbeat is acknowledged; no state change needed
        Ok(())
    }

    // --- Execution history operations ---

    pub fn get_execution_history(
        &self,
        execution_arn: &str,
    ) -> Result<Vec<&HistoryEvent>, SfnError> {
        // Verify execution exists
        let mut found = false;
        for sm in self.state_machines.values() {
            if sm
                .executions
                .iter()
                .any(|e| e.execution_arn == execution_arn)
            {
                found = true;
                break;
            }
        }
        if !found {
            return Err(SfnError::ExecutionDoesNotExist(execution_arn.to_string()));
        }

        Ok(self
            .execution_history
            .get(execution_arn)
            .map(|events| events.iter().collect())
            .unwrap_or_default())
    }

    // --- Map run operations ---

    pub fn describe_map_run(&self, map_run_arn: &str) -> Result<&MapRun, SfnError> {
        self.map_runs
            .get(map_run_arn)
            .ok_or_else(|| SfnError::MapRunDoesNotExist(map_run_arn.to_string()))
    }

    pub fn list_map_runs(&self, execution_arn: &str) -> Result<Vec<&MapRun>, SfnError> {
        // Verify execution exists
        let mut found = false;
        for sm in self.state_machines.values() {
            if sm
                .executions
                .iter()
                .any(|e| e.execution_arn == execution_arn)
            {
                found = true;
                break;
            }
        }
        if !found {
            return Err(SfnError::ExecutionDoesNotExist(execution_arn.to_string()));
        }

        Ok(self
            .map_runs
            .values()
            .filter(|mr| mr.execution_arn == execution_arn)
            .collect())
    }

    pub fn update_map_run(
        &mut self,
        map_run_arn: &str,
        max_concurrency: Option<i32>,
        tolerated_failure_count: Option<i64>,
        tolerated_failure_percentage: Option<f32>,
    ) -> Result<(), SfnError> {
        let mr = self
            .map_runs
            .get_mut(map_run_arn)
            .ok_or_else(|| SfnError::MapRunDoesNotExist(map_run_arn.to_string()))?;

        if let Some(mc) = max_concurrency {
            mr.max_concurrency = mc;
        }
        if let Some(tfc) = tolerated_failure_count {
            mr.tolerated_failure_count = tfc;
        }
        if let Some(tfp) = tolerated_failure_percentage {
            mr.tolerated_failure_percentage = tfp;
        }
        Ok(())
    }

    // --- State machine for execution ---

    pub fn describe_state_machine_for_execution(
        &self,
        execution_arn: &str,
    ) -> Result<&StateMachine, SfnError> {
        for sm in self.state_machines.values() {
            if sm
                .executions
                .iter()
                .any(|e| e.execution_arn == execution_arn)
            {
                return Ok(sm);
            }
        }
        Err(SfnError::ExecutionDoesNotExist(execution_arn.to_string()))
    }

    // --- State machine version operations ---

    pub fn publish_state_machine_version(
        &mut self,
        state_machine_arn: &str,
        description: Option<&str>,
        account_id: &str,
        region: &str,
    ) -> Result<&StateMachineVersion, SfnError> {
        // Verify state machine exists
        if !self.state_machines.contains_key(state_machine_arn) {
            return Err(not_found_error(state_machine_arn));
        }

        // Derive the state machine name from the ARN
        let sm_name = state_machine_arn
            .split(':')
            .next_back()
            .unwrap_or("unknown");

        let version_num = {
            let counter = self
                .next_version
                .entry(state_machine_arn.to_string())
                .or_insert(0);
            *counter += 1;
            *counter
        };

        let version_arn =
            format!("arn:aws:states:{region}:{account_id}:stateMachine:{sm_name}:{version_num}");

        let version = StateMachineVersion {
            version_arn: version_arn.clone(),
            state_machine_arn: state_machine_arn.to_string(),
            description: description.map(|s| s.to_string()),
            creation_date: Utc::now(),
            version_number: version_num,
        };

        self.versions.insert(version_arn.clone(), version);
        Ok(self.versions.get(&version_arn).unwrap())
    }

    pub fn delete_state_machine_version(&mut self, version_arn: &str) -> Result<(), SfnError> {
        // Idempotent
        self.versions.remove(version_arn);
        Ok(())
    }

    pub fn list_state_machine_versions(
        &self,
        state_machine_arn: &str,
    ) -> Result<Vec<&StateMachineVersion>, SfnError> {
        if !self.state_machines.contains_key(state_machine_arn) {
            return Err(not_found_error(state_machine_arn));
        }
        let mut versions: Vec<&StateMachineVersion> = self
            .versions
            .values()
            .filter(|v| v.state_machine_arn == state_machine_arn)
            .collect();
        versions.sort_by_key(|v| v.version_number);
        Ok(versions)
    }

    // --- State machine alias operations ---

    pub fn create_state_machine_alias(
        &mut self,
        name: &str,
        description: Option<&str>,
        routing_configuration: Vec<(String, i32)>,
        account_id: &str,
        region: &str,
    ) -> Result<&StateMachineAlias, SfnError> {
        // Derive the base state machine ARN from the first routing config version ARN.
        // Version ARNs look like: arn:aws:states:region:account:stateMachine:name:N
        // Alias ARN looks like:   arn:aws:states:region:account:stateMachine:name:aliasName
        let base_sm_arn = routing_configuration
            .first()
            .map(|(version_arn, _)| {
                // Strip the last segment (version number) to get the base ARN
                let parts: Vec<&str> = version_arn.rsplitn(2, ':').collect();
                if parts.len() == 2 {
                    parts[1].to_string()
                } else {
                    version_arn.clone()
                }
            })
            .unwrap_or_else(|| {
                format!("arn:aws:states:{region}:{account_id}:stateMachine:unknown")
            });

        let sm_name = base_sm_arn.split(':').next_back().unwrap_or("unknown");
        let alias_arn =
            format!("arn:aws:states:{region}:{account_id}:stateMachine:{sm_name}:{name}");

        if self.aliases.contains_key(&alias_arn) {
            return Err(SfnError::StateMachineAliasMustBeUnique(alias_arn.clone()));
        }

        let now = Utc::now();
        let alias = StateMachineAlias {
            alias_arn: alias_arn.clone(),
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            routing_configuration,
            creation_date: now,
            update_date: now,
        };

        self.aliases.insert(alias_arn.clone(), alias);
        Ok(self.aliases.get(&alias_arn).unwrap())
    }

    pub fn describe_state_machine_alias(
        &self,
        alias_arn: &str,
    ) -> Result<&StateMachineAlias, SfnError> {
        self.aliases
            .get(alias_arn)
            .ok_or_else(|| SfnError::StateMachineAliasDoesNotExist(alias_arn.to_string()))
    }

    pub fn delete_state_machine_alias(&mut self, alias_arn: &str) -> Result<(), SfnError> {
        // Idempotent
        self.aliases.remove(alias_arn);
        Ok(())
    }

    pub fn list_state_machine_aliases(
        &self,
        state_machine_arn: &str,
    ) -> Result<Vec<&StateMachineAlias>, SfnError> {
        if !self.state_machines.contains_key(state_machine_arn) {
            return Err(not_found_error(state_machine_arn));
        }
        // Aliases whose routing_configuration references versions of this state machine
        let mut aliases: Vec<&StateMachineAlias> = self
            .aliases
            .values()
            .filter(|a| {
                a.routing_configuration.iter().any(|(version_arn, _)| {
                    // Version ARN = base_sm_arn:N; base_sm_arn == state_machine_arn
                    version_arn.starts_with(&format!("{state_machine_arn}:"))
                })
            })
            .collect();
        aliases.sort_by(|a, b| a.alias_arn.cmp(&b.alias_arn));
        Ok(aliases)
    }

    pub fn update_state_machine_alias(
        &mut self,
        alias_arn: &str,
        description: Option<&str>,
        routing_configuration: Option<Vec<(String, i32)>>,
    ) -> Result<&StateMachineAlias, SfnError> {
        let alias = self
            .aliases
            .get_mut(alias_arn)
            .ok_or_else(|| SfnError::StateMachineAliasDoesNotExist(alias_arn.to_string()))?;

        if let Some(desc) = description {
            alias.description = Some(desc.to_string());
        }
        if let Some(rc) = routing_configuration {
            alias.routing_configuration = rc;
        }
        alias.update_date = Utc::now();
        Ok(alias)
    }

    // --- RedriveExecution ---

    pub fn redrive_execution(&mut self, execution_arn: &str) -> Result<(), SfnError> {
        for sm in self.state_machines.values_mut() {
            if let Some(exec) = sm
                .executions
                .iter_mut()
                .find(|e| e.execution_arn == execution_arn)
            {
                if exec.status == ExecutionStatus::Failed
                    || exec.status == ExecutionStatus::TimedOut
                    || exec.status == ExecutionStatus::Aborted
                {
                    exec.status = ExecutionStatus::Running;
                    exec.stop_date = None;
                    return Ok(());
                } else {
                    return Err(SfnError::ExecutionNotAbortedOrFailed(
                        execution_arn.to_string(),
                    ));
                }
            }
        }
        Err(SfnError::ExecutionDoesNotExist(execution_arn.to_string()))
    }
}

fn not_found_error(arn: &str) -> SfnError {
    SfnError::StateMachineDoesNotExist(arn.to_string())
}
