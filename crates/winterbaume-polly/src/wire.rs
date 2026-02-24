//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-polly

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

#[allow(unused_imports)]
use http::header::HeaderName;
use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for restJson protocol.
pub fn serialize_delete_lexicon_response(result: &DeleteLexiconOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_describe_voices_response(result: &DescribeVoicesOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_lexicon_response(result: &GetLexiconOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_speech_synthesis_task_response(
    result: &GetSpeechSynthesisTaskOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_lexicons_response(result: &ListLexiconsOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_speech_synthesis_tasks_response(
    result: &ListSpeechSynthesisTasksOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_lexicon_response(result: &PutLexiconOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_speech_synthesis_stream_response(
    result: &StartSpeechSynthesisStreamOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = match &result.event_stream {
        Some(v) => serde_json::to_string(v).unwrap_or_else(|_| "{}".to_string()),
        None => String::new(),
    };
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_speech_synthesis_task_response(
    result: &StartSpeechSynthesisTaskOutput,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_synthesize_speech_response(result: &SynthesizeSpeechOutput) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    let resp = MockResponse::rest_json(status, body);
    // Header "content-type": set by caller from content_type field
    // Header "x-amzn-requestcharacters": set by caller from request_characters field
    resp
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_lexicon_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteLexiconInput, String> {
    let mut input = DeleteLexiconInput::default();
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_describe_voices_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DescribeVoicesInput, String> {
    let mut input = DescribeVoicesInput::default();
    if let Some(value) = query.get("Engine") {
        input.engine = Some(value.to_string());
    }
    if let Some(value) = query.get("IncludeAdditionalLanguageCodes") {
        input.include_additional_language_codes = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("LanguageCode") {
        input.language_code = Some(value.to_string());
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_lexicon_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetLexiconInput, String> {
    let mut input = GetLexiconInput::default();
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_speech_synthesis_task_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetSpeechSynthesisTaskInput, String> {
    let mut input = GetSpeechSynthesisTaskInput::default();
    for (name, value) in labels {
        match *name {
            "TaskId" => {
                input.task_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_lexicons_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListLexiconsInput, String> {
    let mut input = ListLexiconsInput::default();
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_speech_synthesis_tasks_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListSpeechSynthesisTasksInput, String> {
    let mut input = ListSpeechSynthesisTasksInput::default();
    if let Some(value) = query.get("MaxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("NextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("Status") {
        input.status = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_lexicon_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutLexiconInput, String> {
    let mut input = PutLexiconInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutLexiconInput>(&request.body)
            .map_err(|err| format!("failed to deserialize PutLexicon request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "Name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_speech_synthesis_stream_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartSpeechSynthesisStreamInput, String> {
    let mut input = StartSpeechSynthesisStreamInput::default();
    if !request.body.is_empty() {
        let payload =
            serde_json::from_slice::<StartSpeechSynthesisStreamActionStream>(&request.body)
                .map_err(|err| {
                    format!("failed to deserialize StartSpeechSynthesisStream payload: {err}")
                })?;
        input.action_stream = Some(payload);
    }
    if let Some(value) = request
        .headers
        .get("x-amzn-Engine")
        .and_then(|value| value.to_str().ok())
    {
        input.engine = value.to_string();
    }
    if let Some(value) = request
        .headers
        .get("x-amzn-LanguageCode")
        .and_then(|value| value.to_str().ok())
    {
        input.language_code = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amzn-LexiconNames")
        .and_then(|value| value.to_str().ok())
    {
        input.lexicon_names = Some(
            value
                .split(',')
                .filter(|item| !item.trim().is_empty())
                .map(|item| Ok(item.trim().to_string()))
                .collect::<Result<Vec<_>, String>>()?,
        );
    }
    if let Some(value) = request
        .headers
        .get("x-amzn-OutputFormat")
        .and_then(|value| value.to_str().ok())
    {
        input.output_format = value.to_string();
    }
    if let Some(value) = request
        .headers
        .get("x-amzn-SampleRate")
        .and_then(|value| value.to_str().ok())
    {
        input.sample_rate = Some(value.to_string());
    }
    if let Some(value) = request
        .headers
        .get("x-amzn-VoiceId")
        .and_then(|value| value.to_str().ok())
    {
        input.voice_id = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_speech_synthesis_task_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartSpeechSynthesisTaskInput, String> {
    let mut input = StartSpeechSynthesisTaskInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartSpeechSynthesisTaskInput>(&request.body).map_err(
            |err| format!("failed to deserialize StartSpeechSynthesisTask request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_synthesize_speech_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<SynthesizeSpeechInput, String> {
    let mut input = SynthesizeSpeechInput::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<SynthesizeSpeechInput>(&request.body)
            .map_err(|err| format!("failed to deserialize SynthesizeSpeech request: {err}"))?;
    }
    Ok(input)
}
