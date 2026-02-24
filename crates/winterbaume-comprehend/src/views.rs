//! Serde-compatible view types for Comprehend state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::ComprehendService;
use crate::state::ComprehendState;
use crate::types::JobType;

/// Serializable view of the entire Comprehend state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ComprehendStateView {
    #[serde(default)]
    pub document_classifiers: HashMap<String, DocumentClassifierView>,
    #[serde(default)]
    pub entity_recognizers: HashMap<String, EntityRecognizerView>,
    #[serde(default)]
    pub endpoints: HashMap<String, EndpointView>,
    #[serde(default)]
    pub flywheels: HashMap<String, FlywheelView>,
    #[serde(default)]
    pub jobs: HashMap<String, ComprehendJobView>,
    #[serde(default)]
    pub resource_policies: HashMap<String, ResourcePolicyView>,
    #[serde(default)]
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentClassifierView {
    pub arn: String,
    pub name: String,
    pub language_code: String,
    pub data_access_role_arn: String,
    pub input_data_config_s3_uri: String,
    pub status: String,
    pub submit_time: f64,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityRecognizerView {
    pub arn: String,
    pub name: String,
    pub language_code: String,
    pub data_access_role_arn: String,
    pub input_data_config_s3_uri: String,
    #[serde(default)]
    pub entity_types: Vec<String>,
    pub status: String,
    pub submit_time: f64,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointView {
    pub arn: String,
    pub name: String,
    pub model_arn: String,
    pub desired_model_arn: String,
    pub desired_inference_units: i32,
    pub current_inference_units: i32,
    pub status: String,
    pub creation_time: f64,
    pub last_modified_time: f64,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlywheelView {
    pub arn: String,
    pub name: String,
    pub data_access_role_arn: String,
    pub data_lake_s3_uri: String,
    pub active_model_arn: String,
    pub model_type: String,
    pub status: String,
    pub creation_time: f64,
    pub last_modified_time: f64,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComprehendJobView {
    pub job_id: String,
    pub job_arn: String,
    pub job_name: Option<String>,
    pub job_status: String,
    pub submit_time: f64,
    pub data_access_role_arn: String,
    pub input_s3_uri: String,
    pub output_s3_uri: String,
    pub language_code: Option<String>,
    /// Job type as a string.
    pub job_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcePolicyView {
    pub policy: String,
    pub revision_id: String,
    pub creation_time: f64,
    pub last_modified_time: f64,
}

fn job_type_to_str(jt: &JobType) -> &'static str {
    match jt {
        JobType::DocumentClassification => "DocumentClassification",
        JobType::DominantLanguageDetection => "DominantLanguageDetection",
        JobType::EntitiesDetection => "EntitiesDetection",
        JobType::EventsDetection => "EventsDetection",
        JobType::KeyPhrasesDetection => "KeyPhrasesDetection",
        JobType::PiiEntitiesDetection => "PiiEntitiesDetection",
        JobType::SentimentDetection => "SentimentDetection",
        JobType::TargetedSentimentDetection => "TargetedSentimentDetection",
        JobType::TopicsDetection => "TopicsDetection",
    }
}

fn job_type_from_str(s: &str) -> JobType {
    match s {
        "DocumentClassification" => JobType::DocumentClassification,
        "DominantLanguageDetection" => JobType::DominantLanguageDetection,
        "EntitiesDetection" => JobType::EntitiesDetection,
        "EventsDetection" => JobType::EventsDetection,
        "KeyPhrasesDetection" => JobType::KeyPhrasesDetection,
        "PiiEntitiesDetection" => JobType::PiiEntitiesDetection,
        "SentimentDetection" => JobType::SentimentDetection,
        "TargetedSentimentDetection" => JobType::TargetedSentimentDetection,
        "TopicsDetection" => JobType::TopicsDetection,
        _ => JobType::EntitiesDetection,
    }
}

// --- From internal types to view types ---

impl From<&ComprehendState> for ComprehendStateView {
    fn from(state: &ComprehendState) -> Self {
        ComprehendStateView {
            document_classifiers: state
                .document_classifiers
                .iter()
                .map(|(k, dc)| {
                    (
                        k.clone(),
                        DocumentClassifierView {
                            arn: dc.arn.clone(),
                            name: dc.name.clone(),
                            language_code: dc.language_code.clone(),
                            data_access_role_arn: dc.data_access_role_arn.clone(),
                            input_data_config_s3_uri: dc.input_data_config_s3_uri.clone(),
                            status: dc.status.clone(),
                            submit_time: dc.submit_time,
                            tags: dc.tags.clone(),
                        },
                    )
                })
                .collect(),
            entity_recognizers: state
                .entity_recognizers
                .iter()
                .map(|(k, er)| {
                    (
                        k.clone(),
                        EntityRecognizerView {
                            arn: er.arn.clone(),
                            name: er.name.clone(),
                            language_code: er.language_code.clone(),
                            data_access_role_arn: er.data_access_role_arn.clone(),
                            input_data_config_s3_uri: er.input_data_config_s3_uri.clone(),
                            entity_types: er.entity_types.clone(),
                            status: er.status.clone(),
                            submit_time: er.submit_time,
                            tags: er.tags.clone(),
                        },
                    )
                })
                .collect(),
            endpoints: state
                .endpoints
                .iter()
                .map(|(k, ep)| {
                    (
                        k.clone(),
                        EndpointView {
                            arn: ep.arn.clone(),
                            name: ep.name.clone(),
                            model_arn: ep.model_arn.clone(),
                            desired_model_arn: ep.desired_model_arn.clone(),
                            desired_inference_units: ep.desired_inference_units,
                            current_inference_units: ep.current_inference_units,
                            status: ep.status.clone(),
                            creation_time: ep.creation_time,
                            last_modified_time: ep.last_modified_time,
                            tags: ep.tags.clone(),
                        },
                    )
                })
                .collect(),
            flywheels: state
                .flywheels
                .iter()
                .map(|(k, fw)| {
                    (
                        k.clone(),
                        FlywheelView {
                            arn: fw.arn.clone(),
                            name: fw.name.clone(),
                            data_access_role_arn: fw.data_access_role_arn.clone(),
                            data_lake_s3_uri: fw.data_lake_s3_uri.clone(),
                            active_model_arn: fw.active_model_arn.clone(),
                            model_type: fw.model_type.clone(),
                            status: fw.status.clone(),
                            creation_time: fw.creation_time,
                            last_modified_time: fw.last_modified_time,
                            tags: fw.tags.clone(),
                        },
                    )
                })
                .collect(),
            jobs: state
                .jobs
                .iter()
                .map(|(k, j)| {
                    (
                        k.clone(),
                        ComprehendJobView {
                            job_id: j.job_id.clone(),
                            job_arn: j.job_arn.clone(),
                            job_name: j.job_name.clone(),
                            job_status: j.job_status.clone(),
                            submit_time: j.submit_time,
                            data_access_role_arn: j.data_access_role_arn.clone(),
                            input_s3_uri: j.input_s3_uri.clone(),
                            output_s3_uri: j.output_s3_uri.clone(),
                            language_code: j.language_code.clone(),
                            job_type: job_type_to_str(&j.job_type).to_string(),
                        },
                    )
                })
                .collect(),
            resource_policies: state
                .resource_policies
                .iter()
                .map(|(k, rp)| {
                    (
                        k.clone(),
                        ResourcePolicyView {
                            policy: rp.policy.clone(),
                            revision_id: rp.revision_id.clone(),
                            creation_time: rp.creation_time,
                            last_modified_time: rp.last_modified_time,
                        },
                    )
                })
                .collect(),
            tags: state.tags.clone(),
        }
    }
}

// --- From view types to internal types ---

impl From<ComprehendStateView> for ComprehendState {
    fn from(view: ComprehendStateView) -> Self {
        ComprehendState {
            document_classifiers: view
                .document_classifiers
                .into_iter()
                .map(|(k, dc)| {
                    (
                        k,
                        crate::types::DocumentClassifier {
                            arn: dc.arn,
                            name: dc.name,
                            language_code: dc.language_code,
                            data_access_role_arn: dc.data_access_role_arn,
                            input_data_config_s3_uri: dc.input_data_config_s3_uri,
                            status: dc.status,
                            submit_time: dc.submit_time,
                            tags: dc.tags,
                        },
                    )
                })
                .collect(),
            entity_recognizers: view
                .entity_recognizers
                .into_iter()
                .map(|(k, er)| {
                    (
                        k,
                        crate::types::EntityRecognizer {
                            arn: er.arn,
                            name: er.name,
                            language_code: er.language_code,
                            data_access_role_arn: er.data_access_role_arn,
                            input_data_config_s3_uri: er.input_data_config_s3_uri,
                            entity_types: er.entity_types,
                            status: er.status,
                            submit_time: er.submit_time,
                            tags: er.tags,
                        },
                    )
                })
                .collect(),
            endpoints: view
                .endpoints
                .into_iter()
                .map(|(k, ep)| {
                    (
                        k,
                        crate::types::Endpoint {
                            arn: ep.arn,
                            name: ep.name,
                            model_arn: ep.model_arn,
                            desired_model_arn: ep.desired_model_arn,
                            desired_inference_units: ep.desired_inference_units,
                            current_inference_units: ep.current_inference_units,
                            status: ep.status,
                            creation_time: ep.creation_time,
                            last_modified_time: ep.last_modified_time,
                            tags: ep.tags,
                        },
                    )
                })
                .collect(),
            flywheels: view
                .flywheels
                .into_iter()
                .map(|(k, fw)| {
                    (
                        k,
                        crate::types::Flywheel {
                            arn: fw.arn,
                            name: fw.name,
                            data_access_role_arn: fw.data_access_role_arn,
                            data_lake_s3_uri: fw.data_lake_s3_uri,
                            active_model_arn: fw.active_model_arn,
                            model_type: fw.model_type,
                            status: fw.status,
                            creation_time: fw.creation_time,
                            last_modified_time: fw.last_modified_time,
                            tags: fw.tags,
                        },
                    )
                })
                .collect(),
            jobs: view
                .jobs
                .into_iter()
                .map(|(k, j)| {
                    (
                        k,
                        crate::types::ComprehendJob {
                            job_id: j.job_id,
                            job_arn: j.job_arn,
                            job_name: j.job_name,
                            job_status: j.job_status,
                            submit_time: j.submit_time,
                            data_access_role_arn: j.data_access_role_arn,
                            input_s3_uri: j.input_s3_uri,
                            output_s3_uri: j.output_s3_uri,
                            language_code: j.language_code,
                            job_type: job_type_from_str(&j.job_type),
                        },
                    )
                })
                .collect(),
            resource_policies: view
                .resource_policies
                .into_iter()
                .map(|(k, rp)| {
                    (
                        k,
                        crate::types::ResourcePolicy {
                            policy: rp.policy,
                            revision_id: rp.revision_id,
                            creation_time: rp.creation_time,
                            last_modified_time: rp.last_modified_time,
                        },
                    )
                })
                .collect(),
            tags: view.tags,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for ComprehendService {
    type StateView = ComprehendStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        ComprehendStateView::from(&*guard)
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            *guard = ComprehendState::from(view);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            let merged = ComprehendState::from(view);
            for (k, v) in merged.document_classifiers {
                guard.document_classifiers.insert(k, v);
            }
            for (k, v) in merged.entity_recognizers {
                guard.entity_recognizers.insert(k, v);
            }
            for (k, v) in merged.endpoints {
                guard.endpoints.insert(k, v);
            }
            for (k, v) in merged.flywheels {
                guard.flywheels.insert(k, v);
            }
            for (k, v) in merged.jobs {
                guard.jobs.insert(k, v);
            }
            for (k, v) in merged.resource_policies {
                guard.resource_policies.insert(k, v);
            }
            for (k, v) in merged.tags {
                guard.tags.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
