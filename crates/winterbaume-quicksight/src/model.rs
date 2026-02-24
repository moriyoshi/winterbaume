//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-quicksight

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchCreateTopicReviewedAnswerRequest {
    #[serde(rename = "Answers")]
    #[serde(default)]
    pub answers: Vec<CreateTopicReviewedAnswer>,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "TopicId")]
    #[serde(default)]
    pub topic_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTopicReviewedAnswer {
    #[serde(rename = "AnswerId")]
    #[serde(default)]
    pub answer_id: String,
    #[serde(rename = "DatasetArn")]
    #[serde(default)]
    pub dataset_arn: String,
    #[serde(rename = "Mir")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mir: Option<TopicIR>,
    #[serde(rename = "PrimaryVisual")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_visual: Option<TopicVisual>,
    #[serde(rename = "Question")]
    #[serde(default)]
    pub question: String,
    #[serde(rename = "Template")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<TopicTemplate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicIR {
    #[serde(rename = "ContributionAnalysis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contribution_analysis: Option<TopicIRContributionAnalysis>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Vec<TopicIRFilterOption>>>,
    #[serde(rename = "GroupByList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by_list: Option<Vec<TopicIRGroupBy>>,
    #[serde(rename = "Metrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<TopicIRMetric>>,
    #[serde(rename = "Sort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<TopicSortClause>,
    #[serde(rename = "Visual")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual: Option<VisualOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicIRContributionAnalysis {
    #[serde(rename = "Direction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    #[serde(rename = "Factors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub factors: Option<Vec<ContributionAnalysisFactor>>,
    #[serde(rename = "SortType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_type: Option<String>,
    #[serde(rename = "TimeRanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_ranges: Option<ContributionAnalysisTimeRanges>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContributionAnalysisFactor {
    #[serde(rename = "FieldName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContributionAnalysisTimeRanges {
    #[serde(rename = "EndRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_range: Option<TopicIRFilterOption>,
    #[serde(rename = "StartRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_range: Option<TopicIRFilterOption>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicIRFilterOption {
    #[serde(rename = "AggMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agg_metrics: Option<Vec<FilterAggMetrics>>,
    #[serde(rename = "Aggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<String>,
    #[serde(rename = "AggregationFunctionParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_function_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "AggregationPartitionBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_partition_by: Option<Vec<AggregationPartitionBy>>,
    #[serde(rename = "Anchor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<Anchor>,
    #[serde(rename = "Constant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant: Option<TopicConstantValue>,
    #[serde(rename = "FilterClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_class: Option<String>,
    #[serde(rename = "FilterType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_type: Option<String>,
    #[serde(rename = "Function")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<String>,
    #[serde(rename = "Inclusive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusive: Option<bool>,
    #[serde(rename = "Inverse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inverse: Option<bool>,
    #[serde(rename = "LastNextOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_next_offset: Option<TopicConstantValue>,
    #[serde(rename = "NullFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub null_filter: Option<String>,
    #[serde(rename = "OperandField")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operand_field: Option<Identifier>,
    #[serde(rename = "Range")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<TopicConstantValue>,
    #[serde(rename = "SortDirection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<String>,
    #[serde(rename = "TimeGranularity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_granularity: Option<String>,
    #[serde(rename = "TopBottomLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_bottom_limit: Option<TopicConstantValue>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterAggMetrics {
    #[serde(rename = "Function")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<String>,
    #[serde(rename = "MetricOperand")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_operand: Option<Identifier>,
    #[serde(rename = "SortDirection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Identifier {
    #[serde(rename = "Identity")]
    #[serde(default)]
    pub identity: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AggregationPartitionBy {
    #[serde(rename = "FieldName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(rename = "TimeGranularity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_granularity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Anchor {
    #[serde(rename = "AnchorType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor_type: Option<String>,
    #[serde(rename = "Offset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    #[serde(rename = "TimeGranularity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_granularity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicConstantValue {
    #[serde(rename = "ConstantType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_type: Option<String>,
    #[serde(rename = "Maximum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<String>,
    #[serde(rename = "Minimum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "ValueList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_list: Option<Vec<CollectiveConstantEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CollectiveConstantEntry {
    #[serde(rename = "ConstantType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_type: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicIRGroupBy {
    #[serde(rename = "DisplayFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_format: Option<String>,
    #[serde(rename = "DisplayFormatOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_format_options: Option<DisplayFormatOptions>,
    #[serde(rename = "FieldName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<Identifier>,
    #[serde(rename = "NamedEntity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_entity: Option<NamedEntityRef>,
    #[serde(rename = "Sort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<TopicSortClause>,
    #[serde(rename = "TimeGranularity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_granularity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisplayFormatOptions {
    #[serde(rename = "BlankCellFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blank_cell_format: Option<String>,
    #[serde(rename = "CurrencySymbol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_symbol: Option<String>,
    #[serde(rename = "DateFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_format: Option<String>,
    #[serde(rename = "DecimalSeparator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal_separator: Option<String>,
    #[serde(rename = "FractionDigits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraction_digits: Option<i32>,
    #[serde(rename = "GroupingSeparator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouping_separator: Option<String>,
    #[serde(rename = "NegativeFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_format: Option<NegativeFormat>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "Suffix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(rename = "UnitScaler")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_scaler: Option<String>,
    #[serde(rename = "UseBlankCellFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_blank_cell_format: Option<bool>,
    #[serde(rename = "UseGrouping")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_grouping: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NegativeFormat {
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "Suffix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NamedEntityRef {
    #[serde(rename = "NamedEntityName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_entity_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicSortClause {
    #[serde(rename = "Operand")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operand: Option<Identifier>,
    #[serde(rename = "SortDirection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicIRMetric {
    #[serde(rename = "CalculatedFieldReferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculated_field_references: Option<Vec<Identifier>>,
    #[serde(rename = "ComparisonMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_method: Option<TopicIRComparisonMethod>,
    #[serde(rename = "DisplayFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_format: Option<String>,
    #[serde(rename = "DisplayFormatOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_format_options: Option<DisplayFormatOptions>,
    #[serde(rename = "Expression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "Function")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<AggFunction>,
    #[serde(rename = "MetricId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_id: Option<Identifier>,
    #[serde(rename = "NamedEntity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_entity: Option<NamedEntityRef>,
    #[serde(rename = "Operands")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operands: Option<Vec<Identifier>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicIRComparisonMethod {
    #[serde(rename = "Period")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "WindowSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AggFunction {
    #[serde(rename = "Aggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<String>,
    #[serde(rename = "AggregationFunctionParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_function_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Period")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    #[serde(rename = "PeriodField")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_field: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VisualOptions {
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicVisual {
    #[serde(rename = "Ir")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ir: Option<TopicIR>,
    #[serde(rename = "Role")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "SupportingVisuals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supporting_visuals: Option<Vec<TopicVisual>>,
    #[serde(rename = "VisualId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicTemplate {
    #[serde(rename = "Slots")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slots: Option<Vec<Slot>>,
    #[serde(rename = "TemplateType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Slot {
    #[serde(rename = "SlotId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_id: Option<String>,
    #[serde(rename = "VisualId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchCreateTopicReviewedAnswerResponse {
    #[serde(rename = "InvalidAnswers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_answers: Option<Vec<InvalidTopicReviewedAnswer>>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "SucceededAnswers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub succeeded_answers: Option<Vec<SucceededTopicReviewedAnswer>>,
    #[serde(rename = "TopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
    #[serde(rename = "TopicId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InvalidTopicReviewedAnswer {
    #[serde(rename = "AnswerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer_id: Option<String>,
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SucceededTopicReviewedAnswer {
    #[serde(rename = "AnswerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteTopicReviewedAnswerRequest {
    #[serde(rename = "AnswerIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer_ids: Option<Vec<String>>,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "TopicId")]
    #[serde(default)]
    pub topic_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteTopicReviewedAnswerResponse {
    #[serde(rename = "InvalidAnswers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_answers: Option<Vec<InvalidTopicReviewedAnswer>>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "SucceededAnswers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub succeeded_answers: Option<Vec<SucceededTopicReviewedAnswer>>,
    #[serde(rename = "TopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
    #[serde(rename = "TopicId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelIngestionRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DataSetId")]
    #[serde(default)]
    pub data_set_id: String,
    #[serde(rename = "IngestionId")]
    #[serde(default)]
    pub ingestion_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelIngestionResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "IngestionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_id: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAccountCustomizationRequest {
    #[serde(rename = "AccountCustomization")]
    #[serde(default)]
    pub account_customization: AccountCustomization,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountCustomization {
    #[serde(rename = "DefaultEmailCustomizationTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_email_customization_template: Option<String>,
    #[serde(rename = "DefaultTheme")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_theme: Option<String>,
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
pub struct CreateAccountCustomizationResponse {
    #[serde(rename = "AccountCustomization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_customization: Option<AccountCustomization>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAccountSubscriptionRequest {
    #[serde(rename = "AccountName")]
    #[serde(default)]
    pub account_name: String,
    #[serde(rename = "ActiveDirectoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_directory_name: Option<String>,
    #[serde(rename = "AdminGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_group: Option<Vec<String>>,
    #[serde(rename = "AdminProGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_pro_group: Option<Vec<String>>,
    #[serde(rename = "AuthenticationMethod")]
    #[serde(default)]
    pub authentication_method: String,
    #[serde(rename = "AuthorGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_group: Option<Vec<String>>,
    #[serde(rename = "AuthorProGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_pro_group: Option<Vec<String>>,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "ContactNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_number: Option<String>,
    #[serde(rename = "DirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_id: Option<String>,
    #[serde(rename = "Edition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[serde(rename = "EmailAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(rename = "FirstName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "IAMIdentityCenterInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_a_m_identity_center_instance_arn: Option<String>,
    #[serde(rename = "LastName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "NotificationEmail")]
    #[serde(default)]
    pub notification_email: String,
    #[serde(rename = "ReaderGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reader_group: Option<Vec<String>>,
    #[serde(rename = "ReaderProGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reader_pro_group: Option<Vec<String>>,
    #[serde(rename = "Realm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realm: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAccountSubscriptionResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "SignupResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signup_response: Option<SignupResponse>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SignupResponse {
    #[serde(rename = "IAMUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_a_m_user: Option<bool>,
    #[serde(rename = "accountName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[serde(rename = "directoryType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_type: Option<String>,
    #[serde(rename = "userLoginName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_login_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateActionConnectorRequest {
    #[serde(rename = "ActionConnectorId")]
    #[serde(default)]
    pub action_connector_id: String,
    #[serde(rename = "AuthenticationConfig")]
    #[serde(default)]
    pub authentication_config: AuthConfig,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "VpcConnectionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_connection_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthConfig {
    #[serde(rename = "AuthenticationMetadata")]
    #[serde(default)]
    pub authentication_metadata: AuthenticationMetadata,
    #[serde(rename = "AuthenticationType")]
    #[serde(default)]
    pub authentication_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthenticationMetadata {
    #[serde(rename = "ApiKeyConnectionMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_connection_metadata: Option<APIKeyConnectionMetadata>,
    #[serde(rename = "AuthorizationCodeGrantMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code_grant_metadata: Option<AuthorizationCodeGrantMetadata>,
    #[serde(rename = "BasicAuthConnectionMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth_connection_metadata: Option<BasicAuthConnectionMetadata>,
    #[serde(rename = "ClientCredentialsGrantMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_credentials_grant_metadata: Option<ClientCredentialsGrantMetadata>,
    #[serde(rename = "IamConnectionMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_connection_metadata: Option<IAMConnectionMetadata>,
    #[serde(rename = "NoneConnectionMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub none_connection_metadata: Option<NoneConnectionMetadata>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct APIKeyConnectionMetadata {
    #[serde(rename = "ApiKey")]
    #[serde(default)]
    pub api_key: String,
    #[serde(rename = "BaseEndpoint")]
    #[serde(default)]
    pub base_endpoint: String,
    #[serde(rename = "Email")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthorizationCodeGrantMetadata {
    #[serde(rename = "AuthorizationCodeGrantCredentialsDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code_grant_credentials_details:
        Option<AuthorizationCodeGrantCredentialsDetails>,
    #[serde(rename = "AuthorizationCodeGrantCredentialsSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code_grant_credentials_source: Option<String>,
    #[serde(rename = "BaseEndpoint")]
    #[serde(default)]
    pub base_endpoint: String,
    #[serde(rename = "RedirectUrl")]
    #[serde(default)]
    pub redirect_url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthorizationCodeGrantCredentialsDetails {
    #[serde(rename = "AuthorizationCodeGrantDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code_grant_details: Option<AuthorizationCodeGrantDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthorizationCodeGrantDetails {
    #[serde(rename = "AuthorizationEndpoint")]
    #[serde(default)]
    pub authorization_endpoint: String,
    #[serde(rename = "ClientId")]
    #[serde(default)]
    pub client_id: String,
    #[serde(rename = "ClientSecret")]
    #[serde(default)]
    pub client_secret: String,
    #[serde(rename = "TokenEndpoint")]
    #[serde(default)]
    pub token_endpoint: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BasicAuthConnectionMetadata {
    #[serde(rename = "BaseEndpoint")]
    #[serde(default)]
    pub base_endpoint: String,
    #[serde(rename = "Password")]
    #[serde(default)]
    pub password: String,
    #[serde(rename = "Username")]
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClientCredentialsGrantMetadata {
    #[serde(rename = "BaseEndpoint")]
    #[serde(default)]
    pub base_endpoint: String,
    #[serde(rename = "ClientCredentialsDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_credentials_details: Option<ClientCredentialsDetails>,
    #[serde(rename = "ClientCredentialsSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_credentials_source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClientCredentialsDetails {
    #[serde(rename = "ClientCredentialsGrantDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_credentials_grant_details: Option<ClientCredentialsGrantDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClientCredentialsGrantDetails {
    #[serde(rename = "ClientId")]
    #[serde(default)]
    pub client_id: String,
    #[serde(rename = "ClientSecret")]
    #[serde(default)]
    pub client_secret: String,
    #[serde(rename = "TokenEndpoint")]
    #[serde(default)]
    pub token_endpoint: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IAMConnectionMetadata {
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NoneConnectionMetadata {
    #[serde(rename = "BaseEndpoint")]
    #[serde(default)]
    pub base_endpoint: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourcePermission {
    #[serde(rename = "Actions")]
    #[serde(default)]
    pub actions: Vec<String>,
    #[serde(rename = "Principal")]
    #[serde(default)]
    pub principal: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateActionConnectorResponse {
    #[serde(rename = "ActionConnectorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_connector_id: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_status: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAnalysisRequest {
    #[serde(rename = "AnalysisId")]
    #[serde(default)]
    pub analysis_id: String,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Definition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<AnalysisDefinition>,
    #[serde(rename = "FolderArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_arns: Option<Vec<String>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Parameters>,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "SourceEntity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_entity: Option<AnalysisSourceEntity>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "ThemeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_arn: Option<String>,
    #[serde(rename = "ValidationStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_strategy: Option<ValidationStrategy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalysisDefinition {
    #[serde(rename = "AnalysisDefaults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_defaults: Option<AnalysisDefaults>,
    #[serde(rename = "CalculatedFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculated_fields: Option<Vec<CalculatedField>>,
    #[serde(rename = "ColumnConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_configurations: Option<Vec<ColumnConfiguration>>,
    #[serde(rename = "DataSetIdentifierDeclarations")]
    #[serde(default)]
    pub data_set_identifier_declarations: Vec<DataSetIdentifierDeclaration>,
    #[serde(rename = "FilterGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_groups: Option<Vec<FilterGroup>>,
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<AssetOptions>,
    #[serde(rename = "ParameterDeclarations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_declarations: Option<Vec<ParameterDeclaration>>,
    #[serde(rename = "QueryExecutionOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_execution_options: Option<QueryExecutionOptions>,
    #[serde(rename = "Sheets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheets: Option<Vec<SheetDefinition>>,
    #[serde(rename = "StaticFiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_files: Option<Vec<StaticFile>>,
    #[serde(rename = "TooltipSheets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip_sheets: Option<Vec<TooltipSheetDefinition>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalysisDefaults {
    #[serde(rename = "DefaultNewSheetConfiguration")]
    #[serde(default)]
    pub default_new_sheet_configuration: DefaultNewSheetConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DefaultNewSheetConfiguration {
    #[serde(rename = "InteractiveLayoutConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactive_layout_configuration: Option<DefaultInteractiveLayoutConfiguration>,
    #[serde(rename = "PaginatedLayoutConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paginated_layout_configuration: Option<DefaultPaginatedLayoutConfiguration>,
    #[serde(rename = "SheetContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_content_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DefaultInteractiveLayoutConfiguration {
    #[serde(rename = "FreeForm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_form: Option<DefaultFreeFormLayoutConfiguration>,
    #[serde(rename = "Grid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid: Option<DefaultGridLayoutConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DefaultFreeFormLayoutConfiguration {
    #[serde(rename = "CanvasSizeOptions")]
    #[serde(default)]
    pub canvas_size_options: FreeFormLayoutCanvasSizeOptions,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FreeFormLayoutCanvasSizeOptions {
    #[serde(rename = "ScreenCanvasSizeOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_canvas_size_options: Option<FreeFormLayoutScreenCanvasSizeOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FreeFormLayoutScreenCanvasSizeOptions {
    #[serde(rename = "OptimizedViewPortWidth")]
    #[serde(default)]
    pub optimized_view_port_width: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DefaultGridLayoutConfiguration {
    #[serde(rename = "CanvasSizeOptions")]
    #[serde(default)]
    pub canvas_size_options: GridLayoutCanvasSizeOptions,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GridLayoutCanvasSizeOptions {
    #[serde(rename = "ScreenCanvasSizeOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_canvas_size_options: Option<GridLayoutScreenCanvasSizeOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GridLayoutScreenCanvasSizeOptions {
    #[serde(rename = "OptimizedViewPortWidth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimized_view_port_width: Option<String>,
    #[serde(rename = "ResizeOption")]
    #[serde(default)]
    pub resize_option: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DefaultPaginatedLayoutConfiguration {
    #[serde(rename = "SectionBased")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_based: Option<DefaultSectionBasedLayoutConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DefaultSectionBasedLayoutConfiguration {
    #[serde(rename = "CanvasSizeOptions")]
    #[serde(default)]
    pub canvas_size_options: SectionBasedLayoutCanvasSizeOptions,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SectionBasedLayoutCanvasSizeOptions {
    #[serde(rename = "PaperCanvasSizeOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paper_canvas_size_options: Option<SectionBasedLayoutPaperCanvasSizeOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SectionBasedLayoutPaperCanvasSizeOptions {
    #[serde(rename = "PaperMargin")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paper_margin: Option<Spacing>,
    #[serde(rename = "PaperOrientation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paper_orientation: Option<String>,
    #[serde(rename = "PaperSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paper_size: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Spacing {
    #[serde(rename = "Bottom")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom: Option<String>,
    #[serde(rename = "Left")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<String>,
    #[serde(rename = "Right")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right: Option<String>,
    #[serde(rename = "Top")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CalculatedField {
    #[serde(rename = "DataSetIdentifier")]
    #[serde(default)]
    pub data_set_identifier: String,
    #[serde(rename = "Expression")]
    #[serde(default)]
    pub expression: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ColumnConfiguration {
    #[serde(rename = "ColorsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colors_configuration: Option<ColorsConfiguration>,
    #[serde(rename = "Column")]
    #[serde(default)]
    pub column: ColumnIdentifier,
    #[serde(rename = "DecalSettingsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decal_settings_configuration: Option<DecalSettingsConfiguration>,
    #[serde(rename = "FormatConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_configuration: Option<FormatConfiguration>,
    #[serde(rename = "Role")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ColorsConfiguration {
    #[serde(rename = "CustomColors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_colors: Option<Vec<CustomColor>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomColor {
    #[serde(rename = "Color")]
    #[serde(default)]
    pub color: String,
    #[serde(rename = "FieldValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_value: Option<String>,
    #[serde(rename = "SpecialValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ColumnIdentifier {
    #[serde(rename = "ColumnName")]
    #[serde(default)]
    pub column_name: String,
    #[serde(rename = "DataSetIdentifier")]
    #[serde(default)]
    pub data_set_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DecalSettingsConfiguration {
    #[serde(rename = "CustomDecalSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_decal_settings: Option<Vec<DecalSettings>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DecalSettings {
    #[serde(rename = "DecalColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decal_color: Option<String>,
    #[serde(rename = "DecalPatternType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decal_pattern_type: Option<String>,
    #[serde(rename = "DecalStyleType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decal_style_type: Option<String>,
    #[serde(rename = "DecalVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decal_visibility: Option<String>,
    #[serde(rename = "ElementValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FormatConfiguration {
    #[serde(rename = "DateTimeFormatConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time_format_configuration: Option<DateTimeFormatConfiguration>,
    #[serde(rename = "NumberFormatConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_format_configuration: Option<NumberFormatConfiguration>,
    #[serde(rename = "StringFormatConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_format_configuration: Option<StringFormatConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DateTimeFormatConfiguration {
    #[serde(rename = "DateTimeFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time_format: Option<String>,
    #[serde(rename = "NullValueFormatConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub null_value_format_configuration: Option<NullValueFormatConfiguration>,
    #[serde(rename = "NumericFormatConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric_format_configuration: Option<NumericFormatConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NullValueFormatConfiguration {
    #[serde(rename = "NullString")]
    #[serde(default)]
    pub null_string: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NumericFormatConfiguration {
    #[serde(rename = "CurrencyDisplayFormatConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_display_format_configuration: Option<CurrencyDisplayFormatConfiguration>,
    #[serde(rename = "NumberDisplayFormatConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_display_format_configuration: Option<NumberDisplayFormatConfiguration>,
    #[serde(rename = "PercentageDisplayFormatConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage_display_format_configuration: Option<PercentageDisplayFormatConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CurrencyDisplayFormatConfiguration {
    #[serde(rename = "DecimalPlacesConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal_places_configuration: Option<DecimalPlacesConfiguration>,
    #[serde(rename = "NegativeValueConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_value_configuration: Option<NegativeValueConfiguration>,
    #[serde(rename = "NullValueFormatConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub null_value_format_configuration: Option<NullValueFormatConfiguration>,
    #[serde(rename = "NumberScale")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_scale: Option<String>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "SeparatorConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator_configuration: Option<NumericSeparatorConfiguration>,
    #[serde(rename = "Suffix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(rename = "Symbol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DecimalPlacesConfiguration {
    #[serde(rename = "DecimalPlaces")]
    #[serde(default)]
    pub decimal_places: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NegativeValueConfiguration {
    #[serde(rename = "DisplayMode")]
    #[serde(default)]
    pub display_mode: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NumericSeparatorConfiguration {
    #[serde(rename = "DecimalSeparator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal_separator: Option<String>,
    #[serde(rename = "ThousandsSeparator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thousands_separator: Option<ThousandSeparatorOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThousandSeparatorOptions {
    #[serde(rename = "GroupingStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouping_style: Option<String>,
    #[serde(rename = "Symbol")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NumberDisplayFormatConfiguration {
    #[serde(rename = "DecimalPlacesConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal_places_configuration: Option<DecimalPlacesConfiguration>,
    #[serde(rename = "NegativeValueConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_value_configuration: Option<NegativeValueConfiguration>,
    #[serde(rename = "NullValueFormatConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub null_value_format_configuration: Option<NullValueFormatConfiguration>,
    #[serde(rename = "NumberScale")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_scale: Option<String>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "SeparatorConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator_configuration: Option<NumericSeparatorConfiguration>,
    #[serde(rename = "Suffix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PercentageDisplayFormatConfiguration {
    #[serde(rename = "DecimalPlacesConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal_places_configuration: Option<DecimalPlacesConfiguration>,
    #[serde(rename = "NegativeValueConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_value_configuration: Option<NegativeValueConfiguration>,
    #[serde(rename = "NullValueFormatConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub null_value_format_configuration: Option<NullValueFormatConfiguration>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "SeparatorConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator_configuration: Option<NumericSeparatorConfiguration>,
    #[serde(rename = "Suffix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NumberFormatConfiguration {
    #[serde(rename = "FormatConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_configuration: Option<NumericFormatConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StringFormatConfiguration {
    #[serde(rename = "NullValueFormatConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub null_value_format_configuration: Option<NullValueFormatConfiguration>,
    #[serde(rename = "NumericFormatConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric_format_configuration: Option<NumericFormatConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSetIdentifierDeclaration {
    #[serde(rename = "DataSetArn")]
    #[serde(default)]
    pub data_set_arn: String,
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterGroup {
    #[serde(rename = "CrossDataset")]
    #[serde(default)]
    pub cross_dataset: String,
    #[serde(rename = "FilterGroupId")]
    #[serde(default)]
    pub filter_group_id: String,
    #[serde(rename = "Filters")]
    #[serde(default)]
    pub filters: Vec<Filter>,
    #[serde(rename = "ScopeConfiguration")]
    #[serde(default)]
    pub scope_configuration: FilterScopeConfiguration,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Filter {
    #[serde(rename = "CategoryFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_filter: Option<CategoryFilter>,
    #[serde(rename = "NestedFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nested_filter: Option<NestedFilter>,
    #[serde(rename = "NumericEqualityFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric_equality_filter: Option<NumericEqualityFilter>,
    #[serde(rename = "NumericRangeFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric_range_filter: Option<NumericRangeFilter>,
    #[serde(rename = "RelativeDatesFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_dates_filter: Option<RelativeDatesFilter>,
    #[serde(rename = "TimeEqualityFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_equality_filter: Option<TimeEqualityFilter>,
    #[serde(rename = "TimeRangeFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_range_filter: Option<TimeRangeFilter>,
    #[serde(rename = "TopBottomFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_bottom_filter: Option<TopBottomFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CategoryFilter {
    #[serde(rename = "Column")]
    #[serde(default)]
    pub column: ColumnIdentifier,
    #[serde(rename = "Configuration")]
    #[serde(default)]
    pub configuration: CategoryFilterConfiguration,
    #[serde(rename = "DefaultFilterControlConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_filter_control_configuration: Option<DefaultFilterControlConfiguration>,
    #[serde(rename = "FilterId")]
    #[serde(default)]
    pub filter_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CategoryFilterConfiguration {
    #[serde(rename = "CustomFilterConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_filter_configuration: Option<CustomFilterConfiguration>,
    #[serde(rename = "CustomFilterListConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_filter_list_configuration: Option<CustomFilterListConfiguration>,
    #[serde(rename = "FilterListConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_list_configuration: Option<FilterListConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomFilterConfiguration {
    #[serde(rename = "CategoryValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_value: Option<String>,
    #[serde(rename = "MatchOperator")]
    #[serde(default)]
    pub match_operator: String,
    #[serde(rename = "NullOption")]
    #[serde(default)]
    pub null_option: String,
    #[serde(rename = "ParameterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_name: Option<String>,
    #[serde(rename = "SelectAllOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_all_options: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomFilterListConfiguration {
    #[serde(rename = "CategoryValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_values: Option<Vec<String>>,
    #[serde(rename = "MatchOperator")]
    #[serde(default)]
    pub match_operator: String,
    #[serde(rename = "NullOption")]
    #[serde(default)]
    pub null_option: String,
    #[serde(rename = "SelectAllOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_all_options: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterListConfiguration {
    #[serde(rename = "CategoryValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_values: Option<Vec<String>>,
    #[serde(rename = "MatchOperator")]
    #[serde(default)]
    pub match_operator: String,
    #[serde(rename = "NullOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub null_option: Option<String>,
    #[serde(rename = "SelectAllOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_all_options: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DefaultFilterControlConfiguration {
    #[serde(rename = "ControlOptions")]
    #[serde(default)]
    pub control_options: DefaultFilterControlOptions,
    #[serde(rename = "ControlTitleFormatText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_title_format_text: Option<ControlTitleFormatText>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DefaultFilterControlOptions {
    #[serde(rename = "DefaultDateTimePickerOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_date_time_picker_options: Option<DefaultDateTimePickerControlOptions>,
    #[serde(rename = "DefaultDropdownOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_dropdown_options: Option<DefaultFilterDropDownControlOptions>,
    #[serde(rename = "DefaultListOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_list_options: Option<DefaultFilterListControlOptions>,
    #[serde(rename = "DefaultRelativeDateTimeOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_relative_date_time_options: Option<DefaultRelativeDateTimeControlOptions>,
    #[serde(rename = "DefaultSliderOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_slider_options: Option<DefaultSliderControlOptions>,
    #[serde(rename = "DefaultTextAreaOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_text_area_options: Option<DefaultTextAreaControlOptions>,
    #[serde(rename = "DefaultTextFieldOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_text_field_options: Option<DefaultTextFieldControlOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DefaultDateTimePickerControlOptions {
    #[serde(rename = "CommitMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_mode: Option<String>,
    #[serde(rename = "DisplayOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_options: Option<DateTimePickerControlDisplayOptions>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DateTimePickerControlDisplayOptions {
    #[serde(rename = "DateIconVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_icon_visibility: Option<String>,
    #[serde(rename = "DateTimeFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time_format: Option<String>,
    #[serde(rename = "HelperTextVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub helper_text_visibility: Option<String>,
    #[serde(rename = "InfoIconLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info_icon_label_options: Option<SheetControlInfoIconLabelOptions>,
    #[serde(rename = "TitleOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_options: Option<LabelOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SheetControlInfoIconLabelOptions {
    #[serde(rename = "InfoIconText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info_icon_text: Option<String>,
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LabelOptions {
    #[serde(rename = "CustomLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_label: Option<String>,
    #[serde(rename = "FontConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_configuration: Option<FontConfiguration>,
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FontConfiguration {
    #[serde(rename = "FontColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_color: Option<String>,
    #[serde(rename = "FontDecoration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_decoration: Option<String>,
    #[serde(rename = "FontFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,
    #[serde(rename = "FontSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<FontSize>,
    #[serde(rename = "FontStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_style: Option<String>,
    #[serde(rename = "FontWeight")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<FontWeight>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FontSize {
    #[serde(rename = "Absolute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute: Option<String>,
    #[serde(rename = "Relative")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FontWeight {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DefaultFilterDropDownControlOptions {
    #[serde(rename = "CommitMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_mode: Option<String>,
    #[serde(rename = "ControlSortConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_sort_configurations: Option<Vec<ControlSortConfiguration>>,
    #[serde(rename = "DisplayOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_options: Option<DropDownControlDisplayOptions>,
    #[serde(rename = "SelectableValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectable_values: Option<FilterSelectableValues>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ControlSortConfiguration {
    #[serde(rename = "ControlColumnSort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_column_sort: Option<AggregationSortConfiguration>,
    #[serde(rename = "SelectableValuesSort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectable_values_sort: Option<SelectableValuesSort>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AggregationSortConfiguration {
    #[serde(rename = "AggregationFunction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_function: Option<AggregationFunction>,
    #[serde(rename = "Column")]
    #[serde(default)]
    pub column: ColumnIdentifier,
    #[serde(rename = "SortDirection")]
    #[serde(default)]
    pub sort_direction: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AggregationFunction {
    #[serde(rename = "AttributeAggregationFunction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_aggregation_function: Option<AttributeAggregationFunction>,
    #[serde(rename = "CategoricalAggregationFunction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categorical_aggregation_function: Option<String>,
    #[serde(rename = "DateAggregationFunction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_aggregation_function: Option<String>,
    #[serde(rename = "NumericalAggregationFunction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numerical_aggregation_function: Option<NumericalAggregationFunction>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttributeAggregationFunction {
    #[serde(rename = "SimpleAttributeAggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_attribute_aggregation: Option<String>,
    #[serde(rename = "ValueForMultipleValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_for_multiple_values: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NumericalAggregationFunction {
    #[serde(rename = "PercentileAggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentile_aggregation: Option<PercentileAggregation>,
    #[serde(rename = "SimpleNumericalAggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_numerical_aggregation: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PercentileAggregation {
    #[serde(rename = "PercentileValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentile_value: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SelectableValuesSort {
    #[serde(rename = "Direction")]
    #[serde(default)]
    pub direction: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DropDownControlDisplayOptions {
    #[serde(rename = "InfoIconLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info_icon_label_options: Option<SheetControlInfoIconLabelOptions>,
    #[serde(rename = "SelectAllOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_all_options: Option<ListControlSelectAllOptions>,
    #[serde(rename = "TitleOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_options: Option<LabelOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListControlSelectAllOptions {
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterSelectableValues {
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DefaultFilterListControlOptions {
    #[serde(rename = "ControlSortConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_sort_configurations: Option<Vec<ControlSortConfiguration>>,
    #[serde(rename = "DisplayOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_options: Option<ListControlDisplayOptions>,
    #[serde(rename = "SelectableValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectable_values: Option<FilterSelectableValues>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListControlDisplayOptions {
    #[serde(rename = "InfoIconLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info_icon_label_options: Option<SheetControlInfoIconLabelOptions>,
    #[serde(rename = "SearchOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_options: Option<ListControlSearchOptions>,
    #[serde(rename = "SelectAllOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_all_options: Option<ListControlSelectAllOptions>,
    #[serde(rename = "TitleOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_options: Option<LabelOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListControlSearchOptions {
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DefaultRelativeDateTimeControlOptions {
    #[serde(rename = "CommitMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_mode: Option<String>,
    #[serde(rename = "DisplayOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_options: Option<RelativeDateTimeControlDisplayOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RelativeDateTimeControlDisplayOptions {
    #[serde(rename = "DateTimeFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time_format: Option<String>,
    #[serde(rename = "InfoIconLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info_icon_label_options: Option<SheetControlInfoIconLabelOptions>,
    #[serde(rename = "TitleOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_options: Option<LabelOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DefaultSliderControlOptions {
    #[serde(rename = "DisplayOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_options: Option<SliderControlDisplayOptions>,
    #[serde(rename = "MaximumValue")]
    #[serde(default)]
    pub maximum_value: f64,
    #[serde(rename = "MinimumValue")]
    #[serde(default)]
    pub minimum_value: f64,
    #[serde(rename = "StepSize")]
    #[serde(default)]
    pub step_size: f64,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SliderControlDisplayOptions {
    #[serde(rename = "InfoIconLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info_icon_label_options: Option<SheetControlInfoIconLabelOptions>,
    #[serde(rename = "TitleOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_options: Option<LabelOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DefaultTextAreaControlOptions {
    #[serde(rename = "Delimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    #[serde(rename = "DisplayOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_options: Option<TextAreaControlDisplayOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TextAreaControlDisplayOptions {
    #[serde(rename = "InfoIconLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info_icon_label_options: Option<SheetControlInfoIconLabelOptions>,
    #[serde(rename = "PlaceholderOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder_options: Option<TextControlPlaceholderOptions>,
    #[serde(rename = "TitleOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_options: Option<LabelOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TextControlPlaceholderOptions {
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DefaultTextFieldControlOptions {
    #[serde(rename = "DisplayOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_options: Option<TextFieldControlDisplayOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TextFieldControlDisplayOptions {
    #[serde(rename = "InfoIconLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info_icon_label_options: Option<SheetControlInfoIconLabelOptions>,
    #[serde(rename = "PlaceholderOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder_options: Option<TextControlPlaceholderOptions>,
    #[serde(rename = "TitleOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_options: Option<LabelOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ControlTitleFormatText {
    #[serde(rename = "PlainText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plain_text: Option<String>,
    #[serde(rename = "RichText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rich_text: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NestedFilter {
    #[serde(rename = "Column")]
    #[serde(default)]
    pub column: ColumnIdentifier,
    #[serde(rename = "FilterId")]
    #[serde(default)]
    pub filter_id: String,
    #[serde(rename = "IncludeInnerSet")]
    #[serde(default)]
    pub include_inner_set: bool,
    #[serde(rename = "InnerFilter")]
    #[serde(default)]
    pub inner_filter: InnerFilter,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InnerFilter {
    #[serde(rename = "CategoryInnerFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_inner_filter: Option<CategoryInnerFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CategoryInnerFilter {
    #[serde(rename = "Column")]
    #[serde(default)]
    pub column: ColumnIdentifier,
    #[serde(rename = "Configuration")]
    #[serde(default)]
    pub configuration: CategoryFilterConfiguration,
    #[serde(rename = "DefaultFilterControlConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_filter_control_configuration: Option<DefaultFilterControlConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NumericEqualityFilter {
    #[serde(rename = "AggregationFunction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_function: Option<AggregationFunction>,
    #[serde(rename = "Column")]
    #[serde(default)]
    pub column: ColumnIdentifier,
    #[serde(rename = "DefaultFilterControlConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_filter_control_configuration: Option<DefaultFilterControlConfiguration>,
    #[serde(rename = "FilterId")]
    #[serde(default)]
    pub filter_id: String,
    #[serde(rename = "MatchOperator")]
    #[serde(default)]
    pub match_operator: String,
    #[serde(rename = "NullOption")]
    #[serde(default)]
    pub null_option: String,
    #[serde(rename = "ParameterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_name: Option<String>,
    #[serde(rename = "SelectAllOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_all_options: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NumericRangeFilter {
    #[serde(rename = "AggregationFunction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_function: Option<AggregationFunction>,
    #[serde(rename = "Column")]
    #[serde(default)]
    pub column: ColumnIdentifier,
    #[serde(rename = "DefaultFilterControlConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_filter_control_configuration: Option<DefaultFilterControlConfiguration>,
    #[serde(rename = "FilterId")]
    #[serde(default)]
    pub filter_id: String,
    #[serde(rename = "IncludeMaximum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_maximum: Option<bool>,
    #[serde(rename = "IncludeMinimum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_minimum: Option<bool>,
    #[serde(rename = "NullOption")]
    #[serde(default)]
    pub null_option: String,
    #[serde(rename = "RangeMaximum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_maximum: Option<NumericRangeFilterValue>,
    #[serde(rename = "RangeMinimum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_minimum: Option<NumericRangeFilterValue>,
    #[serde(rename = "SelectAllOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_all_options: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NumericRangeFilterValue {
    #[serde(rename = "Parameter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter: Option<String>,
    #[serde(rename = "StaticValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_value: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RelativeDatesFilter {
    #[serde(rename = "AnchorDateConfiguration")]
    #[serde(default)]
    pub anchor_date_configuration: AnchorDateConfiguration,
    #[serde(rename = "Column")]
    #[serde(default)]
    pub column: ColumnIdentifier,
    #[serde(rename = "DefaultFilterControlConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_filter_control_configuration: Option<DefaultFilterControlConfiguration>,
    #[serde(rename = "ExcludePeriodConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_period_configuration: Option<ExcludePeriodConfiguration>,
    #[serde(rename = "FilterId")]
    #[serde(default)]
    pub filter_id: String,
    #[serde(rename = "MinimumGranularity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_granularity: Option<String>,
    #[serde(rename = "NullOption")]
    #[serde(default)]
    pub null_option: String,
    #[serde(rename = "ParameterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_name: Option<String>,
    #[serde(rename = "RelativeDateType")]
    #[serde(default)]
    pub relative_date_type: String,
    #[serde(rename = "RelativeDateValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_date_value: Option<i32>,
    #[serde(rename = "TimeGranularity")]
    #[serde(default)]
    pub time_granularity: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnchorDateConfiguration {
    #[serde(rename = "AnchorOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor_option: Option<String>,
    #[serde(rename = "ParameterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExcludePeriodConfiguration {
    #[serde(rename = "Amount")]
    #[serde(default)]
    pub amount: i32,
    #[serde(rename = "Granularity")]
    #[serde(default)]
    pub granularity: String,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimeEqualityFilter {
    #[serde(rename = "Column")]
    #[serde(default)]
    pub column: ColumnIdentifier,
    #[serde(rename = "DefaultFilterControlConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_filter_control_configuration: Option<DefaultFilterControlConfiguration>,
    #[serde(rename = "FilterId")]
    #[serde(default)]
    pub filter_id: String,
    #[serde(rename = "ParameterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_name: Option<String>,
    #[serde(rename = "RollingDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rolling_date: Option<RollingDateConfiguration>,
    #[serde(rename = "TimeGranularity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_granularity: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RollingDateConfiguration {
    #[serde(rename = "DataSetIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_identifier: Option<String>,
    #[serde(rename = "Expression")]
    #[serde(default)]
    pub expression: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimeRangeFilter {
    #[serde(rename = "Column")]
    #[serde(default)]
    pub column: ColumnIdentifier,
    #[serde(rename = "DefaultFilterControlConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_filter_control_configuration: Option<DefaultFilterControlConfiguration>,
    #[serde(rename = "ExcludePeriodConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_period_configuration: Option<ExcludePeriodConfiguration>,
    #[serde(rename = "FilterId")]
    #[serde(default)]
    pub filter_id: String,
    #[serde(rename = "IncludeMaximum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_maximum: Option<bool>,
    #[serde(rename = "IncludeMinimum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_minimum: Option<bool>,
    #[serde(rename = "NullOption")]
    #[serde(default)]
    pub null_option: String,
    #[serde(rename = "RangeMaximumValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_maximum_value: Option<TimeRangeFilterValue>,
    #[serde(rename = "RangeMinimumValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_minimum_value: Option<TimeRangeFilterValue>,
    #[serde(rename = "TimeGranularity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_granularity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimeRangeFilterValue {
    #[serde(rename = "Parameter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter: Option<String>,
    #[serde(rename = "RollingDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rolling_date: Option<RollingDateConfiguration>,
    #[serde(rename = "StaticValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_value: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopBottomFilter {
    #[serde(rename = "AggregationSortConfigurations")]
    #[serde(default)]
    pub aggregation_sort_configurations: Vec<AggregationSortConfiguration>,
    #[serde(rename = "Column")]
    #[serde(default)]
    pub column: ColumnIdentifier,
    #[serde(rename = "DefaultFilterControlConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_filter_control_configuration: Option<DefaultFilterControlConfiguration>,
    #[serde(rename = "FilterId")]
    #[serde(default)]
    pub filter_id: String,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "ParameterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_name: Option<String>,
    #[serde(rename = "TimeGranularity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_granularity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterScopeConfiguration {
    #[serde(rename = "AllSheets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_sheets: Option<AllSheetsFilterScopeConfiguration>,
    #[serde(rename = "SelectedSheets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_sheets: Option<SelectedSheetsFilterScopeConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AllSheetsFilterScopeConfiguration {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SelectedSheetsFilterScopeConfiguration {
    #[serde(rename = "SheetVisualScopingConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_visual_scoping_configurations: Option<Vec<SheetVisualScopingConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SheetVisualScopingConfiguration {
    #[serde(rename = "Scope")]
    #[serde(default)]
    pub scope: String,
    #[serde(rename = "SheetId")]
    #[serde(default)]
    pub sheet_id: String,
    #[serde(rename = "VisualIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetOptions {
    #[serde(rename = "CustomActionDefaults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_action_defaults: Option<VisualCustomActionDefaults>,
    #[serde(rename = "ExcludedDataSetArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_data_set_arns: Option<Vec<String>>,
    #[serde(rename = "QBusinessInsightsStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q_business_insights_status: Option<String>,
    #[serde(rename = "Timezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(rename = "WeekStart")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub week_start: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VisualCustomActionDefaults {
    #[serde(rename = "highlightOperation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlight_operation: Option<VisualHighlightOperation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VisualHighlightOperation {
    #[serde(rename = "Trigger")]
    #[serde(default)]
    pub trigger: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParameterDeclaration {
    #[serde(rename = "DateTimeParameterDeclaration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time_parameter_declaration: Option<DateTimeParameterDeclaration>,
    #[serde(rename = "DecimalParameterDeclaration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal_parameter_declaration: Option<DecimalParameterDeclaration>,
    #[serde(rename = "IntegerParameterDeclaration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integer_parameter_declaration: Option<IntegerParameterDeclaration>,
    #[serde(rename = "StringParameterDeclaration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_parameter_declaration: Option<StringParameterDeclaration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DateTimeParameterDeclaration {
    #[serde(rename = "DefaultValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_values: Option<DateTimeDefaultValues>,
    #[serde(rename = "MappedDataSetParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapped_data_set_parameters: Option<Vec<MappedDataSetParameter>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "TimeGranularity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_granularity: Option<String>,
    #[serde(rename = "ValueWhenUnset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_when_unset: Option<DateTimeValueWhenUnsetConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DateTimeDefaultValues {
    #[serde(rename = "DynamicValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_value: Option<DynamicDefaultValue>,
    #[serde(rename = "RollingDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rolling_date: Option<RollingDateConfiguration>,
    #[serde(rename = "StaticValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_values: Option<Vec<f64>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DynamicDefaultValue {
    #[serde(rename = "DefaultValueColumn")]
    #[serde(default)]
    pub default_value_column: ColumnIdentifier,
    #[serde(rename = "GroupNameColumn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name_column: Option<ColumnIdentifier>,
    #[serde(rename = "UserNameColumn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name_column: Option<ColumnIdentifier>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MappedDataSetParameter {
    #[serde(rename = "DataSetIdentifier")]
    #[serde(default)]
    pub data_set_identifier: String,
    #[serde(rename = "DataSetParameterName")]
    #[serde(default)]
    pub data_set_parameter_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DateTimeValueWhenUnsetConfiguration {
    #[serde(rename = "CustomValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_value: Option<f64>,
    #[serde(rename = "ValueWhenUnsetOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_when_unset_option: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DecimalParameterDeclaration {
    #[serde(rename = "DefaultValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_values: Option<DecimalDefaultValues>,
    #[serde(rename = "MappedDataSetParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapped_data_set_parameters: Option<Vec<MappedDataSetParameter>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ParameterValueType")]
    #[serde(default)]
    pub parameter_value_type: String,
    #[serde(rename = "ValueWhenUnset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_when_unset: Option<DecimalValueWhenUnsetConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DecimalDefaultValues {
    #[serde(rename = "DynamicValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_value: Option<DynamicDefaultValue>,
    #[serde(rename = "StaticValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_values: Option<Vec<f64>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DecimalValueWhenUnsetConfiguration {
    #[serde(rename = "CustomValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_value: Option<f64>,
    #[serde(rename = "ValueWhenUnsetOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_when_unset_option: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntegerParameterDeclaration {
    #[serde(rename = "DefaultValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_values: Option<IntegerDefaultValues>,
    #[serde(rename = "MappedDataSetParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapped_data_set_parameters: Option<Vec<MappedDataSetParameter>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ParameterValueType")]
    #[serde(default)]
    pub parameter_value_type: String,
    #[serde(rename = "ValueWhenUnset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_when_unset: Option<IntegerValueWhenUnsetConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntegerDefaultValues {
    #[serde(rename = "DynamicValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_value: Option<DynamicDefaultValue>,
    #[serde(rename = "StaticValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_values: Option<Vec<i64>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntegerValueWhenUnsetConfiguration {
    #[serde(rename = "CustomValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_value: Option<i64>,
    #[serde(rename = "ValueWhenUnsetOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_when_unset_option: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StringParameterDeclaration {
    #[serde(rename = "DefaultValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_values: Option<StringDefaultValues>,
    #[serde(rename = "MappedDataSetParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapped_data_set_parameters: Option<Vec<MappedDataSetParameter>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ParameterValueType")]
    #[serde(default)]
    pub parameter_value_type: String,
    #[serde(rename = "ValueWhenUnset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_when_unset: Option<StringValueWhenUnsetConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StringDefaultValues {
    #[serde(rename = "DynamicValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_value: Option<DynamicDefaultValue>,
    #[serde(rename = "StaticValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StringValueWhenUnsetConfiguration {
    #[serde(rename = "CustomValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_value: Option<String>,
    #[serde(rename = "ValueWhenUnsetOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_when_unset_option: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryExecutionOptions {
    #[serde(rename = "QueryExecutionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_execution_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SheetDefinition {
    #[serde(rename = "ContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "CustomActionDefaults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_action_defaults: Option<VisualCustomActionDefaults>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "FilterControls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_controls: Option<Vec<FilterControl>>,
    #[serde(rename = "Images")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<SheetImage>>,
    #[serde(rename = "Layouts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layouts: Option<Vec<Layout>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ParameterControls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_controls: Option<Vec<ParameterControl>>,
    #[serde(rename = "SheetControlLayouts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_control_layouts: Option<Vec<SheetControlLayout>>,
    #[serde(rename = "SheetId")]
    #[serde(default)]
    pub sheet_id: String,
    #[serde(rename = "TextBoxes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_boxes: Option<Vec<SheetTextBox>>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "Visuals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visuals: Option<Vec<Visual>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterControl {
    #[serde(rename = "CrossSheet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_sheet: Option<FilterCrossSheetControl>,
    #[serde(rename = "DateTimePicker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time_picker: Option<FilterDateTimePickerControl>,
    #[serde(rename = "Dropdown")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropdown: Option<FilterDropDownControl>,
    #[serde(rename = "List")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<FilterListControl>,
    #[serde(rename = "RelativeDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_date_time: Option<FilterRelativeDateTimeControl>,
    #[serde(rename = "Slider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slider: Option<FilterSliderControl>,
    #[serde(rename = "TextArea")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_area: Option<FilterTextAreaControl>,
    #[serde(rename = "TextField")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_field: Option<FilterTextFieldControl>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterCrossSheetControl {
    #[serde(rename = "CascadingControlConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cascading_control_configuration: Option<CascadingControlConfiguration>,
    #[serde(rename = "FilterControlId")]
    #[serde(default)]
    pub filter_control_id: String,
    #[serde(rename = "SourceFilterId")]
    #[serde(default)]
    pub source_filter_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CascadingControlConfiguration {
    #[serde(rename = "SourceControls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_controls: Option<Vec<CascadingControlSource>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CascadingControlSource {
    #[serde(rename = "ColumnToMatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_to_match: Option<ColumnIdentifier>,
    #[serde(rename = "SourceSheetControlId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_sheet_control_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterDateTimePickerControl {
    #[serde(rename = "CommitMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_mode: Option<String>,
    #[serde(rename = "ControlTitleFormatText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_title_format_text: Option<ControlTitleFormatText>,
    #[serde(rename = "DisplayOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_options: Option<DateTimePickerControlDisplayOptions>,
    #[serde(rename = "FilterControlId")]
    #[serde(default)]
    pub filter_control_id: String,
    #[serde(rename = "SourceFilterId")]
    #[serde(default)]
    pub source_filter_id: String,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterDropDownControl {
    #[serde(rename = "CascadingControlConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cascading_control_configuration: Option<CascadingControlConfiguration>,
    #[serde(rename = "CommitMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_mode: Option<String>,
    #[serde(rename = "ControlSortConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_sort_configurations: Option<Vec<ControlSortConfiguration>>,
    #[serde(rename = "ControlTitleFormatText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_title_format_text: Option<ControlTitleFormatText>,
    #[serde(rename = "DisplayOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_options: Option<DropDownControlDisplayOptions>,
    #[serde(rename = "FilterControlId")]
    #[serde(default)]
    pub filter_control_id: String,
    #[serde(rename = "SelectableValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectable_values: Option<FilterSelectableValues>,
    #[serde(rename = "SourceFilterId")]
    #[serde(default)]
    pub source_filter_id: String,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterListControl {
    #[serde(rename = "CascadingControlConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cascading_control_configuration: Option<CascadingControlConfiguration>,
    #[serde(rename = "ControlSortConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_sort_configurations: Option<Vec<ControlSortConfiguration>>,
    #[serde(rename = "ControlTitleFormatText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_title_format_text: Option<ControlTitleFormatText>,
    #[serde(rename = "DisplayOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_options: Option<ListControlDisplayOptions>,
    #[serde(rename = "FilterControlId")]
    #[serde(default)]
    pub filter_control_id: String,
    #[serde(rename = "SelectableValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectable_values: Option<FilterSelectableValues>,
    #[serde(rename = "SourceFilterId")]
    #[serde(default)]
    pub source_filter_id: String,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterRelativeDateTimeControl {
    #[serde(rename = "CommitMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_mode: Option<String>,
    #[serde(rename = "ControlTitleFormatText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_title_format_text: Option<ControlTitleFormatText>,
    #[serde(rename = "DisplayOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_options: Option<RelativeDateTimeControlDisplayOptions>,
    #[serde(rename = "FilterControlId")]
    #[serde(default)]
    pub filter_control_id: String,
    #[serde(rename = "SourceFilterId")]
    #[serde(default)]
    pub source_filter_id: String,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterSliderControl {
    #[serde(rename = "ControlTitleFormatText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_title_format_text: Option<ControlTitleFormatText>,
    #[serde(rename = "DisplayOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_options: Option<SliderControlDisplayOptions>,
    #[serde(rename = "FilterControlId")]
    #[serde(default)]
    pub filter_control_id: String,
    #[serde(rename = "MaximumValue")]
    #[serde(default)]
    pub maximum_value: f64,
    #[serde(rename = "MinimumValue")]
    #[serde(default)]
    pub minimum_value: f64,
    #[serde(rename = "SourceFilterId")]
    #[serde(default)]
    pub source_filter_id: String,
    #[serde(rename = "StepSize")]
    #[serde(default)]
    pub step_size: f64,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterTextAreaControl {
    #[serde(rename = "ControlTitleFormatText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_title_format_text: Option<ControlTitleFormatText>,
    #[serde(rename = "Delimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    #[serde(rename = "DisplayOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_options: Option<TextAreaControlDisplayOptions>,
    #[serde(rename = "FilterControlId")]
    #[serde(default)]
    pub filter_control_id: String,
    #[serde(rename = "SourceFilterId")]
    #[serde(default)]
    pub source_filter_id: String,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterTextFieldControl {
    #[serde(rename = "ControlTitleFormatText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_title_format_text: Option<ControlTitleFormatText>,
    #[serde(rename = "DisplayOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_options: Option<TextFieldControlDisplayOptions>,
    #[serde(rename = "FilterControlId")]
    #[serde(default)]
    pub filter_control_id: String,
    #[serde(rename = "SourceFilterId")]
    #[serde(default)]
    pub source_filter_id: String,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SheetImage {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<ImageCustomAction>>,
    #[serde(rename = "ImageContentAltText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_content_alt_text: Option<String>,
    #[serde(rename = "Interactions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions: Option<ImageInteractionOptions>,
    #[serde(rename = "Scaling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling: Option<SheetImageScalingConfiguration>,
    #[serde(rename = "SheetImageId")]
    #[serde(default)]
    pub sheet_image_id: String,
    #[serde(rename = "Source")]
    #[serde(default)]
    pub source: SheetImageSource,
    #[serde(rename = "Tooltip")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<SheetImageTooltipConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImageCustomAction {
    #[serde(rename = "ActionOperations")]
    #[serde(default)]
    pub action_operations: Vec<ImageCustomActionOperation>,
    #[serde(rename = "CustomActionId")]
    #[serde(default)]
    pub custom_action_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Trigger")]
    #[serde(default)]
    pub trigger: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImageCustomActionOperation {
    #[serde(rename = "NavigationOperation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub navigation_operation: Option<CustomActionNavigationOperation>,
    #[serde(rename = "SetParametersOperation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_parameters_operation: Option<CustomActionSetParametersOperation>,
    #[serde(rename = "URLOperation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_r_l_operation: Option<CustomActionURLOperation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomActionNavigationOperation {
    #[serde(rename = "LocalNavigationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_navigation_configuration: Option<LocalNavigationConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LocalNavigationConfiguration {
    #[serde(rename = "TargetSheetId")]
    #[serde(default)]
    pub target_sheet_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomActionSetParametersOperation {
    #[serde(rename = "ParameterValueConfigurations")]
    #[serde(default)]
    pub parameter_value_configurations: Vec<SetParameterValueConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetParameterValueConfiguration {
    #[serde(rename = "DestinationParameterName")]
    #[serde(default)]
    pub destination_parameter_name: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: DestinationParameterValueConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DestinationParameterValueConfiguration {
    #[serde(rename = "CustomValuesConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_values_configuration: Option<CustomValuesConfiguration>,
    #[serde(rename = "SelectAllValueOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_all_value_options: Option<String>,
    #[serde(rename = "SourceColumn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_column: Option<ColumnIdentifier>,
    #[serde(rename = "SourceField")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_field: Option<String>,
    #[serde(rename = "SourceParameterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_parameter_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomValuesConfiguration {
    #[serde(rename = "CustomValues")]
    #[serde(default)]
    pub custom_values: CustomParameterValues,
    #[serde(rename = "IncludeNullValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_null_value: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomParameterValues {
    #[serde(rename = "DateTimeValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time_values: Option<Vec<f64>>,
    #[serde(rename = "DecimalValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal_values: Option<Vec<f64>>,
    #[serde(rename = "IntegerValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integer_values: Option<Vec<i64>>,
    #[serde(rename = "StringValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomActionURLOperation {
    #[serde(rename = "URLTarget")]
    #[serde(default)]
    pub u_r_l_target: String,
    #[serde(rename = "URLTemplate")]
    #[serde(default)]
    pub u_r_l_template: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImageInteractionOptions {
    #[serde(rename = "ImageMenuOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_menu_option: Option<ImageMenuOption>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImageMenuOption {
    #[serde(rename = "AvailabilityStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SheetImageScalingConfiguration {
    #[serde(rename = "ScalingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SheetImageSource {
    #[serde(rename = "SheetImageStaticFileSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_image_static_file_source: Option<SheetImageStaticFileSource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SheetImageStaticFileSource {
    #[serde(rename = "StaticFileId")]
    #[serde(default)]
    pub static_file_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SheetImageTooltipConfiguration {
    #[serde(rename = "TooltipText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip_text: Option<SheetImageTooltipText>,
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SheetImageTooltipText {
    #[serde(rename = "PlainText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plain_text: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Layout {
    #[serde(rename = "Configuration")]
    #[serde(default)]
    pub configuration: LayoutConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LayoutConfiguration {
    #[serde(rename = "FreeFormLayout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_form_layout: Option<FreeFormLayoutConfiguration>,
    #[serde(rename = "GridLayout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid_layout: Option<GridLayoutConfiguration>,
    #[serde(rename = "SectionBasedLayout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section_based_layout: Option<SectionBasedLayoutConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FreeFormLayoutConfiguration {
    #[serde(rename = "CanvasSizeOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canvas_size_options: Option<FreeFormLayoutCanvasSizeOptions>,
    #[serde(rename = "Elements")]
    #[serde(default)]
    pub elements: Vec<FreeFormLayoutElement>,
    #[serde(rename = "Groups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<SheetLayoutGroup>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FreeFormLayoutElement {
    #[serde(rename = "BackgroundStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_style: Option<FreeFormLayoutElementBackgroundStyle>,
    #[serde(rename = "BorderRadius")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_radius: Option<String>,
    #[serde(rename = "BorderStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_style: Option<FreeFormLayoutElementBorderStyle>,
    #[serde(rename = "ElementId")]
    #[serde(default)]
    pub element_id: String,
    #[serde(rename = "ElementType")]
    #[serde(default)]
    pub element_type: String,
    #[serde(rename = "Height")]
    #[serde(default)]
    pub height: String,
    #[serde(rename = "LoadingAnimation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loading_animation: Option<LoadingAnimation>,
    #[serde(rename = "Padding")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<String>,
    #[serde(rename = "RenderingRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_rules: Option<Vec<SheetElementRenderingRule>>,
    #[serde(rename = "SelectedBorderStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_border_style: Option<FreeFormLayoutElementBorderStyle>,
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(rename = "Width")]
    #[serde(default)]
    pub width: String,
    #[serde(rename = "XAxisLocation")]
    #[serde(default)]
    pub x_axis_location: String,
    #[serde(rename = "YAxisLocation")]
    #[serde(default)]
    pub y_axis_location: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FreeFormLayoutElementBackgroundStyle {
    #[serde(rename = "Color")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FreeFormLayoutElementBorderStyle {
    #[serde(rename = "Color")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(rename = "Width")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoadingAnimation {
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SheetElementRenderingRule {
    #[serde(rename = "ConfigurationOverrides")]
    #[serde(default)]
    pub configuration_overrides: SheetElementConfigurationOverrides,
    #[serde(rename = "Expression")]
    #[serde(default)]
    pub expression: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SheetElementConfigurationOverrides {
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SheetLayoutGroup {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Members")]
    #[serde(default)]
    pub members: Vec<SheetLayoutGroupMember>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SheetLayoutGroupMember {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GridLayoutConfiguration {
    #[serde(rename = "CanvasSizeOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canvas_size_options: Option<GridLayoutCanvasSizeOptions>,
    #[serde(rename = "Elements")]
    #[serde(default)]
    pub elements: Vec<GridLayoutElement>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GridLayoutElement {
    #[serde(rename = "BackgroundStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_style: Option<GridLayoutElementBackgroundStyle>,
    #[serde(rename = "BorderRadius")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_radius: Option<String>,
    #[serde(rename = "BorderStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_style: Option<GridLayoutElementBorderStyle>,
    #[serde(rename = "ColumnIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_index: Option<i32>,
    #[serde(rename = "ColumnSpan")]
    #[serde(default)]
    pub column_span: i32,
    #[serde(rename = "ElementId")]
    #[serde(default)]
    pub element_id: String,
    #[serde(rename = "ElementType")]
    #[serde(default)]
    pub element_type: String,
    #[serde(rename = "LoadingAnimation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loading_animation: Option<LoadingAnimation>,
    #[serde(rename = "Padding")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<String>,
    #[serde(rename = "RowIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_index: Option<i32>,
    #[serde(rename = "RowSpan")]
    #[serde(default)]
    pub row_span: i32,
    #[serde(rename = "SelectedBorderStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_border_style: Option<GridLayoutElementBorderStyle>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GridLayoutElementBackgroundStyle {
    #[serde(rename = "Color")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GridLayoutElementBorderStyle {
    #[serde(rename = "Color")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(rename = "Width")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SectionBasedLayoutConfiguration {
    #[serde(rename = "BodySections")]
    #[serde(default)]
    pub body_sections: Vec<BodySectionConfiguration>,
    #[serde(rename = "CanvasSizeOptions")]
    #[serde(default)]
    pub canvas_size_options: SectionBasedLayoutCanvasSizeOptions,
    #[serde(rename = "FooterSections")]
    #[serde(default)]
    pub footer_sections: Vec<HeaderFooterSectionConfiguration>,
    #[serde(rename = "HeaderSections")]
    #[serde(default)]
    pub header_sections: Vec<HeaderFooterSectionConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BodySectionConfiguration {
    #[serde(rename = "Content")]
    #[serde(default)]
    pub content: BodySectionContent,
    #[serde(rename = "PageBreakConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_break_configuration: Option<SectionPageBreakConfiguration>,
    #[serde(rename = "RepeatConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_configuration: Option<BodySectionRepeatConfiguration>,
    #[serde(rename = "SectionId")]
    #[serde(default)]
    pub section_id: String,
    #[serde(rename = "Style")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<SectionStyle>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BodySectionContent {
    #[serde(rename = "Layout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<SectionLayoutConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SectionLayoutConfiguration {
    #[serde(rename = "FreeFormLayout")]
    #[serde(default)]
    pub free_form_layout: FreeFormSectionLayoutConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FreeFormSectionLayoutConfiguration {
    #[serde(rename = "Elements")]
    #[serde(default)]
    pub elements: Vec<FreeFormLayoutElement>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SectionPageBreakConfiguration {
    #[serde(rename = "After")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<SectionAfterPageBreak>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SectionAfterPageBreak {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BodySectionRepeatConfiguration {
    #[serde(rename = "DimensionConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_configurations: Option<Vec<BodySectionRepeatDimensionConfiguration>>,
    #[serde(rename = "NonRepeatingVisuals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_repeating_visuals: Option<Vec<String>>,
    #[serde(rename = "PageBreakConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_break_configuration: Option<BodySectionRepeatPageBreakConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BodySectionRepeatDimensionConfiguration {
    #[serde(rename = "DynamicCategoryDimensionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_category_dimension_configuration:
        Option<BodySectionDynamicCategoryDimensionConfiguration>,
    #[serde(rename = "DynamicNumericDimensionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_numeric_dimension_configuration:
        Option<BodySectionDynamicNumericDimensionConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BodySectionDynamicCategoryDimensionConfiguration {
    #[serde(rename = "Column")]
    #[serde(default)]
    pub column: ColumnIdentifier,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "SortByMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by_metrics: Option<Vec<ColumnSort>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ColumnSort {
    #[serde(rename = "AggregationFunction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_function: Option<AggregationFunction>,
    #[serde(rename = "Direction")]
    #[serde(default)]
    pub direction: String,
    #[serde(rename = "SortBy")]
    #[serde(default)]
    pub sort_by: ColumnIdentifier,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BodySectionDynamicNumericDimensionConfiguration {
    #[serde(rename = "Column")]
    #[serde(default)]
    pub column: ColumnIdentifier,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "SortByMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by_metrics: Option<Vec<ColumnSort>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BodySectionRepeatPageBreakConfiguration {
    #[serde(rename = "After")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<SectionAfterPageBreak>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SectionStyle {
    #[serde(rename = "Height")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
    #[serde(rename = "Padding")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<Spacing>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HeaderFooterSectionConfiguration {
    #[serde(rename = "Layout")]
    #[serde(default)]
    pub layout: SectionLayoutConfiguration,
    #[serde(rename = "SectionId")]
    #[serde(default)]
    pub section_id: String,
    #[serde(rename = "Style")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<SectionStyle>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParameterControl {
    #[serde(rename = "DateTimePicker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time_picker: Option<ParameterDateTimePickerControl>,
    #[serde(rename = "Dropdown")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropdown: Option<ParameterDropDownControl>,
    #[serde(rename = "List")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<ParameterListControl>,
    #[serde(rename = "Slider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slider: Option<ParameterSliderControl>,
    #[serde(rename = "TextArea")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_area: Option<ParameterTextAreaControl>,
    #[serde(rename = "TextField")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_field: Option<ParameterTextFieldControl>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParameterDateTimePickerControl {
    #[serde(rename = "ControlTitleFormatText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_title_format_text: Option<ControlTitleFormatText>,
    #[serde(rename = "DisplayOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_options: Option<DateTimePickerControlDisplayOptions>,
    #[serde(rename = "ParameterControlId")]
    #[serde(default)]
    pub parameter_control_id: String,
    #[serde(rename = "SourceParameterName")]
    #[serde(default)]
    pub source_parameter_name: String,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParameterDropDownControl {
    #[serde(rename = "CascadingControlConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cascading_control_configuration: Option<CascadingControlConfiguration>,
    #[serde(rename = "CommitMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_mode: Option<String>,
    #[serde(rename = "ControlSortConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_sort_configurations: Option<Vec<ControlSortConfiguration>>,
    #[serde(rename = "ControlTitleFormatText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_title_format_text: Option<ControlTitleFormatText>,
    #[serde(rename = "DisplayOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_options: Option<DropDownControlDisplayOptions>,
    #[serde(rename = "ParameterControlId")]
    #[serde(default)]
    pub parameter_control_id: String,
    #[serde(rename = "SelectableValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectable_values: Option<ParameterSelectableValues>,
    #[serde(rename = "SourceParameterName")]
    #[serde(default)]
    pub source_parameter_name: String,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParameterSelectableValues {
    #[serde(rename = "LinkToDataSetColumn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_to_data_set_column: Option<ColumnIdentifier>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParameterListControl {
    #[serde(rename = "CascadingControlConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cascading_control_configuration: Option<CascadingControlConfiguration>,
    #[serde(rename = "ControlSortConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_sort_configurations: Option<Vec<ControlSortConfiguration>>,
    #[serde(rename = "ControlTitleFormatText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_title_format_text: Option<ControlTitleFormatText>,
    #[serde(rename = "DisplayOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_options: Option<ListControlDisplayOptions>,
    #[serde(rename = "ParameterControlId")]
    #[serde(default)]
    pub parameter_control_id: String,
    #[serde(rename = "SelectableValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectable_values: Option<ParameterSelectableValues>,
    #[serde(rename = "SourceParameterName")]
    #[serde(default)]
    pub source_parameter_name: String,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParameterSliderControl {
    #[serde(rename = "ControlTitleFormatText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_title_format_text: Option<ControlTitleFormatText>,
    #[serde(rename = "DisplayOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_options: Option<SliderControlDisplayOptions>,
    #[serde(rename = "MaximumValue")]
    #[serde(default)]
    pub maximum_value: f64,
    #[serde(rename = "MinimumValue")]
    #[serde(default)]
    pub minimum_value: f64,
    #[serde(rename = "ParameterControlId")]
    #[serde(default)]
    pub parameter_control_id: String,
    #[serde(rename = "SourceParameterName")]
    #[serde(default)]
    pub source_parameter_name: String,
    #[serde(rename = "StepSize")]
    #[serde(default)]
    pub step_size: f64,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParameterTextAreaControl {
    #[serde(rename = "ControlTitleFormatText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_title_format_text: Option<ControlTitleFormatText>,
    #[serde(rename = "Delimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    #[serde(rename = "DisplayOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_options: Option<TextAreaControlDisplayOptions>,
    #[serde(rename = "ParameterControlId")]
    #[serde(default)]
    pub parameter_control_id: String,
    #[serde(rename = "SourceParameterName")]
    #[serde(default)]
    pub source_parameter_name: String,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParameterTextFieldControl {
    #[serde(rename = "ControlTitleFormatText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_title_format_text: Option<ControlTitleFormatText>,
    #[serde(rename = "DisplayOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_options: Option<TextFieldControlDisplayOptions>,
    #[serde(rename = "ParameterControlId")]
    #[serde(default)]
    pub parameter_control_id: String,
    #[serde(rename = "SourceParameterName")]
    #[serde(default)]
    pub source_parameter_name: String,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SheetControlLayout {
    #[serde(rename = "Configuration")]
    #[serde(default)]
    pub configuration: SheetControlLayoutConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SheetControlLayoutConfiguration {
    #[serde(rename = "GridLayout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid_layout: Option<GridLayoutConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SheetTextBox {
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "Interactions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions: Option<TextBoxInteractionOptions>,
    #[serde(rename = "SheetTextBoxId")]
    #[serde(default)]
    pub sheet_text_box_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TextBoxInteractionOptions {
    #[serde(rename = "TextBoxMenuOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_box_menu_option: Option<TextBoxMenuOption>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TextBoxMenuOption {
    #[serde(rename = "AvailabilityStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Visual {
    #[serde(rename = "BarChartVisual")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar_chart_visual: Option<BarChartVisual>,
    #[serde(rename = "BoxPlotVisual")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub box_plot_visual: Option<BoxPlotVisual>,
    #[serde(rename = "ComboChartVisual")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub combo_chart_visual: Option<ComboChartVisual>,
    #[serde(rename = "CustomContentVisual")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_content_visual: Option<CustomContentVisual>,
    #[serde(rename = "EmptyVisual")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_visual: Option<EmptyVisual>,
    #[serde(rename = "FilledMapVisual")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filled_map_visual: Option<FilledMapVisual>,
    #[serde(rename = "FunnelChartVisual")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funnel_chart_visual: Option<FunnelChartVisual>,
    #[serde(rename = "GaugeChartVisual")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gauge_chart_visual: Option<GaugeChartVisual>,
    #[serde(rename = "GeospatialMapVisual")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geospatial_map_visual: Option<GeospatialMapVisual>,
    #[serde(rename = "HeatMapVisual")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heat_map_visual: Option<HeatMapVisual>,
    #[serde(rename = "HistogramVisual")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub histogram_visual: Option<HistogramVisual>,
    #[serde(rename = "InsightVisual")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_visual: Option<InsightVisual>,
    #[serde(rename = "KPIVisual")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_p_i_visual: Option<KPIVisual>,
    #[serde(rename = "LayerMapVisual")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_map_visual: Option<LayerMapVisual>,
    #[serde(rename = "LineChartVisual")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_chart_visual: Option<LineChartVisual>,
    #[serde(rename = "PieChartVisual")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pie_chart_visual: Option<PieChartVisual>,
    #[serde(rename = "PivotTableVisual")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pivot_table_visual: Option<PivotTableVisual>,
    #[serde(rename = "PluginVisual")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_visual: Option<PluginVisual>,
    #[serde(rename = "RadarChartVisual")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_chart_visual: Option<RadarChartVisual>,
    #[serde(rename = "SankeyDiagramVisual")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sankey_diagram_visual: Option<SankeyDiagramVisual>,
    #[serde(rename = "ScatterPlotVisual")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scatter_plot_visual: Option<ScatterPlotVisual>,
    #[serde(rename = "TableVisual")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_visual: Option<TableVisual>,
    #[serde(rename = "TreeMapVisual")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tree_map_visual: Option<TreeMapVisual>,
    #[serde(rename = "WaterfallVisual")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waterfall_visual: Option<WaterfallVisual>,
    #[serde(rename = "WordCloudVisual")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_cloud_visual: Option<WordCloudVisual>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BarChartVisual {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ChartConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_configuration: Option<BarChartConfiguration>,
    #[serde(rename = "ColumnHierarchies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "Subtitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "VisualContentAltText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_content_alt_text: Option<String>,
    #[serde(rename = "VisualId")]
    #[serde(default)]
    pub visual_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VisualCustomAction {
    #[serde(rename = "ActionOperations")]
    #[serde(default)]
    pub action_operations: Vec<VisualCustomActionOperation>,
    #[serde(rename = "CustomActionId")]
    #[serde(default)]
    pub custom_action_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Trigger")]
    #[serde(default)]
    pub trigger: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VisualCustomActionOperation {
    #[serde(rename = "FilterOperation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_operation: Option<CustomActionFilterOperation>,
    #[serde(rename = "NavigationOperation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub navigation_operation: Option<CustomActionNavigationOperation>,
    #[serde(rename = "SetParametersOperation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_parameters_operation: Option<CustomActionSetParametersOperation>,
    #[serde(rename = "URLOperation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_r_l_operation: Option<CustomActionURLOperation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomActionFilterOperation {
    #[serde(rename = "SelectedFieldsConfiguration")]
    #[serde(default)]
    pub selected_fields_configuration: FilterOperationSelectedFieldsConfiguration,
    #[serde(rename = "TargetVisualsConfiguration")]
    #[serde(default)]
    pub target_visuals_configuration: FilterOperationTargetVisualsConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterOperationSelectedFieldsConfiguration {
    #[serde(rename = "SelectedColumns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_columns: Option<Vec<ColumnIdentifier>>,
    #[serde(rename = "SelectedFieldOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_field_options: Option<String>,
    #[serde(rename = "SelectedFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_fields: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterOperationTargetVisualsConfiguration {
    #[serde(rename = "SameSheetTargetVisualConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub same_sheet_target_visual_configuration: Option<SameSheetTargetVisualConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SameSheetTargetVisualConfiguration {
    #[serde(rename = "TargetVisualOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_visual_options: Option<String>,
    #[serde(rename = "TargetVisuals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_visuals: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BarChartConfiguration {
    #[serde(rename = "BarsArrangement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bars_arrangement: Option<String>,
    #[serde(rename = "CategoryAxis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_axis: Option<AxisDisplayOptions>,
    #[serde(rename = "CategoryLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "ColorLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "ContributionAnalysisDefaults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contribution_analysis_defaults: Option<Vec<ContributionAnalysisDefault>>,
    #[serde(rename = "DataLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_labels: Option<DataLabelOptions>,
    #[serde(rename = "DefaultSeriesSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_series_settings: Option<BarChartDefaultSeriesSettings>,
    #[serde(rename = "FieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_wells: Option<BarChartFieldWells>,
    #[serde(rename = "Interactions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions: Option<VisualInteractionOptions>,
    #[serde(rename = "Legend")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "Orientation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<String>,
    #[serde(rename = "ReferenceLines")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_lines: Option<Vec<ReferenceLine>>,
    #[serde(rename = "Series")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<Vec<BarSeriesItem>>,
    #[serde(rename = "SmallMultiplesOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_multiples_options: Option<SmallMultiplesOptions>,
    #[serde(rename = "SortConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_configuration: Option<BarChartSortConfiguration>,
    #[serde(rename = "Tooltip")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "ValueAxis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_axis: Option<AxisDisplayOptions>,
    #[serde(rename = "ValueLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "VisualPalette")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_palette: Option<VisualPalette>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AxisDisplayOptions {
    #[serde(rename = "AxisLineVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis_line_visibility: Option<String>,
    #[serde(rename = "AxisOffset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis_offset: Option<String>,
    #[serde(rename = "DataOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_options: Option<AxisDataOptions>,
    #[serde(rename = "GridLineVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid_line_visibility: Option<String>,
    #[serde(rename = "ScrollbarOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrollbar_options: Option<ScrollBarOptions>,
    #[serde(rename = "TickLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tick_label_options: Option<AxisTickLabelOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AxisDataOptions {
    #[serde(rename = "DateAxisOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_axis_options: Option<DateAxisOptions>,
    #[serde(rename = "NumericAxisOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric_axis_options: Option<NumericAxisOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DateAxisOptions {
    #[serde(rename = "MissingDateVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_date_visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NumericAxisOptions {
    #[serde(rename = "Range")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<AxisDisplayRange>,
    #[serde(rename = "Scale")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<AxisScale>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AxisDisplayRange {
    #[serde(rename = "DataDriven")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_driven: Option<AxisDisplayDataDrivenRange>,
    #[serde(rename = "MinMax")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_max: Option<AxisDisplayMinMaxRange>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AxisDisplayDataDrivenRange {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AxisDisplayMinMaxRange {
    #[serde(rename = "Maximum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[serde(rename = "Minimum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AxisScale {
    #[serde(rename = "Linear")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linear: Option<AxisLinearScale>,
    #[serde(rename = "Logarithmic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logarithmic: Option<AxisLogarithmicScale>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AxisLinearScale {
    #[serde(rename = "StepCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_count: Option<i32>,
    #[serde(rename = "StepSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_size: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AxisLogarithmicScale {
    #[serde(rename = "Base")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScrollBarOptions {
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(rename = "VisibleRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible_range: Option<VisibleRangeOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VisibleRangeOptions {
    #[serde(rename = "PercentRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_range: Option<PercentVisibleRange>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PercentVisibleRange {
    #[serde(rename = "From")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<f64>,
    #[serde(rename = "To")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AxisTickLabelOptions {
    #[serde(rename = "LabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_options: Option<LabelOptions>,
    #[serde(rename = "RotationAngle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_angle: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChartAxisLabelOptions {
    #[serde(rename = "AxisLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis_label_options: Option<Vec<AxisLabelOptions>>,
    #[serde(rename = "SortIconVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_icon_visibility: Option<String>,
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AxisLabelOptions {
    #[serde(rename = "ApplyTo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_to: Option<AxisLabelReferenceOptions>,
    #[serde(rename = "CustomLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_label: Option<String>,
    #[serde(rename = "FontConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_configuration: Option<FontConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AxisLabelReferenceOptions {
    #[serde(rename = "Column")]
    #[serde(default)]
    pub column: ColumnIdentifier,
    #[serde(rename = "FieldId")]
    #[serde(default)]
    pub field_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContributionAnalysisDefault {
    #[serde(rename = "ContributorDimensions")]
    #[serde(default)]
    pub contributor_dimensions: Vec<ColumnIdentifier>,
    #[serde(rename = "MeasureFieldId")]
    #[serde(default)]
    pub measure_field_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataLabelOptions {
    #[serde(rename = "CategoryLabelVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_label_visibility: Option<String>,
    #[serde(rename = "DataLabelTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_label_types: Option<Vec<DataLabelType>>,
    #[serde(rename = "LabelColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_color: Option<String>,
    #[serde(rename = "LabelContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_content: Option<String>,
    #[serde(rename = "LabelFontConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_font_configuration: Option<FontConfiguration>,
    #[serde(rename = "MeasureLabelVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measure_label_visibility: Option<String>,
    #[serde(rename = "Overlap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlap: Option<String>,
    #[serde(rename = "Position")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(rename = "TotalsVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totals_visibility: Option<String>,
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataLabelType {
    #[serde(rename = "DataPathLabelType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_path_label_type: Option<DataPathLabelType>,
    #[serde(rename = "FieldLabelType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_label_type: Option<FieldLabelType>,
    #[serde(rename = "MaximumLabelType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_label_type: Option<MaximumLabelType>,
    #[serde(rename = "MinimumLabelType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_label_type: Option<MinimumLabelType>,
    #[serde(rename = "RangeEndsLabelType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_ends_label_type: Option<RangeEndsLabelType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataPathLabelType {
    #[serde(rename = "FieldId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    #[serde(rename = "FieldValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_value: Option<String>,
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FieldLabelType {
    #[serde(rename = "FieldId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MaximumLabelType {
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MinimumLabelType {
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RangeEndsLabelType {
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BarChartDefaultSeriesSettings {
    #[serde(rename = "BorderSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_settings: Option<BorderSettings>,
    #[serde(rename = "DecalSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decal_settings: Option<DecalSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BorderSettings {
    #[serde(rename = "BorderColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_color: Option<String>,
    #[serde(rename = "BorderVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_visibility: Option<String>,
    #[serde(rename = "BorderWidth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_width: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BarChartFieldWells {
    #[serde(rename = "BarChartAggregatedFieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar_chart_aggregated_field_wells: Option<BarChartAggregatedFieldWells>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BarChartAggregatedFieldWells {
    #[serde(rename = "Category")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Vec<DimensionField>>,
    #[serde(rename = "Colors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colors: Option<Vec<DimensionField>>,
    #[serde(rename = "SmallMultiples")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_multiples: Option<Vec<DimensionField>>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<MeasureField>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DimensionField {
    #[serde(rename = "CategoricalDimensionField")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categorical_dimension_field: Option<CategoricalDimensionField>,
    #[serde(rename = "DateDimensionField")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_dimension_field: Option<DateDimensionField>,
    #[serde(rename = "NumericalDimensionField")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numerical_dimension_field: Option<NumericalDimensionField>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CategoricalDimensionField {
    #[serde(rename = "Column")]
    #[serde(default)]
    pub column: ColumnIdentifier,
    #[serde(rename = "FieldId")]
    #[serde(default)]
    pub field_id: String,
    #[serde(rename = "FormatConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_configuration: Option<StringFormatConfiguration>,
    #[serde(rename = "HierarchyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DateDimensionField {
    #[serde(rename = "Column")]
    #[serde(default)]
    pub column: ColumnIdentifier,
    #[serde(rename = "DateGranularity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_granularity: Option<String>,
    #[serde(rename = "FieldId")]
    #[serde(default)]
    pub field_id: String,
    #[serde(rename = "FormatConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_configuration: Option<DateTimeFormatConfiguration>,
    #[serde(rename = "HierarchyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NumericalDimensionField {
    #[serde(rename = "Column")]
    #[serde(default)]
    pub column: ColumnIdentifier,
    #[serde(rename = "FieldId")]
    #[serde(default)]
    pub field_id: String,
    #[serde(rename = "FormatConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_configuration: Option<NumberFormatConfiguration>,
    #[serde(rename = "HierarchyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchy_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MeasureField {
    #[serde(rename = "CalculatedMeasureField")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculated_measure_field: Option<CalculatedMeasureField>,
    #[serde(rename = "CategoricalMeasureField")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categorical_measure_field: Option<CategoricalMeasureField>,
    #[serde(rename = "DateMeasureField")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_measure_field: Option<DateMeasureField>,
    #[serde(rename = "NumericalMeasureField")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numerical_measure_field: Option<NumericalMeasureField>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CalculatedMeasureField {
    #[serde(rename = "Expression")]
    #[serde(default)]
    pub expression: String,
    #[serde(rename = "FieldId")]
    #[serde(default)]
    pub field_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CategoricalMeasureField {
    #[serde(rename = "AggregationFunction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_function: Option<String>,
    #[serde(rename = "Column")]
    #[serde(default)]
    pub column: ColumnIdentifier,
    #[serde(rename = "FieldId")]
    #[serde(default)]
    pub field_id: String,
    #[serde(rename = "FormatConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_configuration: Option<StringFormatConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DateMeasureField {
    #[serde(rename = "AggregationFunction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_function: Option<String>,
    #[serde(rename = "Column")]
    #[serde(default)]
    pub column: ColumnIdentifier,
    #[serde(rename = "FieldId")]
    #[serde(default)]
    pub field_id: String,
    #[serde(rename = "FormatConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_configuration: Option<DateTimeFormatConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NumericalMeasureField {
    #[serde(rename = "AggregationFunction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_function: Option<NumericalAggregationFunction>,
    #[serde(rename = "Column")]
    #[serde(default)]
    pub column: ColumnIdentifier,
    #[serde(rename = "FieldId")]
    #[serde(default)]
    pub field_id: String,
    #[serde(rename = "FormatConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_configuration: Option<NumberFormatConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VisualInteractionOptions {
    #[serde(rename = "ContextMenuOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_menu_option: Option<ContextMenuOption>,
    #[serde(rename = "VisualMenuOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_menu_option: Option<VisualMenuOption>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContextMenuOption {
    #[serde(rename = "AvailabilityStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VisualMenuOption {
    #[serde(rename = "AvailabilityStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LegendOptions {
    #[serde(rename = "Height")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
    #[serde(rename = "Position")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<LabelOptions>,
    #[serde(rename = "ValueFontConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_font_configuration: Option<FontConfiguration>,
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(rename = "Width")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReferenceLine {
    #[serde(rename = "DataConfiguration")]
    #[serde(default)]
    pub data_configuration: ReferenceLineDataConfiguration,
    #[serde(rename = "LabelConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_configuration: Option<ReferenceLineLabelConfiguration>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StyleConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style_configuration: Option<ReferenceLineStyleConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReferenceLineDataConfiguration {
    #[serde(rename = "AxisBinding")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis_binding: Option<String>,
    #[serde(rename = "DynamicConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_configuration: Option<ReferenceLineDynamicDataConfiguration>,
    #[serde(rename = "SeriesType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series_type: Option<String>,
    #[serde(rename = "StaticConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_configuration: Option<ReferenceLineStaticDataConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReferenceLineDynamicDataConfiguration {
    #[serde(rename = "Calculation")]
    #[serde(default)]
    pub calculation: NumericalAggregationFunction,
    #[serde(rename = "Column")]
    #[serde(default)]
    pub column: ColumnIdentifier,
    #[serde(rename = "MeasureAggregationFunction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measure_aggregation_function: Option<AggregationFunction>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReferenceLineStaticDataConfiguration {
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReferenceLineLabelConfiguration {
    #[serde(rename = "CustomLabelConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_label_configuration: Option<ReferenceLineCustomLabelConfiguration>,
    #[serde(rename = "FontColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_color: Option<String>,
    #[serde(rename = "FontConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_configuration: Option<FontConfiguration>,
    #[serde(rename = "HorizontalPosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_position: Option<String>,
    #[serde(rename = "ValueLabelConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_label_configuration: Option<ReferenceLineValueLabelConfiguration>,
    #[serde(rename = "VerticalPosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_position: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReferenceLineCustomLabelConfiguration {
    #[serde(rename = "CustomLabel")]
    #[serde(default)]
    pub custom_label: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReferenceLineValueLabelConfiguration {
    #[serde(rename = "FormatConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_configuration: Option<NumericFormatConfiguration>,
    #[serde(rename = "RelativePosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_position: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReferenceLineStyleConfiguration {
    #[serde(rename = "Color")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "Pattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BarSeriesItem {
    #[serde(rename = "DataFieldBarSeriesItem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_field_bar_series_item: Option<DataFieldBarSeriesItem>,
    #[serde(rename = "FieldBarSeriesItem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_bar_series_item: Option<FieldBarSeriesItem>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataFieldBarSeriesItem {
    #[serde(rename = "FieldId")]
    #[serde(default)]
    pub field_id: String,
    #[serde(rename = "FieldValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_value: Option<String>,
    #[serde(rename = "Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<BarChartSeriesSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BarChartSeriesSettings {
    #[serde(rename = "BorderSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_settings: Option<BorderSettings>,
    #[serde(rename = "DecalSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decal_settings: Option<DecalSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FieldBarSeriesItem {
    #[serde(rename = "FieldId")]
    #[serde(default)]
    pub field_id: String,
    #[serde(rename = "Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<BarChartSeriesSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SmallMultiplesOptions {
    #[serde(rename = "MaxVisibleColumns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_visible_columns: Option<i64>,
    #[serde(rename = "MaxVisibleRows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_visible_rows: Option<i64>,
    #[serde(rename = "PanelConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub panel_configuration: Option<PanelConfiguration>,
    #[serde(rename = "XAxis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_axis: Option<SmallMultiplesAxisProperties>,
    #[serde(rename = "YAxis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_axis: Option<SmallMultiplesAxisProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PanelConfiguration {
    #[serde(rename = "BackgroundColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(rename = "BackgroundVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_visibility: Option<String>,
    #[serde(rename = "BorderColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_color: Option<String>,
    #[serde(rename = "BorderStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_style: Option<String>,
    #[serde(rename = "BorderThickness")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_thickness: Option<String>,
    #[serde(rename = "BorderVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_visibility: Option<String>,
    #[serde(rename = "GutterSpacing")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gutter_spacing: Option<String>,
    #[serde(rename = "GutterVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gutter_visibility: Option<String>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<PanelTitleOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PanelTitleOptions {
    #[serde(rename = "FontConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_configuration: Option<FontConfiguration>,
    #[serde(rename = "HorizontalTextAlignment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_text_alignment: Option<String>,
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SmallMultiplesAxisProperties {
    #[serde(rename = "Placement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement: Option<String>,
    #[serde(rename = "Scale")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BarChartSortConfiguration {
    #[serde(rename = "CategoryItemsLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_items_limit: Option<ItemsLimitConfiguration>,
    #[serde(rename = "CategorySort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "ColorItemsLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_items_limit: Option<ItemsLimitConfiguration>,
    #[serde(rename = "ColorSort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "SmallMultiplesLimitConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_multiples_limit_configuration: Option<ItemsLimitConfiguration>,
    #[serde(rename = "SmallMultiplesSort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_multiples_sort: Option<Vec<FieldSortOptions>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ItemsLimitConfiguration {
    #[serde(rename = "ItemsLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items_limit: Option<i64>,
    #[serde(rename = "OtherCategories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_categories: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FieldSortOptions {
    #[serde(rename = "ColumnSort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_sort: Option<ColumnSort>,
    #[serde(rename = "FieldSort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_sort: Option<FieldSort>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FieldSort {
    #[serde(rename = "Direction")]
    #[serde(default)]
    pub direction: String,
    #[serde(rename = "FieldId")]
    #[serde(default)]
    pub field_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TooltipOptions {
    #[serde(rename = "FieldBasedTooltip")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_based_tooltip: Option<FieldBasedTooltip>,
    #[serde(rename = "SelectedTooltipType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_tooltip_type: Option<String>,
    #[serde(rename = "SheetTooltip")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_tooltip: Option<SheetTooltip>,
    #[serde(rename = "TooltipVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip_visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FieldBasedTooltip {
    #[serde(rename = "AggregationVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_visibility: Option<String>,
    #[serde(rename = "TooltipFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip_fields: Option<Vec<TooltipItem>>,
    #[serde(rename = "TooltipTitleType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip_title_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TooltipItem {
    #[serde(rename = "ColumnTooltipItem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_tooltip_item: Option<ColumnTooltipItem>,
    #[serde(rename = "FieldTooltipItem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_tooltip_item: Option<FieldTooltipItem>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ColumnTooltipItem {
    #[serde(rename = "Aggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<AggregationFunction>,
    #[serde(rename = "Column")]
    #[serde(default)]
    pub column: ColumnIdentifier,
    #[serde(rename = "Label")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "TooltipTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip_target: Option<String>,
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FieldTooltipItem {
    #[serde(rename = "FieldId")]
    #[serde(default)]
    pub field_id: String,
    #[serde(rename = "Label")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "TooltipTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip_target: Option<String>,
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SheetTooltip {
    #[serde(rename = "SheetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VisualPalette {
    #[serde(rename = "ChartColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_color: Option<String>,
    #[serde(rename = "ColorMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_map: Option<Vec<DataPathColor>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataPathColor {
    #[serde(rename = "Color")]
    #[serde(default)]
    pub color: String,
    #[serde(rename = "Element")]
    #[serde(default)]
    pub element: DataPathValue,
    #[serde(rename = "TimeGranularity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_granularity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataPathValue {
    #[serde(rename = "DataPathType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_path_type: Option<DataPathType>,
    #[serde(rename = "FieldId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    #[serde(rename = "FieldValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataPathType {
    #[serde(rename = "PivotTableDataPathType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pivot_table_data_path_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ColumnHierarchy {
    #[serde(rename = "DateTimeHierarchy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time_hierarchy: Option<DateTimeHierarchy>,
    #[serde(rename = "ExplicitHierarchy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_hierarchy: Option<ExplicitHierarchy>,
    #[serde(rename = "PredefinedHierarchy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predefined_hierarchy: Option<PredefinedHierarchy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DateTimeHierarchy {
    #[serde(rename = "DrillDownFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drill_down_filters: Option<Vec<DrillDownFilter>>,
    #[serde(rename = "HierarchyId")]
    #[serde(default)]
    pub hierarchy_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DrillDownFilter {
    #[serde(rename = "CategoryFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_filter: Option<CategoryDrillDownFilter>,
    #[serde(rename = "NumericEqualityFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric_equality_filter: Option<NumericEqualityDrillDownFilter>,
    #[serde(rename = "TimeRangeFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_range_filter: Option<TimeRangeDrillDownFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CategoryDrillDownFilter {
    #[serde(rename = "CategoryValues")]
    #[serde(default)]
    pub category_values: Vec<String>,
    #[serde(rename = "Column")]
    #[serde(default)]
    pub column: ColumnIdentifier,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NumericEqualityDrillDownFilter {
    #[serde(rename = "Column")]
    #[serde(default)]
    pub column: ColumnIdentifier,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimeRangeDrillDownFilter {
    #[serde(rename = "Column")]
    #[serde(default)]
    pub column: ColumnIdentifier,
    #[serde(rename = "RangeMaximum")]
    #[serde(default)]
    pub range_maximum: f64,
    #[serde(rename = "RangeMinimum")]
    #[serde(default)]
    pub range_minimum: f64,
    #[serde(rename = "TimeGranularity")]
    #[serde(default)]
    pub time_granularity: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExplicitHierarchy {
    #[serde(rename = "Columns")]
    #[serde(default)]
    pub columns: Vec<ColumnIdentifier>,
    #[serde(rename = "DrillDownFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drill_down_filters: Option<Vec<DrillDownFilter>>,
    #[serde(rename = "HierarchyId")]
    #[serde(default)]
    pub hierarchy_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PredefinedHierarchy {
    #[serde(rename = "Columns")]
    #[serde(default)]
    pub columns: Vec<ColumnIdentifier>,
    #[serde(rename = "DrillDownFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drill_down_filters: Option<Vec<DrillDownFilter>>,
    #[serde(rename = "HierarchyId")]
    #[serde(default)]
    pub hierarchy_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VisualSubtitleLabelOptions {
    #[serde(rename = "FormatText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_text: Option<LongFormatText>,
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LongFormatText {
    #[serde(rename = "PlainText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plain_text: Option<String>,
    #[serde(rename = "RichText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rich_text: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VisualTitleLabelOptions {
    #[serde(rename = "FormatText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_text: Option<ShortFormatText>,
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ShortFormatText {
    #[serde(rename = "PlainText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plain_text: Option<String>,
    #[serde(rename = "RichText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rich_text: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BoxPlotVisual {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ChartConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_configuration: Option<BoxPlotChartConfiguration>,
    #[serde(rename = "ColumnHierarchies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "Subtitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "VisualContentAltText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_content_alt_text: Option<String>,
    #[serde(rename = "VisualId")]
    #[serde(default)]
    pub visual_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BoxPlotChartConfiguration {
    #[serde(rename = "BoxPlotOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub box_plot_options: Option<BoxPlotOptions>,
    #[serde(rename = "CategoryAxis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_axis: Option<AxisDisplayOptions>,
    #[serde(rename = "CategoryLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "FieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_wells: Option<BoxPlotFieldWells>,
    #[serde(rename = "Interactions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions: Option<VisualInteractionOptions>,
    #[serde(rename = "Legend")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "PrimaryYAxisDisplayOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_y_axis_display_options: Option<AxisDisplayOptions>,
    #[serde(rename = "PrimaryYAxisLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_y_axis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "ReferenceLines")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_lines: Option<Vec<ReferenceLine>>,
    #[serde(rename = "SortConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_configuration: Option<BoxPlotSortConfiguration>,
    #[serde(rename = "Tooltip")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "VisualPalette")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_palette: Option<VisualPalette>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BoxPlotOptions {
    #[serde(rename = "AllDataPointsVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_data_points_visibility: Option<String>,
    #[serde(rename = "OutlierVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outlier_visibility: Option<String>,
    #[serde(rename = "StyleOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style_options: Option<BoxPlotStyleOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BoxPlotStyleOptions {
    #[serde(rename = "FillStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_style: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BoxPlotFieldWells {
    #[serde(rename = "BoxPlotAggregatedFieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub box_plot_aggregated_field_wells: Option<BoxPlotAggregatedFieldWells>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BoxPlotAggregatedFieldWells {
    #[serde(rename = "GroupBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<DimensionField>>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<MeasureField>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BoxPlotSortConfiguration {
    #[serde(rename = "CategorySort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "PaginationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_configuration: Option<PaginationConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PaginationConfiguration {
    #[serde(rename = "PageNumber")]
    #[serde(default)]
    pub page_number: i64,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    pub page_size: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComboChartVisual {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ChartConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_configuration: Option<ComboChartConfiguration>,
    #[serde(rename = "ColumnHierarchies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "Subtitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "VisualContentAltText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_content_alt_text: Option<String>,
    #[serde(rename = "VisualId")]
    #[serde(default)]
    pub visual_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComboChartConfiguration {
    #[serde(rename = "BarDataLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar_data_labels: Option<DataLabelOptions>,
    #[serde(rename = "BarsArrangement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bars_arrangement: Option<String>,
    #[serde(rename = "CategoryAxis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_axis: Option<AxisDisplayOptions>,
    #[serde(rename = "CategoryLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "ColorLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "DefaultSeriesSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_series_settings: Option<ComboChartDefaultSeriesSettings>,
    #[serde(rename = "FieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_wells: Option<ComboChartFieldWells>,
    #[serde(rename = "Interactions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions: Option<VisualInteractionOptions>,
    #[serde(rename = "Legend")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "LineDataLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_data_labels: Option<DataLabelOptions>,
    #[serde(rename = "PrimaryYAxisDisplayOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_y_axis_display_options: Option<AxisDisplayOptions>,
    #[serde(rename = "PrimaryYAxisLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_y_axis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "ReferenceLines")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_lines: Option<Vec<ReferenceLine>>,
    #[serde(rename = "SecondaryYAxisDisplayOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_y_axis_display_options: Option<AxisDisplayOptions>,
    #[serde(rename = "SecondaryYAxisLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_y_axis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "Series")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<Vec<ComboSeriesItem>>,
    #[serde(rename = "SingleAxisOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_axis_options: Option<SingleAxisOptions>,
    #[serde(rename = "SortConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_configuration: Option<ComboChartSortConfiguration>,
    #[serde(rename = "Tooltip")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "VisualPalette")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_palette: Option<VisualPalette>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComboChartDefaultSeriesSettings {
    #[serde(rename = "BorderSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_settings: Option<BorderSettings>,
    #[serde(rename = "DecalSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decal_settings: Option<DecalSettings>,
    #[serde(rename = "LineStyleSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_style_settings: Option<LineChartLineStyleSettings>,
    #[serde(rename = "MarkerStyleSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker_style_settings: Option<LineChartMarkerStyleSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LineChartLineStyleSettings {
    #[serde(rename = "LineInterpolation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_interpolation: Option<String>,
    #[serde(rename = "LineStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_style: Option<String>,
    #[serde(rename = "LineVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_visibility: Option<String>,
    #[serde(rename = "LineWidth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_width: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LineChartMarkerStyleSettings {
    #[serde(rename = "MarkerColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker_color: Option<String>,
    #[serde(rename = "MarkerShape")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker_shape: Option<String>,
    #[serde(rename = "MarkerSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker_size: Option<String>,
    #[serde(rename = "MarkerVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker_visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComboChartFieldWells {
    #[serde(rename = "ComboChartAggregatedFieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub combo_chart_aggregated_field_wells: Option<ComboChartAggregatedFieldWells>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComboChartAggregatedFieldWells {
    #[serde(rename = "BarValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar_values: Option<Vec<MeasureField>>,
    #[serde(rename = "Category")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Vec<DimensionField>>,
    #[serde(rename = "Colors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colors: Option<Vec<DimensionField>>,
    #[serde(rename = "LineValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_values: Option<Vec<MeasureField>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComboSeriesItem {
    #[serde(rename = "DataFieldComboSeriesItem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_field_combo_series_item: Option<DataFieldComboSeriesItem>,
    #[serde(rename = "FieldComboSeriesItem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_combo_series_item: Option<FieldComboSeriesItem>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataFieldComboSeriesItem {
    #[serde(rename = "FieldId")]
    #[serde(default)]
    pub field_id: String,
    #[serde(rename = "FieldValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_value: Option<String>,
    #[serde(rename = "Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<ComboChartSeriesSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComboChartSeriesSettings {
    #[serde(rename = "BorderSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_settings: Option<BorderSettings>,
    #[serde(rename = "DecalSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decal_settings: Option<DecalSettings>,
    #[serde(rename = "LineStyleSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_style_settings: Option<LineChartLineStyleSettings>,
    #[serde(rename = "MarkerStyleSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker_style_settings: Option<LineChartMarkerStyleSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FieldComboSeriesItem {
    #[serde(rename = "FieldId")]
    #[serde(default)]
    pub field_id: String,
    #[serde(rename = "Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<ComboChartSeriesSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SingleAxisOptions {
    #[serde(rename = "YAxisOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_axis_options: Option<YAxisOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct YAxisOptions {
    #[serde(rename = "YAxis")]
    #[serde(default)]
    pub y_axis: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComboChartSortConfiguration {
    #[serde(rename = "CategoryItemsLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_items_limit: Option<ItemsLimitConfiguration>,
    #[serde(rename = "CategorySort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "ColorItemsLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_items_limit: Option<ItemsLimitConfiguration>,
    #[serde(rename = "ColorSort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_sort: Option<Vec<FieldSortOptions>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomContentVisual {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ChartConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_configuration: Option<CustomContentConfiguration>,
    #[serde(rename = "DataSetIdentifier")]
    #[serde(default)]
    pub data_set_identifier: String,
    #[serde(rename = "Subtitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "VisualContentAltText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_content_alt_text: Option<String>,
    #[serde(rename = "VisualId")]
    #[serde(default)]
    pub visual_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomContentConfiguration {
    #[serde(rename = "ContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "ContentUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_url: Option<String>,
    #[serde(rename = "ImageScaling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_scaling: Option<String>,
    #[serde(rename = "Interactions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions: Option<VisualInteractionOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EmptyVisual {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "DataSetIdentifier")]
    #[serde(default)]
    pub data_set_identifier: String,
    #[serde(rename = "VisualId")]
    #[serde(default)]
    pub visual_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilledMapVisual {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ChartConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_configuration: Option<FilledMapConfiguration>,
    #[serde(rename = "ColumnHierarchies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "ConditionalFormatting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_formatting: Option<FilledMapConditionalFormatting>,
    #[serde(rename = "GeocodingPreferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geocoding_preferences: Option<Vec<GeocodePreference>>,
    #[serde(rename = "Subtitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "VisualContentAltText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_content_alt_text: Option<String>,
    #[serde(rename = "VisualId")]
    #[serde(default)]
    pub visual_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilledMapConfiguration {
    #[serde(rename = "FieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_wells: Option<FilledMapFieldWells>,
    #[serde(rename = "Interactions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions: Option<VisualInteractionOptions>,
    #[serde(rename = "Legend")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "MapStyleOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_style_options: Option<GeospatialMapStyleOptions>,
    #[serde(rename = "SortConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_configuration: Option<FilledMapSortConfiguration>,
    #[serde(rename = "Tooltip")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "WindowOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_options: Option<GeospatialWindowOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilledMapFieldWells {
    #[serde(rename = "FilledMapAggregatedFieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filled_map_aggregated_field_wells: Option<FilledMapAggregatedFieldWells>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilledMapAggregatedFieldWells {
    #[serde(rename = "Geospatial")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geospatial: Option<Vec<DimensionField>>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<MeasureField>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialMapStyleOptions {
    #[serde(rename = "BaseMapStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_map_style: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilledMapSortConfiguration {
    #[serde(rename = "CategorySort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_sort: Option<Vec<FieldSortOptions>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialWindowOptions {
    #[serde(rename = "Bounds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounds: Option<GeospatialCoordinateBounds>,
    #[serde(rename = "MapZoomMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_zoom_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialCoordinateBounds {
    #[serde(rename = "East")]
    #[serde(default)]
    pub east: f64,
    #[serde(rename = "North")]
    #[serde(default)]
    pub north: f64,
    #[serde(rename = "South")]
    #[serde(default)]
    pub south: f64,
    #[serde(rename = "West")]
    #[serde(default)]
    pub west: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilledMapConditionalFormatting {
    #[serde(rename = "ConditionalFormattingOptions")]
    #[serde(default)]
    pub conditional_formatting_options: Vec<FilledMapConditionalFormattingOption>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilledMapConditionalFormattingOption {
    #[serde(rename = "Shape")]
    #[serde(default)]
    pub shape: FilledMapShapeConditionalFormatting,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilledMapShapeConditionalFormatting {
    #[serde(rename = "FieldId")]
    #[serde(default)]
    pub field_id: String,
    #[serde(rename = "Format")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ShapeConditionalFormat>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ShapeConditionalFormat {
    #[serde(rename = "BackgroundColor")]
    #[serde(default)]
    pub background_color: ConditionalFormattingColor,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConditionalFormattingColor {
    #[serde(rename = "Gradient")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gradient: Option<ConditionalFormattingGradientColor>,
    #[serde(rename = "Solid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solid: Option<ConditionalFormattingSolidColor>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConditionalFormattingGradientColor {
    #[serde(rename = "Color")]
    #[serde(default)]
    pub color: GradientColor,
    #[serde(rename = "Expression")]
    #[serde(default)]
    pub expression: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GradientColor {
    #[serde(rename = "Stops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stops: Option<Vec<GradientStop>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GradientStop {
    #[serde(rename = "Color")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "DataValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_value: Option<f64>,
    #[serde(rename = "GradientOffset")]
    #[serde(default)]
    pub gradient_offset: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConditionalFormattingSolidColor {
    #[serde(rename = "Color")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "Expression")]
    #[serde(default)]
    pub expression: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeocodePreference {
    #[serde(rename = "Preference")]
    #[serde(default)]
    pub preference: GeocodePreferenceValue,
    #[serde(rename = "RequestKey")]
    #[serde(default)]
    pub request_key: GeocoderHierarchy,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeocodePreferenceValue {
    #[serde(rename = "Coordinate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinate: Option<Coordinate>,
    #[serde(rename = "GeocoderHierarchy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geocoder_hierarchy: Option<GeocoderHierarchy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Coordinate {
    #[serde(rename = "Latitude")]
    #[serde(default)]
    pub latitude: f64,
    #[serde(rename = "Longitude")]
    #[serde(default)]
    pub longitude: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeocoderHierarchy {
    #[serde(rename = "City")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "Country")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "County")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub county: Option<String>,
    #[serde(rename = "PostCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FunnelChartVisual {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ChartConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_configuration: Option<FunnelChartConfiguration>,
    #[serde(rename = "ColumnHierarchies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "Subtitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "VisualContentAltText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_content_alt_text: Option<String>,
    #[serde(rename = "VisualId")]
    #[serde(default)]
    pub visual_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FunnelChartConfiguration {
    #[serde(rename = "CategoryLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "DataLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_label_options: Option<FunnelChartDataLabelOptions>,
    #[serde(rename = "FieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_wells: Option<FunnelChartFieldWells>,
    #[serde(rename = "Interactions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions: Option<VisualInteractionOptions>,
    #[serde(rename = "SortConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_configuration: Option<FunnelChartSortConfiguration>,
    #[serde(rename = "Tooltip")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "ValueLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "VisualPalette")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_palette: Option<VisualPalette>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FunnelChartDataLabelOptions {
    #[serde(rename = "CategoryLabelVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_label_visibility: Option<String>,
    #[serde(rename = "LabelColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_color: Option<String>,
    #[serde(rename = "LabelFontConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_font_configuration: Option<FontConfiguration>,
    #[serde(rename = "MeasureDataLabelStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measure_data_label_style: Option<String>,
    #[serde(rename = "MeasureLabelVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measure_label_visibility: Option<String>,
    #[serde(rename = "Position")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FunnelChartFieldWells {
    #[serde(rename = "FunnelChartAggregatedFieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funnel_chart_aggregated_field_wells: Option<FunnelChartAggregatedFieldWells>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FunnelChartAggregatedFieldWells {
    #[serde(rename = "Category")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Vec<DimensionField>>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<MeasureField>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FunnelChartSortConfiguration {
    #[serde(rename = "CategoryItemsLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_items_limit: Option<ItemsLimitConfiguration>,
    #[serde(rename = "CategorySort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_sort: Option<Vec<FieldSortOptions>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GaugeChartVisual {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ChartConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_configuration: Option<GaugeChartConfiguration>,
    #[serde(rename = "ConditionalFormatting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_formatting: Option<GaugeChartConditionalFormatting>,
    #[serde(rename = "Subtitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "VisualContentAltText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_content_alt_text: Option<String>,
    #[serde(rename = "VisualId")]
    #[serde(default)]
    pub visual_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GaugeChartConfiguration {
    #[serde(rename = "ColorConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_configuration: Option<GaugeChartColorConfiguration>,
    #[serde(rename = "DataLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_labels: Option<DataLabelOptions>,
    #[serde(rename = "FieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_wells: Option<GaugeChartFieldWells>,
    #[serde(rename = "GaugeChartOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gauge_chart_options: Option<GaugeChartOptions>,
    #[serde(rename = "Interactions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions: Option<VisualInteractionOptions>,
    #[serde(rename = "TooltipOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip_options: Option<TooltipOptions>,
    #[serde(rename = "VisualPalette")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_palette: Option<VisualPalette>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GaugeChartColorConfiguration {
    #[serde(rename = "BackgroundColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(rename = "ForegroundColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreground_color: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GaugeChartFieldWells {
    #[serde(rename = "TargetValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_values: Option<Vec<MeasureField>>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<MeasureField>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GaugeChartOptions {
    #[serde(rename = "Arc")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arc: Option<ArcConfiguration>,
    #[serde(rename = "ArcAxis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arc_axis: Option<ArcAxisConfiguration>,
    #[serde(rename = "Comparison")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison: Option<ComparisonConfiguration>,
    #[serde(rename = "PrimaryValueDisplayType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_value_display_type: Option<String>,
    #[serde(rename = "PrimaryValueFontConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_value_font_configuration: Option<FontConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ArcConfiguration {
    #[serde(rename = "ArcAngle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arc_angle: Option<f64>,
    #[serde(rename = "ArcThickness")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arc_thickness: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ArcAxisConfiguration {
    #[serde(rename = "Range")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<ArcAxisDisplayRange>,
    #[serde(rename = "ReserveRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserve_range: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ArcAxisDisplayRange {
    #[serde(rename = "Max")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(rename = "Min")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComparisonConfiguration {
    #[serde(rename = "ComparisonFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_format: Option<ComparisonFormatConfiguration>,
    #[serde(rename = "ComparisonMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_method: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComparisonFormatConfiguration {
    #[serde(rename = "NumberDisplayFormatConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_display_format_configuration: Option<NumberDisplayFormatConfiguration>,
    #[serde(rename = "PercentageDisplayFormatConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage_display_format_configuration: Option<PercentageDisplayFormatConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GaugeChartConditionalFormatting {
    #[serde(rename = "ConditionalFormattingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_formatting_options: Option<Vec<GaugeChartConditionalFormattingOption>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GaugeChartConditionalFormattingOption {
    #[serde(rename = "Arc")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arc: Option<GaugeChartArcConditionalFormatting>,
    #[serde(rename = "PrimaryValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_value: Option<GaugeChartPrimaryValueConditionalFormatting>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GaugeChartArcConditionalFormatting {
    #[serde(rename = "ForegroundColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreground_color: Option<ConditionalFormattingColor>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GaugeChartPrimaryValueConditionalFormatting {
    #[serde(rename = "Icon")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<ConditionalFormattingIcon>,
    #[serde(rename = "TextColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<ConditionalFormattingColor>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConditionalFormattingIcon {
    #[serde(rename = "CustomCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_condition: Option<ConditionalFormattingCustomIconCondition>,
    #[serde(rename = "IconSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_set: Option<ConditionalFormattingIconSet>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConditionalFormattingCustomIconCondition {
    #[serde(rename = "Color")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "DisplayConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_configuration: Option<ConditionalFormattingIconDisplayConfiguration>,
    #[serde(rename = "Expression")]
    #[serde(default)]
    pub expression: String,
    #[serde(rename = "IconOptions")]
    #[serde(default)]
    pub icon_options: ConditionalFormattingCustomIconOptions,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConditionalFormattingIconDisplayConfiguration {
    #[serde(rename = "IconDisplayOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_display_option: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConditionalFormattingCustomIconOptions {
    #[serde(rename = "Icon")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(rename = "UnicodeIcon")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unicode_icon: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConditionalFormattingIconSet {
    #[serde(rename = "Expression")]
    #[serde(default)]
    pub expression: String,
    #[serde(rename = "IconSetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_set_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialMapVisual {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ChartConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_configuration: Option<GeospatialMapConfiguration>,
    #[serde(rename = "ColumnHierarchies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "GeocodingPreferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geocoding_preferences: Option<Vec<GeocodePreference>>,
    #[serde(rename = "Subtitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "VisualContentAltText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_content_alt_text: Option<String>,
    #[serde(rename = "VisualId")]
    #[serde(default)]
    pub visual_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialMapConfiguration {
    #[serde(rename = "FieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_wells: Option<GeospatialMapFieldWells>,
    #[serde(rename = "Interactions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions: Option<VisualInteractionOptions>,
    #[serde(rename = "Legend")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "MapStyleOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_style_options: Option<GeospatialMapStyleOptions>,
    #[serde(rename = "PointStyleOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_style_options: Option<GeospatialPointStyleOptions>,
    #[serde(rename = "Tooltip")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "VisualPalette")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_palette: Option<VisualPalette>,
    #[serde(rename = "WindowOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_options: Option<GeospatialWindowOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialMapFieldWells {
    #[serde(rename = "GeospatialMapAggregatedFieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geospatial_map_aggregated_field_wells: Option<GeospatialMapAggregatedFieldWells>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialMapAggregatedFieldWells {
    #[serde(rename = "Colors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colors: Option<Vec<DimensionField>>,
    #[serde(rename = "Geospatial")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geospatial: Option<Vec<DimensionField>>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<MeasureField>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialPointStyleOptions {
    #[serde(rename = "ClusterMarkerConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_marker_configuration: Option<ClusterMarkerConfiguration>,
    #[serde(rename = "HeatmapConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heatmap_configuration: Option<GeospatialHeatmapConfiguration>,
    #[serde(rename = "SelectedPointStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_point_style: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterMarkerConfiguration {
    #[serde(rename = "ClusterMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_marker: Option<ClusterMarker>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterMarker {
    #[serde(rename = "SimpleClusterMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_cluster_marker: Option<SimpleClusterMarker>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SimpleClusterMarker {
    #[serde(rename = "Color")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialHeatmapConfiguration {
    #[serde(rename = "HeatmapColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heatmap_color: Option<GeospatialHeatmapColorScale>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialHeatmapColorScale {
    #[serde(rename = "Colors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colors: Option<Vec<GeospatialHeatmapDataColor>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialHeatmapDataColor {
    #[serde(rename = "Color")]
    #[serde(default)]
    pub color: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HeatMapVisual {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ChartConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_configuration: Option<HeatMapConfiguration>,
    #[serde(rename = "ColumnHierarchies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "Subtitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "VisualContentAltText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_content_alt_text: Option<String>,
    #[serde(rename = "VisualId")]
    #[serde(default)]
    pub visual_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HeatMapConfiguration {
    #[serde(rename = "ColorScale")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_scale: Option<ColorScale>,
    #[serde(rename = "ColumnAxisDisplayOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_axis_display_options: Option<AxisDisplayOptions>,
    #[serde(rename = "ColumnLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "DataLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_labels: Option<DataLabelOptions>,
    #[serde(rename = "FieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_wells: Option<HeatMapFieldWells>,
    #[serde(rename = "Interactions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions: Option<VisualInteractionOptions>,
    #[serde(rename = "Legend")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "RowAxisDisplayOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_axis_display_options: Option<AxisDisplayOptions>,
    #[serde(rename = "RowLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "SortConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_configuration: Option<HeatMapSortConfiguration>,
    #[serde(rename = "Tooltip")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<TooltipOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ColorScale {
    #[serde(rename = "ColorFillType")]
    #[serde(default)]
    pub color_fill_type: String,
    #[serde(rename = "Colors")]
    #[serde(default)]
    pub colors: Vec<DataColor>,
    #[serde(rename = "NullValueColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub null_value_color: Option<DataColor>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataColor {
    #[serde(rename = "Color")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "DataValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_value: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HeatMapFieldWells {
    #[serde(rename = "HeatMapAggregatedFieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heat_map_aggregated_field_wells: Option<HeatMapAggregatedFieldWells>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HeatMapAggregatedFieldWells {
    #[serde(rename = "Columns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<DimensionField>>,
    #[serde(rename = "Rows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<DimensionField>>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<MeasureField>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HeatMapSortConfiguration {
    #[serde(rename = "HeatMapColumnItemsLimitConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heat_map_column_items_limit_configuration: Option<ItemsLimitConfiguration>,
    #[serde(rename = "HeatMapColumnSort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heat_map_column_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "HeatMapRowItemsLimitConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heat_map_row_items_limit_configuration: Option<ItemsLimitConfiguration>,
    #[serde(rename = "HeatMapRowSort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heat_map_row_sort: Option<Vec<FieldSortOptions>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HistogramVisual {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ChartConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_configuration: Option<HistogramConfiguration>,
    #[serde(rename = "Subtitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "VisualContentAltText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_content_alt_text: Option<String>,
    #[serde(rename = "VisualId")]
    #[serde(default)]
    pub visual_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HistogramConfiguration {
    #[serde(rename = "BinOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_options: Option<HistogramBinOptions>,
    #[serde(rename = "DataLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_labels: Option<DataLabelOptions>,
    #[serde(rename = "FieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_wells: Option<HistogramFieldWells>,
    #[serde(rename = "Interactions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions: Option<VisualInteractionOptions>,
    #[serde(rename = "Tooltip")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "VisualPalette")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_palette: Option<VisualPalette>,
    #[serde(rename = "XAxisDisplayOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_axis_display_options: Option<AxisDisplayOptions>,
    #[serde(rename = "XAxisLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_axis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "YAxisDisplayOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_axis_display_options: Option<AxisDisplayOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HistogramBinOptions {
    #[serde(rename = "BinCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_count: Option<BinCountOptions>,
    #[serde(rename = "BinWidth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_width: Option<BinWidthOptions>,
    #[serde(rename = "SelectedBinType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_bin_type: Option<String>,
    #[serde(rename = "StartValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_value: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BinCountOptions {
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BinWidthOptions {
    #[serde(rename = "BinCountLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_count_limit: Option<i64>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HistogramFieldWells {
    #[serde(rename = "HistogramAggregatedFieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub histogram_aggregated_field_wells: Option<HistogramAggregatedFieldWells>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HistogramAggregatedFieldWells {
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<MeasureField>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InsightVisual {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "DataSetIdentifier")]
    #[serde(default)]
    pub data_set_identifier: String,
    #[serde(rename = "InsightConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_configuration: Option<InsightConfiguration>,
    #[serde(rename = "Subtitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "VisualContentAltText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_content_alt_text: Option<String>,
    #[serde(rename = "VisualId")]
    #[serde(default)]
    pub visual_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InsightConfiguration {
    #[serde(rename = "Computations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computations: Option<Vec<Computation>>,
    #[serde(rename = "CustomNarrative")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_narrative: Option<CustomNarrativeOptions>,
    #[serde(rename = "Interactions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions: Option<VisualInteractionOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Computation {
    #[serde(rename = "Forecast")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast: Option<ForecastComputation>,
    #[serde(rename = "GrowthRate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub growth_rate: Option<GrowthRateComputation>,
    #[serde(rename = "MaximumMinimum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_minimum: Option<MaximumMinimumComputation>,
    #[serde(rename = "MetricComparison")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_comparison: Option<MetricComparisonComputation>,
    #[serde(rename = "PeriodOverPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_over_period: Option<PeriodOverPeriodComputation>,
    #[serde(rename = "PeriodToDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_to_date: Option<PeriodToDateComputation>,
    #[serde(rename = "TopBottomMovers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_bottom_movers: Option<TopBottomMoversComputation>,
    #[serde(rename = "TopBottomRanked")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_bottom_ranked: Option<TopBottomRankedComputation>,
    #[serde(rename = "TotalAggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_aggregation: Option<TotalAggregationComputation>,
    #[serde(rename = "UniqueValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_values: Option<UniqueValuesComputation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ForecastComputation {
    #[serde(rename = "ComputationId")]
    #[serde(default)]
    pub computation_id: String,
    #[serde(rename = "CustomSeasonalityValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_seasonality_value: Option<i32>,
    #[serde(rename = "LowerBoundary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_boundary: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PeriodsBackward")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub periods_backward: Option<i32>,
    #[serde(rename = "PeriodsForward")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub periods_forward: Option<i32>,
    #[serde(rename = "PredictionInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prediction_interval: Option<i32>,
    #[serde(rename = "Seasonality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seasonality: Option<String>,
    #[serde(rename = "Time")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<DimensionField>,
    #[serde(rename = "UpperBoundary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_boundary: Option<f64>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<MeasureField>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GrowthRateComputation {
    #[serde(rename = "ComputationId")]
    #[serde(default)]
    pub computation_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PeriodSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_size: Option<i32>,
    #[serde(rename = "Time")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<DimensionField>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<MeasureField>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MaximumMinimumComputation {
    #[serde(rename = "ComputationId")]
    #[serde(default)]
    pub computation_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Time")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<DimensionField>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<MeasureField>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricComparisonComputation {
    #[serde(rename = "ComputationId")]
    #[serde(default)]
    pub computation_id: String,
    #[serde(rename = "FromValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_value: Option<MeasureField>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "TargetValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_value: Option<MeasureField>,
    #[serde(rename = "Time")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<DimensionField>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PeriodOverPeriodComputation {
    #[serde(rename = "ComputationId")]
    #[serde(default)]
    pub computation_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Time")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<DimensionField>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<MeasureField>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PeriodToDateComputation {
    #[serde(rename = "ComputationId")]
    #[serde(default)]
    pub computation_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PeriodTimeGranularity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_time_granularity: Option<String>,
    #[serde(rename = "Time")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<DimensionField>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<MeasureField>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopBottomMoversComputation {
    #[serde(rename = "Category")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<DimensionField>,
    #[serde(rename = "ComputationId")]
    #[serde(default)]
    pub computation_id: String,
    #[serde(rename = "MoverSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mover_size: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    #[serde(rename = "Time")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<DimensionField>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<MeasureField>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopBottomRankedComputation {
    #[serde(rename = "Category")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<DimensionField>,
    #[serde(rename = "ComputationId")]
    #[serde(default)]
    pub computation_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ResultSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_size: Option<i32>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<MeasureField>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TotalAggregationComputation {
    #[serde(rename = "ComputationId")]
    #[serde(default)]
    pub computation_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<MeasureField>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UniqueValuesComputation {
    #[serde(rename = "Category")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<DimensionField>,
    #[serde(rename = "ComputationId")]
    #[serde(default)]
    pub computation_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomNarrativeOptions {
    #[serde(rename = "Narrative")]
    #[serde(default)]
    pub narrative: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KPIVisual {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ChartConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_configuration: Option<KPIConfiguration>,
    #[serde(rename = "ColumnHierarchies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "ConditionalFormatting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_formatting: Option<KPIConditionalFormatting>,
    #[serde(rename = "Subtitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "VisualContentAltText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_content_alt_text: Option<String>,
    #[serde(rename = "VisualId")]
    #[serde(default)]
    pub visual_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KPIConfiguration {
    #[serde(rename = "FieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_wells: Option<KPIFieldWells>,
    #[serde(rename = "Interactions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions: Option<VisualInteractionOptions>,
    #[serde(rename = "KPIOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_p_i_options: Option<KPIOptions>,
    #[serde(rename = "SortConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_configuration: Option<KPISortConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KPIFieldWells {
    #[serde(rename = "TargetValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_values: Option<Vec<MeasureField>>,
    #[serde(rename = "TrendGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trend_groups: Option<Vec<DimensionField>>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<MeasureField>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KPIOptions {
    #[serde(rename = "Comparison")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison: Option<ComparisonConfiguration>,
    #[serde(rename = "PrimaryValueDisplayType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_value_display_type: Option<String>,
    #[serde(rename = "PrimaryValueFontConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_value_font_configuration: Option<FontConfiguration>,
    #[serde(rename = "ProgressBar")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_bar: Option<ProgressBarOptions>,
    #[serde(rename = "SecondaryValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_value: Option<SecondaryValueOptions>,
    #[serde(rename = "SecondaryValueFontConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_value_font_configuration: Option<FontConfiguration>,
    #[serde(rename = "Sparkline")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sparkline: Option<KPISparklineOptions>,
    #[serde(rename = "TrendArrows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trend_arrows: Option<TrendArrowOptions>,
    #[serde(rename = "VisualLayoutOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_layout_options: Option<KPIVisualLayoutOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProgressBarOptions {
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecondaryValueOptions {
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KPISparklineOptions {
    #[serde(rename = "Color")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "TooltipVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip_visibility: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrendArrowOptions {
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KPIVisualLayoutOptions {
    #[serde(rename = "StandardLayout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_layout: Option<KPIVisualStandardLayout>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KPIVisualStandardLayout {
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KPISortConfiguration {
    #[serde(rename = "TrendGroupSort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trend_group_sort: Option<Vec<FieldSortOptions>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KPIConditionalFormatting {
    #[serde(rename = "ConditionalFormattingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_formatting_options: Option<Vec<KPIConditionalFormattingOption>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KPIConditionalFormattingOption {
    #[serde(rename = "ActualValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_value: Option<KPIActualValueConditionalFormatting>,
    #[serde(rename = "ComparisonValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_value: Option<KPIComparisonValueConditionalFormatting>,
    #[serde(rename = "PrimaryValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_value: Option<KPIPrimaryValueConditionalFormatting>,
    #[serde(rename = "ProgressBar")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_bar: Option<KPIProgressBarConditionalFormatting>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KPIActualValueConditionalFormatting {
    #[serde(rename = "Icon")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<ConditionalFormattingIcon>,
    #[serde(rename = "TextColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<ConditionalFormattingColor>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KPIComparisonValueConditionalFormatting {
    #[serde(rename = "Icon")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<ConditionalFormattingIcon>,
    #[serde(rename = "TextColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<ConditionalFormattingColor>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KPIPrimaryValueConditionalFormatting {
    #[serde(rename = "Icon")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<ConditionalFormattingIcon>,
    #[serde(rename = "TextColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<ConditionalFormattingColor>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KPIProgressBarConditionalFormatting {
    #[serde(rename = "ForegroundColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreground_color: Option<ConditionalFormattingColor>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LayerMapVisual {
    #[serde(rename = "ChartConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_configuration: Option<GeospatialLayerMapConfiguration>,
    #[serde(rename = "DataSetIdentifier")]
    #[serde(default)]
    pub data_set_identifier: String,
    #[serde(rename = "Subtitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "VisualContentAltText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_content_alt_text: Option<String>,
    #[serde(rename = "VisualId")]
    #[serde(default)]
    pub visual_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialLayerMapConfiguration {
    #[serde(rename = "Interactions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions: Option<VisualInteractionOptions>,
    #[serde(rename = "Legend")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "MapLayers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_layers: Option<Vec<GeospatialLayerItem>>,
    #[serde(rename = "MapState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_state: Option<GeospatialMapState>,
    #[serde(rename = "MapStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_style: Option<GeospatialMapStyle>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialLayerItem {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<LayerCustomAction>>,
    #[serde(rename = "DataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<GeospatialDataSourceItem>,
    #[serde(rename = "JoinDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_definition: Option<GeospatialLayerJoinDefinition>,
    #[serde(rename = "Label")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "LayerDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_definition: Option<GeospatialLayerDefinition>,
    #[serde(rename = "LayerId")]
    #[serde(default)]
    pub layer_id: String,
    #[serde(rename = "LayerType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_type: Option<String>,
    #[serde(rename = "Tooltip")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LayerCustomAction {
    #[serde(rename = "ActionOperations")]
    #[serde(default)]
    pub action_operations: Vec<LayerCustomActionOperation>,
    #[serde(rename = "CustomActionId")]
    #[serde(default)]
    pub custom_action_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Trigger")]
    #[serde(default)]
    pub trigger: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LayerCustomActionOperation {
    #[serde(rename = "FilterOperation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_operation: Option<CustomActionFilterOperation>,
    #[serde(rename = "NavigationOperation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub navigation_operation: Option<CustomActionNavigationOperation>,
    #[serde(rename = "SetParametersOperation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_parameters_operation: Option<CustomActionSetParametersOperation>,
    #[serde(rename = "URLOperation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_r_l_operation: Option<CustomActionURLOperation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialDataSourceItem {
    #[serde(rename = "StaticFileDataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_file_data_source: Option<GeospatialStaticFileSource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialStaticFileSource {
    #[serde(rename = "StaticFileId")]
    #[serde(default)]
    pub static_file_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialLayerJoinDefinition {
    #[serde(rename = "ColorField")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_field: Option<GeospatialLayerColorField>,
    #[serde(rename = "DatasetKeyField")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_key_field: Option<UnaggregatedField>,
    #[serde(rename = "ShapeKeyField")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shape_key_field: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialLayerColorField {
    #[serde(rename = "ColorDimensionsFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_dimensions_fields: Option<Vec<DimensionField>>,
    #[serde(rename = "ColorValuesFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_values_fields: Option<Vec<MeasureField>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnaggregatedField {
    #[serde(rename = "Column")]
    #[serde(default)]
    pub column: ColumnIdentifier,
    #[serde(rename = "FieldId")]
    #[serde(default)]
    pub field_id: String,
    #[serde(rename = "FormatConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_configuration: Option<FormatConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialLayerDefinition {
    #[serde(rename = "LineLayer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_layer: Option<GeospatialLineLayer>,
    #[serde(rename = "PointLayer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_layer: Option<GeospatialPointLayer>,
    #[serde(rename = "PolygonLayer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polygon_layer: Option<GeospatialPolygonLayer>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialLineLayer {
    #[serde(rename = "Style")]
    #[serde(default)]
    pub style: GeospatialLineStyle,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialLineStyle {
    #[serde(rename = "LineSymbolStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_symbol_style: Option<GeospatialLineSymbolStyle>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialLineSymbolStyle {
    #[serde(rename = "FillColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_color: Option<GeospatialColor>,
    #[serde(rename = "LineWidth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_width: Option<GeospatialLineWidth>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialColor {
    #[serde(rename = "Categorical")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categorical: Option<GeospatialCategoricalColor>,
    #[serde(rename = "Gradient")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gradient: Option<GeospatialGradientColor>,
    #[serde(rename = "Solid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solid: Option<GeospatialSolidColor>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialCategoricalColor {
    #[serde(rename = "CategoryDataColors")]
    #[serde(default)]
    pub category_data_colors: Vec<GeospatialCategoricalDataColor>,
    #[serde(rename = "DefaultOpacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_opacity: Option<f64>,
    #[serde(rename = "NullDataSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub null_data_settings: Option<GeospatialNullDataSettings>,
    #[serde(rename = "NullDataVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub null_data_visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialCategoricalDataColor {
    #[serde(rename = "Color")]
    #[serde(default)]
    pub color: String,
    #[serde(rename = "DataValue")]
    #[serde(default)]
    pub data_value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialNullDataSettings {
    #[serde(rename = "SymbolStyle")]
    #[serde(default)]
    pub symbol_style: GeospatialNullSymbolStyle,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialNullSymbolStyle {
    #[serde(rename = "FillColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_color: Option<String>,
    #[serde(rename = "StrokeColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_color: Option<String>,
    #[serde(rename = "StrokeWidth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_width: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialGradientColor {
    #[serde(rename = "DefaultOpacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_opacity: Option<f64>,
    #[serde(rename = "NullDataSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub null_data_settings: Option<GeospatialNullDataSettings>,
    #[serde(rename = "NullDataVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub null_data_visibility: Option<String>,
    #[serde(rename = "StepColors")]
    #[serde(default)]
    pub step_colors: Vec<GeospatialGradientStepColor>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialGradientStepColor {
    #[serde(rename = "Color")]
    #[serde(default)]
    pub color: String,
    #[serde(rename = "DataValue")]
    #[serde(default)]
    pub data_value: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialSolidColor {
    #[serde(rename = "Color")]
    #[serde(default)]
    pub color: String,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialLineWidth {
    #[serde(rename = "LineWidth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_width: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialPointLayer {
    #[serde(rename = "Style")]
    #[serde(default)]
    pub style: GeospatialPointStyle,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialPointStyle {
    #[serde(rename = "CircleSymbolStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_symbol_style: Option<GeospatialCircleSymbolStyle>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialCircleSymbolStyle {
    #[serde(rename = "CircleRadius")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_radius: Option<GeospatialCircleRadius>,
    #[serde(rename = "FillColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_color: Option<GeospatialColor>,
    #[serde(rename = "StrokeColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_color: Option<GeospatialColor>,
    #[serde(rename = "StrokeWidth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_width: Option<GeospatialLineWidth>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialCircleRadius {
    #[serde(rename = "Radius")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialPolygonLayer {
    #[serde(rename = "Style")]
    #[serde(default)]
    pub style: GeospatialPolygonStyle,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialPolygonStyle {
    #[serde(rename = "PolygonSymbolStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polygon_symbol_style: Option<GeospatialPolygonSymbolStyle>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialPolygonSymbolStyle {
    #[serde(rename = "FillColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_color: Option<GeospatialColor>,
    #[serde(rename = "StrokeColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_color: Option<GeospatialColor>,
    #[serde(rename = "StrokeWidth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_width: Option<GeospatialLineWidth>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialMapState {
    #[serde(rename = "Bounds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounds: Option<GeospatialCoordinateBounds>,
    #[serde(rename = "MapNavigation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_navigation: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeospatialMapStyle {
    #[serde(rename = "BackgroundColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(rename = "BaseMapStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_map_style: Option<String>,
    #[serde(rename = "BaseMapVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_map_visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LineChartVisual {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ChartConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_configuration: Option<LineChartConfiguration>,
    #[serde(rename = "ColumnHierarchies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "Subtitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "VisualContentAltText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_content_alt_text: Option<String>,
    #[serde(rename = "VisualId")]
    #[serde(default)]
    pub visual_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LineChartConfiguration {
    #[serde(rename = "ContributionAnalysisDefaults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contribution_analysis_defaults: Option<Vec<ContributionAnalysisDefault>>,
    #[serde(rename = "DataLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_labels: Option<DataLabelOptions>,
    #[serde(rename = "DefaultSeriesSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_series_settings: Option<LineChartDefaultSeriesSettings>,
    #[serde(rename = "FieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_wells: Option<LineChartFieldWells>,
    #[serde(rename = "ForecastConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_configurations: Option<Vec<ForecastConfiguration>>,
    #[serde(rename = "Interactions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions: Option<VisualInteractionOptions>,
    #[serde(rename = "Legend")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "PrimaryYAxisDisplayOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_y_axis_display_options: Option<LineSeriesAxisDisplayOptions>,
    #[serde(rename = "PrimaryYAxisLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_y_axis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "ReferenceLines")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_lines: Option<Vec<ReferenceLine>>,
    #[serde(rename = "SecondaryYAxisDisplayOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_y_axis_display_options: Option<LineSeriesAxisDisplayOptions>,
    #[serde(rename = "SecondaryYAxisLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_y_axis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "Series")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub series: Option<Vec<SeriesItem>>,
    #[serde(rename = "SingleAxisOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_axis_options: Option<SingleAxisOptions>,
    #[serde(rename = "SmallMultiplesOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_multiples_options: Option<SmallMultiplesOptions>,
    #[serde(rename = "SortConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_configuration: Option<LineChartSortConfiguration>,
    #[serde(rename = "Tooltip")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "VisualPalette")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_palette: Option<VisualPalette>,
    #[serde(rename = "XAxisDisplayOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_axis_display_options: Option<AxisDisplayOptions>,
    #[serde(rename = "XAxisLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_axis_label_options: Option<ChartAxisLabelOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LineChartDefaultSeriesSettings {
    #[serde(rename = "AxisBinding")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis_binding: Option<String>,
    #[serde(rename = "DecalSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decal_settings: Option<DecalSettings>,
    #[serde(rename = "LineStyleSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_style_settings: Option<LineChartLineStyleSettings>,
    #[serde(rename = "MarkerStyleSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker_style_settings: Option<LineChartMarkerStyleSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LineChartFieldWells {
    #[serde(rename = "LineChartAggregatedFieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_chart_aggregated_field_wells: Option<LineChartAggregatedFieldWells>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LineChartAggregatedFieldWells {
    #[serde(rename = "Category")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Vec<DimensionField>>,
    #[serde(rename = "Colors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colors: Option<Vec<DimensionField>>,
    #[serde(rename = "SmallMultiples")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_multiples: Option<Vec<DimensionField>>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<MeasureField>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ForecastConfiguration {
    #[serde(rename = "ForecastProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_properties: Option<TimeBasedForecastProperties>,
    #[serde(rename = "Scenario")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<ForecastScenario>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimeBasedForecastProperties {
    #[serde(rename = "LowerBoundary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lower_boundary: Option<f64>,
    #[serde(rename = "PeriodsBackward")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub periods_backward: Option<i32>,
    #[serde(rename = "PeriodsForward")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub periods_forward: Option<i32>,
    #[serde(rename = "PredictionInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prediction_interval: Option<i32>,
    #[serde(rename = "Seasonality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seasonality: Option<i32>,
    #[serde(rename = "UpperBoundary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper_boundary: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ForecastScenario {
    #[serde(rename = "WhatIfPointScenario")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub what_if_point_scenario: Option<WhatIfPointScenario>,
    #[serde(rename = "WhatIfRangeScenario")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub what_if_range_scenario: Option<WhatIfRangeScenario>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WhatIfPointScenario {
    #[serde(rename = "Date")]
    #[serde(default)]
    pub date: f64,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WhatIfRangeScenario {
    #[serde(rename = "EndDate")]
    #[serde(default)]
    pub end_date: f64,
    #[serde(rename = "StartDate")]
    #[serde(default)]
    pub start_date: f64,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LineSeriesAxisDisplayOptions {
    #[serde(rename = "AxisOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis_options: Option<AxisDisplayOptions>,
    #[serde(rename = "MissingDataConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_data_configurations: Option<Vec<MissingDataConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MissingDataConfiguration {
    #[serde(rename = "TreatmentOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_option: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SeriesItem {
    #[serde(rename = "DataFieldSeriesItem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_field_series_item: Option<DataFieldSeriesItem>,
    #[serde(rename = "FieldSeriesItem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_series_item: Option<FieldSeriesItem>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataFieldSeriesItem {
    #[serde(rename = "AxisBinding")]
    #[serde(default)]
    pub axis_binding: String,
    #[serde(rename = "FieldId")]
    #[serde(default)]
    pub field_id: String,
    #[serde(rename = "FieldValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_value: Option<String>,
    #[serde(rename = "Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<LineChartSeriesSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LineChartSeriesSettings {
    #[serde(rename = "DecalSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decal_settings: Option<DecalSettings>,
    #[serde(rename = "LineStyleSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_style_settings: Option<LineChartLineStyleSettings>,
    #[serde(rename = "MarkerStyleSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker_style_settings: Option<LineChartMarkerStyleSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FieldSeriesItem {
    #[serde(rename = "AxisBinding")]
    #[serde(default)]
    pub axis_binding: String,
    #[serde(rename = "FieldId")]
    #[serde(default)]
    pub field_id: String,
    #[serde(rename = "Settings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<LineChartSeriesSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LineChartSortConfiguration {
    #[serde(rename = "CategoryItemsLimitConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_items_limit_configuration: Option<ItemsLimitConfiguration>,
    #[serde(rename = "CategorySort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "ColorItemsLimitConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_items_limit_configuration: Option<ItemsLimitConfiguration>,
    #[serde(rename = "SmallMultiplesLimitConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_multiples_limit_configuration: Option<ItemsLimitConfiguration>,
    #[serde(rename = "SmallMultiplesSort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_multiples_sort: Option<Vec<FieldSortOptions>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PieChartVisual {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ChartConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_configuration: Option<PieChartConfiguration>,
    #[serde(rename = "ColumnHierarchies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "Subtitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "VisualContentAltText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_content_alt_text: Option<String>,
    #[serde(rename = "VisualId")]
    #[serde(default)]
    pub visual_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PieChartConfiguration {
    #[serde(rename = "CategoryLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "ContributionAnalysisDefaults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contribution_analysis_defaults: Option<Vec<ContributionAnalysisDefault>>,
    #[serde(rename = "DataLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_labels: Option<DataLabelOptions>,
    #[serde(rename = "DonutOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub donut_options: Option<DonutOptions>,
    #[serde(rename = "FieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_wells: Option<PieChartFieldWells>,
    #[serde(rename = "Interactions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions: Option<VisualInteractionOptions>,
    #[serde(rename = "Legend")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "SmallMultiplesOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_multiples_options: Option<SmallMultiplesOptions>,
    #[serde(rename = "SortConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_configuration: Option<PieChartSortConfiguration>,
    #[serde(rename = "Tooltip")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "ValueLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "VisualPalette")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_palette: Option<VisualPalette>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DonutOptions {
    #[serde(rename = "ArcOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arc_options: Option<ArcOptions>,
    #[serde(rename = "DonutCenterOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub donut_center_options: Option<DonutCenterOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ArcOptions {
    #[serde(rename = "ArcThickness")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arc_thickness: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DonutCenterOptions {
    #[serde(rename = "LabelVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PieChartFieldWells {
    #[serde(rename = "PieChartAggregatedFieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pie_chart_aggregated_field_wells: Option<PieChartAggregatedFieldWells>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PieChartAggregatedFieldWells {
    #[serde(rename = "Category")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Vec<DimensionField>>,
    #[serde(rename = "SmallMultiples")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_multiples: Option<Vec<DimensionField>>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<MeasureField>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PieChartSortConfiguration {
    #[serde(rename = "CategoryItemsLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_items_limit: Option<ItemsLimitConfiguration>,
    #[serde(rename = "CategorySort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "SmallMultiplesLimitConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_multiples_limit_configuration: Option<ItemsLimitConfiguration>,
    #[serde(rename = "SmallMultiplesSort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_multiples_sort: Option<Vec<FieldSortOptions>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PivotTableVisual {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ChartConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_configuration: Option<PivotTableConfiguration>,
    #[serde(rename = "ConditionalFormatting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_formatting: Option<PivotTableConditionalFormatting>,
    #[serde(rename = "Subtitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "VisualContentAltText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_content_alt_text: Option<String>,
    #[serde(rename = "VisualId")]
    #[serde(default)]
    pub visual_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PivotTableConfiguration {
    #[serde(rename = "DashboardCustomizationVisualOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_customization_visual_options: Option<DashboardCustomizationVisualOptions>,
    #[serde(rename = "FieldOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_options: Option<PivotTableFieldOptions>,
    #[serde(rename = "FieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_wells: Option<PivotTableFieldWells>,
    #[serde(rename = "Interactions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions: Option<VisualInteractionOptions>,
    #[serde(rename = "PaginatedReportOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paginated_report_options: Option<PivotTablePaginatedReportOptions>,
    #[serde(rename = "SortConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_configuration: Option<PivotTableSortConfiguration>,
    #[serde(rename = "TableOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_options: Option<PivotTableOptions>,
    #[serde(rename = "Tooltip")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "TotalOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_options: Option<PivotTableTotalOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DashboardCustomizationVisualOptions {
    #[serde(rename = "FieldsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields_configuration: Option<VisualCustomizationFieldsConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VisualCustomizationFieldsConfiguration {
    #[serde(rename = "AdditionalFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_fields: Option<Vec<ColumnIdentifier>>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PivotTableFieldOptions {
    #[serde(rename = "CollapseStateOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collapse_state_options: Option<Vec<PivotTableFieldCollapseStateOption>>,
    #[serde(rename = "DataPathOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_path_options: Option<Vec<PivotTableDataPathOption>>,
    #[serde(rename = "SelectedFieldOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_field_options: Option<Vec<PivotTableFieldOption>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PivotTableFieldCollapseStateOption {
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "Target")]
    #[serde(default)]
    pub target: PivotTableFieldCollapseStateTarget,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PivotTableFieldCollapseStateTarget {
    #[serde(rename = "FieldDataPathValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_data_path_values: Option<Vec<DataPathValue>>,
    #[serde(rename = "FieldId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PivotTableDataPathOption {
    #[serde(rename = "DataPathList")]
    #[serde(default)]
    pub data_path_list: Vec<DataPathValue>,
    #[serde(rename = "Width")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PivotTableFieldOption {
    #[serde(rename = "CustomLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_label: Option<String>,
    #[serde(rename = "FieldId")]
    #[serde(default)]
    pub field_id: String,
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PivotTableFieldWells {
    #[serde(rename = "PivotTableAggregatedFieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pivot_table_aggregated_field_wells: Option<PivotTableAggregatedFieldWells>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PivotTableAggregatedFieldWells {
    #[serde(rename = "Columns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<DimensionField>>,
    #[serde(rename = "Rows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<DimensionField>>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<MeasureField>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PivotTablePaginatedReportOptions {
    #[serde(rename = "OverflowColumnHeaderVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overflow_column_header_visibility: Option<String>,
    #[serde(rename = "VerticalOverflowVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_overflow_visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PivotTableSortConfiguration {
    #[serde(rename = "FieldSortOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_sort_options: Option<Vec<PivotFieldSortOptions>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PivotFieldSortOptions {
    #[serde(rename = "FieldId")]
    #[serde(default)]
    pub field_id: String,
    #[serde(rename = "SortBy")]
    #[serde(default)]
    pub sort_by: PivotTableSortBy,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PivotTableSortBy {
    #[serde(rename = "Column")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<ColumnSort>,
    #[serde(rename = "DataPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_path: Option<DataPathSort>,
    #[serde(rename = "Field")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<FieldSort>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataPathSort {
    #[serde(rename = "Direction")]
    #[serde(default)]
    pub direction: String,
    #[serde(rename = "SortPaths")]
    #[serde(default)]
    pub sort_paths: Vec<DataPathValue>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PivotTableOptions {
    #[serde(rename = "CellStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell_style: Option<TableCellStyle>,
    #[serde(rename = "CollapsedRowDimensionsVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collapsed_row_dimensions_visibility: Option<String>,
    #[serde(rename = "ColumnHeaderStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_header_style: Option<TableCellStyle>,
    #[serde(rename = "ColumnNamesVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_names_visibility: Option<String>,
    #[serde(rename = "DefaultCellWidth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_cell_width: Option<String>,
    #[serde(rename = "MetricPlacement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_placement: Option<String>,
    #[serde(rename = "RowAlternateColorOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_alternate_color_options: Option<RowAlternateColorOptions>,
    #[serde(rename = "RowFieldNamesStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_field_names_style: Option<TableCellStyle>,
    #[serde(rename = "RowHeaderStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_header_style: Option<TableCellStyle>,
    #[serde(rename = "RowsLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows_label_options: Option<PivotTableRowsLabelOptions>,
    #[serde(rename = "RowsLayout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows_layout: Option<String>,
    #[serde(rename = "SingleMetricVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_metric_visibility: Option<String>,
    #[serde(rename = "ToggleButtonsVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toggle_buttons_visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableCellStyle {
    #[serde(rename = "BackgroundColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(rename = "Border")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border: Option<GlobalTableBorderOptions>,
    #[serde(rename = "FontConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_configuration: Option<FontConfiguration>,
    #[serde(rename = "Height")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(rename = "HorizontalTextAlignment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_text_alignment: Option<String>,
    #[serde(rename = "TextWrap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_wrap: Option<String>,
    #[serde(rename = "VerticalTextAlignment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_text_alignment: Option<String>,
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlobalTableBorderOptions {
    #[serde(rename = "SideSpecificBorder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side_specific_border: Option<TableSideBorderOptions>,
    #[serde(rename = "UniformBorder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uniform_border: Option<TableBorderOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableSideBorderOptions {
    #[serde(rename = "Bottom")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bottom: Option<TableBorderOptions>,
    #[serde(rename = "InnerHorizontal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inner_horizontal: Option<TableBorderOptions>,
    #[serde(rename = "InnerVertical")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inner_vertical: Option<TableBorderOptions>,
    #[serde(rename = "Left")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<TableBorderOptions>,
    #[serde(rename = "Right")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right: Option<TableBorderOptions>,
    #[serde(rename = "Top")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<TableBorderOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableBorderOptions {
    #[serde(rename = "Color")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "Style")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(rename = "Thickness")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thickness: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RowAlternateColorOptions {
    #[serde(rename = "RowAlternateColors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_alternate_colors: Option<Vec<String>>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "UsePrimaryBackgroundColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_primary_background_color: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PivotTableRowsLabelOptions {
    #[serde(rename = "CustomLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_label: Option<String>,
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PivotTableTotalOptions {
    #[serde(rename = "ColumnSubtotalOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_subtotal_options: Option<SubtotalOptions>,
    #[serde(rename = "ColumnTotalOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_total_options: Option<PivotTotalOptions>,
    #[serde(rename = "RowSubtotalOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_subtotal_options: Option<SubtotalOptions>,
    #[serde(rename = "RowTotalOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_total_options: Option<PivotTotalOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubtotalOptions {
    #[serde(rename = "CustomLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_label: Option<String>,
    #[serde(rename = "FieldLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_level: Option<String>,
    #[serde(rename = "FieldLevelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_level_options: Option<Vec<PivotTableFieldSubtotalOptions>>,
    #[serde(rename = "MetricHeaderCellStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_header_cell_style: Option<TableCellStyle>,
    #[serde(rename = "StyleTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style_targets: Option<Vec<TableStyleTarget>>,
    #[serde(rename = "TotalCellStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_cell_style: Option<TableCellStyle>,
    #[serde(rename = "TotalsVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totals_visibility: Option<String>,
    #[serde(rename = "ValueCellStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_cell_style: Option<TableCellStyle>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PivotTableFieldSubtotalOptions {
    #[serde(rename = "FieldId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableStyleTarget {
    #[serde(rename = "CellType")]
    #[serde(default)]
    pub cell_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PivotTotalOptions {
    #[serde(rename = "CustomLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_label: Option<String>,
    #[serde(rename = "MetricHeaderCellStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_header_cell_style: Option<TableCellStyle>,
    #[serde(rename = "Placement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement: Option<String>,
    #[serde(rename = "ScrollStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scroll_status: Option<String>,
    #[serde(rename = "TotalAggregationOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_aggregation_options: Option<Vec<TotalAggregationOption>>,
    #[serde(rename = "TotalCellStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_cell_style: Option<TableCellStyle>,
    #[serde(rename = "TotalsVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totals_visibility: Option<String>,
    #[serde(rename = "ValueCellStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_cell_style: Option<TableCellStyle>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TotalAggregationOption {
    #[serde(rename = "FieldId")]
    #[serde(default)]
    pub field_id: String,
    #[serde(rename = "TotalAggregationFunction")]
    #[serde(default)]
    pub total_aggregation_function: TotalAggregationFunction,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TotalAggregationFunction {
    #[serde(rename = "SimpleTotalAggregationFunction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_total_aggregation_function: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PivotTableConditionalFormatting {
    #[serde(rename = "ConditionalFormattingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_formatting_options: Option<Vec<PivotTableConditionalFormattingOption>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PivotTableConditionalFormattingOption {
    #[serde(rename = "Cell")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell: Option<PivotTableCellConditionalFormatting>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PivotTableCellConditionalFormatting {
    #[serde(rename = "FieldId")]
    #[serde(default)]
    pub field_id: String,
    #[serde(rename = "Scope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<PivotTableConditionalFormattingScope>,
    #[serde(rename = "Scopes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<PivotTableConditionalFormattingScope>>,
    #[serde(rename = "TextFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_format: Option<TextConditionalFormat>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PivotTableConditionalFormattingScope {
    #[serde(rename = "Role")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TextConditionalFormat {
    #[serde(rename = "BackgroundColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<ConditionalFormattingColor>,
    #[serde(rename = "Icon")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<ConditionalFormattingIcon>,
    #[serde(rename = "TextColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<ConditionalFormattingColor>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PluginVisual {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ChartConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_configuration: Option<PluginVisualConfiguration>,
    #[serde(rename = "PluginArn")]
    #[serde(default)]
    pub plugin_arn: String,
    #[serde(rename = "Subtitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "VisualContentAltText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_content_alt_text: Option<String>,
    #[serde(rename = "VisualId")]
    #[serde(default)]
    pub visual_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PluginVisualConfiguration {
    #[serde(rename = "FieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_wells: Option<Vec<PluginVisualFieldWell>>,
    #[serde(rename = "SortConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_configuration: Option<PluginVisualSortConfiguration>,
    #[serde(rename = "VisualOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_options: Option<PluginVisualOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PluginVisualFieldWell {
    #[serde(rename = "AxisName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis_name: Option<String>,
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<DimensionField>>,
    #[serde(rename = "Measures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measures: Option<Vec<MeasureField>>,
    #[serde(rename = "Unaggregated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unaggregated: Option<Vec<UnaggregatedField>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PluginVisualSortConfiguration {
    #[serde(rename = "PluginVisualTableQuerySort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_visual_table_query_sort: Option<PluginVisualTableQuerySort>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PluginVisualTableQuerySort {
    #[serde(rename = "ItemsLimitConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items_limit_configuration: Option<PluginVisualItemsLimitConfiguration>,
    #[serde(rename = "RowSort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_sort: Option<Vec<FieldSortOptions>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PluginVisualItemsLimitConfiguration {
    #[serde(rename = "ItemsLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items_limit: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PluginVisualOptions {
    #[serde(rename = "VisualProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_properties: Option<Vec<PluginVisualProperty>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PluginVisualProperty {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RadarChartVisual {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ChartConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_configuration: Option<RadarChartConfiguration>,
    #[serde(rename = "ColumnHierarchies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "Subtitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "VisualContentAltText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_content_alt_text: Option<String>,
    #[serde(rename = "VisualId")]
    #[serde(default)]
    pub visual_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RadarChartConfiguration {
    #[serde(rename = "AlternateBandColorsVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate_band_colors_visibility: Option<String>,
    #[serde(rename = "AlternateBandEvenColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate_band_even_color: Option<String>,
    #[serde(rename = "AlternateBandOddColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate_band_odd_color: Option<String>,
    #[serde(rename = "AxesRangeScale")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axes_range_scale: Option<String>,
    #[serde(rename = "BaseSeriesSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_series_settings: Option<RadarChartSeriesSettings>,
    #[serde(rename = "CategoryAxis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_axis: Option<AxisDisplayOptions>,
    #[serde(rename = "CategoryLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "ColorAxis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_axis: Option<AxisDisplayOptions>,
    #[serde(rename = "ColorLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "FieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_wells: Option<RadarChartFieldWells>,
    #[serde(rename = "Interactions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions: Option<VisualInteractionOptions>,
    #[serde(rename = "Legend")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "Shape")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shape: Option<String>,
    #[serde(rename = "SortConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_configuration: Option<RadarChartSortConfiguration>,
    #[serde(rename = "StartAngle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_angle: Option<f64>,
    #[serde(rename = "VisualPalette")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_palette: Option<VisualPalette>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RadarChartSeriesSettings {
    #[serde(rename = "AreaStyleSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area_style_settings: Option<RadarChartAreaStyleSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RadarChartAreaStyleSettings {
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RadarChartFieldWells {
    #[serde(rename = "RadarChartAggregatedFieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_chart_aggregated_field_wells: Option<RadarChartAggregatedFieldWells>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RadarChartAggregatedFieldWells {
    #[serde(rename = "Category")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Vec<DimensionField>>,
    #[serde(rename = "Color")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Vec<DimensionField>>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<MeasureField>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RadarChartSortConfiguration {
    #[serde(rename = "CategoryItemsLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_items_limit: Option<ItemsLimitConfiguration>,
    #[serde(rename = "CategorySort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_sort: Option<Vec<FieldSortOptions>>,
    #[serde(rename = "ColorItemsLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_items_limit: Option<ItemsLimitConfiguration>,
    #[serde(rename = "ColorSort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_sort: Option<Vec<FieldSortOptions>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SankeyDiagramVisual {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ChartConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_configuration: Option<SankeyDiagramChartConfiguration>,
    #[serde(rename = "Subtitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "VisualContentAltText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_content_alt_text: Option<String>,
    #[serde(rename = "VisualId")]
    #[serde(default)]
    pub visual_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SankeyDiagramChartConfiguration {
    #[serde(rename = "DataLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_labels: Option<DataLabelOptions>,
    #[serde(rename = "FieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_wells: Option<SankeyDiagramFieldWells>,
    #[serde(rename = "Interactions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions: Option<VisualInteractionOptions>,
    #[serde(rename = "SortConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_configuration: Option<SankeyDiagramSortConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SankeyDiagramFieldWells {
    #[serde(rename = "SankeyDiagramAggregatedFieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sankey_diagram_aggregated_field_wells: Option<SankeyDiagramAggregatedFieldWells>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SankeyDiagramAggregatedFieldWells {
    #[serde(rename = "Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<Vec<DimensionField>>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Vec<DimensionField>>,
    #[serde(rename = "Weight")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<Vec<MeasureField>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SankeyDiagramSortConfiguration {
    #[serde(rename = "DestinationItemsLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_items_limit: Option<ItemsLimitConfiguration>,
    #[serde(rename = "SourceItemsLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_items_limit: Option<ItemsLimitConfiguration>,
    #[serde(rename = "WeightSort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_sort: Option<Vec<FieldSortOptions>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScatterPlotVisual {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ChartConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_configuration: Option<ScatterPlotConfiguration>,
    #[serde(rename = "ColumnHierarchies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "Subtitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "VisualContentAltText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_content_alt_text: Option<String>,
    #[serde(rename = "VisualId")]
    #[serde(default)]
    pub visual_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScatterPlotConfiguration {
    #[serde(rename = "DataLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_labels: Option<DataLabelOptions>,
    #[serde(rename = "FieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_wells: Option<ScatterPlotFieldWells>,
    #[serde(rename = "Interactions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions: Option<VisualInteractionOptions>,
    #[serde(rename = "Legend")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "SortConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_configuration: Option<ScatterPlotSortConfiguration>,
    #[serde(rename = "Tooltip")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "VisualPalette")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_palette: Option<VisualPalette>,
    #[serde(rename = "XAxisDisplayOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_axis_display_options: Option<AxisDisplayOptions>,
    #[serde(rename = "XAxisLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_axis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "YAxisDisplayOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_axis_display_options: Option<AxisDisplayOptions>,
    #[serde(rename = "YAxisLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_axis_label_options: Option<ChartAxisLabelOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScatterPlotFieldWells {
    #[serde(rename = "ScatterPlotCategoricallyAggregatedFieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scatter_plot_categorically_aggregated_field_wells:
        Option<ScatterPlotCategoricallyAggregatedFieldWells>,
    #[serde(rename = "ScatterPlotUnaggregatedFieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scatter_plot_unaggregated_field_wells: Option<ScatterPlotUnaggregatedFieldWells>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScatterPlotCategoricallyAggregatedFieldWells {
    #[serde(rename = "Category")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Vec<DimensionField>>,
    #[serde(rename = "Label")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<Vec<DimensionField>>,
    #[serde(rename = "Size")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<Vec<MeasureField>>,
    #[serde(rename = "XAxis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_axis: Option<Vec<MeasureField>>,
    #[serde(rename = "YAxis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_axis: Option<Vec<MeasureField>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScatterPlotUnaggregatedFieldWells {
    #[serde(rename = "Category")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Vec<DimensionField>>,
    #[serde(rename = "Label")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<Vec<DimensionField>>,
    #[serde(rename = "Size")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<Vec<MeasureField>>,
    #[serde(rename = "XAxis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_axis: Option<Vec<DimensionField>>,
    #[serde(rename = "YAxis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_axis: Option<Vec<DimensionField>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScatterPlotSortConfiguration {
    #[serde(rename = "ScatterPlotLimitConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scatter_plot_limit_configuration: Option<ItemsLimitConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableVisual {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ChartConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_configuration: Option<TableConfiguration>,
    #[serde(rename = "ConditionalFormatting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_formatting: Option<TableConditionalFormatting>,
    #[serde(rename = "Subtitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "VisualContentAltText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_content_alt_text: Option<String>,
    #[serde(rename = "VisualId")]
    #[serde(default)]
    pub visual_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableConfiguration {
    #[serde(rename = "DashboardCustomizationVisualOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_customization_visual_options: Option<DashboardCustomizationVisualOptions>,
    #[serde(rename = "FieldOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_options: Option<TableFieldOptions>,
    #[serde(rename = "FieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_wells: Option<TableFieldWells>,
    #[serde(rename = "Interactions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions: Option<VisualInteractionOptions>,
    #[serde(rename = "PaginatedReportOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paginated_report_options: Option<TablePaginatedReportOptions>,
    #[serde(rename = "SortConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_configuration: Option<TableSortConfiguration>,
    #[serde(rename = "TableInlineVisualizations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_inline_visualizations: Option<Vec<TableInlineVisualization>>,
    #[serde(rename = "TableOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_options: Option<TableOptions>,
    #[serde(rename = "Tooltip")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<TooltipOptions>,
    #[serde(rename = "TotalOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_options: Option<TotalOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableFieldOptions {
    #[serde(rename = "Order")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Vec<String>>,
    #[serde(rename = "PinnedFieldOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_field_options: Option<TablePinnedFieldOptions>,
    #[serde(rename = "SelectedFieldOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_field_options: Option<Vec<TableFieldOption>>,
    #[serde(rename = "TransposedTableOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transposed_table_options: Option<Vec<TransposedTableOption>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TablePinnedFieldOptions {
    #[serde(rename = "PinnedLeftFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_left_fields: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableFieldOption {
    #[serde(rename = "CustomLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_label: Option<String>,
    #[serde(rename = "FieldId")]
    #[serde(default)]
    pub field_id: String,
    #[serde(rename = "URLStyling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_r_l_styling: Option<TableFieldURLConfiguration>,
    #[serde(rename = "Visibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(rename = "Width")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableFieldURLConfiguration {
    #[serde(rename = "ImageConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_configuration: Option<TableFieldImageConfiguration>,
    #[serde(rename = "LinkConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_configuration: Option<TableFieldLinkConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableFieldImageConfiguration {
    #[serde(rename = "SizingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sizing_options: Option<TableCellImageSizingConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableCellImageSizingConfiguration {
    #[serde(rename = "TableCellImageScalingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_cell_image_scaling_configuration: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableFieldLinkConfiguration {
    #[serde(rename = "Content")]
    #[serde(default)]
    pub content: TableFieldLinkContentConfiguration,
    #[serde(rename = "Target")]
    #[serde(default)]
    pub target: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableFieldLinkContentConfiguration {
    #[serde(rename = "CustomIconContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_icon_content: Option<TableFieldCustomIconContent>,
    #[serde(rename = "CustomTextContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_text_content: Option<TableFieldCustomTextContent>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableFieldCustomIconContent {
    #[serde(rename = "Icon")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableFieldCustomTextContent {
    #[serde(rename = "FontConfiguration")]
    #[serde(default)]
    pub font_configuration: FontConfiguration,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransposedTableOption {
    #[serde(rename = "ColumnIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_index: Option<i32>,
    #[serde(rename = "ColumnType")]
    #[serde(default)]
    pub column_type: String,
    #[serde(rename = "ColumnWidth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_width: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableFieldWells {
    #[serde(rename = "TableAggregatedFieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_aggregated_field_wells: Option<TableAggregatedFieldWells>,
    #[serde(rename = "TableUnaggregatedFieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_unaggregated_field_wells: Option<TableUnaggregatedFieldWells>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableAggregatedFieldWells {
    #[serde(rename = "GroupBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<DimensionField>>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<MeasureField>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableUnaggregatedFieldWells {
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<UnaggregatedField>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TablePaginatedReportOptions {
    #[serde(rename = "OverflowColumnHeaderVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overflow_column_header_visibility: Option<String>,
    #[serde(rename = "VerticalOverflowVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_overflow_visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableSortConfiguration {
    #[serde(rename = "PaginationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_configuration: Option<PaginationConfiguration>,
    #[serde(rename = "RowSort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_sort: Option<Vec<FieldSortOptions>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableInlineVisualization {
    #[serde(rename = "DataBars")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_bars: Option<DataBarsOptions>,
    #[serde(rename = "Sparklines")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sparklines: Option<SparklinesOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataBarsOptions {
    #[serde(rename = "FieldId")]
    #[serde(default)]
    pub field_id: String,
    #[serde(rename = "NegativeColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_color: Option<String>,
    #[serde(rename = "PositiveColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positive_color: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SparklinesOptions {
    #[serde(rename = "AllPointsMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_points_marker: Option<LineChartMarkerStyleSettings>,
    #[serde(rename = "FieldId")]
    #[serde(default)]
    pub field_id: String,
    #[serde(rename = "LineColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_color: Option<String>,
    #[serde(rename = "LineInterpolation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_interpolation: Option<String>,
    #[serde(rename = "MaxValueMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value_marker: Option<LineChartMarkerStyleSettings>,
    #[serde(rename = "MinValueMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value_marker: Option<LineChartMarkerStyleSettings>,
    #[serde(rename = "VisualType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_type: Option<String>,
    #[serde(rename = "XAxisField")]
    #[serde(default)]
    pub x_axis_field: DimensionField,
    #[serde(rename = "YAxisBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_axis_behavior: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableOptions {
    #[serde(rename = "CellStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell_style: Option<TableCellStyle>,
    #[serde(rename = "HeaderStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_style: Option<TableCellStyle>,
    #[serde(rename = "Orientation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<String>,
    #[serde(rename = "RowAlternateColorOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_alternate_color_options: Option<RowAlternateColorOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TotalOptions {
    #[serde(rename = "CustomLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_label: Option<String>,
    #[serde(rename = "Placement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement: Option<String>,
    #[serde(rename = "ScrollStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scroll_status: Option<String>,
    #[serde(rename = "TotalAggregationOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_aggregation_options: Option<Vec<TotalAggregationOption>>,
    #[serde(rename = "TotalCellStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_cell_style: Option<TableCellStyle>,
    #[serde(rename = "TotalsVisibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totals_visibility: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableConditionalFormatting {
    #[serde(rename = "ConditionalFormattingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_formatting_options: Option<Vec<TableConditionalFormattingOption>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableConditionalFormattingOption {
    #[serde(rename = "Cell")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell: Option<TableCellConditionalFormatting>,
    #[serde(rename = "Row")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row: Option<TableRowConditionalFormatting>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableCellConditionalFormatting {
    #[serde(rename = "FieldId")]
    #[serde(default)]
    pub field_id: String,
    #[serde(rename = "TextFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_format: Option<TextConditionalFormat>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableRowConditionalFormatting {
    #[serde(rename = "BackgroundColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<ConditionalFormattingColor>,
    #[serde(rename = "TextColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<ConditionalFormattingColor>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TreeMapVisual {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ChartConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_configuration: Option<TreeMapConfiguration>,
    #[serde(rename = "ColumnHierarchies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "Subtitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "VisualContentAltText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_content_alt_text: Option<String>,
    #[serde(rename = "VisualId")]
    #[serde(default)]
    pub visual_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TreeMapConfiguration {
    #[serde(rename = "ColorLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "ColorScale")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_scale: Option<ColorScale>,
    #[serde(rename = "DataLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_labels: Option<DataLabelOptions>,
    #[serde(rename = "FieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_wells: Option<TreeMapFieldWells>,
    #[serde(rename = "GroupLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "Interactions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions: Option<VisualInteractionOptions>,
    #[serde(rename = "Legend")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "SizeLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "SortConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_configuration: Option<TreeMapSortConfiguration>,
    #[serde(rename = "Tooltip")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<TooltipOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TreeMapFieldWells {
    #[serde(rename = "TreeMapAggregatedFieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tree_map_aggregated_field_wells: Option<TreeMapAggregatedFieldWells>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TreeMapAggregatedFieldWells {
    #[serde(rename = "Colors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colors: Option<Vec<MeasureField>>,
    #[serde(rename = "Groups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<DimensionField>>,
    #[serde(rename = "Sizes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sizes: Option<Vec<MeasureField>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TreeMapSortConfiguration {
    #[serde(rename = "TreeMapGroupItemsLimitConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tree_map_group_items_limit_configuration: Option<ItemsLimitConfiguration>,
    #[serde(rename = "TreeMapSort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tree_map_sort: Option<Vec<FieldSortOptions>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WaterfallVisual {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ChartConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_configuration: Option<WaterfallChartConfiguration>,
    #[serde(rename = "ColumnHierarchies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "Subtitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "VisualContentAltText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_content_alt_text: Option<String>,
    #[serde(rename = "VisualId")]
    #[serde(default)]
    pub visual_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WaterfallChartConfiguration {
    #[serde(rename = "CategoryAxisDisplayOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_axis_display_options: Option<AxisDisplayOptions>,
    #[serde(rename = "CategoryAxisLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_axis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "ColorConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_configuration: Option<WaterfallChartColorConfiguration>,
    #[serde(rename = "DataLabels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_labels: Option<DataLabelOptions>,
    #[serde(rename = "FieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_wells: Option<WaterfallChartFieldWells>,
    #[serde(rename = "Interactions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions: Option<VisualInteractionOptions>,
    #[serde(rename = "Legend")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend: Option<LegendOptions>,
    #[serde(rename = "PrimaryYAxisDisplayOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_y_axis_display_options: Option<AxisDisplayOptions>,
    #[serde(rename = "PrimaryYAxisLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_y_axis_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "SortConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_configuration: Option<WaterfallChartSortConfiguration>,
    #[serde(rename = "VisualPalette")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_palette: Option<VisualPalette>,
    #[serde(rename = "WaterfallChartOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waterfall_chart_options: Option<WaterfallChartOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WaterfallChartColorConfiguration {
    #[serde(rename = "GroupColorConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_color_configuration: Option<WaterfallChartGroupColorConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WaterfallChartGroupColorConfiguration {
    #[serde(rename = "NegativeBarColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_bar_color: Option<String>,
    #[serde(rename = "PositiveBarColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positive_bar_color: Option<String>,
    #[serde(rename = "TotalBarColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_bar_color: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WaterfallChartFieldWells {
    #[serde(rename = "WaterfallChartAggregatedFieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waterfall_chart_aggregated_field_wells: Option<WaterfallChartAggregatedFieldWells>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WaterfallChartAggregatedFieldWells {
    #[serde(rename = "Breakdowns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakdowns: Option<Vec<DimensionField>>,
    #[serde(rename = "Categories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<DimensionField>>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<MeasureField>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WaterfallChartSortConfiguration {
    #[serde(rename = "BreakdownItemsLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakdown_items_limit: Option<ItemsLimitConfiguration>,
    #[serde(rename = "CategorySort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_sort: Option<Vec<FieldSortOptions>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WaterfallChartOptions {
    #[serde(rename = "TotalBarLabel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_bar_label: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WordCloudVisual {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<VisualCustomAction>>,
    #[serde(rename = "ChartConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart_configuration: Option<WordCloudChartConfiguration>,
    #[serde(rename = "ColumnHierarchies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_hierarchies: Option<Vec<ColumnHierarchy>>,
    #[serde(rename = "Subtitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<VisualSubtitleLabelOptions>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<VisualTitleLabelOptions>,
    #[serde(rename = "VisualContentAltText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_content_alt_text: Option<String>,
    #[serde(rename = "VisualId")]
    #[serde(default)]
    pub visual_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WordCloudChartConfiguration {
    #[serde(rename = "CategoryLabelOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_label_options: Option<ChartAxisLabelOptions>,
    #[serde(rename = "FieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_wells: Option<WordCloudFieldWells>,
    #[serde(rename = "Interactions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions: Option<VisualInteractionOptions>,
    #[serde(rename = "SortConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_configuration: Option<WordCloudSortConfiguration>,
    #[serde(rename = "WordCloudOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_cloud_options: Option<WordCloudOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WordCloudFieldWells {
    #[serde(rename = "WordCloudAggregatedFieldWells")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_cloud_aggregated_field_wells: Option<WordCloudAggregatedFieldWells>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WordCloudAggregatedFieldWells {
    #[serde(rename = "GroupBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<DimensionField>>,
    #[serde(rename = "Size")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<Vec<MeasureField>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WordCloudSortConfiguration {
    #[serde(rename = "CategoryItemsLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_items_limit: Option<ItemsLimitConfiguration>,
    #[serde(rename = "CategorySort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_sort: Option<Vec<FieldSortOptions>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WordCloudOptions {
    #[serde(rename = "CloudLayout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_layout: Option<String>,
    #[serde(rename = "MaximumStringLength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_string_length: Option<i32>,
    #[serde(rename = "WordCasing")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_casing: Option<String>,
    #[serde(rename = "WordOrientation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_orientation: Option<String>,
    #[serde(rename = "WordPadding")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_padding: Option<String>,
    #[serde(rename = "WordScaling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_scaling: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StaticFile {
    #[serde(rename = "ImageStaticFile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_static_file: Option<ImageStaticFile>,
    #[serde(rename = "SpatialStaticFile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spatial_static_file: Option<SpatialStaticFile>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImageStaticFile {
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<StaticFileSource>,
    #[serde(rename = "StaticFileId")]
    #[serde(default)]
    pub static_file_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StaticFileSource {
    #[serde(rename = "S3Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_options: Option<StaticFileS3SourceOptions>,
    #[serde(rename = "UrlOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_options: Option<StaticFileUrlSourceOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StaticFileS3SourceOptions {
    #[serde(rename = "BucketName")]
    #[serde(default)]
    pub bucket_name: String,
    #[serde(rename = "ObjectKey")]
    #[serde(default)]
    pub object_key: String,
    #[serde(rename = "Region")]
    #[serde(default)]
    pub region: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StaticFileUrlSourceOptions {
    #[serde(rename = "Url")]
    #[serde(default)]
    pub url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SpatialStaticFile {
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<StaticFileSource>,
    #[serde(rename = "StaticFileId")]
    #[serde(default)]
    pub static_file_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TooltipSheetDefinition {
    #[serde(rename = "Images")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<SheetImage>>,
    #[serde(rename = "Layouts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layouts: Option<Vec<Layout>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SheetId")]
    #[serde(default)]
    pub sheet_id: String,
    #[serde(rename = "TextBoxes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_boxes: Option<Vec<SheetTextBox>>,
    #[serde(rename = "Visuals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visuals: Option<Vec<Visual>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Parameters {
    #[serde(rename = "DateTimeParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time_parameters: Option<Vec<DateTimeParameter>>,
    #[serde(rename = "DecimalParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal_parameters: Option<Vec<DecimalParameter>>,
    #[serde(rename = "IntegerParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integer_parameters: Option<Vec<IntegerParameter>>,
    #[serde(rename = "StringParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_parameters: Option<Vec<StringParameter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DateTimeParameter {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DecimalParameter {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntegerParameter {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StringParameter {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalysisSourceEntity {
    #[serde(rename = "SourceTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_template: Option<AnalysisSourceTemplate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalysisSourceTemplate {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "DataSetReferences")]
    #[serde(default)]
    pub data_set_references: Vec<DataSetReference>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSetReference {
    #[serde(rename = "DataSetArn")]
    #[serde(default)]
    pub data_set_arn: String,
    #[serde(rename = "DataSetPlaceholder")]
    #[serde(default)]
    pub data_set_placeholder: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidationStrategy {
    #[serde(rename = "Mode")]
    #[serde(default)]
    pub mode: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAnalysisResponse {
    #[serde(rename = "AnalysisId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_id: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_status: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBrandRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "BrandDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_definition: Option<BrandDefinition>,
    #[serde(rename = "BrandId")]
    #[serde(default)]
    pub brand_id: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BrandDefinition {
    #[serde(rename = "ApplicationTheme")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_theme: Option<ApplicationTheme>,
    #[serde(rename = "BrandName")]
    #[serde(default)]
    pub brand_name: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LogoConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_configuration: Option<LogoConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationTheme {
    #[serde(rename = "BrandColorPalette")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_color_palette: Option<BrandColorPalette>,
    #[serde(rename = "BrandElementStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_element_style: Option<BrandElementStyle>,
    #[serde(rename = "ContextualAccentPalette")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contextual_accent_palette: Option<ContextualAccentPalette>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BrandColorPalette {
    #[serde(rename = "Accent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent: Option<Palette>,
    #[serde(rename = "Danger")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub danger: Option<Palette>,
    #[serde(rename = "Dimension")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension: Option<Palette>,
    #[serde(rename = "Info")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<Palette>,
    #[serde(rename = "Measure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measure: Option<Palette>,
    #[serde(rename = "Primary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<Palette>,
    #[serde(rename = "Secondary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary: Option<Palette>,
    #[serde(rename = "Success")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<Palette>,
    #[serde(rename = "Warning")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<Palette>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Palette {
    #[serde(rename = "Background")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    #[serde(rename = "Foreground")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreground: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BrandElementStyle {
    #[serde(rename = "NavbarStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub navbar_style: Option<NavbarStyle>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NavbarStyle {
    #[serde(rename = "ContextualNavbar")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contextual_navbar: Option<Palette>,
    #[serde(rename = "GlobalNavbar")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_navbar: Option<Palette>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContextualAccentPalette {
    #[serde(rename = "Automation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automation: Option<Palette>,
    #[serde(rename = "Connection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<Palette>,
    #[serde(rename = "Insight")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight: Option<Palette>,
    #[serde(rename = "Visualization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visualization: Option<Palette>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogoConfiguration {
    #[serde(rename = "AltText")]
    #[serde(default)]
    pub alt_text: String,
    #[serde(rename = "LogoSet")]
    #[serde(default)]
    pub logo_set: LogoSetConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogoSetConfiguration {
    #[serde(rename = "Favicon")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<ImageSetConfiguration>,
    #[serde(rename = "Primary")]
    #[serde(default)]
    pub primary: ImageSetConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImageSetConfiguration {
    #[serde(rename = "Original")]
    #[serde(default)]
    pub original: ImageConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImageConfiguration {
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ImageSource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImageSource {
    #[serde(rename = "PublicUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_url: Option<String>,
    #[serde(rename = "S3Uri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBrandResponse {
    #[serde(rename = "BrandDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_definition: Option<BrandDefinition>,
    #[serde(rename = "BrandDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_detail: Option<BrandDetail>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BrandDetail {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "BrandId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_id: Option<String>,
    #[serde(rename = "BrandStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_status: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "Logo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<Logo>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    #[serde(rename = "VersionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Logo {
    #[serde(rename = "AltText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_text: Option<String>,
    #[serde(rename = "LogoSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_set: Option<LogoSet>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogoSet {
    #[serde(rename = "Favicon")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<ImageSet>,
    #[serde(rename = "Primary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<ImageSet>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImageSet {
    #[serde(rename = "Height32")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height32: Option<Image>,
    #[serde(rename = "Height64")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height64: Option<Image>,
    #[serde(rename = "Original")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original: Option<Image>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Image {
    #[serde(rename = "GeneratedImageUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_image_url: Option<String>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ImageSource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCustomPermissionsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Capabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Capabilities>,
    #[serde(rename = "CustomPermissionsName")]
    #[serde(default)]
    pub custom_permissions_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Capabilities {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "AddOrRunAnomalyDetectionForAnalyses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_or_run_anomaly_detection_for_analyses: Option<String>,
    #[serde(rename = "AmazonBedrockARSAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_bedrock_a_r_s_action: Option<String>,
    #[serde(rename = "AmazonBedrockFSAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_bedrock_f_s_action: Option<String>,
    #[serde(rename = "AmazonBedrockKRSAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_bedrock_k_r_s_action: Option<String>,
    #[serde(rename = "AmazonSThreeAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_s_three_action: Option<String>,
    #[serde(rename = "Analysis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis: Option<String>,
    #[serde(rename = "ApproveFlowShareRequests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approve_flow_share_requests: Option<String>,
    #[serde(rename = "AsanaAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asana_action: Option<String>,
    #[serde(rename = "Automate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automate: Option<String>,
    #[serde(rename = "BambooHRAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bamboo_h_r_action: Option<String>,
    #[serde(rename = "BoxAgentAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub box_agent_action: Option<String>,
    #[serde(rename = "BuildCalculatedFieldWithQ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_calculated_field_with_q: Option<String>,
    #[serde(rename = "CanvaAgentAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canva_agent_action: Option<String>,
    #[serde(rename = "ChatAgent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_agent: Option<String>,
    #[serde(rename = "ComprehendAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comprehend_action: Option<String>,
    #[serde(rename = "ComprehendMedicalAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comprehend_medical_action: Option<String>,
    #[serde(rename = "ConfluenceAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confluence_action: Option<String>,
    #[serde(rename = "CreateAndUpdateAmazonBedrockARSAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_amazon_bedrock_a_r_s_action: Option<String>,
    #[serde(rename = "CreateAndUpdateAmazonBedrockFSAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_amazon_bedrock_f_s_action: Option<String>,
    #[serde(rename = "CreateAndUpdateAmazonBedrockKRSAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_amazon_bedrock_k_r_s_action: Option<String>,
    #[serde(rename = "CreateAndUpdateAmazonSThreeAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_amazon_s_three_action: Option<String>,
    #[serde(rename = "CreateAndUpdateAsanaAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_asana_action: Option<String>,
    #[serde(rename = "CreateAndUpdateBambooHRAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_bamboo_h_r_action: Option<String>,
    #[serde(rename = "CreateAndUpdateBoxAgentAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_box_agent_action: Option<String>,
    #[serde(rename = "CreateAndUpdateCanvaAgentAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_canva_agent_action: Option<String>,
    #[serde(rename = "CreateAndUpdateComprehendAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_comprehend_action: Option<String>,
    #[serde(rename = "CreateAndUpdateComprehendMedicalAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_comprehend_medical_action: Option<String>,
    #[serde(rename = "CreateAndUpdateConfluenceAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_confluence_action: Option<String>,
    #[serde(rename = "CreateAndUpdateDashboardEmailReports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_dashboard_email_reports: Option<String>,
    #[serde(rename = "CreateAndUpdateDataSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_data_sources: Option<String>,
    #[serde(rename = "CreateAndUpdateDatasets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_datasets: Option<String>,
    #[serde(rename = "CreateAndUpdateFactSetAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_fact_set_action: Option<String>,
    #[serde(rename = "CreateAndUpdateGenericHTTPAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_generic_h_t_t_p_action: Option<String>,
    #[serde(rename = "CreateAndUpdateGithubAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_github_action: Option<String>,
    #[serde(rename = "CreateAndUpdateGoogleCalendarAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_google_calendar_action: Option<String>,
    #[serde(rename = "CreateAndUpdateHubspotAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_hubspot_action: Option<String>,
    #[serde(rename = "CreateAndUpdateHuggingFaceAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_hugging_face_action: Option<String>,
    #[serde(rename = "CreateAndUpdateIntercomAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_intercom_action: Option<String>,
    #[serde(rename = "CreateAndUpdateJiraAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_jira_action: Option<String>,
    #[serde(rename = "CreateAndUpdateLinearAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_linear_action: Option<String>,
    #[serde(rename = "CreateAndUpdateMCPAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_m_c_p_action: Option<String>,
    #[serde(rename = "CreateAndUpdateMSExchangeAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_m_s_exchange_action: Option<String>,
    #[serde(rename = "CreateAndUpdateMSTeamsAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_m_s_teams_action: Option<String>,
    #[serde(rename = "CreateAndUpdateMondayAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_monday_action: Option<String>,
    #[serde(rename = "CreateAndUpdateNewRelicAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_new_relic_action: Option<String>,
    #[serde(rename = "CreateAndUpdateNotionAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_notion_action: Option<String>,
    #[serde(rename = "CreateAndUpdateOneDriveAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_one_drive_action: Option<String>,
    #[serde(rename = "CreateAndUpdateOpenAPIAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_open_a_p_i_action: Option<String>,
    #[serde(rename = "CreateAndUpdatePagerDutyAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_pager_duty_action: Option<String>,
    #[serde(rename = "CreateAndUpdateSAPBillOfMaterialAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_s_a_p_bill_of_material_action: Option<String>,
    #[serde(rename = "CreateAndUpdateSAPBusinessPartnerAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_s_a_p_business_partner_action: Option<String>,
    #[serde(rename = "CreateAndUpdateSAPMaterialStockAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_s_a_p_material_stock_action: Option<String>,
    #[serde(rename = "CreateAndUpdateSAPPhysicalInventoryAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_s_a_p_physical_inventory_action: Option<String>,
    #[serde(rename = "CreateAndUpdateSAPProductMasterDataAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_s_a_p_product_master_data_action: Option<String>,
    #[serde(rename = "CreateAndUpdateSalesforceAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_salesforce_action: Option<String>,
    #[serde(rename = "CreateAndUpdateSandPGMIAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_sand_p_g_m_i_action: Option<String>,
    #[serde(rename = "CreateAndUpdateSandPGlobalEnergyAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_sand_p_global_energy_action: Option<String>,
    #[serde(rename = "CreateAndUpdateServiceNowAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_service_now_action: Option<String>,
    #[serde(rename = "CreateAndUpdateSharePointAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_share_point_action: Option<String>,
    #[serde(rename = "CreateAndUpdateSlackAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_slack_action: Option<String>,
    #[serde(rename = "CreateAndUpdateSmartsheetAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_smartsheet_action: Option<String>,
    #[serde(rename = "CreateAndUpdateTextractAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_textract_action: Option<String>,
    #[serde(rename = "CreateAndUpdateThemes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_themes: Option<String>,
    #[serde(rename = "CreateAndUpdateThresholdAlerts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_threshold_alerts: Option<String>,
    #[serde(rename = "CreateAndUpdateZendeskAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_and_update_zendesk_action: Option<String>,
    #[serde(rename = "CreateChatAgents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_chat_agents: Option<String>,
    #[serde(rename = "CreateDashboardExecutiveSummaryWithQ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_dashboard_executive_summary_with_q: Option<String>,
    #[serde(rename = "CreateSPICEDataset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_s_p_i_c_e_dataset: Option<String>,
    #[serde(rename = "CreateSharedFolders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_shared_folders: Option<String>,
    #[serde(rename = "CreateSpaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_spaces: Option<String>,
    #[serde(rename = "Dashboard")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard: Option<String>,
    #[serde(rename = "EditVisualWithQ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_visual_with_q: Option<String>,
    #[serde(rename = "ExportToCsv")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_to_csv: Option<String>,
    #[serde(rename = "ExportToCsvInScheduledReports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_to_csv_in_scheduled_reports: Option<String>,
    #[serde(rename = "ExportToExcel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_to_excel: Option<String>,
    #[serde(rename = "ExportToExcelInScheduledReports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_to_excel_in_scheduled_reports: Option<String>,
    #[serde(rename = "ExportToPdf")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_to_pdf: Option<String>,
    #[serde(rename = "ExportToPdfInScheduledReports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_to_pdf_in_scheduled_reports: Option<String>,
    #[serde(rename = "Extension")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,
    #[serde(rename = "FactSetAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fact_set_action: Option<String>,
    #[serde(rename = "Flow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<String>,
    #[serde(rename = "GenerateAnalyses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_analyses: Option<String>,
    #[serde(rename = "GenericHTTPAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generic_h_t_t_p_action: Option<String>,
    #[serde(rename = "GithubAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github_action: Option<String>,
    #[serde(rename = "GoogleCalendarAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_calendar_action: Option<String>,
    #[serde(rename = "HubspotAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hubspot_action: Option<String>,
    #[serde(rename = "HuggingFaceAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hugging_face_action: Option<String>,
    #[serde(rename = "IncludeContentInScheduledReportsEmail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_content_in_scheduled_reports_email: Option<String>,
    #[serde(rename = "IntercomAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intercom_action: Option<String>,
    #[serde(rename = "JiraAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jira_action: Option<String>,
    #[serde(rename = "KnowledgeBase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base: Option<String>,
    #[serde(rename = "LinearAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linear_action: Option<String>,
    #[serde(rename = "MCPAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_c_p_action: Option<String>,
    #[serde(rename = "MSExchangeAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_s_exchange_action: Option<String>,
    #[serde(rename = "MSTeamsAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_s_teams_action: Option<String>,
    #[serde(rename = "ManageSharedFolders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manage_shared_folders: Option<String>,
    #[serde(rename = "MondayAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monday_action: Option<String>,
    #[serde(rename = "NewRelicAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_relic_action: Option<String>,
    #[serde(rename = "NotionAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notion_action: Option<String>,
    #[serde(rename = "OneDriveAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_drive_action: Option<String>,
    #[serde(rename = "OpenAPIAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_a_p_i_action: Option<String>,
    #[serde(rename = "PagerDutyAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pager_duty_action: Option<String>,
    #[serde(rename = "PerformFlowUiTask")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perform_flow_ui_task: Option<String>,
    #[serde(rename = "PrintReports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_reports: Option<String>,
    #[serde(rename = "PublishWithoutApproval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_without_approval: Option<String>,
    #[serde(rename = "RenameSharedFolders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rename_shared_folders: Option<String>,
    #[serde(rename = "Research")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub research: Option<String>,
    #[serde(rename = "SAPBillOfMaterialAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_a_p_bill_of_material_action: Option<String>,
    #[serde(rename = "SAPBusinessPartnerAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_a_p_business_partner_action: Option<String>,
    #[serde(rename = "SAPMaterialStockAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_a_p_material_stock_action: Option<String>,
    #[serde(rename = "SAPPhysicalInventoryAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_a_p_physical_inventory_action: Option<String>,
    #[serde(rename = "SAPProductMasterDataAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_a_p_product_master_data_action: Option<String>,
    #[serde(rename = "SalesforceAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salesforce_action: Option<String>,
    #[serde(rename = "SandPGMIAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sand_p_g_m_i_action: Option<String>,
    #[serde(rename = "SandPGlobalEnergyAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sand_p_global_energy_action: Option<String>,
    #[serde(rename = "Scenario")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenario: Option<String>,
    #[serde(rename = "SelfUpgradeUserRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_upgrade_user_role: Option<String>,
    #[serde(rename = "ServiceNowAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_now_action: Option<String>,
    #[serde(rename = "ShareAmazonBedrockARSAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_amazon_bedrock_a_r_s_action: Option<String>,
    #[serde(rename = "ShareAmazonBedrockFSAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_amazon_bedrock_f_s_action: Option<String>,
    #[serde(rename = "ShareAmazonBedrockKRSAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_amazon_bedrock_k_r_s_action: Option<String>,
    #[serde(rename = "ShareAmazonSThreeAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_amazon_s_three_action: Option<String>,
    #[serde(rename = "ShareAnalyses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_analyses: Option<String>,
    #[serde(rename = "ShareAsanaAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_asana_action: Option<String>,
    #[serde(rename = "ShareBambooHRAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_bamboo_h_r_action: Option<String>,
    #[serde(rename = "ShareBoxAgentAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_box_agent_action: Option<String>,
    #[serde(rename = "ShareCanvaAgentAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_canva_agent_action: Option<String>,
    #[serde(rename = "ShareChatAgents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_chat_agents: Option<String>,
    #[serde(rename = "ShareComprehendAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_comprehend_action: Option<String>,
    #[serde(rename = "ShareComprehendMedicalAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_comprehend_medical_action: Option<String>,
    #[serde(rename = "ShareConfluenceAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_confluence_action: Option<String>,
    #[serde(rename = "ShareDashboards")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_dashboards: Option<String>,
    #[serde(rename = "ShareDataSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_data_sources: Option<String>,
    #[serde(rename = "ShareDatasets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_datasets: Option<String>,
    #[serde(rename = "ShareFactSetAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_fact_set_action: Option<String>,
    #[serde(rename = "ShareGenericHTTPAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_generic_h_t_t_p_action: Option<String>,
    #[serde(rename = "ShareGithubAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_github_action: Option<String>,
    #[serde(rename = "ShareGoogleCalendarAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_google_calendar_action: Option<String>,
    #[serde(rename = "ShareHubspotAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_hubspot_action: Option<String>,
    #[serde(rename = "ShareHuggingFaceAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_hugging_face_action: Option<String>,
    #[serde(rename = "ShareIntercomAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_intercom_action: Option<String>,
    #[serde(rename = "ShareJiraAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_jira_action: Option<String>,
    #[serde(rename = "ShareLinearAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_linear_action: Option<String>,
    #[serde(rename = "ShareMCPAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_m_c_p_action: Option<String>,
    #[serde(rename = "ShareMSExchangeAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_m_s_exchange_action: Option<String>,
    #[serde(rename = "ShareMSTeamsAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_m_s_teams_action: Option<String>,
    #[serde(rename = "ShareMondayAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_monday_action: Option<String>,
    #[serde(rename = "ShareNewRelicAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_new_relic_action: Option<String>,
    #[serde(rename = "ShareNotionAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_notion_action: Option<String>,
    #[serde(rename = "ShareOneDriveAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_one_drive_action: Option<String>,
    #[serde(rename = "ShareOpenAPIAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_open_a_p_i_action: Option<String>,
    #[serde(rename = "SharePagerDutyAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_pager_duty_action: Option<String>,
    #[serde(rename = "SharePointAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_point_action: Option<String>,
    #[serde(rename = "ShareSAPBillOfMaterialAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_s_a_p_bill_of_material_action: Option<String>,
    #[serde(rename = "ShareSAPBusinessPartnerAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_s_a_p_business_partner_action: Option<String>,
    #[serde(rename = "ShareSAPMaterialStockAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_s_a_p_material_stock_action: Option<String>,
    #[serde(rename = "ShareSAPPhysicalInventoryAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_s_a_p_physical_inventory_action: Option<String>,
    #[serde(rename = "ShareSAPProductMasterDataAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_s_a_p_product_master_data_action: Option<String>,
    #[serde(rename = "ShareSalesforceAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_salesforce_action: Option<String>,
    #[serde(rename = "ShareSandPGMIAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_sand_p_g_m_i_action: Option<String>,
    #[serde(rename = "ShareSandPGlobalEnergyAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_sand_p_global_energy_action: Option<String>,
    #[serde(rename = "ShareServiceNowAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_service_now_action: Option<String>,
    #[serde(rename = "ShareSharePointAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_share_point_action: Option<String>,
    #[serde(rename = "ShareSlackAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_slack_action: Option<String>,
    #[serde(rename = "ShareSmartsheetAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_smartsheet_action: Option<String>,
    #[serde(rename = "ShareSpaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_spaces: Option<String>,
    #[serde(rename = "ShareTextractAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_textract_action: Option<String>,
    #[serde(rename = "ShareZendeskAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_zendesk_action: Option<String>,
    #[serde(rename = "SlackAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slack_action: Option<String>,
    #[serde(rename = "SmartsheetAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smartsheet_action: Option<String>,
    #[serde(rename = "Space")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space: Option<String>,
    #[serde(rename = "Story")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story: Option<String>,
    #[serde(rename = "SubscribeDashboardEmailReports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_dashboard_email_reports: Option<String>,
    #[serde(rename = "TextractAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub textract_action: Option<String>,
    #[serde(rename = "Topic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(rename = "UseAgentWebSearch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_agent_web_search: Option<String>,
    #[serde(rename = "UseAmazonBedrockARSAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_amazon_bedrock_a_r_s_action: Option<String>,
    #[serde(rename = "UseAmazonBedrockFSAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_amazon_bedrock_f_s_action: Option<String>,
    #[serde(rename = "UseAmazonBedrockKRSAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_amazon_bedrock_k_r_s_action: Option<String>,
    #[serde(rename = "UseAmazonSThreeAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_amazon_s_three_action: Option<String>,
    #[serde(rename = "UseAsanaAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_asana_action: Option<String>,
    #[serde(rename = "UseBambooHRAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_bamboo_h_r_action: Option<String>,
    #[serde(rename = "UseBedrockModels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_bedrock_models: Option<String>,
    #[serde(rename = "UseBoxAgentAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_box_agent_action: Option<String>,
    #[serde(rename = "UseCanvaAgentAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_canva_agent_action: Option<String>,
    #[serde(rename = "UseComprehendAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_comprehend_action: Option<String>,
    #[serde(rename = "UseComprehendMedicalAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_comprehend_medical_action: Option<String>,
    #[serde(rename = "UseConfluenceAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_confluence_action: Option<String>,
    #[serde(rename = "UseFactSetAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_fact_set_action: Option<String>,
    #[serde(rename = "UseGenericHTTPAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_generic_h_t_t_p_action: Option<String>,
    #[serde(rename = "UseGithubAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_github_action: Option<String>,
    #[serde(rename = "UseGoogleCalendarAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_google_calendar_action: Option<String>,
    #[serde(rename = "UseHubspotAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_hubspot_action: Option<String>,
    #[serde(rename = "UseHuggingFaceAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_hugging_face_action: Option<String>,
    #[serde(rename = "UseIntercomAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_intercom_action: Option<String>,
    #[serde(rename = "UseJiraAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_jira_action: Option<String>,
    #[serde(rename = "UseLinearAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_linear_action: Option<String>,
    #[serde(rename = "UseMCPAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_m_c_p_action: Option<String>,
    #[serde(rename = "UseMSExchangeAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_m_s_exchange_action: Option<String>,
    #[serde(rename = "UseMSTeamsAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_m_s_teams_action: Option<String>,
    #[serde(rename = "UseMondayAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_monday_action: Option<String>,
    #[serde(rename = "UseNewRelicAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_new_relic_action: Option<String>,
    #[serde(rename = "UseNotionAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_notion_action: Option<String>,
    #[serde(rename = "UseOneDriveAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_one_drive_action: Option<String>,
    #[serde(rename = "UseOpenAPIAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_open_a_p_i_action: Option<String>,
    #[serde(rename = "UsePagerDutyAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_pager_duty_action: Option<String>,
    #[serde(rename = "UseSAPBillOfMaterialAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_s_a_p_bill_of_material_action: Option<String>,
    #[serde(rename = "UseSAPBusinessPartnerAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_s_a_p_business_partner_action: Option<String>,
    #[serde(rename = "UseSAPMaterialStockAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_s_a_p_material_stock_action: Option<String>,
    #[serde(rename = "UseSAPPhysicalInventoryAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_s_a_p_physical_inventory_action: Option<String>,
    #[serde(rename = "UseSAPProductMasterDataAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_s_a_p_product_master_data_action: Option<String>,
    #[serde(rename = "UseSalesforceAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_salesforce_action: Option<String>,
    #[serde(rename = "UseSandPGMIAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_sand_p_g_m_i_action: Option<String>,
    #[serde(rename = "UseSandPGlobalEnergyAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_sand_p_global_energy_action: Option<String>,
    #[serde(rename = "UseServiceNowAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_service_now_action: Option<String>,
    #[serde(rename = "UseSharePointAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_share_point_action: Option<String>,
    #[serde(rename = "UseSlackAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_slack_action: Option<String>,
    #[serde(rename = "UseSmartsheetAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_smartsheet_action: Option<String>,
    #[serde(rename = "UseTextractAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_textract_action: Option<String>,
    #[serde(rename = "UseZendeskAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_zendesk_action: Option<String>,
    #[serde(rename = "ViewAccountSPICECapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_account_s_p_i_c_e_capacity: Option<String>,
    #[serde(rename = "ZendeskAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zendesk_action: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCustomPermissionsResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDashboardRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DashboardId")]
    #[serde(default)]
    pub dashboard_id: String,
    #[serde(rename = "DashboardPublishOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_publish_options: Option<DashboardPublishOptions>,
    #[serde(rename = "Definition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<DashboardVersionDefinition>,
    #[serde(rename = "FolderArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_arns: Option<Vec<String>>,
    #[serde(rename = "LinkEntities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_entities: Option<Vec<String>>,
    #[serde(rename = "LinkSharingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_sharing_configuration: Option<LinkSharingConfiguration>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Parameters>,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "SourceEntity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_entity: Option<DashboardSourceEntity>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "ThemeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_arn: Option<String>,
    #[serde(rename = "ValidationStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_strategy: Option<ValidationStrategy>,
    #[serde(rename = "VersionDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DashboardPublishOptions {
    #[serde(rename = "AdHocFilteringOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_hoc_filtering_option: Option<AdHocFilteringOption>,
    #[serde(rename = "DataPointDrillUpDownOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_point_drill_up_down_option: Option<DataPointDrillUpDownOption>,
    #[serde(rename = "DataPointMenuLabelOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_point_menu_label_option: Option<DataPointMenuLabelOption>,
    #[serde(rename = "DataPointTooltipOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_point_tooltip_option: Option<DataPointTooltipOption>,
    #[serde(rename = "DataQAEnabledOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_q_a_enabled_option: Option<DataQAEnabledOption>,
    #[serde(rename = "DataStoriesSharingOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_stories_sharing_option: Option<DataStoriesSharingOption>,
    #[serde(rename = "ExecutiveSummaryOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executive_summary_option: Option<ExecutiveSummaryOption>,
    #[serde(rename = "ExportToCSVOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_to_c_s_v_option: Option<ExportToCSVOption>,
    #[serde(rename = "ExportWithHiddenFieldsOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_with_hidden_fields_option: Option<ExportWithHiddenFieldsOption>,
    #[serde(rename = "QuickSuiteActionsOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_suite_actions_option: Option<QuickSuiteActionsOption>,
    #[serde(rename = "SheetControlsOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_controls_option: Option<SheetControlsOption>,
    #[serde(rename = "SheetLayoutElementMaximizationOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_layout_element_maximization_option: Option<SheetLayoutElementMaximizationOption>,
    #[serde(rename = "VisualAxisSortOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_axis_sort_option: Option<VisualAxisSortOption>,
    #[serde(rename = "VisualMenuOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_menu_option: Option<VisualMenuOption>,
    #[serde(rename = "VisualPublishOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_publish_options: Option<DashboardVisualPublishOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdHocFilteringOption {
    #[serde(rename = "AvailabilityStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataPointDrillUpDownOption {
    #[serde(rename = "AvailabilityStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataPointMenuLabelOption {
    #[serde(rename = "AvailabilityStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataPointTooltipOption {
    #[serde(rename = "AvailabilityStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataQAEnabledOption {
    #[serde(rename = "AvailabilityStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataStoriesSharingOption {
    #[serde(rename = "AvailabilityStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutiveSummaryOption {
    #[serde(rename = "AvailabilityStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportToCSVOption {
    #[serde(rename = "AvailabilityStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportWithHiddenFieldsOption {
    #[serde(rename = "AvailabilityStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QuickSuiteActionsOption {
    #[serde(rename = "AvailabilityStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SheetControlsOption {
    #[serde(rename = "VisibilityState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SheetLayoutElementMaximizationOption {
    #[serde(rename = "AvailabilityStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VisualAxisSortOption {
    #[serde(rename = "AvailabilityStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DashboardVisualPublishOptions {
    #[serde(rename = "ExportHiddenFieldsOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_hidden_fields_option: Option<ExportHiddenFieldsOption>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportHiddenFieldsOption {
    #[serde(rename = "AvailabilityStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DashboardVersionDefinition {
    #[serde(rename = "AnalysisDefaults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_defaults: Option<AnalysisDefaults>,
    #[serde(rename = "CalculatedFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculated_fields: Option<Vec<CalculatedField>>,
    #[serde(rename = "ColumnConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_configurations: Option<Vec<ColumnConfiguration>>,
    #[serde(rename = "DataSetIdentifierDeclarations")]
    #[serde(default)]
    pub data_set_identifier_declarations: Vec<DataSetIdentifierDeclaration>,
    #[serde(rename = "FilterGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_groups: Option<Vec<FilterGroup>>,
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<AssetOptions>,
    #[serde(rename = "ParameterDeclarations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_declarations: Option<Vec<ParameterDeclaration>>,
    #[serde(rename = "Sheets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheets: Option<Vec<SheetDefinition>>,
    #[serde(rename = "StaticFiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_files: Option<Vec<StaticFile>>,
    #[serde(rename = "TooltipSheets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip_sheets: Option<Vec<TooltipSheetDefinition>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LinkSharingConfiguration {
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DashboardSourceEntity {
    #[serde(rename = "SourceTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_template: Option<DashboardSourceTemplate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DashboardSourceTemplate {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "DataSetReferences")]
    #[serde(default)]
    pub data_set_references: Vec<DataSetReference>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDashboardResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_status: Option<String>,
    #[serde(rename = "DashboardId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_id: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "VersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDataSetRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "ColumnGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_groups: Option<Vec<ColumnGroup>>,
    #[serde(rename = "ColumnLevelPermissionRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_level_permission_rules: Option<Vec<ColumnLevelPermissionRule>>,
    #[serde(rename = "DataPrepConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_prep_configuration: Option<DataPrepConfiguration>,
    #[serde(rename = "DataSetId")]
    #[serde(default)]
    pub data_set_id: String,
    #[serde(rename = "DataSetUsageConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_usage_configuration: Option<DataSetUsageConfiguration>,
    #[serde(rename = "DatasetParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_parameters: Option<Vec<DatasetParameter>>,
    #[serde(rename = "FieldFolders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_folders: Option<std::collections::HashMap<String, FieldFolder>>,
    #[serde(rename = "FolderArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_arns: Option<Vec<String>>,
    #[serde(rename = "ImportMode")]
    #[serde(default)]
    pub import_mode: String,
    #[serde(rename = "LogicalTableMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_table_map: Option<std::collections::HashMap<String, LogicalTable>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "PerformanceConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_configuration: Option<PerformanceConfiguration>,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "PhysicalTableMap")]
    #[serde(default)]
    pub physical_table_map: std::collections::HashMap<String, PhysicalTable>,
    #[serde(rename = "RowLevelPermissionDataSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_level_permission_data_set: Option<RowLevelPermissionDataSet>,
    #[serde(rename = "RowLevelPermissionTagConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_level_permission_tag_configuration: Option<RowLevelPermissionTagConfiguration>,
    #[serde(rename = "SemanticModelConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_model_configuration: Option<SemanticModelConfiguration>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "UseAs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_as: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ColumnGroup {
    #[serde(rename = "GeoSpatialColumnGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo_spatial_column_group: Option<GeoSpatialColumnGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeoSpatialColumnGroup {
    #[serde(rename = "Columns")]
    #[serde(default)]
    pub columns: Vec<String>,
    #[serde(rename = "CountryCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ColumnLevelPermissionRule {
    #[serde(rename = "ColumnNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_names: Option<Vec<String>>,
    #[serde(rename = "Principals")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principals: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataPrepConfiguration {
    #[serde(rename = "DestinationTableMap")]
    #[serde(default)]
    pub destination_table_map: std::collections::HashMap<String, DestinationTable>,
    #[serde(rename = "SourceTableMap")]
    #[serde(default)]
    pub source_table_map: std::collections::HashMap<String, SourceTable>,
    #[serde(rename = "TransformStepMap")]
    #[serde(default)]
    pub transform_step_map: std::collections::HashMap<String, TransformStep>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DestinationTable {
    #[serde(rename = "Alias")]
    #[serde(default)]
    pub alias: String,
    #[serde(rename = "Source")]
    #[serde(default)]
    pub source: DestinationTableSource,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DestinationTableSource {
    #[serde(rename = "TransformOperationId")]
    #[serde(default)]
    pub transform_operation_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceTable {
    #[serde(rename = "DataSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set: Option<ParentDataSet>,
    #[serde(rename = "PhysicalTableId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_table_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParentDataSet {
    #[serde(rename = "DataSetArn")]
    #[serde(default)]
    pub data_set_arn: String,
    #[serde(rename = "InputColumns")]
    #[serde(default)]
    pub input_columns: Vec<InputColumn>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputColumn {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "SubType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransformStep {
    #[serde(rename = "AggregateStep")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_step: Option<AggregateOperation>,
    #[serde(rename = "AppendStep")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub append_step: Option<AppendOperation>,
    #[serde(rename = "CastColumnTypesStep")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cast_column_types_step: Option<CastColumnTypesOperation>,
    #[serde(rename = "CreateColumnsStep")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_columns_step: Option<CreateColumnsOperation>,
    #[serde(rename = "FiltersStep")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters_step: Option<FiltersOperation>,
    #[serde(rename = "ImportTableStep")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_table_step: Option<ImportTableOperation>,
    #[serde(rename = "JoinStep")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_step: Option<JoinOperation>,
    #[serde(rename = "PivotStep")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pivot_step: Option<PivotOperation>,
    #[serde(rename = "ProjectStep")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_step: Option<ProjectOperation>,
    #[serde(rename = "RenameColumnsStep")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rename_columns_step: Option<RenameColumnsOperation>,
    #[serde(rename = "UnpivotStep")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unpivot_step: Option<UnpivotOperation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AggregateOperation {
    #[serde(rename = "Aggregations")]
    #[serde(default)]
    pub aggregations: Vec<Aggregation>,
    #[serde(rename = "Alias")]
    #[serde(default)]
    pub alias: String,
    #[serde(rename = "GroupByColumnNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by_column_names: Option<Vec<String>>,
    #[serde(rename = "Source")]
    #[serde(default)]
    pub source: TransformOperationSource,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Aggregation {
    #[serde(rename = "AggregationFunction")]
    #[serde(default)]
    pub aggregation_function: DataPrepAggregationFunction,
    #[serde(rename = "NewColumnId")]
    #[serde(default)]
    pub new_column_id: String,
    #[serde(rename = "NewColumnName")]
    #[serde(default)]
    pub new_column_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataPrepAggregationFunction {
    #[serde(rename = "ListAggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_aggregation: Option<DataPrepListAggregationFunction>,
    #[serde(rename = "SimpleAggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_aggregation: Option<DataPrepSimpleAggregationFunction>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataPrepListAggregationFunction {
    #[serde(rename = "Distinct")]
    #[serde(default)]
    pub distinct: bool,
    #[serde(rename = "InputColumnName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_column_name: Option<String>,
    #[serde(rename = "Separator")]
    #[serde(default)]
    pub separator: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataPrepSimpleAggregationFunction {
    #[serde(rename = "FunctionType")]
    #[serde(default)]
    pub function_type: String,
    #[serde(rename = "InputColumnName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_column_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransformOperationSource {
    #[serde(rename = "ColumnIdMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_id_mappings: Option<Vec<DataSetColumnIdMapping>>,
    #[serde(rename = "TransformOperationId")]
    #[serde(default)]
    pub transform_operation_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSetColumnIdMapping {
    #[serde(rename = "SourceColumnId")]
    #[serde(default)]
    pub source_column_id: String,
    #[serde(rename = "TargetColumnId")]
    #[serde(default)]
    pub target_column_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AppendOperation {
    #[serde(rename = "Alias")]
    #[serde(default)]
    pub alias: String,
    #[serde(rename = "AppendedColumns")]
    #[serde(default)]
    pub appended_columns: Vec<AppendedColumn>,
    #[serde(rename = "FirstSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_source: Option<TransformOperationSource>,
    #[serde(rename = "SecondSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_source: Option<TransformOperationSource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AppendedColumn {
    #[serde(rename = "ColumnName")]
    #[serde(default)]
    pub column_name: String,
    #[serde(rename = "NewColumnId")]
    #[serde(default)]
    pub new_column_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CastColumnTypesOperation {
    #[serde(rename = "Alias")]
    #[serde(default)]
    pub alias: String,
    #[serde(rename = "CastColumnTypeOperations")]
    #[serde(default)]
    pub cast_column_type_operations: Vec<CastColumnTypeOperation>,
    #[serde(rename = "Source")]
    #[serde(default)]
    pub source: TransformOperationSource,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CastColumnTypeOperation {
    #[serde(rename = "ColumnName")]
    #[serde(default)]
    pub column_name: String,
    #[serde(rename = "Format")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "NewColumnType")]
    #[serde(default)]
    pub new_column_type: String,
    #[serde(rename = "SubType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateColumnsOperation {
    #[serde(rename = "Alias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "Columns")]
    #[serde(default)]
    pub columns: Vec<CalculatedColumn>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<TransformOperationSource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CalculatedColumn {
    #[serde(rename = "ColumnId")]
    #[serde(default)]
    pub column_id: String,
    #[serde(rename = "ColumnName")]
    #[serde(default)]
    pub column_name: String,
    #[serde(rename = "Expression")]
    #[serde(default)]
    pub expression: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FiltersOperation {
    #[serde(rename = "Alias")]
    #[serde(default)]
    pub alias: String,
    #[serde(rename = "FilterOperations")]
    #[serde(default)]
    pub filter_operations: Vec<FilterOperation>,
    #[serde(rename = "Source")]
    #[serde(default)]
    pub source: TransformOperationSource,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterOperation {
    #[serde(rename = "ConditionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_expression: Option<String>,
    #[serde(rename = "DateFilterCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_filter_condition: Option<DataSetDateFilterCondition>,
    #[serde(rename = "NumericFilterCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric_filter_condition: Option<DataSetNumericFilterCondition>,
    #[serde(rename = "StringFilterCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_filter_condition: Option<DataSetStringFilterCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSetDateFilterCondition {
    #[serde(rename = "ColumnName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_name: Option<String>,
    #[serde(rename = "ComparisonFilterCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_filter_condition: Option<DataSetDateComparisonFilterCondition>,
    #[serde(rename = "RangeFilterCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_filter_condition: Option<DataSetDateRangeFilterCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSetDateComparisonFilterCondition {
    #[serde(rename = "Operator")]
    #[serde(default)]
    pub operator: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<DataSetDateFilterValue>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSetDateFilterValue {
    #[serde(rename = "StaticValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_value: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSetDateRangeFilterCondition {
    #[serde(rename = "IncludeMaximum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_maximum: Option<bool>,
    #[serde(rename = "IncludeMinimum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_minimum: Option<bool>,
    #[serde(rename = "RangeMaximum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_maximum: Option<DataSetDateFilterValue>,
    #[serde(rename = "RangeMinimum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_minimum: Option<DataSetDateFilterValue>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSetNumericFilterCondition {
    #[serde(rename = "ColumnName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_name: Option<String>,
    #[serde(rename = "ComparisonFilterCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_filter_condition: Option<DataSetNumericComparisonFilterCondition>,
    #[serde(rename = "RangeFilterCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_filter_condition: Option<DataSetNumericRangeFilterCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSetNumericComparisonFilterCondition {
    #[serde(rename = "Operator")]
    #[serde(default)]
    pub operator: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<DataSetNumericFilterValue>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSetNumericFilterValue {
    #[serde(rename = "StaticValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_value: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSetNumericRangeFilterCondition {
    #[serde(rename = "IncludeMaximum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_maximum: Option<bool>,
    #[serde(rename = "IncludeMinimum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_minimum: Option<bool>,
    #[serde(rename = "RangeMaximum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_maximum: Option<DataSetNumericFilterValue>,
    #[serde(rename = "RangeMinimum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_minimum: Option<DataSetNumericFilterValue>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSetStringFilterCondition {
    #[serde(rename = "ColumnName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_name: Option<String>,
    #[serde(rename = "ComparisonFilterCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparison_filter_condition: Option<DataSetStringComparisonFilterCondition>,
    #[serde(rename = "ListFilterCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_filter_condition: Option<DataSetStringListFilterCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSetStringComparisonFilterCondition {
    #[serde(rename = "Operator")]
    #[serde(default)]
    pub operator: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<DataSetStringFilterValue>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSetStringFilterValue {
    #[serde(rename = "StaticValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSetStringListFilterCondition {
    #[serde(rename = "Operator")]
    #[serde(default)]
    pub operator: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<DataSetStringListFilterValue>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSetStringListFilterValue {
    #[serde(rename = "StaticValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportTableOperation {
    #[serde(rename = "Alias")]
    #[serde(default)]
    pub alias: String,
    #[serde(rename = "Source")]
    #[serde(default)]
    pub source: ImportTableOperationSource,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportTableOperationSource {
    #[serde(rename = "ColumnIdMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_id_mappings: Option<Vec<DataSetColumnIdMapping>>,
    #[serde(rename = "SourceTableId")]
    #[serde(default)]
    pub source_table_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JoinOperation {
    #[serde(rename = "Alias")]
    #[serde(default)]
    pub alias: String,
    #[serde(rename = "LeftOperand")]
    #[serde(default)]
    pub left_operand: TransformOperationSource,
    #[serde(rename = "LeftOperandProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_operand_properties: Option<JoinOperandProperties>,
    #[serde(rename = "OnClause")]
    #[serde(default)]
    pub on_clause: String,
    #[serde(rename = "RightOperand")]
    #[serde(default)]
    pub right_operand: TransformOperationSource,
    #[serde(rename = "RightOperandProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_operand_properties: Option<JoinOperandProperties>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JoinOperandProperties {
    #[serde(rename = "OutputColumnNameOverrides")]
    #[serde(default)]
    pub output_column_name_overrides: Vec<OutputColumnNameOverride>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutputColumnNameOverride {
    #[serde(rename = "OutputColumnName")]
    #[serde(default)]
    pub output_column_name: String,
    #[serde(rename = "SourceColumnName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_column_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PivotOperation {
    #[serde(rename = "Alias")]
    #[serde(default)]
    pub alias: String,
    #[serde(rename = "GroupByColumnNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by_column_names: Option<Vec<String>>,
    #[serde(rename = "PivotConfiguration")]
    #[serde(default)]
    pub pivot_configuration: PivotConfiguration,
    #[serde(rename = "Source")]
    #[serde(default)]
    pub source: TransformOperationSource,
    #[serde(rename = "ValueColumnConfiguration")]
    #[serde(default)]
    pub value_column_configuration: ValueColumnConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PivotConfiguration {
    #[serde(rename = "LabelColumnName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_column_name: Option<String>,
    #[serde(rename = "PivotedLabels")]
    #[serde(default)]
    pub pivoted_labels: Vec<PivotedLabel>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PivotedLabel {
    #[serde(rename = "LabelName")]
    #[serde(default)]
    pub label_name: String,
    #[serde(rename = "NewColumnId")]
    #[serde(default)]
    pub new_column_id: String,
    #[serde(rename = "NewColumnName")]
    #[serde(default)]
    pub new_column_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValueColumnConfiguration {
    #[serde(rename = "AggregationFunction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_function: Option<DataPrepAggregationFunction>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProjectOperation {
    #[serde(rename = "Alias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "ProjectedColumns")]
    #[serde(default)]
    pub projected_columns: Vec<String>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<TransformOperationSource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RenameColumnsOperation {
    #[serde(rename = "Alias")]
    #[serde(default)]
    pub alias: String,
    #[serde(rename = "RenameColumnOperations")]
    #[serde(default)]
    pub rename_column_operations: Vec<RenameColumnOperation>,
    #[serde(rename = "Source")]
    #[serde(default)]
    pub source: TransformOperationSource,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RenameColumnOperation {
    #[serde(rename = "ColumnName")]
    #[serde(default)]
    pub column_name: String,
    #[serde(rename = "NewColumnName")]
    #[serde(default)]
    pub new_column_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnpivotOperation {
    #[serde(rename = "Alias")]
    #[serde(default)]
    pub alias: String,
    #[serde(rename = "ColumnsToUnpivot")]
    #[serde(default)]
    pub columns_to_unpivot: Vec<ColumnToUnpivot>,
    #[serde(rename = "Source")]
    #[serde(default)]
    pub source: TransformOperationSource,
    #[serde(rename = "UnpivotedLabelColumnId")]
    #[serde(default)]
    pub unpivoted_label_column_id: String,
    #[serde(rename = "UnpivotedLabelColumnName")]
    #[serde(default)]
    pub unpivoted_label_column_name: String,
    #[serde(rename = "UnpivotedValueColumnId")]
    #[serde(default)]
    pub unpivoted_value_column_id: String,
    #[serde(rename = "UnpivotedValueColumnName")]
    #[serde(default)]
    pub unpivoted_value_column_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ColumnToUnpivot {
    #[serde(rename = "ColumnName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_name: Option<String>,
    #[serde(rename = "NewValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSetUsageConfiguration {
    #[serde(rename = "DisableUseAsDirectQuerySource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_use_as_direct_query_source: Option<bool>,
    #[serde(rename = "DisableUseAsImportedSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_use_as_imported_source: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatasetParameter {
    #[serde(rename = "DateTimeDatasetParameter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time_dataset_parameter: Option<DateTimeDatasetParameter>,
    #[serde(rename = "DecimalDatasetParameter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal_dataset_parameter: Option<DecimalDatasetParameter>,
    #[serde(rename = "IntegerDatasetParameter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integer_dataset_parameter: Option<IntegerDatasetParameter>,
    #[serde(rename = "StringDatasetParameter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_dataset_parameter: Option<StringDatasetParameter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DateTimeDatasetParameter {
    #[serde(rename = "DefaultValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_values: Option<DateTimeDatasetParameterDefaultValues>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "TimeGranularity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_granularity: Option<String>,
    #[serde(rename = "ValueType")]
    #[serde(default)]
    pub value_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DateTimeDatasetParameterDefaultValues {
    #[serde(rename = "StaticValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_values: Option<Vec<f64>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DecimalDatasetParameter {
    #[serde(rename = "DefaultValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_values: Option<DecimalDatasetParameterDefaultValues>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ValueType")]
    #[serde(default)]
    pub value_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DecimalDatasetParameterDefaultValues {
    #[serde(rename = "StaticValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_values: Option<Vec<f64>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntegerDatasetParameter {
    #[serde(rename = "DefaultValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_values: Option<IntegerDatasetParameterDefaultValues>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ValueType")]
    #[serde(default)]
    pub value_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntegerDatasetParameterDefaultValues {
    #[serde(rename = "StaticValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_values: Option<Vec<i64>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StringDatasetParameter {
    #[serde(rename = "DefaultValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_values: Option<StringDatasetParameterDefaultValues>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ValueType")]
    #[serde(default)]
    pub value_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StringDatasetParameterDefaultValues {
    #[serde(rename = "StaticValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FieldFolder {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogicalTable {
    #[serde(rename = "Alias")]
    #[serde(default)]
    pub alias: String,
    #[serde(rename = "DataTransforms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_transforms: Option<Vec<TransformOperation>>,
    #[serde(rename = "Source")]
    #[serde(default)]
    pub source: LogicalTableSource,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransformOperation {
    #[serde(rename = "CastColumnTypeOperation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cast_column_type_operation: Option<CastColumnTypeOperation>,
    #[serde(rename = "CreateColumnsOperation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_columns_operation: Option<CreateColumnsOperation>,
    #[serde(rename = "FilterOperation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_operation: Option<FilterOperation>,
    #[serde(rename = "OverrideDatasetParameterOperation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_dataset_parameter_operation: Option<OverrideDatasetParameterOperation>,
    #[serde(rename = "ProjectOperation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_operation: Option<ProjectOperation>,
    #[serde(rename = "RenameColumnOperation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rename_column_operation: Option<RenameColumnOperation>,
    #[serde(rename = "TagColumnOperation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_column_operation: Option<TagColumnOperation>,
    #[serde(rename = "UntagColumnOperation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub untag_column_operation: Option<UntagColumnOperation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OverrideDatasetParameterOperation {
    #[serde(rename = "NewDefaultValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_default_values: Option<NewDefaultValues>,
    #[serde(rename = "NewParameterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_parameter_name: Option<String>,
    #[serde(rename = "ParameterName")]
    #[serde(default)]
    pub parameter_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NewDefaultValues {
    #[serde(rename = "DateTimeStaticValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_time_static_values: Option<Vec<f64>>,
    #[serde(rename = "DecimalStaticValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal_static_values: Option<Vec<f64>>,
    #[serde(rename = "IntegerStaticValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integer_static_values: Option<Vec<i64>>,
    #[serde(rename = "StringStaticValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_static_values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagColumnOperation {
    #[serde(rename = "ColumnName")]
    #[serde(default)]
    pub column_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<ColumnTag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ColumnTag {
    #[serde(rename = "ColumnDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_description: Option<ColumnDescription>,
    #[serde(rename = "ColumnGeographicRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_geographic_role: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ColumnDescription {
    #[serde(rename = "Text")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagColumnOperation {
    #[serde(rename = "ColumnName")]
    #[serde(default)]
    pub column_name: String,
    #[serde(rename = "TagNames")]
    #[serde(default)]
    pub tag_names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogicalTableSource {
    #[serde(rename = "DataSetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_arn: Option<String>,
    #[serde(rename = "JoinInstruction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_instruction: Option<JoinInstruction>,
    #[serde(rename = "PhysicalTableId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_table_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JoinInstruction {
    #[serde(rename = "LeftJoinKeyProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_join_key_properties: Option<JoinKeyProperties>,
    #[serde(rename = "LeftOperand")]
    #[serde(default)]
    pub left_operand: String,
    #[serde(rename = "OnClause")]
    #[serde(default)]
    pub on_clause: String,
    #[serde(rename = "RightJoinKeyProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_join_key_properties: Option<JoinKeyProperties>,
    #[serde(rename = "RightOperand")]
    #[serde(default)]
    pub right_operand: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JoinKeyProperties {
    #[serde(rename = "UniqueKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_key: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PerformanceConfiguration {
    #[serde(rename = "UniqueKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_keys: Option<Vec<UniqueKey>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UniqueKey {
    #[serde(rename = "ColumnNames")]
    #[serde(default)]
    pub column_names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PhysicalTable {
    #[serde(rename = "CustomSql")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_sql: Option<CustomSql>,
    #[serde(rename = "RelationalTable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relational_table: Option<RelationalTable>,
    #[serde(rename = "S3Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_source: Option<S3Source>,
    #[serde(rename = "SaaSTable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saa_s_table: Option<SaaSTable>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomSql {
    #[serde(rename = "Columns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<InputColumn>>,
    #[serde(rename = "DataSourceArn")]
    #[serde(default)]
    pub data_source_arn: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "SqlQuery")]
    #[serde(default)]
    pub sql_query: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RelationalTable {
    #[serde(rename = "Catalog")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog: Option<String>,
    #[serde(rename = "DataSourceArn")]
    #[serde(default)]
    pub data_source_arn: String,
    #[serde(rename = "InputColumns")]
    #[serde(default)]
    pub input_columns: Vec<InputColumn>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Schema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Source {
    #[serde(rename = "DataSourceArn")]
    #[serde(default)]
    pub data_source_arn: String,
    #[serde(rename = "InputColumns")]
    #[serde(default)]
    pub input_columns: Vec<InputColumn>,
    #[serde(rename = "UploadSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_settings: Option<UploadSettings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UploadSettings {
    #[serde(rename = "ContainsHeader")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains_header: Option<bool>,
    #[serde(rename = "CustomCellAddressRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_cell_address_range: Option<String>,
    #[serde(rename = "Delimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    #[serde(rename = "Format")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "StartFromRow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_from_row: Option<i32>,
    #[serde(rename = "TextQualifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_qualifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SaaSTable {
    #[serde(rename = "DataSourceArn")]
    #[serde(default)]
    pub data_source_arn: String,
    #[serde(rename = "InputColumns")]
    #[serde(default)]
    pub input_columns: Vec<InputColumn>,
    #[serde(rename = "TablePath")]
    #[serde(default)]
    pub table_path: Vec<TablePathElement>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TablePathElement {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RowLevelPermissionDataSet {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "FormatVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_version: Option<String>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "PermissionPolicy")]
    #[serde(default)]
    pub permission_policy: String,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RowLevelPermissionTagConfiguration {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TagRuleConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_rule_configurations: Option<Vec<Vec<String>>>,
    #[serde(rename = "TagRules")]
    #[serde(default)]
    pub tag_rules: Vec<RowLevelPermissionTagRule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RowLevelPermissionTagRule {
    #[serde(rename = "ColumnName")]
    #[serde(default)]
    pub column_name: String,
    #[serde(rename = "MatchAllValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_all_value: Option<String>,
    #[serde(rename = "TagKey")]
    #[serde(default)]
    pub tag_key: String,
    #[serde(rename = "TagMultiValueDelimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_multi_value_delimiter: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SemanticModelConfiguration {
    #[serde(rename = "TableMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_map: Option<std::collections::HashMap<String, SemanticTable>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SemanticTable {
    #[serde(rename = "Alias")]
    #[serde(default)]
    pub alias: String,
    #[serde(rename = "DestinationTableId")]
    #[serde(default)]
    pub destination_table_id: String,
    #[serde(rename = "RowLevelPermissionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_level_permission_configuration: Option<RowLevelPermissionConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RowLevelPermissionConfiguration {
    #[serde(rename = "RowLevelPermissionDataSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_level_permission_data_set: Option<RowLevelPermissionDataSet>,
    #[serde(rename = "TagConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_configuration: Option<RowLevelPermissionTagConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDataSetResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "DataSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    #[serde(rename = "IngestionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_arn: Option<String>,
    #[serde(rename = "IngestionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_id: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDataSourceRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Credentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<DataSourceCredentials>,
    #[serde(rename = "DataSourceId")]
    #[serde(default)]
    pub data_source_id: String,
    #[serde(rename = "DataSourceParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_parameters: Option<DataSourceParameters>,
    #[serde(rename = "FolderArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_arns: Option<Vec<String>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "SslProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_properties: Option<SslProperties>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "VpcConnectionProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_connection_properties: Option<VpcConnectionProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSourceCredentials {
    #[serde(rename = "CopySourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_source_arn: Option<String>,
    #[serde(rename = "CredentialPair")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_pair: Option<CredentialPair>,
    #[serde(rename = "KeyPairCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair_credentials: Option<KeyPairCredentials>,
    #[serde(rename = "OAuthClientCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth_client_credentials: Option<OAuthClientCredentials>,
    #[serde(rename = "SecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
    #[serde(rename = "WebProxyCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_proxy_credentials: Option<WebProxyCredentials>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CredentialPair {
    #[serde(rename = "AlternateDataSourceParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate_data_source_parameters: Option<Vec<DataSourceParameters>>,
    #[serde(rename = "Password")]
    #[serde(default)]
    pub password: String,
    #[serde(rename = "Username")]
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSourceParameters {
    #[serde(rename = "AmazonElasticsearchParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_elasticsearch_parameters: Option<AmazonElasticsearchParameters>,
    #[serde(rename = "AmazonOpenSearchParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_open_search_parameters: Option<AmazonOpenSearchParameters>,
    #[serde(rename = "AthenaParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub athena_parameters: Option<AthenaParameters>,
    #[serde(rename = "AuroraParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aurora_parameters: Option<AuroraParameters>,
    #[serde(rename = "AuroraPostgreSqlParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aurora_postgre_sql_parameters: Option<AuroraPostgreSqlParameters>,
    #[serde(rename = "AwsIotAnalyticsParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_iot_analytics_parameters: Option<AwsIotAnalyticsParameters>,
    #[serde(rename = "BigQueryParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub big_query_parameters: Option<BigQueryParameters>,
    #[serde(rename = "ConfluenceParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confluence_parameters: Option<ConfluenceParameters>,
    #[serde(rename = "CustomConnectionParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_connection_parameters: Option<CustomConnectionParameters>,
    #[serde(rename = "DatabricksParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub databricks_parameters: Option<DatabricksParameters>,
    #[serde(rename = "ExasolParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exasol_parameters: Option<ExasolParameters>,
    #[serde(rename = "ImpalaParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impala_parameters: Option<ImpalaParameters>,
    #[serde(rename = "JiraParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jira_parameters: Option<JiraParameters>,
    #[serde(rename = "MariaDbParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maria_db_parameters: Option<MariaDbParameters>,
    #[serde(rename = "MySqlParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_sql_parameters: Option<MySqlParameters>,
    #[serde(rename = "OracleParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oracle_parameters: Option<OracleParameters>,
    #[serde(rename = "PostgreSqlParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postgre_sql_parameters: Option<PostgreSqlParameters>,
    #[serde(rename = "PrestoParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presto_parameters: Option<PrestoParameters>,
    #[serde(rename = "QBusinessParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q_business_parameters: Option<QBusinessParameters>,
    #[serde(rename = "RdsParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_parameters: Option<RdsParameters>,
    #[serde(rename = "RedshiftParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_parameters: Option<RedshiftParameters>,
    #[serde(rename = "S3KnowledgeBaseParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_knowledge_base_parameters: Option<S3KnowledgeBaseParameters>,
    #[serde(rename = "S3Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_parameters: Option<S3Parameters>,
    #[serde(rename = "S3TablesParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_tables_parameters: Option<S3TablesParameters>,
    #[serde(rename = "ServiceNowParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_now_parameters: Option<ServiceNowParameters>,
    #[serde(rename = "SnowflakeParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snowflake_parameters: Option<SnowflakeParameters>,
    #[serde(rename = "SparkParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spark_parameters: Option<SparkParameters>,
    #[serde(rename = "SqlServerParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_server_parameters: Option<SqlServerParameters>,
    #[serde(rename = "StarburstParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starburst_parameters: Option<StarburstParameters>,
    #[serde(rename = "TeradataParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teradata_parameters: Option<TeradataParameters>,
    #[serde(rename = "TrinoParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trino_parameters: Option<TrinoParameters>,
    #[serde(rename = "TwitterParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter_parameters: Option<TwitterParameters>,
    #[serde(rename = "WebCrawlerParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_crawler_parameters: Option<WebCrawlerParameters>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AmazonElasticsearchParameters {
    #[serde(rename = "Domain")]
    #[serde(default)]
    pub domain: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AmazonOpenSearchParameters {
    #[serde(rename = "Domain")]
    #[serde(default)]
    pub domain: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AthenaParameters {
    #[serde(rename = "ConsumerAccountRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_account_role_arn: Option<String>,
    #[serde(rename = "IdentityCenterConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_center_configuration: Option<IdentityCenterConfiguration>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "WorkGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_group: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IdentityCenterConfiguration {
    #[serde(rename = "EnableIdentityPropagation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_identity_propagation: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuroraParameters {
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "Host")]
    #[serde(default)]
    pub host: String,
    #[serde(rename = "Port")]
    #[serde(default)]
    pub port: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuroraPostgreSqlParameters {
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "Host")]
    #[serde(default)]
    pub host: String,
    #[serde(rename = "Port")]
    #[serde(default)]
    pub port: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsIotAnalyticsParameters {
    #[serde(rename = "DataSetName")]
    #[serde(default)]
    pub data_set_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BigQueryParameters {
    #[serde(rename = "DataSetRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_region: Option<String>,
    #[serde(rename = "ProjectId")]
    #[serde(default)]
    pub project_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfluenceParameters {
    #[serde(rename = "ConfluenceUrl")]
    #[serde(default)]
    pub confluence_url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomConnectionParameters {
    #[serde(rename = "ConnectionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatabricksParameters {
    #[serde(rename = "Host")]
    #[serde(default)]
    pub host: String,
    #[serde(rename = "Port")]
    #[serde(default)]
    pub port: i32,
    #[serde(rename = "SqlEndpointPath")]
    #[serde(default)]
    pub sql_endpoint_path: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExasolParameters {
    #[serde(rename = "Host")]
    #[serde(default)]
    pub host: String,
    #[serde(rename = "Port")]
    #[serde(default)]
    pub port: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImpalaParameters {
    #[serde(rename = "Database")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[serde(rename = "Host")]
    #[serde(default)]
    pub host: String,
    #[serde(rename = "Port")]
    #[serde(default)]
    pub port: i32,
    #[serde(rename = "SqlEndpointPath")]
    #[serde(default)]
    pub sql_endpoint_path: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JiraParameters {
    #[serde(rename = "SiteBaseUrl")]
    #[serde(default)]
    pub site_base_url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MariaDbParameters {
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "Host")]
    #[serde(default)]
    pub host: String,
    #[serde(rename = "Port")]
    #[serde(default)]
    pub port: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MySqlParameters {
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "Host")]
    #[serde(default)]
    pub host: String,
    #[serde(rename = "Port")]
    #[serde(default)]
    pub port: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OracleParameters {
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "Host")]
    #[serde(default)]
    pub host: String,
    #[serde(rename = "Port")]
    #[serde(default)]
    pub port: i32,
    #[serde(rename = "UseServiceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_service_name: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PostgreSqlParameters {
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "Host")]
    #[serde(default)]
    pub host: String,
    #[serde(rename = "Port")]
    #[serde(default)]
    pub port: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PrestoParameters {
    #[serde(rename = "Catalog")]
    #[serde(default)]
    pub catalog: String,
    #[serde(rename = "Host")]
    #[serde(default)]
    pub host: String,
    #[serde(rename = "Port")]
    #[serde(default)]
    pub port: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QBusinessParameters {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    pub application_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RdsParameters {
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    pub instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RedshiftParameters {
    #[serde(rename = "ClusterId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "Host")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(rename = "IAMParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_a_m_parameters: Option<RedshiftIAMParameters>,
    #[serde(rename = "IdentityCenterConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_center_configuration: Option<IdentityCenterConfiguration>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RedshiftIAMParameters {
    #[serde(rename = "AutoCreateDatabaseUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_create_database_user: Option<bool>,
    #[serde(rename = "DatabaseGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_groups: Option<Vec<String>>,
    #[serde(rename = "DatabaseUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_user: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3KnowledgeBaseParameters {
    #[serde(rename = "BucketUrl")]
    #[serde(default)]
    pub bucket_url: String,
    #[serde(rename = "MetadataFilesLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_files_location: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Parameters {
    #[serde(rename = "ManifestFileLocation")]
    #[serde(default)]
    pub manifest_file_location: ManifestFileLocation,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManifestFileLocation {
    #[serde(rename = "Bucket")]
    #[serde(default)]
    pub bucket: String,
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3TablesParameters {
    #[serde(rename = "TableBucketArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_bucket_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceNowParameters {
    #[serde(rename = "SiteBaseUrl")]
    #[serde(default)]
    pub site_base_url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnowflakeParameters {
    #[serde(rename = "AuthenticationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "DatabaseAccessControlRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_access_control_role: Option<String>,
    #[serde(rename = "Host")]
    #[serde(default)]
    pub host: String,
    #[serde(rename = "OAuthParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth_parameters: Option<OAuthParameters>,
    #[serde(rename = "Warehouse")]
    #[serde(default)]
    pub warehouse: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OAuthParameters {
    #[serde(rename = "IdentityProviderCACertificatesBundleS3Uri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_c_a_certificates_bundle_s3_uri: Option<String>,
    #[serde(rename = "IdentityProviderResourceUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_resource_uri: Option<String>,
    #[serde(rename = "IdentityProviderVpcConnectionProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_provider_vpc_connection_properties: Option<VpcConnectionProperties>,
    #[serde(rename = "OAuthScope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth_scope: Option<String>,
    #[serde(rename = "TokenProviderUrl")]
    #[serde(default)]
    pub token_provider_url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcConnectionProperties {
    #[serde(rename = "VpcConnectionArn")]
    #[serde(default)]
    pub vpc_connection_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SparkParameters {
    #[serde(rename = "Host")]
    #[serde(default)]
    pub host: String,
    #[serde(rename = "Port")]
    #[serde(default)]
    pub port: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SqlServerParameters {
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "Host")]
    #[serde(default)]
    pub host: String,
    #[serde(rename = "Port")]
    #[serde(default)]
    pub port: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StarburstParameters {
    #[serde(rename = "AuthenticationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    #[serde(rename = "Catalog")]
    #[serde(default)]
    pub catalog: String,
    #[serde(rename = "DatabaseAccessControlRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_access_control_role: Option<String>,
    #[serde(rename = "Host")]
    #[serde(default)]
    pub host: String,
    #[serde(rename = "OAuthParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth_parameters: Option<OAuthParameters>,
    #[serde(rename = "Port")]
    #[serde(default)]
    pub port: i32,
    #[serde(rename = "ProductType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TeradataParameters {
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "Host")]
    #[serde(default)]
    pub host: String,
    #[serde(rename = "Port")]
    #[serde(default)]
    pub port: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrinoParameters {
    #[serde(rename = "Catalog")]
    #[serde(default)]
    pub catalog: String,
    #[serde(rename = "Host")]
    #[serde(default)]
    pub host: String,
    #[serde(rename = "Port")]
    #[serde(default)]
    pub port: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TwitterParameters {
    #[serde(rename = "MaxRows")]
    #[serde(default)]
    pub max_rows: i32,
    #[serde(rename = "Query")]
    #[serde(default)]
    pub query: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WebCrawlerParameters {
    #[serde(rename = "LoginPageUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_page_url: Option<String>,
    #[serde(rename = "PasswordButtonXpath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_button_xpath: Option<String>,
    #[serde(rename = "PasswordFieldXpath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_field_xpath: Option<String>,
    #[serde(rename = "UsernameButtonXpath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username_button_xpath: Option<String>,
    #[serde(rename = "UsernameFieldXpath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username_field_xpath: Option<String>,
    #[serde(rename = "WebCrawlerAuthType")]
    #[serde(default)]
    pub web_crawler_auth_type: String,
    #[serde(rename = "WebProxyHostName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_proxy_host_name: Option<String>,
    #[serde(rename = "WebProxyPortNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_proxy_port_number: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KeyPairCredentials {
    #[serde(rename = "KeyPairUsername")]
    #[serde(default)]
    pub key_pair_username: String,
    #[serde(rename = "PrivateKey")]
    #[serde(default)]
    pub private_key: String,
    #[serde(rename = "PrivateKeyPassphrase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key_passphrase: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OAuthClientCredentials {
    #[serde(rename = "ClientId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "ClientSecret")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(rename = "Username")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WebProxyCredentials {
    #[serde(rename = "WebProxyPassword")]
    #[serde(default)]
    pub web_proxy_password: String,
    #[serde(rename = "WebProxyUsername")]
    #[serde(default)]
    pub web_proxy_username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SslProperties {
    #[serde(rename = "DisableSsl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_ssl: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDataSourceResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_status: Option<String>,
    #[serde(rename = "DataSourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFolderMembershipRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "FolderId")]
    #[serde(default)]
    pub folder_id: String,
    #[serde(rename = "MemberId")]
    #[serde(default)]
    pub member_id: String,
    #[serde(rename = "MemberType")]
    #[serde(default)]
    pub member_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFolderMembershipResponse {
    #[serde(rename = "FolderMember")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_member: Option<FolderMember>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FolderMember {
    #[serde(rename = "MemberId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    #[serde(rename = "MemberType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFolderRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "FolderId")]
    #[serde(default)]
    pub folder_id: String,
    #[serde(rename = "FolderType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_type: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ParentFolderArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_arn: Option<String>,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "SharingModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharing_model: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFolderResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "FolderId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGroupMembershipRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    pub group_name: String,
    #[serde(rename = "MemberName")]
    #[serde(default)]
    pub member_name: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGroupMembershipResponse {
    #[serde(rename = "GroupMember")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_member: Option<GroupMember>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GroupMember {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "MemberName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGroupRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    pub group_name: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGroupResponse {
    #[serde(rename = "Group")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Group {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "PrincipalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIAMPolicyAssignmentRequest {
    #[serde(rename = "AssignmentName")]
    #[serde(default)]
    pub assignment_name: String,
    #[serde(rename = "AssignmentStatus")]
    #[serde(default)]
    pub assignment_status: String,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Identities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identities: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "PolicyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIAMPolicyAssignmentResponse {
    #[serde(rename = "AssignmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_id: Option<String>,
    #[serde(rename = "AssignmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_name: Option<String>,
    #[serde(rename = "AssignmentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_status: Option<String>,
    #[serde(rename = "Identities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identities: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "PolicyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIngestionRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DataSetId")]
    #[serde(default)]
    pub data_set_id: String,
    #[serde(rename = "IngestionId")]
    #[serde(default)]
    pub ingestion_id: String,
    #[serde(rename = "IngestionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIngestionResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "IngestionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_id: Option<String>,
    #[serde(rename = "IngestionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_status: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateNamespaceRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "IdentityStore")]
    #[serde(default)]
    pub identity_store: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateNamespaceResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CapacityRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_region: Option<String>,
    #[serde(rename = "CreationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_status: Option<String>,
    #[serde(rename = "IdentityStore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_store: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRefreshScheduleRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DataSetId")]
    #[serde(default)]
    pub data_set_id: String,
    #[serde(rename = "Schedule")]
    #[serde(default)]
    pub schedule: RefreshSchedule,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RefreshSchedule {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "RefreshType")]
    #[serde(default)]
    pub refresh_type: String,
    #[serde(rename = "ScheduleFrequency")]
    #[serde(default)]
    pub schedule_frequency: RefreshFrequency,
    #[serde(rename = "ScheduleId")]
    #[serde(default)]
    pub schedule_id: String,
    #[serde(rename = "StartAfterDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_after_date_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RefreshFrequency {
    #[serde(rename = "Interval")]
    #[serde(default)]
    pub interval: String,
    #[serde(rename = "RefreshOnDay")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_on_day: Option<ScheduleRefreshOnEntity>,
    #[serde(rename = "TimeOfTheDay")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_of_the_day: Option<String>,
    #[serde(rename = "Timezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduleRefreshOnEntity {
    #[serde(rename = "DayOfMonth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_month: Option<String>,
    #[serde(rename = "DayOfWeek")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRefreshScheduleResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "ScheduleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRoleMembershipRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "MemberName")]
    #[serde(default)]
    pub member_name: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "Role")]
    #[serde(default)]
    pub role: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRoleMembershipResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTemplateAliasRequest {
    #[serde(rename = "AliasName")]
    #[serde(default)]
    pub alias_name: String,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "TemplateId")]
    #[serde(default)]
    pub template_id: String,
    #[serde(rename = "TemplateVersionNumber")]
    #[serde(default)]
    pub template_version_number: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTemplateAliasResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "TemplateAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_alias: Option<TemplateAlias>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TemplateAlias {
    #[serde(rename = "AliasName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "TemplateVersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_version_number: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTemplateRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Definition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<TemplateVersionDefinition>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "SourceEntity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_entity: Option<TemplateSourceEntity>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TemplateId")]
    #[serde(default)]
    pub template_id: String,
    #[serde(rename = "ValidationStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_strategy: Option<ValidationStrategy>,
    #[serde(rename = "VersionDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TemplateVersionDefinition {
    #[serde(rename = "AnalysisDefaults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_defaults: Option<AnalysisDefaults>,
    #[serde(rename = "CalculatedFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculated_fields: Option<Vec<CalculatedField>>,
    #[serde(rename = "ColumnConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_configurations: Option<Vec<ColumnConfiguration>>,
    #[serde(rename = "DataSetConfigurations")]
    #[serde(default)]
    pub data_set_configurations: Vec<DataSetConfiguration>,
    #[serde(rename = "FilterGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_groups: Option<Vec<FilterGroup>>,
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<AssetOptions>,
    #[serde(rename = "ParameterDeclarations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_declarations: Option<Vec<ParameterDeclaration>>,
    #[serde(rename = "QueryExecutionOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_execution_options: Option<QueryExecutionOptions>,
    #[serde(rename = "Sheets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheets: Option<Vec<SheetDefinition>>,
    #[serde(rename = "StaticFiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_files: Option<Vec<StaticFile>>,
    #[serde(rename = "TooltipSheets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip_sheets: Option<Vec<TooltipSheetDefinition>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSetConfiguration {
    #[serde(rename = "ColumnGroupSchemaList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_group_schema_list: Option<Vec<ColumnGroupSchema>>,
    #[serde(rename = "DataSetSchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_schema: Option<DataSetSchema>,
    #[serde(rename = "Placeholder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ColumnGroupSchema {
    #[serde(rename = "ColumnGroupColumnSchemaList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_group_column_schema_list: Option<Vec<ColumnGroupColumnSchema>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ColumnGroupColumnSchema {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSetSchema {
    #[serde(rename = "ColumnSchemaList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_schema_list: Option<Vec<ColumnSchema>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ColumnSchema {
    #[serde(rename = "DataType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(rename = "GeographicRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geographic_role: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TemplateSourceEntity {
    #[serde(rename = "SourceAnalysis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_analysis: Option<TemplateSourceAnalysis>,
    #[serde(rename = "SourceTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_template: Option<TemplateSourceTemplate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TemplateSourceAnalysis {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "DataSetReferences")]
    #[serde(default)]
    pub data_set_references: Vec<DataSetReference>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TemplateSourceTemplate {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTemplateResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_status: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "TemplateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(rename = "VersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateThemeAliasRequest {
    #[serde(rename = "AliasName")]
    #[serde(default)]
    pub alias_name: String,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "ThemeId")]
    #[serde(default)]
    pub theme_id: String,
    #[serde(rename = "ThemeVersionNumber")]
    #[serde(default)]
    pub theme_version_number: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateThemeAliasResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "ThemeAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_alias: Option<ThemeAlias>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThemeAlias {
    #[serde(rename = "AliasName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ThemeVersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_version_number: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateThemeRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "BaseThemeId")]
    #[serde(default)]
    pub base_theme_id: String,
    #[serde(rename = "Configuration")]
    #[serde(default)]
    pub configuration: ThemeConfiguration,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "ThemeId")]
    #[serde(default)]
    pub theme_id: String,
    #[serde(rename = "VersionDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThemeConfiguration {
    #[serde(rename = "DataColorPalette")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_color_palette: Option<DataColorPalette>,
    #[serde(rename = "Sheet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet: Option<SheetStyle>,
    #[serde(rename = "Typography")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typography: Option<Typography>,
    #[serde(rename = "UIColorPalette")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_i_color_palette: Option<UIColorPalette>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataColorPalette {
    #[serde(rename = "Colors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colors: Option<Vec<String>>,
    #[serde(rename = "EmptyFillColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_fill_color: Option<String>,
    #[serde(rename = "MinMaxGradient")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_max_gradient: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SheetStyle {
    #[serde(rename = "Background")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<SheetBackgroundStyle>,
    #[serde(rename = "Tile")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile: Option<TileStyle>,
    #[serde(rename = "TileLayout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tile_layout: Option<TileLayoutStyle>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SheetBackgroundStyle {
    #[serde(rename = "Color")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "Gradient")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gradient: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TileStyle {
    #[serde(rename = "BackgroundColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(rename = "Border")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border: Option<BorderStyle>,
    #[serde(rename = "BorderRadius")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_radius: Option<String>,
    #[serde(rename = "Padding")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BorderStyle {
    #[serde(rename = "Color")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "Show")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show: Option<bool>,
    #[serde(rename = "Width")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TileLayoutStyle {
    #[serde(rename = "Gutter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gutter: Option<GutterStyle>,
    #[serde(rename = "Margin")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<MarginStyle>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GutterStyle {
    #[serde(rename = "Show")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MarginStyle {
    #[serde(rename = "Show")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Typography {
    #[serde(rename = "AxisLabelFontConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis_label_font_configuration: Option<FontConfiguration>,
    #[serde(rename = "AxisTitleFontConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis_title_font_configuration: Option<FontConfiguration>,
    #[serde(rename = "ControlTitleFontConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_title_font_configuration: Option<ControlTitleFontConfiguration>,
    #[serde(rename = "DataLabelFontConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_label_font_configuration: Option<FontConfiguration>,
    #[serde(rename = "FontFamilies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_families: Option<Vec<Font>>,
    #[serde(rename = "LegendTitleFontConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend_title_font_configuration: Option<FontConfiguration>,
    #[serde(rename = "LegendValueFontConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend_value_font_configuration: Option<FontConfiguration>,
    #[serde(rename = "VisualSubtitleFontConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_subtitle_font_configuration: Option<VisualSubtitleFontConfiguration>,
    #[serde(rename = "VisualTitleFontConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_title_font_configuration: Option<VisualTitleFontConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ControlTitleFontConfiguration {
    #[serde(rename = "FontConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_configuration: Option<FontConfiguration>,
    #[serde(rename = "TextAlignment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_alignment: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Font {
    #[serde(rename = "FontFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VisualSubtitleFontConfiguration {
    #[serde(rename = "FontConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_configuration: Option<FontConfiguration>,
    #[serde(rename = "TextAlignment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_alignment: Option<String>,
    #[serde(rename = "TextTransform")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_transform: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VisualTitleFontConfiguration {
    #[serde(rename = "FontConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_configuration: Option<FontConfiguration>,
    #[serde(rename = "TextAlignment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_alignment: Option<String>,
    #[serde(rename = "TextTransform")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_transform: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UIColorPalette {
    #[serde(rename = "Accent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent: Option<String>,
    #[serde(rename = "AccentForeground")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_foreground: Option<String>,
    #[serde(rename = "Danger")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub danger: Option<String>,
    #[serde(rename = "DangerForeground")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub danger_foreground: Option<String>,
    #[serde(rename = "Dimension")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension: Option<String>,
    #[serde(rename = "DimensionForeground")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_foreground: Option<String>,
    #[serde(rename = "Measure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measure: Option<String>,
    #[serde(rename = "MeasureForeground")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measure_foreground: Option<String>,
    #[serde(rename = "PrimaryBackground")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_background: Option<String>,
    #[serde(rename = "PrimaryForeground")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_foreground: Option<String>,
    #[serde(rename = "SecondaryBackground")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_background: Option<String>,
    #[serde(rename = "SecondaryForeground")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_foreground: Option<String>,
    #[serde(rename = "Success")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<String>,
    #[serde(rename = "SuccessForeground")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_foreground: Option<String>,
    #[serde(rename = "Warning")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<String>,
    #[serde(rename = "WarningForeground")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning_foreground: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateThemeResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_status: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "ThemeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_id: Option<String>,
    #[serde(rename = "VersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTopicRefreshScheduleRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DatasetArn")]
    #[serde(default)]
    pub dataset_arn: String,
    #[serde(rename = "DatasetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_name: Option<String>,
    #[serde(rename = "RefreshSchedule")]
    #[serde(default)]
    pub refresh_schedule: TopicRefreshSchedule,
    #[serde(rename = "TopicId")]
    #[serde(default)]
    pub topic_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicRefreshSchedule {
    #[serde(rename = "BasedOnSpiceSchedule")]
    #[serde(default)]
    pub based_on_spice_schedule: bool,
    #[serde(rename = "IsEnabled")]
    #[serde(default)]
    pub is_enabled: bool,
    #[serde(rename = "RepeatAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_at: Option<String>,
    #[serde(rename = "StartingAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_at: Option<f64>,
    #[serde(rename = "Timezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(rename = "TopicScheduleType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_schedule_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTopicRefreshScheduleResponse {
    #[serde(rename = "DatasetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arn: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "TopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
    #[serde(rename = "TopicId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTopicRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "CustomInstructions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_instructions: Option<CustomInstructions>,
    #[serde(rename = "FolderArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_arns: Option<Vec<String>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Topic")]
    #[serde(default)]
    pub topic: TopicDetails,
    #[serde(rename = "TopicId")]
    #[serde(default)]
    pub topic_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomInstructions {
    #[serde(rename = "CustomInstructionsString")]
    #[serde(default)]
    pub custom_instructions_string: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicDetails {
    #[serde(rename = "ConfigOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_options: Option<TopicConfigOptions>,
    #[serde(rename = "DataSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sets: Option<Vec<DatasetMetadata>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "UserExperienceVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_experience_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicConfigOptions {
    #[serde(rename = "QBusinessInsightsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q_business_insights_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DatasetMetadata {
    #[serde(rename = "CalculatedFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculated_fields: Option<Vec<TopicCalculatedField>>,
    #[serde(rename = "Columns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<TopicColumn>>,
    #[serde(rename = "DataAggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_aggregation: Option<DataAggregation>,
    #[serde(rename = "DatasetArn")]
    #[serde(default)]
    pub dataset_arn: String,
    #[serde(rename = "DatasetDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_description: Option<String>,
    #[serde(rename = "DatasetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_name: Option<String>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<TopicFilter>>,
    #[serde(rename = "NamedEntities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_entities: Option<Vec<TopicNamedEntity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicCalculatedField {
    #[serde(rename = "Aggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<String>,
    #[serde(rename = "AllowedAggregations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_aggregations: Option<Vec<String>>,
    #[serde(rename = "CalculatedFieldDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculated_field_description: Option<String>,
    #[serde(rename = "CalculatedFieldName")]
    #[serde(default)]
    pub calculated_field_name: String,
    #[serde(rename = "CalculatedFieldSynonyms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculated_field_synonyms: Option<Vec<String>>,
    #[serde(rename = "CellValueSynonyms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell_value_synonyms: Option<Vec<CellValueSynonym>>,
    #[serde(rename = "ColumnDataRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_data_role: Option<String>,
    #[serde(rename = "ComparativeOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparative_order: Option<ComparativeOrder>,
    #[serde(rename = "DefaultFormatting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_formatting: Option<DefaultFormatting>,
    #[serde(rename = "DisableIndexing")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_indexing: Option<bool>,
    #[serde(rename = "Expression")]
    #[serde(default)]
    pub expression: String,
    #[serde(rename = "IsIncludedInTopic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_included_in_topic: Option<bool>,
    #[serde(rename = "NeverAggregateInFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub never_aggregate_in_filter: Option<bool>,
    #[serde(rename = "NonAdditive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_additive: Option<bool>,
    #[serde(rename = "NotAllowedAggregations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_allowed_aggregations: Option<Vec<String>>,
    #[serde(rename = "SemanticType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_type: Option<SemanticType>,
    #[serde(rename = "TimeGranularity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_granularity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CellValueSynonym {
    #[serde(rename = "CellValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell_value: Option<String>,
    #[serde(rename = "Synonyms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synonyms: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComparativeOrder {
    #[serde(rename = "SpecifedOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specifed_order: Option<Vec<String>>,
    #[serde(rename = "TreatUndefinedSpecifiedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treat_undefined_specified_values: Option<String>,
    #[serde(rename = "UseOrdering")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_ordering: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DefaultFormatting {
    #[serde(rename = "DisplayFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_format: Option<String>,
    #[serde(rename = "DisplayFormatOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_format_options: Option<DisplayFormatOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SemanticType {
    #[serde(rename = "FalseyCellValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub falsey_cell_value: Option<String>,
    #[serde(rename = "FalseyCellValueSynonyms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub falsey_cell_value_synonyms: Option<Vec<String>>,
    #[serde(rename = "SubTypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type_name: Option<String>,
    #[serde(rename = "TruthyCellValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truthy_cell_value: Option<String>,
    #[serde(rename = "TruthyCellValueSynonyms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truthy_cell_value_synonyms: Option<Vec<String>>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[serde(rename = "TypeParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_parameters: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicColumn {
    #[serde(rename = "Aggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<String>,
    #[serde(rename = "AllowedAggregations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_aggregations: Option<Vec<String>>,
    #[serde(rename = "CellValueSynonyms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cell_value_synonyms: Option<Vec<CellValueSynonym>>,
    #[serde(rename = "ColumnDataRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_data_role: Option<String>,
    #[serde(rename = "ColumnDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_description: Option<String>,
    #[serde(rename = "ColumnFriendlyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_friendly_name: Option<String>,
    #[serde(rename = "ColumnName")]
    #[serde(default)]
    pub column_name: String,
    #[serde(rename = "ColumnSynonyms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_synonyms: Option<Vec<String>>,
    #[serde(rename = "ComparativeOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comparative_order: Option<ComparativeOrder>,
    #[serde(rename = "DefaultFormatting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_formatting: Option<DefaultFormatting>,
    #[serde(rename = "DisableIndexing")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_indexing: Option<bool>,
    #[serde(rename = "IsIncludedInTopic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_included_in_topic: Option<bool>,
    #[serde(rename = "NeverAggregateInFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub never_aggregate_in_filter: Option<bool>,
    #[serde(rename = "NonAdditive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_additive: Option<bool>,
    #[serde(rename = "NotAllowedAggregations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_allowed_aggregations: Option<Vec<String>>,
    #[serde(rename = "SemanticType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_type: Option<SemanticType>,
    #[serde(rename = "TimeGranularity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_granularity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataAggregation {
    #[serde(rename = "DatasetRowDateGranularity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_row_date_granularity: Option<String>,
    #[serde(rename = "DefaultDateColumnName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_date_column_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicFilter {
    #[serde(rename = "CategoryFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_filter: Option<TopicCategoryFilter>,
    #[serde(rename = "DateRangeFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_range_filter: Option<TopicDateRangeFilter>,
    #[serde(rename = "FilterClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_class: Option<String>,
    #[serde(rename = "FilterDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_description: Option<String>,
    #[serde(rename = "FilterName")]
    #[serde(default)]
    pub filter_name: String,
    #[serde(rename = "FilterSynonyms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_synonyms: Option<Vec<String>>,
    #[serde(rename = "FilterType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_type: Option<String>,
    #[serde(rename = "NullFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub null_filter: Option<TopicNullFilter>,
    #[serde(rename = "NumericEqualityFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric_equality_filter: Option<TopicNumericEqualityFilter>,
    #[serde(rename = "NumericRangeFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric_range_filter: Option<TopicNumericRangeFilter>,
    #[serde(rename = "OperandFieldName")]
    #[serde(default)]
    pub operand_field_name: String,
    #[serde(rename = "RelativeDateFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_date_filter: Option<TopicRelativeDateFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicCategoryFilter {
    #[serde(rename = "CategoryFilterFunction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_filter_function: Option<String>,
    #[serde(rename = "CategoryFilterType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_filter_type: Option<String>,
    #[serde(rename = "Constant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant: Option<TopicCategoryFilterConstant>,
    #[serde(rename = "Inverse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inverse: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicCategoryFilterConstant {
    #[serde(rename = "CollectiveConstant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collective_constant: Option<CollectiveConstant>,
    #[serde(rename = "ConstantType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_type: Option<String>,
    #[serde(rename = "SingularConstant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub singular_constant: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CollectiveConstant {
    #[serde(rename = "ValueList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicDateRangeFilter {
    #[serde(rename = "Constant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant: Option<TopicRangeFilterConstant>,
    #[serde(rename = "Inclusive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusive: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicRangeFilterConstant {
    #[serde(rename = "ConstantType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_type: Option<String>,
    #[serde(rename = "RangeConstant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_constant: Option<RangeConstant>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RangeConstant {
    #[serde(rename = "Maximum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<String>,
    #[serde(rename = "Minimum")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicNullFilter {
    #[serde(rename = "Constant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant: Option<TopicSingularFilterConstant>,
    #[serde(rename = "Inverse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inverse: Option<bool>,
    #[serde(rename = "NullFilterType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub null_filter_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicSingularFilterConstant {
    #[serde(rename = "ConstantType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_type: Option<String>,
    #[serde(rename = "SingularConstant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub singular_constant: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicNumericEqualityFilter {
    #[serde(rename = "Aggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<String>,
    #[serde(rename = "Constant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant: Option<TopicSingularFilterConstant>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicNumericRangeFilter {
    #[serde(rename = "Aggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<String>,
    #[serde(rename = "Constant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant: Option<TopicRangeFilterConstant>,
    #[serde(rename = "Inclusive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusive: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicRelativeDateFilter {
    #[serde(rename = "Constant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant: Option<TopicSingularFilterConstant>,
    #[serde(rename = "RelativeDateFilterFunction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_date_filter_function: Option<String>,
    #[serde(rename = "TimeGranularity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_granularity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicNamedEntity {
    #[serde(rename = "Definition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<Vec<NamedEntityDefinition>>,
    #[serde(rename = "EntityDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_description: Option<String>,
    #[serde(rename = "EntityName")]
    #[serde(default)]
    pub entity_name: String,
    #[serde(rename = "EntitySynonyms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_synonyms: Option<Vec<String>>,
    #[serde(rename = "SemanticEntityType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_entity_type: Option<SemanticEntityType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NamedEntityDefinition {
    #[serde(rename = "FieldName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(rename = "Metric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric: Option<NamedEntityDefinitionMetric>,
    #[serde(rename = "PropertyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_name: Option<String>,
    #[serde(rename = "PropertyRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_role: Option<String>,
    #[serde(rename = "PropertyUsage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_usage: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NamedEntityDefinitionMetric {
    #[serde(rename = "Aggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<String>,
    #[serde(rename = "AggregationFunctionParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_function_parameters: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SemanticEntityType {
    #[serde(rename = "SubTypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type_name: Option<String>,
    #[serde(rename = "TypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[serde(rename = "TypeParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_parameters: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTopicResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "RefreshArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_arn: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "TopicId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVPCConnectionRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DnsResolvers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_resolvers: Option<Vec<String>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    pub security_group_ids: Vec<String>,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    pub subnet_ids: Vec<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VPCConnectionId")]
    #[serde(default)]
    pub v_p_c_connection_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVPCConnectionResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "AvailabilityStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_status: Option<String>,
    #[serde(rename = "CreationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_status: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "VPCConnectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_p_c_connection_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAccountCustomPermissionRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAccountCustomPermissionResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAccountCustomizationRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAccountCustomizationResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAccountSubscriptionRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAccountSubscriptionResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteActionConnectorRequest {
    #[serde(rename = "ActionConnectorId")]
    #[serde(default)]
    pub action_connector_id: String,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteActionConnectorResponse {
    #[serde(rename = "ActionConnectorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_connector_id: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAnalysisRequest {
    #[serde(rename = "AnalysisId")]
    #[serde(default)]
    pub analysis_id: String,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "ForceDeleteWithoutRecovery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_delete_without_recovery: Option<bool>,
    #[serde(rename = "RecoveryWindowInDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_window_in_days: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAnalysisResponse {
    #[serde(rename = "AnalysisId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_id: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "DeletionTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_time: Option<f64>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBrandAssignmentRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBrandAssignmentResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBrandRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "BrandId")]
    #[serde(default)]
    pub brand_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBrandResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCustomPermissionsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "CustomPermissionsName")]
    #[serde(default)]
    pub custom_permissions_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCustomPermissionsResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDashboardRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DashboardId")]
    #[serde(default)]
    pub dashboard_id: String,
    #[serde(rename = "VersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDashboardResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "DashboardId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_id: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDataSetRefreshPropertiesRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DataSetId")]
    #[serde(default)]
    pub data_set_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDataSetRefreshPropertiesResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDataSetRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DataSetId")]
    #[serde(default)]
    pub data_set_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDataSetResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "DataSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDataSourceRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DataSourceId")]
    #[serde(default)]
    pub data_source_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDataSourceResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "DataSourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDefaultQBusinessApplicationRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDefaultQBusinessApplicationResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFolderMembershipRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "FolderId")]
    #[serde(default)]
    pub folder_id: String,
    #[serde(rename = "MemberId")]
    #[serde(default)]
    pub member_id: String,
    #[serde(rename = "MemberType")]
    #[serde(default)]
    pub member_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFolderMembershipResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFolderRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "FolderId")]
    #[serde(default)]
    pub folder_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFolderResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "FolderId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteGroupMembershipRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    pub group_name: String,
    #[serde(rename = "MemberName")]
    #[serde(default)]
    pub member_name: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteGroupMembershipResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteGroupRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    pub group_name: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteGroupResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIAMPolicyAssignmentRequest {
    #[serde(rename = "AssignmentName")]
    #[serde(default)]
    pub assignment_name: String,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIAMPolicyAssignmentResponse {
    #[serde(rename = "AssignmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_name: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIdentityPropagationConfigRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Service")]
    #[serde(default)]
    pub service: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIdentityPropagationConfigResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteNamespaceRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteNamespaceResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRefreshScheduleRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DataSetId")]
    #[serde(default)]
    pub data_set_id: String,
    #[serde(rename = "ScheduleId")]
    #[serde(default)]
    pub schedule_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRefreshScheduleResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "ScheduleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRoleCustomPermissionRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "Role")]
    #[serde(default)]
    pub role: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRoleCustomPermissionResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRoleMembershipRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "MemberName")]
    #[serde(default)]
    pub member_name: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "Role")]
    #[serde(default)]
    pub role: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRoleMembershipResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTemplateAliasRequest {
    #[serde(rename = "AliasName")]
    #[serde(default)]
    pub alias_name: String,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "TemplateId")]
    #[serde(default)]
    pub template_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTemplateAliasResponse {
    #[serde(rename = "AliasName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "TemplateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTemplateRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "TemplateId")]
    #[serde(default)]
    pub template_id: String,
    #[serde(rename = "VersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTemplateResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "TemplateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteThemeAliasRequest {
    #[serde(rename = "AliasName")]
    #[serde(default)]
    pub alias_name: String,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "ThemeId")]
    #[serde(default)]
    pub theme_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteThemeAliasResponse {
    #[serde(rename = "AliasName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "ThemeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteThemeRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "ThemeId")]
    #[serde(default)]
    pub theme_id: String,
    #[serde(rename = "VersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteThemeResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "ThemeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTopicRefreshScheduleRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DatasetId")]
    #[serde(default)]
    pub dataset_id: String,
    #[serde(rename = "TopicId")]
    #[serde(default)]
    pub topic_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTopicRefreshScheduleResponse {
    #[serde(rename = "DatasetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arn: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "TopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
    #[serde(rename = "TopicId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTopicRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "TopicId")]
    #[serde(default)]
    pub topic_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTopicResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "TopicId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUserByPrincipalIdRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "PrincipalId")]
    #[serde(default)]
    pub principal_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUserByPrincipalIdResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUserCustomPermissionRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUserCustomPermissionResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUserRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUserResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVPCConnectionRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "VPCConnectionId")]
    #[serde(default)]
    pub v_p_c_connection_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVPCConnectionResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "AvailabilityStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_status: Option<String>,
    #[serde(rename = "DeletionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_status: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "VPCConnectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_p_c_connection_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAccountCustomPermissionRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAccountCustomPermissionResponse {
    #[serde(rename = "CustomPermissionsName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_permissions_name: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAccountCustomizationRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "Resolved")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAccountCustomizationResponse {
    #[serde(rename = "AccountCustomization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_customization: Option<AccountCustomization>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAccountSettingsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAccountSettingsResponse {
    #[serde(rename = "AccountSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_settings: Option<AccountSettings>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountSettings {
    #[serde(rename = "AccountName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[serde(rename = "DefaultNamespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_namespace: Option<String>,
    #[serde(rename = "Edition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[serde(rename = "NotificationEmail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_email: Option<String>,
    #[serde(rename = "PublicSharingEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_sharing_enabled: Option<bool>,
    #[serde(rename = "TerminationProtectionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_protection_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAccountSubscriptionRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAccountSubscriptionResponse {
    #[serde(rename = "AccountInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_info: Option<AccountInfo>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountInfo {
    #[serde(rename = "AccountName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[serde(rename = "AccountSubscriptionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_subscription_status: Option<String>,
    #[serde(rename = "AuthenticationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    #[serde(rename = "Edition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[serde(rename = "IAMIdentityCenterInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_a_m_identity_center_instance_arn: Option<String>,
    #[serde(rename = "NotificationEmail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_email: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeActionConnectorPermissionsRequest {
    #[serde(rename = "ActionConnectorId")]
    #[serde(default)]
    pub action_connector_id: String,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeActionConnectorPermissionsResponse {
    #[serde(rename = "ActionConnectorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_connector_id: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeActionConnectorRequest {
    #[serde(rename = "ActionConnectorId")]
    #[serde(default)]
    pub action_connector_id: String,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeActionConnectorResponse {
    #[serde(rename = "ActionConnector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_connector: Option<ActionConnector>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionConnector {
    #[serde(rename = "ActionConnectorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_connector_id: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "AuthenticationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_config: Option<ReadAuthConfig>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EnabledActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_actions: Option<Vec<String>>,
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ActionConnectorError>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "VpcConnectionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_connection_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReadAuthConfig {
    #[serde(rename = "AuthenticationMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_metadata: Option<ReadAuthenticationMetadata>,
    #[serde(rename = "AuthenticationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReadAuthenticationMetadata {
    #[serde(rename = "ApiKeyConnectionMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_connection_metadata: Option<ReadAPIKeyConnectionMetadata>,
    #[serde(rename = "AuthorizationCodeGrantMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code_grant_metadata: Option<ReadAuthorizationCodeGrantMetadata>,
    #[serde(rename = "BasicAuthConnectionMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth_connection_metadata: Option<ReadBasicAuthConnectionMetadata>,
    #[serde(rename = "ClientCredentialsGrantMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_credentials_grant_metadata: Option<ReadClientCredentialsGrantMetadata>,
    #[serde(rename = "IamConnectionMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_connection_metadata: Option<ReadIamConnectionMetadata>,
    #[serde(rename = "NoneConnectionMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub none_connection_metadata: Option<ReadNoneConnectionMetadata>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReadAPIKeyConnectionMetadata {
    #[serde(rename = "BaseEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_endpoint: Option<String>,
    #[serde(rename = "Email")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReadAuthorizationCodeGrantMetadata {
    #[serde(rename = "AuthorizationCodeGrantCredentialsSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code_grant_credentials_source: Option<String>,
    #[serde(rename = "BaseEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_endpoint: Option<String>,
    #[serde(rename = "ReadAuthorizationCodeGrantCredentialsDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_authorization_code_grant_credentials_details:
        Option<ReadAuthorizationCodeGrantCredentialsDetails>,
    #[serde(rename = "RedirectUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReadAuthorizationCodeGrantCredentialsDetails {
    #[serde(rename = "ReadAuthorizationCodeGrantDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_authorization_code_grant_details: Option<ReadAuthorizationCodeGrantDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReadAuthorizationCodeGrantDetails {
    #[serde(rename = "AuthorizationEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_endpoint: Option<String>,
    #[serde(rename = "ClientId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "TokenEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_endpoint: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReadBasicAuthConnectionMetadata {
    #[serde(rename = "BaseEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_endpoint: Option<String>,
    #[serde(rename = "Username")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReadClientCredentialsGrantMetadata {
    #[serde(rename = "BaseEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_endpoint: Option<String>,
    #[serde(rename = "ClientCredentialsSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_credentials_source: Option<String>,
    #[serde(rename = "ReadClientCredentialsDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_client_credentials_details: Option<ReadClientCredentialsDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReadClientCredentialsDetails {
    #[serde(rename = "ReadClientCredentialsGrantDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_client_credentials_grant_details: Option<ReadClientCredentialsGrantDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReadClientCredentialsGrantDetails {
    #[serde(rename = "ClientId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "TokenEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_endpoint: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReadIamConnectionMetadata {
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "SourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReadNoneConnectionMetadata {
    #[serde(rename = "BaseEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_endpoint: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionConnectorError {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAnalysisDefinitionRequest {
    #[serde(rename = "AnalysisId")]
    #[serde(default)]
    pub analysis_id: String,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAnalysisDefinitionResponse {
    #[serde(rename = "AnalysisId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_id: Option<String>,
    #[serde(rename = "Definition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<AnalysisDefinition>,
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<AnalysisError>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "ResourceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_status: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "ThemeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalysisError {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "ViolatedEntities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violated_entities: Option<Vec<Entity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Entity {
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAnalysisPermissionsRequest {
    #[serde(rename = "AnalysisId")]
    #[serde(default)]
    pub analysis_id: String,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAnalysisPermissionsResponse {
    #[serde(rename = "AnalysisArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_arn: Option<String>,
    #[serde(rename = "AnalysisId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_id: Option<String>,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAnalysisRequest {
    #[serde(rename = "AnalysisId")]
    #[serde(default)]
    pub analysis_id: String,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAnalysisResponse {
    #[serde(rename = "Analysis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis: Option<Analysis>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Analysis {
    #[serde(rename = "AnalysisId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_id: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "DataSetArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_arns: Option<Vec<String>>,
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<AnalysisError>>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Sheets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheets: Option<Vec<Sheet>>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "ThemeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Sheet {
    #[serde(rename = "Images")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<SheetImage>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SheetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAssetBundleExportJobRequest {
    #[serde(rename = "AssetBundleExportJobId")]
    #[serde(default)]
    pub asset_bundle_export_job_id: String,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAssetBundleExportJobResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "AssetBundleExportJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_bundle_export_job_id: Option<String>,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    #[serde(rename = "CloudFormationOverridePropertyConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_formation_override_property_configuration:
        Option<AssetBundleCloudFormationOverridePropertyConfiguration>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "DownloadUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<AssetBundleExportJobError>>,
    #[serde(rename = "ExportFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_format: Option<String>,
    #[serde(rename = "IncludeAllDependencies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_all_dependencies: Option<bool>,
    #[serde(rename = "IncludeFolderMembers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_folder_members: Option<String>,
    #[serde(rename = "IncludeFolderMemberships")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_folder_memberships: Option<bool>,
    #[serde(rename = "IncludePermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_permissions: Option<bool>,
    #[serde(rename = "IncludeTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_tags: Option<bool>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "ResourceArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arns: Option<Vec<String>>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "ValidationStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_strategy: Option<AssetBundleExportJobValidationStrategy>,
    #[serde(rename = "Warnings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<AssetBundleExportJobWarning>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleCloudFormationOverridePropertyConfiguration {
    #[serde(rename = "Analyses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyses: Option<Vec<AssetBundleExportJobAnalysisOverrideProperties>>,
    #[serde(rename = "Dashboards")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboards: Option<Vec<AssetBundleExportJobDashboardOverrideProperties>>,
    #[serde(rename = "DataSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sets: Option<Vec<AssetBundleExportJobDataSetOverrideProperties>>,
    #[serde(rename = "DataSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<Vec<AssetBundleExportJobDataSourceOverrideProperties>>,
    #[serde(rename = "Folders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folders: Option<Vec<AssetBundleExportJobFolderOverrideProperties>>,
    #[serde(rename = "RefreshSchedules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_schedules: Option<Vec<AssetBundleExportJobRefreshScheduleOverrideProperties>>,
    #[serde(rename = "ResourceIdOverrideConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id_override_configuration:
        Option<AssetBundleExportJobResourceIdOverrideConfiguration>,
    #[serde(rename = "Themes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub themes: Option<Vec<AssetBundleExportJobThemeOverrideProperties>>,
    #[serde(rename = "VPCConnections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_p_c_connections: Option<Vec<AssetBundleExportJobVPCConnectionOverrideProperties>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleExportJobAnalysisOverrideProperties {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "Properties")]
    #[serde(default)]
    pub properties: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleExportJobDashboardOverrideProperties {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "Properties")]
    #[serde(default)]
    pub properties: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleExportJobDataSetOverrideProperties {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "Properties")]
    #[serde(default)]
    pub properties: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleExportJobDataSourceOverrideProperties {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "Properties")]
    #[serde(default)]
    pub properties: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleExportJobFolderOverrideProperties {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "Properties")]
    #[serde(default)]
    pub properties: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleExportJobRefreshScheduleOverrideProperties {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "Properties")]
    #[serde(default)]
    pub properties: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleExportJobResourceIdOverrideConfiguration {
    #[serde(rename = "PrefixForAllResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_for_all_resources: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleExportJobThemeOverrideProperties {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "Properties")]
    #[serde(default)]
    pub properties: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleExportJobVPCConnectionOverrideProperties {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "Properties")]
    #[serde(default)]
    pub properties: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleExportJobError {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleExportJobValidationStrategy {
    #[serde(rename = "StrictModeForAllResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict_mode_for_all_resources: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleExportJobWarning {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAssetBundleImportJobRequest {
    #[serde(rename = "AssetBundleImportJobId")]
    #[serde(default)]
    pub asset_bundle_import_job_id: String,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAssetBundleImportJobResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "AssetBundleImportJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_bundle_import_job_id: Option<String>,
    #[serde(rename = "AssetBundleImportSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_bundle_import_source: Option<AssetBundleImportSourceDescription>,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<AssetBundleImportJobError>>,
    #[serde(rename = "FailureAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_action: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "OverrideParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_parameters: Option<AssetBundleImportJobOverrideParameters>,
    #[serde(rename = "OverridePermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_permissions: Option<AssetBundleImportJobOverridePermissions>,
    #[serde(rename = "OverrideTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_tags: Option<AssetBundleImportJobOverrideTags>,
    #[serde(rename = "OverrideValidationStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_validation_strategy: Option<AssetBundleImportJobOverrideValidationStrategy>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "RollbackErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_errors: Option<Vec<AssetBundleImportJobError>>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "Warnings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<AssetBundleImportJobWarning>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleImportSourceDescription {
    #[serde(rename = "Body")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "S3Uri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleImportJobError {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleImportJobOverrideParameters {
    #[serde(rename = "Analyses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyses: Option<Vec<AssetBundleImportJobAnalysisOverrideParameters>>,
    #[serde(rename = "Dashboards")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboards: Option<Vec<AssetBundleImportJobDashboardOverrideParameters>>,
    #[serde(rename = "DataSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sets: Option<Vec<AssetBundleImportJobDataSetOverrideParameters>>,
    #[serde(rename = "DataSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<Vec<AssetBundleImportJobDataSourceOverrideParameters>>,
    #[serde(rename = "Folders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folders: Option<Vec<AssetBundleImportJobFolderOverrideParameters>>,
    #[serde(rename = "RefreshSchedules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_schedules: Option<Vec<AssetBundleImportJobRefreshScheduleOverrideParameters>>,
    #[serde(rename = "ResourceIdOverrideConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id_override_configuration:
        Option<AssetBundleImportJobResourceIdOverrideConfiguration>,
    #[serde(rename = "Themes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub themes: Option<Vec<AssetBundleImportJobThemeOverrideParameters>>,
    #[serde(rename = "VPCConnections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_p_c_connections: Option<Vec<AssetBundleImportJobVPCConnectionOverrideParameters>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleImportJobAnalysisOverrideParameters {
    #[serde(rename = "AnalysisId")]
    #[serde(default)]
    pub analysis_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleImportJobDashboardOverrideParameters {
    #[serde(rename = "DashboardId")]
    #[serde(default)]
    pub dashboard_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleImportJobDataSetOverrideParameters {
    #[serde(rename = "DataSetId")]
    #[serde(default)]
    pub data_set_id: String,
    #[serde(rename = "DataSetRefreshProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_refresh_properties: Option<DataSetRefreshProperties>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSetRefreshProperties {
    #[serde(rename = "FailureConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_configuration: Option<RefreshFailureConfiguration>,
    #[serde(rename = "RefreshConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_configuration: Option<RefreshConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RefreshFailureConfiguration {
    #[serde(rename = "EmailAlert")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_alert: Option<RefreshFailureEmailAlert>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RefreshFailureEmailAlert {
    #[serde(rename = "AlertStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RefreshConfiguration {
    #[serde(rename = "IncrementalRefresh")]
    #[serde(default)]
    pub incremental_refresh: IncrementalRefresh,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IncrementalRefresh {
    #[serde(rename = "LookbackWindow")]
    #[serde(default)]
    pub lookback_window: LookbackWindow,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LookbackWindow {
    #[serde(rename = "ColumnName")]
    #[serde(default)]
    pub column_name: String,
    #[serde(rename = "Size")]
    #[serde(default)]
    pub size: i64,
    #[serde(rename = "SizeUnit")]
    #[serde(default)]
    pub size_unit: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleImportJobDataSourceOverrideParameters {
    #[serde(rename = "Credentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<AssetBundleImportJobDataSourceCredentials>,
    #[serde(rename = "DataSourceId")]
    #[serde(default)]
    pub data_source_id: String,
    #[serde(rename = "DataSourceParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_parameters: Option<DataSourceParameters>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SslProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_properties: Option<SslProperties>,
    #[serde(rename = "VpcConnectionProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_connection_properties: Option<VpcConnectionProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleImportJobDataSourceCredentials {
    #[serde(rename = "CredentialPair")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential_pair: Option<AssetBundleImportJobDataSourceCredentialPair>,
    #[serde(rename = "SecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleImportJobDataSourceCredentialPair {
    #[serde(rename = "Password")]
    #[serde(default)]
    pub password: String,
    #[serde(rename = "Username")]
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleImportJobFolderOverrideParameters {
    #[serde(rename = "FolderId")]
    #[serde(default)]
    pub folder_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ParentFolderArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleImportJobRefreshScheduleOverrideParameters {
    #[serde(rename = "DataSetId")]
    #[serde(default)]
    pub data_set_id: String,
    #[serde(rename = "ScheduleId")]
    #[serde(default)]
    pub schedule_id: String,
    #[serde(rename = "StartAfterDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_after_date_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleImportJobResourceIdOverrideConfiguration {
    #[serde(rename = "PrefixForAllResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix_for_all_resources: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleImportJobThemeOverrideParameters {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ThemeId")]
    #[serde(default)]
    pub theme_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleImportJobVPCConnectionOverrideParameters {
    #[serde(rename = "DnsResolvers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_resolvers: Option<Vec<String>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(rename = "VPCConnectionId")]
    #[serde(default)]
    pub v_p_c_connection_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleImportJobOverridePermissions {
    #[serde(rename = "Analyses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyses: Option<Vec<AssetBundleImportJobAnalysisOverridePermissions>>,
    #[serde(rename = "Dashboards")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboards: Option<Vec<AssetBundleImportJobDashboardOverridePermissions>>,
    #[serde(rename = "DataSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sets: Option<Vec<AssetBundleImportJobDataSetOverridePermissions>>,
    #[serde(rename = "DataSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<Vec<AssetBundleImportJobDataSourceOverridePermissions>>,
    #[serde(rename = "Folders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folders: Option<Vec<AssetBundleImportJobFolderOverridePermissions>>,
    #[serde(rename = "Themes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub themes: Option<Vec<AssetBundleImportJobThemeOverridePermissions>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleImportJobAnalysisOverridePermissions {
    #[serde(rename = "AnalysisIds")]
    #[serde(default)]
    pub analysis_ids: Vec<String>,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    pub permissions: AssetBundleResourcePermissions,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleResourcePermissions {
    #[serde(rename = "Actions")]
    #[serde(default)]
    pub actions: Vec<String>,
    #[serde(rename = "Principals")]
    #[serde(default)]
    pub principals: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleImportJobDashboardOverridePermissions {
    #[serde(rename = "DashboardIds")]
    #[serde(default)]
    pub dashboard_ids: Vec<String>,
    #[serde(rename = "LinkSharingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_sharing_configuration: Option<AssetBundleResourceLinkSharingConfiguration>,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<AssetBundleResourcePermissions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleResourceLinkSharingConfiguration {
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<AssetBundleResourcePermissions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleImportJobDataSetOverridePermissions {
    #[serde(rename = "DataSetIds")]
    #[serde(default)]
    pub data_set_ids: Vec<String>,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    pub permissions: AssetBundleResourcePermissions,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleImportJobDataSourceOverridePermissions {
    #[serde(rename = "DataSourceIds")]
    #[serde(default)]
    pub data_source_ids: Vec<String>,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    pub permissions: AssetBundleResourcePermissions,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleImportJobFolderOverridePermissions {
    #[serde(rename = "FolderIds")]
    #[serde(default)]
    pub folder_ids: Vec<String>,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<AssetBundleResourcePermissions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleImportJobThemeOverridePermissions {
    #[serde(rename = "Permissions")]
    #[serde(default)]
    pub permissions: AssetBundleResourcePermissions,
    #[serde(rename = "ThemeIds")]
    #[serde(default)]
    pub theme_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleImportJobOverrideTags {
    #[serde(rename = "Analyses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyses: Option<Vec<AssetBundleImportJobAnalysisOverrideTags>>,
    #[serde(rename = "Dashboards")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboards: Option<Vec<AssetBundleImportJobDashboardOverrideTags>>,
    #[serde(rename = "DataSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sets: Option<Vec<AssetBundleImportJobDataSetOverrideTags>>,
    #[serde(rename = "DataSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<Vec<AssetBundleImportJobDataSourceOverrideTags>>,
    #[serde(rename = "Folders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folders: Option<Vec<AssetBundleImportJobFolderOverrideTags>>,
    #[serde(rename = "Themes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub themes: Option<Vec<AssetBundleImportJobThemeOverrideTags>>,
    #[serde(rename = "VPCConnections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_p_c_connections: Option<Vec<AssetBundleImportJobVPCConnectionOverrideTags>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleImportJobAnalysisOverrideTags {
    #[serde(rename = "AnalysisIds")]
    #[serde(default)]
    pub analysis_ids: Vec<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleImportJobDashboardOverrideTags {
    #[serde(rename = "DashboardIds")]
    #[serde(default)]
    pub dashboard_ids: Vec<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleImportJobDataSetOverrideTags {
    #[serde(rename = "DataSetIds")]
    #[serde(default)]
    pub data_set_ids: Vec<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleImportJobDataSourceOverrideTags {
    #[serde(rename = "DataSourceIds")]
    #[serde(default)]
    pub data_source_ids: Vec<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleImportJobFolderOverrideTags {
    #[serde(rename = "FolderIds")]
    #[serde(default)]
    pub folder_ids: Vec<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleImportJobThemeOverrideTags {
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
    #[serde(rename = "ThemeIds")]
    #[serde(default)]
    pub theme_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleImportJobVPCConnectionOverrideTags {
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
    #[serde(rename = "VPCConnectionIds")]
    #[serde(default)]
    pub v_p_c_connection_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleImportJobOverrideValidationStrategy {
    #[serde(rename = "StrictModeForAllResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict_mode_for_all_resources: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleImportJobWarning {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAutomationJobRequest {
    #[serde(rename = "AutomationGroupId")]
    #[serde(default)]
    pub automation_group_id: String,
    #[serde(rename = "AutomationId")]
    #[serde(default)]
    pub automation_id: String,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "IncludeInputPayload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_input_payload: Option<bool>,
    #[serde(rename = "IncludeOutputPayload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_output_payload: Option<bool>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeAutomationJobResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "EndedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<f64>,
    #[serde(rename = "InputPayload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_payload: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "OutputPayload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_payload: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "StartedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBrandAssignmentRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBrandAssignmentResponse {
    #[serde(rename = "BrandArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_arn: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBrandPublishedVersionRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "BrandId")]
    #[serde(default)]
    pub brand_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBrandPublishedVersionResponse {
    #[serde(rename = "BrandDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_definition: Option<BrandDefinition>,
    #[serde(rename = "BrandDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_detail: Option<BrandDetail>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBrandRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "BrandId")]
    #[serde(default)]
    pub brand_id: String,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBrandResponse {
    #[serde(rename = "BrandDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_definition: Option<BrandDefinition>,
    #[serde(rename = "BrandDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_detail: Option<BrandDetail>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCustomPermissionsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "CustomPermissionsName")]
    #[serde(default)]
    pub custom_permissions_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCustomPermissionsResponse {
    #[serde(rename = "CustomPermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_permissions: Option<CustomPermissions>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomPermissions {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Capabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Capabilities>,
    #[serde(rename = "CustomPermissionsName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_permissions_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDashboardDefinitionRequest {
    #[serde(rename = "AliasName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DashboardId")]
    #[serde(default)]
    pub dashboard_id: String,
    #[serde(rename = "VersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDashboardDefinitionResponse {
    #[serde(rename = "DashboardId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_id: Option<String>,
    #[serde(rename = "DashboardPublishOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_publish_options: Option<DashboardPublishOptions>,
    #[serde(rename = "Definition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<DashboardVersionDefinition>,
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<DashboardError>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "ResourceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_status: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "ThemeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DashboardError {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "ViolatedEntities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violated_entities: Option<Vec<Entity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDashboardPermissionsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DashboardId")]
    #[serde(default)]
    pub dashboard_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDashboardPermissionsResponse {
    #[serde(rename = "DashboardArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_arn: Option<String>,
    #[serde(rename = "DashboardId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_id: Option<String>,
    #[serde(rename = "LinkSharingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_sharing_configuration: Option<LinkSharingConfiguration>,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDashboardRequest {
    #[serde(rename = "AliasName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DashboardId")]
    #[serde(default)]
    pub dashboard_id: String,
    #[serde(rename = "VersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDashboardResponse {
    #[serde(rename = "Dashboard")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard: Option<Dashboard>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Dashboard {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "DashboardId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_id: Option<String>,
    #[serde(rename = "LastPublishedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_published_time: Option<f64>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "LinkEntities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_entities: Option<Vec<String>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<DashboardVersion>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DashboardVersion {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "DataSetArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_arns: Option<Vec<String>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<DashboardError>>,
    #[serde(rename = "Sheets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheets: Option<Vec<Sheet>>,
    #[serde(rename = "SourceEntityArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_entity_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "ThemeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_arn: Option<String>,
    #[serde(rename = "VersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDashboardSnapshotJobRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DashboardId")]
    #[serde(default)]
    pub dashboard_id: String,
    #[serde(rename = "SnapshotJobId")]
    #[serde(default)]
    pub snapshot_job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDashboardSnapshotJobResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "DashboardId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_id: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "SnapshotConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_configuration: Option<SnapshotConfiguration>,
    #[serde(rename = "SnapshotJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_job_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "UserConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_configuration: Option<SnapshotUserConfigurationRedacted>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnapshotConfiguration {
    #[serde(rename = "DestinationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_configuration: Option<SnapshotDestinationConfiguration>,
    #[serde(rename = "FileGroups")]
    #[serde(default)]
    pub file_groups: Vec<SnapshotFileGroup>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Parameters>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnapshotDestinationConfiguration {
    #[serde(rename = "S3Destinations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_destinations: Option<Vec<SnapshotS3DestinationConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnapshotS3DestinationConfiguration {
    #[serde(rename = "BucketConfiguration")]
    #[serde(default)]
    pub bucket_configuration: S3BucketConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3BucketConfiguration {
    #[serde(rename = "BucketName")]
    #[serde(default)]
    pub bucket_name: String,
    #[serde(rename = "BucketPrefix")]
    #[serde(default)]
    pub bucket_prefix: String,
    #[serde(rename = "BucketRegion")]
    #[serde(default)]
    pub bucket_region: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnapshotFileGroup {
    #[serde(rename = "Files")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<SnapshotFile>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnapshotFile {
    #[serde(rename = "FormatType")]
    #[serde(default)]
    pub format_type: String,
    #[serde(rename = "SheetSelections")]
    #[serde(default)]
    pub sheet_selections: Vec<SnapshotFileSheetSelection>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnapshotFileSheetSelection {
    #[serde(rename = "SelectionScope")]
    #[serde(default)]
    pub selection_scope: String,
    #[serde(rename = "SheetId")]
    #[serde(default)]
    pub sheet_id: String,
    #[serde(rename = "VisualIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnapshotUserConfigurationRedacted {
    #[serde(rename = "AnonymousUsers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous_users: Option<Vec<SnapshotAnonymousUserRedacted>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnapshotAnonymousUserRedacted {
    #[serde(rename = "RowLevelPermissionTagKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_level_permission_tag_keys: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDashboardSnapshotJobResultRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DashboardId")]
    #[serde(default)]
    pub dashboard_id: String,
    #[serde(rename = "SnapshotJobId")]
    #[serde(default)]
    pub snapshot_job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDashboardSnapshotJobResultResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "ErrorInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_info: Option<SnapshotJobErrorInfo>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Result")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<SnapshotJobResult>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnapshotJobErrorInfo {
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "ErrorType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnapshotJobResult {
    #[serde(rename = "AnonymousUsers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous_users: Option<Vec<AnonymousUserSnapshotJobResult>>,
    #[serde(rename = "RegisteredUsers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_users: Option<Vec<RegisteredUserSnapshotJobResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnonymousUserSnapshotJobResult {
    #[serde(rename = "FileGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_groups: Option<Vec<SnapshotJobResultFileGroup>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnapshotJobResultFileGroup {
    #[serde(rename = "Files")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<SnapshotFile>>,
    #[serde(rename = "S3Results")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_results: Option<Vec<SnapshotJobS3Result>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnapshotJobS3Result {
    #[serde(rename = "ErrorInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_info: Option<Vec<SnapshotJobResultErrorInfo>>,
    #[serde(rename = "S3DestinationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_destination_configuration: Option<SnapshotS3DestinationConfiguration>,
    #[serde(rename = "S3Uri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnapshotJobResultErrorInfo {
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "ErrorType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisteredUserSnapshotJobResult {
    #[serde(rename = "FileGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_groups: Option<Vec<SnapshotJobResultFileGroup>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDashboardsQAConfigurationRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDashboardsQAConfigurationResponse {
    #[serde(rename = "DashboardsQAStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboards_q_a_status: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDataSetPermissionsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DataSetId")]
    #[serde(default)]
    pub data_set_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDataSetPermissionsResponse {
    #[serde(rename = "DataSetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_arn: Option<String>,
    #[serde(rename = "DataSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDataSetRefreshPropertiesRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DataSetId")]
    #[serde(default)]
    pub data_set_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDataSetRefreshPropertiesResponse {
    #[serde(rename = "DataSetRefreshProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_refresh_properties: Option<DataSetRefreshProperties>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDataSetRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DataSetId")]
    #[serde(default)]
    pub data_set_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDataSetResponse {
    #[serde(rename = "DataSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set: Option<DataSet>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSet {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ColumnGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_groups: Option<Vec<ColumnGroup>>,
    #[serde(rename = "ColumnLevelPermissionRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_level_permission_rules: Option<Vec<ColumnLevelPermissionRule>>,
    #[serde(rename = "ConsumedSpiceCapacityInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_spice_capacity_in_bytes: Option<i64>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "DataPrepConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_prep_configuration: Option<DataPrepConfiguration>,
    #[serde(rename = "DataSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    #[serde(rename = "DataSetUsageConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_usage_configuration: Option<DataSetUsageConfiguration>,
    #[serde(rename = "DatasetParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_parameters: Option<Vec<DatasetParameter>>,
    #[serde(rename = "FieldFolders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_folders: Option<std::collections::HashMap<String, FieldFolder>>,
    #[serde(rename = "ImportMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_mode: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "LogicalTableMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_table_map: Option<std::collections::HashMap<String, LogicalTable>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OutputColumns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_columns: Option<Vec<OutputColumn>>,
    #[serde(rename = "PerformanceConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_configuration: Option<PerformanceConfiguration>,
    #[serde(rename = "PhysicalTableMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_table_map: Option<std::collections::HashMap<String, PhysicalTable>>,
    #[serde(rename = "RowLevelPermissionDataSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_level_permission_data_set: Option<RowLevelPermissionDataSet>,
    #[serde(rename = "RowLevelPermissionTagConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_level_permission_tag_configuration: Option<RowLevelPermissionTagConfiguration>,
    #[serde(rename = "SemanticModelConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_model_configuration: Option<SemanticModelConfiguration>,
    #[serde(rename = "UseAs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_as: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutputColumn {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SubType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDataSourcePermissionsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DataSourceId")]
    #[serde(default)]
    pub data_source_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDataSourcePermissionsResponse {
    #[serde(rename = "DataSourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_arn: Option<String>,
    #[serde(rename = "DataSourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDataSourceRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DataSourceId")]
    #[serde(default)]
    pub data_source_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDataSourceResponse {
    #[serde(rename = "DataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<DataSource>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSource {
    #[serde(rename = "AlternateDataSourceParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate_data_source_parameters: Option<Vec<DataSourceParameters>>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "DataSourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,
    #[serde(rename = "DataSourceParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_parameters: Option<DataSourceParameters>,
    #[serde(rename = "ErrorInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_info: Option<DataSourceErrorInfo>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
    #[serde(rename = "SslProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_properties: Option<SslProperties>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "VpcConnectionProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_connection_properties: Option<VpcConnectionProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSourceErrorInfo {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDefaultQBusinessApplicationRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDefaultQBusinessApplicationResponse {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFolderPermissionsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "FolderId")]
    #[serde(default)]
    pub folder_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFolderPermissionsResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "FolderId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFolderRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "FolderId")]
    #[serde(default)]
    pub folder_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFolderResolvedPermissionsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "FolderId")]
    #[serde(default)]
    pub folder_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFolderResolvedPermissionsResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "FolderId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFolderResponse {
    #[serde(rename = "Folder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder: Option<Folder>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Folder {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "FolderId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    #[serde(rename = "FolderPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_path: Option<Vec<String>>,
    #[serde(rename = "FolderType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_type: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SharingModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharing_model: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeGroupMembershipRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    pub group_name: String,
    #[serde(rename = "MemberName")]
    #[serde(default)]
    pub member_name: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeGroupMembershipResponse {
    #[serde(rename = "GroupMember")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_member: Option<GroupMember>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeGroupRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    pub group_name: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeGroupResponse {
    #[serde(rename = "Group")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeIAMPolicyAssignmentRequest {
    #[serde(rename = "AssignmentName")]
    #[serde(default)]
    pub assignment_name: String,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeIAMPolicyAssignmentResponse {
    #[serde(rename = "IAMPolicyAssignment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_a_m_policy_assignment: Option<IAMPolicyAssignment>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IAMPolicyAssignment {
    #[serde(rename = "AssignmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_id: Option<String>,
    #[serde(rename = "AssignmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_name: Option<String>,
    #[serde(rename = "AssignmentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_status: Option<String>,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    #[serde(rename = "Identities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identities: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "PolicyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeIngestionRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DataSetId")]
    #[serde(default)]
    pub data_set_id: String,
    #[serde(rename = "IngestionId")]
    #[serde(default)]
    pub ingestion_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeIngestionResponse {
    #[serde(rename = "Ingestion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion: Option<Ingestion>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Ingestion {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "ErrorInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_info: Option<ErrorInfo>,
    #[serde(rename = "IngestionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_id: Option<String>,
    #[serde(rename = "IngestionSizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_size_in_bytes: Option<i64>,
    #[serde(rename = "IngestionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_status: Option<String>,
    #[serde(rename = "IngestionTimeInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_time_in_seconds: Option<i64>,
    #[serde(rename = "QueueInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_info: Option<QueueInfo>,
    #[serde(rename = "RequestSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_source: Option<String>,
    #[serde(rename = "RequestType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_type: Option<String>,
    #[serde(rename = "RowInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_info: Option<RowInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ErrorInfo {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueueInfo {
    #[serde(rename = "QueuedIngestion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued_ingestion: Option<String>,
    #[serde(rename = "WaitingOnIngestion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waiting_on_ingestion: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RowInfo {
    #[serde(rename = "RowsDropped")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows_dropped: Option<i64>,
    #[serde(rename = "RowsIngested")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows_ingested: Option<i64>,
    #[serde(rename = "TotalRowsInDataset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_rows_in_dataset: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeIpRestrictionRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeIpRestrictionResponse {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "IpRestrictionRuleMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_restriction_rule_map: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "VpcEndpointIdRestrictionRuleMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_id_restriction_rule_map: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "VpcIdRestrictionRuleMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id_restriction_rule_map: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeKeyRegistrationRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DefaultKeyOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_key_only: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeKeyRegistrationResponse {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    #[serde(rename = "KeyRegistration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_registration: Option<Vec<RegisteredCustomerManagedKey>>,
    #[serde(rename = "QDataKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q_data_key: Option<QDataKey>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisteredCustomerManagedKey {
    #[serde(rename = "DefaultKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_key: Option<bool>,
    #[serde(rename = "KeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QDataKey {
    #[serde(rename = "QDataKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q_data_key_arn: Option<String>,
    #[serde(rename = "QDataKeyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q_data_key_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeNamespaceRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeNamespaceResponse {
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<NamespaceInfoV2>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NamespaceInfoV2 {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CapacityRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_region: Option<String>,
    #[serde(rename = "CreationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_status: Option<String>,
    #[serde(rename = "IamIdentityCenterApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_identity_center_application_arn: Option<String>,
    #[serde(rename = "IamIdentityCenterInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_identity_center_instance_arn: Option<String>,
    #[serde(rename = "IdentityStore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_store: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NamespaceError")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_error: Option<NamespaceError>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NamespaceError {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeQPersonalizationConfigurationRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeQPersonalizationConfigurationResponse {
    #[serde(rename = "PersonalizationMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personalization_mode: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeQuickSightQSearchConfigurationRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeQuickSightQSearchConfigurationResponse {
    #[serde(rename = "QSearchStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q_search_status: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRefreshScheduleRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DataSetId")]
    #[serde(default)]
    pub data_set_id: String,
    #[serde(rename = "ScheduleId")]
    #[serde(default)]
    pub schedule_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRefreshScheduleResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "RefreshSchedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_schedule: Option<RefreshSchedule>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRoleCustomPermissionRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "Role")]
    #[serde(default)]
    pub role: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRoleCustomPermissionResponse {
    #[serde(rename = "CustomPermissionsName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_permissions_name: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSelfUpgradeConfigurationRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSelfUpgradeConfigurationResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "SelfUpgradeConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_upgrade_configuration: Option<SelfUpgradeConfiguration>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SelfUpgradeConfiguration {
    #[serde(rename = "SelfUpgradeStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_upgrade_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTemplateAliasRequest {
    #[serde(rename = "AliasName")]
    #[serde(default)]
    pub alias_name: String,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "TemplateId")]
    #[serde(default)]
    pub template_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTemplateAliasResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "TemplateAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_alias: Option<TemplateAlias>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTemplateDefinitionRequest {
    #[serde(rename = "AliasName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "TemplateId")]
    #[serde(default)]
    pub template_id: String,
    #[serde(rename = "VersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTemplateDefinitionResponse {
    #[serde(rename = "Definition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<TemplateVersionDefinition>,
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<TemplateError>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "ResourceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_status: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "TemplateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(rename = "ThemeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TemplateError {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "ViolatedEntities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub violated_entities: Option<Vec<Entity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTemplatePermissionsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "TemplateId")]
    #[serde(default)]
    pub template_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTemplatePermissionsResponse {
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "TemplateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_arn: Option<String>,
    #[serde(rename = "TemplateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTemplateRequest {
    #[serde(rename = "AliasName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "TemplateId")]
    #[serde(default)]
    pub template_id: String,
    #[serde(rename = "VersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTemplateResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "Template")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<Template>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Template {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "TemplateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<TemplateVersion>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TemplateVersion {
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "DataSetConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_configurations: Option<Vec<DataSetConfiguration>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<TemplateError>>,
    #[serde(rename = "Sheets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheets: Option<Vec<Sheet>>,
    #[serde(rename = "SourceEntityArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_entity_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "ThemeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_arn: Option<String>,
    #[serde(rename = "VersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeThemeAliasRequest {
    #[serde(rename = "AliasName")]
    #[serde(default)]
    pub alias_name: String,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "ThemeId")]
    #[serde(default)]
    pub theme_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeThemeAliasResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "ThemeAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_alias: Option<ThemeAlias>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeThemePermissionsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "ThemeId")]
    #[serde(default)]
    pub theme_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeThemePermissionsResponse {
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "ThemeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_arn: Option<String>,
    #[serde(rename = "ThemeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeThemeRequest {
    #[serde(rename = "AliasName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_name: Option<String>,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "ThemeId")]
    #[serde(default)]
    pub theme_id: String,
    #[serde(rename = "VersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeThemeResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "Theme")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<Theme>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Theme {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ThemeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<ThemeVersion>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThemeVersion {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "BaseThemeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_theme_id: Option<String>,
    #[serde(rename = "Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ThemeConfiguration>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ThemeError>>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "VersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThemeError {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTopicPermissionsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "TopicId")]
    #[serde(default)]
    pub topic_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTopicPermissionsResponse {
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "TopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
    #[serde(rename = "TopicId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTopicRefreshRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "RefreshId")]
    #[serde(default)]
    pub refresh_id: String,
    #[serde(rename = "TopicId")]
    #[serde(default)]
    pub topic_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTopicRefreshResponse {
    #[serde(rename = "RefreshDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_details: Option<TopicRefreshDetails>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicRefreshDetails {
    #[serde(rename = "RefreshArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_arn: Option<String>,
    #[serde(rename = "RefreshId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_id: Option<String>,
    #[serde(rename = "RefreshStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTopicRefreshScheduleRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DatasetId")]
    #[serde(default)]
    pub dataset_id: String,
    #[serde(rename = "TopicId")]
    #[serde(default)]
    pub topic_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTopicRefreshScheduleResponse {
    #[serde(rename = "DatasetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arn: Option<String>,
    #[serde(rename = "RefreshSchedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_schedule: Option<TopicRefreshSchedule>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "TopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
    #[serde(rename = "TopicId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTopicRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "TopicId")]
    #[serde(default)]
    pub topic_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTopicResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CustomInstructions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_instructions: Option<CustomInstructions>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "Topic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<TopicDetails>,
    #[serde(rename = "TopicId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeUserRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeUserResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "User")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "Active")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CustomPermissionsName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_permissions_name: Option<String>,
    #[serde(rename = "Email")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "ExternalLoginFederationProviderType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_login_federation_provider_type: Option<String>,
    #[serde(rename = "ExternalLoginFederationProviderUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_login_federation_provider_url: Option<String>,
    #[serde(rename = "ExternalLoginId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_login_id: Option<String>,
    #[serde(rename = "IdentityType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_type: Option<String>,
    #[serde(rename = "PrincipalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
    #[serde(rename = "Role")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeVPCConnectionRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "VPCConnectionId")]
    #[serde(default)]
    pub v_p_c_connection_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeVPCConnectionResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "VPCConnection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_p_c_connection: Option<VPCConnection>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VPCConnection {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "AvailabilityStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_status: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "DnsResolvers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_resolvers: Option<Vec<String>>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NetworkInterfaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interfaces: Option<Vec<NetworkInterface>>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "VPCConnectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_p_c_connection_id: Option<String>,
    #[serde(rename = "VPCId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_p_c_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkInterface {
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "NetworkInterfaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "SubnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GenerateEmbedUrlForAnonymousUserRequest {
    #[serde(rename = "AllowedDomains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_domains: Option<Vec<String>>,
    #[serde(rename = "AuthorizedResourceArns")]
    #[serde(default)]
    pub authorized_resource_arns: Vec<String>,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "ExperienceConfiguration")]
    #[serde(default)]
    pub experience_configuration: AnonymousUserEmbeddingExperienceConfiguration,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "SessionLifetimeInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_lifetime_in_minutes: Option<i64>,
    #[serde(rename = "SessionTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_tags: Option<Vec<SessionTag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnonymousUserEmbeddingExperienceConfiguration {
    #[serde(rename = "Dashboard")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard: Option<AnonymousUserDashboardEmbeddingConfiguration>,
    #[serde(rename = "DashboardVisual")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_visual: Option<AnonymousUserDashboardVisualEmbeddingConfiguration>,
    #[serde(rename = "GenerativeQnA")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generative_qn_a: Option<AnonymousUserGenerativeQnAEmbeddingConfiguration>,
    #[serde(rename = "QSearchBar")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q_search_bar: Option<AnonymousUserQSearchBarEmbeddingConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnonymousUserDashboardEmbeddingConfiguration {
    #[serde(rename = "DisabledFeatures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_features: Option<Vec<String>>,
    #[serde(rename = "EnabledFeatures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_features: Option<Vec<String>>,
    #[serde(rename = "FeatureConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_configurations: Option<AnonymousUserDashboardFeatureConfigurations>,
    #[serde(rename = "InitialDashboardId")]
    #[serde(default)]
    pub initial_dashboard_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnonymousUserDashboardFeatureConfigurations {
    #[serde(rename = "SharedView")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_view: Option<SharedViewConfigurations>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SharedViewConfigurations {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnonymousUserDashboardVisualEmbeddingConfiguration {
    #[serde(rename = "InitialDashboardVisualId")]
    #[serde(default)]
    pub initial_dashboard_visual_id: DashboardVisualId,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DashboardVisualId {
    #[serde(rename = "DashboardId")]
    #[serde(default)]
    pub dashboard_id: String,
    #[serde(rename = "SheetId")]
    #[serde(default)]
    pub sheet_id: String,
    #[serde(rename = "VisualId")]
    #[serde(default)]
    pub visual_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnonymousUserGenerativeQnAEmbeddingConfiguration {
    #[serde(rename = "InitialTopicId")]
    #[serde(default)]
    pub initial_topic_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnonymousUserQSearchBarEmbeddingConfiguration {
    #[serde(rename = "InitialTopicId")]
    #[serde(default)]
    pub initial_topic_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SessionTag {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GenerateEmbedUrlForAnonymousUserResponse {
    #[serde(rename = "AnonymousUserArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous_user_arn: Option<String>,
    #[serde(rename = "EmbedUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_url: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GenerateEmbedUrlForRegisteredUserRequest {
    #[serde(rename = "AllowedDomains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_domains: Option<Vec<String>>,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "ExperienceConfiguration")]
    #[serde(default)]
    pub experience_configuration: RegisteredUserEmbeddingExperienceConfiguration,
    #[serde(rename = "SessionLifetimeInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_lifetime_in_minutes: Option<i64>,
    #[serde(rename = "UserArn")]
    #[serde(default)]
    pub user_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisteredUserEmbeddingExperienceConfiguration {
    #[serde(rename = "Dashboard")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard: Option<RegisteredUserDashboardEmbeddingConfiguration>,
    #[serde(rename = "DashboardVisual")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_visual: Option<RegisteredUserDashboardVisualEmbeddingConfiguration>,
    #[serde(rename = "GenerativeQnA")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generative_qn_a: Option<RegisteredUserGenerativeQnAEmbeddingConfiguration>,
    #[serde(rename = "QSearchBar")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q_search_bar: Option<RegisteredUserQSearchBarEmbeddingConfiguration>,
    #[serde(rename = "QuickChat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_chat: Option<RegisteredUserQuickChatEmbeddingConfiguration>,
    #[serde(rename = "QuickSightConsole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_sight_console: Option<RegisteredUserQuickSightConsoleEmbeddingConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisteredUserDashboardEmbeddingConfiguration {
    #[serde(rename = "FeatureConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_configurations: Option<RegisteredUserDashboardFeatureConfigurations>,
    #[serde(rename = "InitialDashboardId")]
    #[serde(default)]
    pub initial_dashboard_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisteredUserDashboardFeatureConfigurations {
    #[serde(rename = "AmazonQInQuickSight")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_q_in_quick_sight: Option<AmazonQInQuickSightDashboardConfigurations>,
    #[serde(rename = "Bookmarks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bookmarks: Option<BookmarksConfigurations>,
    #[serde(rename = "DashboardCustomizationSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_customization_summary: Option<DashboardCustomizationSummaryConfigurations>,
    #[serde(rename = "RecentSnapshots")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recent_snapshots: Option<RecentSnapshotsConfigurations>,
    #[serde(rename = "Schedules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedules: Option<SchedulesConfigurations>,
    #[serde(rename = "SharedView")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_view: Option<SharedViewConfigurations>,
    #[serde(rename = "StatePersistence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_persistence: Option<StatePersistenceConfigurations>,
    #[serde(rename = "ThresholdAlerts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_alerts: Option<ThresholdAlertsConfigurations>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AmazonQInQuickSightDashboardConfigurations {
    #[serde(rename = "ExecutiveSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executive_summary: Option<ExecutiveSummaryConfigurations>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutiveSummaryConfigurations {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BookmarksConfigurations {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DashboardCustomizationSummaryConfigurations {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecentSnapshotsConfigurations {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SchedulesConfigurations {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StatePersistenceConfigurations {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThresholdAlertsConfigurations {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisteredUserDashboardVisualEmbeddingConfiguration {
    #[serde(rename = "InitialDashboardVisualId")]
    #[serde(default)]
    pub initial_dashboard_visual_id: DashboardVisualId,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisteredUserGenerativeQnAEmbeddingConfiguration {
    #[serde(rename = "InitialTopicId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_topic_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisteredUserQSearchBarEmbeddingConfiguration {
    #[serde(rename = "InitialTopicId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_topic_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisteredUserQuickChatEmbeddingConfiguration {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisteredUserQuickSightConsoleEmbeddingConfiguration {
    #[serde(rename = "FeatureConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_configurations: Option<RegisteredUserConsoleFeatureConfigurations>,
    #[serde(rename = "InitialPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisteredUserConsoleFeatureConfigurations {
    #[serde(rename = "AmazonQInQuickSight")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_q_in_quick_sight: Option<AmazonQInQuickSightConsoleConfigurations>,
    #[serde(rename = "DashboardCustomizationSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_customization_summary: Option<DashboardCustomizationSummaryConfigurations>,
    #[serde(rename = "RecentSnapshots")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recent_snapshots: Option<RecentSnapshotsConfigurations>,
    #[serde(rename = "Schedules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedules: Option<SchedulesConfigurations>,
    #[serde(rename = "SharedView")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_view: Option<SharedViewConfigurations>,
    #[serde(rename = "StatePersistence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_persistence: Option<StatePersistenceConfigurations>,
    #[serde(rename = "ThresholdAlerts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_alerts: Option<ThresholdAlertsConfigurations>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AmazonQInQuickSightConsoleConfigurations {
    #[serde(rename = "DataQnA")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_qn_a: Option<DataQnAConfigurations>,
    #[serde(rename = "DataStories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_stories: Option<DataStoriesConfigurations>,
    #[serde(rename = "ExecutiveSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executive_summary: Option<ExecutiveSummaryConfigurations>,
    #[serde(rename = "GenerativeAuthoring")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generative_authoring: Option<GenerativeAuthoringConfigurations>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataQnAConfigurations {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataStoriesConfigurations {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GenerativeAuthoringConfigurations {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GenerateEmbedUrlForRegisteredUserResponse {
    #[serde(rename = "EmbedUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_url: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GenerateEmbedUrlForRegisteredUserWithIdentityRequest {
    #[serde(rename = "AllowedDomains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_domains: Option<Vec<String>>,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "ExperienceConfiguration")]
    #[serde(default)]
    pub experience_configuration: RegisteredUserEmbeddingExperienceConfiguration,
    #[serde(rename = "SessionLifetimeInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_lifetime_in_minutes: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GenerateEmbedUrlForRegisteredUserWithIdentityResponse {
    #[serde(rename = "EmbedUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_url: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDashboardEmbedUrlRequest {
    #[serde(rename = "AdditionalDashboardIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_dashboard_ids: Option<Vec<String>>,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DashboardId")]
    #[serde(default)]
    pub dashboard_id: String,
    #[serde(rename = "IdentityType")]
    #[serde(default)]
    pub identity_type: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "ResetDisabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_disabled: Option<bool>,
    #[serde(rename = "SessionLifetimeInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_lifetime_in_minutes: Option<i64>,
    #[serde(rename = "StatePersistenceEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_persistence_enabled: Option<bool>,
    #[serde(rename = "UndoRedoDisabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub undo_redo_disabled: Option<bool>,
    #[serde(rename = "UserArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDashboardEmbedUrlResponse {
    #[serde(rename = "EmbedUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_url: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFlowMetadataInput {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "FlowId")]
    #[serde(default)]
    pub flow_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFlowMetadataOutput {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "FlowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_id: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PublishState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_state: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "RunCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_count: Option<i32>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "UserCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFlowPermissionsInput {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "FlowId")]
    #[serde(default)]
    pub flow_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFlowPermissionsOutput {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "FlowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_id: Option<String>,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<Permission>>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Permission {
    #[serde(rename = "Actions")]
    #[serde(default)]
    pub actions: Vec<String>,
    #[serde(rename = "Principal")]
    #[serde(default)]
    pub principal: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIdentityContextRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "ContextRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_region: Option<String>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "SessionExpiresAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_expires_at: Option<f64>,
    #[serde(rename = "UserIdentifier")]
    #[serde(default)]
    pub user_identifier: UserIdentifier,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserIdentifier {
    #[serde(rename = "Email")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "UserArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_arn: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIdentityContextResponse {
    #[serde(rename = "Context")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSessionEmbedUrlRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "EntryPoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_point: Option<String>,
    #[serde(rename = "SessionLifetimeInMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_lifetime_in_minutes: Option<i64>,
    #[serde(rename = "UserArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSessionEmbedUrlResponse {
    #[serde(rename = "EmbedUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed_url: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListActionConnectorsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
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
pub struct ListActionConnectorsResponse {
    #[serde(rename = "ActionConnectorSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_connector_summaries: Option<Vec<ActionConnectorSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionConnectorSummary {
    #[serde(rename = "ActionConnectorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_connector_id: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ActionConnectorError>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAnalysesRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
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
pub struct ListAnalysesResponse {
    #[serde(rename = "AnalysisSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_summary_list: Option<Vec<AnalysisSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnalysisSummary {
    #[serde(rename = "AnalysisId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_id: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAssetBundleExportJobsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
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
pub struct ListAssetBundleExportJobsResponse {
    #[serde(rename = "AssetBundleExportJobSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_bundle_export_job_summary_list: Option<Vec<AssetBundleExportJobSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleExportJobSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "AssetBundleExportJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_bundle_export_job_id: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "ExportFormat")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_format: Option<String>,
    #[serde(rename = "IncludeAllDependencies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_all_dependencies: Option<bool>,
    #[serde(rename = "IncludePermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_permissions: Option<bool>,
    #[serde(rename = "IncludeTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_tags: Option<bool>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAssetBundleImportJobsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
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
pub struct ListAssetBundleImportJobsResponse {
    #[serde(rename = "AssetBundleImportJobSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_bundle_import_job_summary_list: Option<Vec<AssetBundleImportJobSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleImportJobSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "AssetBundleImportJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_bundle_import_job_id: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "FailureAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_action: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBrandsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
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
pub struct ListBrandsResponse {
    #[serde(rename = "Brands")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brands: Option<Vec<BrandSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BrandSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "BrandId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_id: Option<String>,
    #[serde(rename = "BrandName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_name: Option<String>,
    #[serde(rename = "BrandStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_status: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCustomPermissionsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
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
pub struct ListCustomPermissionsResponse {
    #[serde(rename = "CustomPermissionsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_permissions_list: Option<Vec<CustomPermissions>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDashboardVersionsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DashboardId")]
    #[serde(default)]
    pub dashboard_id: String,
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
pub struct ListDashboardVersionsResponse {
    #[serde(rename = "DashboardVersionSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_version_summary_list: Option<Vec<DashboardVersionSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DashboardVersionSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "SourceEntityArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_entity_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "VersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDashboardsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
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
pub struct ListDashboardsResponse {
    #[serde(rename = "DashboardSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_summary_list: Option<Vec<DashboardSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DashboardSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "DashboardId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_id: Option<String>,
    #[serde(rename = "LastPublishedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_published_time: Option<f64>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PublishedVersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_version_number: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDataSetsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
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
pub struct ListDataSetsResponse {
    #[serde(rename = "DataSetSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_summaries: Option<Vec<DataSetSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSetSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ColumnLevelPermissionRulesApplied")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_level_permission_rules_applied: Option<bool>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "DataSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    #[serde(rename = "ImportMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_mode: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RowLevelPermissionDataSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_level_permission_data_set: Option<RowLevelPermissionDataSet>,
    #[serde(rename = "RowLevelPermissionDataSetMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_level_permission_data_set_map:
        Option<std::collections::HashMap<String, RowLevelPermissionDataSet>>,
    #[serde(rename = "RowLevelPermissionTagConfigurationApplied")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_level_permission_tag_configuration_applied: Option<bool>,
    #[serde(rename = "UseAs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_as: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDataSourcesRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
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
pub struct ListDataSourcesResponse {
    #[serde(rename = "DataSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<Vec<DataSource>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFlowsInput {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
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
pub struct ListFlowsOutput {
    #[serde(rename = "FlowSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_summary_list: Option<Vec<FlowSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlowSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "FlowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_id: Option<String>,
    #[serde(rename = "LastPublishedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_published_at: Option<f64>,
    #[serde(rename = "LastPublishedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_published_by: Option<String>,
    #[serde(rename = "LastUpdatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PublishState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_state: Option<String>,
    #[serde(rename = "RunCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_count: Option<i32>,
    #[serde(rename = "UserCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFolderMembersRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "FolderId")]
    #[serde(default)]
    pub folder_id: String,
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
pub struct ListFolderMembersResponse {
    #[serde(rename = "FolderMemberList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_member_list: Option<Vec<MemberIdArnPair>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MemberIdArnPair {
    #[serde(rename = "MemberArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_arn: Option<String>,
    #[serde(rename = "MemberId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFoldersForResourceRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFoldersForResourceResponse {
    #[serde(rename = "Folders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folders: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFoldersRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
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
pub struct ListFoldersResponse {
    #[serde(rename = "FolderSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_summary_list: Option<Vec<FolderSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FolderSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "FolderId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    #[serde(rename = "FolderType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_type: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SharingModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharing_model: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListGroupMembershipsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    pub group_name: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListGroupMembershipsResponse {
    #[serde(rename = "GroupMemberList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_member_list: Option<Vec<GroupMember>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListGroupsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListGroupsResponse {
    #[serde(rename = "GroupList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_list: Option<Vec<Group>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIAMPolicyAssignmentsForUserRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIAMPolicyAssignmentsForUserResponse {
    #[serde(rename = "ActiveAssignments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_assignments: Option<Vec<ActiveIAMPolicyAssignment>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActiveIAMPolicyAssignment {
    #[serde(rename = "AssignmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_name: Option<String>,
    #[serde(rename = "PolicyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIAMPolicyAssignmentsRequest {
    #[serde(rename = "AssignmentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_status: Option<String>,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIAMPolicyAssignmentsResponse {
    #[serde(rename = "IAMPolicyAssignments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_a_m_policy_assignments: Option<Vec<IAMPolicyAssignmentSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IAMPolicyAssignmentSummary {
    #[serde(rename = "AssignmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_name: Option<String>,
    #[serde(rename = "AssignmentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIdentityPropagationConfigsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
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
pub struct ListIdentityPropagationConfigsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Services")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<AuthorizedTargetsByService>>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthorizedTargetsByService {
    #[serde(rename = "AuthorizedTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_targets: Option<Vec<String>>,
    #[serde(rename = "Service")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIngestionsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DataSetId")]
    #[serde(default)]
    pub data_set_id: String,
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
pub struct ListIngestionsResponse {
    #[serde(rename = "Ingestions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestions: Option<Vec<Ingestion>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListNamespacesRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
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
pub struct ListNamespacesResponse {
    #[serde(rename = "Namespaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<NamespaceInfoV2>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRefreshSchedulesRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DataSetId")]
    #[serde(default)]
    pub data_set_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRefreshSchedulesResponse {
    #[serde(rename = "RefreshSchedules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_schedules: Option<Vec<RefreshSchedule>>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRoleMembershipsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Role")]
    #[serde(default)]
    pub role: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRoleMembershipsResponse {
    #[serde(rename = "MembersList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members_list: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSelfUpgradesRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSelfUpgradesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "SelfUpgradeRequestDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_upgrade_request_details: Option<Vec<SelfUpgradeRequestDetail>>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SelfUpgradeRequestDetail {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<i64>,
    #[serde(rename = "OriginalRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_role: Option<String>,
    #[serde(rename = "RequestNote")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_note: Option<String>,
    #[serde(rename = "RequestStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_status: Option<String>,
    #[serde(rename = "RequestedRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_role: Option<String>,
    #[serde(rename = "UpgradeRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_request_id: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(rename = "lastUpdateAttemptTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_attempt_time: Option<i64>,
    #[serde(rename = "lastUpdateFailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_failure_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTemplateAliasesRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TemplateId")]
    #[serde(default)]
    pub template_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTemplateAliasesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "TemplateAliasList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_alias_list: Option<Vec<TemplateAlias>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTemplateVersionsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TemplateId")]
    #[serde(default)]
    pub template_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTemplateVersionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "TemplateVersionSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_version_summary_list: Option<Vec<TemplateVersionSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TemplateVersionSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "VersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTemplatesRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
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
pub struct ListTemplatesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "TemplateSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_summary_list: Option<Vec<TemplateSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TemplateSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "LatestVersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_number: Option<i64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "TemplateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListThemeAliasesRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ThemeId")]
    #[serde(default)]
    pub theme_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListThemeAliasesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "ThemeAliasList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_alias_list: Option<Vec<ThemeAlias>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListThemeVersionsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ThemeId")]
    #[serde(default)]
    pub theme_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListThemeVersionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "ThemeVersionSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_version_summary_list: Option<Vec<ThemeVersionSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThemeVersionSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "VersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_number: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListThemesRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListThemesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "ThemeSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_summary_list: Option<Vec<ThemeSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThemeSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "LatestVersionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_version_number: Option<i64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ThemeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTopicRefreshSchedulesRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "TopicId")]
    #[serde(default)]
    pub topic_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTopicRefreshSchedulesResponse {
    #[serde(rename = "RefreshSchedules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_schedules: Option<Vec<TopicRefreshScheduleSummary>>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "TopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
    #[serde(rename = "TopicId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicRefreshScheduleSummary {
    #[serde(rename = "DatasetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arn: Option<String>,
    #[serde(rename = "DatasetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_id: Option<String>,
    #[serde(rename = "DatasetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_name: Option<String>,
    #[serde(rename = "RefreshSchedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_schedule: Option<TopicRefreshSchedule>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTopicReviewedAnswersRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "TopicId")]
    #[serde(default)]
    pub topic_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTopicReviewedAnswersResponse {
    #[serde(rename = "Answers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answers: Option<Vec<TopicReviewedAnswer>>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "TopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
    #[serde(rename = "TopicId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicReviewedAnswer {
    #[serde(rename = "AnswerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer_id: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "DatasetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arn: Option<String>,
    #[serde(rename = "Mir")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mir: Option<TopicIR>,
    #[serde(rename = "PrimaryVisual")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_visual: Option<TopicVisual>,
    #[serde(rename = "Question")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question: Option<String>,
    #[serde(rename = "Template")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<TopicTemplate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTopicsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
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
pub struct ListTopicsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "TopicsSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics_summaries: Option<Vec<TopicSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "TopicId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<String>,
    #[serde(rename = "UserExperienceVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_experience_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListUserGroupsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListUserGroupsResponse {
    #[serde(rename = "GroupList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_list: Option<Vec<Group>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListUsersRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListUsersResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "UserList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_list: Option<Vec<User>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVPCConnectionsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
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
pub struct ListVPCConnectionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "VPCConnectionSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_p_c_connection_summaries: Option<Vec<VPCConnectionSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VPCConnectionSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "AvailabilityStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_status: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "DnsResolvers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_resolvers: Option<Vec<String>>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NetworkInterfaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interfaces: Option<Vec<NetworkInterface>>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "VPCConnectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_p_c_connection_id: Option<String>,
    #[serde(rename = "VPCId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_p_c_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PredictQAResultsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "IncludeGeneratedAnswer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_generated_answer: Option<String>,
    #[serde(rename = "IncludeQuickSightQIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_quick_sight_q_index: Option<String>,
    #[serde(rename = "MaxTopicsToConsider")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_topics_to_consider: Option<i32>,
    #[serde(rename = "QueryText")]
    #[serde(default)]
    pub query_text: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PredictQAResultsResponse {
    #[serde(rename = "AdditionalResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_results: Option<Vec<QAResult>>,
    #[serde(rename = "PrimaryResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_result: Option<QAResult>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QAResult {
    #[serde(rename = "DashboardVisual")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_visual: Option<DashboardVisualResult>,
    #[serde(rename = "GeneratedAnswer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_answer: Option<GeneratedAnswerResult>,
    #[serde(rename = "ResultType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DashboardVisualResult {
    #[serde(rename = "DashboardId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_id: Option<String>,
    #[serde(rename = "DashboardName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_name: Option<String>,
    #[serde(rename = "DashboardUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_url: Option<String>,
    #[serde(rename = "SheetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_id: Option<String>,
    #[serde(rename = "SheetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_name: Option<String>,
    #[serde(rename = "VisualId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_id: Option<String>,
    #[serde(rename = "VisualSubtitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_subtitle: Option<String>,
    #[serde(rename = "VisualTitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visual_title: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeneratedAnswerResult {
    #[serde(rename = "AnswerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer_id: Option<String>,
    #[serde(rename = "AnswerStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer_status: Option<String>,
    #[serde(rename = "QuestionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_id: Option<String>,
    #[serde(rename = "QuestionText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_text: Option<String>,
    #[serde(rename = "QuestionUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_url: Option<String>,
    #[serde(rename = "Restatement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restatement: Option<String>,
    #[serde(rename = "TopicId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<String>,
    #[serde(rename = "TopicName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutDataSetRefreshPropertiesRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DataSetId")]
    #[serde(default)]
    pub data_set_id: String,
    #[serde(rename = "DataSetRefreshProperties")]
    #[serde(default)]
    pub data_set_refresh_properties: DataSetRefreshProperties,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutDataSetRefreshPropertiesResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterUserRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "CustomFederationProviderUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_federation_provider_url: Option<String>,
    #[serde(rename = "CustomPermissionsName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_permissions_name: Option<String>,
    #[serde(rename = "Email")]
    #[serde(default)]
    pub email: String,
    #[serde(rename = "ExternalLoginFederationProviderType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_login_federation_provider_type: Option<String>,
    #[serde(rename = "ExternalLoginId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_login_id: Option<String>,
    #[serde(rename = "IamArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_arn: Option<String>,
    #[serde(rename = "IdentityType")]
    #[serde(default)]
    pub identity_type: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "SessionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_name: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(rename = "UserRole")]
    #[serde(default)]
    pub user_role: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterUserResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "User")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    #[serde(rename = "UserInvitationUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_invitation_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestoreAnalysisRequest {
    #[serde(rename = "AnalysisId")]
    #[serde(default)]
    pub analysis_id: String,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "RestoreToFolders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_to_folders: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestoreAnalysisResponse {
    #[serde(rename = "AnalysisId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_id: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "RestorationFailedFolderArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restoration_failed_folder_arns: Option<Vec<String>>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchActionConnectorsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Filters")]
    #[serde(default)]
    pub filters: Vec<ActionConnectorSearchFilter>,
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
pub struct ActionConnectorSearchFilter {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Operator")]
    #[serde(default)]
    pub operator: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchActionConnectorsResponse {
    #[serde(rename = "ActionConnectorSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_connector_summaries: Option<Vec<ActionConnectorSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchAnalysesRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Filters")]
    #[serde(default)]
    pub filters: Vec<AnalysisSearchFilter>,
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
pub struct AnalysisSearchFilter {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Operator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchAnalysesResponse {
    #[serde(rename = "AnalysisSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_summary_list: Option<Vec<AnalysisSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchDashboardsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Filters")]
    #[serde(default)]
    pub filters: Vec<DashboardSearchFilter>,
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
pub struct DashboardSearchFilter {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Operator")]
    #[serde(default)]
    pub operator: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchDashboardsResponse {
    #[serde(rename = "DashboardSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_summary_list: Option<Vec<DashboardSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchDataSetsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Filters")]
    #[serde(default)]
    pub filters: Vec<DataSetSearchFilter>,
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
pub struct DataSetSearchFilter {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Operator")]
    #[serde(default)]
    pub operator: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchDataSetsResponse {
    #[serde(rename = "DataSetSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_summaries: Option<Vec<DataSetSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchDataSourcesRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Filters")]
    #[serde(default)]
    pub filters: Vec<DataSourceSearchFilter>,
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
pub struct DataSourceSearchFilter {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Operator")]
    #[serde(default)]
    pub operator: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchDataSourcesResponse {
    #[serde(rename = "DataSourceSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_summaries: Option<Vec<DataSourceSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSourceSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "DataSourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchFlowsInput {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Filters")]
    #[serde(default)]
    pub filters: Vec<SearchFlowsFilter>,
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
pub struct SearchFlowsFilter {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Operator")]
    #[serde(default)]
    pub operator: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchFlowsOutput {
    #[serde(rename = "FlowSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_summary_list: Option<Vec<FlowSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchFoldersRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Filters")]
    #[serde(default)]
    pub filters: Vec<FolderSearchFilter>,
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
pub struct FolderSearchFilter {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Operator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchFoldersResponse {
    #[serde(rename = "FolderSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_summary_list: Option<Vec<FolderSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchGroupsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Filters")]
    #[serde(default)]
    pub filters: Vec<GroupSearchFilter>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GroupSearchFilter {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Operator")]
    #[serde(default)]
    pub operator: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchGroupsResponse {
    #[serde(rename = "GroupList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_list: Option<Vec<Group>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchTopicsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Filters")]
    #[serde(default)]
    pub filters: Vec<TopicSearchFilter>,
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
pub struct TopicSearchFilter {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Operator")]
    #[serde(default)]
    pub operator: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchTopicsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "TopicSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_summary_list: Option<Vec<TopicSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartAssetBundleExportJobRequest {
    #[serde(rename = "AssetBundleExportJobId")]
    #[serde(default)]
    pub asset_bundle_export_job_id: String,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "CloudFormationOverridePropertyConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_formation_override_property_configuration:
        Option<AssetBundleCloudFormationOverridePropertyConfiguration>,
    #[serde(rename = "ExportFormat")]
    #[serde(default)]
    pub export_format: String,
    #[serde(rename = "IncludeAllDependencies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_all_dependencies: Option<bool>,
    #[serde(rename = "IncludeFolderMembers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_folder_members: Option<String>,
    #[serde(rename = "IncludeFolderMemberships")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_folder_memberships: Option<bool>,
    #[serde(rename = "IncludePermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_permissions: Option<bool>,
    #[serde(rename = "IncludeTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_tags: Option<bool>,
    #[serde(rename = "ResourceArns")]
    #[serde(default)]
    pub resource_arns: Vec<String>,
    #[serde(rename = "ValidationStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_strategy: Option<AssetBundleExportJobValidationStrategy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartAssetBundleExportJobResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "AssetBundleExportJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_bundle_export_job_id: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartAssetBundleImportJobRequest {
    #[serde(rename = "AssetBundleImportJobId")]
    #[serde(default)]
    pub asset_bundle_import_job_id: String,
    #[serde(rename = "AssetBundleImportSource")]
    #[serde(default)]
    pub asset_bundle_import_source: AssetBundleImportSource,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "FailureAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_action: Option<String>,
    #[serde(rename = "OverrideParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_parameters: Option<AssetBundleImportJobOverrideParameters>,
    #[serde(rename = "OverridePermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_permissions: Option<AssetBundleImportJobOverridePermissions>,
    #[serde(rename = "OverrideTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_tags: Option<AssetBundleImportJobOverrideTags>,
    #[serde(rename = "OverrideValidationStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_validation_strategy: Option<AssetBundleImportJobOverrideValidationStrategy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetBundleImportSource {
    #[serde(rename = "Body")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "S3Uri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_uri: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartAssetBundleImportJobResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "AssetBundleImportJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_bundle_import_job_id: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartAutomationJobRequest {
    #[serde(rename = "AutomationGroupId")]
    #[serde(default)]
    pub automation_group_id: String,
    #[serde(rename = "AutomationId")]
    #[serde(default)]
    pub automation_id: String,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "InputPayload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_payload: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartAutomationJobResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDashboardSnapshotJobRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DashboardId")]
    #[serde(default)]
    pub dashboard_id: String,
    #[serde(rename = "SnapshotConfiguration")]
    #[serde(default)]
    pub snapshot_configuration: SnapshotConfiguration,
    #[serde(rename = "SnapshotJobId")]
    #[serde(default)]
    pub snapshot_job_id: String,
    #[serde(rename = "UserConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_configuration: Option<SnapshotUserConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnapshotUserConfiguration {
    #[serde(rename = "AnonymousUsers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous_users: Option<Vec<SnapshotAnonymousUser>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnapshotAnonymousUser {
    #[serde(rename = "RowLevelPermissionTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_level_permission_tags: Option<Vec<SessionTag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDashboardSnapshotJobResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "SnapshotJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_job_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDashboardSnapshotJobScheduleRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DashboardId")]
    #[serde(default)]
    pub dashboard_id: String,
    #[serde(rename = "ScheduleId")]
    #[serde(default)]
    pub schedule_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDashboardSnapshotJobScheduleResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
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
pub struct TagResourceResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

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
pub struct UntagResourceResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAccountCustomPermissionRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "CustomPermissionsName")]
    #[serde(default)]
    pub custom_permissions_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAccountCustomPermissionResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAccountCustomizationRequest {
    #[serde(rename = "AccountCustomization")]
    #[serde(default)]
    pub account_customization: AccountCustomization,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAccountCustomizationResponse {
    #[serde(rename = "AccountCustomization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_customization: Option<AccountCustomization>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAccountSettingsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DefaultNamespace")]
    #[serde(default)]
    pub default_namespace: String,
    #[serde(rename = "NotificationEmail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_email: Option<String>,
    #[serde(rename = "TerminationProtectionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_protection_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAccountSettingsResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateActionConnectorPermissionsRequest {
    #[serde(rename = "ActionConnectorId")]
    #[serde(default)]
    pub action_connector_id: String,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "GrantPermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "RevokePermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_permissions: Option<Vec<ResourcePermission>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateActionConnectorPermissionsResponse {
    #[serde(rename = "ActionConnectorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_connector_id: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateActionConnectorRequest {
    #[serde(rename = "ActionConnectorId")]
    #[serde(default)]
    pub action_connector_id: String,
    #[serde(rename = "AuthenticationConfig")]
    #[serde(default)]
    pub authentication_config: AuthConfig,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "VpcConnectionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_connection_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateActionConnectorResponse {
    #[serde(rename = "ActionConnectorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_connector_id: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "UpdateStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAnalysisPermissionsRequest {
    #[serde(rename = "AnalysisId")]
    #[serde(default)]
    pub analysis_id: String,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "GrantPermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "RevokePermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_permissions: Option<Vec<ResourcePermission>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAnalysisPermissionsResponse {
    #[serde(rename = "AnalysisArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_arn: Option<String>,
    #[serde(rename = "AnalysisId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_id: Option<String>,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAnalysisRequest {
    #[serde(rename = "AnalysisId")]
    #[serde(default)]
    pub analysis_id: String,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Definition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<AnalysisDefinition>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Parameters>,
    #[serde(rename = "SourceEntity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_entity: Option<AnalysisSourceEntity>,
    #[serde(rename = "ThemeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_arn: Option<String>,
    #[serde(rename = "ValidationStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_strategy: Option<ValidationStrategy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAnalysisResponse {
    #[serde(rename = "AnalysisId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analysis_id: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "UpdateStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApplicationWithTokenExchangeGrantRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApplicationWithTokenExchangeGrantResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBrandAssignmentRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "BrandArn")]
    #[serde(default)]
    pub brand_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBrandAssignmentResponse {
    #[serde(rename = "BrandArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_arn: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBrandPublishedVersionRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "BrandId")]
    #[serde(default)]
    pub brand_id: String,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    pub version_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBrandPublishedVersionResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBrandRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "BrandDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_definition: Option<BrandDefinition>,
    #[serde(rename = "BrandId")]
    #[serde(default)]
    pub brand_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBrandResponse {
    #[serde(rename = "BrandDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_definition: Option<BrandDefinition>,
    #[serde(rename = "BrandDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_detail: Option<BrandDetail>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCustomPermissionsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Capabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Capabilities>,
    #[serde(rename = "CustomPermissionsName")]
    #[serde(default)]
    pub custom_permissions_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCustomPermissionsResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDashboardLinksRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DashboardId")]
    #[serde(default)]
    pub dashboard_id: String,
    #[serde(rename = "LinkEntities")]
    #[serde(default)]
    pub link_entities: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDashboardLinksResponse {
    #[serde(rename = "DashboardArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_arn: Option<String>,
    #[serde(rename = "LinkEntities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_entities: Option<Vec<String>>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDashboardPermissionsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DashboardId")]
    #[serde(default)]
    pub dashboard_id: String,
    #[serde(rename = "GrantLinkPermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_link_permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "GrantPermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "RevokeLinkPermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_link_permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "RevokePermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_permissions: Option<Vec<ResourcePermission>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDashboardPermissionsResponse {
    #[serde(rename = "DashboardArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_arn: Option<String>,
    #[serde(rename = "DashboardId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_id: Option<String>,
    #[serde(rename = "LinkSharingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_sharing_configuration: Option<LinkSharingConfiguration>,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDashboardPublishedVersionRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DashboardId")]
    #[serde(default)]
    pub dashboard_id: String,
    #[serde(rename = "VersionNumber")]
    #[serde(default)]
    pub version_number: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDashboardPublishedVersionResponse {
    #[serde(rename = "DashboardArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_arn: Option<String>,
    #[serde(rename = "DashboardId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_id: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDashboardRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DashboardId")]
    #[serde(default)]
    pub dashboard_id: String,
    #[serde(rename = "DashboardPublishOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_publish_options: Option<DashboardPublishOptions>,
    #[serde(rename = "Definition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<DashboardVersionDefinition>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Parameters>,
    #[serde(rename = "SourceEntity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_entity: Option<DashboardSourceEntity>,
    #[serde(rename = "ThemeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_arn: Option<String>,
    #[serde(rename = "ValidationStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_strategy: Option<ValidationStrategy>,
    #[serde(rename = "VersionDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDashboardResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_status: Option<String>,
    #[serde(rename = "DashboardId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_id: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "VersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDashboardsQAConfigurationRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DashboardsQAStatus")]
    #[serde(default)]
    pub dashboards_q_a_status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDashboardsQAConfigurationResponse {
    #[serde(rename = "DashboardsQAStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboards_q_a_status: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDataSetPermissionsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DataSetId")]
    #[serde(default)]
    pub data_set_id: String,
    #[serde(rename = "GrantPermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "RevokePermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_permissions: Option<Vec<ResourcePermission>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDataSetPermissionsResponse {
    #[serde(rename = "DataSetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_arn: Option<String>,
    #[serde(rename = "DataSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDataSetRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "ColumnGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_groups: Option<Vec<ColumnGroup>>,
    #[serde(rename = "ColumnLevelPermissionRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_level_permission_rules: Option<Vec<ColumnLevelPermissionRule>>,
    #[serde(rename = "DataPrepConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_prep_configuration: Option<DataPrepConfiguration>,
    #[serde(rename = "DataSetId")]
    #[serde(default)]
    pub data_set_id: String,
    #[serde(rename = "DataSetUsageConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_usage_configuration: Option<DataSetUsageConfiguration>,
    #[serde(rename = "DatasetParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_parameters: Option<Vec<DatasetParameter>>,
    #[serde(rename = "FieldFolders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_folders: Option<std::collections::HashMap<String, FieldFolder>>,
    #[serde(rename = "ImportMode")]
    #[serde(default)]
    pub import_mode: String,
    #[serde(rename = "LogicalTableMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_table_map: Option<std::collections::HashMap<String, LogicalTable>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "PerformanceConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_configuration: Option<PerformanceConfiguration>,
    #[serde(rename = "PhysicalTableMap")]
    #[serde(default)]
    pub physical_table_map: std::collections::HashMap<String, PhysicalTable>,
    #[serde(rename = "RowLevelPermissionDataSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_level_permission_data_set: Option<RowLevelPermissionDataSet>,
    #[serde(rename = "RowLevelPermissionTagConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_level_permission_tag_configuration: Option<RowLevelPermissionTagConfiguration>,
    #[serde(rename = "SemanticModelConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_model_configuration: Option<SemanticModelConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDataSetResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "DataSetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_set_id: Option<String>,
    #[serde(rename = "IngestionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_arn: Option<String>,
    #[serde(rename = "IngestionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_id: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDataSourcePermissionsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DataSourceId")]
    #[serde(default)]
    pub data_source_id: String,
    #[serde(rename = "GrantPermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "RevokePermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_permissions: Option<Vec<ResourcePermission>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDataSourcePermissionsResponse {
    #[serde(rename = "DataSourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_arn: Option<String>,
    #[serde(rename = "DataSourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDataSourceRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Credentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<DataSourceCredentials>,
    #[serde(rename = "DataSourceId")]
    #[serde(default)]
    pub data_source_id: String,
    #[serde(rename = "DataSourceParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_parameters: Option<DataSourceParameters>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "SslProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_properties: Option<SslProperties>,
    #[serde(rename = "VpcConnectionProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_connection_properties: Option<VpcConnectionProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDataSourceResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "DataSourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "UpdateStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDefaultQBusinessApplicationRequest {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDefaultQBusinessApplicationResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFlowPermissionsInput {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "FlowId")]
    #[serde(default)]
    pub flow_id: String,
    #[serde(rename = "GrantPermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_permissions: Option<Vec<Permission>>,
    #[serde(rename = "RevokePermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_permissions: Option<Vec<Permission>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFlowPermissionsOutput {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "FlowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_id: Option<String>,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<Permission>>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFolderPermissionsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "FolderId")]
    #[serde(default)]
    pub folder_id: String,
    #[serde(rename = "GrantPermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "RevokePermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_permissions: Option<Vec<ResourcePermission>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFolderPermissionsResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "FolderId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFolderRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "FolderId")]
    #[serde(default)]
    pub folder_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFolderResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "FolderId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGroupRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "GroupName")]
    #[serde(default)]
    pub group_name: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGroupResponse {
    #[serde(rename = "Group")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIAMPolicyAssignmentRequest {
    #[serde(rename = "AssignmentName")]
    #[serde(default)]
    pub assignment_name: String,
    #[serde(rename = "AssignmentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_status: Option<String>,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Identities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identities: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "PolicyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIAMPolicyAssignmentResponse {
    #[serde(rename = "AssignmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_id: Option<String>,
    #[serde(rename = "AssignmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_name: Option<String>,
    #[serde(rename = "AssignmentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignment_status: Option<String>,
    #[serde(rename = "Identities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identities: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "PolicyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_arn: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIdentityPropagationConfigRequest {
    #[serde(rename = "AuthorizedTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_targets: Option<Vec<String>>,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Service")]
    #[serde(default)]
    pub service: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIdentityPropagationConfigResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIpRestrictionRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "IpRestrictionRuleMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_restriction_rule_map: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "VpcEndpointIdRestrictionRuleMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_id_restriction_rule_map: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "VpcIdRestrictionRuleMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id_restriction_rule_map: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIpRestrictionResponse {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_account_id: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateKeyRegistrationRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "KeyRegistration")]
    #[serde(default)]
    pub key_registration: Vec<RegisteredCustomerManagedKey>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateKeyRegistrationResponse {
    #[serde(rename = "FailedKeyRegistration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_key_registration: Option<Vec<FailedKeyRegistrationEntry>>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "SuccessfulKeyRegistration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_key_registration: Option<Vec<SuccessfulKeyRegistrationEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailedKeyRegistrationEntry {
    #[serde(rename = "KeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_arn: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "SenderFault")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_fault: Option<bool>,
    #[serde(rename = "StatusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SuccessfulKeyRegistrationEntry {
    #[serde(rename = "KeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_arn: Option<String>,
    #[serde(rename = "StatusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePublicSharingSettingsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "PublicSharingEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_sharing_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePublicSharingSettingsResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateQPersonalizationConfigurationRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "PersonalizationMode")]
    #[serde(default)]
    pub personalization_mode: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateQPersonalizationConfigurationResponse {
    #[serde(rename = "PersonalizationMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personalization_mode: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateQuickSightQSearchConfigurationRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "QSearchStatus")]
    #[serde(default)]
    pub q_search_status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateQuickSightQSearchConfigurationResponse {
    #[serde(rename = "QSearchStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q_search_status: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRefreshScheduleRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DataSetId")]
    #[serde(default)]
    pub data_set_id: String,
    #[serde(rename = "Schedule")]
    #[serde(default)]
    pub schedule: RefreshSchedule,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRefreshScheduleResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "ScheduleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRoleCustomPermissionRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "CustomPermissionsName")]
    #[serde(default)]
    pub custom_permissions_name: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "Role")]
    #[serde(default)]
    pub role: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRoleCustomPermissionResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSPICECapacityConfigurationRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "PurchaseMode")]
    #[serde(default)]
    pub purchase_mode: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSPICECapacityConfigurationResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSelfUpgradeConfigurationRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "SelfUpgradeStatus")]
    #[serde(default)]
    pub self_upgrade_status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSelfUpgradeConfigurationResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSelfUpgradeRequest {
    #[serde(rename = "Action")]
    #[serde(default)]
    pub action: String,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "UpgradeRequestId")]
    #[serde(default)]
    pub upgrade_request_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSelfUpgradeResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "SelfUpgradeRequestDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_upgrade_request_detail: Option<SelfUpgradeRequestDetail>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTemplateAliasRequest {
    #[serde(rename = "AliasName")]
    #[serde(default)]
    pub alias_name: String,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "TemplateId")]
    #[serde(default)]
    pub template_id: String,
    #[serde(rename = "TemplateVersionNumber")]
    #[serde(default)]
    pub template_version_number: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTemplateAliasResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "TemplateAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_alias: Option<TemplateAlias>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTemplatePermissionsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "GrantPermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "RevokePermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "TemplateId")]
    #[serde(default)]
    pub template_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTemplatePermissionsResponse {
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "TemplateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_arn: Option<String>,
    #[serde(rename = "TemplateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTemplateRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "Definition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<TemplateVersionDefinition>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SourceEntity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_entity: Option<TemplateSourceEntity>,
    #[serde(rename = "TemplateId")]
    #[serde(default)]
    pub template_id: String,
    #[serde(rename = "ValidationStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_strategy: Option<ValidationStrategy>,
    #[serde(rename = "VersionDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTemplateResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_status: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "TemplateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(rename = "VersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateThemeAliasRequest {
    #[serde(rename = "AliasName")]
    #[serde(default)]
    pub alias_name: String,
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "ThemeId")]
    #[serde(default)]
    pub theme_id: String,
    #[serde(rename = "ThemeVersionNumber")]
    #[serde(default)]
    pub theme_version_number: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateThemeAliasResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "ThemeAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_alias: Option<ThemeAlias>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateThemePermissionsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "GrantPermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "RevokePermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "ThemeId")]
    #[serde(default)]
    pub theme_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateThemePermissionsResponse {
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "ThemeArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_arn: Option<String>,
    #[serde(rename = "ThemeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateThemeRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "BaseThemeId")]
    #[serde(default)]
    pub base_theme_id: String,
    #[serde(rename = "Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ThemeConfiguration>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ThemeId")]
    #[serde(default)]
    pub theme_id: String,
    #[serde(rename = "VersionDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateThemeResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_status: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "ThemeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_id: Option<String>,
    #[serde(rename = "VersionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTopicPermissionsRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "GrantPermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "RevokePermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "TopicId")]
    #[serde(default)]
    pub topic_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTopicPermissionsResponse {
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<ResourcePermission>>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "TopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
    #[serde(rename = "TopicId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTopicRefreshScheduleRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DatasetId")]
    #[serde(default)]
    pub dataset_id: String,
    #[serde(rename = "RefreshSchedule")]
    #[serde(default)]
    pub refresh_schedule: TopicRefreshSchedule,
    #[serde(rename = "TopicId")]
    #[serde(default)]
    pub topic_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTopicRefreshScheduleResponse {
    #[serde(rename = "DatasetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arn: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "TopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
    #[serde(rename = "TopicId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTopicRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "CustomInstructions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_instructions: Option<CustomInstructions>,
    #[serde(rename = "Topic")]
    #[serde(default)]
    pub topic: TopicDetails,
    #[serde(rename = "TopicId")]
    #[serde(default)]
    pub topic_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTopicResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "RefreshArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_arn: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "TopicId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUserCustomPermissionRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "CustomPermissionsName")]
    #[serde(default)]
    pub custom_permissions_name: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUserCustomPermissionResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUserRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "CustomFederationProviderUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_federation_provider_url: Option<String>,
    #[serde(rename = "CustomPermissionsName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_permissions_name: Option<String>,
    #[serde(rename = "Email")]
    #[serde(default)]
    pub email: String,
    #[serde(rename = "ExternalLoginFederationProviderType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_login_federation_provider_type: Option<String>,
    #[serde(rename = "ExternalLoginId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_login_id: Option<String>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "Role")]
    #[serde(default)]
    pub role: String,
    #[serde(rename = "UnapplyCustomPermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unapply_custom_permissions: Option<bool>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    pub user_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUserResponse {
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "User")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateVPCConnectionRequest {
    #[serde(rename = "AwsAccountId")]
    #[serde(default)]
    pub aws_account_id: String,
    #[serde(rename = "DnsResolvers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_resolvers: Option<Vec<String>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    pub security_group_ids: Vec<String>,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    pub subnet_ids: Vec<String>,
    #[serde(rename = "VPCConnectionId")]
    #[serde(default)]
    pub v_p_c_connection_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateVPCConnectionResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "AvailabilityStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_status: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "UpdateStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_status: Option<String>,
    #[serde(rename = "VPCConnectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_p_c_connection_id: Option<String>,
}
