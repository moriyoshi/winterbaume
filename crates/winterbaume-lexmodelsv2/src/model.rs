//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-lexmodelsv2

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchCreateCustomVocabularyItemRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(rename = "customVocabularyItemList")]
    #[serde(default)]
    pub custom_vocabulary_item_list: Vec<NewCustomVocabularyItem>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NewCustomVocabularyItem {
    #[serde(rename = "displayAs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_as: Option<String>,
    #[serde(default)]
    pub phrase: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchCreateCustomVocabularyItemResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<FailedCustomVocabularyItem>>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<CustomVocabularyItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailedCustomVocabularyItem {
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "itemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomVocabularyItem {
    #[serde(rename = "displayAs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_as: Option<String>,
    #[serde(rename = "itemId")]
    #[serde(default)]
    pub item_id: String,
    #[serde(default)]
    pub phrase: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteCustomVocabularyItemRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(rename = "customVocabularyItemList")]
    #[serde(default)]
    pub custom_vocabulary_item_list: Vec<CustomVocabularyEntryId>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomVocabularyEntryId {
    #[serde(rename = "itemId")]
    #[serde(default)]
    pub item_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteCustomVocabularyItemResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<FailedCustomVocabularyItem>>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<CustomVocabularyItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateCustomVocabularyItemRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(rename = "customVocabularyItemList")]
    #[serde(default)]
    pub custom_vocabulary_item_list: Vec<CustomVocabularyItem>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateCustomVocabularyItemResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<FailedCustomVocabularyItem>>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<CustomVocabularyItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BuildBotLocaleRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BuildBotLocaleResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botLocaleStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_locale_status: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "lastBuildSubmittedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_build_submitted_date_time: Option<f64>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBotAliasRequest {
    #[serde(rename = "botAliasLocaleSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_alias_locale_settings:
        Option<std::collections::HashMap<String, BotAliasLocaleSettings>>,
    #[serde(rename = "botAliasName")]
    #[serde(default)]
    pub bot_alias_name: String,
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "conversationLogSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_log_settings: Option<ConversationLogSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "sentimentAnalysisSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiment_analysis_settings: Option<SentimentAnalysisSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BotAliasLocaleSettings {
    #[serde(rename = "codeHookSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_hook_specification: Option<CodeHookSpecification>,
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeHookSpecification {
    #[serde(rename = "lambdaCodeHook")]
    #[serde(default)]
    pub lambda_code_hook: LambdaCodeHook,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LambdaCodeHook {
    #[serde(rename = "codeHookInterfaceVersion")]
    #[serde(default)]
    pub code_hook_interface_version: String,
    #[serde(rename = "lambdaARN")]
    #[serde(default)]
    pub lambda_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConversationLogSettings {
    #[serde(rename = "audioLogSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_log_settings: Option<Vec<AudioLogSetting>>,
    #[serde(rename = "textLogSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_log_settings: Option<Vec<TextLogSetting>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AudioLogSetting {
    #[serde(default)]
    pub destination: AudioLogDestination,
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "selectiveLoggingEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective_logging_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AudioLogDestination {
    #[serde(rename = "s3Bucket")]
    #[serde(default)]
    pub s3_bucket: S3BucketLogDestination,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3BucketLogDestination {
    #[serde(rename = "kmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "logPrefix")]
    #[serde(default)]
    pub log_prefix: String,
    #[serde(rename = "s3BucketArn")]
    #[serde(default)]
    pub s3_bucket_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TextLogSetting {
    #[serde(default)]
    pub destination: TextLogDestination,
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "selectiveLoggingEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective_logging_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TextLogDestination {
    #[serde(rename = "cloudWatch")]
    #[serde(default)]
    pub cloud_watch: CloudWatchLogGroupLogDestination,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudWatchLogGroupLogDestination {
    #[serde(rename = "cloudWatchLogGroupArn")]
    #[serde(default)]
    pub cloud_watch_log_group_arn: String,
    #[serde(rename = "logPrefix")]
    #[serde(default)]
    pub log_prefix: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SentimentAnalysisSettings {
    #[serde(rename = "detectSentiment")]
    #[serde(default)]
    pub detect_sentiment: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBotAliasResponse {
    #[serde(rename = "botAliasId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_alias_id: Option<String>,
    #[serde(rename = "botAliasLocaleSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_alias_locale_settings:
        Option<std::collections::HashMap<String, BotAliasLocaleSettings>>,
    #[serde(rename = "botAliasName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_alias_name: Option<String>,
    #[serde(rename = "botAliasStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_alias_status: Option<String>,
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "conversationLogSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_log_settings: Option<ConversationLogSettings>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "sentimentAnalysisSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiment_analysis_settings: Option<SentimentAnalysisSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBotLocaleRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "generativeAISettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generative_a_i_settings: Option<GenerativeAISettings>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
    #[serde(rename = "nluIntentConfidenceThreshold")]
    #[serde(default)]
    pub nlu_intent_confidence_threshold: f64,
    #[serde(rename = "speechDetectionSensitivity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_detection_sensitivity: Option<String>,
    #[serde(rename = "speechRecognitionSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_recognition_settings: Option<SpeechRecognitionSettings>,
    #[serde(rename = "unifiedSpeechSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unified_speech_settings: Option<UnifiedSpeechSettings>,
    #[serde(rename = "voiceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_settings: Option<VoiceSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GenerativeAISettings {
    #[serde(rename = "buildtimeSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buildtime_settings: Option<BuildtimeSettings>,
    #[serde(rename = "runtimeSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_settings: Option<RuntimeSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BuildtimeSettings {
    #[serde(rename = "descriptiveBotBuilder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_bot_builder: Option<DescriptiveBotBuilderSpecification>,
    #[serde(rename = "sampleUtteranceGeneration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_utterance_generation: Option<SampleUtteranceGenerationSpecification>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescriptiveBotBuilderSpecification {
    #[serde(rename = "bedrockModelSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bedrock_model_specification: Option<BedrockModelSpecification>,
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BedrockModelSpecification {
    #[serde(rename = "customPrompt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_prompt: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrail: Option<BedrockGuardrailConfiguration>,
    #[serde(rename = "modelArn")]
    #[serde(default)]
    pub model_arn: String,
    #[serde(rename = "traceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BedrockGuardrailConfiguration {
    #[serde(default)]
    pub identifier: String,
    #[serde(default)]
    pub version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SampleUtteranceGenerationSpecification {
    #[serde(rename = "bedrockModelSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bedrock_model_specification: Option<BedrockModelSpecification>,
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuntimeSettings {
    #[serde(rename = "nluImprovement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nlu_improvement: Option<NluImprovementSpecification>,
    #[serde(rename = "slotResolutionImprovement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_resolution_improvement: Option<SlotResolutionImprovementSpecification>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NluImprovementSpecification {
    #[serde(rename = "assistedNluMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assisted_nlu_mode: Option<String>,
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "intentDisambiguationSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_disambiguation_settings: Option<IntentDisambiguationSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntentDisambiguationSettings {
    #[serde(rename = "customDisambiguationMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_disambiguation_message: Option<String>,
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "maxDisambiguationIntents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_disambiguation_intents: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SlotResolutionImprovementSpecification {
    #[serde(rename = "bedrockModelSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bedrock_model_specification: Option<BedrockModelSpecification>,
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SpeechRecognitionSettings {
    #[serde(rename = "speechModelConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_model_config: Option<SpeechModelConfig>,
    #[serde(rename = "speechModelPreference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_model_preference: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SpeechModelConfig {
    #[serde(rename = "deepgramConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deepgram_config: Option<DeepgramSpeechModelConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeepgramSpeechModelConfig {
    #[serde(rename = "apiTokenSecretArn")]
    #[serde(default)]
    pub api_token_secret_arn: String,
    #[serde(rename = "modelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnifiedSpeechSettings {
    #[serde(rename = "speechFoundationModel")]
    #[serde(default)]
    pub speech_foundation_model: SpeechFoundationModel,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SpeechFoundationModel {
    #[serde(rename = "modelArn")]
    #[serde(default)]
    pub model_arn: String,
    #[serde(rename = "voiceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VoiceSettings {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "voiceId")]
    #[serde(default)]
    pub voice_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBotLocaleResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botLocaleStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_locale_status: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "generativeAISettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generative_a_i_settings: Option<GenerativeAISettings>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(rename = "localeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_name: Option<String>,
    #[serde(rename = "nluIntentConfidenceThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nlu_intent_confidence_threshold: Option<f64>,
    #[serde(rename = "speechDetectionSensitivity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_detection_sensitivity: Option<String>,
    #[serde(rename = "speechRecognitionSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_recognition_settings: Option<SpeechRecognitionSettings>,
    #[serde(rename = "unifiedSpeechSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unified_speech_settings: Option<UnifiedSpeechSettings>,
    #[serde(rename = "voiceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_settings: Option<VoiceSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBotReplicaRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "replicaRegion")]
    #[serde(default)]
    pub replica_region: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBotReplicaResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botReplicaStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_replica_status: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "replicaRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_region: Option<String>,
    #[serde(rename = "sourceRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBotRequest {
    #[serde(rename = "botMembers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_members: Option<Vec<BotMember>>,
    #[serde(rename = "botName")]
    #[serde(default)]
    pub bot_name: String,
    #[serde(rename = "botTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "botType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_type: Option<String>,
    #[serde(rename = "dataPrivacy")]
    #[serde(default)]
    pub data_privacy: DataPrivacy,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "errorLogSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_log_settings: Option<ErrorLogSettings>,
    #[serde(rename = "idleSessionTTLInSeconds")]
    #[serde(default)]
    pub idle_session_t_t_l_in_seconds: i32,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "testBotAliasTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_bot_alias_tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BotMember {
    #[serde(rename = "botMemberAliasId")]
    #[serde(default)]
    pub bot_member_alias_id: String,
    #[serde(rename = "botMemberAliasName")]
    #[serde(default)]
    pub bot_member_alias_name: String,
    #[serde(rename = "botMemberId")]
    #[serde(default)]
    pub bot_member_id: String,
    #[serde(rename = "botMemberName")]
    #[serde(default)]
    pub bot_member_name: String,
    #[serde(rename = "botMemberVersion")]
    #[serde(default)]
    pub bot_member_version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataPrivacy {
    #[serde(rename = "childDirected")]
    #[serde(default)]
    pub child_directed: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ErrorLogSettings {
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBotResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botMembers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_members: Option<Vec<BotMember>>,
    #[serde(rename = "botName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_name: Option<String>,
    #[serde(rename = "botStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_status: Option<String>,
    #[serde(rename = "botTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "botType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_type: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "dataPrivacy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_privacy: Option<DataPrivacy>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "errorLogSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_log_settings: Option<ErrorLogSettings>,
    #[serde(rename = "idleSessionTTLInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_session_t_t_l_in_seconds: Option<i32>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "testBotAliasTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_bot_alias_tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBotVersionRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersionLocaleSpecification")]
    #[serde(default)]
    pub bot_version_locale_specification:
        std::collections::HashMap<String, BotVersionLocaleDetails>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BotVersionLocaleDetails {
    #[serde(rename = "sourceBotVersion")]
    #[serde(default)]
    pub source_bot_version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBotVersionResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_status: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "botVersionLocaleSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version_locale_specification:
        Option<std::collections::HashMap<String, BotVersionLocaleDetails>>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateExportRequest {
    #[serde(rename = "fileFormat")]
    #[serde(default)]
    pub file_format: String,
    #[serde(rename = "filePassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_password: Option<String>,
    #[serde(rename = "resourceSpecification")]
    #[serde(default)]
    pub resource_specification: ExportResourceSpecification,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportResourceSpecification {
    #[serde(rename = "botExportSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_export_specification: Option<BotExportSpecification>,
    #[serde(rename = "botLocaleExportSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_locale_export_specification: Option<BotLocaleExportSpecification>,
    #[serde(rename = "customVocabularyExportSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_vocabulary_export_specification: Option<CustomVocabularyExportSpecification>,
    #[serde(rename = "testSetExportSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_set_export_specification: Option<TestSetExportSpecification>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BotExportSpecification {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BotLocaleExportSpecification {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomVocabularyExportSpecification {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestSetExportSpecification {
    #[serde(rename = "testSetId")]
    #[serde(default)]
    pub test_set_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateExportResponse {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "exportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_id: Option<String>,
    #[serde(rename = "exportStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_status: Option<String>,
    #[serde(rename = "fileFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_format: Option<String>,
    #[serde(rename = "resourceSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_specification: Option<ExportResourceSpecification>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIntentRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "dialogCodeHook")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialog_code_hook: Option<DialogCodeHookSettings>,
    #[serde(rename = "fulfillmentCodeHook")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_code_hook: Option<FulfillmentCodeHookSettings>,
    #[serde(rename = "initialResponseSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_response_setting: Option<InitialResponseSetting>,
    #[serde(rename = "inputContexts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_contexts: Option<Vec<InputContext>>,
    #[serde(rename = "intentClosingSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_closing_setting: Option<IntentClosingSetting>,
    #[serde(rename = "intentConfirmationSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_confirmation_setting: Option<IntentConfirmationSetting>,
    #[serde(rename = "intentDisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_display_name: Option<String>,
    #[serde(rename = "intentName")]
    #[serde(default)]
    pub intent_name: String,
    #[serde(rename = "kendraConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kendra_configuration: Option<KendraConfiguration>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
    #[serde(rename = "outputContexts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_contexts: Option<Vec<OutputContext>>,
    #[serde(rename = "parentIntentSignature")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_intent_signature: Option<String>,
    #[serde(rename = "qInConnectIntentConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q_in_connect_intent_configuration: Option<QInConnectIntentConfiguration>,
    #[serde(rename = "qnAIntentConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qn_a_intent_configuration: Option<QnAIntentConfiguration>,
    #[serde(rename = "sampleUtterances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_utterances: Option<Vec<SampleUtterance>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DialogCodeHookSettings {
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FulfillmentCodeHookSettings {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "fulfillmentUpdatesSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_updates_specification: Option<FulfillmentUpdatesSpecification>,
    #[serde(rename = "postFulfillmentStatusSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_fulfillment_status_specification: Option<PostFulfillmentStatusSpecification>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FulfillmentUpdatesSpecification {
    #[serde(default)]
    pub active: bool,
    #[serde(rename = "startResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_response: Option<FulfillmentStartResponseSpecification>,
    #[serde(rename = "timeoutInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_seconds: Option<i32>,
    #[serde(rename = "updateResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_response: Option<FulfillmentUpdateResponseSpecification>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FulfillmentStartResponseSpecification {
    #[serde(rename = "allowInterrupt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_interrupt: Option<bool>,
    #[serde(rename = "delayInSeconds")]
    #[serde(default)]
    pub delay_in_seconds: i32,
    #[serde(rename = "messageGroups")]
    #[serde(default)]
    pub message_groups: Vec<MessageGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MessageGroup {
    #[serde(default)]
    pub message: Message,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variations: Option<Vec<Message>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Message {
    #[serde(rename = "customPayload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_payload: Option<CustomPayload>,
    #[serde(rename = "imageResponseCard")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_response_card: Option<ImageResponseCard>,
    #[serde(rename = "plainTextMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plain_text_message: Option<PlainTextMessage>,
    #[serde(rename = "ssmlMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssml_message: Option<SSMLMessage>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomPayload {
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImageResponseCard {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<Button>>,
    #[serde(rename = "imageUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(default)]
    pub title: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Button {
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PlainTextMessage {
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SSMLMessage {
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FulfillmentUpdateResponseSpecification {
    #[serde(rename = "allowInterrupt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_interrupt: Option<bool>,
    #[serde(rename = "frequencyInSeconds")]
    #[serde(default)]
    pub frequency_in_seconds: i32,
    #[serde(rename = "messageGroups")]
    #[serde(default)]
    pub message_groups: Vec<MessageGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PostFulfillmentStatusSpecification {
    #[serde(rename = "failureConditional")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_conditional: Option<ConditionalSpecification>,
    #[serde(rename = "failureNextStep")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_next_step: Option<DialogState>,
    #[serde(rename = "failureResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_response: Option<ResponseSpecification>,
    #[serde(rename = "successConditional")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_conditional: Option<ConditionalSpecification>,
    #[serde(rename = "successNextStep")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_next_step: Option<DialogState>,
    #[serde(rename = "successResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_response: Option<ResponseSpecification>,
    #[serde(rename = "timeoutConditional")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_conditional: Option<ConditionalSpecification>,
    #[serde(rename = "timeoutNextStep")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_next_step: Option<DialogState>,
    #[serde(rename = "timeoutResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_response: Option<ResponseSpecification>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConditionalSpecification {
    #[serde(default)]
    pub active: bool,
    #[serde(rename = "conditionalBranches")]
    #[serde(default)]
    pub conditional_branches: Vec<ConditionalBranch>,
    #[serde(rename = "defaultBranch")]
    #[serde(default)]
    pub default_branch: DefaultConditionalBranch,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConditionalBranch {
    #[serde(default)]
    pub condition: Condition,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "nextStep")]
    #[serde(default)]
    pub next_step: DialogState,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<ResponseSpecification>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Condition {
    #[serde(rename = "expressionString")]
    #[serde(default)]
    pub expression_string: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DialogState {
    #[serde(rename = "dialogAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialog_action: Option<DialogAction>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent: Option<IntentOverride>,
    #[serde(rename = "sessionAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_attributes: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DialogAction {
    #[serde(rename = "slotToElicit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_to_elicit: Option<String>,
    #[serde(rename = "suppressNextMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppress_next_message: Option<bool>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntentOverride {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slots: Option<std::collections::HashMap<String, SlotValueOverride>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SlotValueOverride {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shape: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<SlotValue>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<SlotValueOverride>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SlotValue {
    #[serde(rename = "interpretedValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpreted_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResponseSpecification {
    #[serde(rename = "allowInterrupt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_interrupt: Option<bool>,
    #[serde(rename = "messageGroups")]
    #[serde(default)]
    pub message_groups: Vec<MessageGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DefaultConditionalBranch {
    #[serde(rename = "nextStep")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_step: Option<DialogState>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<ResponseSpecification>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InitialResponseSetting {
    #[serde(rename = "codeHook")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_hook: Option<DialogCodeHookInvocationSetting>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional: Option<ConditionalSpecification>,
    #[serde(rename = "initialResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_response: Option<ResponseSpecification>,
    #[serde(rename = "nextStep")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_step: Option<DialogState>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DialogCodeHookInvocationSetting {
    #[serde(default)]
    pub active: bool,
    #[serde(rename = "enableCodeHookInvocation")]
    #[serde(default)]
    pub enable_code_hook_invocation: bool,
    #[serde(rename = "invocationLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_label: Option<String>,
    #[serde(rename = "postCodeHookSpecification")]
    #[serde(default)]
    pub post_code_hook_specification: PostDialogCodeHookInvocationSpecification,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PostDialogCodeHookInvocationSpecification {
    #[serde(rename = "failureConditional")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_conditional: Option<ConditionalSpecification>,
    #[serde(rename = "failureNextStep")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_next_step: Option<DialogState>,
    #[serde(rename = "failureResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_response: Option<ResponseSpecification>,
    #[serde(rename = "successConditional")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_conditional: Option<ConditionalSpecification>,
    #[serde(rename = "successNextStep")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_next_step: Option<DialogState>,
    #[serde(rename = "successResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_response: Option<ResponseSpecification>,
    #[serde(rename = "timeoutConditional")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_conditional: Option<ConditionalSpecification>,
    #[serde(rename = "timeoutNextStep")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_next_step: Option<DialogState>,
    #[serde(rename = "timeoutResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_response: Option<ResponseSpecification>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputContext {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntentClosingSetting {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "closingResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closing_response: Option<ResponseSpecification>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional: Option<ConditionalSpecification>,
    #[serde(rename = "nextStep")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_step: Option<DialogState>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntentConfirmationSetting {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "codeHook")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_hook: Option<DialogCodeHookInvocationSetting>,
    #[serde(rename = "confirmationConditional")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_conditional: Option<ConditionalSpecification>,
    #[serde(rename = "confirmationNextStep")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_next_step: Option<DialogState>,
    #[serde(rename = "confirmationResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_response: Option<ResponseSpecification>,
    #[serde(rename = "declinationConditional")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub declination_conditional: Option<ConditionalSpecification>,
    #[serde(rename = "declinationNextStep")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub declination_next_step: Option<DialogState>,
    #[serde(rename = "declinationResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub declination_response: Option<ResponseSpecification>,
    #[serde(rename = "elicitationCodeHook")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elicitation_code_hook: Option<ElicitationCodeHookInvocationSetting>,
    #[serde(rename = "failureConditional")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_conditional: Option<ConditionalSpecification>,
    #[serde(rename = "failureNextStep")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_next_step: Option<DialogState>,
    #[serde(rename = "failureResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_response: Option<ResponseSpecification>,
    #[serde(rename = "promptSpecification")]
    #[serde(default)]
    pub prompt_specification: PromptSpecification,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ElicitationCodeHookInvocationSetting {
    #[serde(rename = "enableCodeHookInvocation")]
    #[serde(default)]
    pub enable_code_hook_invocation: bool,
    #[serde(rename = "invocationLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_label: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PromptSpecification {
    #[serde(rename = "allowInterrupt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_interrupt: Option<bool>,
    #[serde(rename = "maxRetries")]
    #[serde(default)]
    pub max_retries: i32,
    #[serde(rename = "messageGroups")]
    #[serde(default)]
    pub message_groups: Vec<MessageGroup>,
    #[serde(rename = "messageSelectionStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_selection_strategy: Option<String>,
    #[serde(rename = "promptAttemptsSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_attempts_specification:
        Option<std::collections::HashMap<String, PromptAttemptSpecification>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PromptAttemptSpecification {
    #[serde(rename = "allowInterrupt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_interrupt: Option<bool>,
    #[serde(rename = "allowedInputTypes")]
    #[serde(default)]
    pub allowed_input_types: AllowedInputTypes,
    #[serde(rename = "audioAndDTMFInputSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_and_d_t_m_f_input_specification: Option<AudioAndDTMFInputSpecification>,
    #[serde(rename = "textInputSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_input_specification: Option<TextInputSpecification>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AllowedInputTypes {
    #[serde(rename = "allowAudioInput")]
    #[serde(default)]
    pub allow_audio_input: bool,
    #[serde(rename = "allowDTMFInput")]
    #[serde(default)]
    pub allow_d_t_m_f_input: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AudioAndDTMFInputSpecification {
    #[serde(rename = "audioSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_specification: Option<AudioSpecification>,
    #[serde(rename = "dtmfSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dtmf_specification: Option<DTMFSpecification>,
    #[serde(rename = "startTimeoutMs")]
    #[serde(default)]
    pub start_timeout_ms: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AudioSpecification {
    #[serde(rename = "endTimeoutMs")]
    #[serde(default)]
    pub end_timeout_ms: i32,
    #[serde(rename = "maxLengthMs")]
    #[serde(default)]
    pub max_length_ms: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DTMFSpecification {
    #[serde(rename = "deletionCharacter")]
    #[serde(default)]
    pub deletion_character: String,
    #[serde(rename = "endCharacter")]
    #[serde(default)]
    pub end_character: String,
    #[serde(rename = "endTimeoutMs")]
    #[serde(default)]
    pub end_timeout_ms: i32,
    #[serde(rename = "maxLength")]
    #[serde(default)]
    pub max_length: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TextInputSpecification {
    #[serde(rename = "startTimeoutMs")]
    #[serde(default)]
    pub start_timeout_ms: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KendraConfiguration {
    #[serde(rename = "kendraIndex")]
    #[serde(default)]
    pub kendra_index: String,
    #[serde(rename = "queryFilterString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_filter_string: Option<String>,
    #[serde(rename = "queryFilterStringEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_filter_string_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutputContext {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "timeToLiveInSeconds")]
    #[serde(default)]
    pub time_to_live_in_seconds: i32,
    #[serde(rename = "turnsToLive")]
    #[serde(default)]
    pub turns_to_live: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QInConnectIntentConfiguration {
    #[serde(rename = "qInConnectAssistantConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q_in_connect_assistant_configuration: Option<QInConnectAssistantConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QInConnectAssistantConfiguration {
    #[serde(rename = "assistantArn")]
    #[serde(default)]
    pub assistant_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QnAIntentConfiguration {
    #[serde(rename = "bedrockModelConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bedrock_model_configuration: Option<BedrockModelSpecification>,
    #[serde(rename = "dataSourceConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_configuration: Option<DataSourceConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSourceConfiguration {
    #[serde(rename = "bedrockKnowledgeStoreConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bedrock_knowledge_store_configuration: Option<BedrockKnowledgeStoreConfiguration>,
    #[serde(rename = "kendraConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kendra_configuration: Option<QnAKendraConfiguration>,
    #[serde(rename = "opensearchConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opensearch_configuration: Option<OpensearchConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BedrockKnowledgeStoreConfiguration {
    #[serde(rename = "bedrockKnowledgeBaseArn")]
    #[serde(default)]
    pub bedrock_knowledge_base_arn: String,
    #[serde(rename = "exactResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact_response: Option<bool>,
    #[serde(rename = "exactResponseFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact_response_fields: Option<BedrockKnowledgeStoreExactResponseFields>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BedrockKnowledgeStoreExactResponseFields {
    #[serde(rename = "answerField")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer_field: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QnAKendraConfiguration {
    #[serde(rename = "exactResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact_response: Option<bool>,
    #[serde(rename = "kendraIndex")]
    #[serde(default)]
    pub kendra_index: String,
    #[serde(rename = "queryFilterString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_filter_string: Option<String>,
    #[serde(rename = "queryFilterStringEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_filter_string_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpensearchConfiguration {
    #[serde(rename = "domainEndpoint")]
    #[serde(default)]
    pub domain_endpoint: String,
    #[serde(rename = "exactResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact_response: Option<bool>,
    #[serde(rename = "exactResponseFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact_response_fields: Option<ExactResponseFields>,
    #[serde(rename = "includeFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_fields: Option<Vec<String>>,
    #[serde(rename = "indexName")]
    #[serde(default)]
    pub index_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExactResponseFields {
    #[serde(rename = "answerField")]
    #[serde(default)]
    pub answer_field: String,
    #[serde(rename = "questionField")]
    #[serde(default)]
    pub question_field: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SampleUtterance {
    #[serde(default)]
    pub utterance: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIntentResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "dialogCodeHook")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialog_code_hook: Option<DialogCodeHookSettings>,
    #[serde(rename = "fulfillmentCodeHook")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_code_hook: Option<FulfillmentCodeHookSettings>,
    #[serde(rename = "initialResponseSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_response_setting: Option<InitialResponseSetting>,
    #[serde(rename = "inputContexts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_contexts: Option<Vec<InputContext>>,
    #[serde(rename = "intentClosingSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_closing_setting: Option<IntentClosingSetting>,
    #[serde(rename = "intentConfirmationSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_confirmation_setting: Option<IntentConfirmationSetting>,
    #[serde(rename = "intentDisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_display_name: Option<String>,
    #[serde(rename = "intentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_id: Option<String>,
    #[serde(rename = "intentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_name: Option<String>,
    #[serde(rename = "kendraConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kendra_configuration: Option<KendraConfiguration>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(rename = "outputContexts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_contexts: Option<Vec<OutputContext>>,
    #[serde(rename = "parentIntentSignature")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_intent_signature: Option<String>,
    #[serde(rename = "qInConnectIntentConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q_in_connect_intent_configuration: Option<QInConnectIntentConfiguration>,
    #[serde(rename = "qnAIntentConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qn_a_intent_configuration: Option<QnAIntentConfiguration>,
    #[serde(rename = "sampleUtterances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_utterances: Option<Vec<SampleUtterance>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateResourcePolicyRequest {
    #[serde(default)]
    pub policy: String,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateResourcePolicyResponse {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "revisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateResourcePolicyStatementRequest {
    #[serde(default)]
    pub action: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition:
        Option<std::collections::HashMap<String, std::collections::HashMap<String, String>>>,
    #[serde(default)]
    pub effect: String,
    #[serde(rename = "expectedRevisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_revision_id: Option<String>,
    #[serde(default)]
    pub principal: Vec<Principal>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "statementId")]
    #[serde(default)]
    pub statement_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Principal {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateResourcePolicyStatementResponse {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "revisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSlotRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "intentId")]
    #[serde(default)]
    pub intent_id: String,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
    #[serde(rename = "multipleValuesSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_values_setting: Option<MultipleValuesSetting>,
    #[serde(rename = "obfuscationSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfuscation_setting: Option<ObfuscationSetting>,
    #[serde(rename = "slotName")]
    #[serde(default)]
    pub slot_name: String,
    #[serde(rename = "slotTypeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_type_id: Option<String>,
    #[serde(rename = "subSlotSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_slot_setting: Option<SubSlotSetting>,
    #[serde(rename = "valueElicitationSetting")]
    #[serde(default)]
    pub value_elicitation_setting: SlotValueElicitationSetting,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultipleValuesSetting {
    #[serde(rename = "allowMultipleValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_multiple_values: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ObfuscationSetting {
    #[serde(rename = "obfuscationSettingType")]
    #[serde(default)]
    pub obfuscation_setting_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubSlotSetting {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "slotSpecifications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_specifications: Option<std::collections::HashMap<String, Specifications>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Specifications {
    #[serde(rename = "slotTypeId")]
    #[serde(default)]
    pub slot_type_id: String,
    #[serde(rename = "valueElicitationSetting")]
    #[serde(default)]
    pub value_elicitation_setting: SubSlotValueElicitationSetting,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubSlotValueElicitationSetting {
    #[serde(rename = "defaultValueSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value_specification: Option<SlotDefaultValueSpecification>,
    #[serde(rename = "promptSpecification")]
    #[serde(default)]
    pub prompt_specification: PromptSpecification,
    #[serde(rename = "sampleUtterances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_utterances: Option<Vec<SampleUtterance>>,
    #[serde(rename = "waitAndContinueSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_and_continue_specification: Option<WaitAndContinueSpecification>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SlotDefaultValueSpecification {
    #[serde(rename = "defaultValueList")]
    #[serde(default)]
    pub default_value_list: Vec<SlotDefaultValue>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SlotDefaultValue {
    #[serde(rename = "defaultValue")]
    #[serde(default)]
    pub default_value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WaitAndContinueSpecification {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "continueResponse")]
    #[serde(default)]
    pub continue_response: ResponseSpecification,
    #[serde(rename = "stillWaitingResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub still_waiting_response: Option<StillWaitingResponseSpecification>,
    #[serde(rename = "waitingResponse")]
    #[serde(default)]
    pub waiting_response: ResponseSpecification,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StillWaitingResponseSpecification {
    #[serde(rename = "allowInterrupt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_interrupt: Option<bool>,
    #[serde(rename = "frequencyInSeconds")]
    #[serde(default)]
    pub frequency_in_seconds: i32,
    #[serde(rename = "messageGroups")]
    #[serde(default)]
    pub message_groups: Vec<MessageGroup>,
    #[serde(rename = "timeoutInSeconds")]
    #[serde(default)]
    pub timeout_in_seconds: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SlotValueElicitationSetting {
    #[serde(rename = "defaultValueSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value_specification: Option<SlotDefaultValueSpecification>,
    #[serde(rename = "promptSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_specification: Option<PromptSpecification>,
    #[serde(rename = "sampleUtterances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_utterances: Option<Vec<SampleUtterance>>,
    #[serde(rename = "slotCaptureSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_capture_setting: Option<SlotCaptureSetting>,
    #[serde(rename = "slotConstraint")]
    #[serde(default)]
    pub slot_constraint: String,
    #[serde(rename = "slotResolutionSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_resolution_setting: Option<SlotResolutionSetting>,
    #[serde(rename = "waitAndContinueSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_and_continue_specification: Option<WaitAndContinueSpecification>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SlotCaptureSetting {
    #[serde(rename = "captureConditional")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_conditional: Option<ConditionalSpecification>,
    #[serde(rename = "captureNextStep")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_next_step: Option<DialogState>,
    #[serde(rename = "captureResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_response: Option<ResponseSpecification>,
    #[serde(rename = "codeHook")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_hook: Option<DialogCodeHookInvocationSetting>,
    #[serde(rename = "elicitationCodeHook")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elicitation_code_hook: Option<ElicitationCodeHookInvocationSetting>,
    #[serde(rename = "failureConditional")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_conditional: Option<ConditionalSpecification>,
    #[serde(rename = "failureNextStep")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_next_step: Option<DialogState>,
    #[serde(rename = "failureResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_response: Option<ResponseSpecification>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SlotResolutionSetting {
    #[serde(rename = "slotResolutionStrategy")]
    #[serde(default)]
    pub slot_resolution_strategy: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSlotResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "intentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_id: Option<String>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(rename = "multipleValuesSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_values_setting: Option<MultipleValuesSetting>,
    #[serde(rename = "obfuscationSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfuscation_setting: Option<ObfuscationSetting>,
    #[serde(rename = "slotId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_id: Option<String>,
    #[serde(rename = "slotName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_name: Option<String>,
    #[serde(rename = "slotTypeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_type_id: Option<String>,
    #[serde(rename = "subSlotSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_slot_setting: Option<SubSlotSetting>,
    #[serde(rename = "valueElicitationSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_elicitation_setting: Option<SlotValueElicitationSetting>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSlotTypeRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(rename = "compositeSlotTypeSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite_slot_type_setting: Option<CompositeSlotTypeSetting>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "externalSourceSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_source_setting: Option<ExternalSourceSetting>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
    #[serde(rename = "parentSlotTypeSignature")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_slot_type_signature: Option<String>,
    #[serde(rename = "slotTypeName")]
    #[serde(default)]
    pub slot_type_name: String,
    #[serde(rename = "slotTypeValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_type_values: Option<Vec<SlotTypeValue>>,
    #[serde(rename = "valueSelectionSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_selection_setting: Option<SlotValueSelectionSetting>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CompositeSlotTypeSetting {
    #[serde(rename = "subSlots")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_slots: Option<Vec<SubSlotTypeComposition>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubSlotTypeComposition {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "slotTypeId")]
    #[serde(default)]
    pub slot_type_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExternalSourceSetting {
    #[serde(rename = "grammarSlotTypeSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grammar_slot_type_setting: Option<GrammarSlotTypeSetting>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GrammarSlotTypeSetting {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<GrammarSlotTypeSource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GrammarSlotTypeSource {
    #[serde(rename = "kmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "s3BucketName")]
    #[serde(default)]
    pub s3_bucket_name: String,
    #[serde(rename = "s3ObjectKey")]
    #[serde(default)]
    pub s3_object_key: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SlotTypeValue {
    #[serde(rename = "sampleValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_value: Option<SampleValue>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synonyms: Option<Vec<SampleValue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SampleValue {
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SlotValueSelectionSetting {
    #[serde(rename = "advancedRecognitionSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_recognition_setting: Option<AdvancedRecognitionSetting>,
    #[serde(rename = "regexFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_filter: Option<SlotValueRegexFilter>,
    #[serde(rename = "resolutionStrategy")]
    #[serde(default)]
    pub resolution_strategy: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdvancedRecognitionSetting {
    #[serde(rename = "audioRecognitionStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_recognition_strategy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SlotValueRegexFilter {
    #[serde(default)]
    pub pattern: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSlotTypeResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "compositeSlotTypeSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite_slot_type_setting: Option<CompositeSlotTypeSetting>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "externalSourceSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_source_setting: Option<ExternalSourceSetting>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(rename = "parentSlotTypeSignature")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_slot_type_signature: Option<String>,
    #[serde(rename = "slotTypeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_type_id: Option<String>,
    #[serde(rename = "slotTypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_type_name: Option<String>,
    #[serde(rename = "slotTypeValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_type_values: Option<Vec<SlotTypeValue>>,
    #[serde(rename = "valueSelectionSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_selection_setting: Option<SlotValueSelectionSetting>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTestSetDiscrepancyReportRequest {
    #[serde(default)]
    pub target: TestSetDiscrepancyReportResourceTarget,
    #[serde(rename = "testSetId")]
    #[serde(default)]
    pub test_set_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestSetDiscrepancyReportResourceTarget {
    #[serde(rename = "botAliasTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_alias_target: Option<TestSetDiscrepancyReportBotAliasTarget>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestSetDiscrepancyReportBotAliasTarget {
    #[serde(rename = "botAliasId")]
    #[serde(default)]
    pub bot_alias_id: String,
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTestSetDiscrepancyReportResponse {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<TestSetDiscrepancyReportResourceTarget>,
    #[serde(rename = "testSetDiscrepancyReportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_set_discrepancy_report_id: Option<String>,
    #[serde(rename = "testSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_set_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUploadUrlRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUploadUrlResponse {
    #[serde(rename = "importId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_id: Option<String>,
    #[serde(rename = "uploadUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBotAliasRequest {
    #[serde(rename = "botAliasId")]
    #[serde(default)]
    pub bot_alias_id: String,
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "skipResourceInUseCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_resource_in_use_check: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBotAliasResponse {
    #[serde(rename = "botAliasId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_alias_id: Option<String>,
    #[serde(rename = "botAliasStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_alias_status: Option<String>,
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBotAnalyzerRecommendationRequest {
    #[serde(rename = "botAnalyzerRequestId")]
    #[serde(default)]
    pub bot_analyzer_request_id: String,
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBotAnalyzerRecommendationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBotLocaleRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBotLocaleResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botLocaleStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_locale_status: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBotReplicaRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "replicaRegion")]
    #[serde(default)]
    pub replica_region: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBotReplicaResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botReplicaStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_replica_status: Option<String>,
    #[serde(rename = "replicaRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBotRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "skipResourceInUseCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_resource_in_use_check: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBotResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBotVersionRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(rename = "skipResourceInUseCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_resource_in_use_check: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBotVersionResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_status: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCustomVocabularyRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCustomVocabularyResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "customVocabularyStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_vocabulary_status: Option<String>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteExportRequest {
    #[serde(rename = "exportId")]
    #[serde(default)]
    pub export_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteExportResponse {
    #[serde(rename = "exportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_id: Option<String>,
    #[serde(rename = "exportStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteImportRequest {
    #[serde(rename = "importId")]
    #[serde(default)]
    pub import_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteImportResponse {
    #[serde(rename = "importId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_id: Option<String>,
    #[serde(rename = "importStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIntentRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(rename = "intentId")]
    #[serde(default)]
    pub intent_id: String,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourcePolicyRequest {
    #[serde(rename = "expectedRevisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_revision_id: Option<String>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourcePolicyResponse {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "revisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourcePolicyStatementRequest {
    #[serde(rename = "expectedRevisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_revision_id: Option<String>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "statementId")]
    #[serde(default)]
    pub statement_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourcePolicyStatementResponse {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "revisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSlotRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(rename = "intentId")]
    #[serde(default)]
    pub intent_id: String,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
    #[serde(rename = "slotId")]
    #[serde(default)]
    pub slot_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSlotTypeRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
    #[serde(rename = "skipResourceInUseCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_resource_in_use_check: Option<bool>,
    #[serde(rename = "slotTypeId")]
    #[serde(default)]
    pub slot_type_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTestSetRequest {
    #[serde(rename = "testSetId")]
    #[serde(default)]
    pub test_set_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUtterancesRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(rename = "sessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUtterancesResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBotAliasRequest {
    #[serde(rename = "botAliasId")]
    #[serde(default)]
    pub bot_alias_id: String,
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBotAliasResponse {
    #[serde(rename = "botAliasHistoryEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_alias_history_events: Option<Vec<BotAliasHistoryEvent>>,
    #[serde(rename = "botAliasId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_alias_id: Option<String>,
    #[serde(rename = "botAliasLocaleSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_alias_locale_settings:
        Option<std::collections::HashMap<String, BotAliasLocaleSettings>>,
    #[serde(rename = "botAliasName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_alias_name: Option<String>,
    #[serde(rename = "botAliasStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_alias_status: Option<String>,
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "conversationLogSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_log_settings: Option<ConversationLogSettings>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "parentBotNetworks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_bot_networks: Option<Vec<ParentBotNetwork>>,
    #[serde(rename = "sentimentAnalysisSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiment_analysis_settings: Option<SentimentAnalysisSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BotAliasHistoryEvent {
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "endDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<f64>,
    #[serde(rename = "startDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParentBotNetwork {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBotAnalyzerRecommendationRequest {
    #[serde(rename = "botAnalyzerRequestId")]
    #[serde(default)]
    pub bot_analyzer_request_id: String,
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBotAnalyzerRecommendationResponse {
    #[serde(rename = "botAnalyzerRecommendationList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_analyzer_recommendation_list: Option<Vec<BotAnalyzerRecommendation>>,
    #[serde(rename = "botAnalyzerStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_analyzer_status: Option<String>,
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BotAnalyzerRecommendation {
    #[serde(rename = "issueDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_description: Option<String>,
    #[serde(rename = "issueLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue_location: Option<IssueLocation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[serde(rename = "proposedFix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proposed_fix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IssueLocation {
    #[serde(rename = "botLocale")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_locale: Option<String>,
    #[serde(rename = "intentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_id: Option<String>,
    #[serde(rename = "slotId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBotLocaleRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBotLocaleResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botLocaleHistoryEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_locale_history_events: Option<Vec<BotLocaleHistoryEvent>>,
    #[serde(rename = "botLocaleStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_locale_status: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "failureReasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reasons: Option<Vec<String>>,
    #[serde(rename = "generativeAISettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generative_a_i_settings: Option<GenerativeAISettings>,
    #[serde(rename = "intentsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intents_count: Option<i32>,
    #[serde(rename = "lastBuildSubmittedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_build_submitted_date_time: Option<f64>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(rename = "localeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_name: Option<String>,
    #[serde(rename = "nluIntentConfidenceThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nlu_intent_confidence_threshold: Option<f64>,
    #[serde(rename = "recommendedActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_actions: Option<Vec<String>>,
    #[serde(rename = "slotTypesCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_types_count: Option<i32>,
    #[serde(rename = "speechDetectionSensitivity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_detection_sensitivity: Option<String>,
    #[serde(rename = "speechRecognitionSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_recognition_settings: Option<SpeechRecognitionSettings>,
    #[serde(rename = "unifiedSpeechSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unified_speech_settings: Option<UnifiedSpeechSettings>,
    #[serde(rename = "voiceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_settings: Option<VoiceSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BotLocaleHistoryEvent {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    #[serde(rename = "eventDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_date: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBotRecommendationRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botRecommendationId")]
    #[serde(default)]
    pub bot_recommendation_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBotRecommendationResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botRecommendationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_recommendation_id: Option<String>,
    #[serde(rename = "botRecommendationResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_recommendation_results: Option<BotRecommendationResults>,
    #[serde(rename = "botRecommendationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_recommendation_status: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "encryptionSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_setting: Option<EncryptionSetting>,
    #[serde(rename = "failureReasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reasons: Option<Vec<String>>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(rename = "transcriptSourceSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript_source_setting: Option<TranscriptSourceSetting>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BotRecommendationResults {
    #[serde(rename = "associatedTranscriptsUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_transcripts_url: Option<String>,
    #[serde(rename = "botLocaleExportUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_locale_export_url: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<BotRecommendationResultStatistics>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BotRecommendationResultStatistics {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intents: Option<IntentStatistics>,
    #[serde(rename = "slotTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_types: Option<SlotTypeStatistics>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntentStatistics {
    #[serde(rename = "discoveredIntentCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovered_intent_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SlotTypeStatistics {
    #[serde(rename = "discoveredSlotTypeCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discovered_slot_type_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptionSetting {
    #[serde(rename = "associatedTranscriptsPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_transcripts_password: Option<String>,
    #[serde(rename = "botLocaleExportPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_locale_export_password: Option<String>,
    #[serde(rename = "kmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TranscriptSourceSetting {
    #[serde(rename = "s3BucketTranscriptSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_transcript_source: Option<S3BucketTranscriptSource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3BucketTranscriptSource {
    #[serde(rename = "kmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "pathFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_format: Option<PathFormat>,
    #[serde(rename = "s3BucketName")]
    #[serde(default)]
    pub s3_bucket_name: String,
    #[serde(rename = "transcriptFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript_filter: Option<TranscriptFilter>,
    #[serde(rename = "transcriptFormat")]
    #[serde(default)]
    pub transcript_format: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PathFormat {
    #[serde(rename = "objectPrefixes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_prefixes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TranscriptFilter {
    #[serde(rename = "lexTranscriptFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lex_transcript_filter: Option<LexTranscriptFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LexTranscriptFilter {
    #[serde(rename = "dateRangeFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_range_filter: Option<DateRangeFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DateRangeFilter {
    #[serde(rename = "endDateTime")]
    #[serde(default)]
    pub end_date_time: f64,
    #[serde(rename = "startDateTime")]
    #[serde(default)]
    pub start_date_time: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBotReplicaRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "replicaRegion")]
    #[serde(default)]
    pub replica_region: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBotReplicaResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botReplicaStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_replica_status: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "failureReasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reasons: Option<Vec<String>>,
    #[serde(rename = "replicaRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_region: Option<String>,
    #[serde(rename = "sourceRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBotRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBotResourceGenerationRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(rename = "generationId")]
    #[serde(default)]
    pub generation_id: String,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBotResourceGenerationResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "failureReasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reasons: Option<Vec<String>>,
    #[serde(rename = "generatedBotLocaleUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_bot_locale_url: Option<String>,
    #[serde(rename = "generationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_id: Option<String>,
    #[serde(rename = "generationInputPrompt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_input_prompt: Option<String>,
    #[serde(rename = "generationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_status: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(rename = "modelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBotResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botMembers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_members: Option<Vec<BotMember>>,
    #[serde(rename = "botName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_name: Option<String>,
    #[serde(rename = "botStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_status: Option<String>,
    #[serde(rename = "botType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_type: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "dataPrivacy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_privacy: Option<DataPrivacy>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "errorLogSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_log_settings: Option<ErrorLogSettings>,
    #[serde(rename = "failureReasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reasons: Option<Vec<String>>,
    #[serde(rename = "idleSessionTTLInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_session_t_t_l_in_seconds: Option<i32>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBotVersionRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBotVersionResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botMembers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_members: Option<Vec<BotMember>>,
    #[serde(rename = "botName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_name: Option<String>,
    #[serde(rename = "botStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_status: Option<String>,
    #[serde(rename = "botType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_type: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "dataPrivacy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_privacy: Option<DataPrivacy>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "failureReasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reasons: Option<Vec<String>>,
    #[serde(rename = "idleSessionTTLInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_session_t_t_l_in_seconds: Option<i32>,
    #[serde(rename = "parentBotNetworks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_bot_networks: Option<Vec<ParentBotNetwork>>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCustomVocabularyMetadataRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCustomVocabularyMetadataResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "customVocabularyStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_vocabulary_status: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeExportRequest {
    #[serde(rename = "exportId")]
    #[serde(default)]
    pub export_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeExportResponse {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "downloadUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    #[serde(rename = "exportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_id: Option<String>,
    #[serde(rename = "exportStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_status: Option<String>,
    #[serde(rename = "failureReasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reasons: Option<Vec<String>>,
    #[serde(rename = "fileFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_format: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "resourceSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_specification: Option<ExportResourceSpecification>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeImportRequest {
    #[serde(rename = "importId")]
    #[serde(default)]
    pub import_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeImportResponse {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "failureReasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reasons: Option<Vec<String>>,
    #[serde(rename = "importId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_id: Option<String>,
    #[serde(rename = "importStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_status: Option<String>,
    #[serde(rename = "importedResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_resource_id: Option<String>,
    #[serde(rename = "importedResourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_resource_name: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "mergeStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_strategy: Option<String>,
    #[serde(rename = "resourceSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_specification: Option<ImportResourceSpecification>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportResourceSpecification {
    #[serde(rename = "botImportSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_import_specification: Option<BotImportSpecification>,
    #[serde(rename = "botLocaleImportSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_locale_import_specification: Option<BotLocaleImportSpecification>,
    #[serde(rename = "customVocabularyImportSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_vocabulary_import_specification: Option<CustomVocabularyImportSpecification>,
    #[serde(rename = "testSetImportResourceSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_set_import_resource_specification: Option<TestSetImportResourceSpecification>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BotImportSpecification {
    #[serde(rename = "botName")]
    #[serde(default)]
    pub bot_name: String,
    #[serde(rename = "botTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "dataPrivacy")]
    #[serde(default)]
    pub data_privacy: DataPrivacy,
    #[serde(rename = "errorLogSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_log_settings: Option<ErrorLogSettings>,
    #[serde(rename = "idleSessionTTLInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_session_t_t_l_in_seconds: Option<i32>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "testBotAliasTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_bot_alias_tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BotLocaleImportSpecification {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
    #[serde(rename = "nluIntentConfidenceThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nlu_intent_confidence_threshold: Option<f64>,
    #[serde(rename = "speechDetectionSensitivity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_detection_sensitivity: Option<String>,
    #[serde(rename = "speechRecognitionSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_recognition_settings: Option<SpeechRecognitionSettings>,
    #[serde(rename = "unifiedSpeechSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unified_speech_settings: Option<UnifiedSpeechSettings>,
    #[serde(rename = "voiceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_settings: Option<VoiceSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomVocabularyImportSpecification {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestSetImportResourceSpecification {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "importInputLocation")]
    #[serde(default)]
    pub import_input_location: TestSetImportInputLocation,
    #[serde(default)]
    pub modality: String,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "storageLocation")]
    #[serde(default)]
    pub storage_location: TestSetStorageLocation,
    #[serde(rename = "testSetName")]
    #[serde(default)]
    pub test_set_name: String,
    #[serde(rename = "testSetTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_set_tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestSetImportInputLocation {
    #[serde(rename = "s3BucketName")]
    #[serde(default)]
    pub s3_bucket_name: String,
    #[serde(rename = "s3Path")]
    #[serde(default)]
    pub s3_path: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestSetStorageLocation {
    #[serde(rename = "kmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "s3BucketName")]
    #[serde(default)]
    pub s3_bucket_name: String,
    #[serde(rename = "s3Path")]
    #[serde(default)]
    pub s3_path: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeIntentRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(rename = "intentId")]
    #[serde(default)]
    pub intent_id: String,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeIntentResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "dialogCodeHook")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialog_code_hook: Option<DialogCodeHookSettings>,
    #[serde(rename = "fulfillmentCodeHook")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_code_hook: Option<FulfillmentCodeHookSettings>,
    #[serde(rename = "initialResponseSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_response_setting: Option<InitialResponseSetting>,
    #[serde(rename = "inputContexts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_contexts: Option<Vec<InputContext>>,
    #[serde(rename = "intentClosingSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_closing_setting: Option<IntentClosingSetting>,
    #[serde(rename = "intentConfirmationSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_confirmation_setting: Option<IntentConfirmationSetting>,
    #[serde(rename = "intentDisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_display_name: Option<String>,
    #[serde(rename = "intentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_id: Option<String>,
    #[serde(rename = "intentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_name: Option<String>,
    #[serde(rename = "kendraConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kendra_configuration: Option<KendraConfiguration>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(rename = "outputContexts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_contexts: Option<Vec<OutputContext>>,
    #[serde(rename = "parentIntentSignature")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_intent_signature: Option<String>,
    #[serde(rename = "qInConnectIntentConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q_in_connect_intent_configuration: Option<QInConnectIntentConfiguration>,
    #[serde(rename = "qnAIntentConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qn_a_intent_configuration: Option<QnAIntentConfiguration>,
    #[serde(rename = "sampleUtterances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_utterances: Option<Vec<SampleUtterance>>,
    #[serde(rename = "slotPriorities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_priorities: Option<Vec<SlotPriority>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SlotPriority {
    #[serde(default)]
    pub priority: i32,
    #[serde(rename = "slotId")]
    #[serde(default)]
    pub slot_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeResourcePolicyRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeResourcePolicyResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "revisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSlotRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(rename = "intentId")]
    #[serde(default)]
    pub intent_id: String,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
    #[serde(rename = "slotId")]
    #[serde(default)]
    pub slot_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSlotResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "intentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_id: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(rename = "multipleValuesSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_values_setting: Option<MultipleValuesSetting>,
    #[serde(rename = "obfuscationSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfuscation_setting: Option<ObfuscationSetting>,
    #[serde(rename = "slotId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_id: Option<String>,
    #[serde(rename = "slotName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_name: Option<String>,
    #[serde(rename = "slotTypeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_type_id: Option<String>,
    #[serde(rename = "subSlotSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_slot_setting: Option<SubSlotSetting>,
    #[serde(rename = "valueElicitationSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_elicitation_setting: Option<SlotValueElicitationSetting>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSlotTypeRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
    #[serde(rename = "slotTypeId")]
    #[serde(default)]
    pub slot_type_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSlotTypeResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "compositeSlotTypeSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite_slot_type_setting: Option<CompositeSlotTypeSetting>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "externalSourceSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_source_setting: Option<ExternalSourceSetting>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(rename = "parentSlotTypeSignature")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_slot_type_signature: Option<String>,
    #[serde(rename = "slotTypeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_type_id: Option<String>,
    #[serde(rename = "slotTypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_type_name: Option<String>,
    #[serde(rename = "slotTypeValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_type_values: Option<Vec<SlotTypeValue>>,
    #[serde(rename = "valueSelectionSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_selection_setting: Option<SlotValueSelectionSetting>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTestExecutionRequest {
    #[serde(rename = "testExecutionId")]
    #[serde(default)]
    pub test_execution_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTestExecutionResponse {
    #[serde(rename = "apiMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_mode: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "failureReasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reasons: Option<Vec<String>>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<TestExecutionTarget>,
    #[serde(rename = "testExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_execution_id: Option<String>,
    #[serde(rename = "testExecutionModality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_execution_modality: Option<String>,
    #[serde(rename = "testExecutionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_execution_status: Option<String>,
    #[serde(rename = "testSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_set_id: Option<String>,
    #[serde(rename = "testSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_set_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestExecutionTarget {
    #[serde(rename = "botAliasTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_alias_target: Option<BotAliasTestExecutionTarget>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BotAliasTestExecutionTarget {
    #[serde(rename = "botAliasId")]
    #[serde(default)]
    pub bot_alias_id: String,
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTestSetDiscrepancyReportRequest {
    #[serde(rename = "testSetDiscrepancyReportId")]
    #[serde(default)]
    pub test_set_discrepancy_report_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTestSetDiscrepancyReportResponse {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "failureReasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reasons: Option<Vec<String>>,
    #[serde(rename = "lastUpdatedDataTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_data_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<TestSetDiscrepancyReportResourceTarget>,
    #[serde(rename = "testSetDiscrepancyRawOutputUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_set_discrepancy_raw_output_url: Option<String>,
    #[serde(rename = "testSetDiscrepancyReportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_set_discrepancy_report_id: Option<String>,
    #[serde(rename = "testSetDiscrepancyReportStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_set_discrepancy_report_status: Option<String>,
    #[serde(rename = "testSetDiscrepancyTopErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_set_discrepancy_top_errors: Option<TestSetDiscrepancyErrors>,
    #[serde(rename = "testSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_set_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestSetDiscrepancyErrors {
    #[serde(rename = "intentDiscrepancies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_discrepancies: Option<Vec<TestSetIntentDiscrepancyItem>>,
    #[serde(rename = "slotDiscrepancies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_discrepancies: Option<Vec<TestSetSlotDiscrepancyItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestSetIntentDiscrepancyItem {
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "intentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestSetSlotDiscrepancyItem {
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "intentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_name: Option<String>,
    #[serde(rename = "slotName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTestSetGenerationRequest {
    #[serde(rename = "testSetGenerationId")]
    #[serde(default)]
    pub test_set_generation_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTestSetGenerationResponse {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "failureReasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reasons: Option<Vec<String>>,
    #[serde(rename = "generationDataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_data_source: Option<TestSetGenerationDataSource>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "storageLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_location: Option<TestSetStorageLocation>,
    #[serde(rename = "testSetGenerationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_set_generation_id: Option<String>,
    #[serde(rename = "testSetGenerationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_set_generation_status: Option<String>,
    #[serde(rename = "testSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_set_id: Option<String>,
    #[serde(rename = "testSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_set_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestSetGenerationDataSource {
    #[serde(rename = "conversationLogsDataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_logs_data_source: Option<ConversationLogsDataSource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConversationLogsDataSource {
    #[serde(rename = "botAliasId")]
    #[serde(default)]
    pub bot_alias_id: String,
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(default)]
    pub filter: ConversationLogsDataSourceFilterBy,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConversationLogsDataSourceFilterBy {
    #[serde(rename = "endTime")]
    #[serde(default)]
    pub end_time: f64,
    #[serde(rename = "inputMode")]
    #[serde(default)]
    pub input_mode: String,
    #[serde(rename = "startTime")]
    #[serde(default)]
    pub start_time: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTestSetRequest {
    #[serde(rename = "testSetId")]
    #[serde(default)]
    pub test_set_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTestSetResponse {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modality: Option<String>,
    #[serde(rename = "numTurns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_turns: Option<i32>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "storageLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_location: Option<TestSetStorageLocation>,
    #[serde(rename = "testSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_set_id: Option<String>,
    #[serde(rename = "testSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_set_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GenerateBotElementRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(rename = "intentId")]
    #[serde(default)]
    pub intent_id: String,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GenerateBotElementResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "intentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_id: Option<String>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(rename = "sampleUtterances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_utterances: Option<Vec<SampleUtterance>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTestExecutionArtifactsUrlRequest {
    #[serde(rename = "testExecutionId")]
    #[serde(default)]
    pub test_execution_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTestExecutionArtifactsUrlResponse {
    #[serde(rename = "downloadArtifactsUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_artifacts_url: Option<String>,
    #[serde(rename = "testExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_execution_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAggregatedUtterancesRequest {
    #[serde(rename = "aggregationDuration")]
    #[serde(default)]
    pub aggregation_duration: UtteranceAggregationDuration,
    #[serde(rename = "botAliasId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_alias_id: Option<String>,
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<AggregatedUtterancesFilter>>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<AggregatedUtterancesSortBy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UtteranceAggregationDuration {
    #[serde(rename = "relativeAggregationDuration")]
    #[serde(default)]
    pub relative_aggregation_duration: RelativeAggregationDuration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RelativeAggregationDuration {
    #[serde(rename = "timeDimension")]
    #[serde(default)]
    pub time_dimension: String,
    #[serde(rename = "timeValue")]
    #[serde(default)]
    pub time_value: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AggregatedUtterancesFilter {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub operator: String,
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AggregatedUtterancesSortBy {
    #[serde(default)]
    pub attribute: String,
    #[serde(default)]
    pub order: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAggregatedUtterancesResponse {
    #[serde(rename = "aggregatedUtterancesSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregated_utterances_summaries: Option<Vec<AggregatedUtterancesSummary>>,
    #[serde(rename = "aggregationDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_duration: Option<UtteranceAggregationDuration>,
    #[serde(rename = "aggregationLastRefreshedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_last_refreshed_date_time: Option<f64>,
    #[serde(rename = "aggregationWindowEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_window_end_time: Option<f64>,
    #[serde(rename = "aggregationWindowStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_window_start_time: Option<f64>,
    #[serde(rename = "botAliasId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_alias_id: Option<String>,
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AggregatedUtterancesSummary {
    #[serde(rename = "containsDataFromDeletedResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains_data_from_deleted_resources: Option<bool>,
    #[serde(rename = "hitCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit_count: Option<i32>,
    #[serde(rename = "missedCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missed_count: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utterance: Option<String>,
    #[serde(rename = "utteranceFirstRecordedInAggregationDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utterance_first_recorded_in_aggregation_duration: Option<f64>,
    #[serde(rename = "utteranceLastRecordedInAggregationDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utterance_last_recorded_in_aggregation_duration: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBotAliasReplicasRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "replicaRegion")]
    #[serde(default)]
    pub replica_region: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBotAliasReplicasResponse {
    #[serde(rename = "botAliasReplicaSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_alias_replica_summaries: Option<Vec<BotAliasReplicaSummary>>,
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "replicaRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_region: Option<String>,
    #[serde(rename = "sourceRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BotAliasReplicaSummary {
    #[serde(rename = "botAliasId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_alias_id: Option<String>,
    #[serde(rename = "botAliasReplicationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_alias_replication_status: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "failureReasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reasons: Option<Vec<String>>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBotAliasesRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBotAliasesResponse {
    #[serde(rename = "botAliasSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_alias_summaries: Option<Vec<BotAliasSummary>>,
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BotAliasSummary {
    #[serde(rename = "botAliasId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_alias_id: Option<String>,
    #[serde(rename = "botAliasName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_alias_name: Option<String>,
    #[serde(rename = "botAliasStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_alias_status: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBotAnalyzerHistoryRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBotAnalyzerHistoryResponse {
    #[serde(rename = "botAnalyzerHistoryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_analyzer_history_list: Option<Vec<BotAnalyzerHistorySummary>>,
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BotAnalyzerHistorySummary {
    #[serde(rename = "botAnalyzerRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_analyzer_request_id: Option<String>,
    #[serde(rename = "botAnalyzerStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_analyzer_status: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBotLocalesRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<BotLocaleFilter>>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<BotLocaleSortBy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BotLocaleFilter {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub operator: String,
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BotLocaleSortBy {
    #[serde(default)]
    pub attribute: String,
    #[serde(default)]
    pub order: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBotLocalesResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botLocaleSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_locale_summaries: Option<Vec<BotLocaleSummary>>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BotLocaleSummary {
    #[serde(rename = "botLocaleStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_locale_status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "lastBuildSubmittedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_build_submitted_date_time: Option<f64>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(rename = "localeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBotRecommendationsRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBotRecommendationsResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botRecommendationSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_recommendation_summaries: Option<Vec<BotRecommendationSummary>>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BotRecommendationSummary {
    #[serde(rename = "botRecommendationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_recommendation_id: Option<String>,
    #[serde(rename = "botRecommendationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_recommendation_status: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBotReplicasRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBotReplicasResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botReplicaSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_replica_summaries: Option<Vec<BotReplicaSummary>>,
    #[serde(rename = "sourceRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BotReplicaSummary {
    #[serde(rename = "botReplicaStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_replica_status: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "failureReasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reasons: Option<Vec<String>>,
    #[serde(rename = "replicaRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBotResourceGenerationsRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<GenerationSortBy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GenerationSortBy {
    #[serde(default)]
    pub attribute: String,
    #[serde(default)]
    pub order: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBotResourceGenerationsResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "generationSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_summaries: Option<Vec<GenerationSummary>>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GenerationSummary {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "generationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_id: Option<String>,
    #[serde(rename = "generationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_status: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBotVersionReplicasRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "replicaRegion")]
    #[serde(default)]
    pub replica_region: String,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<BotVersionReplicaSortBy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BotVersionReplicaSortBy {
    #[serde(default)]
    pub attribute: String,
    #[serde(default)]
    pub order: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBotVersionReplicasResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botVersionReplicaSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version_replica_summaries: Option<Vec<BotVersionReplicaSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "replicaRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replica_region: Option<String>,
    #[serde(rename = "sourceRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BotVersionReplicaSummary {
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "botVersionReplicationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version_replication_status: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "failureReasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reasons: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBotVersionsRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<BotVersionSortBy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BotVersionSortBy {
    #[serde(default)]
    pub attribute: String,
    #[serde(default)]
    pub order: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBotVersionsResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botVersionSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version_summaries: Option<Vec<BotVersionSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BotVersionSummary {
    #[serde(rename = "botName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_name: Option<String>,
    #[serde(rename = "botStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_status: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBotsRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<BotFilter>>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<BotSortBy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BotFilter {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub operator: String,
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BotSortBy {
    #[serde(default)]
    pub attribute: String,
    #[serde(default)]
    pub order: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBotsResponse {
    #[serde(rename = "botSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_summaries: Option<Vec<BotSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BotSummary {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_name: Option<String>,
    #[serde(rename = "botStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_status: Option<String>,
    #[serde(rename = "botType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "latestBotVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_bot_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBuiltInIntentsRequest {
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<BuiltInIntentSortBy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BuiltInIntentSortBy {
    #[serde(default)]
    pub attribute: String,
    #[serde(default)]
    pub order: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBuiltInIntentsResponse {
    #[serde(rename = "builtInIntentSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub built_in_intent_summaries: Option<Vec<BuiltInIntentSummary>>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BuiltInIntentSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "intentSignature")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_signature: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBuiltInSlotTypesRequest {
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<BuiltInSlotTypeSortBy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BuiltInSlotTypeSortBy {
    #[serde(default)]
    pub attribute: String,
    #[serde(default)]
    pub order: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBuiltInSlotTypesResponse {
    #[serde(rename = "builtInSlotTypeSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub built_in_slot_type_summaries: Option<Vec<BuiltInSlotTypeSummary>>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BuiltInSlotTypeSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "slotTypeSignature")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_type_signature: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCustomVocabularyItemsRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCustomVocabularyItemsResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "customVocabularyItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_vocabulary_items: Option<Vec<CustomVocabularyItem>>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListExportsRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ExportFilter>>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<ExportSortBy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportFilter {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub operator: String,
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportSortBy {
    #[serde(default)]
    pub attribute: String,
    #[serde(default)]
    pub order: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListExportsResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "exportSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_summaries: Option<Vec<ExportSummary>>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportSummary {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "exportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_id: Option<String>,
    #[serde(rename = "exportStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_status: Option<String>,
    #[serde(rename = "fileFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_format: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "resourceSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_specification: Option<ExportResourceSpecification>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListImportsRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ImportFilter>>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<ImportSortBy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportFilter {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub operator: String,
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportSortBy {
    #[serde(default)]
    pub attribute: String,
    #[serde(default)]
    pub order: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListImportsResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "importSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_summaries: Option<Vec<ImportSummary>>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportSummary {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "importId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_id: Option<String>,
    #[serde(rename = "importStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_status: Option<String>,
    #[serde(rename = "importedResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_resource_id: Option<String>,
    #[serde(rename = "importedResourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_resource_name: Option<String>,
    #[serde(rename = "importedResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_resource_type: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "mergeStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_strategy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIntentMetricsRequest {
    #[serde(rename = "binBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_by: Option<Vec<AnalyticsBinBySpecification>>,
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "endDateTime")]
    #[serde(default)]
    pub end_date_time: f64,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<AnalyticsIntentFilter>>,
    #[serde(rename = "groupBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<AnalyticsIntentGroupBySpecification>>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(default)]
    pub metrics: Vec<AnalyticsIntentMetric>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "startDateTime")]
    #[serde(default)]
    pub start_date_time: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyticsBinBySpecification {
    #[serde(default)]
    pub interval: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyticsIntentFilter {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub operator: String,
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyticsIntentGroupBySpecification {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyticsIntentMetric {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(default)]
    pub statistic: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIntentMetricsResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<AnalyticsIntentResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyticsIntentResult {
    #[serde(rename = "binKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_keys: Option<Vec<AnalyticsBinKey>>,
    #[serde(rename = "groupByKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by_keys: Option<Vec<AnalyticsIntentGroupByKey>>,
    #[serde(rename = "metricsResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_results: Option<Vec<AnalyticsIntentMetricResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyticsBinKey {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyticsIntentGroupByKey {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyticsIntentMetricResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIntentPathsRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "endDateTime")]
    #[serde(default)]
    pub end_date_time: f64,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<AnalyticsPathFilter>>,
    #[serde(rename = "intentPath")]
    #[serde(default)]
    pub intent_path: String,
    #[serde(rename = "startDateTime")]
    #[serde(default)]
    pub start_date_time: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyticsPathFilter {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub operator: String,
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIntentPathsResponse {
    #[serde(rename = "nodeSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_summaries: Option<Vec<AnalyticsIntentNodeSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyticsIntentNodeSummary {
    #[serde(rename = "intentCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_count: Option<i32>,
    #[serde(rename = "intentLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_level: Option<i32>,
    #[serde(rename = "intentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_name: Option<String>,
    #[serde(rename = "intentPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_path: Option<String>,
    #[serde(rename = "nodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIntentStageMetricsRequest {
    #[serde(rename = "binBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_by: Option<Vec<AnalyticsBinBySpecification>>,
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "endDateTime")]
    #[serde(default)]
    pub end_date_time: f64,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<AnalyticsIntentStageFilter>>,
    #[serde(rename = "groupBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<AnalyticsIntentStageGroupBySpecification>>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(default)]
    pub metrics: Vec<AnalyticsIntentStageMetric>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "startDateTime")]
    #[serde(default)]
    pub start_date_time: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyticsIntentStageFilter {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub operator: String,
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyticsIntentStageGroupBySpecification {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyticsIntentStageMetric {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(default)]
    pub statistic: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIntentStageMetricsResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<AnalyticsIntentStageResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyticsIntentStageResult {
    #[serde(rename = "binKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_keys: Option<Vec<AnalyticsBinKey>>,
    #[serde(rename = "groupByKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by_keys: Option<Vec<AnalyticsIntentStageGroupByKey>>,
    #[serde(rename = "metricsResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_results: Option<Vec<AnalyticsIntentStageMetricResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyticsIntentStageGroupByKey {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyticsIntentStageMetricResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIntentsRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<IntentFilter>>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<IntentSortBy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntentFilter {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub operator: String,
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntentSortBy {
    #[serde(default)]
    pub attribute: String,
    #[serde(default)]
    pub order: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIntentsResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "intentSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_summaries: Option<Vec<IntentSummary>>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntentSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "inputContexts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_contexts: Option<Vec<InputContext>>,
    #[serde(rename = "intentDisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_display_name: Option<String>,
    #[serde(rename = "intentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_id: Option<String>,
    #[serde(rename = "intentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_name: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "outputContexts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_contexts: Option<Vec<OutputContext>>,
    #[serde(rename = "parentIntentSignature")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_intent_signature: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRecommendedIntentsRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botRecommendationId")]
    #[serde(default)]
    pub bot_recommendation_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRecommendedIntentsResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botRecommendationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_recommendation_id: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "summaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_list: Option<Vec<RecommendedIntentSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecommendedIntentSummary {
    #[serde(rename = "intentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_id: Option<String>,
    #[serde(rename = "intentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_name: Option<String>,
    #[serde(rename = "sampleUtterancesCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_utterances_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSessionAnalyticsDataRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "endDateTime")]
    #[serde(default)]
    pub end_date_time: f64,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<AnalyticsSessionFilter>>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<SessionDataSortBy>,
    #[serde(rename = "startDateTime")]
    #[serde(default)]
    pub start_date_time: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyticsSessionFilter {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub operator: String,
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SessionDataSortBy {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub order: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSessionAnalyticsDataResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sessions: Option<Vec<SessionSpecification>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SessionSpecification {
    #[serde(rename = "botAliasId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_alias_id: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(rename = "conversationDurationSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_duration_seconds: Option<i64>,
    #[serde(rename = "conversationEndState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_end_state: Option<String>,
    #[serde(rename = "conversationEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_end_time: Option<f64>,
    #[serde(rename = "conversationStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_start_time: Option<f64>,
    #[serde(rename = "invokedIntentSamples")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoked_intent_samples: Option<Vec<InvokedIntentSample>>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "numberOfTurns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_turns: Option<i64>,
    #[serde(rename = "originatingRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originating_request_id: Option<String>,
    #[serde(rename = "sessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InvokedIntentSample {
    #[serde(rename = "intentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSessionMetricsRequest {
    #[serde(rename = "binBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_by: Option<Vec<AnalyticsBinBySpecification>>,
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "endDateTime")]
    #[serde(default)]
    pub end_date_time: f64,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<AnalyticsSessionFilter>>,
    #[serde(rename = "groupBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<AnalyticsSessionGroupBySpecification>>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(default)]
    pub metrics: Vec<AnalyticsSessionMetric>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "startDateTime")]
    #[serde(default)]
    pub start_date_time: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyticsSessionGroupBySpecification {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyticsSessionMetric {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(default)]
    pub statistic: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSessionMetricsResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<AnalyticsSessionResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyticsSessionResult {
    #[serde(rename = "binKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_keys: Option<Vec<AnalyticsBinKey>>,
    #[serde(rename = "groupByKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by_keys: Option<Vec<AnalyticsSessionGroupByKey>>,
    #[serde(rename = "metricsResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_results: Option<Vec<AnalyticsSessionMetricResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyticsSessionGroupByKey {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyticsSessionMetricResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSlotTypesRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<SlotTypeFilter>>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<SlotTypeSortBy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SlotTypeFilter {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub operator: String,
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SlotTypeSortBy {
    #[serde(default)]
    pub attribute: String,
    #[serde(default)]
    pub order: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSlotTypesResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "slotTypeSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_type_summaries: Option<Vec<SlotTypeSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SlotTypeSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "parentSlotTypeSignature")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_slot_type_signature: Option<String>,
    #[serde(rename = "slotTypeCategory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_type_category: Option<String>,
    #[serde(rename = "slotTypeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_type_id: Option<String>,
    #[serde(rename = "slotTypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_type_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSlotsRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<SlotFilter>>,
    #[serde(rename = "intentId")]
    #[serde(default)]
    pub intent_id: String,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<SlotSortBy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SlotFilter {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub operator: String,
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SlotSortBy {
    #[serde(default)]
    pub attribute: String,
    #[serde(default)]
    pub order: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSlotsResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "intentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_id: Option<String>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "slotSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_summaries: Option<Vec<SlotSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SlotSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "slotConstraint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_constraint: Option<String>,
    #[serde(rename = "slotId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_id: Option<String>,
    #[serde(rename = "slotName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_name: Option<String>,
    #[serde(rename = "slotTypeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_type_id: Option<String>,
    #[serde(rename = "valueElicitationPromptSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_elicitation_prompt_specification: Option<PromptSpecification>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "resourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTestExecutionResultItemsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "resultFilterBy")]
    #[serde(default)]
    pub result_filter_by: TestExecutionResultFilterBy,
    #[serde(rename = "testExecutionId")]
    #[serde(default)]
    pub test_execution_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestExecutionResultFilterBy {
    #[serde(rename = "conversationLevelTestResultsFilterBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_level_test_results_filter_by: Option<ConversationLevelTestResultsFilterBy>,
    #[serde(rename = "resultTypeFilter")]
    #[serde(default)]
    pub result_type_filter: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConversationLevelTestResultsFilterBy {
    #[serde(rename = "endToEndResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_to_end_result: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTestExecutionResultItemsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "testExecutionResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_execution_results: Option<TestExecutionResultItems>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestExecutionResultItems {
    #[serde(rename = "conversationLevelTestResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_level_test_results: Option<ConversationLevelTestResults>,
    #[serde(rename = "intentClassificationTestResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_classification_test_results: Option<IntentClassificationTestResults>,
    #[serde(rename = "intentLevelSlotResolutionTestResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_level_slot_resolution_test_results: Option<IntentLevelSlotResolutionTestResults>,
    #[serde(rename = "overallTestResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall_test_results: Option<OverallTestResults>,
    #[serde(rename = "utteranceLevelTestResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utterance_level_test_results: Option<UtteranceLevelTestResults>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConversationLevelTestResults {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ConversationLevelTestResultItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConversationLevelTestResultItem {
    #[serde(rename = "conversationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    #[serde(rename = "endToEndResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_to_end_result: Option<String>,
    #[serde(rename = "intentClassificationResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_classification_results: Option<Vec<ConversationLevelIntentClassificationResultItem>>,
    #[serde(rename = "slotResolutionResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_resolution_results: Option<Vec<ConversationLevelSlotResolutionResultItem>>,
    #[serde(rename = "speechTranscriptionResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_transcription_result: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConversationLevelIntentClassificationResultItem {
    #[serde(rename = "intentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_name: Option<String>,
    #[serde(rename = "matchResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_result: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConversationLevelSlotResolutionResultItem {
    #[serde(rename = "intentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_name: Option<String>,
    #[serde(rename = "matchResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_result: Option<String>,
    #[serde(rename = "slotName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntentClassificationTestResults {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<IntentClassificationTestResultItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntentClassificationTestResultItem {
    #[serde(rename = "intentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_name: Option<String>,
    #[serde(rename = "multiTurnConversation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_turn_conversation: Option<bool>,
    #[serde(rename = "resultCounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_counts: Option<IntentClassificationTestResultItemCounts>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntentClassificationTestResultItemCounts {
    #[serde(rename = "intentMatchResultCounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_match_result_counts: Option<std::collections::HashMap<String, i32>>,
    #[serde(rename = "speechTranscriptionResultCounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_transcription_result_counts: Option<std::collections::HashMap<String, i32>>,
    #[serde(rename = "totalResultCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_result_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntentLevelSlotResolutionTestResults {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<IntentLevelSlotResolutionTestResultItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntentLevelSlotResolutionTestResultItem {
    #[serde(rename = "intentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_name: Option<String>,
    #[serde(rename = "multiTurnConversation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_turn_conversation: Option<bool>,
    #[serde(rename = "slotResolutionResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_resolution_results: Option<Vec<SlotResolutionTestResultItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SlotResolutionTestResultItem {
    #[serde(rename = "resultCounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_counts: Option<SlotResolutionTestResultItemCounts>,
    #[serde(rename = "slotName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SlotResolutionTestResultItemCounts {
    #[serde(rename = "slotMatchResultCounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_match_result_counts: Option<std::collections::HashMap<String, i32>>,
    #[serde(rename = "speechTranscriptionResultCounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_transcription_result_counts: Option<std::collections::HashMap<String, i32>>,
    #[serde(rename = "totalResultCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_result_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OverallTestResults {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<OverallTestResultItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OverallTestResultItem {
    #[serde(rename = "endToEndResultCounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_to_end_result_counts: Option<std::collections::HashMap<String, i32>>,
    #[serde(rename = "multiTurnConversation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_turn_conversation: Option<bool>,
    #[serde(rename = "speechTranscriptionResultCounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_transcription_result_counts: Option<std::collections::HashMap<String, i32>>,
    #[serde(rename = "totalResultCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_result_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UtteranceLevelTestResults {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<UtteranceLevelTestResultItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UtteranceLevelTestResultItem {
    #[serde(rename = "conversationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    #[serde(rename = "recordNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_number: Option<i64>,
    #[serde(rename = "turnResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_result: Option<TestSetTurnResult>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestSetTurnResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<AgentTurnResult>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserTurnResult>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgentTurnResult {
    #[serde(rename = "actualAgentPrompt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_agent_prompt: Option<String>,
    #[serde(rename = "actualElicitedSlot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_elicited_slot: Option<String>,
    #[serde(rename = "actualIntent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_intent: Option<String>,
    #[serde(rename = "errorDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<ExecutionErrorDetails>,
    #[serde(rename = "expectedAgentPrompt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_agent_prompt: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutionErrorDetails {
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserTurnResult {
    #[serde(rename = "actualOutput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_output: Option<UserTurnOutputSpecification>,
    #[serde(rename = "conversationLevelResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_level_result: Option<ConversationLevelResultDetail>,
    #[serde(rename = "endToEndResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_to_end_result: Option<String>,
    #[serde(rename = "errorDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<ExecutionErrorDetails>,
    #[serde(rename = "expectedOutput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_output: Option<UserTurnOutputSpecification>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<UserTurnInputSpecification>,
    #[serde(rename = "intentMatchResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_match_result: Option<String>,
    #[serde(rename = "slotMatchResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_match_result: Option<String>,
    #[serde(rename = "speechTranscriptionResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_transcription_result: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserTurnOutputSpecification {
    #[serde(rename = "activeContexts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_contexts: Option<Vec<ActiveContext>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent: Option<UserTurnIntentOutput>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActiveContext {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserTurnIntentOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slots: Option<std::collections::HashMap<String, UserTurnSlotOutput>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserTurnSlotOutput {
    #[serde(rename = "subSlots")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_slots: Option<std::collections::HashMap<String, UserTurnSlotOutput>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<UserTurnSlotOutput>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConversationLevelResultDetail {
    #[serde(rename = "endToEndResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_to_end_result: Option<String>,
    #[serde(rename = "speechTranscriptionResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_transcription_result: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserTurnInputSpecification {
    #[serde(rename = "requestAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "sessionState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_state: Option<InputSessionStateSpecification>,
    #[serde(rename = "utteranceInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utterance_input: Option<UtteranceInputSpecification>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputSessionStateSpecification {
    #[serde(rename = "activeContexts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_contexts: Option<Vec<ActiveContext>>,
    #[serde(rename = "runtimeHints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_hints: Option<RuntimeHints>,
    #[serde(rename = "sessionAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_attributes: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuntimeHints {
    #[serde(rename = "slotHints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_hints: Option<
        std::collections::HashMap<String, std::collections::HashMap<String, RuntimeHintDetails>>,
    >,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuntimeHintDetails {
    #[serde(rename = "runtimeHintValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_hint_values: Option<Vec<RuntimeHintValue>>,
    #[serde(rename = "subSlotHints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_slot_hints: Option<std::collections::HashMap<String, RuntimeHintDetails>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuntimeHintValue {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phrase: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UtteranceInputSpecification {
    #[serde(rename = "audioInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_input: Option<UtteranceAudioInputSpecification>,
    #[serde(rename = "textInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_input: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UtteranceAudioInputSpecification {
    #[serde(rename = "audioFileS3Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_file_s3_location: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTestExecutionsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<TestExecutionSortBy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestExecutionSortBy {
    #[serde(default)]
    pub attribute: String,
    #[serde(default)]
    pub order: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTestExecutionsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "testExecutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_executions: Option<Vec<TestExecutionSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestExecutionSummary {
    #[serde(rename = "apiMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_mode: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<TestExecutionTarget>,
    #[serde(rename = "testExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_execution_id: Option<String>,
    #[serde(rename = "testExecutionModality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_execution_modality: Option<String>,
    #[serde(rename = "testExecutionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_execution_status: Option<String>,
    #[serde(rename = "testSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_set_id: Option<String>,
    #[serde(rename = "testSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_set_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTestSetRecordsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "testSetId")]
    #[serde(default)]
    pub test_set_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTestSetRecordsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "testSetRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_set_records: Option<Vec<TestSetTurnRecord>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestSetTurnRecord {
    #[serde(rename = "conversationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    #[serde(rename = "recordNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_number: Option<i64>,
    #[serde(rename = "turnNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_number: Option<i32>,
    #[serde(rename = "turnSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_specification: Option<TurnSpecification>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TurnSpecification {
    #[serde(rename = "agentTurn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_turn: Option<AgentTurnSpecification>,
    #[serde(rename = "userTurn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_turn: Option<UserTurnSpecification>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgentTurnSpecification {
    #[serde(rename = "agentPrompt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_prompt: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserTurnSpecification {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected: Option<UserTurnOutputSpecification>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<UserTurnInputSpecification>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTestSetsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<TestSetSortBy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestSetSortBy {
    #[serde(default)]
    pub attribute: String,
    #[serde(default)]
    pub order: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTestSetsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "testSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_sets: Option<Vec<TestSetSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestSetSummary {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modality: Option<String>,
    #[serde(rename = "numTurns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_turns: Option<i32>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "storageLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_location: Option<TestSetStorageLocation>,
    #[serde(rename = "testSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_set_id: Option<String>,
    #[serde(rename = "testSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_set_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListUtteranceAnalyticsDataRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "endDateTime")]
    #[serde(default)]
    pub end_date_time: f64,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<AnalyticsUtteranceFilter>>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<UtteranceDataSortBy>,
    #[serde(rename = "startDateTime")]
    #[serde(default)]
    pub start_date_time: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyticsUtteranceFilter {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub operator: String,
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UtteranceDataSortBy {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub order: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListUtteranceAnalyticsDataResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utterances: Option<Vec<UtteranceSpecification>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UtteranceSpecification {
    #[serde(rename = "associatedIntentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_intent_name: Option<String>,
    #[serde(rename = "associatedSlotName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_slot_name: Option<String>,
    #[serde(rename = "audioVoiceDurationMillis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_voice_duration_millis: Option<i64>,
    #[serde(rename = "botAliasId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_alias_id: Option<String>,
    #[serde(rename = "botResponseAudioVoiceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_response_audio_voice_id: Option<String>,
    #[serde(rename = "botResponses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_responses: Option<Vec<UtteranceBotResponse>>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(rename = "conversationEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_end_time: Option<f64>,
    #[serde(rename = "conversationStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_start_time: Option<f64>,
    #[serde(rename = "dialogActionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialog_action_type: Option<String>,
    #[serde(rename = "inputType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_type: Option<String>,
    #[serde(rename = "intentState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_state: Option<String>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "outputType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_type: Option<String>,
    #[serde(rename = "sessionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "slotsFilledInSession")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slots_filled_in_session: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utterance: Option<String>,
    #[serde(rename = "utteranceRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utterance_request_id: Option<String>,
    #[serde(rename = "utteranceTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utterance_timestamp: Option<f64>,
    #[serde(rename = "utteranceUnderstood")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utterance_understood: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UtteranceBotResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "contentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "imageResponseCard")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_response_card: Option<ImageResponseCard>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListUtteranceMetricsRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<AnalyticsUtteranceAttribute>>,
    #[serde(rename = "binBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_by: Option<Vec<AnalyticsBinBySpecification>>,
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "endDateTime")]
    #[serde(default)]
    pub end_date_time: f64,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<AnalyticsUtteranceFilter>>,
    #[serde(rename = "groupBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<AnalyticsUtteranceGroupBySpecification>>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(default)]
    pub metrics: Vec<AnalyticsUtteranceMetric>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "startDateTime")]
    #[serde(default)]
    pub start_date_time: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyticsUtteranceAttribute {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyticsUtteranceGroupBySpecification {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyticsUtteranceMetric {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(default)]
    pub statistic: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListUtteranceMetricsResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<AnalyticsUtteranceResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyticsUtteranceResult {
    #[serde(rename = "attributeResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_results: Option<Vec<AnalyticsUtteranceAttributeResult>>,
    #[serde(rename = "binKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_keys: Option<Vec<AnalyticsBinKey>>,
    #[serde(rename = "groupByKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by_keys: Option<Vec<AnalyticsUtteranceGroupByKey>>,
    #[serde(rename = "metricsResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_results: Option<Vec<AnalyticsUtteranceMetricResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyticsUtteranceAttributeResult {
    #[serde(rename = "lastUsedIntent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_used_intent: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyticsUtteranceGroupByKey {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyticsUtteranceMetricResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistic: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchAssociatedTranscriptsRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botRecommendationId")]
    #[serde(default)]
    pub bot_recommendation_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(default)]
    pub filters: Vec<AssociatedTranscriptFilter>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_index: Option<i32>,
    #[serde(rename = "searchOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociatedTranscriptFilter {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchAssociatedTranscriptsResponse {
    #[serde(rename = "associatedTranscripts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_transcripts: Option<Vec<AssociatedTranscript>>,
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botRecommendationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_recommendation_id: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(rename = "nextIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_index: Option<i32>,
    #[serde(rename = "totalResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_results: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociatedTranscript {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartBotAnalyzerRequest {
    #[serde(rename = "analysisScope")]
    #[serde(default)]
    pub analysis_scope: String,
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartBotAnalyzerResponse {
    #[serde(rename = "botAnalyzerRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_analyzer_request_id: Option<String>,
    #[serde(rename = "botAnalyzerStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_analyzer_status: Option<String>,
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartBotRecommendationRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(rename = "encryptionSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_setting: Option<EncryptionSetting>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
    #[serde(rename = "transcriptSourceSetting")]
    #[serde(default)]
    pub transcript_source_setting: TranscriptSourceSetting,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartBotRecommendationResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botRecommendationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_recommendation_id: Option<String>,
    #[serde(rename = "botRecommendationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_recommendation_status: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "encryptionSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_setting: Option<EncryptionSetting>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(rename = "transcriptSourceSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript_source_setting: Option<TranscriptSourceSetting>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartBotResourceGenerationRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(rename = "generationInputPrompt")]
    #[serde(default)]
    pub generation_input_prompt: String,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartBotResourceGenerationResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "generationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_id: Option<String>,
    #[serde(rename = "generationInputPrompt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_input_prompt: Option<String>,
    #[serde(rename = "generationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_status: Option<String>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartImportRequest {
    #[serde(rename = "filePassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_password: Option<String>,
    #[serde(rename = "importId")]
    #[serde(default)]
    pub import_id: String,
    #[serde(rename = "mergeStrategy")]
    #[serde(default)]
    pub merge_strategy: String,
    #[serde(rename = "resourceSpecification")]
    #[serde(default)]
    pub resource_specification: ImportResourceSpecification,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartImportResponse {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "importId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_id: Option<String>,
    #[serde(rename = "importStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_status: Option<String>,
    #[serde(rename = "mergeStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_strategy: Option<String>,
    #[serde(rename = "resourceSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_specification: Option<ImportResourceSpecification>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartTestExecutionRequest {
    #[serde(rename = "apiMode")]
    #[serde(default)]
    pub api_mode: String,
    #[serde(default)]
    pub target: TestExecutionTarget,
    #[serde(rename = "testExecutionModality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_execution_modality: Option<String>,
    #[serde(rename = "testSetId")]
    #[serde(default)]
    pub test_set_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartTestExecutionResponse {
    #[serde(rename = "apiMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_mode: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<TestExecutionTarget>,
    #[serde(rename = "testExecutionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_execution_id: Option<String>,
    #[serde(rename = "testExecutionModality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_execution_modality: Option<String>,
    #[serde(rename = "testSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_set_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartTestSetGenerationRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "generationDataSource")]
    #[serde(default)]
    pub generation_data_source: TestSetGenerationDataSource,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "storageLocation")]
    #[serde(default)]
    pub storage_location: TestSetStorageLocation,
    #[serde(rename = "testSetName")]
    #[serde(default)]
    pub test_set_name: String,
    #[serde(rename = "testSetTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_set_tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartTestSetGenerationResponse {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "generationDataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_data_source: Option<TestSetGenerationDataSource>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "storageLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_location: Option<TestSetStorageLocation>,
    #[serde(rename = "testSetGenerationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_set_generation_id: Option<String>,
    #[serde(rename = "testSetGenerationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_set_generation_status: Option<String>,
    #[serde(rename = "testSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_set_name: Option<String>,
    #[serde(rename = "testSetTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_set_tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopBotAnalyzerRequest {
    #[serde(rename = "botAnalyzerRequestId")]
    #[serde(default)]
    pub bot_analyzer_request_id: String,
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopBotAnalyzerResponse {
    #[serde(rename = "botAnalyzerRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_analyzer_request_id: Option<String>,
    #[serde(rename = "botAnalyzerStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_analyzer_status: Option<String>,
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopBotRecommendationRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botRecommendationId")]
    #[serde(default)]
    pub bot_recommendation_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopBotRecommendationResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botRecommendationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_recommendation_id: Option<String>,
    #[serde(rename = "botRecommendationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_recommendation_status: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "resourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "resourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "tagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBotAliasRequest {
    #[serde(rename = "botAliasId")]
    #[serde(default)]
    pub bot_alias_id: String,
    #[serde(rename = "botAliasLocaleSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_alias_locale_settings:
        Option<std::collections::HashMap<String, BotAliasLocaleSettings>>,
    #[serde(rename = "botAliasName")]
    #[serde(default)]
    pub bot_alias_name: String,
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "conversationLogSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_log_settings: Option<ConversationLogSettings>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "sentimentAnalysisSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiment_analysis_settings: Option<SentimentAnalysisSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBotAliasResponse {
    #[serde(rename = "botAliasId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_alias_id: Option<String>,
    #[serde(rename = "botAliasLocaleSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_alias_locale_settings:
        Option<std::collections::HashMap<String, BotAliasLocaleSettings>>,
    #[serde(rename = "botAliasName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_alias_name: Option<String>,
    #[serde(rename = "botAliasStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_alias_status: Option<String>,
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "conversationLogSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_log_settings: Option<ConversationLogSettings>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "sentimentAnalysisSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiment_analysis_settings: Option<SentimentAnalysisSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBotLocaleRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "generativeAISettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generative_a_i_settings: Option<GenerativeAISettings>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
    #[serde(rename = "nluIntentConfidenceThreshold")]
    #[serde(default)]
    pub nlu_intent_confidence_threshold: f64,
    #[serde(rename = "speechDetectionSensitivity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_detection_sensitivity: Option<String>,
    #[serde(rename = "speechRecognitionSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_recognition_settings: Option<SpeechRecognitionSettings>,
    #[serde(rename = "unifiedSpeechSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unified_speech_settings: Option<UnifiedSpeechSettings>,
    #[serde(rename = "voiceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_settings: Option<VoiceSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBotLocaleResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botLocaleStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_locale_status: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "failureReasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reasons: Option<Vec<String>>,
    #[serde(rename = "generativeAISettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generative_a_i_settings: Option<GenerativeAISettings>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(rename = "localeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_name: Option<String>,
    #[serde(rename = "nluIntentConfidenceThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nlu_intent_confidence_threshold: Option<f64>,
    #[serde(rename = "recommendedActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_actions: Option<Vec<String>>,
    #[serde(rename = "speechDetectionSensitivity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_detection_sensitivity: Option<String>,
    #[serde(rename = "speechRecognitionSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_recognition_settings: Option<SpeechRecognitionSettings>,
    #[serde(rename = "unifiedSpeechSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unified_speech_settings: Option<UnifiedSpeechSettings>,
    #[serde(rename = "voiceSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_settings: Option<VoiceSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBotRecommendationRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botRecommendationId")]
    #[serde(default)]
    pub bot_recommendation_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(rename = "encryptionSetting")]
    #[serde(default)]
    pub encryption_setting: EncryptionSetting,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBotRecommendationResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botRecommendationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_recommendation_id: Option<String>,
    #[serde(rename = "botRecommendationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_recommendation_status: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "encryptionSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_setting: Option<EncryptionSetting>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(rename = "transcriptSourceSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript_source_setting: Option<TranscriptSourceSetting>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBotRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botMembers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_members: Option<Vec<BotMember>>,
    #[serde(rename = "botName")]
    #[serde(default)]
    pub bot_name: String,
    #[serde(rename = "botType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_type: Option<String>,
    #[serde(rename = "dataPrivacy")]
    #[serde(default)]
    pub data_privacy: DataPrivacy,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "errorLogSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_log_settings: Option<ErrorLogSettings>,
    #[serde(rename = "idleSessionTTLInSeconds")]
    #[serde(default)]
    pub idle_session_t_t_l_in_seconds: i32,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBotResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botMembers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_members: Option<Vec<BotMember>>,
    #[serde(rename = "botName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_name: Option<String>,
    #[serde(rename = "botStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_status: Option<String>,
    #[serde(rename = "botType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_type: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "dataPrivacy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_privacy: Option<DataPrivacy>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "errorLogSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_log_settings: Option<ErrorLogSettings>,
    #[serde(rename = "idleSessionTTLInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_session_t_t_l_in_seconds: Option<i32>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateExportRequest {
    #[serde(rename = "exportId")]
    #[serde(default)]
    pub export_id: String,
    #[serde(rename = "filePassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_password: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateExportResponse {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "exportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_id: Option<String>,
    #[serde(rename = "exportStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_status: Option<String>,
    #[serde(rename = "fileFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_format: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "resourceSpecification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_specification: Option<ExportResourceSpecification>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIntentRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "dialogCodeHook")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialog_code_hook: Option<DialogCodeHookSettings>,
    #[serde(rename = "fulfillmentCodeHook")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_code_hook: Option<FulfillmentCodeHookSettings>,
    #[serde(rename = "initialResponseSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_response_setting: Option<InitialResponseSetting>,
    #[serde(rename = "inputContexts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_contexts: Option<Vec<InputContext>>,
    #[serde(rename = "intentClosingSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_closing_setting: Option<IntentClosingSetting>,
    #[serde(rename = "intentConfirmationSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_confirmation_setting: Option<IntentConfirmationSetting>,
    #[serde(rename = "intentDisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_display_name: Option<String>,
    #[serde(rename = "intentId")]
    #[serde(default)]
    pub intent_id: String,
    #[serde(rename = "intentName")]
    #[serde(default)]
    pub intent_name: String,
    #[serde(rename = "kendraConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kendra_configuration: Option<KendraConfiguration>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
    #[serde(rename = "outputContexts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_contexts: Option<Vec<OutputContext>>,
    #[serde(rename = "parentIntentSignature")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_intent_signature: Option<String>,
    #[serde(rename = "qInConnectIntentConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q_in_connect_intent_configuration: Option<QInConnectIntentConfiguration>,
    #[serde(rename = "qnAIntentConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qn_a_intent_configuration: Option<QnAIntentConfiguration>,
    #[serde(rename = "sampleUtterances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_utterances: Option<Vec<SampleUtterance>>,
    #[serde(rename = "slotPriorities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_priorities: Option<Vec<SlotPriority>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIntentResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "dialogCodeHook")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialog_code_hook: Option<DialogCodeHookSettings>,
    #[serde(rename = "fulfillmentCodeHook")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_code_hook: Option<FulfillmentCodeHookSettings>,
    #[serde(rename = "initialResponseSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_response_setting: Option<InitialResponseSetting>,
    #[serde(rename = "inputContexts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_contexts: Option<Vec<InputContext>>,
    #[serde(rename = "intentClosingSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_closing_setting: Option<IntentClosingSetting>,
    #[serde(rename = "intentConfirmationSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_confirmation_setting: Option<IntentConfirmationSetting>,
    #[serde(rename = "intentDisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_display_name: Option<String>,
    #[serde(rename = "intentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_id: Option<String>,
    #[serde(rename = "intentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_name: Option<String>,
    #[serde(rename = "kendraConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kendra_configuration: Option<KendraConfiguration>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(rename = "outputContexts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_contexts: Option<Vec<OutputContext>>,
    #[serde(rename = "parentIntentSignature")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_intent_signature: Option<String>,
    #[serde(rename = "qInConnectIntentConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q_in_connect_intent_configuration: Option<QInConnectIntentConfiguration>,
    #[serde(rename = "qnAIntentConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qn_a_intent_configuration: Option<QnAIntentConfiguration>,
    #[serde(rename = "sampleUtterances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_utterances: Option<Vec<SampleUtterance>>,
    #[serde(rename = "slotPriorities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_priorities: Option<Vec<SlotPriority>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResourcePolicyRequest {
    #[serde(rename = "expectedRevisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_revision_id: Option<String>,
    #[serde(default)]
    pub policy: String,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResourcePolicyResponse {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "revisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSlotRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "intentId")]
    #[serde(default)]
    pub intent_id: String,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
    #[serde(rename = "multipleValuesSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_values_setting: Option<MultipleValuesSetting>,
    #[serde(rename = "obfuscationSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfuscation_setting: Option<ObfuscationSetting>,
    #[serde(rename = "slotId")]
    #[serde(default)]
    pub slot_id: String,
    #[serde(rename = "slotName")]
    #[serde(default)]
    pub slot_name: String,
    #[serde(rename = "slotTypeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_type_id: Option<String>,
    #[serde(rename = "subSlotSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_slot_setting: Option<SubSlotSetting>,
    #[serde(rename = "valueElicitationSetting")]
    #[serde(default)]
    pub value_elicitation_setting: SlotValueElicitationSetting,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSlotResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "intentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent_id: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(rename = "multipleValuesSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_values_setting: Option<MultipleValuesSetting>,
    #[serde(rename = "obfuscationSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfuscation_setting: Option<ObfuscationSetting>,
    #[serde(rename = "slotId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_id: Option<String>,
    #[serde(rename = "slotName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_name: Option<String>,
    #[serde(rename = "slotTypeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_type_id: Option<String>,
    #[serde(rename = "subSlotSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_slot_setting: Option<SubSlotSetting>,
    #[serde(rename = "valueElicitationSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_elicitation_setting: Option<SlotValueElicitationSetting>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSlotTypeRequest {
    #[serde(rename = "botId")]
    #[serde(default)]
    pub bot_id: String,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    pub bot_version: String,
    #[serde(rename = "compositeSlotTypeSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite_slot_type_setting: Option<CompositeSlotTypeSetting>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "externalSourceSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_source_setting: Option<ExternalSourceSetting>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
    #[serde(rename = "parentSlotTypeSignature")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_slot_type_signature: Option<String>,
    #[serde(rename = "slotTypeId")]
    #[serde(default)]
    pub slot_type_id: String,
    #[serde(rename = "slotTypeName")]
    #[serde(default)]
    pub slot_type_name: String,
    #[serde(rename = "slotTypeValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_type_values: Option<Vec<SlotTypeValue>>,
    #[serde(rename = "valueSelectionSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_selection_setting: Option<SlotValueSelectionSetting>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSlotTypeResponse {
    #[serde(rename = "botId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    #[serde(rename = "botVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_version: Option<String>,
    #[serde(rename = "compositeSlotTypeSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite_slot_type_setting: Option<CompositeSlotTypeSetting>,
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "externalSourceSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_source_setting: Option<ExternalSourceSetting>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(rename = "localeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale_id: Option<String>,
    #[serde(rename = "parentSlotTypeSignature")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_slot_type_signature: Option<String>,
    #[serde(rename = "slotTypeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_type_id: Option<String>,
    #[serde(rename = "slotTypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_type_name: Option<String>,
    #[serde(rename = "slotTypeValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_type_values: Option<Vec<SlotTypeValue>>,
    #[serde(rename = "valueSelectionSetting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_selection_setting: Option<SlotValueSelectionSetting>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTestSetRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "testSetId")]
    #[serde(default)]
    pub test_set_id: String,
    #[serde(rename = "testSetName")]
    #[serde(default)]
    pub test_set_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTestSetResponse {
    #[serde(rename = "creationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "lastUpdatedDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date_time: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modality: Option<String>,
    #[serde(rename = "numTurns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_turns: Option<i32>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "storageLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_location: Option<TestSetStorageLocation>,
    #[serde(rename = "testSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_set_id: Option<String>,
    #[serde(rename = "testSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_set_name: Option<String>,
}
