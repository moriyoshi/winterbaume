//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-polly

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLexiconInput {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLexiconOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeVoicesInput {
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "IncludeAdditionalLanguageCodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_additional_language_codes: Option<bool>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeVoicesOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Voices")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voices: Option<Vec<Voice>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Voice {
    #[serde(rename = "AdditionalLanguageCodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_language_codes: Option<Vec<String>>,
    #[serde(rename = "Gender")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "LanguageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SupportedEngines")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_engines: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLexiconInput {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLexiconOutput {
    #[serde(rename = "Lexicon")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lexicon: Option<Lexicon>,
    #[serde(rename = "LexiconAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lexicon_attributes: Option<LexiconAttributes>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Lexicon {
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LexiconAttributes {
    #[serde(rename = "Alphabet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alphabet: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "LastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<f64>,
    #[serde(rename = "LexemesCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lexemes_count: Option<i32>,
    #[serde(rename = "LexiconArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lexicon_arn: Option<String>,
    #[serde(rename = "Size")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSpeechSynthesisTaskInput {
    #[serde(rename = "TaskId")]
    #[serde(default)]
    pub task_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSpeechSynthesisTaskOutput {
    #[serde(rename = "SynthesisTask")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synthesis_task: Option<SynthesisTask>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SynthesisTask {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "LexiconNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lexicon_names: Option<Vec<String>>,
    #[serde(rename = "OutputFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    #[serde(rename = "OutputUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_uri: Option<String>,
    #[serde(rename = "RequestCharacters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_characters: Option<i32>,
    #[serde(rename = "SampleRate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<String>,
    #[serde(rename = "SnsTopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
    #[serde(rename = "SpeechMarkTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_mark_types: Option<Vec<String>>,
    #[serde(rename = "TaskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(rename = "TaskStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_status: Option<String>,
    #[serde(rename = "TaskStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_status_reason: Option<String>,
    #[serde(rename = "TextType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_type: Option<String>,
    #[serde(rename = "VoiceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLexiconsInput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLexiconsOutput {
    #[serde(rename = "Lexicons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lexicons: Option<Vec<LexiconDescription>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LexiconDescription {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<LexiconAttributes>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSpeechSynthesisTasksInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSpeechSynthesisTasksOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SynthesisTasks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synthesis_tasks: Option<Vec<SynthesisTask>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutLexiconInput {
    #[serde(rename = "Content")]
    #[serde(default)]
    pub content: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutLexiconOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSpeechSynthesisStreamInput {
    #[serde(rename = "ActionStream")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_stream: Option<StartSpeechSynthesisStreamActionStream>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    pub engine: String,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "LexiconNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lexicon_names: Option<Vec<String>>,
    #[serde(rename = "OutputFormat")]
    #[serde(default)]
    pub output_format: String,
    #[serde(rename = "SampleRate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<String>,
    #[serde(rename = "VoiceId")]
    #[serde(default)]
    pub voice_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSpeechSynthesisStreamActionStream {
    #[serde(rename = "CloseStreamEvent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_stream_event: Option<CloseStreamEvent>,
    #[serde(rename = "TextEvent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_event: Option<TextEvent>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloseStreamEvent {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TextEvent {
    #[serde(rename = "FlushStreamConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flush_stream_configuration: Option<FlushStreamConfiguration>,
    #[serde(rename = "Text")]
    #[serde(default)]
    pub text: String,
    #[serde(rename = "TextType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlushStreamConfiguration {
    #[serde(rename = "Force")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSpeechSynthesisStreamOutput {
    #[serde(rename = "EventStream")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_stream: Option<StartSpeechSynthesisStreamEventStream>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSpeechSynthesisStreamEventStream {
    #[serde(rename = "AudioEvent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_event: Option<AudioEvent>,
    #[serde(rename = "ServiceFailureException")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_failure_exception: Option<ServiceFailureException>,
    #[serde(rename = "ServiceQuotaExceededException")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_quota_exceeded_exception: Option<ServiceQuotaExceededException>,
    #[serde(rename = "StreamClosedEvent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_closed_event: Option<StreamClosedEvent>,
    #[serde(rename = "ThrottlingException")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttling_exception: Option<ThrottlingException>,
    #[serde(rename = "ValidationException")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_exception: Option<ValidationException>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AudioEvent {
    #[serde(rename = "AudioChunk")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_chunk: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceFailureException {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceQuotaExceededException {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "quotaCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota_code: Option<String>,
    #[serde(rename = "serviceCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StreamClosedEvent {
    #[serde(rename = "RequestCharacters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_characters: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThrottlingException {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "throttlingReasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttling_reasons: Option<Vec<ThrottlingReason>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThrottlingReason {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidationException {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<ValidationExceptionField>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidationExceptionField {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSpeechSynthesisTaskInput {
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "LexiconNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lexicon_names: Option<Vec<String>>,
    #[serde(rename = "OutputFormat")]
    #[serde(default)]
    pub output_format: String,
    #[serde(rename = "OutputS3BucketName")]
    #[serde(default)]
    pub output_s3_bucket_name: String,
    #[serde(rename = "OutputS3KeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_key_prefix: Option<String>,
    #[serde(rename = "SampleRate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<String>,
    #[serde(rename = "SnsTopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
    #[serde(rename = "SpeechMarkTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_mark_types: Option<Vec<String>>,
    #[serde(rename = "Text")]
    #[serde(default)]
    pub text: String,
    #[serde(rename = "TextType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_type: Option<String>,
    #[serde(rename = "VoiceId")]
    #[serde(default)]
    pub voice_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSpeechSynthesisTaskOutput {
    #[serde(rename = "SynthesisTask")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synthesis_task: Option<SynthesisTask>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SynthesizeSpeechInput {
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "LexiconNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lexicon_names: Option<Vec<String>>,
    #[serde(rename = "OutputFormat")]
    #[serde(default)]
    pub output_format: String,
    #[serde(rename = "SampleRate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<String>,
    #[serde(rename = "SpeechMarkTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_mark_types: Option<Vec<String>>,
    #[serde(rename = "Text")]
    #[serde(default)]
    pub text: String,
    #[serde(rename = "TextType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_type: Option<String>,
    #[serde(rename = "VoiceId")]
    #[serde(default)]
    pub voice_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SynthesizeSpeechOutput {
    #[serde(rename = "AudioStream")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_stream: Option<String>,
    #[serde(rename = "ContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "RequestCharacters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_characters: Option<i32>,
}
