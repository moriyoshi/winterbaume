//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-sesv2

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetMetricDataRequest {
    #[serde(rename = "Queries")]
    #[serde(default)]
    pub queries: Vec<BatchGetMetricDataQuery>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetMetricDataQuery {
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "EndDate")]
    #[serde(default)]
    pub end_date: f64,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Metric")]
    #[serde(default)]
    pub metric: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "StartDate")]
    #[serde(default)]
    pub start_date: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetMetricDataResponse {
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<MetricDataError>>,
    #[serde(rename = "Results")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<MetricDataResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricDataError {
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricDataResult {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Timestamps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamps: Option<Vec<f64>>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<i64>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelExportJobRequest {
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelExportJobResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConfigurationSetEventDestinationRequest {
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    pub configuration_set_name: String,
    #[serde(rename = "EventDestination")]
    #[serde(default)]
    pub event_destination: EventDestinationDefinition,
    #[serde(rename = "EventDestinationName")]
    #[serde(default)]
    pub event_destination_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventDestinationDefinition {
    #[serde(rename = "CloudWatchDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_destination: Option<CloudWatchDestination>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "EventBridgeDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bridge_destination: Option<EventBridgeDestination>,
    #[serde(rename = "KinesisFirehoseDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_firehose_destination: Option<KinesisFirehoseDestination>,
    #[serde(rename = "MatchingEventTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matching_event_types: Option<Vec<String>>,
    #[serde(rename = "PinpointDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinpoint_destination: Option<PinpointDestination>,
    #[serde(rename = "SnsDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_destination: Option<SnsDestination>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudWatchDestination {
    #[serde(rename = "DimensionConfigurations")]
    #[serde(default)]
    pub dimension_configurations: Vec<CloudWatchDimensionConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudWatchDimensionConfiguration {
    #[serde(rename = "DefaultDimensionValue")]
    #[serde(default)]
    pub default_dimension_value: String,
    #[serde(rename = "DimensionName")]
    #[serde(default)]
    pub dimension_name: String,
    #[serde(rename = "DimensionValueSource")]
    #[serde(default)]
    pub dimension_value_source: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventBridgeDestination {
    #[serde(rename = "EventBusArn")]
    #[serde(default)]
    pub event_bus_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KinesisFirehoseDestination {
    #[serde(rename = "DeliveryStreamArn")]
    #[serde(default)]
    pub delivery_stream_arn: String,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    pub iam_role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PinpointDestination {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnsDestination {
    #[serde(rename = "TopicArn")]
    #[serde(default)]
    pub topic_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConfigurationSetEventDestinationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConfigurationSetRequest {
    #[serde(rename = "ArchivingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archiving_options: Option<ArchivingOptions>,
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    pub configuration_set_name: String,
    #[serde(rename = "DeliveryOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_options: Option<DeliveryOptions>,
    #[serde(rename = "ReputationOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reputation_options: Option<ReputationOptions>,
    #[serde(rename = "SendingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_options: Option<SendingOptions>,
    #[serde(rename = "SuppressionOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppression_options: Option<SuppressionOptions>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TrackingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_options: Option<TrackingOptions>,
    #[serde(rename = "VdmOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vdm_options: Option<VdmOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ArchivingOptions {
    #[serde(rename = "ArchiveArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeliveryOptions {
    #[serde(rename = "MaxDeliverySeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_delivery_seconds: Option<i64>,
    #[serde(rename = "SendingPoolName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_pool_name: Option<String>,
    #[serde(rename = "TlsPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReputationOptions {
    #[serde(rename = "LastFreshStart")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_fresh_start: Option<f64>,
    #[serde(rename = "ReputationMetricsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reputation_metrics_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendingOptions {
    #[serde(rename = "SendingEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SuppressionOptions {
    #[serde(rename = "SuppressedReasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppressed_reasons: Option<Vec<String>>,
    #[serde(rename = "ValidationOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_options: Option<SuppressionValidationOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SuppressionValidationOptions {
    #[serde(rename = "ConditionThreshold")]
    #[serde(default)]
    pub condition_threshold: SuppressionConditionThreshold,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SuppressionConditionThreshold {
    #[serde(rename = "ConditionThresholdEnabled")]
    #[serde(default)]
    pub condition_threshold_enabled: String,
    #[serde(rename = "OverallConfidenceThreshold")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall_confidence_threshold: Option<SuppressionConfidenceThreshold>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SuppressionConfidenceThreshold {
    #[serde(rename = "ConfidenceVerdictThreshold")]
    #[serde(default)]
    pub confidence_verdict_threshold: String,
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
pub struct TrackingOptions {
    #[serde(rename = "CustomRedirectDomain")]
    #[serde(default)]
    pub custom_redirect_domain: String,
    #[serde(rename = "HttpsPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub https_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VdmOptions {
    #[serde(rename = "DashboardOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_options: Option<DashboardOptions>,
    #[serde(rename = "GuardianOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardian_options: Option<GuardianOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DashboardOptions {
    #[serde(rename = "EngagementMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engagement_metrics: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuardianOptions {
    #[serde(rename = "OptimizedSharedDelivery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimized_shared_delivery: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConfigurationSetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateContactListRequest {
    #[serde(rename = "ContactListName")]
    #[serde(default)]
    pub contact_list_name: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Topics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<Topic>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Topic {
    #[serde(rename = "DefaultSubscriptionStatus")]
    #[serde(default)]
    pub default_subscription_status: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    pub display_name: String,
    #[serde(rename = "TopicName")]
    #[serde(default)]
    pub topic_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateContactListResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateContactRequest {
    #[serde(rename = "AttributesData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_data: Option<String>,
    #[serde(rename = "ContactListName")]
    #[serde(default)]
    pub contact_list_name: String,
    #[serde(rename = "EmailAddress")]
    #[serde(default)]
    pub email_address: String,
    #[serde(rename = "TopicPreferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_preferences: Option<Vec<TopicPreference>>,
    #[serde(rename = "UnsubscribeAll")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscribe_all: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicPreference {
    #[serde(rename = "SubscriptionStatus")]
    #[serde(default)]
    pub subscription_status: String,
    #[serde(rename = "TopicName")]
    #[serde(default)]
    pub topic_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateContactResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCustomVerificationEmailTemplateRequest {
    #[serde(rename = "FailureRedirectionURL")]
    #[serde(default)]
    pub failure_redirection_u_r_l: String,
    #[serde(rename = "FromEmailAddress")]
    #[serde(default)]
    pub from_email_address: String,
    #[serde(rename = "SuccessRedirectionURL")]
    #[serde(default)]
    pub success_redirection_u_r_l: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TemplateContent")]
    #[serde(default)]
    pub template_content: String,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    pub template_name: String,
    #[serde(rename = "TemplateSubject")]
    #[serde(default)]
    pub template_subject: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCustomVerificationEmailTemplateResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDedicatedIpPoolRequest {
    #[serde(rename = "PoolName")]
    #[serde(default)]
    pub pool_name: String,
    #[serde(rename = "ScalingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_mode: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDedicatedIpPoolResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDeliverabilityTestReportRequest {
    #[serde(rename = "Content")]
    #[serde(default)]
    pub content: EmailContent,
    #[serde(rename = "FromEmailAddress")]
    #[serde(default)]
    pub from_email_address: String,
    #[serde(rename = "ReportName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_name: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EmailContent {
    #[serde(rename = "Raw")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw: Option<RawMessage>,
    #[serde(rename = "Simple")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple: Option<Message>,
    #[serde(rename = "Template")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<Template>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RawMessage {
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Message {
    #[serde(rename = "Attachments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    #[serde(rename = "Body")]
    #[serde(default)]
    pub body: Body,
    #[serde(rename = "Headers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<MessageHeader>>,
    #[serde(rename = "Subject")]
    #[serde(default)]
    pub subject: Content,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Attachment {
    #[serde(rename = "ContentDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_description: Option<String>,
    #[serde(rename = "ContentDisposition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_disposition: Option<String>,
    #[serde(rename = "ContentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_id: Option<String>,
    #[serde(rename = "ContentTransferEncoding")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_transfer_encoding: Option<String>,
    #[serde(rename = "ContentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "FileName")]
    #[serde(default)]
    pub file_name: String,
    #[serde(rename = "RawContent")]
    #[serde(default)]
    pub raw_content: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Body {
    #[serde(rename = "Html")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<Content>,
    #[serde(rename = "Text")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Content>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Content {
    #[serde(rename = "Charset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charset: Option<String>,
    #[serde(rename = "Data")]
    #[serde(default)]
    pub data: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MessageHeader {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Template {
    #[serde(rename = "Attachments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    #[serde(rename = "Headers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<MessageHeader>>,
    #[serde(rename = "TemplateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_arn: Option<String>,
    #[serde(rename = "TemplateContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_content: Option<EmailTemplateContent>,
    #[serde(rename = "TemplateData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_data: Option<String>,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EmailTemplateContent {
    #[serde(rename = "Html")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(rename = "Subject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(rename = "Text")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDeliverabilityTestReportResponse {
    #[serde(rename = "DeliverabilityTestStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliverability_test_status: Option<String>,
    #[serde(rename = "ReportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEmailIdentityPolicyRequest {
    #[serde(rename = "EmailIdentity")]
    #[serde(default)]
    pub email_identity: String,
    #[serde(rename = "Policy")]
    #[serde(default)]
    pub policy: String,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    pub policy_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEmailIdentityPolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEmailIdentityRequest {
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_set_name: Option<String>,
    #[serde(rename = "DkimSigningAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkim_signing_attributes: Option<DkimSigningAttributes>,
    #[serde(rename = "EmailIdentity")]
    #[serde(default)]
    pub email_identity: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DkimSigningAttributes {
    #[serde(rename = "DomainSigningAttributesOrigin")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_signing_attributes_origin: Option<String>,
    #[serde(rename = "DomainSigningPrivateKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_signing_private_key: Option<String>,
    #[serde(rename = "DomainSigningSelector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_signing_selector: Option<String>,
    #[serde(rename = "NextSigningKeyLength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_signing_key_length: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEmailIdentityResponse {
    #[serde(rename = "DkimAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkim_attributes: Option<DkimAttributes>,
    #[serde(rename = "IdentityType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_type: Option<String>,
    #[serde(rename = "VerifiedForSendingStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_for_sending_status: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DkimAttributes {
    #[serde(rename = "CurrentSigningKeyLength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_signing_key_length: Option<String>,
    #[serde(rename = "LastKeyGenerationTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_key_generation_timestamp: Option<f64>,
    #[serde(rename = "NextSigningKeyLength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_signing_key_length: Option<String>,
    #[serde(rename = "SigningAttributesOrigin")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_attributes_origin: Option<String>,
    #[serde(rename = "SigningEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_enabled: Option<bool>,
    #[serde(rename = "SigningHostedZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_hosted_zone: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tokens")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEmailTemplateRequest {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TemplateContent")]
    #[serde(default)]
    pub template_content: EmailTemplateContent,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    pub template_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEmailTemplateResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateExportJobRequest {
    #[serde(rename = "ExportDataSource")]
    #[serde(default)]
    pub export_data_source: ExportDataSource,
    #[serde(rename = "ExportDestination")]
    #[serde(default)]
    pub export_destination: ExportDestination,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportDataSource {
    #[serde(rename = "MessageInsightsDataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_insights_data_source: Option<MessageInsightsDataSource>,
    #[serde(rename = "MetricsDataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_data_source: Option<MetricsDataSource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MessageInsightsDataSource {
    #[serde(rename = "EndDate")]
    #[serde(default)]
    pub end_date: f64,
    #[serde(rename = "Exclude")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<MessageInsightsFilters>,
    #[serde(rename = "Include")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<MessageInsightsFilters>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "StartDate")]
    #[serde(default)]
    pub start_date: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MessageInsightsFilters {
    #[serde(rename = "Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<Vec<String>>,
    #[serde(rename = "FromEmailAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_email_address: Option<Vec<String>>,
    #[serde(rename = "Isp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp: Option<Vec<String>>,
    #[serde(rename = "LastDeliveryEvent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_delivery_event: Option<Vec<String>>,
    #[serde(rename = "LastEngagementEvent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_engagement_event: Option<Vec<String>>,
    #[serde(rename = "Subject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricsDataSource {
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    pub dimensions: std::collections::HashMap<String, Vec<String>>,
    #[serde(rename = "EndDate")]
    #[serde(default)]
    pub end_date: f64,
    #[serde(rename = "Metrics")]
    #[serde(default)]
    pub metrics: Vec<ExportMetric>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "StartDate")]
    #[serde(default)]
    pub start_date: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportMetric {
    #[serde(rename = "Aggregation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportDestination {
    #[serde(rename = "DataFormat")]
    #[serde(default)]
    pub data_format: String,
    #[serde(rename = "S3Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateExportJobResponse {
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateImportJobRequest {
    #[serde(rename = "ImportDataSource")]
    #[serde(default)]
    pub import_data_source: ImportDataSource,
    #[serde(rename = "ImportDestination")]
    #[serde(default)]
    pub import_destination: ImportDestination,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportDataSource {
    #[serde(rename = "DataFormat")]
    #[serde(default)]
    pub data_format: String,
    #[serde(rename = "S3Url")]
    #[serde(default)]
    pub s3_url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportDestination {
    #[serde(rename = "ContactListDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_list_destination: Option<ContactListDestination>,
    #[serde(rename = "SuppressionListDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppression_list_destination: Option<SuppressionListDestination>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContactListDestination {
    #[serde(rename = "ContactListImportAction")]
    #[serde(default)]
    pub contact_list_import_action: String,
    #[serde(rename = "ContactListName")]
    #[serde(default)]
    pub contact_list_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SuppressionListDestination {
    #[serde(rename = "SuppressionListImportAction")]
    #[serde(default)]
    pub suppression_list_import_action: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateImportJobResponse {
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMultiRegionEndpointRequest {
    #[serde(rename = "Details")]
    #[serde(default)]
    pub details: Details,
    #[serde(rename = "EndpointName")]
    #[serde(default)]
    pub endpoint_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Details {
    #[serde(rename = "RoutesDetails")]
    #[serde(default)]
    pub routes_details: Vec<RouteDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RouteDetails {
    #[serde(rename = "Region")]
    #[serde(default)]
    pub region: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateMultiRegionEndpointResponse {
    #[serde(rename = "EndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTenantRequest {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TenantName")]
    #[serde(default)]
    pub tenant_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTenantResourceAssociationRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "TenantName")]
    #[serde(default)]
    pub tenant_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTenantResourceAssociationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTenantResponse {
    #[serde(rename = "CreatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    #[serde(rename = "SendingStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TenantArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_arn: Option<String>,
    #[serde(rename = "TenantId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "TenantName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConfigurationSetEventDestinationRequest {
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    pub configuration_set_name: String,
    #[serde(rename = "EventDestinationName")]
    #[serde(default)]
    pub event_destination_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConfigurationSetEventDestinationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConfigurationSetRequest {
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    pub configuration_set_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConfigurationSetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteContactListRequest {
    #[serde(rename = "ContactListName")]
    #[serde(default)]
    pub contact_list_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteContactListResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteContactRequest {
    #[serde(rename = "ContactListName")]
    #[serde(default)]
    pub contact_list_name: String,
    #[serde(rename = "EmailAddress")]
    #[serde(default)]
    pub email_address: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteContactResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCustomVerificationEmailTemplateRequest {
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    pub template_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCustomVerificationEmailTemplateResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDedicatedIpPoolRequest {
    #[serde(rename = "PoolName")]
    #[serde(default)]
    pub pool_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDedicatedIpPoolResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEmailIdentityPolicyRequest {
    #[serde(rename = "EmailIdentity")]
    #[serde(default)]
    pub email_identity: String,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    pub policy_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEmailIdentityPolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEmailIdentityRequest {
    #[serde(rename = "EmailIdentity")]
    #[serde(default)]
    pub email_identity: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEmailIdentityResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEmailTemplateRequest {
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    pub template_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEmailTemplateResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMultiRegionEndpointRequest {
    #[serde(rename = "EndpointName")]
    #[serde(default)]
    pub endpoint_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteMultiRegionEndpointResponse {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSuppressedDestinationRequest {
    #[serde(rename = "EmailAddress")]
    #[serde(default)]
    pub email_address: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSuppressedDestinationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTenantRequest {
    #[serde(rename = "TenantName")]
    #[serde(default)]
    pub tenant_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTenantResourceAssociationRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "TenantName")]
    #[serde(default)]
    pub tenant_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTenantResourceAssociationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTenantResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccountRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccountResponse {
    #[serde(rename = "DedicatedIpAutoWarmupEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_ip_auto_warmup_enabled: Option<bool>,
    #[serde(rename = "Details")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<AccountDetails>,
    #[serde(rename = "EnforcementStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforcement_status: Option<String>,
    #[serde(rename = "ProductionAccessEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production_access_enabled: Option<bool>,
    #[serde(rename = "SendQuota")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_quota: Option<SendQuota>,
    #[serde(rename = "SendingEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_enabled: Option<bool>,
    #[serde(rename = "SuppressionAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppression_attributes: Option<SuppressionAttributes>,
    #[serde(rename = "VdmAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vdm_attributes: Option<VdmAttributes>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountDetails {
    #[serde(rename = "AdditionalContactEmailAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_contact_email_addresses: Option<Vec<String>>,
    #[serde(rename = "ContactLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_language: Option<String>,
    #[serde(rename = "MailType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_type: Option<String>,
    #[serde(rename = "ReviewDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_details: Option<ReviewDetails>,
    #[serde(rename = "UseCaseDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_case_description: Option<String>,
    #[serde(rename = "WebsiteURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_u_r_l: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReviewDetails {
    #[serde(rename = "CaseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendQuota {
    #[serde(rename = "Max24HourSend")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max24_hour_send: Option<f64>,
    #[serde(rename = "MaxSendRate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_send_rate: Option<f64>,
    #[serde(rename = "SentLast24Hours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_last24_hours: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SuppressionAttributes {
    #[serde(rename = "SuppressedReasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppressed_reasons: Option<Vec<String>>,
    #[serde(rename = "ValidationAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_attributes: Option<SuppressionValidationAttributes>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SuppressionValidationAttributes {
    #[serde(rename = "ConditionThreshold")]
    #[serde(default)]
    pub condition_threshold: SuppressionConditionThreshold,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VdmAttributes {
    #[serde(rename = "DashboardAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_attributes: Option<DashboardAttributes>,
    #[serde(rename = "GuardianAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardian_attributes: Option<GuardianAttributes>,
    #[serde(rename = "VdmEnabled")]
    #[serde(default)]
    pub vdm_enabled: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DashboardAttributes {
    #[serde(rename = "EngagementMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engagement_metrics: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuardianAttributes {
    #[serde(rename = "OptimizedSharedDelivery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimized_shared_delivery: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBlacklistReportsRequest {
    #[serde(rename = "BlacklistItemNames")]
    #[serde(default)]
    pub blacklist_item_names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBlacklistReportsResponse {
    #[serde(rename = "BlacklistReport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blacklist_report: Option<std::collections::HashMap<String, Vec<BlacklistEntry>>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BlacklistEntry {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ListingTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listing_time: Option<f64>,
    #[serde(rename = "RblName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rbl_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConfigurationSetEventDestinationsRequest {
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    pub configuration_set_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConfigurationSetEventDestinationsResponse {
    #[serde(rename = "EventDestinations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_destinations: Option<Vec<EventDestination>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventDestination {
    #[serde(rename = "CloudWatchDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_destination: Option<CloudWatchDestination>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "EventBridgeDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bridge_destination: Option<EventBridgeDestination>,
    #[serde(rename = "KinesisFirehoseDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_firehose_destination: Option<KinesisFirehoseDestination>,
    #[serde(rename = "MatchingEventTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matching_event_types: Option<Vec<String>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PinpointDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinpoint_destination: Option<PinpointDestination>,
    #[serde(rename = "SnsDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_destination: Option<SnsDestination>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConfigurationSetRequest {
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    pub configuration_set_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConfigurationSetResponse {
    #[serde(rename = "ArchivingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archiving_options: Option<ArchivingOptions>,
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_set_name: Option<String>,
    #[serde(rename = "DeliveryOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_options: Option<DeliveryOptions>,
    #[serde(rename = "ReputationOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reputation_options: Option<ReputationOptions>,
    #[serde(rename = "SendingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_options: Option<SendingOptions>,
    #[serde(rename = "SuppressionOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppression_options: Option<SuppressionOptions>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TrackingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_options: Option<TrackingOptions>,
    #[serde(rename = "VdmOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vdm_options: Option<VdmOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetContactListRequest {
    #[serde(rename = "ContactListName")]
    #[serde(default)]
    pub contact_list_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetContactListResponse {
    #[serde(rename = "ContactListName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_list_name: Option<String>,
    #[serde(rename = "CreatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<f64>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Topics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<Topic>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetContactRequest {
    #[serde(rename = "ContactListName")]
    #[serde(default)]
    pub contact_list_name: String,
    #[serde(rename = "EmailAddress")]
    #[serde(default)]
    pub email_address: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetContactResponse {
    #[serde(rename = "AttributesData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_data: Option<String>,
    #[serde(rename = "ContactListName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_list_name: Option<String>,
    #[serde(rename = "CreatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    #[serde(rename = "EmailAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<f64>,
    #[serde(rename = "TopicDefaultPreferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_default_preferences: Option<Vec<TopicPreference>>,
    #[serde(rename = "TopicPreferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_preferences: Option<Vec<TopicPreference>>,
    #[serde(rename = "UnsubscribeAll")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscribe_all: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCustomVerificationEmailTemplateRequest {
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    pub template_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCustomVerificationEmailTemplateResponse {
    #[serde(rename = "FailureRedirectionURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_redirection_u_r_l: Option<String>,
    #[serde(rename = "FromEmailAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_email_address: Option<String>,
    #[serde(rename = "SuccessRedirectionURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_redirection_u_r_l: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TemplateContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_content: Option<String>,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    #[serde(rename = "TemplateSubject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_subject: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDedicatedIpPoolRequest {
    #[serde(rename = "PoolName")]
    #[serde(default)]
    pub pool_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDedicatedIpPoolResponse {
    #[serde(rename = "DedicatedIpPool")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_ip_pool: Option<DedicatedIpPool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DedicatedIpPool {
    #[serde(rename = "PoolName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_name: Option<String>,
    #[serde(rename = "ScalingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDedicatedIpRequest {
    #[serde(rename = "Ip")]
    #[serde(default)]
    pub ip: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDedicatedIpResponse {
    #[serde(rename = "DedicatedIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_ip: Option<DedicatedIp>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DedicatedIp {
    #[serde(rename = "Ip")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(rename = "PoolName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_name: Option<String>,
    #[serde(rename = "WarmupPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warmup_percentage: Option<i32>,
    #[serde(rename = "WarmupStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warmup_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDedicatedIpsRequest {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "PoolName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDedicatedIpsResponse {
    #[serde(rename = "DedicatedIps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_ips: Option<Vec<DedicatedIp>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeliverabilityDashboardOptionsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeliverabilityDashboardOptionsResponse {
    #[serde(rename = "AccountStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_status: Option<String>,
    #[serde(rename = "ActiveSubscribedDomains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_subscribed_domains: Option<Vec<DomainDeliverabilityTrackingOption>>,
    #[serde(rename = "DashboardEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_enabled: Option<bool>,
    #[serde(rename = "PendingExpirationSubscribedDomains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_expiration_subscribed_domains: Option<Vec<DomainDeliverabilityTrackingOption>>,
    #[serde(rename = "SubscriptionExpiryDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_expiry_date: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainDeliverabilityTrackingOption {
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "InboxPlacementTrackingOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_placement_tracking_option: Option<InboxPlacementTrackingOption>,
    #[serde(rename = "SubscriptionStartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_start_date: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InboxPlacementTrackingOption {
    #[serde(rename = "Global")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global: Option<bool>,
    #[serde(rename = "TrackedIsps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracked_isps: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeliverabilityTestReportRequest {
    #[serde(rename = "ReportId")]
    #[serde(default)]
    pub report_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDeliverabilityTestReportResponse {
    #[serde(rename = "DeliverabilityTestReport")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliverability_test_report: Option<DeliverabilityTestReport>,
    #[serde(rename = "IspPlacements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp_placements: Option<Vec<IspPlacement>>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "OverallPlacement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall_placement: Option<PlacementStatistics>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeliverabilityTestReport {
    #[serde(rename = "CreateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_date: Option<f64>,
    #[serde(rename = "DeliverabilityTestStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliverability_test_status: Option<String>,
    #[serde(rename = "FromEmailAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_email_address: Option<String>,
    #[serde(rename = "ReportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_id: Option<String>,
    #[serde(rename = "ReportName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report_name: Option<String>,
    #[serde(rename = "Subject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IspPlacement {
    #[serde(rename = "IspName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp_name: Option<String>,
    #[serde(rename = "PlacementStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_statistics: Option<PlacementStatistics>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PlacementStatistics {
    #[serde(rename = "DkimPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkim_percentage: Option<f64>,
    #[serde(rename = "InboxPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_percentage: Option<f64>,
    #[serde(rename = "MissingPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_percentage: Option<f64>,
    #[serde(rename = "SpamPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spam_percentage: Option<f64>,
    #[serde(rename = "SpfPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spf_percentage: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDomainDeliverabilityCampaignRequest {
    #[serde(rename = "CampaignId")]
    #[serde(default)]
    pub campaign_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDomainDeliverabilityCampaignResponse {
    #[serde(rename = "DomainDeliverabilityCampaign")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_deliverability_campaign: Option<DomainDeliverabilityCampaign>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainDeliverabilityCampaign {
    #[serde(rename = "CampaignId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    #[serde(rename = "DeleteRate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_rate: Option<f64>,
    #[serde(rename = "Esps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub esps: Option<Vec<String>>,
    #[serde(rename = "FirstSeenDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_seen_date_time: Option<f64>,
    #[serde(rename = "FromAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_address: Option<String>,
    #[serde(rename = "ImageUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(rename = "InboxCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_count: Option<i64>,
    #[serde(rename = "LastSeenDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen_date_time: Option<f64>,
    #[serde(rename = "ProjectedVolume")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projected_volume: Option<i64>,
    #[serde(rename = "ReadDeleteRate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_delete_rate: Option<f64>,
    #[serde(rename = "ReadRate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_rate: Option<f64>,
    #[serde(rename = "SendingIps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_ips: Option<Vec<String>>,
    #[serde(rename = "SpamCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spam_count: Option<i64>,
    #[serde(rename = "Subject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDomainStatisticsReportRequest {
    #[serde(rename = "Domain")]
    #[serde(default)]
    pub domain: String,
    #[serde(rename = "EndDate")]
    #[serde(default)]
    pub end_date: f64,
    #[serde(rename = "StartDate")]
    #[serde(default)]
    pub start_date: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDomainStatisticsReportResponse {
    #[serde(rename = "DailyVolumes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_volumes: Option<Vec<DailyVolume>>,
    #[serde(rename = "OverallVolume")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall_volume: Option<OverallVolume>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DailyVolume {
    #[serde(rename = "DomainIspPlacements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_isp_placements: Option<Vec<DomainIspPlacement>>,
    #[serde(rename = "StartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<f64>,
    #[serde(rename = "VolumeStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_statistics: Option<VolumeStatistics>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainIspPlacement {
    #[serde(rename = "InboxPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_percentage: Option<f64>,
    #[serde(rename = "InboxRawCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_raw_count: Option<i64>,
    #[serde(rename = "IspName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp_name: Option<String>,
    #[serde(rename = "SpamPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spam_percentage: Option<f64>,
    #[serde(rename = "SpamRawCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spam_raw_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VolumeStatistics {
    #[serde(rename = "InboxRawCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbox_raw_count: Option<i64>,
    #[serde(rename = "ProjectedInbox")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projected_inbox: Option<i64>,
    #[serde(rename = "ProjectedSpam")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projected_spam: Option<i64>,
    #[serde(rename = "SpamRawCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spam_raw_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OverallVolume {
    #[serde(rename = "DomainIspPlacements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_isp_placements: Option<Vec<DomainIspPlacement>>,
    #[serde(rename = "ReadRatePercent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_rate_percent: Option<f64>,
    #[serde(rename = "VolumeStatistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_statistics: Option<VolumeStatistics>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEmailAddressInsightsRequest {
    #[serde(rename = "EmailAddress")]
    #[serde(default)]
    pub email_address: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEmailAddressInsightsResponse {
    #[serde(rename = "MailboxValidation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailbox_validation: Option<MailboxValidation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MailboxValidation {
    #[serde(rename = "Evaluations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluations: Option<EmailAddressInsightsMailboxEvaluations>,
    #[serde(rename = "IsValid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_valid: Option<EmailAddressInsightsVerdict>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EmailAddressInsightsMailboxEvaluations {
    #[serde(rename = "HasValidDnsRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_valid_dns_records: Option<EmailAddressInsightsVerdict>,
    #[serde(rename = "HasValidSyntax")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_valid_syntax: Option<EmailAddressInsightsVerdict>,
    #[serde(rename = "IsDisposable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_disposable: Option<EmailAddressInsightsVerdict>,
    #[serde(rename = "IsRandomInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_random_input: Option<EmailAddressInsightsVerdict>,
    #[serde(rename = "IsRoleAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_role_address: Option<EmailAddressInsightsVerdict>,
    #[serde(rename = "MailboxExists")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailbox_exists: Option<EmailAddressInsightsVerdict>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EmailAddressInsightsVerdict {
    #[serde(rename = "ConfidenceVerdict")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_verdict: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEmailIdentityPoliciesRequest {
    #[serde(rename = "EmailIdentity")]
    #[serde(default)]
    pub email_identity: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEmailIdentityPoliciesResponse {
    #[serde(rename = "Policies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEmailIdentityRequest {
    #[serde(rename = "EmailIdentity")]
    #[serde(default)]
    pub email_identity: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEmailIdentityResponse {
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_set_name: Option<String>,
    #[serde(rename = "DkimAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkim_attributes: Option<DkimAttributes>,
    #[serde(rename = "FeedbackForwardingStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_forwarding_status: Option<bool>,
    #[serde(rename = "IdentityType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_type: Option<String>,
    #[serde(rename = "MailFromAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_from_attributes: Option<MailFromAttributes>,
    #[serde(rename = "Policies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VerificationInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_info: Option<VerificationInfo>,
    #[serde(rename = "VerificationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_status: Option<String>,
    #[serde(rename = "VerifiedForSendingStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_for_sending_status: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MailFromAttributes {
    #[serde(rename = "BehaviorOnMxFailure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior_on_mx_failure: Option<String>,
    #[serde(rename = "MailFromDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_from_domain: Option<String>,
    #[serde(rename = "MailFromDomainStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_from_domain_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VerificationInfo {
    #[serde(rename = "ErrorType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,
    #[serde(rename = "LastCheckedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_checked_timestamp: Option<f64>,
    #[serde(rename = "LastSuccessTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_success_timestamp: Option<f64>,
    #[serde(rename = "SOARecord")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_o_a_record: Option<SOARecord>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SOARecord {
    #[serde(rename = "AdminEmail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_email: Option<String>,
    #[serde(rename = "PrimaryNameServer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_name_server: Option<String>,
    #[serde(rename = "SerialNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEmailTemplateRequest {
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    pub template_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEmailTemplateResponse {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TemplateContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_content: Option<EmailTemplateContent>,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetExportJobRequest {
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetExportJobResponse {
    #[serde(rename = "CompletedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_timestamp: Option<f64>,
    #[serde(rename = "CreatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    #[serde(rename = "ExportDataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_data_source: Option<ExportDataSource>,
    #[serde(rename = "ExportDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_destination: Option<ExportDestination>,
    #[serde(rename = "ExportSourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_source_type: Option<String>,
    #[serde(rename = "FailureInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_info: Option<FailureInfo>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "Statistics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<ExportStatistics>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailureInfo {
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "FailedRecordsS3Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_records_s3_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportStatistics {
    #[serde(rename = "ExportedRecordsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exported_records_count: Option<i32>,
    #[serde(rename = "ProcessedRecordsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_records_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetImportJobRequest {
    #[serde(rename = "JobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetImportJobResponse {
    #[serde(rename = "CompletedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_timestamp: Option<f64>,
    #[serde(rename = "CreatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    #[serde(rename = "FailedRecordsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_records_count: Option<i32>,
    #[serde(rename = "FailureInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_info: Option<FailureInfo>,
    #[serde(rename = "ImportDataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_data_source: Option<ImportDataSource>,
    #[serde(rename = "ImportDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_destination: Option<ImportDestination>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "ProcessedRecordsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_records_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMessageInsightsRequest {
    #[serde(rename = "MessageId")]
    #[serde(default)]
    pub message_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMessageInsightsResponse {
    #[serde(rename = "EmailTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_tags: Option<Vec<MessageTag>>,
    #[serde(rename = "FromEmailAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_email_address: Option<String>,
    #[serde(rename = "Insights")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insights: Option<Vec<EmailInsights>>,
    #[serde(rename = "MessageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(rename = "Subject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MessageTag {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EmailInsights {
    #[serde(rename = "Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(rename = "Events")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<InsightsEvent>>,
    #[serde(rename = "Isp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isp: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InsightsEvent {
    #[serde(rename = "Details")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<EventDetails>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventDetails {
    #[serde(rename = "Bounce")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounce: Option<Bounce>,
    #[serde(rename = "Complaint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complaint: Option<Complaint>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Bounce {
    #[serde(rename = "BounceSubType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounce_sub_type: Option<String>,
    #[serde(rename = "BounceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounce_type: Option<String>,
    #[serde(rename = "DiagnosticCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnostic_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Complaint {
    #[serde(rename = "ComplaintFeedbackType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complaint_feedback_type: Option<String>,
    #[serde(rename = "ComplaintSubType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complaint_sub_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMultiRegionEndpointRequest {
    #[serde(rename = "EndpointName")]
    #[serde(default)]
    pub endpoint_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMultiRegionEndpointResponse {
    #[serde(rename = "CreatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    #[serde(rename = "EndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    #[serde(rename = "EndpointName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_name: Option<String>,
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<f64>,
    #[serde(rename = "Routes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routes: Option<Vec<Route>>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Route {
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetReputationEntityRequest {
    #[serde(rename = "ReputationEntityReference")]
    #[serde(default)]
    pub reputation_entity_reference: String,
    #[serde(rename = "ReputationEntityType")]
    #[serde(default)]
    pub reputation_entity_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetReputationEntityResponse {
    #[serde(rename = "ReputationEntity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reputation_entity: Option<ReputationEntity>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReputationEntity {
    #[serde(rename = "AwsSesManagedStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_ses_managed_status: Option<StatusRecord>,
    #[serde(rename = "CustomerManagedStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_managed_status: Option<StatusRecord>,
    #[serde(rename = "ReputationEntityReference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reputation_entity_reference: Option<String>,
    #[serde(rename = "ReputationEntityType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reputation_entity_type: Option<String>,
    #[serde(rename = "ReputationImpact")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reputation_impact: Option<String>,
    #[serde(rename = "ReputationManagementPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reputation_management_policy: Option<String>,
    #[serde(rename = "SendingStatusAggregate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_status_aggregate: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StatusRecord {
    #[serde(rename = "Cause")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSuppressedDestinationRequest {
    #[serde(rename = "EmailAddress")]
    #[serde(default)]
    pub email_address: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSuppressedDestinationResponse {
    #[serde(rename = "SuppressedDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppressed_destination: Option<SuppressedDestination>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SuppressedDestination {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<SuppressedDestinationAttributes>,
    #[serde(rename = "EmailAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(rename = "LastUpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    #[serde(rename = "Reason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SuppressedDestinationAttributes {
    #[serde(rename = "FeedbackId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_id: Option<String>,
    #[serde(rename = "MessageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTenantRequest {
    #[serde(rename = "TenantName")]
    #[serde(default)]
    pub tenant_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTenantResponse {
    #[serde(rename = "Tenant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Tenant>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tenant {
    #[serde(rename = "CreatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    #[serde(rename = "SendingStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TenantArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_arn: Option<String>,
    #[serde(rename = "TenantId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "TenantName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConfigurationSetsRequest {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConfigurationSetsResponse {
    #[serde(rename = "ConfigurationSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_sets: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListContactListsRequest {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListContactListsResponse {
    #[serde(rename = "ContactLists")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_lists: Option<Vec<ContactList>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContactList {
    #[serde(rename = "ContactListName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_list_name: Option<String>,
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListContactsRequest {
    #[serde(rename = "ContactListName")]
    #[serde(default)]
    pub contact_list_name: String,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<ListContactsFilter>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListContactsFilter {
    #[serde(rename = "FilteredStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filtered_status: Option<String>,
    #[serde(rename = "TopicFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_filter: Option<TopicFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TopicFilter {
    #[serde(rename = "TopicName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
    #[serde(rename = "UseDefaultIfPreferenceUnavailable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_default_if_preference_unavailable: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListContactsResponse {
    #[serde(rename = "Contacts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Vec<Contact>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Contact {
    #[serde(rename = "EmailAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<f64>,
    #[serde(rename = "TopicDefaultPreferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_default_preferences: Option<Vec<TopicPreference>>,
    #[serde(rename = "TopicPreferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_preferences: Option<Vec<TopicPreference>>,
    #[serde(rename = "UnsubscribeAll")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscribe_all: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCustomVerificationEmailTemplatesRequest {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCustomVerificationEmailTemplatesResponse {
    #[serde(rename = "CustomVerificationEmailTemplates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_verification_email_templates: Option<Vec<CustomVerificationEmailTemplateMetadata>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomVerificationEmailTemplateMetadata {
    #[serde(rename = "FailureRedirectionURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_redirection_u_r_l: Option<String>,
    #[serde(rename = "FromEmailAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_email_address: Option<String>,
    #[serde(rename = "SuccessRedirectionURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_redirection_u_r_l: Option<String>,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    #[serde(rename = "TemplateSubject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_subject: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDedicatedIpPoolsRequest {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDedicatedIpPoolsResponse {
    #[serde(rename = "DedicatedIpPools")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_ip_pools: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDeliverabilityTestReportsRequest {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDeliverabilityTestReportsResponse {
    #[serde(rename = "DeliverabilityTestReports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliverability_test_reports: Option<Vec<DeliverabilityTestReport>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDomainDeliverabilityCampaignsRequest {
    #[serde(rename = "EndDate")]
    #[serde(default)]
    pub end_date: f64,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "StartDate")]
    #[serde(default)]
    pub start_date: f64,
    #[serde(rename = "SubscribedDomain")]
    #[serde(default)]
    pub subscribed_domain: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDomainDeliverabilityCampaignsResponse {
    #[serde(rename = "DomainDeliverabilityCampaigns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_deliverability_campaigns: Option<Vec<DomainDeliverabilityCampaign>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEmailIdentitiesRequest {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEmailIdentitiesResponse {
    #[serde(rename = "EmailIdentities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_identities: Option<Vec<IdentityInfo>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IdentityInfo {
    #[serde(rename = "IdentityName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_name: Option<String>,
    #[serde(rename = "IdentityType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_type: Option<String>,
    #[serde(rename = "SendingEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_enabled: Option<bool>,
    #[serde(rename = "VerificationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEmailTemplatesRequest {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEmailTemplatesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TemplatesMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub templates_metadata: Option<Vec<EmailTemplateMetadata>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EmailTemplateMetadata {
    #[serde(rename = "CreatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListExportJobsRequest {
    #[serde(rename = "ExportSourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_source_type: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListExportJobsResponse {
    #[serde(rename = "ExportJobs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_jobs: Option<Vec<ExportJobSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportJobSummary {
    #[serde(rename = "CompletedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_timestamp: Option<f64>,
    #[serde(rename = "CreatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    #[serde(rename = "ExportSourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_source_type: Option<String>,
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
pub struct ListImportJobsRequest {
    #[serde(rename = "ImportDestinationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_destination_type: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListImportJobsResponse {
    #[serde(rename = "ImportJobs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_jobs: Option<Vec<ImportJobSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportJobSummary {
    #[serde(rename = "CreatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    #[serde(rename = "FailedRecordsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_records_count: Option<i32>,
    #[serde(rename = "ImportDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_destination: Option<ImportDestination>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "ProcessedRecordsCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_records_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMultiRegionEndpointsRequest {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMultiRegionEndpointsResponse {
    #[serde(rename = "MultiRegionEndpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_region_endpoints: Option<Vec<MultiRegionEndpoint>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiRegionEndpoint {
    #[serde(rename = "CreatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    #[serde(rename = "EndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    #[serde(rename = "EndpointName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_name: Option<String>,
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<f64>,
    #[serde(rename = "Regions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRecommendationsRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRecommendationsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Recommendations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendations: Option<Vec<Recommendation>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Recommendation {
    #[serde(rename = "CreatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Impact")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact: Option<String>,
    #[serde(rename = "LastUpdatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<f64>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
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
pub struct ListReputationEntitiesRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListReputationEntitiesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ReputationEntities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reputation_entities: Option<Vec<ReputationEntity>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourceTenantsRequest {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResourceTenantsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceTenants")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tenants: Option<Vec<ResourceTenantMetadata>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceTenantMetadata {
    #[serde(rename = "AssociatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_timestamp: Option<f64>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "TenantId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "TenantName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSuppressedDestinationsRequest {
    #[serde(rename = "EndDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<f64>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "Reasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasons: Option<Vec<String>>,
    #[serde(rename = "StartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSuppressedDestinationsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SuppressedDestinationSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppressed_destination_summaries: Option<Vec<SuppressedDestinationSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SuppressedDestinationSummary {
    #[serde(rename = "EmailAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(rename = "LastUpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    #[serde(rename = "Reason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTenantResourcesRequest {
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "TenantName")]
    #[serde(default)]
    pub tenant_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTenantResourcesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TenantResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_resources: Option<Vec<TenantResource>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TenantResource {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTenantsRequest {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTenantsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Tenants")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenants: Option<Vec<TenantInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TenantInfo {
    #[serde(rename = "CreatedTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<f64>,
    #[serde(rename = "TenantArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_arn: Option<String>,
    #[serde(rename = "TenantId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "TenantName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAccountDedicatedIpWarmupAttributesRequest {
    #[serde(rename = "AutoWarmupEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_warmup_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAccountDedicatedIpWarmupAttributesResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAccountDetailsRequest {
    #[serde(rename = "AdditionalContactEmailAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_contact_email_addresses: Option<Vec<String>>,
    #[serde(rename = "ContactLanguage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_language: Option<String>,
    #[serde(rename = "MailType")]
    #[serde(default)]
    pub mail_type: String,
    #[serde(rename = "ProductionAccessEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production_access_enabled: Option<bool>,
    #[serde(rename = "UseCaseDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_case_description: Option<String>,
    #[serde(rename = "WebsiteURL")]
    #[serde(default)]
    pub website_u_r_l: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAccountDetailsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAccountSendingAttributesRequest {
    #[serde(rename = "SendingEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAccountSendingAttributesResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAccountSuppressionAttributesRequest {
    #[serde(rename = "SuppressedReasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppressed_reasons: Option<Vec<String>>,
    #[serde(rename = "ValidationAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_attributes: Option<SuppressionValidationAttributes>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAccountSuppressionAttributesResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAccountVdmAttributesRequest {
    #[serde(rename = "VdmAttributes")]
    #[serde(default)]
    pub vdm_attributes: VdmAttributes,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAccountVdmAttributesResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutConfigurationSetArchivingOptionsRequest {
    #[serde(rename = "ArchiveArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_arn: Option<String>,
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    pub configuration_set_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutConfigurationSetArchivingOptionsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutConfigurationSetDeliveryOptionsRequest {
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    pub configuration_set_name: String,
    #[serde(rename = "MaxDeliverySeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_delivery_seconds: Option<i64>,
    #[serde(rename = "SendingPoolName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_pool_name: Option<String>,
    #[serde(rename = "TlsPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutConfigurationSetDeliveryOptionsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutConfigurationSetReputationOptionsRequest {
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    pub configuration_set_name: String,
    #[serde(rename = "ReputationMetricsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reputation_metrics_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutConfigurationSetReputationOptionsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutConfigurationSetSendingOptionsRequest {
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    pub configuration_set_name: String,
    #[serde(rename = "SendingEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutConfigurationSetSendingOptionsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutConfigurationSetSuppressionOptionsRequest {
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    pub configuration_set_name: String,
    #[serde(rename = "SuppressedReasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppressed_reasons: Option<Vec<String>>,
    #[serde(rename = "ValidationOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_options: Option<SuppressionValidationOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutConfigurationSetSuppressionOptionsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutConfigurationSetTrackingOptionsRequest {
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    pub configuration_set_name: String,
    #[serde(rename = "CustomRedirectDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_redirect_domain: Option<String>,
    #[serde(rename = "HttpsPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub https_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutConfigurationSetTrackingOptionsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutConfigurationSetVdmOptionsRequest {
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    pub configuration_set_name: String,
    #[serde(rename = "VdmOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vdm_options: Option<VdmOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutConfigurationSetVdmOptionsResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutDedicatedIpInPoolRequest {
    #[serde(rename = "DestinationPoolName")]
    #[serde(default)]
    pub destination_pool_name: String,
    #[serde(rename = "Ip")]
    #[serde(default)]
    pub ip: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutDedicatedIpInPoolResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutDedicatedIpPoolScalingAttributesRequest {
    #[serde(rename = "PoolName")]
    #[serde(default)]
    pub pool_name: String,
    #[serde(rename = "ScalingMode")]
    #[serde(default)]
    pub scaling_mode: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutDedicatedIpPoolScalingAttributesResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutDedicatedIpWarmupAttributesRequest {
    #[serde(rename = "Ip")]
    #[serde(default)]
    pub ip: String,
    #[serde(rename = "WarmupPercentage")]
    #[serde(default)]
    pub warmup_percentage: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutDedicatedIpWarmupAttributesResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutDeliverabilityDashboardOptionRequest {
    #[serde(rename = "DashboardEnabled")]
    #[serde(default)]
    pub dashboard_enabled: bool,
    #[serde(rename = "SubscribedDomains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribed_domains: Option<Vec<DomainDeliverabilityTrackingOption>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutDeliverabilityDashboardOptionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutEmailIdentityConfigurationSetAttributesRequest {
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_set_name: Option<String>,
    #[serde(rename = "EmailIdentity")]
    #[serde(default)]
    pub email_identity: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutEmailIdentityConfigurationSetAttributesResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutEmailIdentityDkimAttributesRequest {
    #[serde(rename = "EmailIdentity")]
    #[serde(default)]
    pub email_identity: String,
    #[serde(rename = "SigningEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutEmailIdentityDkimAttributesResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutEmailIdentityDkimSigningAttributesRequest {
    #[serde(rename = "EmailIdentity")]
    #[serde(default)]
    pub email_identity: String,
    #[serde(rename = "SigningAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_attributes: Option<DkimSigningAttributes>,
    #[serde(rename = "SigningAttributesOrigin")]
    #[serde(default)]
    pub signing_attributes_origin: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutEmailIdentityDkimSigningAttributesResponse {
    #[serde(rename = "DkimStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkim_status: Option<String>,
    #[serde(rename = "DkimTokens")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkim_tokens: Option<Vec<String>>,
    #[serde(rename = "SigningHostedZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_hosted_zone: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutEmailIdentityFeedbackAttributesRequest {
    #[serde(rename = "EmailForwardingEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_forwarding_enabled: Option<bool>,
    #[serde(rename = "EmailIdentity")]
    #[serde(default)]
    pub email_identity: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutEmailIdentityFeedbackAttributesResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutEmailIdentityMailFromAttributesRequest {
    #[serde(rename = "BehaviorOnMxFailure")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior_on_mx_failure: Option<String>,
    #[serde(rename = "EmailIdentity")]
    #[serde(default)]
    pub email_identity: String,
    #[serde(rename = "MailFromDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_from_domain: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutEmailIdentityMailFromAttributesResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutSuppressedDestinationRequest {
    #[serde(rename = "EmailAddress")]
    #[serde(default)]
    pub email_address: String,
    #[serde(rename = "Reason")]
    #[serde(default)]
    pub reason: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutSuppressedDestinationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendBulkEmailRequest {
    #[serde(rename = "BulkEmailEntries")]
    #[serde(default)]
    pub bulk_email_entries: Vec<BulkEmailEntry>,
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_set_name: Option<String>,
    #[serde(rename = "DefaultContent")]
    #[serde(default)]
    pub default_content: BulkEmailContent,
    #[serde(rename = "DefaultEmailTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_email_tags: Option<Vec<MessageTag>>,
    #[serde(rename = "EndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    #[serde(rename = "FeedbackForwardingEmailAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_forwarding_email_address: Option<String>,
    #[serde(rename = "FeedbackForwardingEmailAddressIdentityArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_forwarding_email_address_identity_arn: Option<String>,
    #[serde(rename = "FromEmailAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_email_address: Option<String>,
    #[serde(rename = "FromEmailAddressIdentityArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_email_address_identity_arn: Option<String>,
    #[serde(rename = "ReplyToAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_addresses: Option<Vec<String>>,
    #[serde(rename = "TenantName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BulkEmailEntry {
    #[serde(rename = "Destination")]
    #[serde(default)]
    pub destination: Destination,
    #[serde(rename = "ReplacementEmailContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement_email_content: Option<ReplacementEmailContent>,
    #[serde(rename = "ReplacementHeaders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement_headers: Option<Vec<MessageHeader>>,
    #[serde(rename = "ReplacementTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement_tags: Option<Vec<MessageTag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Destination {
    #[serde(rename = "BccAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc_addresses: Option<Vec<String>>,
    #[serde(rename = "CcAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_addresses: Option<Vec<String>>,
    #[serde(rename = "ToAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_addresses: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplacementEmailContent {
    #[serde(rename = "ReplacementTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement_template: Option<ReplacementTemplate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplacementTemplate {
    #[serde(rename = "ReplacementTemplateData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement_template_data: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BulkEmailContent {
    #[serde(rename = "Template")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<Template>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendBulkEmailResponse {
    #[serde(rename = "BulkEmailEntryResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_email_entry_results: Option<Vec<BulkEmailEntryResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BulkEmailEntryResult {
    #[serde(rename = "Error")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "MessageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendCustomVerificationEmailRequest {
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_set_name: Option<String>,
    #[serde(rename = "EmailAddress")]
    #[serde(default)]
    pub email_address: String,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    pub template_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendCustomVerificationEmailResponse {
    #[serde(rename = "MessageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendEmailRequest {
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_set_name: Option<String>,
    #[serde(rename = "Content")]
    #[serde(default)]
    pub content: EmailContent,
    #[serde(rename = "Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<Destination>,
    #[serde(rename = "EmailTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_tags: Option<Vec<MessageTag>>,
    #[serde(rename = "EndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    #[serde(rename = "FeedbackForwardingEmailAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_forwarding_email_address: Option<String>,
    #[serde(rename = "FeedbackForwardingEmailAddressIdentityArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_forwarding_email_address_identity_arn: Option<String>,
    #[serde(rename = "FromEmailAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_email_address: Option<String>,
    #[serde(rename = "FromEmailAddressIdentityArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_email_address_identity_arn: Option<String>,
    #[serde(rename = "ListManagementOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_management_options: Option<ListManagementOptions>,
    #[serde(rename = "ReplyToAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_addresses: Option<Vec<String>>,
    #[serde(rename = "TenantName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListManagementOptions {
    #[serde(rename = "ContactListName")]
    #[serde(default)]
    pub contact_list_name: String,
    #[serde(rename = "TopicName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendEmailResponse {
    #[serde(rename = "MessageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
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
pub struct TestRenderEmailTemplateRequest {
    #[serde(rename = "TemplateData")]
    #[serde(default)]
    pub template_data: String,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    pub template_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestRenderEmailTemplateResponse {
    #[serde(rename = "RenderedTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendered_template: Option<String>,
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
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConfigurationSetEventDestinationRequest {
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    pub configuration_set_name: String,
    #[serde(rename = "EventDestination")]
    #[serde(default)]
    pub event_destination: EventDestinationDefinition,
    #[serde(rename = "EventDestinationName")]
    #[serde(default)]
    pub event_destination_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConfigurationSetEventDestinationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateContactListRequest {
    #[serde(rename = "ContactListName")]
    #[serde(default)]
    pub contact_list_name: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Topics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<Topic>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateContactListResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateContactRequest {
    #[serde(rename = "AttributesData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_data: Option<String>,
    #[serde(rename = "ContactListName")]
    #[serde(default)]
    pub contact_list_name: String,
    #[serde(rename = "EmailAddress")]
    #[serde(default)]
    pub email_address: String,
    #[serde(rename = "TopicPreferences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_preferences: Option<Vec<TopicPreference>>,
    #[serde(rename = "UnsubscribeAll")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscribe_all: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateContactResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCustomVerificationEmailTemplateRequest {
    #[serde(rename = "FailureRedirectionURL")]
    #[serde(default)]
    pub failure_redirection_u_r_l: String,
    #[serde(rename = "FromEmailAddress")]
    #[serde(default)]
    pub from_email_address: String,
    #[serde(rename = "SuccessRedirectionURL")]
    #[serde(default)]
    pub success_redirection_u_r_l: String,
    #[serde(rename = "TemplateContent")]
    #[serde(default)]
    pub template_content: String,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    pub template_name: String,
    #[serde(rename = "TemplateSubject")]
    #[serde(default)]
    pub template_subject: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCustomVerificationEmailTemplateResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEmailIdentityPolicyRequest {
    #[serde(rename = "EmailIdentity")]
    #[serde(default)]
    pub email_identity: String,
    #[serde(rename = "Policy")]
    #[serde(default)]
    pub policy: String,
    #[serde(rename = "PolicyName")]
    #[serde(default)]
    pub policy_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEmailIdentityPolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEmailTemplateRequest {
    #[serde(rename = "TemplateContent")]
    #[serde(default)]
    pub template_content: EmailTemplateContent,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    pub template_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEmailTemplateResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateReputationEntityCustomerManagedStatusRequest {
    #[serde(rename = "ReputationEntityReference")]
    #[serde(default)]
    pub reputation_entity_reference: String,
    #[serde(rename = "ReputationEntityType")]
    #[serde(default)]
    pub reputation_entity_type: String,
    #[serde(rename = "SendingStatus")]
    #[serde(default)]
    pub sending_status: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateReputationEntityCustomerManagedStatusResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateReputationEntityPolicyRequest {
    #[serde(rename = "ReputationEntityPolicy")]
    #[serde(default)]
    pub reputation_entity_policy: String,
    #[serde(rename = "ReputationEntityReference")]
    #[serde(default)]
    pub reputation_entity_reference: String,
    #[serde(rename = "ReputationEntityType")]
    #[serde(default)]
    pub reputation_entity_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateReputationEntityPolicyResponse {}
