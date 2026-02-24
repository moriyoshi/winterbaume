use std::collections::HashMap;

use crate::types::*;

/// In-memory state for the SWF service.
#[derive(Debug, Default)]
pub struct SwfState {
    /// Domains keyed by domain name.
    pub domains: HashMap<String, Domain>,
    /// Activity types keyed by (domain, name, version).
    pub activity_types: HashMap<(String, String, String), ActivityTypeDef>,
    /// Workflow types keyed by (domain, name, version).
    pub workflow_types: HashMap<(String, String, String), WorkflowTypeDef>,
    /// Workflow executions keyed by (domain, workflow_id, run_id).
    pub executions: HashMap<(String, String, String), WorkflowExecutionDef>,
    /// Activity tasks currently polled by workers, keyed by task_token.
    pub activity_tasks: HashMap<String, ActivityTaskEntry>,
    /// Pending signals keyed by (domain, workflow_id).
    pub signals: HashMap<(String, String), Vec<WorkflowSignal>>,
}

/// Error type for SWF operations.
#[derive(Debug, thiserror::Error)]
pub enum SwfError {
    #[error("{0}")]
    DomainAlreadyExists(String),
    #[error("{0}")]
    DomainDeprecated(String),
    #[error("ActivityType=[name={0}, version={1}]")]
    ActivityTypeAlreadyExists(String, String),
    #[error("WorkflowType=[name={0}, version={1}]")]
    WorkflowTypeAlreadyExists(String, String),
    #[error("ActivityType=[name={0}, version={1}]")]
    ActivityTypeDeprecated(String, String),
    #[error("WorkflowType=[name={0}, version={1}]")]
    WorkflowTypeDeprecated(String, String),
    #[error("Unknown domain: {0}")]
    UnknownDomain(String),
    #[error("Unknown type: {type_name}=[name={name}, version={version}]")]
    UnknownType {
        type_name: String,
        name: String,
        version: String,
    },
    #[error("Unknown execution: workflowId={workflow_id}, runId={run_id}")]
    UnknownExecution { workflow_id: String, run_id: String },
    #[error("No open execution for workflowId={0}")]
    NoOpenExecution(String),
    #[error("Execution already closed: workflowId={0}")]
    ExecutionAlreadyClosed(String),
    #[error("Execution closed: workflowId={0}")]
    ExecutionClosed(String),
    #[error("Unknown task token: {0}")]
    UnknownTaskToken(String),
    #[error("Task already completed")]
    TaskAlreadyCompleted,
    #[error("Workflow execution already running: {0}")]
    WorkflowExecutionAlreadyStarted(String),
}

impl SwfState {
    pub fn register_domain(
        &mut self,
        name: &str,
        description: Option<&str>,
        retention_period: &str,
        account_id: &str,
        region: &str,
    ) -> Result<(), SwfError> {
        if self.domains.contains_key(name) {
            return Err(SwfError::DomainAlreadyExists(name.to_string()));
        }

        let arn = format!("arn:aws:swf:{region}:{account_id}:/domain/{name}");

        let domain = Domain {
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            status: RegistrationStatus::Registered,
            workflow_execution_retention_period_in_days: retention_period.to_string(),
            arn,
        };

        self.domains.insert(name.to_string(), domain);
        Ok(())
    }

    pub fn describe_domain(&self, name: &str) -> Result<&Domain, SwfError> {
        self.domains.get(name).ok_or_else(|| unknown_resource(name))
    }

    pub fn deprecate_domain(&mut self, name: &str) -> Result<(), SwfError> {
        let domain = self
            .domains
            .get_mut(name)
            .ok_or_else(|| unknown_resource(name))?;

        if domain.status == RegistrationStatus::Deprecated {
            return Err(SwfError::DomainDeprecated(name.to_string()));
        }

        domain.status = RegistrationStatus::Deprecated;
        Ok(())
    }

    pub fn list_domains(&self, registration_status: RegistrationStatus) -> Vec<&Domain> {
        let mut domains: Vec<&Domain> = self
            .domains
            .values()
            .filter(|d| d.status == registration_status)
            .collect();
        domains.sort_by(|a, b| a.name.cmp(&b.name));
        domains
    }

    // --- Activity Type operations ---

    pub fn register_activity_type(
        &mut self,
        domain: &str,
        name: &str,
        version: &str,
        description: Option<&str>,
        default_task_list: Option<&str>,
        default_task_heartbeat_timeout: Option<&str>,
        default_task_schedule_to_start_timeout: Option<&str>,
        default_task_schedule_to_close_timeout: Option<&str>,
        default_task_start_to_close_timeout: Option<&str>,
        default_task_priority: Option<&str>,
    ) -> Result<(), SwfError> {
        self.ensure_domain_exists(domain)?;
        let key = (domain.to_string(), name.to_string(), version.to_string());
        if self.activity_types.contains_key(&key) {
            return Err(SwfError::ActivityTypeAlreadyExists(
                name.to_string(),
                version.to_string(),
            ));
        }

        let now = timestamp();
        let at = ActivityTypeDef {
            name: name.to_string(),
            version: version.to_string(),
            domain: domain.to_string(),
            description: description.map(|s| s.to_string()),
            status: RegistrationStatus::Registered,
            creation_date: now,
            deprecation_date: None,
            default_task_list: default_task_list.map(|s| s.to_string()),
            default_task_heartbeat_timeout: default_task_heartbeat_timeout.map(|s| s.to_string()),
            default_task_schedule_to_start_timeout: default_task_schedule_to_start_timeout
                .map(|s| s.to_string()),
            default_task_schedule_to_close_timeout: default_task_schedule_to_close_timeout
                .map(|s| s.to_string()),
            default_task_start_to_close_timeout: default_task_start_to_close_timeout
                .map(|s| s.to_string()),
            default_task_priority: default_task_priority.map(|s| s.to_string()),
        };
        self.activity_types.insert(key, at);
        Ok(())
    }

    pub fn deprecate_activity_type(
        &mut self,
        domain: &str,
        name: &str,
        version: &str,
    ) -> Result<(), SwfError> {
        self.ensure_domain_exists(domain)?;
        let key = (domain.to_string(), name.to_string(), version.to_string());
        let at = self
            .activity_types
            .get_mut(&key)
            .ok_or_else(|| unknown_type("ActivityType", name, version))?;
        if at.status == RegistrationStatus::Deprecated {
            return Err(SwfError::ActivityTypeDeprecated(
                name.to_string(),
                version.to_string(),
            ));
        }
        at.status = RegistrationStatus::Deprecated;
        at.deprecation_date = Some(timestamp());
        Ok(())
    }

    pub fn describe_activity_type(
        &self,
        domain: &str,
        name: &str,
        version: &str,
    ) -> Result<&ActivityTypeDef, SwfError> {
        self.ensure_domain_exists(domain)?;
        let key = (domain.to_string(), name.to_string(), version.to_string());
        self.activity_types
            .get(&key)
            .ok_or_else(|| unknown_type("ActivityType", name, version))
    }

    // --- Workflow Type operations ---

    pub fn register_workflow_type(
        &mut self,
        domain: &str,
        name: &str,
        version: &str,
        description: Option<&str>,
        default_task_list: Option<&str>,
        default_execution_start_to_close_timeout: Option<&str>,
        default_task_start_to_close_timeout: Option<&str>,
        default_child_policy: Option<&str>,
        default_lambda_role: Option<&str>,
        default_task_priority: Option<&str>,
    ) -> Result<(), SwfError> {
        self.ensure_domain_exists(domain)?;
        let key = (domain.to_string(), name.to_string(), version.to_string());
        if self.workflow_types.contains_key(&key) {
            return Err(SwfError::WorkflowTypeAlreadyExists(
                name.to_string(),
                version.to_string(),
            ));
        }

        let now = timestamp();
        let wt = WorkflowTypeDef {
            name: name.to_string(),
            version: version.to_string(),
            domain: domain.to_string(),
            description: description.map(|s| s.to_string()),
            status: RegistrationStatus::Registered,
            creation_date: now,
            deprecation_date: None,
            default_task_list: default_task_list.map(|s| s.to_string()),
            default_execution_start_to_close_timeout: default_execution_start_to_close_timeout
                .map(|s| s.to_string()),
            default_task_start_to_close_timeout: default_task_start_to_close_timeout
                .map(|s| s.to_string()),
            default_child_policy: default_child_policy.map(|s| s.to_string()),
            default_lambda_role: default_lambda_role.map(|s| s.to_string()),
            default_task_priority: default_task_priority.map(|s| s.to_string()),
        };
        self.workflow_types.insert(key, wt);
        Ok(())
    }

    pub fn deprecate_workflow_type(
        &mut self,
        domain: &str,
        name: &str,
        version: &str,
    ) -> Result<(), SwfError> {
        self.ensure_domain_exists(domain)?;
        let key = (domain.to_string(), name.to_string(), version.to_string());
        let wt = self
            .workflow_types
            .get_mut(&key)
            .ok_or_else(|| unknown_type("WorkflowType", name, version))?;
        if wt.status == RegistrationStatus::Deprecated {
            return Err(SwfError::WorkflowTypeDeprecated(
                name.to_string(),
                version.to_string(),
            ));
        }
        wt.status = RegistrationStatus::Deprecated;
        wt.deprecation_date = Some(timestamp());
        Ok(())
    }

    pub fn describe_workflow_type(
        &self,
        domain: &str,
        name: &str,
        version: &str,
    ) -> Result<&WorkflowTypeDef, SwfError> {
        self.ensure_domain_exists(domain)?;
        let key = (domain.to_string(), name.to_string(), version.to_string());
        self.workflow_types
            .get(&key)
            .ok_or_else(|| unknown_type("WorkflowType", name, version))
    }

    // --- Workflow Execution operations ---

    pub fn start_workflow_execution(
        &mut self,
        domain: &str,
        workflow_id: &str,
        workflow_type_name: &str,
        workflow_type_version: &str,
        task_list: Option<&str>,
        execution_start_to_close_timeout: Option<&str>,
        task_start_to_close_timeout: Option<&str>,
        child_policy: Option<&str>,
        tag_list: Option<Vec<String>>,
        lambda_role: Option<&str>,
        task_priority: Option<&str>,
    ) -> Result<String, SwfError> {
        self.ensure_domain_exists(domain)?;

        // Check workflow type exists
        let wt_key = (
            domain.to_string(),
            workflow_type_name.to_string(),
            workflow_type_version.to_string(),
        );
        let wt = self.workflow_types.get(&wt_key).ok_or_else(|| {
            unknown_type("WorkflowType", workflow_type_name, workflow_type_version)
        })?;

        // Check workflow type is not deprecated
        if wt.status == RegistrationStatus::Deprecated {
            return Err(SwfError::WorkflowTypeDeprecated(
                workflow_type_name.to_string(),
                workflow_type_version.to_string(),
            ));
        }

        // Check no open execution with same workflow_id in this domain
        let has_open = self.executions.values().any(|e| {
            e.domain == domain && e.workflow_id == workflow_id && e.status == ExecutionStatus::Open
        });
        if has_open {
            return Err(SwfError::WorkflowExecutionAlreadyStarted(
                workflow_id.to_string(),
            ));
        }

        let run_id = uuid::Uuid::new_v4().to_string();
        let now = timestamp();

        let tl = task_list
            .map(|s| s.to_string())
            .or_else(|| wt.default_task_list.clone())
            .unwrap_or_else(|| "default".to_string());
        let cp = child_policy
            .map(|s| s.to_string())
            .or_else(|| wt.default_child_policy.clone())
            .unwrap_or_else(|| "TERMINATE".to_string());
        let estct = execution_start_to_close_timeout
            .map(|s| s.to_string())
            .or_else(|| wt.default_execution_start_to_close_timeout.clone())
            .unwrap_or_else(|| "3600".to_string());
        let tstct = task_start_to_close_timeout
            .map(|s| s.to_string())
            .or_else(|| wt.default_task_start_to_close_timeout.clone())
            .unwrap_or_else(|| "300".to_string());

        let initial_event = HistoryEventDef {
            event_id: 1,
            event_timestamp: now,
            event_type: "WorkflowExecutionStarted".to_string(),
        };

        let exec = WorkflowExecutionDef {
            workflow_id: workflow_id.to_string(),
            run_id: run_id.clone(),
            domain: domain.to_string(),
            workflow_type_name: workflow_type_name.to_string(),
            workflow_type_version: workflow_type_version.to_string(),
            status: ExecutionStatus::Open,
            close_status: None,
            start_timestamp: now,
            close_timestamp: None,
            tag_list,
            cancel_requested: false,
            task_list: tl,
            child_policy: cp,
            execution_start_to_close_timeout: estct,
            task_start_to_close_timeout: tstct,
            task_priority: task_priority.map(|s| s.to_string()),
            lambda_role: lambda_role.map(|s| s.to_string()),
            history_events: vec![initial_event],
        };

        let key = (domain.to_string(), workflow_id.to_string(), run_id.clone());
        self.executions.insert(key, exec);
        Ok(run_id)
    }

    pub fn describe_workflow_execution(
        &self,
        domain: &str,
        workflow_id: &str,
        run_id: &str,
    ) -> Result<&WorkflowExecutionDef, SwfError> {
        self.ensure_domain_exists(domain)?;
        let key = (
            domain.to_string(),
            workflow_id.to_string(),
            run_id.to_string(),
        );
        self.executions
            .get(&key)
            .ok_or_else(|| SwfError::UnknownExecution {
                workflow_id: workflow_id.to_string(),
                run_id: run_id.to_string(),
            })
    }

    pub fn terminate_workflow_execution(
        &mut self,
        domain: &str,
        workflow_id: &str,
        run_id: Option<&str>,
        reason: Option<&str>,
        details: Option<&str>,
        child_policy: Option<&str>,
    ) -> Result<(), SwfError> {
        self.ensure_domain_exists(domain)?;

        // Find the execution - if run_id is given use it, otherwise find the open one
        let exec = if let Some(rid) = run_id {
            let key = (domain.to_string(), workflow_id.to_string(), rid.to_string());
            self.executions
                .get_mut(&key)
                .ok_or_else(|| SwfError::UnknownExecution {
                    workflow_id: workflow_id.to_string(),
                    run_id: rid.to_string(),
                })?
        } else {
            self.executions
                .values_mut()
                .find(|e| {
                    e.domain == domain
                        && e.workflow_id == workflow_id
                        && e.status == ExecutionStatus::Open
                })
                .ok_or_else(|| SwfError::NoOpenExecution(workflow_id.to_string()))?
        };

        if exec.status == ExecutionStatus::Closed {
            return Err(SwfError::ExecutionAlreadyClosed(workflow_id.to_string()));
        }

        let now = timestamp();
        let _reason = reason;
        let _details = details;
        if let Some(cp) = child_policy {
            exec.child_policy = cp.to_string();
        }

        let event_id = exec.history_events.len() as i64 + 1;
        exec.history_events.push(HistoryEventDef {
            event_id,
            event_timestamp: now,
            event_type: "WorkflowExecutionTerminated".to_string(),
        });

        exec.status = ExecutionStatus::Closed;
        exec.close_status = Some("TERMINATED".to_string());
        exec.close_timestamp = Some(now);
        Ok(())
    }

    pub fn get_workflow_execution_history(
        &self,
        domain: &str,
        workflow_id: &str,
        run_id: &str,
    ) -> Result<&WorkflowExecutionDef, SwfError> {
        self.describe_workflow_execution(domain, workflow_id, run_id)
    }

    pub fn count_closed_workflow_executions(&self, domain: &str) -> Result<i32, SwfError> {
        self.ensure_domain_exists(domain)?;
        let count = self
            .executions
            .values()
            .filter(|e| e.domain == domain && e.status == ExecutionStatus::Closed)
            .count();
        Ok(count as i32)
    }

    pub fn count_open_workflow_executions(&self, domain: &str) -> Result<i32, SwfError> {
        self.ensure_domain_exists(domain)?;
        let count = self
            .executions
            .values()
            .filter(|e| e.domain == domain && e.status == ExecutionStatus::Open)
            .count();
        Ok(count as i32)
    }

    pub fn count_pending_activity_tasks(
        &self,
        domain: &str,
        _task_list: &str,
    ) -> Result<i32, SwfError> {
        self.ensure_domain_exists(domain)?;
        // In a real implementation, this would count pending activity tasks.
        // For mock purposes, return 0.
        Ok(0)
    }

    pub fn count_pending_decision_tasks(
        &self,
        domain: &str,
        _task_list: &str,
    ) -> Result<i32, SwfError> {
        self.ensure_domain_exists(domain)?;
        // In a real implementation, this would count pending decision tasks.
        // For mock purposes, return 0.
        Ok(0)
    }

    pub fn list_open_workflow_executions(
        &self,
        domain: &str,
    ) -> Result<Vec<&WorkflowExecutionDef>, SwfError> {
        self.ensure_domain_exists(domain)?;
        let mut execs: Vec<&WorkflowExecutionDef> = self
            .executions
            .values()
            .filter(|e| e.domain == domain && e.status == ExecutionStatus::Open)
            .collect();
        execs.sort_by(|a, b| {
            b.start_timestamp
                .partial_cmp(&a.start_timestamp)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        Ok(execs)
    }

    pub fn list_closed_workflow_executions(
        &self,
        domain: &str,
    ) -> Result<Vec<&WorkflowExecutionDef>, SwfError> {
        self.ensure_domain_exists(domain)?;
        let mut execs: Vec<&WorkflowExecutionDef> = self
            .executions
            .values()
            .filter(|e| e.domain == domain && e.status == ExecutionStatus::Closed)
            .collect();
        execs.sort_by(|a, b| {
            b.start_timestamp
                .partial_cmp(&a.start_timestamp)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        Ok(execs)
    }

    pub fn poll_for_decision_task(
        &self,
        domain: &str,
        _task_list: &str,
    ) -> Result<Option<&WorkflowExecutionDef>, SwfError> {
        self.ensure_domain_exists(domain)?;
        // Return the first open execution for the task list (simplified mock)
        let exec = self
            .executions
            .values()
            .find(|e| e.domain == domain && e.status == ExecutionStatus::Open);
        Ok(exec)
    }

    /// Poll for an activity task. Returns a task token and activity info if an
    /// activity task is scheduled (simplified: always returns an empty response
    /// unless there are scheduled activity tasks in the execution history).
    pub fn poll_for_activity_task(
        &mut self,
        domain: &str,
        task_list: &str,
    ) -> Result<Option<ActivityTaskEntry>, SwfError> {
        self.ensure_domain_exists(domain)?;
        // Find any open execution that has a scheduled activity task
        let token = uuid::Uuid::new_v4().to_string();
        // Find the first open execution in this domain/task_list
        let exec_key = self
            .executions
            .iter()
            .find(|(_, e)| {
                e.domain == domain && e.status == ExecutionStatus::Open && e.task_list == task_list
            })
            .map(|(k, _)| k.clone());

        if let Some(key) = exec_key {
            let exec = &self.executions[&key];
            let entry = ActivityTaskEntry {
                task_token: token.clone(),
                activity_id: format!("activity-{}", &token[..8]),
                activity_type_name: exec.workflow_type_name.clone(),
                activity_type_version: exec.workflow_type_version.clone(),
                domain: domain.to_string(),
                workflow_id: exec.workflow_id.clone(),
                run_id: exec.run_id.clone(),
                input: None,
                completed: false,
            };
            self.activity_tasks.insert(token, entry.clone());
            Ok(Some(entry))
        } else {
            Ok(None)
        }
    }

    /// Record a heartbeat for an activity task. Returns the cancel status.
    pub fn record_activity_task_heartbeat(
        &self,
        task_token: &str,
        _details: Option<&str>,
    ) -> Result<bool, SwfError> {
        match self.activity_tasks.get(task_token) {
            Some(task) => {
                // Check if the execution has been terminated
                let exec_key = (
                    task.domain.clone(),
                    task.workflow_id.clone(),
                    task.run_id.clone(),
                );
                let cancel_requested = self
                    .executions
                    .get(&exec_key)
                    .map(|e| e.cancel_requested)
                    .unwrap_or(false);
                Ok(cancel_requested)
            }
            None => Err(SwfError::UnknownTaskToken(task_token.to_string())),
        }
    }

    /// Complete an activity task.
    pub fn respond_activity_task_completed(
        &mut self,
        task_token: &str,
        result: Option<&str>,
    ) -> Result<(), SwfError> {
        let task = self
            .activity_tasks
            .get_mut(task_token)
            .ok_or_else(|| SwfError::UnknownTaskToken(task_token.to_string()))?;

        if task.completed {
            return Err(SwfError::TaskAlreadyCompleted);
        }

        let exec_key = (
            task.domain.clone(),
            task.workflow_id.clone(),
            task.run_id.clone(),
        );
        task.completed = true;

        if let Some(exec) = self.executions.get_mut(&exec_key) {
            let event_id = exec.history_events.len() as i64 + 1;
            exec.history_events.push(HistoryEventDef {
                event_id,
                event_timestamp: timestamp(),
                event_type: "ActivityTaskCompleted".to_string(),
            });
            let _ = result;
        }
        Ok(())
    }

    /// Fail an activity task.
    pub fn respond_activity_task_failed(
        &mut self,
        task_token: &str,
        reason: Option<&str>,
        details: Option<&str>,
    ) -> Result<(), SwfError> {
        let task = self
            .activity_tasks
            .get_mut(task_token)
            .ok_or_else(|| SwfError::UnknownTaskToken(task_token.to_string()))?;

        if task.completed {
            return Err(SwfError::TaskAlreadyCompleted);
        }

        let exec_key = (
            task.domain.clone(),
            task.workflow_id.clone(),
            task.run_id.clone(),
        );
        task.completed = true;

        if let Some(exec) = self.executions.get_mut(&exec_key) {
            let event_id = exec.history_events.len() as i64 + 1;
            exec.history_events.push(HistoryEventDef {
                event_id,
                event_timestamp: timestamp(),
                event_type: "ActivityTaskFailed".to_string(),
            });
            let _ = (reason, details);
        }
        Ok(())
    }

    /// Complete a decision task.
    pub fn respond_decision_task_completed(
        &mut self,
        task_token: &str,
        decisions: Vec<serde_json::Value>,
    ) -> Result<(), SwfError> {
        // Find which execution this task belongs to (simplified: match by open executions)
        let _ = (task_token, decisions);
        // In a real implementation we'd process decisions and modify the workflow
        Ok(())
    }

    /// Signal a workflow execution.
    pub fn signal_workflow_execution(
        &mut self,
        domain: &str,
        workflow_id: &str,
        signal_name: &str,
        input: Option<&str>,
        run_id: Option<&str>,
    ) -> Result<(), SwfError> {
        self.ensure_domain_exists(domain)?;

        // Find the execution
        let exec = if let Some(rid) = run_id {
            let key = (domain.to_string(), workflow_id.to_string(), rid.to_string());
            self.executions
                .get_mut(&key)
                .ok_or_else(|| SwfError::UnknownExecution {
                    workflow_id: workflow_id.to_string(),
                    run_id: rid.to_string(),
                })?
        } else {
            self.executions
                .values_mut()
                .find(|e| {
                    e.domain == domain
                        && e.workflow_id == workflow_id
                        && e.status == ExecutionStatus::Open
                })
                .ok_or_else(|| SwfError::NoOpenExecution(workflow_id.to_string()))?
        };

        if exec.status == ExecutionStatus::Closed {
            return Err(SwfError::ExecutionClosed(workflow_id.to_string()));
        }

        let event_id = exec.history_events.len() as i64 + 1;
        exec.history_events.push(HistoryEventDef {
            event_id,
            event_timestamp: timestamp(),
            event_type: "WorkflowExecutionSignaled".to_string(),
        });

        let signal = WorkflowSignal {
            signal_name: signal_name.to_string(),
            input: input.map(|s| s.to_string()),
        };
        self.signals
            .entry((domain.to_string(), workflow_id.to_string()))
            .or_default()
            .push(signal);

        Ok(())
    }

    /// Undeprecate a domain.
    pub fn undeprecate_domain(&mut self, name: &str) -> Result<(), SwfError> {
        let domain = self
            .domains
            .get_mut(name)
            .ok_or_else(|| unknown_resource(name))?;

        if domain.status == RegistrationStatus::Registered {
            return Err(SwfError::DomainAlreadyExists(name.to_string()));
        }

        domain.status = RegistrationStatus::Registered;
        Ok(())
    }

    /// List activity types for a domain.
    pub fn list_activity_types(
        &self,
        domain: &str,
        registration_status: RegistrationStatus,
    ) -> Result<Vec<&ActivityTypeDef>, SwfError> {
        self.ensure_domain_exists(domain)?;
        let mut types: Vec<&ActivityTypeDef> = self
            .activity_types
            .values()
            .filter(|at| at.domain == domain && at.status == registration_status)
            .collect();
        types.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(types)
    }

    /// List workflow types for a domain.
    pub fn list_workflow_types(
        &self,
        domain: &str,
        registration_status: RegistrationStatus,
    ) -> Result<Vec<&WorkflowTypeDef>, SwfError> {
        self.ensure_domain_exists(domain)?;
        let mut types: Vec<&WorkflowTypeDef> = self
            .workflow_types
            .values()
            .filter(|wt| wt.domain == domain && wt.status == registration_status)
            .collect();
        types.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(types)
    }

    // --- helpers ---

    fn ensure_domain_exists(&self, name: &str) -> Result<(), SwfError> {
        if !self.domains.contains_key(name) {
            return Err(unknown_resource(name));
        }
        Ok(())
    }
}

fn unknown_resource(name: &str) -> SwfError {
    SwfError::UnknownDomain(name.to_string())
}

fn unknown_type(type_name: &str, name: &str, version: &str) -> SwfError {
    SwfError::UnknownType {
        type_name: type_name.to_string(),
        name: name.to_string(),
        version: version.to_string(),
    }
}

fn timestamp() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
}
