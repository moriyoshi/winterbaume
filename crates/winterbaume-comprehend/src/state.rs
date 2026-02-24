use std::collections::HashMap;

use chrono::Utc;
use uuid::Uuid;

use crate::types::*;

#[derive(Debug, Default)]
pub struct ComprehendState {
    pub document_classifiers: HashMap<String, DocumentClassifier>,
    pub entity_recognizers: HashMap<String, EntityRecognizer>,
    pub endpoints: HashMap<String, Endpoint>,
    pub flywheels: HashMap<String, Flywheel>,
    pub jobs: HashMap<String, ComprehendJob>,
    /// Resource ARN -> resource policy
    pub resource_policies: HashMap<String, ResourcePolicy>,
    /// Resource ARN -> tags
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, thiserror::Error)]
pub enum ComprehendError {
    #[error("Resource {resource} already exists.")]
    ResourceInUse { resource: String },

    #[error("Resource {resource} not found.")]
    ResourceNotFound { resource: String },

    #[error("Job {job_id} not found.")]
    JobNotFound { job_id: String },
}

fn now_epoch() -> f64 {
    Utc::now().timestamp() as f64
}

impl ComprehendState {
    // ---- Document Classifier ----

    pub fn create_document_classifier(
        &mut self,
        name: &str,
        language_code: &str,
        data_access_role_arn: &str,
        input_data_config_s3_uri: &str,
        version_name: Option<&str>,
        account_id: &str,
        region: &str,
        tags: &[(String, String)],
    ) -> Result<String, ComprehendError> {
        let arn = if let Some(ver) = version_name {
            format!("arn:aws:comprehend:{region}:{account_id}:document-classifier/{name}/{ver}")
        } else {
            format!("arn:aws:comprehend:{region}:{account_id}:document-classifier/{name}")
        };
        if self.document_classifiers.contains_key(&arn) {
            return Err(ComprehendError::ResourceInUse {
                resource: format!("Document classifier {name}"),
            });
        }
        let mut tag_map = HashMap::new();
        for (k, v) in tags {
            tag_map.insert(k.clone(), v.clone());
        }
        self.document_classifiers.insert(
            arn.clone(),
            DocumentClassifier {
                arn: arn.clone(),
                name: name.to_string(),
                language_code: language_code.to_string(),
                data_access_role_arn: data_access_role_arn.to_string(),
                input_data_config_s3_uri: input_data_config_s3_uri.to_string(),
                status: "TRAINING".to_string(),
                submit_time: now_epoch(),
                tags: tag_map.clone(),
            },
        );
        self.tags.insert(arn.clone(), tag_map);
        Ok(arn)
    }

    pub fn describe_document_classifier(
        &self,
        arn: &str,
    ) -> Result<&DocumentClassifier, ComprehendError> {
        self.document_classifiers
            .get(arn)
            .ok_or_else(|| ComprehendError::ResourceNotFound {
                resource: format!("Document classifier {arn}"),
            })
    }

    pub fn delete_document_classifier(&mut self, arn: &str) -> Result<(), ComprehendError> {
        if self.document_classifiers.remove(arn).is_none() {
            return Err(ComprehendError::ResourceNotFound {
                resource: format!("Document classifier {arn}"),
            });
        }
        self.tags.remove(arn);
        Ok(())
    }

    pub fn list_document_classifiers(&self) -> Vec<&DocumentClassifier> {
        self.document_classifiers.values().collect()
    }

    // ---- Entity Recognizer ----

    pub fn create_entity_recognizer(
        &mut self,
        name: &str,
        language_code: &str,
        data_access_role_arn: &str,
        input_data_config_s3_uri: &str,
        entity_types: Vec<String>,
        version_name: Option<&str>,
        account_id: &str,
        region: &str,
        tags: &[(String, String)],
    ) -> Result<String, ComprehendError> {
        let arn = if let Some(ver) = version_name {
            format!(
                "arn:aws:comprehend:{region}:{account_id}:entity-recognizer/{name}/version/{ver}"
            )
        } else {
            format!("arn:aws:comprehend:{region}:{account_id}:entity-recognizer/{name}")
        };
        if self.entity_recognizers.contains_key(&arn) {
            return Err(ComprehendError::ResourceInUse {
                resource: format!("Entity recognizer {name}"),
            });
        }
        let mut tag_map = HashMap::new();
        for (k, v) in tags {
            tag_map.insert(k.clone(), v.clone());
        }
        self.entity_recognizers.insert(
            arn.clone(),
            EntityRecognizer {
                arn: arn.clone(),
                name: name.to_string(),
                language_code: language_code.to_string(),
                data_access_role_arn: data_access_role_arn.to_string(),
                input_data_config_s3_uri: input_data_config_s3_uri.to_string(),
                entity_types,
                status: "TRAINED".to_string(),
                submit_time: now_epoch(),
                tags: tag_map.clone(),
            },
        );
        self.tags.insert(arn.clone(), tag_map);
        Ok(arn)
    }

    pub fn describe_entity_recognizer(
        &self,
        arn: &str,
    ) -> Result<&EntityRecognizer, ComprehendError> {
        self.entity_recognizers
            .get(arn)
            .ok_or_else(|| ComprehendError::ResourceNotFound {
                resource: format!("Entity recognizer {arn}"),
            })
    }

    pub fn delete_entity_recognizer(&mut self, arn: &str) -> Result<(), ComprehendError> {
        if self.entity_recognizers.remove(arn).is_none() {
            return Err(ComprehendError::ResourceNotFound {
                resource: format!("Entity recognizer {arn}"),
            });
        }
        self.tags.remove(arn);
        Ok(())
    }

    pub fn list_entity_recognizers(&self) -> Vec<&EntityRecognizer> {
        self.entity_recognizers.values().collect()
    }

    // ---- Endpoint ----

    pub fn create_endpoint(
        &mut self,
        name: &str,
        model_arn: &str,
        desired_inference_units: i32,
        account_id: &str,
        region: &str,
        tags: &[(String, String)],
    ) -> Result<(String, String), ComprehendError> {
        let arn =
            format!("arn:aws:comprehend:{region}:{account_id}:document-classifier-endpoint/{name}");
        if self.endpoints.contains_key(&arn) {
            return Err(ComprehendError::ResourceInUse {
                resource: format!("Endpoint {name}"),
            });
        }
        let now = now_epoch();
        let mut tag_map = HashMap::new();
        for (k, v) in tags {
            tag_map.insert(k.clone(), v.clone());
        }
        self.endpoints.insert(
            arn.clone(),
            Endpoint {
                arn: arn.clone(),
                name: name.to_string(),
                model_arn: model_arn.to_string(),
                desired_model_arn: model_arn.to_string(),
                desired_inference_units,
                current_inference_units: desired_inference_units,
                status: "IN_SERVICE".to_string(),
                creation_time: now,
                last_modified_time: now,
                tags: tag_map.clone(),
            },
        );
        self.tags.insert(arn.clone(), tag_map);
        Ok((arn, model_arn.to_string()))
    }

    pub fn describe_endpoint(&self, arn: &str) -> Result<&Endpoint, ComprehendError> {
        self.endpoints
            .get(arn)
            .ok_or_else(|| ComprehendError::ResourceNotFound {
                resource: format!("Endpoint {arn}"),
            })
    }

    pub fn delete_endpoint(&mut self, arn: &str) -> Result<(), ComprehendError> {
        if self.endpoints.remove(arn).is_none() {
            return Err(ComprehendError::ResourceNotFound {
                resource: format!("Endpoint {arn}"),
            });
        }
        self.tags.remove(arn);
        Ok(())
    }

    pub fn update_endpoint(
        &mut self,
        arn: &str,
        desired_model_arn: Option<&str>,
        desired_inference_units: Option<i32>,
    ) -> Result<Option<String>, ComprehendError> {
        let endpoint =
            self.endpoints
                .get_mut(arn)
                .ok_or_else(|| ComprehendError::ResourceNotFound {
                    resource: format!("Endpoint {arn}"),
                })?;
        if let Some(m) = desired_model_arn {
            endpoint.desired_model_arn = m.to_string();
        }
        if let Some(u) = desired_inference_units {
            endpoint.desired_inference_units = u;
        }
        endpoint.last_modified_time = now_epoch();
        Ok(desired_model_arn.map(|s| s.to_string()))
    }

    pub fn list_endpoints(&self) -> Vec<&Endpoint> {
        self.endpoints.values().collect()
    }

    // ---- Flywheel ----

    pub fn create_flywheel(
        &mut self,
        name: &str,
        active_model_arn: &str,
        data_access_role_arn: &str,
        data_lake_s3_uri: &str,
        model_type: &str,
        account_id: &str,
        region: &str,
        tags: &[(String, String)],
    ) -> Result<(String, String), ComprehendError> {
        let arn = format!("arn:aws:comprehend:{region}:{account_id}:flywheel/{name}");
        if self.flywheels.contains_key(&arn) {
            return Err(ComprehendError::ResourceInUse {
                resource: format!("Flywheel {name}"),
            });
        }
        let now = now_epoch();
        let mut tag_map = HashMap::new();
        for (k, v) in tags {
            tag_map.insert(k.clone(), v.clone());
        }
        self.flywheels.insert(
            arn.clone(),
            Flywheel {
                arn: arn.clone(),
                name: name.to_string(),
                data_access_role_arn: data_access_role_arn.to_string(),
                data_lake_s3_uri: data_lake_s3_uri.to_string(),
                active_model_arn: active_model_arn.to_string(),
                model_type: model_type.to_string(),
                status: "ACTIVE".to_string(),
                creation_time: now,
                last_modified_time: now,
                tags: tag_map.clone(),
            },
        );
        self.tags.insert(arn.clone(), tag_map);
        Ok((arn, active_model_arn.to_string()))
    }

    pub fn describe_flywheel(&self, arn: &str) -> Result<&Flywheel, ComprehendError> {
        self.flywheels
            .get(arn)
            .ok_or_else(|| ComprehendError::ResourceNotFound {
                resource: format!("Flywheel {arn}"),
            })
    }

    pub fn delete_flywheel(&mut self, arn: &str) -> Result<(), ComprehendError> {
        if self.flywheels.remove(arn).is_none() {
            return Err(ComprehendError::ResourceNotFound {
                resource: format!("Flywheel {arn}"),
            });
        }
        self.tags.remove(arn);
        Ok(())
    }

    pub fn list_flywheels(&self) -> Vec<&Flywheel> {
        self.flywheels.values().collect()
    }

    pub fn start_flywheel_iteration(&self, flywheel_arn: &str) -> Result<String, ComprehendError> {
        if !self.flywheels.contains_key(flywheel_arn) {
            return Err(ComprehendError::ResourceNotFound {
                resource: format!("Flywheel {flywheel_arn}"),
            });
        }
        Ok(Uuid::new_v4().to_string())
    }

    // ---- Jobs ----

    pub fn start_job(
        &mut self,
        job_name: Option<&str>,
        data_access_role_arn: &str,
        input_s3_uri: &str,
        output_s3_uri: &str,
        language_code: Option<&str>,
        job_type: JobType,
        account_id: &str,
        region: &str,
    ) -> (String, String) {
        let job_id = Uuid::new_v4().to_string();
        let type_str = match job_type {
            JobType::DocumentClassification => "document-classification-job",
            JobType::DominantLanguageDetection => "dominant-language-detection-job",
            JobType::EntitiesDetection => "entities-detection-job",
            JobType::EventsDetection => "events-detection-job",
            JobType::KeyPhrasesDetection => "key-phrases-detection-job",
            JobType::PiiEntitiesDetection => "pii-entities-detection-job",
            JobType::SentimentDetection => "sentiment-detection-job",
            JobType::TargetedSentimentDetection => "targeted-sentiment-detection-job",
            JobType::TopicsDetection => "topics-detection-job",
        };
        let job_arn = format!("arn:aws:comprehend:{region}:{account_id}:{type_str}/{job_id}");
        self.jobs.insert(
            job_id.clone(),
            ComprehendJob {
                job_id: job_id.clone(),
                job_arn: job_arn.clone(),
                job_name: job_name.map(|s| s.to_string()),
                job_status: "SUBMITTED".to_string(),
                submit_time: now_epoch(),
                data_access_role_arn: data_access_role_arn.to_string(),
                input_s3_uri: input_s3_uri.to_string(),
                output_s3_uri: output_s3_uri.to_string(),
                language_code: language_code.map(|s| s.to_string()),
                job_type,
            },
        );
        (job_id, job_arn)
    }

    pub fn describe_job(
        &self,
        job_id: &str,
        expected_type: &JobType,
    ) -> Result<&ComprehendJob, ComprehendError> {
        let job = self
            .jobs
            .get(job_id)
            .ok_or_else(|| ComprehendError::JobNotFound {
                job_id: job_id.to_string(),
            })?;
        if &job.job_type != expected_type {
            return Err(ComprehendError::JobNotFound {
                job_id: job_id.to_string(),
            });
        }
        Ok(job)
    }

    pub fn stop_job(
        &mut self,
        job_id: &str,
        expected_type: &JobType,
    ) -> Result<&ComprehendJob, ComprehendError> {
        let job = self
            .jobs
            .get_mut(job_id)
            .ok_or_else(|| ComprehendError::JobNotFound {
                job_id: job_id.to_string(),
            })?;
        if &job.job_type != expected_type {
            return Err(ComprehendError::JobNotFound {
                job_id: job_id.to_string(),
            });
        }
        job.job_status = "STOP_REQUESTED".to_string();
        Ok(job)
    }

    pub fn list_jobs(&self, job_type: &JobType) -> Vec<&ComprehendJob> {
        self.jobs
            .values()
            .filter(|j| &j.job_type == job_type)
            .collect()
    }

    // ---- Resource Policy ----

    pub fn put_resource_policy(
        &mut self,
        resource_arn: &str,
        policy: &str,
    ) -> Result<String, ComprehendError> {
        let revision_id = Uuid::new_v4().to_string();
        let now = now_epoch();
        let entry = self
            .resource_policies
            .entry(resource_arn.to_string())
            .or_insert_with(|| ResourcePolicy {
                policy: String::new(),
                revision_id: String::new(),
                creation_time: now,
                last_modified_time: now,
            });
        entry.policy = policy.to_string();
        entry.revision_id = revision_id.clone();
        entry.last_modified_time = now;
        Ok(revision_id)
    }

    pub fn describe_resource_policy(
        &self,
        resource_arn: &str,
    ) -> Result<&ResourcePolicy, ComprehendError> {
        self.resource_policies
            .get(resource_arn)
            .ok_or_else(|| ComprehendError::ResourceNotFound {
                resource: format!("Resource policy for {resource_arn}"),
            })
    }

    pub fn delete_resource_policy(&mut self, resource_arn: &str) -> Result<(), ComprehendError> {
        if self.resource_policies.remove(resource_arn).is_none() {
            return Err(ComprehendError::ResourceNotFound {
                resource: format!("Resource policy for {resource_arn}"),
            });
        }
        Ok(())
    }

    // ---- Tags ----

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: &[(String, String)],
    ) -> Result<(), ComprehendError> {
        let tag_map = self.tags.entry(resource_arn.to_string()).or_default();
        for (k, v) in tags {
            tag_map.insert(k.clone(), v.clone());
        }
        Ok(())
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), ComprehendError> {
        if let Some(tag_map) = self.tags.get_mut(resource_arn) {
            for k in tag_keys {
                tag_map.remove(k);
            }
        }
        Ok(())
    }

    pub fn list_tags_for_resource(&self, resource_arn: &str) -> Vec<(String, String)> {
        self.tags
            .get(resource_arn)
            .map(|m| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
            .unwrap_or_default()
    }

    // ---- Stop Training ----

    pub fn stop_training_document_classifier(&mut self, arn: &str) -> Result<(), ComprehendError> {
        let dc = self.document_classifiers.get_mut(arn).ok_or_else(|| {
            ComprehendError::ResourceNotFound {
                resource: format!("Document classifier {arn}"),
            }
        })?;
        dc.status = "STOP_REQUESTED".to_string();
        Ok(())
    }

    pub fn stop_training_entity_recognizer(&mut self, arn: &str) -> Result<(), ComprehendError> {
        // Verify the recognizer exists, but don't change its status.
        // In moto, stop_training_entity_recognizer does not alter the status:
        // if the recognizer is already TRAINED, it stays TRAINED.
        self.entity_recognizers
            .get(arn)
            .ok_or_else(|| ComprehendError::ResourceNotFound {
                resource: format!("Entity recognizer {arn}"),
            })?;
        Ok(())
    }
}
