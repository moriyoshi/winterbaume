use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use http::header::HeaderName;
use serde_json::json;
use winterbaume_core::{
    BackendState, MockRequest, MockResponse, MockService, StateChangeNotifier, default_account_id,
    extract_path, extract_query_string, parse_query_string, percent_decode, rest_json_error,
};

use crate::state::{PollyError, PollyState};
use crate::views::PollyStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: HeaderName = HeaderName::from_static("x-amzn-errortype");

pub struct PollyService {
    pub(crate) state: Arc<BackendState<PollyState>>,
    pub(crate) notifier: StateChangeNotifier<PollyStateView>,
}

impl PollyService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for PollyService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for PollyService {
    fn service_name(&self) -> &str {
        "polly"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://polly\..*\.amazonaws\.com",
            r"https?://polly\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl PollyService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = default_account_id();
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let method = request.method.as_str();

        let segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();

        // Routes:
        // GET    /v1/voices           - DescribeVoices
        // PUT    /v1/lexicons/{name}  - PutLexicon
        // GET    /v1/lexicons/{name}  - GetLexicon
        // DELETE /v1/lexicons/{name}  - DeleteLexicon
        // GET    /v1/lexicons         - ListLexicons
        // POST   /v1/speech           - SynthesizeSpeech

        if segments.is_empty() || segments[0] != "v1" {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }

        let query_map: HashMap<String, String> =
            parse_query_string(extract_query_string(&request.uri));

        match (method, segments.get(1).copied(), segments.len()) {
            // GET /v1/voices
            ("GET", Some("voices"), 2) => {
                self.handle_describe_voices(&state, &request, &[], &query_map)
                    .await
            }
            // GET /v1/lexicons - ListLexicons
            ("GET", Some("lexicons"), 2) => {
                self.handle_list_lexicons(&state, &request, &[], &query_map)
                    .await
            }
            // PUT /v1/lexicons/{name} - PutLexicon
            ("PUT", Some("lexicons"), 3) => {
                let name = percent_decode(segments[2]);
                let labels: &[(&str, &str)] = &[("Name", name.as_str())];
                self.handle_put_lexicon(&state, &request, labels, &query_map)
                    .await
            }
            // GET /v1/lexicons/{name} - GetLexicon
            ("GET", Some("lexicons"), 3) => {
                let name = percent_decode(segments[2]);
                let labels: &[(&str, &str)] = &[("Name", name.as_str())];
                self.handle_get_lexicon(&state, &request, labels, &query_map)
                    .await
            }
            // DELETE /v1/lexicons/{name} - DeleteLexicon
            ("DELETE", Some("lexicons"), 3) => {
                let name = percent_decode(segments[2]);
                let labels: &[(&str, &str)] = &[("Name", name.as_str())];
                self.handle_delete_lexicon(&state, &request, labels, &query_map)
                    .await
            }
            // POST /v1/speech - SynthesizeSpeech
            ("POST", Some("speech"), 2) => {
                self.handle_synthesize_speech(&state, &request, &[], &query_map)
                    .await
            }
            // POST /v1/synthesisTasks - StartSpeechSynthesisTask
            ("POST", Some("synthesisTasks"), 2) => {
                self.handle_start_speech_synthesis_task(&state, &request, &[], &query_map)
                    .await
            }
            // GET /v1/synthesisTasks/{TaskId} - GetSpeechSynthesisTask
            ("GET", Some("synthesisTasks"), 3) => {
                let task_id = percent_decode(segments[2]);
                let labels: &[(&str, &str)] = &[("TaskId", task_id.as_str())];
                self.handle_get_speech_synthesis_task(&state, &request, labels, &query_map)
                    .await
            }
            // GET /v1/synthesisTasks - ListSpeechSynthesisTasks
            ("GET", Some("synthesisTasks"), 2) => {
                self.handle_list_speech_synthesis_tasks(&state, &request, &[], &query_map)
                    .await
            }

            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    async fn handle_describe_voices(
        &self,
        state: &Arc<tokio::sync::RwLock<PollyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_describe_voices_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.describe_voices(input.language_code.as_deref()) {
            Ok(voices) => wire::serialize_describe_voices_response(&wire::DescribeVoicesOutput {
                voices: Some(
                    voices
                        .iter()
                        .map(|v| wire::Voice {
                            id: Some(v.id.clone()),
                            name: Some(v.name.clone()),
                            gender: Some(v.gender.clone()),
                            language_code: Some(v.language_code.clone()),
                            language_name: Some(v.language_name.clone()),
                            supported_engines: Some(v.supported_engines.clone()),
                            ..Default::default()
                        })
                        .collect(),
                ),
                ..Default::default()
            }),
            Err(e) => polly_error_response(&e),
        }
    }

    async fn handle_put_lexicon(
        &self,
        state: &Arc<tokio::sync::RwLock<PollyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_put_lexicon_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "BadRequestException", "Invalid JSON body"),
        };
        if input.content.is_empty() {
            return rest_json_error(400, "InvalidParameterValue", "Content is required");
        }

        let mut state = state.write().await;
        match state.put_lexicon(&input.name, &input.content) {
            Ok(()) => wire::serialize_put_lexicon_response(&wire::PutLexiconOutput {}),
            Err(e) => polly_error_response(&e),
        }
    }

    async fn handle_get_lexicon(
        &self,
        state: &Arc<tokio::sync::RwLock<PollyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_get_lexicon_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let state = state.read().await;
        match state.get_lexicon(&input.name) {
            Ok(lexicon) => wire::serialize_get_lexicon_response(&wire::GetLexiconOutput {
                lexicon: Some(wire::Lexicon {
                    name: Some(lexicon.name.clone()),
                    content: Some(lexicon.content.clone()),
                }),
                lexicon_attributes: Some(wire::LexiconAttributes {
                    alphabet: Some("ipa".to_string()),
                    language_code: Some(lexicon.language_code.clone()),
                    last_modified: Some(lexicon.last_modified.timestamp() as f64),
                    lexemes_count: Some(lexicon.lexemes_count),
                    lexicon_arn: Some(format!(
                        "arn:aws:polly:us-east-1:123456789012:lexicon/{}",
                        lexicon.name
                    )),
                    size: Some(lexicon.size),
                }),
            }),
            Err(e) => polly_error_response(&e),
        }
    }

    async fn handle_delete_lexicon(
        &self,
        state: &Arc<tokio::sync::RwLock<PollyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input = match wire::deserialize_delete_lexicon_request(request, labels, query) {
            Ok(v) => v,
            Err(e) => return rest_json_error(400, "ValidationException", &e),
        };
        let mut state = state.write().await;
        match state.delete_lexicon(&input.name) {
            Ok(()) => wire::serialize_delete_lexicon_response(&wire::DeleteLexiconOutput {}),
            Err(e) => polly_error_response(&e),
        }
    }

    async fn handle_list_lexicons(
        &self,
        state: &Arc<tokio::sync::RwLock<PollyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) = wire::deserialize_list_lexicons_request(request, labels, query) {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let lexicons = state.list_lexicons();
        wire::serialize_list_lexicons_response(&wire::ListLexiconsOutput {
            lexicons: Some(
                lexicons
                    .iter()
                    .map(|l| wire::LexiconDescription {
                        name: Some(l.name.clone()),
                        attributes: Some(wire::LexiconAttributes {
                            alphabet: Some("ipa".to_string()),
                            language_code: Some(l.language_code.clone()),
                            last_modified: Some(l.last_modified.timestamp() as f64),
                            lexemes_count: Some(l.lexemes_count),
                            lexicon_arn: Some(format!(
                                "arn:aws:polly:us-east-1:123456789012:lexicon/{}",
                                l.name
                            )),
                            size: Some(l.size),
                        }),
                    })
                    .collect(),
            ),
            ..Default::default()
        })
    }

    async fn handle_start_speech_synthesis_task(
        &self,
        state: &Arc<tokio::sync::RwLock<PollyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_start_speech_synthesis_task_request(request, labels, query) {
                Ok(v) => v,
                Err(_) => return rest_json_error(400, "BadRequestException", "Invalid JSON body"),
            };
        if input.voice_id.is_empty() {
            return rest_json_error(400, "InvalidParameterValue", "VoiceId is required");
        }
        if input.text.is_empty() {
            return rest_json_error(400, "InvalidParameterValue", "Text is required");
        }
        if input.output_s3_bucket_name.is_empty() {
            return rest_json_error(
                400,
                "InvalidParameterValue",
                "OutputS3BucketName is required",
            );
        }
        let output_format = if input.output_format.is_empty() {
            "mp3"
        } else {
            input.output_format.as_str()
        };
        let output_s3_key_prefix = input.output_s3_key_prefix.as_deref().unwrap_or("");
        let engine = input.engine.as_deref().unwrap_or("standard");

        let mut state = state.write().await;
        let task = state.start_speech_synthesis_task(
            &input.voice_id,
            &input.text,
            output_format,
            &input.output_s3_bucket_name,
            output_s3_key_prefix,
            engine,
        );

        wire::serialize_start_speech_synthesis_task_response(
            &wire::StartSpeechSynthesisTaskOutput {
                synthesis_task: Some(wire::SynthesisTask {
                    task_id: Some(task.task_id.clone()),
                    task_status: Some(task.task_status.clone()),
                    output_uri: Some(task.output_uri.clone()),
                    voice_id: Some(task.voice_id.clone()),
                    creation_time: Some(task.creation_time.timestamp() as f64),
                    output_format: Some(task.output_format.clone()),
                    engine: Some(task.engine.clone()),
                    request_characters: Some(task.text.len() as i32),
                    ..Default::default()
                }),
            },
        )
    }

    async fn handle_get_speech_synthesis_task(
        &self,
        state: &Arc<tokio::sync::RwLock<PollyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_get_speech_synthesis_task_request(request, labels, query) {
                Ok(v) => v,
                Err(e) => return rest_json_error(400, "ValidationException", &e),
            };
        let state = state.read().await;
        match state.get_speech_synthesis_task(&input.task_id) {
            Ok(task) => wire::serialize_get_speech_synthesis_task_response(
                &wire::GetSpeechSynthesisTaskOutput {
                    synthesis_task: Some(wire::SynthesisTask {
                        task_id: Some(task.task_id.clone()),
                        task_status: Some(task.task_status.clone()),
                        output_uri: Some(task.output_uri.clone()),
                        voice_id: Some(task.voice_id.clone()),
                        creation_time: Some(task.creation_time.timestamp() as f64),
                        output_format: Some(task.output_format.clone()),
                        engine: Some(task.engine.clone()),
                        request_characters: Some(task.text.len() as i32),
                        ..Default::default()
                    }),
                },
            ),
            Err(e) => polly_error_response(&e),
        }
    }

    async fn handle_list_speech_synthesis_tasks(
        &self,
        state: &Arc<tokio::sync::RwLock<PollyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        if let Err(e) =
            wire::deserialize_list_speech_synthesis_tasks_request(request, labels, query)
        {
            return rest_json_error(400, "ValidationException", &e);
        }
        let state = state.read().await;
        let tasks = state.list_speech_synthesis_tasks();
        wire::serialize_list_speech_synthesis_tasks_response(
            &wire::ListSpeechSynthesisTasksOutput {
                synthesis_tasks: Some(
                    tasks
                        .iter()
                        .map(|t| wire::SynthesisTask {
                            task_id: Some(t.task_id.clone()),
                            task_status: Some(t.task_status.clone()),
                            output_uri: Some(t.output_uri.clone()),
                            voice_id: Some(t.voice_id.clone()),
                            creation_time: Some(t.creation_time.timestamp() as f64),
                            output_format: Some(t.output_format.clone()),
                            engine: Some(t.engine.clone()),
                            request_characters: Some(t.text.len() as i32),
                            ..Default::default()
                        })
                        .collect(),
                ),
                ..Default::default()
            },
        )
    }

    async fn handle_synthesize_speech(
        &self,
        state: &Arc<tokio::sync::RwLock<PollyState>>,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
    ) -> MockResponse {
        use crate::types::VOICES;

        let input = match wire::deserialize_synthesize_speech_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "BadRequestException", "Invalid JSON body"),
        };

        if input.text.is_empty() {
            return rest_json_error(400, "InvalidParameterValue", "Text is required");
        }
        if input.voice_id.is_empty() {
            return rest_json_error(400, "InvalidParameterValue", "VoiceId is required");
        }

        // Validate voice_id
        let voice_valid = VOICES
            .iter()
            .any(|(id, _, _, _, _, _)| *id == input.voice_id);
        if !voice_valid {
            return polly_error_response(&PollyError::UnknownVoiceId {
                voice_id: input.voice_id.clone(),
            });
        }

        let output_format = if input.output_format.is_empty() {
            "mp3"
        } else {
            input.output_format.as_str()
        };

        // Validate output format
        let valid_formats = ["mp3", "ogg_vorbis", "pcm", "json"];
        if !valid_formats.contains(&output_format) {
            return polly_error_response(&PollyError::InvalidOutputFormat {
                format: output_format.to_string(),
            });
        }

        // Validate text type if provided
        let text_type = input.text_type.as_deref().unwrap_or("text");
        let valid_text_types = ["text", "ssml"];
        if !valid_text_types.contains(&text_type) {
            return polly_error_response(&PollyError::InvalidTextType {
                text_type: text_type.to_string(),
            });
        }

        // Validate text length (max 3000 characters for plain text)
        if input.text.len() > 3000 {
            return polly_error_response(&PollyError::TextLengthExceeded);
        }

        // Validate sample rate if provided
        if let Some(sample_rate) = input.sample_rate.as_deref() {
            let valid_rates_pcm = ["8000", "16000"];
            let valid_rates_mp3 = ["8000", "16000", "22050", "24000"];
            let valid_rates_ogg = ["8000", "16000", "22050", "24000"];
            let is_valid_rate = match output_format {
                "pcm" => valid_rates_pcm.contains(&sample_rate),
                "mp3" => valid_rates_mp3.contains(&sample_rate),
                "ogg_vorbis" => valid_rates_ogg.contains(&sample_rate),
                _ => true,
            };
            if !is_valid_rate {
                return polly_error_response(&PollyError::InvalidSampleRate {
                    sample_rate: sample_rate.to_string(),
                });
            }
        }

        // Validate speech marks: not supported for non-json output
        if let Some(marks) = input.speech_mark_types.as_deref() {
            if !marks.is_empty() && output_format != "json" {
                return polly_error_response(&PollyError::MarksNotSupportedForFormat);
            }
        }

        // Validate lexicon names if provided
        if let Some(lexicon_names) = input.lexicon_names.as_deref() {
            let state = state.read().await;
            for lname in lexicon_names {
                if state.get_lexicon(lname).is_err() {
                    return polly_error_response(&PollyError::LexiconNotFound {
                        name: lname.clone(),
                    });
                }
            }
        }

        // Return a minimal audio placeholder
        let content_type = match output_format {
            "mp3" => "audio/mpeg",
            "ogg_vorbis" => "audio/ogg",
            "pcm" => "audio/pcm",
            "json" => "application/x-json-stream",
            _ => "audio/mpeg",
        };

        // Generate minimal valid audio bytes based on output format
        let audio_bytes: Vec<u8> = match output_format {
            "mp3" => {
                // Minimal valid MP3 frame: MPEG1, Layer3, 128kbps, 44100Hz, stereo
                // Frame header FF FB 90 00 followed by zero-filled data (417 bytes total)
                let mut frame = vec![0u8; 417];
                frame[0] = 0xFF;
                frame[1] = 0xFB;
                frame[2] = 0x90;
                frame[3] = 0x00;
                frame
            }
            "pcm" => {
                // Zero-filled PCM samples (silence): ~20ms of 16kHz 16-bit mono per character
                let byte_count = (input.text.len() * 320).min(32000);
                vec![0u8; byte_count]
            }
            "ogg_vorbis" => {
                // Minimal OGG page header (capture pattern "OggS" + structure fields)
                let mut page = vec![0u8; 27];
                page[0] = b'O';
                page[1] = b'g';
                page[2] = b'g';
                page[3] = b'S';
                // version = 0, header_type = 0x02 (beginning of stream)
                page[5] = 0x02;
                page
            }
            "json" => {
                // Generate NDJSON speech marks, one per word
                let mut marks = String::new();
                let mut time_ms: u64 = 0;
                let mut char_offset: usize = 0;
                for word in input.text.split_whitespace() {
                    let start = input.text[char_offset..].find(word).unwrap_or(0) + char_offset;
                    let end = start + word.len();
                    marks.push_str(&format!(
                        "{{\"time\":{},\"type\":\"word\",\"start\":{},\"end\":{},\"value\":\"{}\"}}\n",
                        time_ms, start, end, word
                    ));
                    time_ms += 200;
                    char_offset = end;
                }
                marks.into_bytes()
            }
            _ => vec![],
        };

        let mut resp = MockResponse::rest_json(200, "");
        resp.body = bytes::Bytes::from(audio_bytes);
        resp.headers
            .insert(http::header::CONTENT_TYPE, content_type.parse().unwrap());
        resp.headers.insert(
            HeaderName::from_static("x-amzn-requestcharacters"),
            input.text.len().to_string().parse().unwrap(),
        );
        resp
    }
}

fn polly_error_response(err: &PollyError) -> MockResponse {
    let (status, error_type) = match err {
        PollyError::UnknownLanguageCode { .. } => (400, "InvalidParameterValue"),
        PollyError::LexiconNameEmpty => (400, "InvalidParameterValue"),
        PollyError::LexiconNameTooLong => (400, "InvalidParameterValue"),
        PollyError::LexiconNameInvalidChars => (400, "InvalidParameterValue"),
        PollyError::LexiconNotFound { .. } => (404, "LexiconNotFoundException"),
        PollyError::SynthesisTaskNotFound { .. } => (404, "SynthesisTaskNotFoundException"),
        PollyError::UnknownVoiceId { .. } => (400, "InvalidParameterValue"),
        PollyError::InvalidOutputFormat { .. } => (400, "InvalidParameterValue"),
        PollyError::InvalidTextType { .. } => (400, "InvalidParameterValue"),
        PollyError::TextLengthExceeded => (400, "TextLengthExceededException"),
        PollyError::InvalidSampleRate { .. } => (400, "InvalidSampleRateException"),
        PollyError::MarksNotSupportedForFormat => (400, "MarksNotSupportedForFormatException"),
    };
    let body = json!({
        "message": err.to_string(),
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}
