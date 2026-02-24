//! Serde-compatible view types for Transcribe state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::TranscribeService;
use crate::state::TranscribeState;
use crate::types::{
    MedicalTranscriptionJobData, MedicalVocabularyData, TranscriptionJobData, Vocabulary,
};

/// Serializable view of the entire Transcribe state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TranscribeStateView {
    #[serde(default)]
    pub vocabularies: HashMap<String, VocabularyView>,
    #[serde(default)]
    pub transcription_jobs: HashMap<String, TranscriptionJobView>,
    #[serde(default)]
    pub medical_transcription_jobs: HashMap<String, MedicalTranscriptionJobView>,
    #[serde(default)]
    pub medical_vocabularies: HashMap<String, MedicalVocabularyView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VocabularyView {
    pub vocabulary_name: String,
    pub language_code: String,
    pub vocabulary_state: String,
    pub last_modified_time: f64,
    pub phrases: Option<Vec<String>>,
    pub vocabulary_file_uri: Option<String>,
    pub failure_reason: Option<String>,
    pub download_uri: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionJobView {
    pub transcription_job_name: String,
    pub transcription_job_status: String,
    pub language_code: String,
    pub media_uri: String,
    pub media_format: Option<String>,
    pub media_sample_rate_hertz: Option<i32>,
    pub output_bucket_name: Option<String>,
    pub creation_time: f64,
    pub start_time: Option<f64>,
    pub completion_time: Option<f64>,
    pub failure_reason: Option<String>,
    pub transcript_file_uri: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicalTranscriptionJobView {
    pub medical_transcription_job_name: String,
    pub transcription_job_status: String,
    pub language_code: String,
    pub media_uri: String,
    pub media_format: Option<String>,
    pub media_sample_rate_hertz: Option<i32>,
    pub output_bucket_name: String,
    pub specialty: String,
    pub job_type: String,
    pub creation_time: f64,
    pub start_time: Option<f64>,
    pub completion_time: Option<f64>,
    pub failure_reason: Option<String>,
    pub transcript_file_uri: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicalVocabularyView {
    pub vocabulary_name: String,
    pub language_code: String,
    pub vocabulary_state: String,
    pub vocabulary_file_uri: String,
    pub last_modified_time: f64,
    pub failure_reason: Option<String>,
    pub download_uri: Option<String>,
}

// --- From internal types to view types ---

impl From<&TranscribeState> for TranscribeStateView {
    fn from(state: &TranscribeState) -> Self {
        TranscribeStateView {
            vocabularies: state
                .vocabularies
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        VocabularyView {
                            vocabulary_name: v.vocabulary_name.clone(),
                            language_code: v.language_code.clone(),
                            vocabulary_state: v.vocabulary_state.clone(),
                            last_modified_time: v.last_modified_time,
                            phrases: v.phrases.clone(),
                            vocabulary_file_uri: v.vocabulary_file_uri.clone(),
                            failure_reason: v.failure_reason.clone(),
                            download_uri: v.download_uri.clone(),
                        },
                    )
                })
                .collect(),
            transcription_jobs: state
                .transcription_jobs
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        TranscriptionJobView {
                            transcription_job_name: v.transcription_job_name.clone(),
                            transcription_job_status: v.transcription_job_status.clone(),
                            language_code: v.language_code.clone(),
                            media_uri: v.media_uri.clone(),
                            media_format: v.media_format.clone(),
                            media_sample_rate_hertz: v.media_sample_rate_hertz,
                            output_bucket_name: v.output_bucket_name.clone(),
                            creation_time: v.creation_time,
                            start_time: v.start_time,
                            completion_time: v.completion_time,
                            failure_reason: v.failure_reason.clone(),
                            transcript_file_uri: v.transcript_file_uri.clone(),
                        },
                    )
                })
                .collect(),
            medical_transcription_jobs: state
                .medical_transcription_jobs
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        MedicalTranscriptionJobView {
                            medical_transcription_job_name: v
                                .medical_transcription_job_name
                                .clone(),
                            transcription_job_status: v.transcription_job_status.clone(),
                            language_code: v.language_code.clone(),
                            media_uri: v.media_uri.clone(),
                            media_format: v.media_format.clone(),
                            media_sample_rate_hertz: v.media_sample_rate_hertz,
                            output_bucket_name: v.output_bucket_name.clone(),
                            specialty: v.specialty.clone(),
                            job_type: v.r#type.clone(),
                            creation_time: v.creation_time,
                            start_time: v.start_time,
                            completion_time: v.completion_time,
                            failure_reason: v.failure_reason.clone(),
                            transcript_file_uri: v.transcript_file_uri.clone(),
                        },
                    )
                })
                .collect(),
            medical_vocabularies: state
                .medical_vocabularies
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        MedicalVocabularyView {
                            vocabulary_name: v.vocabulary_name.clone(),
                            language_code: v.language_code.clone(),
                            vocabulary_state: v.vocabulary_state.clone(),
                            vocabulary_file_uri: v.vocabulary_file_uri.clone(),
                            last_modified_time: v.last_modified_time,
                            failure_reason: v.failure_reason.clone(),
                            download_uri: v.download_uri.clone(),
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- From view types to internal types ---

impl From<TranscribeStateView> for TranscribeState {
    fn from(view: TranscribeStateView) -> Self {
        TranscribeState {
            vocabularies: view
                .vocabularies
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        Vocabulary {
                            vocabulary_name: v.vocabulary_name,
                            language_code: v.language_code,
                            vocabulary_state: v.vocabulary_state,
                            last_modified_time: v.last_modified_time,
                            phrases: v.phrases,
                            vocabulary_file_uri: v.vocabulary_file_uri,
                            failure_reason: v.failure_reason,
                            download_uri: v.download_uri,
                        },
                    )
                })
                .collect(),
            transcription_jobs: view
                .transcription_jobs
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        TranscriptionJobData {
                            transcription_job_name: v.transcription_job_name,
                            transcription_job_status: v.transcription_job_status,
                            language_code: v.language_code,
                            media_uri: v.media_uri,
                            media_format: v.media_format,
                            media_sample_rate_hertz: v.media_sample_rate_hertz,
                            output_bucket_name: v.output_bucket_name,
                            creation_time: v.creation_time,
                            start_time: v.start_time,
                            completion_time: v.completion_time,
                            failure_reason: v.failure_reason,
                            transcript_file_uri: v.transcript_file_uri,
                        },
                    )
                })
                .collect(),
            medical_transcription_jobs: view
                .medical_transcription_jobs
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        MedicalTranscriptionJobData {
                            medical_transcription_job_name: v.medical_transcription_job_name,
                            transcription_job_status: v.transcription_job_status,
                            language_code: v.language_code,
                            media_uri: v.media_uri,
                            media_format: v.media_format,
                            media_sample_rate_hertz: v.media_sample_rate_hertz,
                            output_bucket_name: v.output_bucket_name,
                            specialty: v.specialty,
                            r#type: v.job_type,
                            creation_time: v.creation_time,
                            start_time: v.start_time,
                            completion_time: v.completion_time,
                            failure_reason: v.failure_reason,
                            transcript_file_uri: v.transcript_file_uri,
                        },
                    )
                })
                .collect(),
            medical_vocabularies: view
                .medical_vocabularies
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        MedicalVocabularyData {
                            vocabulary_name: v.vocabulary_name,
                            language_code: v.language_code,
                            vocabulary_state: v.vocabulary_state,
                            vocabulary_file_uri: v.vocabulary_file_uri,
                            last_modified_time: v.last_modified_time,
                            failure_reason: v.failure_reason,
                            download_uri: v.download_uri,
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for TranscribeService {
    type StateView = TranscribeStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        TranscribeStateView::from(&*guard)
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
            *guard = TranscribeState::from(view);
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
            let new_state = TranscribeState::from(view);
            guard.vocabularies.extend(new_state.vocabularies);
            guard
                .transcription_jobs
                .extend(new_state.transcription_jobs);
            guard
                .medical_transcription_jobs
                .extend(new_state.medical_transcription_jobs);
            guard
                .medical_vocabularies
                .extend(new_state.medical_vocabularies);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
