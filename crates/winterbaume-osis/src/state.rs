use std::collections::HashMap;

use chrono::Utc;
use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct OsisState {
    pub pipelines: HashMap<String, Pipeline>,
}

#[derive(Debug, Error)]
pub enum OsisError {
    #[error("Pipeline with name {pipeline_name} already exists.")]
    PipelineAlreadyExists { pipeline_name: String },

    #[error("Pipeline {pipeline_name} could not be found.")]
    PipelineNotFound { pipeline_name: String },

    #[error("Resource with ARN {arn} could not be found.")]
    ResourceNotFound { arn: String },
}

impl OsisState {
    pub fn create_pipeline(
        &mut self,
        pipeline_name: &str,
        min_units: i32,
        max_units: i32,
        pipeline_configuration_body: &str,
        account_id: &str,
        region: &str,
        initial_tags: HashMap<String, String>,
    ) -> Result<&Pipeline, OsisError> {
        if self.pipelines.contains_key(pipeline_name) {
            return Err(OsisError::PipelineAlreadyExists {
                pipeline_name: pipeline_name.to_string(),
            });
        }

        let now = Utc::now();
        let pipeline_arn = format!("arn:aws:osis:{region}:{account_id}:pipeline/{pipeline_name}");
        let ingest_endpoint = format!("{pipeline_name}-{account_id}.{region}.osis.amazonaws.com");

        let pipeline = Pipeline {
            pipeline_name: pipeline_name.to_string(),
            pipeline_arn,
            min_units,
            max_units,
            status: "ACTIVE".to_string(),
            pipeline_configuration_body: pipeline_configuration_body.to_string(),
            created_at: now,
            last_updated_at: now,
            ingest_endpoint_urls: vec![ingest_endpoint],
            tags: initial_tags,
        };

        self.pipelines.insert(pipeline_name.to_string(), pipeline);
        Ok(self.pipelines.get(pipeline_name).unwrap())
    }

    pub fn get_pipeline(&self, pipeline_name: &str) -> Result<&Pipeline, OsisError> {
        self.pipelines
            .get(pipeline_name)
            .ok_or_else(|| OsisError::PipelineNotFound {
                pipeline_name: pipeline_name.to_string(),
            })
    }

    pub fn delete_pipeline(&mut self, pipeline_name: &str) -> Result<(), OsisError> {
        if self.pipelines.remove(pipeline_name).is_none() {
            return Err(OsisError::PipelineNotFound {
                pipeline_name: pipeline_name.to_string(),
            });
        }
        Ok(())
    }

    pub fn list_pipelines(&self) -> Vec<&Pipeline> {
        self.pipelines.values().collect()
    }

    pub fn start_pipeline(&mut self, pipeline_name: &str) -> Result<&Pipeline, OsisError> {
        let pipeline =
            self.pipelines
                .get_mut(pipeline_name)
                .ok_or_else(|| OsisError::PipelineNotFound {
                    pipeline_name: pipeline_name.to_string(),
                })?;
        pipeline.status = "ACTIVE".to_string();
        pipeline.last_updated_at = Utc::now();
        Ok(pipeline)
    }

    pub fn stop_pipeline(&mut self, pipeline_name: &str) -> Result<&Pipeline, OsisError> {
        let pipeline =
            self.pipelines
                .get_mut(pipeline_name)
                .ok_or_else(|| OsisError::PipelineNotFound {
                    pipeline_name: pipeline_name.to_string(),
                })?;
        pipeline.status = "STOPPED".to_string();
        pipeline.last_updated_at = Utc::now();
        Ok(pipeline)
    }

    pub fn update_pipeline(
        &mut self,
        pipeline_name: &str,
        min_units: Option<i32>,
        max_units: Option<i32>,
        pipeline_configuration_body: Option<&str>,
    ) -> Result<&Pipeline, OsisError> {
        let pipeline =
            self.pipelines
                .get_mut(pipeline_name)
                .ok_or_else(|| OsisError::PipelineNotFound {
                    pipeline_name: pipeline_name.to_string(),
                })?;
        if let Some(v) = min_units {
            pipeline.min_units = v;
        }
        if let Some(v) = max_units {
            pipeline.max_units = v;
        }
        if let Some(v) = pipeline_configuration_body {
            pipeline.pipeline_configuration_body = v.to_string();
        }
        pipeline.last_updated_at = Utc::now();
        Ok(pipeline)
    }

    pub fn tag_resource(
        &mut self,
        arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), OsisError> {
        let pipeline =
            self.find_pipeline_by_arn_mut(arn)
                .ok_or_else(|| OsisError::ResourceNotFound {
                    arn: arn.to_string(),
                })?;
        pipeline.tags.extend(tags);
        Ok(())
    }

    pub fn untag_resource(&mut self, arn: &str, tag_keys: &[String]) -> Result<(), OsisError> {
        let pipeline =
            self.find_pipeline_by_arn_mut(arn)
                .ok_or_else(|| OsisError::ResourceNotFound {
                    arn: arn.to_string(),
                })?;
        for key in tag_keys {
            pipeline.tags.remove(key);
        }
        Ok(())
    }

    pub fn list_tags_for_resource(&self, arn: &str) -> Result<&HashMap<String, String>, OsisError> {
        let pipeline =
            self.find_pipeline_by_arn(arn)
                .ok_or_else(|| OsisError::ResourceNotFound {
                    arn: arn.to_string(),
                })?;
        Ok(&pipeline.tags)
    }

    fn find_pipeline_by_arn(&self, arn: &str) -> Option<&Pipeline> {
        self.pipelines.values().find(|p| p.pipeline_arn == arn)
    }

    fn find_pipeline_by_arn_mut(&mut self, arn: &str) -> Option<&mut Pipeline> {
        self.pipelines.values_mut().find(|p| p.pipeline_arn == arn)
    }
}
