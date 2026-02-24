use std::collections::HashMap;

use chrono::Utc;
use serde_json::Value;
use thiserror::Error;
use uuid::Uuid;

use crate::types::*;

#[derive(Debug, Default)]
pub struct DataPipelineState {
    pub pipelines: HashMap<String, Pipeline>,
    pub tasks: HashMap<String, TaskEntry>,
    next_id: u64,
}

#[derive(Debug, Clone)]
pub struct TaskEntry {
    pub task_id: String,
    pub pipeline_id: String,
    pub attempt_id: String,
    pub status: TaskStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    InProgress,
    Finished,
    Failed,
    FalseAlarm,
}

#[derive(Debug, Error)]
pub enum DataPipelineError {
    #[error("Pipeline '{pipeline_id}' does not exist.")]
    PipelineNotFound { pipeline_id: String },

    #[error("A pipeline with uniqueId '{unique_id}' already exists.")]
    DuplicateUniqueId { unique_id: String },

    #[error("Task '{task_id}' does not exist.")]
    TaskNotFound { task_id: String },

    #[error("Invalid task status: '{task_status}'.")]
    InvalidTaskStatus { task_status: String },

    #[error("Object '{object_id}' does not exist in pipeline '{pipeline_id}'.")]
    ObjectNotFound {
        object_id: String,
        pipeline_id: String,
    },
}

impl DataPipelineState {
    fn next_pipeline_id(&mut self) -> String {
        self.next_id += 1;
        format!("df-{:017}", self.next_id)
    }

    pub fn create_pipeline(
        &mut self,
        name: &str,
        unique_id: &str,
        description: Option<&str>,
        tags: Vec<PipelineTag>,
    ) -> Result<&Pipeline, DataPipelineError> {
        // Check for duplicate unique_id
        for p in self.pipelines.values() {
            if p.unique_id == unique_id {
                return Err(DataPipelineError::DuplicateUniqueId {
                    unique_id: unique_id.to_string(),
                });
            }
        }

        let pipeline_id = self.next_pipeline_id();
        let pipeline = Pipeline {
            pipeline_id: pipeline_id.clone(),
            name: name.to_string(),
            description: description.unwrap_or("").to_string(),
            unique_id: unique_id.to_string(),
            tags,
            created_at: Utc::now(),
            status: PipelineStatus::Pending,
            pipeline_objects: Vec::new(),
            parameter_objects: Vec::new(),
            parameter_values: Vec::new(),
        };

        self.pipelines.insert(pipeline_id.clone(), pipeline);
        Ok(self.pipelines.get(&pipeline_id).unwrap())
    }

    pub fn delete_pipeline(&mut self, pipeline_id: &str) -> Result<(), DataPipelineError> {
        if self.pipelines.remove(pipeline_id).is_none() {
            return Err(DataPipelineError::PipelineNotFound {
                pipeline_id: pipeline_id.to_string(),
            });
        }
        Ok(())
    }

    pub fn describe_pipelines(
        &self,
        pipeline_ids: &[String],
    ) -> Result<Vec<&Pipeline>, DataPipelineError> {
        let mut result = Vec::new();
        for id in pipeline_ids {
            let pipeline =
                self.pipelines
                    .get(id)
                    .ok_or_else(|| DataPipelineError::PipelineNotFound {
                        pipeline_id: id.to_string(),
                    })?;
            result.push(pipeline);
        }
        Ok(result)
    }

    pub fn list_pipelines(&self) -> Vec<&Pipeline> {
        self.pipelines.values().collect()
    }

    pub fn get_pipeline_definition(
        &self,
        pipeline_id: &str,
    ) -> Result<&Pipeline, DataPipelineError> {
        self.pipelines
            .get(pipeline_id)
            .ok_or_else(|| DataPipelineError::PipelineNotFound {
                pipeline_id: pipeline_id.to_string(),
            })
    }

    pub fn put_pipeline_definition(
        &mut self,
        pipeline_id: &str,
        pipeline_objects: Vec<PipelineObject>,
        parameter_objects: Vec<Value>,
        parameter_values: Vec<Value>,
    ) -> Result<(), DataPipelineError> {
        let pipeline = self.pipelines.get_mut(pipeline_id).ok_or_else(|| {
            DataPipelineError::PipelineNotFound {
                pipeline_id: pipeline_id.to_string(),
            }
        })?;

        pipeline.pipeline_objects = pipeline_objects;
        pipeline.parameter_objects = parameter_objects;
        pipeline.parameter_values = parameter_values;
        Ok(())
    }

    pub fn activate_pipeline(&mut self, pipeline_id: &str) -> Result<(), DataPipelineError> {
        let pipeline = self.pipelines.get_mut(pipeline_id).ok_or_else(|| {
            DataPipelineError::PipelineNotFound {
                pipeline_id: pipeline_id.to_string(),
            }
        })?;
        pipeline.status = PipelineStatus::Active;
        Ok(())
    }

    pub fn deactivate_pipeline(&mut self, pipeline_id: &str) -> Result<(), DataPipelineError> {
        let pipeline = self.pipelines.get_mut(pipeline_id).ok_or_else(|| {
            DataPipelineError::PipelineNotFound {
                pipeline_id: pipeline_id.to_string(),
            }
        })?;
        pipeline.status = PipelineStatus::Inactive;
        Ok(())
    }

    pub fn add_tags(
        &mut self,
        pipeline_id: &str,
        new_tags: Vec<PipelineTag>,
    ) -> Result<(), DataPipelineError> {
        let pipeline = self.pipelines.get_mut(pipeline_id).ok_or_else(|| {
            DataPipelineError::PipelineNotFound {
                pipeline_id: pipeline_id.to_string(),
            }
        })?;
        for tag in new_tags {
            if let Some(existing) = pipeline.tags.iter_mut().find(|t| t.key == tag.key) {
                existing.value = tag.value;
            } else {
                pipeline.tags.push(tag);
            }
        }
        Ok(())
    }

    pub fn remove_tags(
        &mut self,
        pipeline_id: &str,
        tag_keys: &[String],
    ) -> Result<(), DataPipelineError> {
        let pipeline = self.pipelines.get_mut(pipeline_id).ok_or_else(|| {
            DataPipelineError::PipelineNotFound {
                pipeline_id: pipeline_id.to_string(),
            }
        })?;
        pipeline.tags.retain(|t| !tag_keys.contains(&t.key));
        Ok(())
    }

    pub fn describe_objects(
        &self,
        pipeline_id: &str,
        object_ids: &[String],
    ) -> Result<Vec<&PipelineObject>, DataPipelineError> {
        let pipeline =
            self.pipelines
                .get(pipeline_id)
                .ok_or_else(|| DataPipelineError::PipelineNotFound {
                    pipeline_id: pipeline_id.to_string(),
                })?;

        if object_ids.is_empty() {
            return Ok(pipeline.pipeline_objects.iter().collect());
        }

        let result: Vec<&PipelineObject> = pipeline
            .pipeline_objects
            .iter()
            .filter(|o| object_ids.contains(&o.id))
            .collect();
        Ok(result)
    }

    pub fn query_objects(
        &self,
        pipeline_id: &str,
        sphere: &str,
    ) -> Result<Vec<String>, DataPipelineError> {
        let pipeline =
            self.pipelines
                .get(pipeline_id)
                .ok_or_else(|| DataPipelineError::PipelineNotFound {
                    pipeline_id: pipeline_id.to_string(),
                })?;

        // Return objects matching the sphere type from the pipeline objects' type field
        let ids: Vec<String> = pipeline
            .pipeline_objects
            .iter()
            .filter(|o| {
                if sphere == "COMPONENT" || sphere == "INSTANCE" || sphere == "ATTEMPT" {
                    // Find the @sphere or type field
                    let type_field = o
                        .fields
                        .iter()
                        .find(|f| f.key == "type" || f.key == "@sphere");
                    if let Some(field) = type_field {
                        if let Some(ref sv) = field.string_value {
                            return sv.to_uppercase().contains(&sphere.to_uppercase())
                                || sphere == "COMPONENT";
                        }
                    }
                    sphere == "COMPONENT"
                } else {
                    true
                }
            })
            .map(|o| o.id.clone())
            .collect();
        Ok(ids)
    }

    pub fn set_status(
        &mut self,
        pipeline_id: &str,
        object_ids: &[String],
        status: &str,
    ) -> Result<(), DataPipelineError> {
        let pipeline = self.pipelines.get_mut(pipeline_id).ok_or_else(|| {
            DataPipelineError::PipelineNotFound {
                pipeline_id: pipeline_id.to_string(),
            }
        })?;

        // Update @status field on the matching pipeline objects
        for obj in pipeline.pipeline_objects.iter_mut() {
            if object_ids.contains(&obj.id) {
                if let Some(field) = obj.fields.iter_mut().find(|f| f.key == "@status") {
                    field.string_value = Some(status.to_string());
                } else {
                    obj.fields.push(PipelineField {
                        key: "@status".to_string(),
                        string_value: Some(status.to_string()),
                        ref_value: None,
                    });
                }
            }
        }
        Ok(())
    }

    pub fn poll_for_task(&mut self, worker_group: &str) -> Option<(String, String, String)> {
        // Find an active pipeline with objects in SCHEDULED state
        for pipeline in self.pipelines.values() {
            if pipeline.status != PipelineStatus::Active {
                continue;
            }
            for obj in &pipeline.pipeline_objects {
                let wg_field = obj.fields.iter().find(|f| f.key == "workerGroup");
                let matches_wg = wg_field
                    .and_then(|f| f.string_value.as_deref())
                    .map(|wg| wg == worker_group)
                    .unwrap_or(false);
                if matches_wg {
                    let task_id = Uuid::new_v4().to_string();
                    let attempt_id = Uuid::new_v4().to_string();
                    let entry = TaskEntry {
                        task_id: task_id.clone(),
                        pipeline_id: pipeline.pipeline_id.clone(),
                        attempt_id: attempt_id.clone(),
                        status: TaskStatus::InProgress,
                    };
                    self.tasks.insert(task_id.clone(), entry);
                    return Some((task_id, pipeline.pipeline_id.clone(), attempt_id));
                }
            }
        }
        None
    }

    pub fn report_task_progress(&mut self, task_id: &str) -> Result<bool, DataPipelineError> {
        // Check the task exists
        if !self.tasks.contains_key(task_id) {
            return Err(DataPipelineError::TaskNotFound {
                task_id: task_id.to_string(),
            });
        }
        Ok(false) // not canceled
    }

    pub fn report_task_runner_heartbeat(&self, _taskrunner_id: &str) -> bool {
        false // not terminating
    }

    pub fn set_task_status(
        &mut self,
        task_id: &str,
        task_status: &str,
    ) -> Result<(), DataPipelineError> {
        let task = self
            .tasks
            .get_mut(task_id)
            .ok_or_else(|| DataPipelineError::TaskNotFound {
                task_id: task_id.to_string(),
            })?;

        task.status = match task_status {
            "FINISHED" => TaskStatus::Finished,
            "FAILED" => TaskStatus::Failed,
            "FALSE_ALARM" => TaskStatus::FalseAlarm,
            _ => {
                return Err(DataPipelineError::InvalidTaskStatus {
                    task_status: task_status.to_string(),
                });
            }
        };
        Ok(())
    }

    pub fn evaluate_expression(
        &self,
        pipeline_id: &str,
        object_id: &str,
        expression: &str,
    ) -> Result<String, DataPipelineError> {
        let pipeline =
            self.pipelines
                .get(pipeline_id)
                .ok_or_else(|| DataPipelineError::PipelineNotFound {
                    pipeline_id: pipeline_id.to_string(),
                })?;

        // Find the object
        let obj = pipeline
            .pipeline_objects
            .iter()
            .find(|o| o.id == object_id)
            .ok_or_else(|| DataPipelineError::ObjectNotFound {
                object_id: object_id.to_string(),
                pipeline_id: pipeline_id.to_string(),
            })?;

        // Simple expression evaluation: replace #{field} references
        let mut result = expression.to_string();
        for field in &obj.fields {
            let placeholder = format!("#{{{}}}", field.key);
            if let Some(ref sv) = field.string_value {
                result = result.replace(&placeholder, sv);
            }
        }
        Ok(result)
    }

    pub fn validate_pipeline_definition(&self, pipeline_id: &str) -> Result<(), DataPipelineError> {
        if !self.pipelines.contains_key(pipeline_id) {
            return Err(DataPipelineError::PipelineNotFound {
                pipeline_id: pipeline_id.to_string(),
            });
        }
        Ok(())
    }
}
