//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-textract

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyzeDocumentRequest {
    #[serde(rename = "AdaptersConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adapters_config: Option<AdaptersConfig>,
    #[serde(rename = "Document")]
    #[serde(default)]
    pub document: Document,
    #[serde(rename = "FeatureTypes")]
    #[serde(default)]
    pub feature_types: Vec<String>,
    #[serde(rename = "HumanLoopConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_loop_config: Option<HumanLoopConfig>,
    #[serde(rename = "QueriesConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queries_config: Option<QueriesConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdaptersConfig {
    #[serde(rename = "Adapters")]
    #[serde(default)]
    pub adapters: Vec<Adapter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Adapter {
    #[serde(rename = "AdapterId")]
    #[serde(default)]
    pub adapter_id: String,
    #[serde(rename = "Pages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<Vec<String>>,
    #[serde(rename = "Version")]
    #[serde(default)]
    pub version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Document {
    #[serde(rename = "Bytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<String>,
    #[serde(rename = "S3Object")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object: Option<S3Object>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Object {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HumanLoopConfig {
    #[serde(rename = "DataAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_attributes: Option<HumanLoopDataAttributes>,
    #[serde(rename = "FlowDefinitionArn")]
    #[serde(default)]
    pub flow_definition_arn: String,
    #[serde(rename = "HumanLoopName")]
    #[serde(default)]
    pub human_loop_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HumanLoopDataAttributes {
    #[serde(rename = "ContentClassifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_classifiers: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueriesConfig {
    #[serde(rename = "Queries")]
    #[serde(default)]
    pub queries: Vec<Query>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Query {
    #[serde(rename = "Alias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "Pages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<Vec<String>>,
    #[serde(rename = "Text")]
    #[serde(default)]
    pub text: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyzeDocumentResponse {
    #[serde(rename = "AnalyzeDocumentModelVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyze_document_model_version: Option<String>,
    #[serde(rename = "Blocks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<Block>>,
    #[serde(rename = "DocumentMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_metadata: Option<DocumentMetadata>,
    #[serde(rename = "HumanLoopActivationOutput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_loop_activation_output: Option<HumanLoopActivationOutput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Block {
    #[serde(rename = "BlockType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_type: Option<String>,
    #[serde(rename = "ColumnIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_index: Option<i32>,
    #[serde(rename = "ColumnSpan")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_span: Option<i32>,
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(rename = "EntityTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_types: Option<Vec<String>>,
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
    #[serde(rename = "Query")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<Query>,
    #[serde(rename = "Relationships")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationships: Option<Vec<Relationship>>,
    #[serde(rename = "RowIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_index: Option<i32>,
    #[serde(rename = "RowSpan")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_span: Option<i32>,
    #[serde(rename = "SelectionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_status: Option<String>,
    #[serde(rename = "Text")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "TextType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_type: Option<String>,
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
    #[serde(rename = "RotationAngle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_angle: Option<f32>,
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
pub struct Relationship {
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
pub struct DocumentMetadata {
    #[serde(rename = "Pages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HumanLoopActivationOutput {
    #[serde(rename = "HumanLoopActivationConditionsEvaluationResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_loop_activation_conditions_evaluation_results: Option<String>,
    #[serde(rename = "HumanLoopActivationReasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_loop_activation_reasons: Option<Vec<String>>,
    #[serde(rename = "HumanLoopArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub human_loop_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyzeExpenseRequest {
    #[serde(rename = "Document")]
    #[serde(default)]
    pub document: Document,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyzeExpenseResponse {
    #[serde(rename = "DocumentMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_metadata: Option<DocumentMetadata>,
    #[serde(rename = "ExpenseDocuments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expense_documents: Option<Vec<ExpenseDocument>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExpenseDocument {
    #[serde(rename = "Blocks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<Block>>,
    #[serde(rename = "ExpenseIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expense_index: Option<i32>,
    #[serde(rename = "LineItemGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_item_groups: Option<Vec<LineItemGroup>>,
    #[serde(rename = "SummaryFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_fields: Option<Vec<ExpenseField>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LineItemGroup {
    #[serde(rename = "LineItemGroupIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_item_group_index: Option<i32>,
    #[serde(rename = "LineItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<LineItemFields>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LineItemFields {
    #[serde(rename = "LineItemExpenseFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_item_expense_fields: Option<Vec<ExpenseField>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExpenseField {
    #[serde(rename = "Currency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<ExpenseCurrency>,
    #[serde(rename = "GroupProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_properties: Option<Vec<ExpenseGroupProperty>>,
    #[serde(rename = "LabelDetection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_detection: Option<ExpenseDetection>,
    #[serde(rename = "PageNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<i32>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ExpenseType>,
    #[serde(rename = "ValueDetection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_detection: Option<ExpenseDetection>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExpenseCurrency {
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExpenseGroupProperty {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Types")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExpenseDetection {
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(rename = "Geometry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<Geometry>,
    #[serde(rename = "Text")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExpenseType {
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(rename = "Text")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyzeIDRequest {
    #[serde(rename = "DocumentPages")]
    #[serde(default)]
    pub document_pages: Vec<Document>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyzeIDResponse {
    #[serde(rename = "AnalyzeIDModelVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyze_i_d_model_version: Option<String>,
    #[serde(rename = "DocumentMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_metadata: Option<DocumentMetadata>,
    #[serde(rename = "IdentityDocuments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_documents: Option<Vec<IdentityDocument>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IdentityDocument {
    #[serde(rename = "Blocks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<Block>>,
    #[serde(rename = "DocumentIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_index: Option<i32>,
    #[serde(rename = "IdentityDocumentFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_document_fields: Option<Vec<IdentityDocumentField>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IdentityDocumentField {
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<AnalyzeIDDetections>,
    #[serde(rename = "ValueDetection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_detection: Option<AnalyzeIDDetections>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalyzeIDDetections {
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(rename = "NormalizedValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized_value: Option<NormalizedValue>,
    #[serde(rename = "Text")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NormalizedValue {
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "ValueType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAdapterRequest {
    #[serde(rename = "AdapterName")]
    #[serde(default)]
    pub adapter_name: String,
    #[serde(rename = "AutoUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_update: Option<String>,
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "FeatureTypes")]
    #[serde(default)]
    pub feature_types: Vec<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAdapterResponse {
    #[serde(rename = "AdapterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adapter_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAdapterVersionRequest {
    #[serde(rename = "AdapterId")]
    #[serde(default)]
    pub adapter_id: String,
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "DatasetConfig")]
    #[serde(default)]
    pub dataset_config: AdapterVersionDatasetConfig,
    #[serde(rename = "KMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_key_id: Option<String>,
    #[serde(rename = "OutputConfig")]
    #[serde(default)]
    pub output_config: OutputConfig,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdapterVersionDatasetConfig {
    #[serde(rename = "ManifestS3Object")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_s3_object: Option<S3Object>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutputConfig {
    #[serde(rename = "S3Bucket")]
    #[serde(default)]
    pub s3_bucket: String,
    #[serde(rename = "S3Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAdapterVersionResponse {
    #[serde(rename = "AdapterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adapter_id: Option<String>,
    #[serde(rename = "AdapterVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adapter_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAdapterRequest {
    #[serde(rename = "AdapterId")]
    #[serde(default)]
    pub adapter_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAdapterResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAdapterVersionRequest {
    #[serde(rename = "AdapterId")]
    #[serde(default)]
    pub adapter_id: String,
    #[serde(rename = "AdapterVersion")]
    #[serde(default)]
    pub adapter_version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAdapterVersionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectDocumentTextRequest {
    #[serde(rename = "Document")]
    #[serde(default)]
    pub document: Document,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectDocumentTextResponse {
    #[serde(rename = "Blocks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<Block>>,
    #[serde(rename = "DetectDocumentTextModelVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detect_document_text_model_version: Option<String>,
    #[serde(rename = "DocumentMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_metadata: Option<DocumentMetadata>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAdapterRequest {
    #[serde(rename = "AdapterId")]
    #[serde(default)]
    pub adapter_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAdapterResponse {
    #[serde(rename = "AdapterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adapter_id: Option<String>,
    #[serde(rename = "AdapterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adapter_name: Option<String>,
    #[serde(rename = "AutoUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_update: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "FeatureTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_types: Option<Vec<String>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAdapterVersionRequest {
    #[serde(rename = "AdapterId")]
    #[serde(default)]
    pub adapter_id: String,
    #[serde(rename = "AdapterVersion")]
    #[serde(default)]
    pub adapter_version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAdapterVersionResponse {
    #[serde(rename = "AdapterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adapter_id: Option<String>,
    #[serde(rename = "AdapterVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adapter_version: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "DatasetConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_config: Option<AdapterVersionDatasetConfig>,
    #[serde(rename = "EvaluationMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_metrics: Option<Vec<AdapterVersionEvaluationMetric>>,
    #[serde(rename = "FeatureTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_types: Option<Vec<String>>,
    #[serde(rename = "KMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_key_id: Option<String>,
    #[serde(rename = "OutputConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_config: Option<OutputConfig>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdapterVersionEvaluationMetric {
    #[serde(rename = "AdapterVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adapter_version: Option<EvaluationMetric>,
    #[serde(rename = "Baseline")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline: Option<EvaluationMetric>,
    #[serde(rename = "FeatureType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluationMetric {
    #[serde(rename = "F1Score")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f1_score: Option<f32>,
    #[serde(rename = "Precision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precision: Option<f32>,
    #[serde(rename = "Recall")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recall: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDocumentAnalysisRequest {
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
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
pub struct GetDocumentAnalysisResponse {
    #[serde(rename = "AnalyzeDocumentModelVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyze_document_model_version: Option<String>,
    #[serde(rename = "Blocks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<Block>>,
    #[serde(rename = "DocumentMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_metadata: Option<DocumentMetadata>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "Warnings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<Warning>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Warning {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "Pages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<Vec<i32>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDocumentTextDetectionRequest {
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
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
pub struct GetDocumentTextDetectionResponse {
    #[serde(rename = "Blocks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<Block>>,
    #[serde(rename = "DetectDocumentTextModelVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detect_document_text_model_version: Option<String>,
    #[serde(rename = "DocumentMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_metadata: Option<DocumentMetadata>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "Warnings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<Warning>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetExpenseAnalysisRequest {
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
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
pub struct GetExpenseAnalysisResponse {
    #[serde(rename = "AnalyzeExpenseModelVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyze_expense_model_version: Option<String>,
    #[serde(rename = "DocumentMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_metadata: Option<DocumentMetadata>,
    #[serde(rename = "ExpenseDocuments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expense_documents: Option<Vec<ExpenseDocument>>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "Warnings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<Warning>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLendingAnalysisRequest {
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
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
pub struct GetLendingAnalysisResponse {
    #[serde(rename = "AnalyzeLendingModelVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyze_lending_model_version: Option<String>,
    #[serde(rename = "DocumentMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_metadata: Option<DocumentMetadata>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Results")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<LendingResult>>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "Warnings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<Warning>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LendingResult {
    #[serde(rename = "Extractions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extractions: Option<Vec<Extraction>>,
    #[serde(rename = "Page")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "PageClassification")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_classification: Option<PageClassification>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Extraction {
    #[serde(rename = "ExpenseDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expense_document: Option<ExpenseDocument>,
    #[serde(rename = "IdentityDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_document: Option<IdentityDocument>,
    #[serde(rename = "LendingDocument")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lending_document: Option<LendingDocument>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LendingDocument {
    #[serde(rename = "LendingFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lending_fields: Option<Vec<LendingField>>,
    #[serde(rename = "SignatureDetections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_detections: Option<Vec<SignatureDetection>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LendingField {
    #[serde(rename = "KeyDetection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_detection: Option<LendingDetection>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "ValueDetections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_detections: Option<Vec<LendingDetection>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LendingDetection {
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(rename = "Geometry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<Geometry>,
    #[serde(rename = "SelectionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_status: Option<String>,
    #[serde(rename = "Text")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SignatureDetection {
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(rename = "Geometry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<Geometry>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PageClassification {
    #[serde(rename = "PageNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<Vec<Prediction>>,
    #[serde(rename = "PageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_type: Option<Vec<Prediction>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Prediction {
    #[serde(rename = "Confidence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLendingAnalysisSummaryRequest {
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLendingAnalysisSummaryResponse {
    #[serde(rename = "AnalyzeLendingModelVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyze_lending_model_version: Option<String>,
    #[serde(rename = "DocumentMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_metadata: Option<DocumentMetadata>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "Summary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<LendingSummary>,
    #[serde(rename = "Warnings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<Warning>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LendingSummary {
    #[serde(rename = "DocumentGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_groups: Option<Vec<DocumentGroup>>,
    #[serde(rename = "UndetectedDocumentTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub undetected_document_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentGroup {
    #[serde(rename = "DetectedSignatures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_signatures: Option<Vec<DetectedSignature>>,
    #[serde(rename = "SplitDocuments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_documents: Option<Vec<SplitDocument>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "UndetectedSignatures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub undetected_signatures: Option<Vec<UndetectedSignature>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetectedSignature {
    #[serde(rename = "Page")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SplitDocument {
    #[serde(rename = "Index")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    #[serde(rename = "Pages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pages: Option<Vec<i32>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UndetectedSignature {
    #[serde(rename = "Page")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAdapterVersionsRequest {
    #[serde(rename = "AdapterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adapter_id: Option<String>,
    #[serde(rename = "AfterCreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_creation_time: Option<f64>,
    #[serde(rename = "BeforeCreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_creation_time: Option<f64>,
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
pub struct ListAdapterVersionsResponse {
    #[serde(rename = "AdapterVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adapter_versions: Option<Vec<AdapterVersionOverview>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdapterVersionOverview {
    #[serde(rename = "AdapterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adapter_id: Option<String>,
    #[serde(rename = "AdapterVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adapter_version: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "FeatureTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_types: Option<Vec<String>>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAdaptersRequest {
    #[serde(rename = "AfterCreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_creation_time: Option<f64>,
    #[serde(rename = "BeforeCreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_creation_time: Option<f64>,
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
pub struct ListAdaptersResponse {
    #[serde(rename = "Adapters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adapters: Option<Vec<AdapterOverview>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdapterOverview {
    #[serde(rename = "AdapterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adapter_id: Option<String>,
    #[serde(rename = "AdapterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adapter_name: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "FeatureTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDocumentAnalysisRequest {
    #[serde(rename = "AdaptersConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adapters_config: Option<AdaptersConfig>,
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "DocumentLocation")]
    #[serde(default)]
    pub document_location: DocumentLocation,
    #[serde(rename = "FeatureTypes")]
    #[serde(default)]
    pub feature_types: Vec<String>,
    #[serde(rename = "JobTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    #[serde(rename = "KMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_key_id: Option<String>,
    #[serde(rename = "NotificationChannel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_channel: Option<NotificationChannel>,
    #[serde(rename = "OutputConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_config: Option<OutputConfig>,
    #[serde(rename = "QueriesConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queries_config: Option<QueriesConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentLocation {
    #[serde(rename = "S3Object")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object: Option<S3Object>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NotificationChannel {
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "SNSTopicArn")]
    #[serde(default)]
    pub s_n_s_topic_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDocumentAnalysisResponse {
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDocumentTextDetectionRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "DocumentLocation")]
    #[serde(default)]
    pub document_location: DocumentLocation,
    #[serde(rename = "JobTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    #[serde(rename = "KMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_key_id: Option<String>,
    #[serde(rename = "NotificationChannel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_channel: Option<NotificationChannel>,
    #[serde(rename = "OutputConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_config: Option<OutputConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDocumentTextDetectionResponse {
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartExpenseAnalysisRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "DocumentLocation")]
    #[serde(default)]
    pub document_location: DocumentLocation,
    #[serde(rename = "JobTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    #[serde(rename = "KMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_key_id: Option<String>,
    #[serde(rename = "NotificationChannel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_channel: Option<NotificationChannel>,
    #[serde(rename = "OutputConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_config: Option<OutputConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartExpenseAnalysisResponse {
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartLendingAnalysisRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "DocumentLocation")]
    #[serde(default)]
    pub document_location: DocumentLocation,
    #[serde(rename = "JobTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    #[serde(rename = "KMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_key_id: Option<String>,
    #[serde(rename = "NotificationChannel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_channel: Option<NotificationChannel>,
    #[serde(rename = "OutputConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_config: Option<OutputConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartLendingAnalysisResponse {
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAdapterRequest {
    #[serde(rename = "AdapterId")]
    #[serde(default)]
    pub adapter_id: String,
    #[serde(rename = "AdapterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adapter_name: Option<String>,
    #[serde(rename = "AutoUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_update: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAdapterResponse {
    #[serde(rename = "AdapterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adapter_id: Option<String>,
    #[serde(rename = "AdapterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adapter_name: Option<String>,
    #[serde(rename = "AutoUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_update: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "FeatureTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_types: Option<Vec<String>>,
}
