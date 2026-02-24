use std::collections::HashMap;

use chrono::Utc;
use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct PipesState {
    pub pipes: HashMap<String, Pipe>,
}

#[derive(Debug, Error)]
pub enum PipesError {
    #[error("Pipe {name} already exists.")]
    ConflictException { name: String },
    #[error("Pipe {name} does not exist.")]
    NotFoundException { name: String },
}

impl PipesState {
    pub fn create_pipe(
        &mut self,
        name: &str,
        source: &str,
        target: &str,
        region: &str,
        account_id: &str,
    ) -> Result<&Pipe, PipesError> {
        if self.pipes.contains_key(name) {
            return Err(PipesError::ConflictException {
                name: name.to_string(),
            });
        }

        let arn = format!("arn:aws:pipes:{region}:{account_id}:pipe/{name}");

        let now = Utc::now();
        let pipe = Pipe {
            name: name.to_string(),
            arn,
            source: source.to_string(),
            target: target.to_string(),
            description: None,
            enrichment: None,
            role_arn: None,
            desired_state: "RUNNING".to_string(),
            current_state: "RUNNING".to_string(),
            creation_time: now,
            last_modified_time: now,
            tags: std::collections::HashMap::new(),
        };

        self.pipes.insert(name.to_string(), pipe);
        Ok(self.pipes.get(name).unwrap())
    }

    pub fn describe_pipe(&self, name: &str) -> Result<&Pipe, PipesError> {
        self.pipes
            .get(name)
            .ok_or_else(|| PipesError::NotFoundException {
                name: name.to_string(),
            })
    }

    pub fn delete_pipe(&mut self, name: &str) -> Result<Pipe, PipesError> {
        self.pipes
            .remove(name)
            .ok_or_else(|| PipesError::NotFoundException {
                name: name.to_string(),
            })
    }

    pub fn list_pipes(&self) -> Vec<&Pipe> {
        self.pipes.values().collect()
    }

    pub fn start_pipe(&mut self, name: &str) -> Result<&Pipe, PipesError> {
        let pipe = self
            .pipes
            .get_mut(name)
            .ok_or_else(|| PipesError::NotFoundException {
                name: name.to_string(),
            })?;
        pipe.desired_state = "RUNNING".to_string();
        pipe.last_modified_time = Utc::now();
        Ok(self.pipes.get(name).unwrap())
    }

    pub fn stop_pipe(&mut self, name: &str) -> Result<&Pipe, PipesError> {
        let pipe = self
            .pipes
            .get_mut(name)
            .ok_or_else(|| PipesError::NotFoundException {
                name: name.to_string(),
            })?;
        pipe.desired_state = "STOPPED".to_string();
        pipe.last_modified_time = Utc::now();
        Ok(self.pipes.get(name).unwrap())
    }

    pub fn update_pipe(
        &mut self,
        name: &str,
        description: Option<&str>,
        desired_state: Option<&str>,
        enrichment: Option<&str>,
        role_arn: Option<&str>,
        source: Option<&str>,
        target: Option<&str>,
    ) -> Result<&Pipe, PipesError> {
        let pipe = self
            .pipes
            .get_mut(name)
            .ok_or_else(|| PipesError::NotFoundException {
                name: name.to_string(),
            })?;
        if let Some(v) = description {
            pipe.description = Some(v.to_string());
        }
        if let Some(v) = desired_state {
            pipe.desired_state = v.to_string();
        }
        if let Some(v) = enrichment {
            pipe.enrichment = Some(v.to_string());
        }
        if let Some(v) = role_arn {
            pipe.role_arn = Some(v.to_string());
        }
        if let Some(v) = source {
            pipe.source = v.to_string();
        }
        if let Some(v) = target {
            pipe.target = v.to_string();
        }
        pipe.last_modified_time = Utc::now();
        Ok(self.pipes.get(name).unwrap())
    }
}
