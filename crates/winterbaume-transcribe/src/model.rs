//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-transcribe

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCallAnalyticsCategoryRequest {
    #[serde(rename = "CategoryName")]
    #[serde(default)]
    pub category_name: String,
    #[serde(rename = "InputType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_type: Option<String>,
    #[serde(rename = "Rules")]
    #[serde(default)]
    pub rules: Vec<Rule>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Rule {
    #[serde(rename = "InterruptionFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interruption_filter: Option<InterruptionFilter>,
    #[serde(rename = "NonTalkTimeFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_talk_time_filter: Option<NonTalkTimeFilter>,
    #[serde(rename = "SentimentFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiment_filter: Option<SentimentFilter>,
    #[serde(rename = "TranscriptFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript_filter: Option<TranscriptFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InterruptionFilter {
    #[serde(rename = "AbsoluteTimeRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_time_range: Option<AbsoluteTimeRange>,
    #[serde(rename = "Negate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negate: Option<bool>,
    #[serde(rename = "ParticipantRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_role: Option<String>,
    #[serde(rename = "RelativeTimeRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_time_range: Option<RelativeTimeRange>,
    #[serde(rename = "Threshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AbsoluteTimeRange {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    #[serde(rename = "First")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    #[serde(rename = "Last")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RelativeTimeRange {
    #[serde(rename = "EndPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_percentage: Option<i32>,
    #[serde(rename = "First")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i32>,
    #[serde(rename = "Last")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i32>,
    #[serde(rename = "StartPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_percentage: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NonTalkTimeFilter {
    #[serde(rename = "AbsoluteTimeRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_time_range: Option<AbsoluteTimeRange>,
    #[serde(rename = "Negate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negate: Option<bool>,
    #[serde(rename = "RelativeTimeRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_time_range: Option<RelativeTimeRange>,
    #[serde(rename = "Threshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SentimentFilter {
    #[serde(rename = "AbsoluteTimeRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_time_range: Option<AbsoluteTimeRange>,
    #[serde(rename = "Negate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negate: Option<bool>,
    #[serde(rename = "ParticipantRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_role: Option<String>,
    #[serde(rename = "RelativeTimeRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_time_range: Option<RelativeTimeRange>,
    #[serde(rename = "Sentiments")]
    #[serde(default)]
    pub sentiments: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TranscriptFilter {
    #[serde(rename = "AbsoluteTimeRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_time_range: Option<AbsoluteTimeRange>,
    #[serde(rename = "Negate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negate: Option<bool>,
    #[serde(rename = "ParticipantRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_role: Option<String>,
    #[serde(rename = "RelativeTimeRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_time_range: Option<RelativeTimeRange>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    pub targets: Vec<String>,
    #[serde(rename = "TranscriptFilterType")]
    #[serde(default)]
    pub transcript_filter_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCallAnalyticsCategoryResponse {
    #[serde(rename = "CategoryProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_properties: Option<CategoryProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CategoryProperties {
    #[serde(rename = "CategoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_name: Option<String>,
    #[serde(rename = "CreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    #[serde(rename = "InputType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_type: Option<String>,
    #[serde(rename = "LastUpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    #[serde(rename = "Rules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<Rule>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLanguageModelRequest {
    #[serde(rename = "BaseModelName")]
    #[serde(default)]
    pub base_model_name: String,
    #[serde(rename = "InputDataConfig")]
    #[serde(default)]
    pub input_data_config: InputDataConfig,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    pub language_code: String,
    #[serde(rename = "ModelName")]
    #[serde(default)]
    pub model_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputDataConfig {
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(default)]
    pub data_access_role_arn: String,
    #[serde(rename = "S3Uri")]
    #[serde(default)]
    pub s3_uri: String,
    #[serde(rename = "TuningDataS3Uri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tuning_data_s3_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateLanguageModelResponse {
    #[serde(rename = "BaseModelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_model_name: Option<String>,
    #[serde(rename = "InputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_config: Option<InputDataConfig>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "ModelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    #[serde(rename = "ModelStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMedicalVocabularyRequest {
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    pub language_code: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VocabularyFileUri")]
    #[serde(default)]
    pub vocabulary_file_uri: String,
    #[serde(rename = "VocabularyName")]
    #[serde(default)]
    pub vocabulary_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMedicalVocabularyResponse {
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "VocabularyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_name: Option<String>,
    #[serde(rename = "VocabularyState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVocabularyFilterRequest {
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_role_arn: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    pub language_code: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VocabularyFilterFileUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_filter_file_uri: Option<String>,
    #[serde(rename = "VocabularyFilterName")]
    #[serde(default)]
    pub vocabulary_filter_name: String,
    #[serde(rename = "Words")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub words: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVocabularyFilterResponse {
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "VocabularyFilterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_filter_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVocabularyRequest {
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_role_arn: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    pub language_code: String,
    #[serde(rename = "Phrases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phrases: Option<Vec<String>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VocabularyFileUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_file_uri: Option<String>,
    #[serde(rename = "VocabularyName")]
    #[serde(default)]
    pub vocabulary_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVocabularyResponse {
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "VocabularyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_name: Option<String>,
    #[serde(rename = "VocabularyState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCallAnalyticsCategoryRequest {
    #[serde(rename = "CategoryName")]
    #[serde(default)]
    pub category_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCallAnalyticsCategoryResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCallAnalyticsJobRequest {
    #[serde(rename = "CallAnalyticsJobName")]
    #[serde(default)]
    pub call_analytics_job_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCallAnalyticsJobResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteLanguageModelRequest {
    #[serde(rename = "ModelName")]
    #[serde(default)]
    pub model_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMedicalScribeJobRequest {
    #[serde(rename = "MedicalScribeJobName")]
    #[serde(default)]
    pub medical_scribe_job_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMedicalTranscriptionJobRequest {
    #[serde(rename = "MedicalTranscriptionJobName")]
    #[serde(default)]
    pub medical_transcription_job_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMedicalVocabularyRequest {
    #[serde(rename = "VocabularyName")]
    #[serde(default)]
    pub vocabulary_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTranscriptionJobRequest {
    #[serde(rename = "TranscriptionJobName")]
    #[serde(default)]
    pub transcription_job_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVocabularyFilterRequest {
    #[serde(rename = "VocabularyFilterName")]
    #[serde(default)]
    pub vocabulary_filter_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVocabularyRequest {
    #[serde(rename = "VocabularyName")]
    #[serde(default)]
    pub vocabulary_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLanguageModelRequest {
    #[serde(rename = "ModelName")]
    #[serde(default)]
    pub model_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeLanguageModelResponse {
    #[serde(rename = "LanguageModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_model: Option<LanguageModel>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LanguageModel {
    #[serde(rename = "BaseModelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_model_name: Option<String>,
    #[serde(rename = "CreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<f64>,
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "InputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_config: Option<InputDataConfig>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "ModelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    #[serde(rename = "ModelStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_status: Option<String>,
    #[serde(rename = "UpgradeAvailability")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_availability: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCallAnalyticsCategoryRequest {
    #[serde(rename = "CategoryName")]
    #[serde(default)]
    pub category_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCallAnalyticsCategoryResponse {
    #[serde(rename = "CategoryProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_properties: Option<CategoryProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCallAnalyticsJobRequest {
    #[serde(rename = "CallAnalyticsJobName")]
    #[serde(default)]
    pub call_analytics_job_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCallAnalyticsJobResponse {
    #[serde(rename = "CallAnalyticsJob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_analytics_job: Option<CallAnalyticsJob>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CallAnalyticsJob {
    #[serde(rename = "CallAnalyticsJobDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_analytics_job_details: Option<CallAnalyticsJobDetails>,
    #[serde(rename = "CallAnalyticsJobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_analytics_job_name: Option<String>,
    #[serde(rename = "CallAnalyticsJobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_analytics_job_status: Option<String>,
    #[serde(rename = "ChannelDefinitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_definitions: Option<Vec<ChannelDefinition>>,
    #[serde(rename = "CompletionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_time: Option<f64>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_role_arn: Option<String>,
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "IdentifiedLanguageScore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identified_language_score: Option<f32>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "Media")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Media>,
    #[serde(rename = "MediaFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_format: Option<String>,
    #[serde(rename = "MediaSampleRateHertz")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_sample_rate_hertz: Option<i32>,
    #[serde(rename = "Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<CallAnalyticsJobSettings>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Transcript")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript: Option<Transcript>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CallAnalyticsJobDetails {
    #[serde(rename = "Skipped")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipped: Option<Vec<CallAnalyticsSkippedFeature>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CallAnalyticsSkippedFeature {
    #[serde(rename = "Feature")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "ReasonCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChannelDefinition {
    #[serde(rename = "ChannelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<i32>,
    #[serde(rename = "ParticipantRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_role: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Media {
    #[serde(rename = "MediaFileUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_file_uri: Option<String>,
    #[serde(rename = "RedactedMediaFileUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redacted_media_file_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CallAnalyticsJobSettings {
    #[serde(rename = "ContentRedaction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_redaction: Option<ContentRedaction>,
    #[serde(rename = "LanguageIdSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_id_settings: Option<std::collections::HashMap<String, LanguageIdSettings>>,
    #[serde(rename = "LanguageModelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_model_name: Option<String>,
    #[serde(rename = "LanguageOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_options: Option<Vec<String>>,
    #[serde(rename = "Summarization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summarization: Option<Summarization>,
    #[serde(rename = "VocabularyFilterMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_filter_method: Option<String>,
    #[serde(rename = "VocabularyFilterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_filter_name: Option<String>,
    #[serde(rename = "VocabularyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContentRedaction {
    #[serde(rename = "PiiEntityTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pii_entity_types: Option<Vec<String>>,
    #[serde(rename = "RedactionOutput")]
    #[serde(default)]
    pub redaction_output: String,
    #[serde(rename = "RedactionType")]
    #[serde(default)]
    pub redaction_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LanguageIdSettings {
    #[serde(rename = "LanguageModelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_model_name: Option<String>,
    #[serde(rename = "VocabularyFilterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_filter_name: Option<String>,
    #[serde(rename = "VocabularyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Summarization {
    #[serde(rename = "GenerateAbstractiveSummary")]
    #[serde(default)]
    pub generate_abstractive_summary: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Transcript {
    #[serde(rename = "RedactedTranscriptFileUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redacted_transcript_file_uri: Option<String>,
    #[serde(rename = "TranscriptFileUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript_file_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMedicalScribeJobRequest {
    #[serde(rename = "MedicalScribeJobName")]
    #[serde(default)]
    pub medical_scribe_job_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMedicalScribeJobResponse {
    #[serde(rename = "MedicalScribeJob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medical_scribe_job: Option<MedicalScribeJob>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MedicalScribeJob {
    #[serde(rename = "ChannelDefinitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_definitions: Option<Vec<MedicalScribeChannelDefinition>>,
    #[serde(rename = "CompletionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_time: Option<f64>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_role_arn: Option<String>,
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "Media")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Media>,
    #[serde(rename = "MedicalScribeContextProvided")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medical_scribe_context_provided: Option<bool>,
    #[serde(rename = "MedicalScribeJobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medical_scribe_job_name: Option<String>,
    #[serde(rename = "MedicalScribeJobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medical_scribe_job_status: Option<String>,
    #[serde(rename = "MedicalScribeOutput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medical_scribe_output: Option<MedicalScribeOutput>,
    #[serde(rename = "Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<MedicalScribeSettings>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MedicalScribeChannelDefinition {
    #[serde(rename = "ChannelId")]
    #[serde(default)]
    pub channel_id: i32,
    #[serde(rename = "ParticipantRole")]
    #[serde(default)]
    pub participant_role: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MedicalScribeOutput {
    #[serde(rename = "ClinicalDocumentUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clinical_document_uri: Option<String>,
    #[serde(rename = "TranscriptFileUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript_file_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MedicalScribeSettings {
    #[serde(rename = "ChannelIdentification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_identification: Option<bool>,
    #[serde(rename = "ClinicalNoteGenerationSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clinical_note_generation_settings: Option<ClinicalNoteGenerationSettings>,
    #[serde(rename = "MaxSpeakerLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_speaker_labels: Option<i32>,
    #[serde(rename = "ShowSpeakerLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_speaker_labels: Option<bool>,
    #[serde(rename = "VocabularyFilterMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_filter_method: Option<String>,
    #[serde(rename = "VocabularyFilterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_filter_name: Option<String>,
    #[serde(rename = "VocabularyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClinicalNoteGenerationSettings {
    #[serde(rename = "NoteTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_template: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMedicalTranscriptionJobRequest {
    #[serde(rename = "MedicalTranscriptionJobName")]
    #[serde(default)]
    pub medical_transcription_job_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMedicalTranscriptionJobResponse {
    #[serde(rename = "MedicalTranscriptionJob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medical_transcription_job: Option<MedicalTranscriptionJob>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MedicalTranscriptionJob {
    #[serde(rename = "CompletionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_time: Option<f64>,
    #[serde(rename = "ContentIdentificationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_identification_type: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "Media")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Media>,
    #[serde(rename = "MediaFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_format: Option<String>,
    #[serde(rename = "MediaSampleRateHertz")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_sample_rate_hertz: Option<i32>,
    #[serde(rename = "MedicalTranscriptionJobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medical_transcription_job_name: Option<String>,
    #[serde(rename = "Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<MedicalTranscriptionSetting>,
    #[serde(rename = "Specialty")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specialty: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Transcript")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript: Option<MedicalTranscript>,
    #[serde(rename = "TranscriptionJobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_job_status: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MedicalTranscriptionSetting {
    #[serde(rename = "ChannelIdentification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_identification: Option<bool>,
    #[serde(rename = "MaxAlternatives")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_alternatives: Option<i32>,
    #[serde(rename = "MaxSpeakerLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_speaker_labels: Option<i32>,
    #[serde(rename = "ShowAlternatives")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_alternatives: Option<bool>,
    #[serde(rename = "ShowSpeakerLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_speaker_labels: Option<bool>,
    #[serde(rename = "VocabularyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MedicalTranscript {
    #[serde(rename = "TranscriptFileUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript_file_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMedicalVocabularyRequest {
    #[serde(rename = "VocabularyName")]
    #[serde(default)]
    pub vocabulary_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMedicalVocabularyResponse {
    #[serde(rename = "DownloadUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_uri: Option<String>,
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "VocabularyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_name: Option<String>,
    #[serde(rename = "VocabularyState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTranscriptionJobRequest {
    #[serde(rename = "TranscriptionJobName")]
    #[serde(default)]
    pub transcription_job_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTranscriptionJobResponse {
    #[serde(rename = "TranscriptionJob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_job: Option<TranscriptionJob>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TranscriptionJob {
    #[serde(rename = "CompletionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_time: Option<f64>,
    #[serde(rename = "ContentRedaction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_redaction: Option<ContentRedaction>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "IdentifiedLanguageScore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identified_language_score: Option<f32>,
    #[serde(rename = "IdentifyLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identify_language: Option<bool>,
    #[serde(rename = "IdentifyMultipleLanguages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identify_multiple_languages: Option<bool>,
    #[serde(rename = "JobExecutionSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_execution_settings: Option<JobExecutionSettings>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "LanguageCodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_codes: Option<Vec<LanguageCodeItem>>,
    #[serde(rename = "LanguageIdSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_id_settings: Option<std::collections::HashMap<String, LanguageIdSettings>>,
    #[serde(rename = "LanguageOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_options: Option<Vec<String>>,
    #[serde(rename = "Media")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Media>,
    #[serde(rename = "MediaFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_format: Option<String>,
    #[serde(rename = "MediaSampleRateHertz")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_sample_rate_hertz: Option<i32>,
    #[serde(rename = "ModelSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_settings: Option<ModelSettings>,
    #[serde(rename = "Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Settings>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "Subtitles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitles: Option<SubtitlesOutput>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "ToxicityDetection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toxicity_detection: Option<Vec<ToxicityDetectionSettings>>,
    #[serde(rename = "Transcript")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript: Option<Transcript>,
    #[serde(rename = "TranscriptionJobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_job_name: Option<String>,
    #[serde(rename = "TranscriptionJobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_job_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobExecutionSettings {
    #[serde(rename = "AllowDeferredExecution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_deferred_execution: Option<bool>,
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LanguageCodeItem {
    #[serde(rename = "DurationInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<f32>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModelSettings {
    #[serde(rename = "LanguageModelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_model_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Settings {
    #[serde(rename = "ChannelIdentification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_identification: Option<bool>,
    #[serde(rename = "MaxAlternatives")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_alternatives: Option<i32>,
    #[serde(rename = "MaxSpeakerLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_speaker_labels: Option<i32>,
    #[serde(rename = "ShowAlternatives")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_alternatives: Option<bool>,
    #[serde(rename = "ShowSpeakerLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_speaker_labels: Option<bool>,
    #[serde(rename = "VocabularyFilterMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_filter_method: Option<String>,
    #[serde(rename = "VocabularyFilterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_filter_name: Option<String>,
    #[serde(rename = "VocabularyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubtitlesOutput {
    #[serde(rename = "Formats")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formats: Option<Vec<String>>,
    #[serde(rename = "OutputStartIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_start_index: Option<i32>,
    #[serde(rename = "SubtitleFileUris")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle_file_uris: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ToxicityDetectionSettings {
    #[serde(rename = "ToxicityCategories")]
    #[serde(default)]
    pub toxicity_categories: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetVocabularyFilterRequest {
    #[serde(rename = "VocabularyFilterName")]
    #[serde(default)]
    pub vocabulary_filter_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetVocabularyFilterResponse {
    #[serde(rename = "DownloadUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_uri: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "VocabularyFilterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_filter_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetVocabularyRequest {
    #[serde(rename = "VocabularyName")]
    #[serde(default)]
    pub vocabulary_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetVocabularyResponse {
    #[serde(rename = "DownloadUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_uri: Option<String>,
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "VocabularyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_name: Option<String>,
    #[serde(rename = "VocabularyState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCallAnalyticsCategoriesRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCallAnalyticsCategoriesResponse {
    #[serde(rename = "Categories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<CategoryProperties>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCallAnalyticsJobsRequest {
    #[serde(rename = "JobNameContains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name_contains: Option<String>,
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
pub struct ListCallAnalyticsJobsResponse {
    #[serde(rename = "CallAnalyticsJobSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_analytics_job_summaries: Option<Vec<CallAnalyticsJobSummary>>,
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
pub struct CallAnalyticsJobSummary {
    #[serde(rename = "CallAnalyticsJobDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_analytics_job_details: Option<CallAnalyticsJobDetails>,
    #[serde(rename = "CallAnalyticsJobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_analytics_job_name: Option<String>,
    #[serde(rename = "CallAnalyticsJobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_analytics_job_status: Option<String>,
    #[serde(rename = "CompletionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_time: Option<f64>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLanguageModelsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NameContains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StatusEquals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_equals: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListLanguageModelsResponse {
    #[serde(rename = "Models")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub models: Option<Vec<LanguageModel>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMedicalScribeJobsRequest {
    #[serde(rename = "JobNameContains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name_contains: Option<String>,
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
pub struct ListMedicalScribeJobsResponse {
    #[serde(rename = "MedicalScribeJobSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medical_scribe_job_summaries: Option<Vec<MedicalScribeJobSummary>>,
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
pub struct MedicalScribeJobSummary {
    #[serde(rename = "CompletionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_time: Option<f64>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "MedicalScribeJobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medical_scribe_job_name: Option<String>,
    #[serde(rename = "MedicalScribeJobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medical_scribe_job_status: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMedicalTranscriptionJobsRequest {
    #[serde(rename = "JobNameContains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name_contains: Option<String>,
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
pub struct ListMedicalTranscriptionJobsResponse {
    #[serde(rename = "MedicalTranscriptionJobSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medical_transcription_job_summaries: Option<Vec<MedicalTranscriptionJobSummary>>,
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
pub struct MedicalTranscriptionJobSummary {
    #[serde(rename = "CompletionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_time: Option<f64>,
    #[serde(rename = "ContentIdentificationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_identification_type: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "MedicalTranscriptionJobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medical_transcription_job_name: Option<String>,
    #[serde(rename = "OutputLocationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location_type: Option<String>,
    #[serde(rename = "Specialty")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specialty: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "TranscriptionJobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_job_status: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMedicalVocabulariesRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NameContains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StateEquals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_equals: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMedicalVocabulariesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Vocabularies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabularies: Option<Vec<VocabularyInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VocabularyInfo {
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "VocabularyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_name: Option<String>,
    #[serde(rename = "VocabularyState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTranscriptionJobsRequest {
    #[serde(rename = "JobNameContains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name_contains: Option<String>,
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
pub struct ListTranscriptionJobsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TranscriptionJobSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_job_summaries: Option<Vec<TranscriptionJobSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TranscriptionJobSummary {
    #[serde(rename = "CompletionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_time: Option<f64>,
    #[serde(rename = "ContentRedaction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_redaction: Option<ContentRedaction>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "IdentifiedLanguageScore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identified_language_score: Option<f32>,
    #[serde(rename = "IdentifyLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identify_language: Option<bool>,
    #[serde(rename = "IdentifyMultipleLanguages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identify_multiple_languages: Option<bool>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "LanguageCodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_codes: Option<Vec<LanguageCodeItem>>,
    #[serde(rename = "ModelSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_settings: Option<ModelSettings>,
    #[serde(rename = "OutputLocationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location_type: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "ToxicityDetection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toxicity_detection: Option<Vec<ToxicityDetectionSettings>>,
    #[serde(rename = "TranscriptionJobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_job_name: Option<String>,
    #[serde(rename = "TranscriptionJobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_job_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVocabulariesRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NameContains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StateEquals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_equals: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVocabulariesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Vocabularies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabularies: Option<Vec<VocabularyInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVocabularyFiltersRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NameContains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_contains: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVocabularyFiltersResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "VocabularyFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_filters: Option<Vec<VocabularyFilterInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VocabularyFilterInfo {
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "VocabularyFilterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_filter_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartCallAnalyticsJobRequest {
    #[serde(rename = "CallAnalyticsJobName")]
    #[serde(default)]
    pub call_analytics_job_name: String,
    #[serde(rename = "ChannelDefinitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_definitions: Option<Vec<ChannelDefinition>>,
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_role_arn: Option<String>,
    #[serde(rename = "Media")]
    #[serde(default)]
    pub media: Media,
    #[serde(rename = "OutputEncryptionKMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_encryption_k_m_s_key_id: Option<String>,
    #[serde(rename = "OutputLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location: Option<String>,
    #[serde(rename = "Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<CallAnalyticsJobSettings>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartCallAnalyticsJobResponse {
    #[serde(rename = "CallAnalyticsJob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_analytics_job: Option<CallAnalyticsJob>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMedicalScribeJobRequest {
    #[serde(rename = "ChannelDefinitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_definitions: Option<Vec<MedicalScribeChannelDefinition>>,
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(default)]
    pub data_access_role_arn: String,
    #[serde(rename = "KMSEncryptionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_encryption_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Media")]
    #[serde(default)]
    pub media: Media,
    #[serde(rename = "MedicalScribeContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medical_scribe_context: Option<MedicalScribeContext>,
    #[serde(rename = "MedicalScribeJobName")]
    #[serde(default)]
    pub medical_scribe_job_name: String,
    #[serde(rename = "OutputBucketName")]
    #[serde(default)]
    pub output_bucket_name: String,
    #[serde(rename = "OutputEncryptionKMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_encryption_k_m_s_key_id: Option<String>,
    #[serde(rename = "Settings")]
    #[serde(default)]
    pub settings: MedicalScribeSettings,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MedicalScribeContext {
    #[serde(rename = "PatientContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patient_context: Option<MedicalScribePatientContext>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MedicalScribePatientContext {
    #[serde(rename = "Pronouns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pronouns: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMedicalScribeJobResponse {
    #[serde(rename = "MedicalScribeJob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medical_scribe_job: Option<MedicalScribeJob>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMedicalTranscriptionJobRequest {
    #[serde(rename = "ContentIdentificationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_identification_type: Option<String>,
    #[serde(rename = "KMSEncryptionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_encryption_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    pub language_code: String,
    #[serde(rename = "Media")]
    #[serde(default)]
    pub media: Media,
    #[serde(rename = "MediaFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_format: Option<String>,
    #[serde(rename = "MediaSampleRateHertz")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_sample_rate_hertz: Option<i32>,
    #[serde(rename = "MedicalTranscriptionJobName")]
    #[serde(default)]
    pub medical_transcription_job_name: String,
    #[serde(rename = "OutputBucketName")]
    #[serde(default)]
    pub output_bucket_name: String,
    #[serde(rename = "OutputEncryptionKMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_encryption_k_m_s_key_id: Option<String>,
    #[serde(rename = "OutputKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_key: Option<String>,
    #[serde(rename = "Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<MedicalTranscriptionSetting>,
    #[serde(rename = "Specialty")]
    #[serde(default)]
    pub specialty: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMedicalTranscriptionJobResponse {
    #[serde(rename = "MedicalTranscriptionJob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medical_transcription_job: Option<MedicalTranscriptionJob>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartTranscriptionJobRequest {
    #[serde(rename = "ContentRedaction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_redaction: Option<ContentRedaction>,
    #[serde(rename = "IdentifyLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identify_language: Option<bool>,
    #[serde(rename = "IdentifyMultipleLanguages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identify_multiple_languages: Option<bool>,
    #[serde(rename = "JobExecutionSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_execution_settings: Option<JobExecutionSettings>,
    #[serde(rename = "KMSEncryptionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_encryption_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "LanguageIdSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_id_settings: Option<std::collections::HashMap<String, LanguageIdSettings>>,
    #[serde(rename = "LanguageOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_options: Option<Vec<String>>,
    #[serde(rename = "Media")]
    #[serde(default)]
    pub media: Media,
    #[serde(rename = "MediaFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_format: Option<String>,
    #[serde(rename = "MediaSampleRateHertz")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_sample_rate_hertz: Option<i32>,
    #[serde(rename = "ModelSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_settings: Option<ModelSettings>,
    #[serde(rename = "OutputBucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_bucket_name: Option<String>,
    #[serde(rename = "OutputEncryptionKMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_encryption_k_m_s_key_id: Option<String>,
    #[serde(rename = "OutputKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_key: Option<String>,
    #[serde(rename = "Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Settings>,
    #[serde(rename = "Subtitles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitles: Option<Subtitles>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "ToxicityDetection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toxicity_detection: Option<Vec<ToxicityDetectionSettings>>,
    #[serde(rename = "TranscriptionJobName")]
    #[serde(default)]
    pub transcription_job_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Subtitles {
    #[serde(rename = "Formats")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formats: Option<Vec<String>>,
    #[serde(rename = "OutputStartIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_start_index: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartTranscriptionJobResponse {
    #[serde(rename = "TranscriptionJob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcription_job: Option<TranscriptionJob>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCallAnalyticsCategoryRequest {
    #[serde(rename = "CategoryName")]
    #[serde(default)]
    pub category_name: String,
    #[serde(rename = "InputType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_type: Option<String>,
    #[serde(rename = "Rules")]
    #[serde(default)]
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCallAnalyticsCategoryResponse {
    #[serde(rename = "CategoryProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_properties: Option<CategoryProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMedicalVocabularyRequest {
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    pub language_code: String,
    #[serde(rename = "VocabularyFileUri")]
    #[serde(default)]
    pub vocabulary_file_uri: String,
    #[serde(rename = "VocabularyName")]
    #[serde(default)]
    pub vocabulary_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateMedicalVocabularyResponse {
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "VocabularyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_name: Option<String>,
    #[serde(rename = "VocabularyState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateVocabularyFilterRequest {
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_role_arn: Option<String>,
    #[serde(rename = "VocabularyFilterFileUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_filter_file_uri: Option<String>,
    #[serde(rename = "VocabularyFilterName")]
    #[serde(default)]
    pub vocabulary_filter_name: String,
    #[serde(rename = "Words")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub words: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateVocabularyFilterResponse {
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "VocabularyFilterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_filter_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateVocabularyRequest {
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_role_arn: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    pub language_code: String,
    #[serde(rename = "Phrases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phrases: Option<Vec<String>>,
    #[serde(rename = "VocabularyFileUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_file_uri: Option<String>,
    #[serde(rename = "VocabularyName")]
    #[serde(default)]
    pub vocabulary_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateVocabularyResponse {
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "VocabularyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_name: Option<String>,
    #[serde(rename = "VocabularyState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_state: Option<String>,
}
