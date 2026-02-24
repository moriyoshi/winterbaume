//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-comprehend

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDetectDominantLanguageRequest {
    #[serde(rename = "TextList")]
    #[serde(default)]
    pub text_list: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDetectDominantLanguageResponse {
    #[serde(rename = "ErrorList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_list: Option<Vec<BatchItemError>>,
    #[serde(rename = "ResultList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_list: Option<Vec<BatchDetectDominantLanguageItemResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchItemError {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "Index")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDetectDominantLanguageItemResult {
    #[serde(rename = "Index")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    #[serde(rename = "Languages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<DominantLanguage>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DominantLanguage {
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "Score")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDetectEntitiesRequest {
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    pub language_code: String,
    #[serde(rename = "TextList")]
    #[serde(default)]
    pub text_list: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDetectEntitiesResponse {
    #[serde(rename = "ErrorList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_list: Option<Vec<BatchItemError>>,
    #[serde(rename = "ResultList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_list: Option<Vec<BatchDetectEntitiesItemResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDetectEntitiesItemResult {
    #[serde(rename = "Entities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<Entity>>,
    #[serde(rename = "Index")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Entity {
    #[serde(rename = "BeginOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_offset: Option<i32>,
    #[serde(rename = "BlockReferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_references: Option<Vec<BlockReference>>,
    #[serde(rename = "EndOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_offset: Option<i32>,
    #[serde(rename = "Score")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
    #[serde(rename = "Text")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BlockReference {
    #[serde(rename = "BeginOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_offset: Option<i32>,
    #[serde(rename = "BlockId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<String>,
    #[serde(rename = "ChildBlocks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_blocks: Option<Vec<ChildBlock>>,
    #[serde(rename = "EndOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_offset: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChildBlock {
    #[serde(rename = "BeginOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_offset: Option<i32>,
    #[serde(rename = "ChildBlockId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_block_id: Option<String>,
    #[serde(rename = "EndOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_offset: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDetectKeyPhrasesRequest {
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    pub language_code: String,
    #[serde(rename = "TextList")]
    #[serde(default)]
    pub text_list: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDetectKeyPhrasesResponse {
    #[serde(rename = "ErrorList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_list: Option<Vec<BatchItemError>>,
    #[serde(rename = "ResultList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_list: Option<Vec<BatchDetectKeyPhrasesItemResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDetectKeyPhrasesItemResult {
    #[serde(rename = "Index")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    #[serde(rename = "KeyPhrases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_phrases: Option<Vec<KeyPhrase>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KeyPhrase {
    #[serde(rename = "BeginOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_offset: Option<i32>,
    #[serde(rename = "EndOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_offset: Option<i32>,
    #[serde(rename = "Score")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
    #[serde(rename = "Text")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDetectSentimentRequest {
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    pub language_code: String,
    #[serde(rename = "TextList")]
    #[serde(default)]
    pub text_list: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDetectSentimentResponse {
    #[serde(rename = "ErrorList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_list: Option<Vec<BatchItemError>>,
    #[serde(rename = "ResultList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_list: Option<Vec<BatchDetectSentimentItemResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDetectSentimentItemResult {
    #[serde(rename = "Index")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    #[serde(rename = "Sentiment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiment: Option<String>,
    #[serde(rename = "SentimentScore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiment_score: Option<SentimentScore>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SentimentScore {
    #[serde(rename = "Mixed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mixed: Option<f32>,
    #[serde(rename = "Negative")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative: Option<f32>,
    #[serde(rename = "Neutral")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neutral: Option<f32>,
    #[serde(rename = "Positive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positive: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDetectSyntaxRequest {
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    pub language_code: String,
    #[serde(rename = "TextList")]
    #[serde(default)]
    pub text_list: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDetectSyntaxResponse {
    #[serde(rename = "ErrorList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_list: Option<Vec<BatchItemError>>,
    #[serde(rename = "ResultList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_list: Option<Vec<BatchDetectSyntaxItemResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDetectSyntaxItemResult {
    #[serde(rename = "Index")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    #[serde(rename = "SyntaxTokens")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syntax_tokens: Option<Vec<SyntaxToken>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SyntaxToken {
    #[serde(rename = "BeginOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_offset: Option<i32>,
    #[serde(rename = "EndOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_offset: Option<i32>,
    #[serde(rename = "PartOfSpeech")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_of_speech: Option<PartOfSpeechTag>,
    #[serde(rename = "Text")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "TokenId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_id: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PartOfSpeechTag {
    #[serde(rename = "Score")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
    #[serde(rename = "Tag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDetectTargetedSentimentRequest {
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    pub language_code: String,
    #[serde(rename = "TextList")]
    #[serde(default)]
    pub text_list: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDetectTargetedSentimentResponse {
    #[serde(rename = "ErrorList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_list: Option<Vec<BatchItemError>>,
    #[serde(rename = "ResultList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_list: Option<Vec<BatchDetectTargetedSentimentItemResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDetectTargetedSentimentItemResult {
    #[serde(rename = "Entities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<TargetedSentimentEntity>>,
    #[serde(rename = "Index")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetedSentimentEntity {
    #[serde(rename = "DescriptiveMentionIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_mention_index: Option<Vec<i32>>,
    #[serde(rename = "Mentions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<TargetedSentimentMention>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetedSentimentMention {
    #[serde(rename = "BeginOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_offset: Option<i32>,
    #[serde(rename = "EndOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_offset: Option<i32>,
    #[serde(rename = "GroupScore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_score: Option<f32>,
    #[serde(rename = "MentionSentiment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mention_sentiment: Option<MentionSentiment>,
    #[serde(rename = "Score")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
    #[serde(rename = "Text")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MentionSentiment {
    #[serde(rename = "Sentiment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiment: Option<String>,
    #[serde(rename = "SentimentScore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiment_score: Option<SentimentScore>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClassifyDocumentRequest {
    #[serde(rename = "Bytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<String>,
    #[serde(rename = "DocumentReaderConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_reader_config: Option<DocumentReaderConfig>,
    #[serde(rename = "EndpointArn")]
    #[serde(default)]
    pub endpoint_arn: String,
    #[serde(rename = "Text")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentReaderConfig {
    #[serde(rename = "DocumentReadAction")]
    #[serde(default)]
    pub document_read_action: String,
    #[serde(rename = "DocumentReadMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_read_mode: Option<String>,
    #[serde(rename = "FeatureTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClassifyDocumentResponse {
    #[serde(rename = "Classes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classes: Option<Vec<DocumentClass>>,
    #[serde(rename = "DocumentMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_metadata: Option<DocumentMetadata>,
    #[serde(rename = "DocumentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_type: Option<Vec<DocumentTypeListItem>>,
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ErrorsListItem>>,
    #[serde(rename = "Labels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<DocumentLabel>>,
    #[serde(rename = "Warnings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<WarningsListItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentClass {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Page")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "Score")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentMetadata {
    #[serde(rename = "ExtractedCharacters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extracted_characters: Option<Vec<ExtractedCharactersListItem>>,
    #[serde(rename = "Pages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExtractedCharactersListItem {
    #[serde(rename = "Count")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "Page")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentTypeListItem {
    #[serde(rename = "Page")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ErrorsListItem {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "Page")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentLabel {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Page")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "Score")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WarningsListItem {
    #[serde(rename = "Page")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "WarnCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warn_code: Option<String>,
    #[serde(rename = "WarnMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warn_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContainsPiiEntitiesRequest {
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    pub language_code: String,
    #[serde(rename = "Text")]
    #[serde(default)]
    pub text: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContainsPiiEntitiesResponse {
    #[serde(rename = "Labels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<EntityLabel>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EntityLabel {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Score")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDatasetRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "DatasetName")]
    #[serde(default)]
    pub dataset_name: String,
    #[serde(rename = "DatasetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_type: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "FlywheelArn")]
    #[serde(default)]
    pub flywheel_arn: String,
    #[serde(rename = "InputDataConfig")]
    #[serde(default)]
    pub input_data_config: DatasetInputDataConfig,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatasetInputDataConfig {
    #[serde(rename = "AugmentedManifests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub augmented_manifests: Option<Vec<DatasetAugmentedManifestsListItem>>,
    #[serde(rename = "DataFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_format: Option<String>,
    #[serde(rename = "DocumentClassifierInputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_classifier_input_data_config: Option<DatasetDocumentClassifierInputDataConfig>,
    #[serde(rename = "EntityRecognizerInputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_recognizer_input_data_config: Option<DatasetEntityRecognizerInputDataConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatasetAugmentedManifestsListItem {
    #[serde(rename = "AnnotationDataS3Uri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_data_s3_uri: Option<String>,
    #[serde(rename = "AttributeNames")]
    #[serde(default)]
    pub attribute_names: Vec<String>,
    #[serde(rename = "DocumentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_type: Option<String>,
    #[serde(rename = "S3Uri")]
    #[serde(default)]
    pub s3_uri: String,
    #[serde(rename = "SourceDocumentsS3Uri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_documents_s3_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatasetDocumentClassifierInputDataConfig {
    #[serde(rename = "LabelDelimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_delimiter: Option<String>,
    #[serde(rename = "S3Uri")]
    #[serde(default)]
    pub s3_uri: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatasetEntityRecognizerInputDataConfig {
    #[serde(rename = "Annotations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<DatasetEntityRecognizerAnnotations>,
    #[serde(rename = "Documents")]
    #[serde(default)]
    pub documents: DatasetEntityRecognizerDocuments,
    #[serde(rename = "EntityList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_list: Option<DatasetEntityRecognizerEntityList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatasetEntityRecognizerAnnotations {
    #[serde(rename = "S3Uri")]
    #[serde(default)]
    pub s3_uri: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatasetEntityRecognizerDocuments {
    #[serde(rename = "InputFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_format: Option<String>,
    #[serde(rename = "S3Uri")]
    #[serde(default)]
    pub s3_uri: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatasetEntityRecognizerEntityList {
    #[serde(rename = "S3Uri")]
    #[serde(default)]
    pub s3_uri: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDatasetResponse {
    #[serde(rename = "DatasetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDocumentClassifierRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(default)]
    pub data_access_role_arn: String,
    #[serde(rename = "DocumentClassifierName")]
    #[serde(default)]
    pub document_classifier_name: String,
    #[serde(rename = "InputDataConfig")]
    #[serde(default)]
    pub input_data_config: DocumentClassifierInputDataConfig,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    pub language_code: String,
    #[serde(rename = "Mode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "ModelKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_kms_key_id: Option<String>,
    #[serde(rename = "ModelPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_policy: Option<String>,
    #[serde(rename = "OutputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_data_config: Option<DocumentClassifierOutputDataConfig>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VersionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    #[serde(rename = "VpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentClassifierInputDataConfig {
    #[serde(rename = "AugmentedManifests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub augmented_manifests: Option<Vec<AugmentedManifestsListItem>>,
    #[serde(rename = "DataFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_format: Option<String>,
    #[serde(rename = "DocumentReaderConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_reader_config: Option<DocumentReaderConfig>,
    #[serde(rename = "DocumentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_type: Option<String>,
    #[serde(rename = "Documents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<DocumentClassifierDocuments>,
    #[serde(rename = "LabelDelimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_delimiter: Option<String>,
    #[serde(rename = "S3Uri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_uri: Option<String>,
    #[serde(rename = "TestS3Uri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_s3_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AugmentedManifestsListItem {
    #[serde(rename = "AnnotationDataS3Uri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_data_s3_uri: Option<String>,
    #[serde(rename = "AttributeNames")]
    #[serde(default)]
    pub attribute_names: Vec<String>,
    #[serde(rename = "DocumentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_type: Option<String>,
    #[serde(rename = "S3Uri")]
    #[serde(default)]
    pub s3_uri: String,
    #[serde(rename = "SourceDocumentsS3Uri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_documents_s3_uri: Option<String>,
    #[serde(rename = "Split")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentClassifierDocuments {
    #[serde(rename = "S3Uri")]
    #[serde(default)]
    pub s3_uri: String,
    #[serde(rename = "TestS3Uri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_s3_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentClassifierOutputDataConfig {
    #[serde(rename = "FlywheelStatsS3Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flywheel_stats_s3_prefix: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "S3Uri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcConfig {
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    pub security_group_ids: Vec<String>,
    #[serde(rename = "Subnets")]
    #[serde(default)]
    pub subnets: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDocumentClassifierResponse {
    #[serde(rename = "DocumentClassifierArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_classifier_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEndpointRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_role_arn: Option<String>,
    #[serde(rename = "DesiredInferenceUnits")]
    #[serde(default)]
    pub desired_inference_units: i32,
    #[serde(rename = "EndpointName")]
    #[serde(default)]
    pub endpoint_name: String,
    #[serde(rename = "FlywheelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flywheel_arn: Option<String>,
    #[serde(rename = "ModelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_arn: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEndpointResponse {
    #[serde(rename = "EndpointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_arn: Option<String>,
    #[serde(rename = "ModelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEntityRecognizerRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(default)]
    pub data_access_role_arn: String,
    #[serde(rename = "InputDataConfig")]
    #[serde(default)]
    pub input_data_config: EntityRecognizerInputDataConfig,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    pub language_code: String,
    #[serde(rename = "ModelKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_kms_key_id: Option<String>,
    #[serde(rename = "ModelPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_policy: Option<String>,
    #[serde(rename = "RecognizerName")]
    #[serde(default)]
    pub recognizer_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VersionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    #[serde(rename = "VpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EntityRecognizerInputDataConfig {
    #[serde(rename = "Annotations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<EntityRecognizerAnnotations>,
    #[serde(rename = "AugmentedManifests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub augmented_manifests: Option<Vec<AugmentedManifestsListItem>>,
    #[serde(rename = "DataFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_format: Option<String>,
    #[serde(rename = "Documents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<EntityRecognizerDocuments>,
    #[serde(rename = "EntityList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_list: Option<EntityRecognizerEntityList>,
    #[serde(rename = "EntityTypes")]
    #[serde(default)]
    pub entity_types: Vec<EntityTypesListItem>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EntityRecognizerAnnotations {
    #[serde(rename = "S3Uri")]
    #[serde(default)]
    pub s3_uri: String,
    #[serde(rename = "TestS3Uri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_s3_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EntityRecognizerDocuments {
    #[serde(rename = "InputFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_format: Option<String>,
    #[serde(rename = "S3Uri")]
    #[serde(default)]
    pub s3_uri: String,
    #[serde(rename = "TestS3Uri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_s3_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EntityRecognizerEntityList {
    #[serde(rename = "S3Uri")]
    #[serde(default)]
    pub s3_uri: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EntityTypesListItem {
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEntityRecognizerResponse {
    #[serde(rename = "EntityRecognizerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_recognizer_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFlywheelRequest {
    #[serde(rename = "ActiveModelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_model_arn: Option<String>,
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(default)]
    pub data_access_role_arn: String,
    #[serde(rename = "DataLakeS3Uri")]
    #[serde(default)]
    pub data_lake_s3_uri: String,
    #[serde(rename = "DataSecurityConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_security_config: Option<DataSecurityConfig>,
    #[serde(rename = "FlywheelName")]
    #[serde(default)]
    pub flywheel_name: String,
    #[serde(rename = "ModelType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_type: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TaskConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_config: Option<TaskConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSecurityConfig {
    #[serde(rename = "DataLakeKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_lake_kms_key_id: Option<String>,
    #[serde(rename = "ModelKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_kms_key_id: Option<String>,
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    #[serde(rename = "VpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskConfig {
    #[serde(rename = "DocumentClassificationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_classification_config: Option<DocumentClassificationConfig>,
    #[serde(rename = "EntityRecognitionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_recognition_config: Option<EntityRecognitionConfig>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    pub language_code: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentClassificationConfig {
    #[serde(rename = "Labels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    #[serde(rename = "Mode")]
    #[serde(default)]
    pub mode: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EntityRecognitionConfig {
    #[serde(rename = "EntityTypes")]
    #[serde(default)]
    pub entity_types: Vec<EntityTypesListItem>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFlywheelResponse {
    #[serde(rename = "ActiveModelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_model_arn: Option<String>,
    #[serde(rename = "FlywheelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flywheel_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDocumentClassifierRequest {
    #[serde(rename = "DocumentClassifierArn")]
    #[serde(default)]
    pub document_classifier_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDocumentClassifierResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEndpointRequest {
    #[serde(rename = "EndpointArn")]
    #[serde(default)]
    pub endpoint_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEndpointResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEntityRecognizerRequest {
    #[serde(rename = "EntityRecognizerArn")]
    #[serde(default)]
    pub entity_recognizer_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEntityRecognizerResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFlywheelRequest {
    #[serde(rename = "FlywheelArn")]
    #[serde(default)]
    pub flywheel_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFlywheelResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourcePolicyRequest {
    #[serde(rename = "PolicyRevisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_revision_id: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourcePolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDatasetRequest {
    #[serde(rename = "DatasetArn")]
    #[serde(default)]
    pub dataset_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDatasetResponse {
    #[serde(rename = "DatasetProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_properties: Option<DatasetProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatasetProperties {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "DatasetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arn: Option<String>,
    #[serde(rename = "DatasetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_name: Option<String>,
    #[serde(rename = "DatasetS3Uri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_s3_uri: Option<String>,
    #[serde(rename = "DatasetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_type: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "NumberOfDocuments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_documents: Option<i64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDocumentClassificationJobRequest {
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDocumentClassificationJobResponse {
    #[serde(rename = "DocumentClassificationJobProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_classification_job_properties: Option<DocumentClassificationJobProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentClassificationJobProperties {
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_role_arn: Option<String>,
    #[serde(rename = "DocumentClassifierArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_classifier_arn: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "FlywheelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flywheel_arn: Option<String>,
    #[serde(rename = "InputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_config: Option<InputDataConfig>,
    #[serde(rename = "JobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "OutputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_data_config: Option<OutputDataConfig>,
    #[serde(rename = "SubmitTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<f64>,
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    #[serde(rename = "VpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputDataConfig {
    #[serde(rename = "DocumentReaderConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_reader_config: Option<DocumentReaderConfig>,
    #[serde(rename = "InputFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_format: Option<String>,
    #[serde(rename = "S3Uri")]
    #[serde(default)]
    pub s3_uri: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutputDataConfig {
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "S3Uri")]
    #[serde(default)]
    pub s3_uri: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDocumentClassifierRequest {
    #[serde(rename = "DocumentClassifierArn")]
    #[serde(default)]
    pub document_classifier_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDocumentClassifierResponse {
    #[serde(rename = "DocumentClassifierProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_classifier_properties: Option<DocumentClassifierProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentClassifierProperties {
    #[serde(rename = "ClassifierMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifier_metadata: Option<ClassifierMetadata>,
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_role_arn: Option<String>,
    #[serde(rename = "DocumentClassifierArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_classifier_arn: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "FlywheelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flywheel_arn: Option<String>,
    #[serde(rename = "InputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_config: Option<DocumentClassifierInputDataConfig>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Mode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "ModelKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_kms_key_id: Option<String>,
    #[serde(rename = "OutputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_data_config: Option<DocumentClassifierOutputDataConfig>,
    #[serde(rename = "SourceModelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_model_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "SubmitTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<f64>,
    #[serde(rename = "TrainingEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_end_time: Option<f64>,
    #[serde(rename = "TrainingStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_start_time: Option<f64>,
    #[serde(rename = "VersionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    #[serde(rename = "VpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClassifierMetadata {
    #[serde(rename = "EvaluationMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_metrics: Option<ClassifierEvaluationMetrics>,
    #[serde(rename = "NumberOfLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_labels: Option<i32>,
    #[serde(rename = "NumberOfTestDocuments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_test_documents: Option<i32>,
    #[serde(rename = "NumberOfTrainedDocuments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_trained_documents: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClassifierEvaluationMetrics {
    #[serde(rename = "Accuracy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accuracy: Option<f64>,
    #[serde(rename = "F1Score")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f1_score: Option<f64>,
    #[serde(rename = "HammingLoss")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hamming_loss: Option<f64>,
    #[serde(rename = "MicroF1Score")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub micro_f1_score: Option<f64>,
    #[serde(rename = "MicroPrecision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub micro_precision: Option<f64>,
    #[serde(rename = "MicroRecall")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub micro_recall: Option<f64>,
    #[serde(rename = "Precision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<f64>,
    #[serde(rename = "Recall")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recall: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDominantLanguageDetectionJobRequest {
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDominantLanguageDetectionJobResponse {
    #[serde(rename = "DominantLanguageDetectionJobProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dominant_language_detection_job_properties: Option<DominantLanguageDetectionJobProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DominantLanguageDetectionJobProperties {
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_role_arn: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "InputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_config: Option<InputDataConfig>,
    #[serde(rename = "JobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "OutputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_data_config: Option<OutputDataConfig>,
    #[serde(rename = "SubmitTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<f64>,
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    #[serde(rename = "VpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEndpointRequest {
    #[serde(rename = "EndpointArn")]
    #[serde(default)]
    pub endpoint_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEndpointResponse {
    #[serde(rename = "EndpointProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_properties: Option<EndpointProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EndpointProperties {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "CurrentInferenceUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_inference_units: Option<i32>,
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_role_arn: Option<String>,
    #[serde(rename = "DesiredDataAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_data_access_role_arn: Option<String>,
    #[serde(rename = "DesiredInferenceUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_inference_units: Option<i32>,
    #[serde(rename = "DesiredModelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_model_arn: Option<String>,
    #[serde(rename = "EndpointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_arn: Option<String>,
    #[serde(rename = "FlywheelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flywheel_arn: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "ModelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEntitiesDetectionJobRequest {
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEntitiesDetectionJobResponse {
    #[serde(rename = "EntitiesDetectionJobProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities_detection_job_properties: Option<EntitiesDetectionJobProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EntitiesDetectionJobProperties {
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_role_arn: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "EntityRecognizerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_recognizer_arn: Option<String>,
    #[serde(rename = "FlywheelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flywheel_arn: Option<String>,
    #[serde(rename = "InputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_config: Option<InputDataConfig>,
    #[serde(rename = "JobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "OutputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_data_config: Option<OutputDataConfig>,
    #[serde(rename = "SubmitTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<f64>,
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    #[serde(rename = "VpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEntityRecognizerRequest {
    #[serde(rename = "EntityRecognizerArn")]
    #[serde(default)]
    pub entity_recognizer_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEntityRecognizerResponse {
    #[serde(rename = "EntityRecognizerProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_recognizer_properties: Option<EntityRecognizerProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EntityRecognizerProperties {
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_role_arn: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "EntityRecognizerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_recognizer_arn: Option<String>,
    #[serde(rename = "FlywheelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flywheel_arn: Option<String>,
    #[serde(rename = "InputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_config: Option<EntityRecognizerInputDataConfig>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "ModelKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_kms_key_id: Option<String>,
    #[serde(rename = "OutputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_data_config: Option<EntityRecognizerOutputDataConfig>,
    #[serde(rename = "RecognizerMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recognizer_metadata: Option<EntityRecognizerMetadata>,
    #[serde(rename = "SourceModelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_model_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "SubmitTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<f64>,
    #[serde(rename = "TrainingEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_end_time: Option<f64>,
    #[serde(rename = "TrainingStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub training_start_time: Option<f64>,
    #[serde(rename = "VersionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    #[serde(rename = "VpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EntityRecognizerOutputDataConfig {
    #[serde(rename = "FlywheelStatsS3Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flywheel_stats_s3_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EntityRecognizerMetadata {
    #[serde(rename = "EntityTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_types: Option<Vec<EntityRecognizerMetadataEntityTypesListItem>>,
    #[serde(rename = "EvaluationMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_metrics: Option<EntityRecognizerEvaluationMetrics>,
    #[serde(rename = "NumberOfTestDocuments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_test_documents: Option<i32>,
    #[serde(rename = "NumberOfTrainedDocuments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_trained_documents: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EntityRecognizerMetadataEntityTypesListItem {
    #[serde(rename = "EvaluationMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_metrics: Option<EntityTypesEvaluationMetrics>,
    #[serde(rename = "NumberOfTrainMentions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_train_mentions: Option<i32>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EntityTypesEvaluationMetrics {
    #[serde(rename = "F1Score")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f1_score: Option<f64>,
    #[serde(rename = "Precision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<f64>,
    #[serde(rename = "Recall")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recall: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EntityRecognizerEvaluationMetrics {
    #[serde(rename = "F1Score")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f1_score: Option<f64>,
    #[serde(rename = "Precision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<f64>,
    #[serde(rename = "Recall")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recall: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEventsDetectionJobRequest {
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEventsDetectionJobResponse {
    #[serde(rename = "EventsDetectionJobProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_detection_job_properties: Option<EventsDetectionJobProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventsDetectionJobProperties {
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_role_arn: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "InputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_config: Option<InputDataConfig>,
    #[serde(rename = "JobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "OutputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_data_config: Option<OutputDataConfig>,
    #[serde(rename = "SubmitTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<f64>,
    #[serde(rename = "TargetEventTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_event_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFlywheelIterationRequest {
    #[serde(rename = "FlywheelArn")]
    #[serde(default)]
    pub flywheel_arn: String,
    #[serde(rename = "FlywheelIterationId")]
    #[serde(default)]
    pub flywheel_iteration_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFlywheelIterationResponse {
    #[serde(rename = "FlywheelIterationProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flywheel_iteration_properties: Option<FlywheelIterationProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlywheelIterationProperties {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "EvaluatedModelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluated_model_arn: Option<String>,
    #[serde(rename = "EvaluatedModelMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluated_model_metrics: Option<FlywheelModelEvaluationMetrics>,
    #[serde(rename = "EvaluationManifestS3Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_manifest_s3_prefix: Option<String>,
    #[serde(rename = "FlywheelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flywheel_arn: Option<String>,
    #[serde(rename = "FlywheelIterationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flywheel_iteration_id: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TrainedModelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trained_model_arn: Option<String>,
    #[serde(rename = "TrainedModelMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trained_model_metrics: Option<FlywheelModelEvaluationMetrics>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlywheelModelEvaluationMetrics {
    #[serde(rename = "AverageAccuracy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_accuracy: Option<f64>,
    #[serde(rename = "AverageF1Score")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_f1_score: Option<f64>,
    #[serde(rename = "AveragePrecision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_precision: Option<f64>,
    #[serde(rename = "AverageRecall")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_recall: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFlywheelRequest {
    #[serde(rename = "FlywheelArn")]
    #[serde(default)]
    pub flywheel_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFlywheelResponse {
    #[serde(rename = "FlywheelProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flywheel_properties: Option<FlywheelProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlywheelProperties {
    #[serde(rename = "ActiveModelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_model_arn: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_role_arn: Option<String>,
    #[serde(rename = "DataLakeS3Uri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_lake_s3_uri: Option<String>,
    #[serde(rename = "DataSecurityConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_security_config: Option<DataSecurityConfig>,
    #[serde(rename = "FlywheelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flywheel_arn: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "LatestFlywheelIteration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_flywheel_iteration: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "ModelType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TaskConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_config: Option<TaskConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeKeyPhrasesDetectionJobRequest {
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeKeyPhrasesDetectionJobResponse {
    #[serde(rename = "KeyPhrasesDetectionJobProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_phrases_detection_job_properties: Option<KeyPhrasesDetectionJobProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KeyPhrasesDetectionJobProperties {
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_role_arn: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "InputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_config: Option<InputDataConfig>,
    #[serde(rename = "JobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "OutputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_data_config: Option<OutputDataConfig>,
    #[serde(rename = "SubmitTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<f64>,
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    #[serde(rename = "VpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePiiEntitiesDetectionJobRequest {
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePiiEntitiesDetectionJobResponse {
    #[serde(rename = "PiiEntitiesDetectionJobProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pii_entities_detection_job_properties: Option<PiiEntitiesDetectionJobProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PiiEntitiesDetectionJobProperties {
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_role_arn: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "InputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_config: Option<InputDataConfig>,
    #[serde(rename = "JobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Mode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "OutputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_data_config: Option<PiiOutputDataConfig>,
    #[serde(rename = "RedactionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redaction_config: Option<RedactionConfig>,
    #[serde(rename = "SubmitTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PiiOutputDataConfig {
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "S3Uri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RedactionConfig {
    #[serde(rename = "MaskCharacter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_character: Option<String>,
    #[serde(rename = "MaskMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_mode: Option<String>,
    #[serde(rename = "PiiEntityTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pii_entity_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeResourcePolicyRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeResourcePolicyResponse {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "PolicyRevisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_revision_id: Option<String>,
    #[serde(rename = "ResourcePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSentimentDetectionJobRequest {
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSentimentDetectionJobResponse {
    #[serde(rename = "SentimentDetectionJobProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiment_detection_job_properties: Option<SentimentDetectionJobProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SentimentDetectionJobProperties {
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_role_arn: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "InputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_config: Option<InputDataConfig>,
    #[serde(rename = "JobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "OutputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_data_config: Option<OutputDataConfig>,
    #[serde(rename = "SubmitTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<f64>,
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    #[serde(rename = "VpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTargetedSentimentDetectionJobRequest {
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTargetedSentimentDetectionJobResponse {
    #[serde(rename = "TargetedSentimentDetectionJobProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targeted_sentiment_detection_job_properties:
        Option<TargetedSentimentDetectionJobProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TargetedSentimentDetectionJobProperties {
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_role_arn: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "InputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_config: Option<InputDataConfig>,
    #[serde(rename = "JobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "OutputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_data_config: Option<OutputDataConfig>,
    #[serde(rename = "SubmitTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<f64>,
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    #[serde(rename = "VpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTopicsDetectionJobRequest {
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTopicsDetectionJobResponse {
    #[serde(rename = "TopicsDetectionJobProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics_detection_job_properties: Option<TopicsDetectionJobProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicsDetectionJobProperties {
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_role_arn: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "InputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_data_config: Option<InputDataConfig>,
    #[serde(rename = "JobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "NumberOfTopics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_topics: Option<i32>,
    #[serde(rename = "OutputDataConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_data_config: Option<OutputDataConfig>,
    #[serde(rename = "SubmitTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<f64>,
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    #[serde(rename = "VpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectDominantLanguageRequest {
    #[serde(rename = "Text")]
    #[serde(default)]
    pub text: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectDominantLanguageResponse {
    #[serde(rename = "Languages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<DominantLanguage>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectEntitiesRequest {
    #[serde(rename = "Bytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<String>,
    #[serde(rename = "DocumentReaderConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_reader_config: Option<DocumentReaderConfig>,
    #[serde(rename = "EndpointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_arn: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "Text")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectEntitiesResponse {
    #[serde(rename = "Blocks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<Block>>,
    #[serde(rename = "DocumentMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_metadata: Option<DocumentMetadata>,
    #[serde(rename = "DocumentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_type: Option<Vec<DocumentTypeListItem>>,
    #[serde(rename = "Entities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<Entity>>,
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ErrorsListItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Block {
    #[serde(rename = "BlockType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_type: Option<String>,
    #[serde(rename = "Geometry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<Geometry>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Page")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "Relationships")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationships: Option<Vec<RelationshipsListItem>>,
    #[serde(rename = "Text")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Geometry {
    #[serde(rename = "BoundingBox")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    #[serde(rename = "Polygon")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polygon: Option<Vec<Point>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BoundingBox {
    #[serde(rename = "Height")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f32>,
    #[serde(rename = "Left")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<f32>,
    #[serde(rename = "Top")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<f32>,
    #[serde(rename = "Width")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Point {
    #[serde(rename = "X")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f32>,
    #[serde(rename = "Y")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RelationshipsListItem {
    #[serde(rename = "Ids")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectKeyPhrasesRequest {
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    pub language_code: String,
    #[serde(rename = "Text")]
    #[serde(default)]
    pub text: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectKeyPhrasesResponse {
    #[serde(rename = "KeyPhrases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_phrases: Option<Vec<KeyPhrase>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectPiiEntitiesRequest {
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    pub language_code: String,
    #[serde(rename = "Text")]
    #[serde(default)]
    pub text: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectPiiEntitiesResponse {
    #[serde(rename = "Entities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<PiiEntity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PiiEntity {
    #[serde(rename = "BeginOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_offset: Option<i32>,
    #[serde(rename = "EndOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_offset: Option<i32>,
    #[serde(rename = "Score")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectSentimentRequest {
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    pub language_code: String,
    #[serde(rename = "Text")]
    #[serde(default)]
    pub text: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectSentimentResponse {
    #[serde(rename = "Sentiment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiment: Option<String>,
    #[serde(rename = "SentimentScore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiment_score: Option<SentimentScore>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectSyntaxRequest {
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    pub language_code: String,
    #[serde(rename = "Text")]
    #[serde(default)]
    pub text: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectSyntaxResponse {
    #[serde(rename = "SyntaxTokens")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syntax_tokens: Option<Vec<SyntaxToken>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectTargetedSentimentRequest {
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    pub language_code: String,
    #[serde(rename = "Text")]
    #[serde(default)]
    pub text: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectTargetedSentimentResponse {
    #[serde(rename = "Entities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<TargetedSentimentEntity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectToxicContentRequest {
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    pub language_code: String,
    #[serde(rename = "TextSegments")]
    #[serde(default)]
    pub text_segments: Vec<TextSegment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TextSegment {
    #[serde(rename = "Text")]
    #[serde(default)]
    pub text: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectToxicContentResponse {
    #[serde(rename = "ResultList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_list: Option<Vec<ToxicLabels>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ToxicLabels {
    #[serde(rename = "Labels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<ToxicContent>>,
    #[serde(rename = "Toxicity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toxicity: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ToxicContent {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Score")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportModelRequest {
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_role_arn: Option<String>,
    #[serde(rename = "ModelKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_kms_key_id: Option<String>,
    #[serde(rename = "ModelName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_name: Option<String>,
    #[serde(rename = "SourceModelArn")]
    #[serde(default)]
    pub source_model_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VersionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportModelResponse {
    #[serde(rename = "ModelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDatasetsRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<DatasetFilter>,
    #[serde(rename = "FlywheelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flywheel_arn: Option<String>,
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
pub struct DatasetFilter {
    #[serde(rename = "CreationTimeAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_after: Option<f64>,
    #[serde(rename = "CreationTimeBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_before: Option<f64>,
    #[serde(rename = "DatasetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDatasetsResponse {
    #[serde(rename = "DatasetPropertiesList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_properties_list: Option<Vec<DatasetProperties>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDocumentClassificationJobsRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<DocumentClassificationJobFilter>,
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
pub struct DocumentClassificationJobFilter {
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "SubmitTimeAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_after: Option<f64>,
    #[serde(rename = "SubmitTimeBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_before: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDocumentClassificationJobsResponse {
    #[serde(rename = "DocumentClassificationJobPropertiesList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_classification_job_properties_list:
        Option<Vec<DocumentClassificationJobProperties>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDocumentClassifierSummariesRequest {
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
pub struct ListDocumentClassifierSummariesResponse {
    #[serde(rename = "DocumentClassifierSummariesList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_classifier_summaries_list: Option<Vec<DocumentClassifierSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentClassifierSummary {
    #[serde(rename = "DocumentClassifierName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_classifier_name: Option<String>,
    #[serde(rename = "LatestVersionCreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_created_at: Option<f64>,
    #[serde(rename = "LatestVersionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_name: Option<String>,
    #[serde(rename = "LatestVersionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_status: Option<String>,
    #[serde(rename = "NumberOfVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_versions: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDocumentClassifiersRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<DocumentClassifierFilter>,
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
pub struct DocumentClassifierFilter {
    #[serde(rename = "DocumentClassifierName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_classifier_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "SubmitTimeAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_after: Option<f64>,
    #[serde(rename = "SubmitTimeBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_before: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDocumentClassifiersResponse {
    #[serde(rename = "DocumentClassifierPropertiesList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_classifier_properties_list: Option<Vec<DocumentClassifierProperties>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDominantLanguageDetectionJobsRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<DominantLanguageDetectionJobFilter>,
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
pub struct DominantLanguageDetectionJobFilter {
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "SubmitTimeAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_after: Option<f64>,
    #[serde(rename = "SubmitTimeBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_before: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDominantLanguageDetectionJobsResponse {
    #[serde(rename = "DominantLanguageDetectionJobPropertiesList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dominant_language_detection_job_properties_list:
        Option<Vec<DominantLanguageDetectionJobProperties>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEndpointsRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<EndpointFilter>,
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
pub struct EndpointFilter {
    #[serde(rename = "CreationTimeAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_after: Option<f64>,
    #[serde(rename = "CreationTimeBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_before: Option<f64>,
    #[serde(rename = "ModelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEndpointsResponse {
    #[serde(rename = "EndpointPropertiesList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_properties_list: Option<Vec<EndpointProperties>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEntitiesDetectionJobsRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<EntitiesDetectionJobFilter>,
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
pub struct EntitiesDetectionJobFilter {
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "SubmitTimeAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_after: Option<f64>,
    #[serde(rename = "SubmitTimeBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_before: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEntitiesDetectionJobsResponse {
    #[serde(rename = "EntitiesDetectionJobPropertiesList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities_detection_job_properties_list: Option<Vec<EntitiesDetectionJobProperties>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEntityRecognizerSummariesRequest {
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
pub struct ListEntityRecognizerSummariesResponse {
    #[serde(rename = "EntityRecognizerSummariesList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_recognizer_summaries_list: Option<Vec<EntityRecognizerSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EntityRecognizerSummary {
    #[serde(rename = "LatestVersionCreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_created_at: Option<f64>,
    #[serde(rename = "LatestVersionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_name: Option<String>,
    #[serde(rename = "LatestVersionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_status: Option<String>,
    #[serde(rename = "NumberOfVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_versions: Option<i32>,
    #[serde(rename = "RecognizerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recognizer_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEntityRecognizersRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<EntityRecognizerFilter>,
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
pub struct EntityRecognizerFilter {
    #[serde(rename = "RecognizerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recognizer_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "SubmitTimeAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_after: Option<f64>,
    #[serde(rename = "SubmitTimeBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_before: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEntityRecognizersResponse {
    #[serde(rename = "EntityRecognizerPropertiesList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_recognizer_properties_list: Option<Vec<EntityRecognizerProperties>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEventsDetectionJobsRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<EventsDetectionJobFilter>,
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
pub struct EventsDetectionJobFilter {
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "SubmitTimeAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_after: Option<f64>,
    #[serde(rename = "SubmitTimeBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_before: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEventsDetectionJobsResponse {
    #[serde(rename = "EventsDetectionJobPropertiesList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_detection_job_properties_list: Option<Vec<EventsDetectionJobProperties>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFlywheelIterationHistoryRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<FlywheelIterationFilter>,
    #[serde(rename = "FlywheelArn")]
    #[serde(default)]
    pub flywheel_arn: String,
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
pub struct FlywheelIterationFilter {
    #[serde(rename = "CreationTimeAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_after: Option<f64>,
    #[serde(rename = "CreationTimeBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_before: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFlywheelIterationHistoryResponse {
    #[serde(rename = "FlywheelIterationPropertiesList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flywheel_iteration_properties_list: Option<Vec<FlywheelIterationProperties>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFlywheelsRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<FlywheelFilter>,
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
pub struct FlywheelFilter {
    #[serde(rename = "CreationTimeAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_after: Option<f64>,
    #[serde(rename = "CreationTimeBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_before: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFlywheelsResponse {
    #[serde(rename = "FlywheelSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flywheel_summary_list: Option<Vec<FlywheelSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlywheelSummary {
    #[serde(rename = "ActiveModelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_model_arn: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "DataLakeS3Uri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_lake_s3_uri: Option<String>,
    #[serde(rename = "FlywheelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flywheel_arn: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "LatestFlywheelIteration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_flywheel_iteration: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "ModelType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListKeyPhrasesDetectionJobsRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<KeyPhrasesDetectionJobFilter>,
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
pub struct KeyPhrasesDetectionJobFilter {
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "SubmitTimeAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_after: Option<f64>,
    #[serde(rename = "SubmitTimeBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_before: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListKeyPhrasesDetectionJobsResponse {
    #[serde(rename = "KeyPhrasesDetectionJobPropertiesList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_phrases_detection_job_properties_list: Option<Vec<KeyPhrasesDetectionJobProperties>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPiiEntitiesDetectionJobsRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<PiiEntitiesDetectionJobFilter>,
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
pub struct PiiEntitiesDetectionJobFilter {
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "SubmitTimeAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_after: Option<f64>,
    #[serde(rename = "SubmitTimeBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_before: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPiiEntitiesDetectionJobsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PiiEntitiesDetectionJobPropertiesList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pii_entities_detection_job_properties_list: Option<Vec<PiiEntitiesDetectionJobProperties>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSentimentDetectionJobsRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<SentimentDetectionJobFilter>,
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
pub struct SentimentDetectionJobFilter {
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "SubmitTimeAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_after: Option<f64>,
    #[serde(rename = "SubmitTimeBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_before: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSentimentDetectionJobsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SentimentDetectionJobPropertiesList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentiment_detection_job_properties_list: Option<Vec<SentimentDetectionJobProperties>>,
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
pub struct ListTargetedSentimentDetectionJobsRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<TargetedSentimentDetectionJobFilter>,
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
pub struct TargetedSentimentDetectionJobFilter {
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "SubmitTimeAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_after: Option<f64>,
    #[serde(rename = "SubmitTimeBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_before: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTargetedSentimentDetectionJobsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TargetedSentimentDetectionJobPropertiesList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targeted_sentiment_detection_job_properties_list:
        Option<Vec<TargetedSentimentDetectionJobProperties>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTopicsDetectionJobsRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<TopicsDetectionJobFilter>,
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
pub struct TopicsDetectionJobFilter {
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "SubmitTimeAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_after: Option<f64>,
    #[serde(rename = "SubmitTimeBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time_before: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTopicsDetectionJobsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TopicsDetectionJobPropertiesList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics_detection_job_properties_list: Option<Vec<TopicsDetectionJobProperties>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResourcePolicyRequest {
    #[serde(rename = "PolicyRevisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_revision_id: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "ResourcePolicy")]
    #[serde(default)]
    pub resource_policy: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResourcePolicyResponse {
    #[serde(rename = "PolicyRevisionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_revision_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDocumentClassificationJobRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(default)]
    pub data_access_role_arn: String,
    #[serde(rename = "DocumentClassifierArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_classifier_arn: Option<String>,
    #[serde(rename = "FlywheelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flywheel_arn: Option<String>,
    #[serde(rename = "InputDataConfig")]
    #[serde(default)]
    pub input_data_config: InputDataConfig,
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "OutputDataConfig")]
    #[serde(default)]
    pub output_data_config: OutputDataConfig,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    #[serde(rename = "VpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDocumentClassificationJobResponse {
    #[serde(rename = "DocumentClassifierArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_classifier_arn: Option<String>,
    #[serde(rename = "JobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDominantLanguageDetectionJobRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(default)]
    pub data_access_role_arn: String,
    #[serde(rename = "InputDataConfig")]
    #[serde(default)]
    pub input_data_config: InputDataConfig,
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "OutputDataConfig")]
    #[serde(default)]
    pub output_data_config: OutputDataConfig,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    #[serde(rename = "VpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDominantLanguageDetectionJobResponse {
    #[serde(rename = "JobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartEntitiesDetectionJobRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(default)]
    pub data_access_role_arn: String,
    #[serde(rename = "EntityRecognizerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_recognizer_arn: Option<String>,
    #[serde(rename = "FlywheelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flywheel_arn: Option<String>,
    #[serde(rename = "InputDataConfig")]
    #[serde(default)]
    pub input_data_config: InputDataConfig,
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    pub language_code: String,
    #[serde(rename = "OutputDataConfig")]
    #[serde(default)]
    pub output_data_config: OutputDataConfig,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    #[serde(rename = "VpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartEntitiesDetectionJobResponse {
    #[serde(rename = "EntityRecognizerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_recognizer_arn: Option<String>,
    #[serde(rename = "JobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartEventsDetectionJobRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(default)]
    pub data_access_role_arn: String,
    #[serde(rename = "InputDataConfig")]
    #[serde(default)]
    pub input_data_config: InputDataConfig,
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    pub language_code: String,
    #[serde(rename = "OutputDataConfig")]
    #[serde(default)]
    pub output_data_config: OutputDataConfig,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TargetEventTypes")]
    #[serde(default)]
    pub target_event_types: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartEventsDetectionJobResponse {
    #[serde(rename = "JobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartFlywheelIterationRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "FlywheelArn")]
    #[serde(default)]
    pub flywheel_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartFlywheelIterationResponse {
    #[serde(rename = "FlywheelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flywheel_arn: Option<String>,
    #[serde(rename = "FlywheelIterationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flywheel_iteration_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartKeyPhrasesDetectionJobRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(default)]
    pub data_access_role_arn: String,
    #[serde(rename = "InputDataConfig")]
    #[serde(default)]
    pub input_data_config: InputDataConfig,
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    pub language_code: String,
    #[serde(rename = "OutputDataConfig")]
    #[serde(default)]
    pub output_data_config: OutputDataConfig,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    #[serde(rename = "VpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartKeyPhrasesDetectionJobResponse {
    #[serde(rename = "JobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartPiiEntitiesDetectionJobRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(default)]
    pub data_access_role_arn: String,
    #[serde(rename = "InputDataConfig")]
    #[serde(default)]
    pub input_data_config: InputDataConfig,
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    pub language_code: String,
    #[serde(rename = "Mode")]
    #[serde(default)]
    pub mode: String,
    #[serde(rename = "OutputDataConfig")]
    #[serde(default)]
    pub output_data_config: OutputDataConfig,
    #[serde(rename = "RedactionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redaction_config: Option<RedactionConfig>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartPiiEntitiesDetectionJobResponse {
    #[serde(rename = "JobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSentimentDetectionJobRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(default)]
    pub data_access_role_arn: String,
    #[serde(rename = "InputDataConfig")]
    #[serde(default)]
    pub input_data_config: InputDataConfig,
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    pub language_code: String,
    #[serde(rename = "OutputDataConfig")]
    #[serde(default)]
    pub output_data_config: OutputDataConfig,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    #[serde(rename = "VpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSentimentDetectionJobResponse {
    #[serde(rename = "JobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartTargetedSentimentDetectionJobRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(default)]
    pub data_access_role_arn: String,
    #[serde(rename = "InputDataConfig")]
    #[serde(default)]
    pub input_data_config: InputDataConfig,
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    pub language_code: String,
    #[serde(rename = "OutputDataConfig")]
    #[serde(default)]
    pub output_data_config: OutputDataConfig,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    #[serde(rename = "VpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartTargetedSentimentDetectionJobResponse {
    #[serde(rename = "JobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartTopicsDetectionJobRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(default)]
    pub data_access_role_arn: String,
    #[serde(rename = "InputDataConfig")]
    #[serde(default)]
    pub input_data_config: InputDataConfig,
    #[serde(rename = "JobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "NumberOfTopics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_topics: Option<i32>,
    #[serde(rename = "OutputDataConfig")]
    #[serde(default)]
    pub output_data_config: OutputDataConfig,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    #[serde(rename = "VpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartTopicsDetectionJobResponse {
    #[serde(rename = "JobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopDominantLanguageDetectionJobRequest {
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopDominantLanguageDetectionJobResponse {
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopEntitiesDetectionJobRequest {
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopEntitiesDetectionJobResponse {
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopEventsDetectionJobRequest {
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopEventsDetectionJobResponse {
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopKeyPhrasesDetectionJobRequest {
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopKeyPhrasesDetectionJobResponse {
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopPiiEntitiesDetectionJobRequest {
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopPiiEntitiesDetectionJobResponse {
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopSentimentDetectionJobRequest {
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopSentimentDetectionJobResponse {
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopTargetedSentimentDetectionJobRequest {
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopTargetedSentimentDetectionJobResponse {
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopTrainingDocumentClassifierRequest {
    #[serde(rename = "DocumentClassifierArn")]
    #[serde(default)]
    pub document_classifier_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopTrainingDocumentClassifierResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopTrainingEntityRecognizerRequest {
    #[serde(rename = "EntityRecognizerArn")]
    #[serde(default)]
    pub entity_recognizer_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopTrainingEntityRecognizerResponse {}

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
pub struct UpdateEndpointRequest {
    #[serde(rename = "DesiredDataAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_data_access_role_arn: Option<String>,
    #[serde(rename = "DesiredInferenceUnits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_inference_units: Option<i32>,
    #[serde(rename = "DesiredModelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_model_arn: Option<String>,
    #[serde(rename = "EndpointArn")]
    #[serde(default)]
    pub endpoint_arn: String,
    #[serde(rename = "FlywheelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flywheel_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEndpointResponse {
    #[serde(rename = "DesiredModelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_model_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFlywheelRequest {
    #[serde(rename = "ActiveModelArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_model_arn: Option<String>,
    #[serde(rename = "DataAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_access_role_arn: Option<String>,
    #[serde(rename = "DataSecurityConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_security_config: Option<UpdateDataSecurityConfig>,
    #[serde(rename = "FlywheelArn")]
    #[serde(default)]
    pub flywheel_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDataSecurityConfig {
    #[serde(rename = "ModelKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_kms_key_id: Option<String>,
    #[serde(rename = "VolumeKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_kms_key_id: Option<String>,
    #[serde(rename = "VpcConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFlywheelResponse {
    #[serde(rename = "FlywheelProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flywheel_properties: Option<FlywheelProperties>,
}
