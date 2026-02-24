use std::collections::HashMap;

use chrono::Utc;
use uuid::Uuid;

use crate::types::{
    ChangeSet, RegisteredType, Stack, StackEvent, StackInstance, StackInstanceKey, StackParameter,
    StackResource, StackSet, StackSetOperation, StackSetOperationResult, StackTag,
};

#[derive(Debug, Default)]
pub struct CloudFormationState {
    /// stacks keyed by stack_name
    pub stacks: HashMap<String, Stack>,
    /// stack sets keyed by stack_set_name
    pub stack_sets: HashMap<String, StackSet>,
    /// stack instances keyed by (stack_set_name, account, region)
    pub stack_instances: HashMap<StackInstanceKey, StackInstance>,
    /// registered CloudFormation types
    pub registered_types: Vec<RegisteredType>,
}

#[derive(Debug, thiserror::Error)]
pub enum CloudFormationError {
    #[error("Stack '{0}' not found")]
    StackNotFound(String),
    #[error("Stack '{0}' already exists")]
    StackAlreadyExists(String),
    #[error("ChangeSet '{0}' not found")]
    ChangeSetNotFound(String),
    #[error("ChangeSet '{0}' already exists")]
    ChangeSetAlreadyExists(String),
    #[error("StackSet '{0}' not found")]
    StackSetNotFound(String),
    #[error("StackSet '{0}' already exists")]
    StackSetAlreadyExists(String),
    #[error("StackInstance '{0}' not found")]
    StackInstanceNotFound(String),
    #[error("StackSetOperation '{0}' not found")]
    StackSetOperationNotFound(String),
    #[error("{0}")]
    ValidationError(String),
}

fn now_timestamp() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%S.000Z").to_string()
}

fn new_uuid() -> String {
    Uuid::new_v4().to_string()
}

fn make_stack_arn(region: &str, account_id: &str, stack_name: &str) -> String {
    let uid = new_uuid();
    format!("arn:aws:cloudformation:{region}:{account_id}:stack/{stack_name}/{uid}")
}

fn make_stack_set_arn(region: &str, account_id: &str, stack_set_name: &str) -> String {
    let uid = new_uuid();
    format!("arn:aws:cloudformation:{region}:{account_id}:stackset/{stack_set_name}:{uid}")
}

fn make_change_set_arn(
    region: &str,
    account_id: &str,
    stack_name: &str,
    change_set_name: &str,
) -> String {
    let uid = new_uuid();
    format!(
        "arn:aws:cloudformation:{region}:{account_id}:changeSet/{change_set_name}/{stack_name}/{uid}"
    )
}

fn make_stack_event(
    stack_id: &str,
    stack_name: &str,
    logical_resource_id: &str,
    resource_type: &str,
    resource_status: &str,
) -> StackEvent {
    StackEvent {
        event_id: new_uuid(),
        stack_id: stack_id.to_string(),
        stack_name: stack_name.to_string(),
        logical_resource_id: logical_resource_id.to_string(),
        physical_resource_id: None,
        resource_type: resource_type.to_string(),
        timestamp: now_timestamp(),
        resource_status: resource_status.to_string(),
        resource_status_reason: None,
    }
}

impl CloudFormationState {
    // --- Stack operations ---

    #[allow(clippy::too_many_arguments)]
    pub fn create_stack(
        &mut self,
        stack_name: &str,
        template_body: Option<String>,
        parameters: Vec<StackParameter>,
        tags: Vec<StackTag>,
        capabilities: Vec<String>,
        role_arn: Option<String>,
        timeout_in_minutes: Option<i32>,
        disable_rollback: bool,
        region: &str,
        account_id: &str,
    ) -> Result<String, CloudFormationError> {
        if self.stacks.contains_key(stack_name) {
            return Err(CloudFormationError::StackAlreadyExists(
                stack_name.to_string(),
            ));
        }
        let stack_id = make_stack_arn(region, account_id, stack_name);
        let now = now_timestamp();
        let event = make_stack_event(
            &stack_id,
            stack_name,
            stack_name,
            "AWS::CloudFormation::Stack",
            "CREATE_COMPLETE",
        );
        let stack = Stack {
            stack_id: stack_id.clone(),
            stack_name: stack_name.to_string(),
            stack_status: "CREATE_COMPLETE".to_string(),
            creation_time: now,
            last_updated_time: None,
            deletion_time: None,
            description: None,
            template_body,
            stack_policy_body: None,
            parameters,
            outputs: Vec::new(),
            tags,
            capabilities,
            resources: Vec::new(),
            events: vec![event],
            change_sets: Vec::new(),
            exports: Vec::new(),
            role_arn,
            timeout_in_minutes,
            disable_rollback,
            enable_termination_protection: false,
        };
        self.stacks.insert(stack_name.to_string(), stack);
        Ok(stack_id)
    }

    pub fn update_stack(
        &mut self,
        stack_name: &str,
        template_body: Option<String>,
        parameters: Vec<StackParameter>,
        tags: Vec<StackTag>,
        capabilities: Vec<String>,
    ) -> Result<String, CloudFormationError> {
        let stack = self
            .stacks
            .get_mut(stack_name)
            .ok_or_else(|| CloudFormationError::StackNotFound(stack_name.to_string()))?;
        let now = now_timestamp();
        if let Some(tb) = template_body {
            stack.template_body = Some(tb);
        }
        stack.stack_status = "UPDATE_COMPLETE".to_string();
        stack.last_updated_time = Some(now.clone());
        if !parameters.is_empty() {
            stack.parameters = parameters;
        }
        if !tags.is_empty() {
            stack.tags = tags;
        }
        if !capabilities.is_empty() {
            stack.capabilities = capabilities;
        }
        let event = make_stack_event(
            &stack.stack_id,
            stack_name,
            stack_name,
            "AWS::CloudFormation::Stack",
            "UPDATE_COMPLETE",
        );
        stack.events.push(event);
        Ok(stack.stack_id.clone())
    }

    pub fn delete_stack(&mut self, stack_name: &str) -> Result<(), CloudFormationError> {
        let stack = self
            .stacks
            .get_mut(stack_name)
            .ok_or_else(|| CloudFormationError::StackNotFound(stack_name.to_string()))?;
        stack.stack_status = "DELETE_COMPLETE".to_string();
        stack.deletion_time = Some(now_timestamp());
        let event = make_stack_event(
            &stack.stack_id,
            stack_name,
            stack_name,
            "AWS::CloudFormation::Stack",
            "DELETE_COMPLETE",
        );
        stack.events.push(event);
        Ok(())
    }

    pub fn describe_stacks(
        &self,
        stack_name: Option<&str>,
    ) -> Result<Vec<&Stack>, CloudFormationError> {
        if let Some(name) = stack_name {
            // Can look up by name or ARN
            let found = self
                .stacks
                .values()
                .find(|s| s.stack_name == name || s.stack_id == name);
            match found {
                Some(s) => {
                    // Don't return DELETE_COMPLETE stacks by name
                    if s.stack_status == "DELETE_COMPLETE" {
                        return Err(CloudFormationError::StackNotFound(name.to_string()));
                    }
                    Ok(vec![s])
                }
                None => Err(CloudFormationError::StackNotFound(name.to_string())),
            }
        } else {
            let stacks: Vec<&Stack> = self
                .stacks
                .values()
                .filter(|s| s.stack_status != "DELETE_COMPLETE")
                .collect();
            Ok(stacks)
        }
    }

    pub fn list_stacks(&self, status_filter: &[String]) -> Vec<&Stack> {
        self.stacks
            .values()
            .filter(|s| status_filter.is_empty() || status_filter.contains(&s.stack_status))
            .collect()
    }

    pub fn get_template(&self, stack_name: &str) -> Result<Option<String>, CloudFormationError> {
        let stack = self
            .stacks
            .get(stack_name)
            .ok_or_else(|| CloudFormationError::StackNotFound(stack_name.to_string()))?;
        Ok(stack.template_body.clone())
    }

    pub fn get_stack_policy(
        &self,
        stack_name: &str,
    ) -> Result<Option<String>, CloudFormationError> {
        let stack = self
            .stacks
            .get(stack_name)
            .ok_or_else(|| CloudFormationError::StackNotFound(stack_name.to_string()))?;
        Ok(stack.stack_policy_body.clone())
    }

    pub fn set_stack_policy(
        &mut self,
        stack_name: &str,
        policy_body: Option<String>,
    ) -> Result<(), CloudFormationError> {
        let stack = self
            .stacks
            .get_mut(stack_name)
            .ok_or_else(|| CloudFormationError::StackNotFound(stack_name.to_string()))?;
        stack.stack_policy_body = policy_body;
        Ok(())
    }

    pub fn describe_stack_events(
        &self,
        stack_name: &str,
    ) -> Result<Vec<&StackEvent>, CloudFormationError> {
        let stack = self
            .stacks
            .get(stack_name)
            .ok_or_else(|| CloudFormationError::StackNotFound(stack_name.to_string()))?;
        let mut events: Vec<&StackEvent> = stack.events.iter().collect();
        // Newest first
        events.reverse();
        Ok(events)
    }

    pub fn describe_stack_resources(
        &self,
        stack_name: &str,
    ) -> Result<Vec<&StackResource>, CloudFormationError> {
        let stack = self
            .stacks
            .get(stack_name)
            .ok_or_else(|| CloudFormationError::StackNotFound(stack_name.to_string()))?;
        Ok(stack.resources.iter().collect())
    }

    pub fn describe_stack_resource(
        &self,
        stack_name: &str,
        logical_resource_id: &str,
    ) -> Result<Option<&StackResource>, CloudFormationError> {
        let stack = self
            .stacks
            .get(stack_name)
            .ok_or_else(|| CloudFormationError::StackNotFound(stack_name.to_string()))?;
        Ok(stack
            .resources
            .iter()
            .find(|r| r.logical_resource_id == logical_resource_id))
    }

    pub fn list_stack_resources(
        &self,
        stack_name: &str,
    ) -> Result<Vec<&StackResource>, CloudFormationError> {
        self.describe_stack_resources(stack_name)
    }

    pub fn list_exports(&self) -> Vec<crate::types::StackExport> {
        let mut exports = Vec::new();
        for stack in self.stacks.values() {
            if stack.stack_status != "DELETE_COMPLETE" {
                exports.extend(stack.exports.iter().cloned());
            }
        }
        exports
    }

    // --- ChangeSet operations ---

    #[allow(clippy::too_many_arguments)]
    pub fn create_change_set(
        &mut self,
        stack_name: &str,
        change_set_name: &str,
        description: Option<String>,
        parameters: Vec<StackParameter>,
        tags: Vec<StackTag>,
        region: &str,
        account_id: &str,
    ) -> Result<(String, String), CloudFormationError> {
        let stack = self
            .stacks
            .get_mut(stack_name)
            .ok_or_else(|| CloudFormationError::StackNotFound(stack_name.to_string()))?;

        // Check if changeset name already exists
        if stack
            .change_sets
            .iter()
            .any(|cs| cs.change_set_name == change_set_name)
        {
            return Err(CloudFormationError::ChangeSetAlreadyExists(
                change_set_name.to_string(),
            ));
        }

        let change_set_id = make_change_set_arn(region, account_id, stack_name, change_set_name);
        let stack_id = stack.stack_id.clone();
        let cs = ChangeSet {
            change_set_id: change_set_id.clone(),
            change_set_name: change_set_name.to_string(),
            stack_id: stack_id.clone(),
            stack_name: stack_name.to_string(),
            status: "CREATE_COMPLETE".to_string(),
            status_reason: None,
            execution_status: "AVAILABLE".to_string(),
            description,
            creation_time: now_timestamp(),
            parameters,
            tags,
        };
        stack.change_sets.push(cs);
        Ok((change_set_id, stack_id))
    }

    pub fn delete_change_set(
        &mut self,
        stack_name: &str,
        change_set_name: &str,
    ) -> Result<(), CloudFormationError> {
        let stack = self
            .stacks
            .get_mut(stack_name)
            .ok_or_else(|| CloudFormationError::StackNotFound(stack_name.to_string()))?;
        let pos = stack.change_sets.iter().position(|cs| {
            cs.change_set_name == change_set_name || cs.change_set_id == change_set_name
        });
        match pos {
            Some(i) => {
                stack.change_sets.remove(i);
                Ok(())
            }
            None => Err(CloudFormationError::ChangeSetNotFound(
                change_set_name.to_string(),
            )),
        }
    }

    pub fn describe_change_set(
        &self,
        stack_name: &str,
        change_set_name: &str,
    ) -> Result<&ChangeSet, CloudFormationError> {
        let stack = self
            .stacks
            .get(stack_name)
            .ok_or_else(|| CloudFormationError::StackNotFound(stack_name.to_string()))?;
        stack
            .change_sets
            .iter()
            .find(|cs| cs.change_set_name == change_set_name || cs.change_set_id == change_set_name)
            .ok_or_else(|| CloudFormationError::ChangeSetNotFound(change_set_name.to_string()))
    }

    pub fn execute_change_set(
        &mut self,
        stack_name: &str,
        change_set_name: &str,
    ) -> Result<(), CloudFormationError> {
        let stack = self
            .stacks
            .get_mut(stack_name)
            .ok_or_else(|| CloudFormationError::StackNotFound(stack_name.to_string()))?;
        let pos = stack.change_sets.iter().position(|cs| {
            cs.change_set_name == change_set_name || cs.change_set_id == change_set_name
        });
        match pos {
            Some(i) => {
                let cs = &mut stack.change_sets[i];
                if cs.execution_status != "AVAILABLE" {
                    return Err(CloudFormationError::ValidationError(format!(
                        "ChangeSet '{change_set_name}' is not in AVAILABLE state"
                    )));
                }
                cs.execution_status = "EXECUTE_COMPLETE".to_string();
                stack.stack_status = "UPDATE_COMPLETE".to_string();
                stack.last_updated_time = Some(now_timestamp());
                Ok(())
            }
            None => Err(CloudFormationError::ChangeSetNotFound(
                change_set_name.to_string(),
            )),
        }
    }

    pub fn list_change_sets(
        &self,
        stack_name: &str,
    ) -> Result<Vec<&ChangeSet>, CloudFormationError> {
        let stack = self
            .stacks
            .get(stack_name)
            .ok_or_else(|| CloudFormationError::StackNotFound(stack_name.to_string()))?;
        Ok(stack.change_sets.iter().collect())
    }

    // --- StackSet operations ---

    pub fn create_stack_set(
        &mut self,
        stack_set_name: &str,
        description: Option<String>,
        template_body: Option<String>,
        parameters: Vec<StackParameter>,
        tags: Vec<StackTag>,
        capabilities: Vec<String>,
        region: &str,
        account_id: &str,
    ) -> Result<String, CloudFormationError> {
        if self.stack_sets.contains_key(stack_set_name) {
            return Err(CloudFormationError::StackSetAlreadyExists(
                stack_set_name.to_string(),
            ));
        }
        let stack_set_id = new_uuid();
        let stack_set_arn = make_stack_set_arn(region, account_id, stack_set_name);
        let ss = StackSet {
            stack_set_id: stack_set_id.clone(),
            stack_set_name: stack_set_name.to_string(),
            stack_set_arn,
            status: "ACTIVE".to_string(),
            description,
            template_body,
            parameters,
            tags,
            capabilities,
            operations: Vec::new(),
        };
        self.stack_sets.insert(stack_set_name.to_string(), ss);
        Ok(stack_set_id)
    }

    pub fn describe_stack_set(
        &self,
        stack_set_name: &str,
    ) -> Result<&StackSet, CloudFormationError> {
        self.stack_sets
            .get(stack_set_name)
            .ok_or_else(|| CloudFormationError::StackSetNotFound(stack_set_name.to_string()))
    }

    pub fn update_stack_set(
        &mut self,
        stack_set_name: &str,
        description: Option<String>,
        template_body: Option<String>,
        parameters: Vec<StackParameter>,
        tags: Vec<StackTag>,
        region: &str,
        account_id: &str,
    ) -> Result<String, CloudFormationError> {
        let ss = self
            .stack_sets
            .get_mut(stack_set_name)
            .ok_or_else(|| CloudFormationError::StackSetNotFound(stack_set_name.to_string()))?;

        if let Some(d) = description {
            ss.description = Some(d);
        }
        if let Some(tb) = template_body {
            ss.template_body = Some(tb);
        }
        if !parameters.is_empty() {
            ss.parameters = parameters;
        }
        if !tags.is_empty() {
            ss.tags = tags;
        }

        let operation_id = new_uuid();
        let op = StackSetOperation {
            operation_id: operation_id.clone(),
            action: "UPDATE".to_string(),
            status: "SUCCEEDED".to_string(),
            creation_timestamp: now_timestamp(),
            end_timestamp: Some(now_timestamp()),
            stack_set_id: ss.stack_set_id.clone(),
            results: Vec::new(),
        };
        ss.operations.push(op);
        let _ = (region, account_id);
        Ok(operation_id)
    }

    pub fn delete_stack_set(&mut self, stack_set_name: &str) -> Result<(), CloudFormationError> {
        if !self.stack_sets.contains_key(stack_set_name) {
            return Err(CloudFormationError::StackSetNotFound(
                stack_set_name.to_string(),
            ));
        }
        self.stack_sets.remove(stack_set_name);
        Ok(())
    }

    pub fn list_stack_sets(&self) -> Vec<&StackSet> {
        self.stack_sets.values().collect()
    }

    // --- StackInstance operations ---

    pub fn create_stack_instances(
        &mut self,
        stack_set_name: &str,
        accounts: &[String],
        regions: &[String],
        parameter_overrides: Vec<StackParameter>,
        account_id: &str,
    ) -> Result<String, CloudFormationError> {
        if !self.stack_sets.contains_key(stack_set_name) {
            return Err(CloudFormationError::StackSetNotFound(
                stack_set_name.to_string(),
            ));
        }
        let ss = self.stack_sets.get(stack_set_name).unwrap();
        let stack_set_id = ss.stack_set_id.clone();

        let target_accounts = if accounts.is_empty() {
            vec![account_id.to_string()]
        } else {
            accounts.to_vec()
        };

        for acct in &target_accounts {
            for rgn in regions {
                let key = StackInstanceKey {
                    stack_set_name: stack_set_name.to_string(),
                    account: acct.clone(),
                    region: rgn.clone(),
                };
                self.stack_instances
                    .entry(key)
                    .or_insert_with(|| StackInstance {
                        stack_set_name: stack_set_name.to_string(),
                        account: acct.clone(),
                        region: rgn.clone(),
                        stack_id: None,
                        status: "CURRENT".to_string(),
                        status_reason: None,
                        stack_set_id: stack_set_id.clone(),
                        parameter_overrides: parameter_overrides.clone(),
                    });
            }
        }

        let operation_id = new_uuid();
        let ss = self.stack_sets.get_mut(stack_set_name).unwrap();
        let op = StackSetOperation {
            operation_id: operation_id.clone(),
            action: "CREATE".to_string(),
            status: "SUCCEEDED".to_string(),
            creation_timestamp: now_timestamp(),
            end_timestamp: Some(now_timestamp()),
            stack_set_id: ss.stack_set_id.clone(),
            results: target_accounts
                .iter()
                .flat_map(|a| {
                    regions.iter().map(move |r| StackSetOperationResult {
                        account: a.clone(),
                        region: r.clone(),
                        status: "SUCCEEDED".to_string(),
                        status_reason: None,
                    })
                })
                .collect(),
        };
        ss.operations.push(op);
        Ok(operation_id)
    }

    pub fn delete_stack_instances(
        &mut self,
        stack_set_name: &str,
        accounts: &[String],
        regions: &[String],
    ) -> Result<String, CloudFormationError> {
        if !self.stack_sets.contains_key(stack_set_name) {
            return Err(CloudFormationError::StackSetNotFound(
                stack_set_name.to_string(),
            ));
        }
        for acct in accounts {
            for rgn in regions {
                let key = StackInstanceKey {
                    stack_set_name: stack_set_name.to_string(),
                    account: acct.clone(),
                    region: rgn.clone(),
                };
                self.stack_instances.remove(&key);
            }
        }
        let operation_id = new_uuid();
        let ss = self.stack_sets.get_mut(stack_set_name).unwrap();
        let op = StackSetOperation {
            operation_id: operation_id.clone(),
            action: "DELETE".to_string(),
            status: "SUCCEEDED".to_string(),
            creation_timestamp: now_timestamp(),
            end_timestamp: Some(now_timestamp()),
            stack_set_id: ss.stack_set_id.clone(),
            results: Vec::new(),
        };
        ss.operations.push(op);
        Ok(operation_id)
    }

    pub fn update_stack_instances(
        &mut self,
        stack_set_name: &str,
        accounts: &[String],
        regions: &[String],
        parameter_overrides: Vec<StackParameter>,
    ) -> Result<String, CloudFormationError> {
        if !self.stack_sets.contains_key(stack_set_name) {
            return Err(CloudFormationError::StackSetNotFound(
                stack_set_name.to_string(),
            ));
        }
        for acct in accounts {
            for rgn in regions {
                let key = StackInstanceKey {
                    stack_set_name: stack_set_name.to_string(),
                    account: acct.clone(),
                    region: rgn.clone(),
                };
                if let Some(inst) = self.stack_instances.get_mut(&key) {
                    if !parameter_overrides.is_empty() {
                        inst.parameter_overrides = parameter_overrides.clone();
                    }
                    inst.status = "CURRENT".to_string();
                }
            }
        }
        let operation_id = new_uuid();
        let ss = self.stack_sets.get_mut(stack_set_name).unwrap();
        let op = StackSetOperation {
            operation_id: operation_id.clone(),
            action: "UPDATE".to_string(),
            status: "SUCCEEDED".to_string(),
            creation_timestamp: now_timestamp(),
            end_timestamp: Some(now_timestamp()),
            stack_set_id: ss.stack_set_id.clone(),
            results: Vec::new(),
        };
        ss.operations.push(op);
        Ok(operation_id)
    }

    pub fn describe_stack_instance(
        &self,
        stack_set_name: &str,
        account: &str,
        region: &str,
    ) -> Result<&StackInstance, CloudFormationError> {
        let key = StackInstanceKey {
            stack_set_name: stack_set_name.to_string(),
            account: account.to_string(),
            region: region.to_string(),
        };
        self.stack_instances.get(&key).ok_or_else(|| {
            CloudFormationError::StackInstanceNotFound(format!(
                "{stack_set_name}/{account}/{region}"
            ))
        })
    }

    pub fn list_stack_instances(&self, stack_set_name: &str) -> Vec<&StackInstance> {
        self.stack_instances
            .values()
            .filter(|i| i.stack_set_name == stack_set_name)
            .collect()
    }

    // --- StackSetOperation operations ---

    pub fn describe_stack_set_operation(
        &self,
        stack_set_name: &str,
        operation_id: &str,
    ) -> Result<&StackSetOperation, CloudFormationError> {
        let ss = self
            .stack_sets
            .get(stack_set_name)
            .ok_or_else(|| CloudFormationError::StackSetNotFound(stack_set_name.to_string()))?;
        ss.operations
            .iter()
            .find(|op| op.operation_id == operation_id)
            .ok_or_else(|| CloudFormationError::StackSetOperationNotFound(operation_id.to_string()))
    }

    pub fn list_stack_set_operations(
        &self,
        stack_set_name: &str,
    ) -> Result<Vec<&StackSetOperation>, CloudFormationError> {
        let ss = self
            .stack_sets
            .get(stack_set_name)
            .ok_or_else(|| CloudFormationError::StackSetNotFound(stack_set_name.to_string()))?;
        Ok(ss.operations.iter().collect())
    }

    pub fn list_stack_set_operation_results(
        &self,
        stack_set_name: &str,
        operation_id: &str,
    ) -> Result<Vec<&StackSetOperationResult>, CloudFormationError> {
        let op = self.describe_stack_set_operation(stack_set_name, operation_id)?;
        Ok(op.results.iter().collect())
    }

    pub fn stop_stack_set_operation(
        &mut self,
        stack_set_name: &str,
        operation_id: &str,
    ) -> Result<(), CloudFormationError> {
        let ss = self
            .stack_sets
            .get_mut(stack_set_name)
            .ok_or_else(|| CloudFormationError::StackSetNotFound(stack_set_name.to_string()))?;
        let op = ss
            .operations
            .iter_mut()
            .find(|op| op.operation_id == operation_id)
            .ok_or_else(|| {
                CloudFormationError::StackSetOperationNotFound(operation_id.to_string())
            })?;
        op.status = "STOPPED".to_string();
        Ok(())
    }

    pub fn update_termination_protection(
        &mut self,
        stack_name: &str,
        enable: bool,
    ) -> Result<String, CloudFormationError> {
        let stack = self
            .stacks
            .get_mut(stack_name)
            .ok_or_else(|| CloudFormationError::StackNotFound(stack_name.to_string()))?;
        stack.enable_termination_protection = enable;
        Ok(stack.stack_id.clone())
    }

    pub fn list_imports(&self, export_name: &str) -> Vec<String> {
        self.stacks
            .values()
            .filter(|s| {
                // Find stacks that reference this export in their parameters
                // (simplified: check template body for Fn::ImportValue references)
                s.template_body
                    .as_deref()
                    .map(|t| t.contains(export_name))
                    .unwrap_or(false)
            })
            .map(|s| s.stack_name.clone())
            .collect()
    }

    pub fn cancel_update_stack(&mut self, stack_name: &str) -> Result<(), CloudFormationError> {
        let stack = self
            .stacks
            .get_mut(stack_name)
            .ok_or_else(|| CloudFormationError::StackNotFound(stack_name.to_string()))?;
        if stack.stack_status != "UPDATE_IN_PROGRESS" && stack.stack_status != "UPDATE_COMPLETE" {
            // Allow cancel from UPDATE_COMPLETE_CLEANUP_IN_PROGRESS too
        }
        stack.stack_status = "UPDATE_ROLLBACK_IN_PROGRESS".to_string();
        let event = make_stack_event(
            &stack.stack_id,
            stack_name,
            stack_name,
            "AWS::CloudFormation::Stack",
            "UPDATE_ROLLBACK_IN_PROGRESS",
        );
        stack.events.push(event);
        Ok(())
    }

    pub fn rollback_stack(&mut self, stack_name: &str) -> Result<String, CloudFormationError> {
        let stack = self
            .stacks
            .get_mut(stack_name)
            .ok_or_else(|| CloudFormationError::StackNotFound(stack_name.to_string()))?;
        stack.stack_status = "UPDATE_ROLLBACK_COMPLETE".to_string();
        let event = make_stack_event(
            &stack.stack_id,
            stack_name,
            stack_name,
            "AWS::CloudFormation::Stack",
            "UPDATE_ROLLBACK_COMPLETE",
        );
        stack.events.push(event);
        Ok(stack.stack_id.clone())
    }
}
