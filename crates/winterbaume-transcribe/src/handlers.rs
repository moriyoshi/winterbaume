use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, StatefulService,
    default_account_id, json_error_response,
};

use crate::state::{TranscribeError, TranscribeState};
use crate::views::TranscribeStateView;
use crate::wire;

pub struct TranscribeService {
    pub(crate) state: Arc<BackendState<TranscribeState>>,
    pub(crate) notifier: StateChangeNotifier<TranscribeStateView>,
}

impl TranscribeService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for TranscribeService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for TranscribeService {
    fn service_name(&self) -> &str {
        "transcribe"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://transcribe\..*\.amazonaws\.com",
            r"https?://transcribe\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl TranscribeService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();

        let action = request
            .headers
            .get("x-amz-target")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.split('.').next_back())
            .map(|s| s.to_string());

        let action = match action {
            Some(a) => a,
            None => {
                return transcribe_error_response(&TranscribeError::MissingAction);
            }
        };

        // Validate the body is well-formed JSON up-front; the typed deserialisers in
        // `wire` re-parse the bytes per operation.
        if !request.body.is_empty() && serde_json::from_slice::<Value>(&request.body).is_err() {
            return transcribe_error_response(&TranscribeError::InvalidJson);
        }
        let body_bytes: &[u8] = &request.body;

        let state = self.state.get(account_id, &region);

        let response = match action.as_str() {
            "CreateVocabulary" => self.handle_create_vocabulary(&state, body_bytes).await,
            "GetVocabulary" => self.handle_get_vocabulary(&state, body_bytes).await,
            "DeleteVocabulary" => self.handle_delete_vocabulary(&state, body_bytes).await,
            "ListVocabularies" => self.handle_list_vocabularies(&state, body_bytes).await,
            // --- Unimplemented operations (auto-generated stubs) ---
            "CreateCallAnalyticsCategory" => json_error_response(
                501,
                "NotImplementedError",
                "CreateCallAnalyticsCategory is not yet implemented in winterbaume-transcribe",
            ),
            "CreateLanguageModel" => json_error_response(
                501,
                "NotImplementedError",
                "CreateLanguageModel is not yet implemented in winterbaume-transcribe",
            ),
            "CreateMedicalVocabulary" => {
                self.handle_create_medical_vocabulary(&state, body_bytes)
                    .await
            }
            "CreateVocabularyFilter" => json_error_response(
                501,
                "NotImplementedError",
                "CreateVocabularyFilter is not yet implemented in winterbaume-transcribe",
            ),
            "DeleteCallAnalyticsCategory" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteCallAnalyticsCategory is not yet implemented in winterbaume-transcribe",
            ),
            "DeleteCallAnalyticsJob" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteCallAnalyticsJob is not yet implemented in winterbaume-transcribe",
            ),
            "DeleteLanguageModel" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteLanguageModel is not yet implemented in winterbaume-transcribe",
            ),
            "DeleteMedicalScribeJob" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteMedicalScribeJob is not yet implemented in winterbaume-transcribe",
            ),
            "DeleteMedicalTranscriptionJob" => {
                self.handle_delete_medical_transcription_job(&state, body_bytes)
                    .await
            }
            "DeleteMedicalVocabulary" => {
                self.handle_delete_medical_vocabulary(&state, body_bytes)
                    .await
            }
            "DeleteTranscriptionJob" => {
                self.handle_delete_transcription_job(&state, body_bytes)
                    .await
            }
            "DeleteVocabularyFilter" => json_error_response(
                501,
                "NotImplementedError",
                "DeleteVocabularyFilter is not yet implemented in winterbaume-transcribe",
            ),
            "DescribeLanguageModel" => json_error_response(
                501,
                "NotImplementedError",
                "DescribeLanguageModel is not yet implemented in winterbaume-transcribe",
            ),
            "GetCallAnalyticsCategory" => json_error_response(
                501,
                "NotImplementedError",
                "GetCallAnalyticsCategory is not yet implemented in winterbaume-transcribe",
            ),
            "GetCallAnalyticsJob" => json_error_response(
                501,
                "NotImplementedError",
                "GetCallAnalyticsJob is not yet implemented in winterbaume-transcribe",
            ),
            "GetMedicalScribeJob" => json_error_response(
                501,
                "NotImplementedError",
                "GetMedicalScribeJob is not yet implemented in winterbaume-transcribe",
            ),
            "GetMedicalTranscriptionJob" => {
                self.handle_get_medical_transcription_job(&state, body_bytes)
                    .await
            }
            "GetMedicalVocabulary" => self.handle_get_medical_vocabulary(&state, body_bytes).await,
            "GetTranscriptionJob" => self.handle_get_transcription_job(&state, body_bytes).await,
            "GetVocabularyFilter" => json_error_response(
                501,
                "NotImplementedError",
                "GetVocabularyFilter is not yet implemented in winterbaume-transcribe",
            ),
            "ListCallAnalyticsCategories" => json_error_response(
                501,
                "NotImplementedError",
                "ListCallAnalyticsCategories is not yet implemented in winterbaume-transcribe",
            ),
            "ListCallAnalyticsJobs" => json_error_response(
                501,
                "NotImplementedError",
                "ListCallAnalyticsJobs is not yet implemented in winterbaume-transcribe",
            ),
            "ListLanguageModels" => json_error_response(
                501,
                "NotImplementedError",
                "ListLanguageModels is not yet implemented in winterbaume-transcribe",
            ),
            "ListMedicalScribeJobs" => json_error_response(
                501,
                "NotImplementedError",
                "ListMedicalScribeJobs is not yet implemented in winterbaume-transcribe",
            ),
            "ListMedicalTranscriptionJobs" => {
                self.handle_list_medical_transcription_jobs(&state, body_bytes)
                    .await
            }
            "ListMedicalVocabularies" => {
                self.handle_list_medical_vocabularies(&state, body_bytes)
                    .await
            }
            "ListTagsForResource" => json_error_response(
                501,
                "NotImplementedError",
                "ListTagsForResource is not yet implemented in winterbaume-transcribe",
            ),
            "ListTranscriptionJobs" => {
                self.handle_list_transcription_jobs(&state, body_bytes)
                    .await
            }
            "ListVocabularyFilters" => json_error_response(
                501,
                "NotImplementedError",
                "ListVocabularyFilters is not yet implemented in winterbaume-transcribe",
            ),
            "StartCallAnalyticsJob" => json_error_response(
                501,
                "NotImplementedError",
                "StartCallAnalyticsJob is not yet implemented in winterbaume-transcribe",
            ),
            "StartMedicalScribeJob" => json_error_response(
                501,
                "NotImplementedError",
                "StartMedicalScribeJob is not yet implemented in winterbaume-transcribe",
            ),
            "StartMedicalTranscriptionJob" => {
                self.handle_start_medical_transcription_job(&state, body_bytes)
                    .await
            }
            "StartTranscriptionJob" => {
                self.handle_start_transcription_job(&state, body_bytes)
                    .await
            }
            "TagResource" => json_error_response(
                501,
                "NotImplementedError",
                "TagResource is not yet implemented in winterbaume-transcribe",
            ),
            "UntagResource" => json_error_response(
                501,
                "NotImplementedError",
                "UntagResource is not yet implemented in winterbaume-transcribe",
            ),
            "UpdateCallAnalyticsCategory" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateCallAnalyticsCategory is not yet implemented in winterbaume-transcribe",
            ),
            "UpdateMedicalVocabulary" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateMedicalVocabulary is not yet implemented in winterbaume-transcribe",
            ),
            "UpdateVocabulary" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateVocabulary is not yet implemented in winterbaume-transcribe",
            ),
            "UpdateVocabularyFilter" => json_error_response(
                501,
                "NotImplementedError",
                "UpdateVocabularyFilter is not yet implemented in winterbaume-transcribe",
            ),
            _ => transcribe_error_response(&TranscribeError::InvalidAction {
                action: action.clone(),
            }),
        };
        if response.status / 100 == 2 {
            self.notify_state_changed(account_id, &region).await;
        }
        response
    }

    async fn handle_create_vocabulary(
        &self,
        state: &Arc<tokio::sync::RwLock<TranscribeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_vocabulary_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.vocabulary_name.is_empty() {
            return transcribe_error_response(&TranscribeError::MissingField {
                field: "VocabularyName".to_string(),
            });
        }
        if input.language_code.is_empty() {
            return transcribe_error_response(&TranscribeError::MissingField {
                field: "LanguageCode".to_string(),
            });
        }

        let now = chrono::Utc::now().timestamp() as f64;

        let mut state = state.write().await;
        match state.create_vocabulary(
            &input.vocabulary_name,
            &input.language_code,
            input.phrases,
            input.vocabulary_file_uri,
            now,
        ) {
            Ok(vocab) => {
                let resp = wire::CreateVocabularyResponse {
                    vocabulary_name: Some(vocab.vocabulary_name.clone()),
                    language_code: Some(vocab.language_code.clone()),
                    vocabulary_state: Some(vocab.vocabulary_state.clone()),
                    last_modified_time: Some(vocab.last_modified_time),
                    failure_reason: None,
                };
                wire::serialize_create_vocabulary_response(&resp)
            }
            Err(e) => transcribe_error_response(&e),
        }
    }

    async fn handle_get_vocabulary(
        &self,
        state: &Arc<tokio::sync::RwLock<TranscribeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_vocabulary_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.vocabulary_name.is_empty() {
            return transcribe_error_response(&TranscribeError::MissingField {
                field: "VocabularyName".to_string(),
            });
        }

        let state = state.read().await;
        match state.get_vocabulary(&input.vocabulary_name) {
            Ok(vocab) => {
                let resp = wire::GetVocabularyResponse {
                    vocabulary_name: Some(vocab.vocabulary_name.clone()),
                    language_code: Some(vocab.language_code.clone()),
                    vocabulary_state: Some(vocab.vocabulary_state.clone()),
                    last_modified_time: Some(vocab.last_modified_time),
                    failure_reason: vocab.failure_reason.clone(),
                    download_uri: vocab.download_uri.clone(),
                };
                wire::serialize_get_vocabulary_response(&resp)
            }
            Err(e) => transcribe_error_response(&e),
        }
    }

    async fn handle_delete_vocabulary(
        &self,
        state: &Arc<tokio::sync::RwLock<TranscribeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_vocabulary_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.vocabulary_name.is_empty() {
            return transcribe_error_response(&TranscribeError::MissingField {
                field: "VocabularyName".to_string(),
            });
        }

        let mut state = state.write().await;
        match state.delete_vocabulary(&input.vocabulary_name) {
            Ok(()) => wire::serialize_delete_vocabulary_response(),
            Err(e) => transcribe_error_response(&e),
        }
    }

    async fn handle_list_vocabularies(
        &self,
        state: &Arc<tokio::sync::RwLock<TranscribeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_vocabularies_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let state_equals = input.state_equals.as_deref();
        let name_contains = input.name_contains.as_deref();

        let state = state.read().await;
        let vocabs = state.list_vocabularies(state_equals, name_contains);

        let entries: Vec<wire::VocabularyInfo> = vocabs
            .iter()
            .map(|v| wire::VocabularyInfo {
                vocabulary_name: Some(v.vocabulary_name.clone()),
                language_code: Some(v.language_code.clone()),
                vocabulary_state: Some(v.vocabulary_state.clone()),
                last_modified_time: Some(v.last_modified_time),
            })
            .collect();

        let resp = wire::ListVocabulariesResponse {
            vocabularies: Some(entries),
            status: state_equals.map(|s| s.to_string()),
            next_token: None,
        };

        wire::serialize_list_vocabularies_response(&resp)
    }

    // --- StartTranscriptionJob ---

    async fn handle_start_transcription_job(
        &self,
        state: &Arc<tokio::sync::RwLock<TranscribeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_start_transcription_job_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.transcription_job_name.is_empty() {
            return transcribe_error_response(&TranscribeError::MissingField {
                field: "TranscriptionJobName".to_string(),
            });
        }
        let language_code = match input.language_code.as_deref() {
            Some(lc) if !lc.is_empty() => lc,
            _ => {
                return transcribe_error_response(&TranscribeError::MissingField {
                    field: "LanguageCode".to_string(),
                });
            }
        };
        let media_uri = match input.media.media_file_uri.as_deref() {
            Some(u) if !u.is_empty() => u,
            _ => {
                return transcribe_error_response(&TranscribeError::MissingField {
                    field: "Media.MediaFileUri".to_string(),
                });
            }
        };
        let now = chrono::Utc::now().timestamp() as f64;

        let mut state = state.write().await;
        match state.start_transcription_job(
            &input.transcription_job_name,
            language_code,
            media_uri,
            input.media_format,
            input.media_sample_rate_hertz,
            input.output_bucket_name,
            now,
        ) {
            Ok(job) => {
                let resp = wire::StartTranscriptionJobResponse {
                    transcription_job: Some(wire::TranscriptionJob {
                        transcription_job_name: Some(job.transcription_job_name.clone()),
                        transcription_job_status: Some(job.transcription_job_status.clone()),
                        language_code: Some(job.language_code.clone()),
                        media: Some(wire::Media {
                            media_file_uri: Some(job.media_uri.clone()),
                            ..Default::default()
                        }),
                        creation_time: Some(job.creation_time),
                        start_time: job.start_time,
                        completion_time: job.completion_time,
                        transcript: job
                            .transcript_file_uri
                            .as_ref()
                            .map(|uri| wire::Transcript {
                                transcript_file_uri: Some(uri.clone()),
                                ..Default::default()
                            }),
                        ..Default::default()
                    }),
                };
                wire::serialize_start_transcription_job_response(&resp)
            }
            Err(e) => transcribe_error_response(&e),
        }
    }

    // --- GetTranscriptionJob ---

    async fn handle_get_transcription_job(
        &self,
        state: &Arc<tokio::sync::RwLock<TranscribeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_transcription_job_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.transcription_job_name.is_empty() {
            return transcribe_error_response(&TranscribeError::MissingField {
                field: "TranscriptionJobName".to_string(),
            });
        }

        let state = state.read().await;
        match state.get_transcription_job(&input.transcription_job_name) {
            Ok(job) => {
                let resp = wire::GetTranscriptionJobResponse {
                    transcription_job: Some(wire::TranscriptionJob {
                        transcription_job_name: Some(job.transcription_job_name.clone()),
                        transcription_job_status: Some(job.transcription_job_status.clone()),
                        language_code: Some(job.language_code.clone()),
                        media: Some(wire::Media {
                            media_file_uri: Some(job.media_uri.clone()),
                            ..Default::default()
                        }),
                        creation_time: Some(job.creation_time),
                        start_time: job.start_time,
                        completion_time: job.completion_time,
                        transcript: job
                            .transcript_file_uri
                            .as_ref()
                            .map(|uri| wire::Transcript {
                                transcript_file_uri: Some(uri.clone()),
                                ..Default::default()
                            }),
                        ..Default::default()
                    }),
                };
                wire::serialize_get_transcription_job_response(&resp)
            }
            Err(e) => transcribe_error_response(&e),
        }
    }

    // --- DeleteTranscriptionJob ---

    async fn handle_delete_transcription_job(
        &self,
        state: &Arc<tokio::sync::RwLock<TranscribeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_transcription_job_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.transcription_job_name.is_empty() {
            return transcribe_error_response(&TranscribeError::MissingField {
                field: "TranscriptionJobName".to_string(),
            });
        }

        let mut state = state.write().await;
        match state.delete_transcription_job(&input.transcription_job_name) {
            Ok(()) => wire::serialize_delete_transcription_job_response(),
            Err(e) => transcribe_error_response(&e),
        }
    }

    // --- ListTranscriptionJobs ---

    async fn handle_list_transcription_jobs(
        &self,
        state: &Arc<tokio::sync::RwLock<TranscribeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_transcription_jobs_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let status_equals = input.status.as_deref();
        let job_name_contains = input.job_name_contains.as_deref();

        let state = state.read().await;
        let jobs = state.list_transcription_jobs(status_equals, job_name_contains);

        let summaries: Vec<wire::TranscriptionJobSummary> = jobs
            .iter()
            .map(|j| wire::TranscriptionJobSummary {
                transcription_job_name: Some(j.transcription_job_name.clone()),
                transcription_job_status: Some(j.transcription_job_status.clone()),
                language_code: Some(j.language_code.clone()),
                creation_time: Some(j.creation_time),
                start_time: j.start_time,
                completion_time: j.completion_time,
                ..Default::default()
            })
            .collect();

        let resp = wire::ListTranscriptionJobsResponse {
            transcription_job_summaries: Some(summaries),
            status: status_equals.map(|s| s.to_string()),
            next_token: None,
        };
        wire::serialize_list_transcription_jobs_response(&resp)
    }

    // --- StartMedicalTranscriptionJob ---

    async fn handle_start_medical_transcription_job(
        &self,
        state: &Arc<tokio::sync::RwLock<TranscribeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_start_medical_transcription_job_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.medical_transcription_job_name.is_empty() {
            return transcribe_error_response(&TranscribeError::MissingField {
                field: "MedicalTranscriptionJobName".to_string(),
            });
        }
        if input.language_code.is_empty() {
            return transcribe_error_response(&TranscribeError::MissingField {
                field: "LanguageCode".to_string(),
            });
        }
        let media_uri = match input.media.media_file_uri.as_deref() {
            Some(u) if !u.is_empty() => u,
            _ => {
                return transcribe_error_response(&TranscribeError::MissingField {
                    field: "Media.MediaFileUri".to_string(),
                });
            }
        };
        if input.output_bucket_name.is_empty() {
            return transcribe_error_response(&TranscribeError::MissingField {
                field: "OutputBucketName".to_string(),
            });
        }
        if input.specialty.is_empty() {
            return transcribe_error_response(&TranscribeError::MissingField {
                field: "Specialty".to_string(),
            });
        }
        if input.r#type.is_empty() {
            return transcribe_error_response(&TranscribeError::MissingField {
                field: "Type".to_string(),
            });
        }
        let now = chrono::Utc::now().timestamp() as f64;

        let mut state = state.write().await;
        match state.start_medical_transcription_job(
            &input.medical_transcription_job_name,
            &input.language_code,
            media_uri,
            input.media_format,
            input.media_sample_rate_hertz,
            &input.output_bucket_name,
            &input.specialty,
            &input.r#type,
            now,
        ) {
            Ok(job) => {
                let resp = wire::StartMedicalTranscriptionJobResponse {
                    medical_transcription_job: Some(wire::MedicalTranscriptionJob {
                        medical_transcription_job_name: Some(
                            job.medical_transcription_job_name.clone(),
                        ),
                        transcription_job_status: Some(job.transcription_job_status.clone()),
                        language_code: Some(job.language_code.clone()),
                        media: Some(wire::Media {
                            media_file_uri: Some(job.media_uri.clone()),
                            ..Default::default()
                        }),
                        specialty: Some(job.specialty.clone()),
                        r#type: Some(job.r#type.clone()),
                        creation_time: Some(job.creation_time),
                        start_time: job.start_time,
                        completion_time: job.completion_time,
                        transcript: job.transcript_file_uri.as_ref().map(|uri| {
                            wire::MedicalTranscript {
                                transcript_file_uri: Some(uri.clone()),
                            }
                        }),
                        ..Default::default()
                    }),
                };
                wire::serialize_start_medical_transcription_job_response(&resp)
            }
            Err(e) => transcribe_error_response(&e),
        }
    }

    // --- GetMedicalTranscriptionJob ---

    async fn handle_get_medical_transcription_job(
        &self,
        state: &Arc<tokio::sync::RwLock<TranscribeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_medical_transcription_job_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.medical_transcription_job_name.is_empty() {
            return transcribe_error_response(&TranscribeError::MissingField {
                field: "MedicalTranscriptionJobName".to_string(),
            });
        }

        let state = state.read().await;
        match state.get_medical_transcription_job(&input.medical_transcription_job_name) {
            Ok(job) => {
                let resp = wire::GetMedicalTranscriptionJobResponse {
                    medical_transcription_job: Some(wire::MedicalTranscriptionJob {
                        medical_transcription_job_name: Some(
                            job.medical_transcription_job_name.clone(),
                        ),
                        transcription_job_status: Some(job.transcription_job_status.clone()),
                        language_code: Some(job.language_code.clone()),
                        media: Some(wire::Media {
                            media_file_uri: Some(job.media_uri.clone()),
                            ..Default::default()
                        }),
                        specialty: Some(job.specialty.clone()),
                        r#type: Some(job.r#type.clone()),
                        creation_time: Some(job.creation_time),
                        start_time: job.start_time,
                        completion_time: job.completion_time,
                        transcript: job.transcript_file_uri.as_ref().map(|uri| {
                            wire::MedicalTranscript {
                                transcript_file_uri: Some(uri.clone()),
                            }
                        }),
                        ..Default::default()
                    }),
                };
                wire::serialize_get_medical_transcription_job_response(&resp)
            }
            Err(e) => transcribe_error_response(&e),
        }
    }

    // --- DeleteMedicalTranscriptionJob ---

    async fn handle_delete_medical_transcription_job(
        &self,
        state: &Arc<tokio::sync::RwLock<TranscribeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_medical_transcription_job_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.medical_transcription_job_name.is_empty() {
            return transcribe_error_response(&TranscribeError::MissingField {
                field: "MedicalTranscriptionJobName".to_string(),
            });
        }

        let mut state = state.write().await;
        match state.delete_medical_transcription_job(&input.medical_transcription_job_name) {
            Ok(()) => wire::serialize_delete_medical_transcription_job_response(),
            Err(e) => transcribe_error_response(&e),
        }
    }

    // --- ListMedicalTranscriptionJobs ---

    async fn handle_list_medical_transcription_jobs(
        &self,
        state: &Arc<tokio::sync::RwLock<TranscribeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_medical_transcription_jobs_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let status_equals = input.status.as_deref();
        let job_name_contains = input.job_name_contains.as_deref();

        let state = state.read().await;
        let jobs = state.list_medical_transcription_jobs(status_equals, job_name_contains);

        let summaries: Vec<wire::MedicalTranscriptionJobSummary> = jobs
            .iter()
            .map(|j| wire::MedicalTranscriptionJobSummary {
                medical_transcription_job_name: Some(j.medical_transcription_job_name.clone()),
                transcription_job_status: Some(j.transcription_job_status.clone()),
                language_code: Some(j.language_code.clone()),
                specialty: Some(j.specialty.clone()),
                creation_time: Some(j.creation_time),
                start_time: j.start_time,
                completion_time: j.completion_time,
                ..Default::default()
            })
            .collect();

        let resp = wire::ListMedicalTranscriptionJobsResponse {
            medical_transcription_job_summaries: Some(summaries),
            status: status_equals.map(|s| s.to_string()),
            next_token: None,
        };
        wire::serialize_list_medical_transcription_jobs_response(&resp)
    }

    // --- CreateMedicalVocabulary ---

    async fn handle_create_medical_vocabulary(
        &self,
        state: &Arc<tokio::sync::RwLock<TranscribeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_create_medical_vocabulary_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.vocabulary_name.is_empty() {
            return transcribe_error_response(&TranscribeError::MissingField {
                field: "VocabularyName".to_string(),
            });
        }
        if input.language_code.is_empty() {
            return transcribe_error_response(&TranscribeError::MissingField {
                field: "LanguageCode".to_string(),
            });
        }
        if input.vocabulary_file_uri.is_empty() {
            return transcribe_error_response(&TranscribeError::MissingField {
                field: "VocabularyFileUri".to_string(),
            });
        }
        let now = chrono::Utc::now().timestamp() as f64;

        let mut state = state.write().await;
        match state.create_medical_vocabulary(
            &input.vocabulary_name,
            &input.language_code,
            &input.vocabulary_file_uri,
            now,
        ) {
            Ok(vocab) => {
                let resp = wire::CreateMedicalVocabularyResponse {
                    vocabulary_name: Some(vocab.vocabulary_name.clone()),
                    language_code: Some(vocab.language_code.clone()),
                    vocabulary_state: Some(vocab.vocabulary_state.clone()),
                    last_modified_time: Some(vocab.last_modified_time),
                    failure_reason: None,
                };
                wire::serialize_create_medical_vocabulary_response(&resp)
            }
            Err(e) => transcribe_error_response(&e),
        }
    }

    // --- GetMedicalVocabulary ---

    async fn handle_get_medical_vocabulary(
        &self,
        state: &Arc<tokio::sync::RwLock<TranscribeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_get_medical_vocabulary_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.vocabulary_name.is_empty() {
            return transcribe_error_response(&TranscribeError::MissingField {
                field: "VocabularyName".to_string(),
            });
        }

        let state = state.read().await;
        match state.get_medical_vocabulary(&input.vocabulary_name) {
            Ok(vocab) => {
                let resp = wire::GetMedicalVocabularyResponse {
                    vocabulary_name: Some(vocab.vocabulary_name.clone()),
                    language_code: Some(vocab.language_code.clone()),
                    vocabulary_state: Some(vocab.vocabulary_state.clone()),
                    last_modified_time: Some(vocab.last_modified_time),
                    failure_reason: vocab.failure_reason.clone(),
                    download_uri: vocab.download_uri.clone(),
                };
                wire::serialize_get_medical_vocabulary_response(&resp)
            }
            Err(e) => transcribe_error_response(&e),
        }
    }

    // --- DeleteMedicalVocabulary ---

    async fn handle_delete_medical_vocabulary(
        &self,
        state: &Arc<tokio::sync::RwLock<TranscribeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_delete_medical_vocabulary_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        if input.vocabulary_name.is_empty() {
            return transcribe_error_response(&TranscribeError::MissingField {
                field: "VocabularyName".to_string(),
            });
        }

        let mut state = state.write().await;
        match state.delete_medical_vocabulary(&input.vocabulary_name) {
            Ok(()) => wire::serialize_delete_medical_vocabulary_response(),
            Err(e) => transcribe_error_response(&e),
        }
    }

    // --- ListMedicalVocabularies ---

    async fn handle_list_medical_vocabularies(
        &self,
        state: &Arc<tokio::sync::RwLock<TranscribeState>>,
        body: &[u8],
    ) -> MockResponse {
        let input = match wire::deserialize_list_medical_vocabularies_request(body) {
            Ok(v) => v,
            Err(e) => return json_error_response(400, "ValidationException", &e),
        };
        let state_equals = input.state_equals.as_deref();
        let name_contains = input.name_contains.as_deref();

        let state = state.read().await;
        let vocabs = state.list_medical_vocabularies(state_equals, name_contains);

        let entries: Vec<wire::VocabularyInfo> = vocabs
            .iter()
            .map(|v| wire::VocabularyInfo {
                vocabulary_name: Some(v.vocabulary_name.clone()),
                language_code: Some(v.language_code.clone()),
                vocabulary_state: Some(v.vocabulary_state.clone()),
                last_modified_time: Some(v.last_modified_time),
            })
            .collect();

        let resp = wire::ListMedicalVocabulariesResponse {
            vocabularies: Some(entries),
            status: state_equals.map(|s| s.to_string()),
            next_token: None,
        };
        wire::serialize_list_medical_vocabularies_response(&resp)
    }
}

fn transcribe_error_response(err: &TranscribeError) -> MockResponse {
    let (status, error_type) = match err {
        TranscribeError::VocabularyAlreadyExists => (409, "ConflictException"),
        TranscribeError::VocabularyNotFound => (400, "NotFoundException"),
        TranscribeError::JobAlreadyExists => (409, "ConflictException"),
        TranscribeError::JobNotFound => (400, "NotFoundException"),
        TranscribeError::MissingAction => (400, "MissingAction"),
        TranscribeError::InvalidJson => (400, "SerializationException"),
        TranscribeError::MissingField { .. } => (400, "BadRequestException"),
        TranscribeError::InvalidAction { .. } => (400, "InvalidAction"),
    };
    let body = json!({
        "__type": error_type,
        "message": err.to_string(),
    });
    MockResponse::json(status, body.to_string())
}
