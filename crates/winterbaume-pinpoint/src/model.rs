//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-pinpoint

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAppRequest {
    #[serde(rename = "CreateApplicationRequest")]
    #[serde(default)]
    pub create_application_request: CreateApplicationRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApplicationRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAppResponse {
    #[serde(rename = "ApplicationResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_response: Option<ApplicationResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCampaignRequest {
    #[serde(rename = "WriteCampaignRequest")]
    #[serde(default)]
    pub write_campaign_request: WriteCampaignRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WriteCampaignRequest {
    #[serde(rename = "AdditionalTreatments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_treatments: Option<Vec<WriteTreatmentResource>>,
    #[serde(rename = "CustomDeliveryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_delivery_configuration: Option<CustomDeliveryConfiguration>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "HoldoutPercent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holdout_percent: Option<i32>,
    #[serde(rename = "Hook")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook: Option<CampaignHook>,
    #[serde(rename = "IsPaused")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_paused: Option<bool>,
    #[serde(rename = "Limits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<CampaignLimits>,
    #[serde(rename = "MessageConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_configuration: Option<MessageConfiguration>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "Schedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
    #[serde(rename = "SegmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
    #[serde(rename = "SegmentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_version: Option<i32>,
    #[serde(rename = "TemplateConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_configuration: Option<TemplateConfiguration>,
    #[serde(rename = "TreatmentDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_description: Option<String>,
    #[serde(rename = "TreatmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WriteTreatmentResource {
    #[serde(rename = "CustomDeliveryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_delivery_configuration: Option<CustomDeliveryConfiguration>,
    #[serde(rename = "MessageConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_configuration: Option<MessageConfiguration>,
    #[serde(rename = "Schedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
    #[serde(rename = "SizePercent")]
    #[serde(default)]
    pub size_percent: i32,
    #[serde(rename = "TemplateConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_configuration: Option<TemplateConfiguration>,
    #[serde(rename = "TreatmentDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_description: Option<String>,
    #[serde(rename = "TreatmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomDeliveryConfiguration {
    #[serde(rename = "DeliveryUri")]
    #[serde(default)]
    pub delivery_uri: String,
    #[serde(rename = "EndpointTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MessageConfiguration {
    #[serde(rename = "ADMMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_d_m_message: Option<Message>,
    #[serde(rename = "APNSMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_p_n_s_message: Option<Message>,
    #[serde(rename = "BaiduMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baidu_message: Option<Message>,
    #[serde(rename = "CustomMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_message: Option<CampaignCustomMessage>,
    #[serde(rename = "DefaultMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_message: Option<Message>,
    #[serde(rename = "EmailMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_message: Option<CampaignEmailMessage>,
    #[serde(rename = "GCMMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g_c_m_message: Option<Message>,
    #[serde(rename = "InAppMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_app_message: Option<CampaignInAppMessage>,
    #[serde(rename = "SMSMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_m_s_message: Option<CampaignSmsMessage>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Message {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "Body")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "ImageIconUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_icon_url: Option<String>,
    #[serde(rename = "ImageSmallIconUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_small_icon_url: Option<String>,
    #[serde(rename = "ImageUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(rename = "JsonBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_body: Option<String>,
    #[serde(rename = "MediaUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_url: Option<String>,
    #[serde(rename = "RawContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_content: Option<String>,
    #[serde(rename = "SilentPush")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent_push: Option<bool>,
    #[serde(rename = "TimeToLive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_live: Option<i32>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CampaignCustomMessage {
    #[serde(rename = "Data")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CampaignEmailMessage {
    #[serde(rename = "Body")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "FromAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_address: Option<String>,
    #[serde(rename = "Headers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<MessageHeader>>,
    #[serde(rename = "HtmlBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_body: Option<String>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MessageHeader {
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
pub struct CampaignInAppMessage {
    #[serde(rename = "Body")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<InAppMessageContent>>,
    #[serde(rename = "CustomConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_config: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Layout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InAppMessageContent {
    #[serde(rename = "BackgroundColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(rename = "BodyConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_config: Option<InAppMessageBodyConfig>,
    #[serde(rename = "HeaderConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_config: Option<InAppMessageHeaderConfig>,
    #[serde(rename = "ImageUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(rename = "PrimaryBtn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_btn: Option<InAppMessageButton>,
    #[serde(rename = "SecondaryBtn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_btn: Option<InAppMessageButton>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InAppMessageBodyConfig {
    #[serde(rename = "Alignment")]
    #[serde(default)]
    pub alignment: String,
    #[serde(rename = "Body")]
    #[serde(default)]
    pub body: String,
    #[serde(rename = "TextColor")]
    #[serde(default)]
    pub text_color: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InAppMessageHeaderConfig {
    #[serde(rename = "Alignment")]
    #[serde(default)]
    pub alignment: String,
    #[serde(rename = "Header")]
    #[serde(default)]
    pub header: String,
    #[serde(rename = "TextColor")]
    #[serde(default)]
    pub text_color: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InAppMessageButton {
    #[serde(rename = "Android")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android: Option<OverrideButtonConfiguration>,
    #[serde(rename = "DefaultConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_config: Option<DefaultButtonConfiguration>,
    #[serde(rename = "IOS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_o_s: Option<OverrideButtonConfiguration>,
    #[serde(rename = "Web")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<OverrideButtonConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OverrideButtonConfiguration {
    #[serde(rename = "ButtonAction")]
    #[serde(default)]
    pub button_action: String,
    #[serde(rename = "Link")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DefaultButtonConfiguration {
    #[serde(rename = "BackgroundColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    #[serde(rename = "BorderRadius")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_radius: Option<i32>,
    #[serde(rename = "ButtonAction")]
    #[serde(default)]
    pub button_action: String,
    #[serde(rename = "Link")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(rename = "Text")]
    #[serde(default)]
    pub text: String,
    #[serde(rename = "TextColor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CampaignSmsMessage {
    #[serde(rename = "Body")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "EntityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    #[serde(rename = "MessageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    #[serde(rename = "OriginationNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origination_number: Option<String>,
    #[serde(rename = "SenderId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<String>,
    #[serde(rename = "TemplateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Schedule {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "EventFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_filter: Option<CampaignEventFilter>,
    #[serde(rename = "Frequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    #[serde(rename = "IsLocalTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_local_time: Option<bool>,
    #[serde(rename = "QuietTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quiet_time: Option<QuietTime>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    pub start_time: String,
    #[serde(rename = "Timezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CampaignEventFilter {
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    pub dimensions: EventDimensions,
    #[serde(rename = "FilterType")]
    #[serde(default)]
    pub filter_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventDimensions {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, AttributeDimension>>,
    #[serde(rename = "EventType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<SetDimension>,
    #[serde(rename = "Metrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<std::collections::HashMap<String, MetricDimension>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttributeDimension {
    #[serde(rename = "AttributeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<String>,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetDimension {
    #[serde(rename = "DimensionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension_type: Option<String>,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetricDimension {
    #[serde(rename = "ComparisonOperator")]
    #[serde(default)]
    pub comparison_operator: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QuietTime {
    #[serde(rename = "End")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[serde(rename = "Start")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TemplateConfiguration {
    #[serde(rename = "EmailTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_template: Option<Template>,
    #[serde(rename = "InAppTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_app_template: Option<Template>,
    #[serde(rename = "PushTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_template: Option<Template>,
    #[serde(rename = "SMSTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_m_s_template: Option<Template>,
    #[serde(rename = "VoiceTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_template: Option<Template>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Template {
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
pub struct CampaignHook {
    #[serde(rename = "LambdaFunctionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function_name: Option<String>,
    #[serde(rename = "Mode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "WebUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CampaignLimits {
    #[serde(rename = "Daily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily: Option<i32>,
    #[serde(rename = "MaximumDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_duration: Option<i32>,
    #[serde(rename = "MessagesPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages_per_second: Option<i32>,
    #[serde(rename = "Session")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<i32>,
    #[serde(rename = "Total")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCampaignResponse {
    #[serde(rename = "CampaignResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_response: Option<CampaignResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CampaignResponse {
    #[serde(rename = "AdditionalTreatments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_treatments: Option<Vec<TreatmentResource>>,
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "CustomDeliveryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_delivery_configuration: Option<CustomDeliveryConfiguration>,
    #[serde(rename = "DefaultState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_state: Option<CampaignState>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "HoldoutPercent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holdout_percent: Option<i32>,
    #[serde(rename = "Hook")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook: Option<CampaignHook>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IsPaused")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_paused: Option<bool>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    #[serde(rename = "Limits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<CampaignLimits>,
    #[serde(rename = "MessageConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_configuration: Option<MessageConfiguration>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "Schedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
    #[serde(rename = "SegmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
    #[serde(rename = "SegmentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_version: Option<i32>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<CampaignState>,
    #[serde(rename = "TemplateConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_configuration: Option<TemplateConfiguration>,
    #[serde(rename = "TreatmentDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_description: Option<String>,
    #[serde(rename = "TreatmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_name: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TreatmentResource {
    #[serde(rename = "CustomDeliveryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_delivery_configuration: Option<CustomDeliveryConfiguration>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "MessageConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_configuration: Option<MessageConfiguration>,
    #[serde(rename = "Schedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
    #[serde(rename = "SizePercent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_percent: Option<i32>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<CampaignState>,
    #[serde(rename = "TemplateConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_configuration: Option<TemplateConfiguration>,
    #[serde(rename = "TreatmentDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_description: Option<String>,
    #[serde(rename = "TreatmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CampaignState {
    #[serde(rename = "CampaignStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEmailTemplateRequest {
    #[serde(rename = "EmailTemplateRequest")]
    #[serde(default)]
    pub email_template_request: EmailTemplateRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EmailTemplateRequest {
    #[serde(rename = "DefaultSubstitutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_substitutions: Option<String>,
    #[serde(rename = "Headers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<MessageHeader>>,
    #[serde(rename = "HtmlPart")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_part: Option<String>,
    #[serde(rename = "RecommenderId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommender_id: Option<String>,
    #[serde(rename = "Subject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(rename = "TemplateDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_description: Option<String>,
    #[serde(rename = "TextPart")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_part: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEmailTemplateResponse {
    #[serde(rename = "CreateTemplateMessageBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_template_message_body: Option<CreateTemplateMessageBody>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTemplateMessageBody {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "RequestID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_i_d: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateExportJobRequest {
    #[serde(rename = "ExportJobRequest")]
    #[serde(default)]
    pub export_job_request: ExportJobRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportJobRequest {
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "S3UrlPrefix")]
    #[serde(default)]
    pub s3_url_prefix: String,
    #[serde(rename = "SegmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
    #[serde(rename = "SegmentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_version: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateExportJobResponse {
    #[serde(rename = "ExportJobResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_job_response: Option<ExportJobResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportJobResponse {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "CompletedPieces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_pieces: Option<i32>,
    #[serde(rename = "CompletionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "Definition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<ExportJobResource>,
    #[serde(rename = "FailedPieces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_pieces: Option<i32>,
    #[serde(rename = "Failures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<String>>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "TotalFailures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_failures: Option<i32>,
    #[serde(rename = "TotalPieces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pieces: Option<i32>,
    #[serde(rename = "TotalProcessed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_processed: Option<i32>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportJobResource {
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "S3UrlPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_url_prefix: Option<String>,
    #[serde(rename = "SegmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
    #[serde(rename = "SegmentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_version: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateImportJobRequest {
    #[serde(rename = "ImportJobRequest")]
    #[serde(default)]
    pub import_job_request: ImportJobRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportJobRequest {
    #[serde(rename = "DefineSegment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub define_segment: Option<bool>,
    #[serde(rename = "ExternalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "Format")]
    #[serde(default)]
    pub format: String,
    #[serde(rename = "RegisterEndpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register_endpoints: Option<bool>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "S3Url")]
    #[serde(default)]
    pub s3_url: String,
    #[serde(rename = "SegmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
    #[serde(rename = "SegmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateImportJobResponse {
    #[serde(rename = "ImportJobResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_job_response: Option<ImportJobResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportJobResponse {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "CompletedPieces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_pieces: Option<i32>,
    #[serde(rename = "CompletionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "Definition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<ImportJobResource>,
    #[serde(rename = "FailedPieces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_pieces: Option<i32>,
    #[serde(rename = "Failures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<String>>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "JobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "TotalFailures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_failures: Option<i32>,
    #[serde(rename = "TotalPieces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pieces: Option<i32>,
    #[serde(rename = "TotalProcessed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_processed: Option<i32>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportJobResource {
    #[serde(rename = "DefineSegment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub define_segment: Option<bool>,
    #[serde(rename = "ExternalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "Format")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "RegisterEndpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register_endpoints: Option<bool>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "S3Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_url: Option<String>,
    #[serde(rename = "SegmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
    #[serde(rename = "SegmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateInAppTemplateRequest {
    #[serde(rename = "InAppTemplateRequest")]
    #[serde(default)]
    pub in_app_template_request: InAppTemplateRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InAppTemplateRequest {
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<InAppMessageContent>>,
    #[serde(rename = "CustomConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_config: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Layout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<String>,
    #[serde(rename = "TemplateDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateInAppTemplateResponse {
    #[serde(rename = "TemplateCreateMessageBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_create_message_body: Option<TemplateCreateMessageBody>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TemplateCreateMessageBody {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "RequestID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_i_d: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateJourneyRequest {
    #[serde(rename = "WriteJourneyRequest")]
    #[serde(default)]
    pub write_journey_request: WriteJourneyRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WriteJourneyRequest {
    #[serde(rename = "Activities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activities: Option<std::collections::HashMap<String, Activity>>,
    #[serde(rename = "ClosedDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_days: Option<ClosedDays>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "JourneyChannelSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journey_channel_settings: Option<JourneyChannelSettings>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    #[serde(rename = "Limits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<JourneyLimits>,
    #[serde(rename = "LocalTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_time: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OpenHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_hours: Option<OpenHours>,
    #[serde(rename = "QuietTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quiet_time: Option<QuietTime>,
    #[serde(rename = "RefreshFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_frequency: Option<String>,
    #[serde(rename = "RefreshOnSegmentUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_on_segment_update: Option<bool>,
    #[serde(rename = "Schedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<JourneySchedule>,
    #[serde(rename = "SendingSchedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_schedule: Option<bool>,
    #[serde(rename = "StartActivity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_activity: Option<String>,
    #[serde(rename = "StartCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_condition: Option<StartCondition>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "TimezoneEstimationMethods")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone_estimation_methods: Option<Vec<String>>,
    #[serde(rename = "WaitForQuietTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for_quiet_time: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Activity {
    #[serde(rename = "CUSTOM")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_u_s_t_o_m: Option<CustomMessageActivity>,
    #[serde(rename = "ConditionalSplit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_split: Option<ConditionalSplitActivity>,
    #[serde(rename = "ContactCenter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_center: Option<ContactCenterActivity>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EMAIL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_m_a_i_l: Option<EmailMessageActivity>,
    #[serde(rename = "Holdout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holdout: Option<HoldoutActivity>,
    #[serde(rename = "MultiCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_condition: Option<MultiConditionalSplitActivity>,
    #[serde(rename = "PUSH")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p_u_s_h: Option<PushMessageActivity>,
    #[serde(rename = "RandomSplit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_split: Option<RandomSplitActivity>,
    #[serde(rename = "SMS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_m_s: Option<SMSMessageActivity>,
    #[serde(rename = "Wait")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait: Option<WaitActivity>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomMessageActivity {
    #[serde(rename = "DeliveryUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_uri: Option<String>,
    #[serde(rename = "EndpointTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_types: Option<Vec<String>>,
    #[serde(rename = "MessageConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_config: Option<JourneyCustomMessage>,
    #[serde(rename = "NextActivity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_activity: Option<String>,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    #[serde(rename = "TemplateVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JourneyCustomMessage {
    #[serde(rename = "Data")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConditionalSplitActivity {
    #[serde(rename = "Condition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<Condition>,
    #[serde(rename = "EvaluationWaitTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_wait_time: Option<WaitTime>,
    #[serde(rename = "FalseActivity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub false_activity: Option<String>,
    #[serde(rename = "TrueActivity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub true_activity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Condition {
    #[serde(rename = "Conditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<SimpleCondition>>,
    #[serde(rename = "Operator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SimpleCondition {
    #[serde(rename = "EventCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_condition: Option<EventCondition>,
    #[serde(rename = "SegmentCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_condition: Option<SegmentCondition>,
    #[serde(rename = "segmentDimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_dimensions: Option<SegmentDimensions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventCondition {
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<EventDimensions>,
    #[serde(rename = "MessageActivity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_activity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SegmentCondition {
    #[serde(rename = "SegmentId")]
    #[serde(default)]
    pub segment_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SegmentDimensions {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, AttributeDimension>>,
    #[serde(rename = "Behavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior: Option<SegmentBehaviors>,
    #[serde(rename = "Demographic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demographic: Option<SegmentDemographics>,
    #[serde(rename = "Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<SegmentLocation>,
    #[serde(rename = "Metrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<std::collections::HashMap<String, MetricDimension>>,
    #[serde(rename = "UserAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_attributes: Option<std::collections::HashMap<String, AttributeDimension>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SegmentBehaviors {
    #[serde(rename = "Recency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recency: Option<RecencyDimension>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecencyDimension {
    #[serde(rename = "Duration")]
    #[serde(default)]
    pub duration: String,
    #[serde(rename = "RecencyType")]
    #[serde(default)]
    pub recency_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SegmentDemographics {
    #[serde(rename = "AppVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<SetDimension>,
    #[serde(rename = "Channel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<SetDimension>,
    #[serde(rename = "DeviceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<SetDimension>,
    #[serde(rename = "Make")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub make: Option<SetDimension>,
    #[serde(rename = "Model")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<SetDimension>,
    #[serde(rename = "Platform")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<SetDimension>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SegmentLocation {
    #[serde(rename = "Country")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<SetDimension>,
    #[serde(rename = "GPSPoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g_p_s_point: Option<GPSPointDimension>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GPSPointDimension {
    #[serde(rename = "Coordinates")]
    #[serde(default)]
    pub coordinates: GPSCoordinates,
    #[serde(rename = "RangeInKilometers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_in_kilometers: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GPSCoordinates {
    #[serde(rename = "Latitude")]
    #[serde(default)]
    pub latitude: f64,
    #[serde(rename = "Longitude")]
    #[serde(default)]
    pub longitude: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WaitTime {
    #[serde(rename = "WaitFor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for: Option<String>,
    #[serde(rename = "WaitUntil")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_until: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContactCenterActivity {
    #[serde(rename = "NextActivity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_activity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EmailMessageActivity {
    #[serde(rename = "MessageConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_config: Option<JourneyEmailMessage>,
    #[serde(rename = "NextActivity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_activity: Option<String>,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    #[serde(rename = "TemplateVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JourneyEmailMessage {
    #[serde(rename = "FromAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_address: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HoldoutActivity {
    #[serde(rename = "NextActivity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_activity: Option<String>,
    #[serde(rename = "Percentage")]
    #[serde(default)]
    pub percentage: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiConditionalSplitActivity {
    #[serde(rename = "Branches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches: Option<Vec<MultiConditionalBranch>>,
    #[serde(rename = "DefaultActivity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_activity: Option<String>,
    #[serde(rename = "EvaluationWaitTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_wait_time: Option<WaitTime>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultiConditionalBranch {
    #[serde(rename = "Condition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<SimpleCondition>,
    #[serde(rename = "NextActivity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_activity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PushMessageActivity {
    #[serde(rename = "MessageConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_config: Option<JourneyPushMessage>,
    #[serde(rename = "NextActivity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_activity: Option<String>,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    #[serde(rename = "TemplateVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JourneyPushMessage {
    #[serde(rename = "TimeToLive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_live: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RandomSplitActivity {
    #[serde(rename = "Branches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches: Option<Vec<RandomSplitEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RandomSplitEntry {
    #[serde(rename = "NextActivity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_activity: Option<String>,
    #[serde(rename = "Percentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SMSMessageActivity {
    #[serde(rename = "MessageConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_config: Option<JourneySMSMessage>,
    #[serde(rename = "NextActivity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_activity: Option<String>,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    #[serde(rename = "TemplateVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JourneySMSMessage {
    #[serde(rename = "EntityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    #[serde(rename = "MessageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    #[serde(rename = "OriginationNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origination_number: Option<String>,
    #[serde(rename = "SenderId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<String>,
    #[serde(rename = "TemplateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WaitActivity {
    #[serde(rename = "NextActivity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_activity: Option<String>,
    #[serde(rename = "WaitTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_time: Option<WaitTime>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClosedDays {
    #[serde(rename = "CUSTOM")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_u_s_t_o_m: Option<Vec<ClosedDaysRule>>,
    #[serde(rename = "EMAIL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_m_a_i_l: Option<Vec<ClosedDaysRule>>,
    #[serde(rename = "PUSH")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p_u_s_h: Option<Vec<ClosedDaysRule>>,
    #[serde(rename = "SMS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_m_s: Option<Vec<ClosedDaysRule>>,
    #[serde(rename = "VOICE")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_o_i_c_e: Option<Vec<ClosedDaysRule>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClosedDaysRule {
    #[serde(rename = "EndDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_time: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "StartDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JourneyChannelSettings {
    #[serde(rename = "ConnectCampaignArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_campaign_arn: Option<String>,
    #[serde(rename = "ConnectCampaignExecutionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_campaign_execution_role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JourneyLimits {
    #[serde(rename = "DailyCap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_cap: Option<i32>,
    #[serde(rename = "EndpointReentryCap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_reentry_cap: Option<i32>,
    #[serde(rename = "EndpointReentryInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_reentry_interval: Option<String>,
    #[serde(rename = "MessagesPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages_per_second: Option<i32>,
    #[serde(rename = "TimeframeCap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeframe_cap: Option<JourneyTimeframeCap>,
    #[serde(rename = "TotalCap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_cap: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JourneyTimeframeCap {
    #[serde(rename = "Cap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap: Option<i32>,
    #[serde(rename = "Days")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpenHours {
    #[serde(rename = "CUSTOM")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_u_s_t_o_m: Option<std::collections::HashMap<String, Vec<OpenHoursRule>>>,
    #[serde(rename = "EMAIL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_m_a_i_l: Option<std::collections::HashMap<String, Vec<OpenHoursRule>>>,
    #[serde(rename = "PUSH")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p_u_s_h: Option<std::collections::HashMap<String, Vec<OpenHoursRule>>>,
    #[serde(rename = "SMS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_m_s: Option<std::collections::HashMap<String, Vec<OpenHoursRule>>>,
    #[serde(rename = "VOICE")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_o_i_c_e: Option<std::collections::HashMap<String, Vec<OpenHoursRule>>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpenHoursRule {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JourneySchedule {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "Timezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartCondition {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EventStartCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_start_condition: Option<EventStartCondition>,
    #[serde(rename = "SegmentStartCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_start_condition: Option<SegmentCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventStartCondition {
    #[serde(rename = "EventFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_filter: Option<EventFilter>,
    #[serde(rename = "SegmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventFilter {
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    pub dimensions: EventDimensions,
    #[serde(rename = "FilterType")]
    #[serde(default)]
    pub filter_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateJourneyResponse {
    #[serde(rename = "JourneyResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journey_response: Option<JourneyResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JourneyResponse {
    #[serde(rename = "Activities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activities: Option<std::collections::HashMap<String, Activity>>,
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "ClosedDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_days: Option<ClosedDays>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "JourneyChannelSettings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journey_channel_settings: Option<JourneyChannelSettings>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    #[serde(rename = "Limits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<JourneyLimits>,
    #[serde(rename = "LocalTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_time: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OpenHours")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_hours: Option<OpenHours>,
    #[serde(rename = "QuietTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quiet_time: Option<QuietTime>,
    #[serde(rename = "RefreshFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_frequency: Option<String>,
    #[serde(rename = "RefreshOnSegmentUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_on_segment_update: Option<bool>,
    #[serde(rename = "Schedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<JourneySchedule>,
    #[serde(rename = "SendingSchedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sending_schedule: Option<bool>,
    #[serde(rename = "StartActivity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_activity: Option<String>,
    #[serde(rename = "StartCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_condition: Option<StartCondition>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "TimezoneEstimationMethods")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone_estimation_methods: Option<Vec<String>>,
    #[serde(rename = "WaitForQuietTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for_quiet_time: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePushTemplateRequest {
    #[serde(rename = "PushNotificationTemplateRequest")]
    #[serde(default)]
    pub push_notification_template_request: PushNotificationTemplateRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PushNotificationTemplateRequest {
    #[serde(rename = "ADM")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_d_m: Option<AndroidPushNotificationTemplate>,
    #[serde(rename = "APNS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_p_n_s: Option<APNSPushNotificationTemplate>,
    #[serde(rename = "Baidu")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baidu: Option<AndroidPushNotificationTemplate>,
    #[serde(rename = "Default")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<DefaultPushNotificationTemplate>,
    #[serde(rename = "DefaultSubstitutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_substitutions: Option<String>,
    #[serde(rename = "GCM")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g_c_m: Option<AndroidPushNotificationTemplate>,
    #[serde(rename = "RecommenderId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommender_id: Option<String>,
    #[serde(rename = "TemplateDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AndroidPushNotificationTemplate {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "Body")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "ImageIconUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_icon_url: Option<String>,
    #[serde(rename = "ImageUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(rename = "RawContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_content: Option<String>,
    #[serde(rename = "SmallImageIconUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_image_icon_url: Option<String>,
    #[serde(rename = "Sound")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct APNSPushNotificationTemplate {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "Body")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "MediaUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_url: Option<String>,
    #[serde(rename = "RawContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_content: Option<String>,
    #[serde(rename = "Sound")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DefaultPushNotificationTemplate {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "Body")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "Sound")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePushTemplateResponse {
    #[serde(rename = "CreateTemplateMessageBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_template_message_body: Option<CreateTemplateMessageBody>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRecommenderConfigurationRequest {
    #[serde(rename = "CreateRecommenderConfiguration")]
    #[serde(default)]
    pub create_recommender_configuration: CreateRecommenderConfigurationShape,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRecommenderConfigurationShape {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RecommendationProviderIdType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_provider_id_type: Option<String>,
    #[serde(rename = "RecommendationProviderRoleArn")]
    #[serde(default)]
    pub recommendation_provider_role_arn: String,
    #[serde(rename = "RecommendationProviderUri")]
    #[serde(default)]
    pub recommendation_provider_uri: String,
    #[serde(rename = "RecommendationTransformerUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_transformer_uri: Option<String>,
    #[serde(rename = "RecommendationsDisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendations_display_name: Option<String>,
    #[serde(rename = "RecommendationsPerMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendations_per_message: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRecommenderConfigurationResponse {
    #[serde(rename = "RecommenderConfigurationResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommender_configuration_response: Option<RecommenderConfigurationResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecommenderConfigurationResponse {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RecommendationProviderIdType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_provider_id_type: Option<String>,
    #[serde(rename = "RecommendationProviderRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_provider_role_arn: Option<String>,
    #[serde(rename = "RecommendationProviderUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_provider_uri: Option<String>,
    #[serde(rename = "RecommendationTransformerUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_transformer_uri: Option<String>,
    #[serde(rename = "RecommendationsDisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendations_display_name: Option<String>,
    #[serde(rename = "RecommendationsPerMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendations_per_message: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSegmentRequest {
    #[serde(rename = "WriteSegmentRequest")]
    #[serde(default)]
    pub write_segment_request: WriteSegmentRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WriteSegmentRequest {
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<SegmentDimensions>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SegmentGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_groups: Option<SegmentGroupList>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SegmentGroupList {
    #[serde(rename = "Groups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<SegmentGroup>>,
    #[serde(rename = "Include")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SegmentGroup {
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<SegmentDimensions>>,
    #[serde(rename = "SourceSegments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_segments: Option<Vec<SegmentReference>>,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SegmentReference {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSegmentResponse {
    #[serde(rename = "SegmentResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_response: Option<SegmentResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SegmentResponse {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "Dimensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<SegmentDimensions>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ImportDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_definition: Option<SegmentImportResource>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SegmentGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_groups: Option<SegmentGroupList>,
    #[serde(rename = "SegmentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_type: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SegmentImportResource {
    #[serde(rename = "ChannelCounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_counts: Option<std::collections::HashMap<String, i32>>,
    #[serde(rename = "ExternalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "Format")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "S3Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_url: Option<String>,
    #[serde(rename = "Size")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSmsTemplateRequest {
    #[serde(rename = "SMSTemplateRequest")]
    #[serde(default)]
    pub s_m_s_template_request: SMSTemplateRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SMSTemplateRequest {
    #[serde(rename = "Body")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "DefaultSubstitutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_substitutions: Option<String>,
    #[serde(rename = "RecommenderId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommender_id: Option<String>,
    #[serde(rename = "TemplateDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSmsTemplateResponse {
    #[serde(rename = "CreateTemplateMessageBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_template_message_body: Option<CreateTemplateMessageBody>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVoiceTemplateRequest {
    #[serde(rename = "VoiceTemplateRequest")]
    #[serde(default)]
    pub voice_template_request: VoiceTemplateRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VoiceTemplateRequest {
    #[serde(rename = "Body")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "DefaultSubstitutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_substitutions: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "TemplateDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_description: Option<String>,
    #[serde(rename = "VoiceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVoiceTemplateResponse {
    #[serde(rename = "CreateTemplateMessageBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_template_message_body: Option<CreateTemplateMessageBody>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAdmChannelRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAdmChannelResponse {
    #[serde(rename = "ADMChannelResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_d_m_channel_response: Option<ADMChannelResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ADMChannelResponse {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "HasCredential")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IsArchived")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    #[serde(rename = "LastModifiedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    #[serde(rename = "Platform")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApnsChannelRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApnsChannelResponse {
    #[serde(rename = "APNSChannelResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_p_n_s_channel_response: Option<APNSChannelResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct APNSChannelResponse {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "DefaultAuthenticationMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_authentication_method: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "HasCredential")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    #[serde(rename = "HasTokenKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_token_key: Option<bool>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IsArchived")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    #[serde(rename = "LastModifiedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    #[serde(rename = "Platform")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApnsSandboxChannelRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApnsSandboxChannelResponse {
    #[serde(rename = "APNSSandboxChannelResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_p_n_s_sandbox_channel_response: Option<APNSSandboxChannelResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct APNSSandboxChannelResponse {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "DefaultAuthenticationMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_authentication_method: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "HasCredential")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    #[serde(rename = "HasTokenKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_token_key: Option<bool>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IsArchived")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    #[serde(rename = "LastModifiedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    #[serde(rename = "Platform")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApnsVoipChannelRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApnsVoipChannelResponse {
    #[serde(rename = "APNSVoipChannelResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_p_n_s_voip_channel_response: Option<APNSVoipChannelResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct APNSVoipChannelResponse {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "DefaultAuthenticationMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_authentication_method: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "HasCredential")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    #[serde(rename = "HasTokenKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_token_key: Option<bool>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IsArchived")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    #[serde(rename = "LastModifiedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    #[serde(rename = "Platform")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApnsVoipSandboxChannelRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApnsVoipSandboxChannelResponse {
    #[serde(rename = "APNSVoipSandboxChannelResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_p_n_s_voip_sandbox_channel_response: Option<APNSVoipSandboxChannelResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct APNSVoipSandboxChannelResponse {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "DefaultAuthenticationMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_authentication_method: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "HasCredential")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    #[serde(rename = "HasTokenKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_token_key: Option<bool>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IsArchived")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    #[serde(rename = "LastModifiedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    #[serde(rename = "Platform")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAppRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAppResponse {
    #[serde(rename = "ApplicationResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_response: Option<ApplicationResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBaiduChannelRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBaiduChannelResponse {
    #[serde(rename = "BaiduChannelResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baidu_channel_response: Option<BaiduChannelResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BaiduChannelResponse {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "Credential")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "HasCredential")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IsArchived")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    #[serde(rename = "LastModifiedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    #[serde(rename = "Platform")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCampaignRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCampaignResponse {
    #[serde(rename = "CampaignResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_response: Option<CampaignResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEmailChannelRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEmailChannelResponse {
    #[serde(rename = "EmailChannelResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_channel_response: Option<EmailChannelResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EmailChannelResponse {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "ConfigurationSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_set: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "FromAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_address: Option<String>,
    #[serde(rename = "HasCredential")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Identity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    #[serde(rename = "IsArchived")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    #[serde(rename = "LastModifiedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    #[serde(rename = "MessagesPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages_per_second: Option<i32>,
    #[serde(rename = "OrchestrationSendingRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orchestration_sending_role_arn: Option<String>,
    #[serde(rename = "Platform")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEmailTemplateRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEmailTemplateResponse {
    #[serde(rename = "MessageBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_body: Option<MessageBody>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MessageBody {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "RequestID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_i_d: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEndpointRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEndpointResponse {
    #[serde(rename = "EndpointResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_response: Option<EndpointResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EndpointResponse {
    #[serde(rename = "Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "ChannelType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_type: Option<String>,
    #[serde(rename = "CohortId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cohort_id: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "Demographic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demographic: Option<EndpointDemographic>,
    #[serde(rename = "EffectiveDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    #[serde(rename = "EndpointStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_status: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<EndpointLocation>,
    #[serde(rename = "Metrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<std::collections::HashMap<String, f64>>,
    #[serde(rename = "OptOut")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_out: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "User")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<EndpointUser>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EndpointDemographic {
    #[serde(rename = "AppVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version: Option<String>,
    #[serde(rename = "Locale")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(rename = "Make")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub make: Option<String>,
    #[serde(rename = "Model")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "ModelVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_version: Option<String>,
    #[serde(rename = "Platform")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "PlatformVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    #[serde(rename = "Timezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EndpointLocation {
    #[serde(rename = "City")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "Country")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "Latitude")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(rename = "Longitude")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    #[serde(rename = "PostalCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EndpointUser {
    #[serde(rename = "UserAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_attributes: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "UserId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEventStreamRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEventStreamResponse {
    #[serde(rename = "EventStream")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_stream: Option<EventStream>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventStream {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "DestinationStreamArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_stream_arn: Option<String>,
    #[serde(rename = "ExternalId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    #[serde(rename = "LastUpdatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_by: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteGcmChannelRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteGcmChannelResponse {
    #[serde(rename = "GCMChannelResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g_c_m_channel_response: Option<GCMChannelResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GCMChannelResponse {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "Credential")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential: Option<String>,
    #[serde(rename = "DefaultAuthenticationMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_authentication_method: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "HasCredential")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    #[serde(rename = "HasFcmServiceCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_fcm_service_credentials: Option<bool>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IsArchived")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    #[serde(rename = "LastModifiedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    #[serde(rename = "Platform")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteInAppTemplateRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteInAppTemplateResponse {
    #[serde(rename = "MessageBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_body: Option<MessageBody>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteJourneyRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteJourneyResponse {
    #[serde(rename = "JourneyResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journey_response: Option<JourneyResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePushTemplateRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePushTemplateResponse {
    #[serde(rename = "MessageBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_body: Option<MessageBody>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRecommenderConfigurationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRecommenderConfigurationResponse {
    #[serde(rename = "RecommenderConfigurationResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommender_configuration_response: Option<RecommenderConfigurationResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSegmentRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSegmentResponse {
    #[serde(rename = "SegmentResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_response: Option<SegmentResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSmsChannelRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSmsChannelResponse {
    #[serde(rename = "SMSChannelResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_m_s_channel_response: Option<SMSChannelResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SMSChannelResponse {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "HasCredential")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IsArchived")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    #[serde(rename = "LastModifiedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    #[serde(rename = "Platform")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "PromotionalMessagesPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotional_messages_per_second: Option<i32>,
    #[serde(rename = "SenderId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<String>,
    #[serde(rename = "ShortCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_code: Option<String>,
    #[serde(rename = "TransactionalMessagesPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactional_messages_per_second: Option<i32>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSmsTemplateRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSmsTemplateResponse {
    #[serde(rename = "MessageBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_body: Option<MessageBody>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUserEndpointsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUserEndpointsResponse {
    #[serde(rename = "EndpointsResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints_response: Option<EndpointsResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EndpointsResponse {
    #[serde(rename = "Item")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Vec<EndpointResponse>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVoiceChannelRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVoiceChannelResponse {
    #[serde(rename = "VoiceChannelResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_channel_response: Option<VoiceChannelResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VoiceChannelResponse {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "HasCredential")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IsArchived")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    #[serde(rename = "LastModifiedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    #[serde(rename = "Platform")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVoiceTemplateRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVoiceTemplateResponse {
    #[serde(rename = "MessageBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_body: Option<MessageBody>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAdmChannelRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAdmChannelResponse {
    #[serde(rename = "ADMChannelResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_d_m_channel_response: Option<ADMChannelResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApnsChannelRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApnsChannelResponse {
    #[serde(rename = "APNSChannelResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_p_n_s_channel_response: Option<APNSChannelResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApnsSandboxChannelRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApnsSandboxChannelResponse {
    #[serde(rename = "APNSSandboxChannelResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_p_n_s_sandbox_channel_response: Option<APNSSandboxChannelResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApnsVoipChannelRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApnsVoipChannelResponse {
    #[serde(rename = "APNSVoipChannelResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_p_n_s_voip_channel_response: Option<APNSVoipChannelResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApnsVoipSandboxChannelRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApnsVoipSandboxChannelResponse {
    #[serde(rename = "APNSVoipSandboxChannelResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_p_n_s_voip_sandbox_channel_response: Option<APNSVoipSandboxChannelResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAppRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAppResponse {
    #[serde(rename = "ApplicationResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_response: Option<ApplicationResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApplicationDateRangeKpiRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApplicationDateRangeKpiResponse {
    #[serde(rename = "ApplicationDateRangeKpiResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_date_range_kpi_response: Option<ApplicationDateRangeKpiResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationDateRangeKpiResponse {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "KpiName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kpi_name: Option<String>,
    #[serde(rename = "KpiResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kpi_result: Option<BaseKpiResult>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BaseKpiResult {
    #[serde(rename = "Rows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<ResultRow>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResultRow {
    #[serde(rename = "GroupedBys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouped_bys: Option<Vec<ResultRowValue>>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<ResultRowValue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResultRowValue {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApplicationSettingsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApplicationSettingsResponse {
    #[serde(rename = "ApplicationSettingsResource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_settings_resource: Option<ApplicationSettingsResource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationSettingsResource {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "CampaignHook")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_hook: Option<CampaignHook>,
    #[serde(rename = "JourneyLimits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journey_limits: Option<ApplicationSettingsJourneyLimits>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    #[serde(rename = "Limits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<CampaignLimits>,
    #[serde(rename = "QuietTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quiet_time: Option<QuietTime>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationSettingsJourneyLimits {
    #[serde(rename = "DailyCap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_cap: Option<i32>,
    #[serde(rename = "TimeframeCap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeframe_cap: Option<JourneyTimeframeCap>,
    #[serde(rename = "TotalCap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_cap: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAppsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAppsResponse {
    #[serde(rename = "ApplicationsResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applications_response: Option<ApplicationsResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationsResponse {
    #[serde(rename = "Item")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Vec<ApplicationResponse>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBaiduChannelRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetBaiduChannelResponse {
    #[serde(rename = "BaiduChannelResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baidu_channel_response: Option<BaiduChannelResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCampaignActivitiesRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCampaignActivitiesResponse {
    #[serde(rename = "ActivitiesResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activities_response: Option<ActivitiesResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActivitiesResponse {
    #[serde(rename = "Item")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Vec<ActivityResponse>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActivityResponse {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "CampaignId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    #[serde(rename = "End")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[serde(rename = "ExecutionMetrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_metrics: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Result")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[serde(rename = "ScheduledStart")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_start: Option<String>,
    #[serde(rename = "Start")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "SuccessfulEndpointCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_endpoint_count: Option<i32>,
    #[serde(rename = "TimezonesCompletedCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezones_completed_count: Option<i32>,
    #[serde(rename = "TimezonesTotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezones_total_count: Option<i32>,
    #[serde(rename = "TotalEndpointCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_endpoint_count: Option<i32>,
    #[serde(rename = "TreatmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCampaignDateRangeKpiRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCampaignDateRangeKpiResponse {
    #[serde(rename = "CampaignDateRangeKpiResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_date_range_kpi_response: Option<CampaignDateRangeKpiResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CampaignDateRangeKpiResponse {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "CampaignId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "KpiName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kpi_name: Option<String>,
    #[serde(rename = "KpiResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kpi_result: Option<BaseKpiResult>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCampaignRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCampaignResponse {
    #[serde(rename = "CampaignResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_response: Option<CampaignResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCampaignVersionRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCampaignVersionResponse {
    #[serde(rename = "CampaignResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_response: Option<CampaignResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCampaignVersionsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCampaignVersionsResponse {
    #[serde(rename = "CampaignsResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaigns_response: Option<CampaignsResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CampaignsResponse {
    #[serde(rename = "Item")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Vec<CampaignResponse>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCampaignsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCampaignsResponse {
    #[serde(rename = "CampaignsResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaigns_response: Option<CampaignsResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetChannelsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetChannelsResponse {
    #[serde(rename = "ChannelsResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels_response: Option<ChannelsResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChannelsResponse {
    #[serde(rename = "Channels")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<std::collections::HashMap<String, ChannelResponse>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChannelResponse {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "HasCredential")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_credential: Option<bool>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IsArchived")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    #[serde(rename = "LastModifiedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEmailChannelRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEmailChannelResponse {
    #[serde(rename = "EmailChannelResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_channel_response: Option<EmailChannelResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEmailTemplateRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEmailTemplateResponse {
    #[serde(rename = "EmailTemplateResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_template_response: Option<EmailTemplateResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EmailTemplateResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "DefaultSubstitutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_substitutions: Option<String>,
    #[serde(rename = "Headers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<MessageHeader>>,
    #[serde(rename = "HtmlPart")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_part: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    #[serde(rename = "RecommenderId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommender_id: Option<String>,
    #[serde(rename = "Subject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(rename = "TemplateDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_description: Option<String>,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    #[serde(rename = "TemplateType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_type: Option<String>,
    #[serde(rename = "TextPart")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_part: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEndpointRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEndpointResponse {
    #[serde(rename = "EndpointResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_response: Option<EndpointResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEventStreamRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEventStreamResponse {
    #[serde(rename = "EventStream")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_stream: Option<EventStream>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetExportJobRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetExportJobResponse {
    #[serde(rename = "ExportJobResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_job_response: Option<ExportJobResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetExportJobsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetExportJobsResponse {
    #[serde(rename = "ExportJobsResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_jobs_response: Option<ExportJobsResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportJobsResponse {
    #[serde(rename = "Item")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Vec<ExportJobResponse>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetGcmChannelRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetGcmChannelResponse {
    #[serde(rename = "GCMChannelResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g_c_m_channel_response: Option<GCMChannelResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetImportJobRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetImportJobResponse {
    #[serde(rename = "ImportJobResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_job_response: Option<ImportJobResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetImportJobsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetImportJobsResponse {
    #[serde(rename = "ImportJobsResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_jobs_response: Option<ImportJobsResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportJobsResponse {
    #[serde(rename = "Item")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Vec<ImportJobResponse>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInAppMessagesRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInAppMessagesResponse {
    #[serde(rename = "InAppMessagesResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_app_messages_response: Option<InAppMessagesResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InAppMessagesResponse {
    #[serde(rename = "InAppMessageCampaigns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_app_message_campaigns: Option<Vec<InAppMessageCampaign>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InAppMessageCampaign {
    #[serde(rename = "CampaignId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    #[serde(rename = "DailyCap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_cap: Option<i32>,
    #[serde(rename = "InAppMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_app_message: Option<InAppMessage>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "Schedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<InAppCampaignSchedule>,
    #[serde(rename = "SessionCap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_cap: Option<i32>,
    #[serde(rename = "TotalCap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_cap: Option<i32>,
    #[serde(rename = "TreatmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatment_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InAppMessage {
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<InAppMessageContent>>,
    #[serde(rename = "CustomConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_config: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Layout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InAppCampaignSchedule {
    #[serde(rename = "EndDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "EventFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_filter: Option<CampaignEventFilter>,
    #[serde(rename = "QuietTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quiet_time: Option<QuietTime>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInAppTemplateRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInAppTemplateResponse {
    #[serde(rename = "InAppTemplateResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_app_template_response: Option<InAppTemplateResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InAppTemplateResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<InAppMessageContent>>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "CustomConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_config: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    #[serde(rename = "Layout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<String>,
    #[serde(rename = "TemplateDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_description: Option<String>,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    #[serde(rename = "TemplateType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_type: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetJourneyDateRangeKpiRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetJourneyDateRangeKpiResponse {
    #[serde(rename = "JourneyDateRangeKpiResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journey_date_range_kpi_response: Option<JourneyDateRangeKpiResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JourneyDateRangeKpiResponse {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "JourneyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journey_id: Option<String>,
    #[serde(rename = "KpiName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kpi_name: Option<String>,
    #[serde(rename = "KpiResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kpi_result: Option<BaseKpiResult>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetJourneyExecutionActivityMetricsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetJourneyExecutionActivityMetricsResponse {
    #[serde(rename = "JourneyExecutionActivityMetricsResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journey_execution_activity_metrics_response:
        Option<JourneyExecutionActivityMetricsResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JourneyExecutionActivityMetricsResponse {
    #[serde(rename = "ActivityType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<String>,
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "JourneyActivityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journey_activity_id: Option<String>,
    #[serde(rename = "JourneyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journey_id: Option<String>,
    #[serde(rename = "LastEvaluatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_evaluated_time: Option<String>,
    #[serde(rename = "Metrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetJourneyExecutionMetricsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetJourneyExecutionMetricsResponse {
    #[serde(rename = "JourneyExecutionMetricsResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journey_execution_metrics_response: Option<JourneyExecutionMetricsResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JourneyExecutionMetricsResponse {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "JourneyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journey_id: Option<String>,
    #[serde(rename = "LastEvaluatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_evaluated_time: Option<String>,
    #[serde(rename = "Metrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetJourneyRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetJourneyResponse {
    #[serde(rename = "JourneyResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journey_response: Option<JourneyResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetJourneyRunExecutionActivityMetricsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetJourneyRunExecutionActivityMetricsResponse {
    #[serde(rename = "JourneyRunExecutionActivityMetricsResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journey_run_execution_activity_metrics_response:
        Option<JourneyRunExecutionActivityMetricsResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JourneyRunExecutionActivityMetricsResponse {
    #[serde(rename = "ActivityType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<String>,
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "JourneyActivityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journey_activity_id: Option<String>,
    #[serde(rename = "JourneyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journey_id: Option<String>,
    #[serde(rename = "LastEvaluatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_evaluated_time: Option<String>,
    #[serde(rename = "Metrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "RunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetJourneyRunExecutionMetricsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetJourneyRunExecutionMetricsResponse {
    #[serde(rename = "JourneyRunExecutionMetricsResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journey_run_execution_metrics_response: Option<JourneyRunExecutionMetricsResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JourneyRunExecutionMetricsResponse {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "JourneyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journey_id: Option<String>,
    #[serde(rename = "LastEvaluatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_evaluated_time: Option<String>,
    #[serde(rename = "Metrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "RunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetJourneyRunsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetJourneyRunsResponse {
    #[serde(rename = "JourneyRunsResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journey_runs_response: Option<JourneyRunsResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JourneyRunsResponse {
    #[serde(rename = "Item")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Vec<JourneyRunResponse>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JourneyRunResponse {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<String>,
    #[serde(rename = "LastUpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<String>,
    #[serde(rename = "RunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPushTemplateRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPushTemplateResponse {
    #[serde(rename = "PushNotificationTemplateResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_notification_template_response: Option<PushNotificationTemplateResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PushNotificationTemplateResponse {
    #[serde(rename = "ADM")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_d_m: Option<AndroidPushNotificationTemplate>,
    #[serde(rename = "APNS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_p_n_s: Option<APNSPushNotificationTemplate>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Baidu")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baidu: Option<AndroidPushNotificationTemplate>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "Default")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<DefaultPushNotificationTemplate>,
    #[serde(rename = "DefaultSubstitutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_substitutions: Option<String>,
    #[serde(rename = "GCM")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g_c_m: Option<AndroidPushNotificationTemplate>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    #[serde(rename = "RecommenderId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommender_id: Option<String>,
    #[serde(rename = "TemplateDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_description: Option<String>,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    #[serde(rename = "TemplateType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_type: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRecommenderConfigurationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRecommenderConfigurationResponse {
    #[serde(rename = "RecommenderConfigurationResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommender_configuration_response: Option<RecommenderConfigurationResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRecommenderConfigurationsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRecommenderConfigurationsResponse {
    #[serde(rename = "ListRecommenderConfigurationsResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_recommender_configurations_response: Option<ListRecommenderConfigurationsResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRecommenderConfigurationsResponse {
    #[serde(rename = "Item")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Vec<RecommenderConfigurationResponse>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSegmentExportJobsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSegmentExportJobsResponse {
    #[serde(rename = "ExportJobsResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_jobs_response: Option<ExportJobsResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSegmentImportJobsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSegmentImportJobsResponse {
    #[serde(rename = "ImportJobsResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_jobs_response: Option<ImportJobsResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSegmentRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSegmentResponse {
    #[serde(rename = "SegmentResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_response: Option<SegmentResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSegmentVersionRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSegmentVersionResponse {
    #[serde(rename = "SegmentResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_response: Option<SegmentResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSegmentVersionsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSegmentVersionsResponse {
    #[serde(rename = "SegmentsResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments_response: Option<SegmentsResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SegmentsResponse {
    #[serde(rename = "Item")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Vec<SegmentResponse>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSegmentsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSegmentsResponse {
    #[serde(rename = "SegmentsResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments_response: Option<SegmentsResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSmsChannelRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSmsChannelResponse {
    #[serde(rename = "SMSChannelResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_m_s_channel_response: Option<SMSChannelResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSmsTemplateRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSmsTemplateResponse {
    #[serde(rename = "SMSTemplateResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_m_s_template_response: Option<SMSTemplateResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SMSTemplateResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Body")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "DefaultSubstitutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_substitutions: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    #[serde(rename = "RecommenderId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommender_id: Option<String>,
    #[serde(rename = "TemplateDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_description: Option<String>,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    #[serde(rename = "TemplateType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_type: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUserEndpointsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUserEndpointsResponse {
    #[serde(rename = "EndpointsResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints_response: Option<EndpointsResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetVoiceChannelRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetVoiceChannelResponse {
    #[serde(rename = "VoiceChannelResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_channel_response: Option<VoiceChannelResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetVoiceTemplateRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetVoiceTemplateResponse {
    #[serde(rename = "VoiceTemplateResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_template_response: Option<VoiceTemplateResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VoiceTemplateResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Body")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "DefaultSubstitutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_substitutions: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    #[serde(rename = "TemplateDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_description: Option<String>,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    #[serde(rename = "TemplateType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_type: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(rename = "VoiceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListJourneysRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListJourneysResponse {
    #[serde(rename = "JourneysResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journeys_response: Option<JourneysResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JourneysResponse {
    #[serde(rename = "Item")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Vec<JourneyResponse>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "TagsModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_model: Option<TagsModel>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagsModel {
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTemplateVersionsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTemplateVersionsResponse {
    #[serde(rename = "TemplateVersionsResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_versions_response: Option<TemplateVersionsResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TemplateVersionsResponse {
    #[serde(rename = "Item")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Vec<TemplateVersionResponse>>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RequestID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_i_d: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TemplateVersionResponse {
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "DefaultSubstitutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_substitutions: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    #[serde(rename = "TemplateDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_description: Option<String>,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    #[serde(rename = "TemplateType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_type: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTemplatesRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTemplatesResponse {
    #[serde(rename = "TemplatesResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub templates_response: Option<TemplatesResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TemplatesResponse {
    #[serde(rename = "Item")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Vec<TemplateResponse>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TemplateResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "DefaultSubstitutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_substitutions: Option<String>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<String>,
    #[serde(rename = "TemplateDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_description: Option<String>,
    #[serde(rename = "TemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<String>,
    #[serde(rename = "TemplateType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_type: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PhoneNumberValidateRequest {
    #[serde(rename = "NumberValidateRequest")]
    #[serde(default)]
    pub number_validate_request: NumberValidateRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NumberValidateRequest {
    #[serde(rename = "IsoCountryCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iso_country_code: Option<String>,
    #[serde(rename = "PhoneNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PhoneNumberValidateResponse {
    #[serde(rename = "NumberValidateResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_validate_response: Option<NumberValidateResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NumberValidateResponse {
    #[serde(rename = "Carrier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,
    #[serde(rename = "City")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "CleansedPhoneNumberE164")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cleansed_phone_number_e164: Option<String>,
    #[serde(rename = "CleansedPhoneNumberNational")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cleansed_phone_number_national: Option<String>,
    #[serde(rename = "Country")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "CountryCodeIso2")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code_iso2: Option<String>,
    #[serde(rename = "CountryCodeNumeric")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code_numeric: Option<String>,
    #[serde(rename = "County")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub county: Option<String>,
    #[serde(rename = "OriginalCountryCodeIso2")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_country_code_iso2: Option<String>,
    #[serde(rename = "OriginalPhoneNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_phone_number: Option<String>,
    #[serde(rename = "PhoneType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_type: Option<String>,
    #[serde(rename = "PhoneTypeCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_type_code: Option<i32>,
    #[serde(rename = "Timezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(rename = "ZipCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutEventStreamRequest {
    #[serde(rename = "WriteEventStream")]
    #[serde(default)]
    pub write_event_stream: WriteEventStream,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WriteEventStream {
    #[serde(rename = "DestinationStreamArn")]
    #[serde(default)]
    pub destination_stream_arn: String,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutEventStreamResponse {
    #[serde(rename = "EventStream")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_stream: Option<EventStream>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutEventsRequest {
    #[serde(rename = "EventsRequest")]
    #[serde(default)]
    pub events_request: EventsRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventsRequest {
    #[serde(rename = "BatchItem")]
    #[serde(default)]
    pub batch_item: std::collections::HashMap<String, EventsBatch>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventsBatch {
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    pub endpoint: PublicEndpoint,
    #[serde(rename = "Events")]
    #[serde(default)]
    pub events: std::collections::HashMap<String, Event>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PublicEndpoint {
    #[serde(rename = "Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "ChannelType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_type: Option<String>,
    #[serde(rename = "Demographic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demographic: Option<EndpointDemographic>,
    #[serde(rename = "EffectiveDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    #[serde(rename = "EndpointStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_status: Option<String>,
    #[serde(rename = "Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<EndpointLocation>,
    #[serde(rename = "Metrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<std::collections::HashMap<String, f64>>,
    #[serde(rename = "OptOut")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_out: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "User")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<EndpointUser>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Event {
    #[serde(rename = "AppPackageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_package_name: Option<String>,
    #[serde(rename = "AppTitle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_title: Option<String>,
    #[serde(rename = "AppVersionCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_version_code: Option<String>,
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ClientSdkVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_sdk_version: Option<String>,
    #[serde(rename = "EventType")]
    #[serde(default)]
    pub event_type: String,
    #[serde(rename = "Metrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<std::collections::HashMap<String, f64>>,
    #[serde(rename = "SdkName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdk_name: Option<String>,
    #[serde(rename = "Session")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<Session>,
    #[serde(rename = "Timestamp")]
    #[serde(default)]
    pub timestamp: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Session {
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "StartTimestamp")]
    #[serde(default)]
    pub start_timestamp: String,
    #[serde(rename = "StopTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_timestamp: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutEventsResponse {
    #[serde(rename = "EventsResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_response: Option<EventsResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventsResponse {
    #[serde(rename = "Results")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<std::collections::HashMap<String, ItemResponse>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ItemResponse {
    #[serde(rename = "EndpointItemResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_item_response: Option<EndpointItemResponse>,
    #[serde(rename = "EventsItemResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_item_response: Option<std::collections::HashMap<String, EventItemResponse>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EndpointItemResponse {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "StatusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventItemResponse {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "StatusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveAttributesRequest {
    #[serde(rename = "UpdateAttributesRequest")]
    #[serde(default)]
    pub update_attributes_request: UpdateAttributesRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAttributesRequest {
    #[serde(rename = "Blacklist")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blacklist: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveAttributesResponse {
    #[serde(rename = "AttributesResource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes_resource: Option<AttributesResource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttributesResource {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "AttributeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_type: Option<String>,
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendMessagesRequest {
    #[serde(rename = "MessageRequest")]
    #[serde(default)]
    pub message_request: MessageRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MessageRequest {
    #[serde(rename = "Addresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<std::collections::HashMap<String, AddressConfiguration>>,
    #[serde(rename = "Context")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Endpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<std::collections::HashMap<String, EndpointSendConfiguration>>,
    #[serde(rename = "MessageConfiguration")]
    #[serde(default)]
    pub message_configuration: DirectMessageConfiguration,
    #[serde(rename = "TemplateConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_configuration: Option<TemplateConfiguration>,
    #[serde(rename = "TraceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddressConfiguration {
    #[serde(rename = "BodyOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_override: Option<String>,
    #[serde(rename = "ChannelType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_type: Option<String>,
    #[serde(rename = "Context")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "RawContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_content: Option<String>,
    #[serde(rename = "Substitutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "TitleOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_override: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EndpointSendConfiguration {
    #[serde(rename = "BodyOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_override: Option<String>,
    #[serde(rename = "Context")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "RawContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_content: Option<String>,
    #[serde(rename = "Substitutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "TitleOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_override: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DirectMessageConfiguration {
    #[serde(rename = "ADMMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_d_m_message: Option<ADMMessage>,
    #[serde(rename = "APNSMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_p_n_s_message: Option<APNSMessage>,
    #[serde(rename = "BaiduMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baidu_message: Option<BaiduMessage>,
    #[serde(rename = "DefaultMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_message: Option<DefaultMessage>,
    #[serde(rename = "DefaultPushNotificationMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_push_notification_message: Option<DefaultPushNotificationMessage>,
    #[serde(rename = "EmailMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_message: Option<EmailMessage>,
    #[serde(rename = "GCMMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g_c_m_message: Option<GCMMessage>,
    #[serde(rename = "SMSMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_m_s_message: Option<SMSMessage>,
    #[serde(rename = "VoiceMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_message: Option<VoiceMessage>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ADMMessage {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "Body")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "ConsolidationKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consolidation_key: Option<String>,
    #[serde(rename = "Data")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ExpiresAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after: Option<String>,
    #[serde(rename = "IconReference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_reference: Option<String>,
    #[serde(rename = "ImageIconUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_icon_url: Option<String>,
    #[serde(rename = "ImageUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(rename = "MD5")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_d5: Option<String>,
    #[serde(rename = "RawContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_content: Option<String>,
    #[serde(rename = "SilentPush")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent_push: Option<bool>,
    #[serde(rename = "SmallImageIconUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_image_icon_url: Option<String>,
    #[serde(rename = "Sound")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,
    #[serde(rename = "Substitutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct APNSMessage {
    #[serde(rename = "APNSPushType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_p_n_s_push_type: Option<String>,
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "Badge")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge: Option<i32>,
    #[serde(rename = "Body")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "Category")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "CollapseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collapse_id: Option<String>,
    #[serde(rename = "Data")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "MediaUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_url: Option<String>,
    #[serde(rename = "PreferredAuthenticationMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_authentication_method: Option<String>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[serde(rename = "RawContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_content: Option<String>,
    #[serde(rename = "SilentPush")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent_push: Option<bool>,
    #[serde(rename = "Sound")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,
    #[serde(rename = "Substitutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "ThreadId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<String>,
    #[serde(rename = "TimeToLive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_live: Option<i32>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BaiduMessage {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "Body")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "Data")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "IconReference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_reference: Option<String>,
    #[serde(rename = "ImageIconUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_icon_url: Option<String>,
    #[serde(rename = "ImageUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(rename = "RawContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_content: Option<String>,
    #[serde(rename = "SilentPush")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent_push: Option<bool>,
    #[serde(rename = "SmallImageIconUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_image_icon_url: Option<String>,
    #[serde(rename = "Sound")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,
    #[serde(rename = "Substitutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "TimeToLive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_live: Option<i32>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DefaultMessage {
    #[serde(rename = "Body")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "Substitutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<std::collections::HashMap<String, Vec<String>>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DefaultPushNotificationMessage {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "Body")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "Data")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "SilentPush")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent_push: Option<bool>,
    #[serde(rename = "Substitutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EmailMessage {
    #[serde(rename = "Body")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "FeedbackForwardingAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_forwarding_address: Option<String>,
    #[serde(rename = "FromAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_address: Option<String>,
    #[serde(rename = "RawEmail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_email: Option<RawEmail>,
    #[serde(rename = "ReplyToAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_addresses: Option<Vec<String>>,
    #[serde(rename = "SimpleEmail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_email: Option<SimpleEmail>,
    #[serde(rename = "Substitutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<std::collections::HashMap<String, Vec<String>>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RawEmail {
    #[serde(rename = "Data")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SimpleEmail {
    #[serde(rename = "Headers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<MessageHeader>>,
    #[serde(rename = "HtmlPart")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_part: Option<SimpleEmailPart>,
    #[serde(rename = "Subject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<SimpleEmailPart>,
    #[serde(rename = "TextPart")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_part: Option<SimpleEmailPart>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SimpleEmailPart {
    #[serde(rename = "Charset")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charset: Option<String>,
    #[serde(rename = "Data")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GCMMessage {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "Body")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "CollapseKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collapse_key: Option<String>,
    #[serde(rename = "Data")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "IconReference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_reference: Option<String>,
    #[serde(rename = "ImageIconUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_icon_url: Option<String>,
    #[serde(rename = "ImageUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(rename = "PreferredAuthenticationMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_authentication_method: Option<String>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[serde(rename = "RawContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_content: Option<String>,
    #[serde(rename = "RestrictedPackageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_package_name: Option<String>,
    #[serde(rename = "SilentPush")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent_push: Option<bool>,
    #[serde(rename = "SmallImageIconUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_image_icon_url: Option<String>,
    #[serde(rename = "Sound")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sound: Option<String>,
    #[serde(rename = "Substitutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "TimeToLive")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_to_live: Option<i32>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SMSMessage {
    #[serde(rename = "Body")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "EntityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    #[serde(rename = "Keyword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
    #[serde(rename = "MediaUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_url: Option<String>,
    #[serde(rename = "MessageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    #[serde(rename = "OriginationNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origination_number: Option<String>,
    #[serde(rename = "SenderId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<String>,
    #[serde(rename = "Substitutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "TemplateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VoiceMessage {
    #[serde(rename = "Body")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "OriginationNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origination_number: Option<String>,
    #[serde(rename = "Substitutions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub substitutions: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "VoiceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendMessagesResponse {
    #[serde(rename = "MessageResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_response: Option<MessageResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MessageResponse {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "EndpointResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_result: Option<std::collections::HashMap<String, EndpointMessageResult>>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Result")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<std::collections::HashMap<String, MessageResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EndpointMessageResult {
    #[serde(rename = "Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "DeliveryStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_status: Option<String>,
    #[serde(rename = "MessageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(rename = "StatusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i32>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "UpdatedToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MessageResult {
    #[serde(rename = "DeliveryStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_status: Option<String>,
    #[serde(rename = "MessageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(rename = "StatusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i32>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "UpdatedToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendOTPMessageRequest {
    #[serde(rename = "SendOTPMessageRequestParameters")]
    #[serde(default)]
    pub send_o_t_p_message_request_parameters: SendOTPMessageRequestParameters,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendOTPMessageRequestParameters {
    #[serde(rename = "AllowedAttempts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_attempts: Option<i32>,
    #[serde(rename = "BrandName")]
    #[serde(default)]
    pub brand_name: String,
    #[serde(rename = "Channel")]
    #[serde(default)]
    pub channel: String,
    #[serde(rename = "CodeLength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_length: Option<i32>,
    #[serde(rename = "DestinationIdentity")]
    #[serde(default)]
    pub destination_identity: String,
    #[serde(rename = "EntityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    #[serde(rename = "Language")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(rename = "OriginationIdentity")]
    #[serde(default)]
    pub origination_identity: String,
    #[serde(rename = "ReferenceId")]
    #[serde(default)]
    pub reference_id: String,
    #[serde(rename = "TemplateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(rename = "ValidityPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendOTPMessageResponse {
    #[serde(rename = "MessageResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_response: Option<MessageResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendUsersMessagesRequest {
    #[serde(rename = "SendUsersMessageRequest")]
    #[serde(default)]
    pub send_users_message_request: SendUsersMessageRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendUsersMessageRequest {
    #[serde(rename = "Context")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "MessageConfiguration")]
    #[serde(default)]
    pub message_configuration: DirectMessageConfiguration,
    #[serde(rename = "TemplateConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_configuration: Option<TemplateConfiguration>,
    #[serde(rename = "TraceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    #[serde(rename = "Users")]
    #[serde(default)]
    pub users: std::collections::HashMap<String, EndpointSendConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendUsersMessagesResponse {
    #[serde(rename = "SendUsersMessageResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_users_message_response: Option<SendUsersMessageResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendUsersMessageResponse {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "Result")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<
        std::collections::HashMap<String, std::collections::HashMap<String, EndpointMessageResult>>,
    >,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "TagsModel")]
    #[serde(default)]
    pub tags_model: TagsModel,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAdmChannelRequest {
    #[serde(rename = "ADMChannelRequest")]
    #[serde(default)]
    pub a_d_m_channel_request: ADMChannelRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ADMChannelRequest {
    #[serde(rename = "ClientId")]
    #[serde(default)]
    pub client_id: String,
    #[serde(rename = "ClientSecret")]
    #[serde(default)]
    pub client_secret: String,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAdmChannelResponse {
    #[serde(rename = "ADMChannelResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_d_m_channel_response: Option<ADMChannelResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApnsChannelRequest {
    #[serde(rename = "APNSChannelRequest")]
    #[serde(default)]
    pub a_p_n_s_channel_request: APNSChannelRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct APNSChannelRequest {
    #[serde(rename = "BundleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    #[serde(rename = "Certificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[serde(rename = "DefaultAuthenticationMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_authentication_method: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "PrivateKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    #[serde(rename = "TeamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    #[serde(rename = "TokenKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_key: Option<String>,
    #[serde(rename = "TokenKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_key_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApnsChannelResponse {
    #[serde(rename = "APNSChannelResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_p_n_s_channel_response: Option<APNSChannelResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApnsSandboxChannelRequest {
    #[serde(rename = "APNSSandboxChannelRequest")]
    #[serde(default)]
    pub a_p_n_s_sandbox_channel_request: APNSSandboxChannelRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct APNSSandboxChannelRequest {
    #[serde(rename = "BundleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    #[serde(rename = "Certificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[serde(rename = "DefaultAuthenticationMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_authentication_method: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "PrivateKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    #[serde(rename = "TeamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    #[serde(rename = "TokenKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_key: Option<String>,
    #[serde(rename = "TokenKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_key_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApnsSandboxChannelResponse {
    #[serde(rename = "APNSSandboxChannelResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_p_n_s_sandbox_channel_response: Option<APNSSandboxChannelResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApnsVoipChannelRequest {
    #[serde(rename = "APNSVoipChannelRequest")]
    #[serde(default)]
    pub a_p_n_s_voip_channel_request: APNSVoipChannelRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct APNSVoipChannelRequest {
    #[serde(rename = "BundleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    #[serde(rename = "Certificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[serde(rename = "DefaultAuthenticationMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_authentication_method: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "PrivateKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    #[serde(rename = "TeamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    #[serde(rename = "TokenKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_key: Option<String>,
    #[serde(rename = "TokenKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_key_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApnsVoipChannelResponse {
    #[serde(rename = "APNSVoipChannelResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_p_n_s_voip_channel_response: Option<APNSVoipChannelResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApnsVoipSandboxChannelRequest {
    #[serde(rename = "APNSVoipSandboxChannelRequest")]
    #[serde(default)]
    pub a_p_n_s_voip_sandbox_channel_request: APNSVoipSandboxChannelRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct APNSVoipSandboxChannelRequest {
    #[serde(rename = "BundleId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    #[serde(rename = "Certificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[serde(rename = "DefaultAuthenticationMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_authentication_method: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "PrivateKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    #[serde(rename = "TeamId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    #[serde(rename = "TokenKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_key: Option<String>,
    #[serde(rename = "TokenKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_key_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApnsVoipSandboxChannelResponse {
    #[serde(rename = "APNSVoipSandboxChannelResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_p_n_s_voip_sandbox_channel_response: Option<APNSVoipSandboxChannelResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApplicationSettingsRequest {
    #[serde(rename = "WriteApplicationSettingsRequest")]
    #[serde(default)]
    pub write_application_settings_request: WriteApplicationSettingsRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WriteApplicationSettingsRequest {
    #[serde(rename = "CampaignHook")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_hook: Option<CampaignHook>,
    #[serde(rename = "CloudWatchMetricsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_metrics_enabled: Option<bool>,
    #[serde(rename = "EventTaggingEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_tagging_enabled: Option<bool>,
    #[serde(rename = "JourneyLimits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journey_limits: Option<ApplicationSettingsJourneyLimits>,
    #[serde(rename = "Limits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<CampaignLimits>,
    #[serde(rename = "QuietTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quiet_time: Option<QuietTime>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApplicationSettingsResponse {
    #[serde(rename = "ApplicationSettingsResource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_settings_resource: Option<ApplicationSettingsResource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBaiduChannelRequest {
    #[serde(rename = "BaiduChannelRequest")]
    #[serde(default)]
    pub baidu_channel_request: BaiduChannelRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BaiduChannelRequest {
    #[serde(rename = "ApiKey")]
    #[serde(default)]
    pub api_key: String,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "SecretKey")]
    #[serde(default)]
    pub secret_key: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBaiduChannelResponse {
    #[serde(rename = "BaiduChannelResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baidu_channel_response: Option<BaiduChannelResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCampaignRequest {
    #[serde(rename = "WriteCampaignRequest")]
    #[serde(default)]
    pub write_campaign_request: WriteCampaignRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCampaignResponse {
    #[serde(rename = "CampaignResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_response: Option<CampaignResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEmailChannelRequest {
    #[serde(rename = "EmailChannelRequest")]
    #[serde(default)]
    pub email_channel_request: EmailChannelRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EmailChannelRequest {
    #[serde(rename = "ConfigurationSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_set: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "FromAddress")]
    #[serde(default)]
    pub from_address: String,
    #[serde(rename = "Identity")]
    #[serde(default)]
    pub identity: String,
    #[serde(rename = "OrchestrationSendingRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orchestration_sending_role_arn: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEmailChannelResponse {
    #[serde(rename = "EmailChannelResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_channel_response: Option<EmailChannelResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEmailTemplateRequest {
    #[serde(rename = "EmailTemplateRequest")]
    #[serde(default)]
    pub email_template_request: EmailTemplateRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEmailTemplateResponse {
    #[serde(rename = "MessageBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_body: Option<MessageBody>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEndpointRequest {
    #[serde(rename = "EndpointRequest")]
    #[serde(default)]
    pub endpoint_request: EndpointRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EndpointRequest {
    #[serde(rename = "Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "ChannelType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_type: Option<String>,
    #[serde(rename = "Demographic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demographic: Option<EndpointDemographic>,
    #[serde(rename = "EffectiveDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    #[serde(rename = "EndpointStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_status: Option<String>,
    #[serde(rename = "Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<EndpointLocation>,
    #[serde(rename = "Metrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<std::collections::HashMap<String, f64>>,
    #[serde(rename = "OptOut")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_out: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "User")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<EndpointUser>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEndpointResponse {
    #[serde(rename = "MessageBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_body: Option<MessageBody>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEndpointsBatchRequest {
    #[serde(rename = "EndpointBatchRequest")]
    #[serde(default)]
    pub endpoint_batch_request: EndpointBatchRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EndpointBatchRequest {
    #[serde(rename = "Item")]
    #[serde(default)]
    pub item: Vec<EndpointBatchItem>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EndpointBatchItem {
    #[serde(rename = "Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "ChannelType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_type: Option<String>,
    #[serde(rename = "Demographic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demographic: Option<EndpointDemographic>,
    #[serde(rename = "EffectiveDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    #[serde(rename = "EndpointStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_status: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<EndpointLocation>,
    #[serde(rename = "Metrics")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<std::collections::HashMap<String, f64>>,
    #[serde(rename = "OptOut")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_out: Option<String>,
    #[serde(rename = "RequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "User")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<EndpointUser>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEndpointsBatchResponse {
    #[serde(rename = "MessageBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_body: Option<MessageBody>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGcmChannelRequest {
    #[serde(rename = "GCMChannelRequest")]
    #[serde(default)]
    pub g_c_m_channel_request: GCMChannelRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GCMChannelRequest {
    #[serde(rename = "ApiKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[serde(rename = "DefaultAuthenticationMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_authentication_method: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "ServiceJson")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_json: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGcmChannelResponse {
    #[serde(rename = "GCMChannelResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub g_c_m_channel_response: Option<GCMChannelResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateInAppTemplateRequest {
    #[serde(rename = "InAppTemplateRequest")]
    #[serde(default)]
    pub in_app_template_request: InAppTemplateRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateInAppTemplateResponse {
    #[serde(rename = "MessageBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_body: Option<MessageBody>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateJourneyRequest {
    #[serde(rename = "WriteJourneyRequest")]
    #[serde(default)]
    pub write_journey_request: WriteJourneyRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateJourneyResponse {
    #[serde(rename = "JourneyResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journey_response: Option<JourneyResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateJourneyStateRequest {
    #[serde(rename = "JourneyStateRequest")]
    #[serde(default)]
    pub journey_state_request: JourneyStateRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JourneyStateRequest {
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateJourneyStateResponse {
    #[serde(rename = "JourneyResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journey_response: Option<JourneyResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePushTemplateRequest {
    #[serde(rename = "PushNotificationTemplateRequest")]
    #[serde(default)]
    pub push_notification_template_request: PushNotificationTemplateRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePushTemplateResponse {
    #[serde(rename = "MessageBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_body: Option<MessageBody>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRecommenderConfigurationRequest {
    #[serde(rename = "UpdateRecommenderConfiguration")]
    #[serde(default)]
    pub update_recommender_configuration: UpdateRecommenderConfigurationShape,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRecommenderConfigurationShape {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RecommendationProviderIdType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_provider_id_type: Option<String>,
    #[serde(rename = "RecommendationProviderRoleArn")]
    #[serde(default)]
    pub recommendation_provider_role_arn: String,
    #[serde(rename = "RecommendationProviderUri")]
    #[serde(default)]
    pub recommendation_provider_uri: String,
    #[serde(rename = "RecommendationTransformerUri")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_transformer_uri: Option<String>,
    #[serde(rename = "RecommendationsDisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendations_display_name: Option<String>,
    #[serde(rename = "RecommendationsPerMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendations_per_message: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateRecommenderConfigurationResponse {
    #[serde(rename = "RecommenderConfigurationResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommender_configuration_response: Option<RecommenderConfigurationResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSegmentRequest {
    #[serde(rename = "WriteSegmentRequest")]
    #[serde(default)]
    pub write_segment_request: WriteSegmentRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSegmentResponse {
    #[serde(rename = "SegmentResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_response: Option<SegmentResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSmsChannelRequest {
    #[serde(rename = "SMSChannelRequest")]
    #[serde(default)]
    pub s_m_s_channel_request: SMSChannelRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SMSChannelRequest {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "SenderId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_id: Option<String>,
    #[serde(rename = "ShortCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSmsChannelResponse {
    #[serde(rename = "SMSChannelResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_m_s_channel_response: Option<SMSChannelResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSmsTemplateRequest {
    #[serde(rename = "SMSTemplateRequest")]
    #[serde(default)]
    pub s_m_s_template_request: SMSTemplateRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSmsTemplateResponse {
    #[serde(rename = "MessageBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_body: Option<MessageBody>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTemplateActiveVersionRequest {
    #[serde(rename = "TemplateActiveVersionRequest")]
    #[serde(default)]
    pub template_active_version_request: TemplateActiveVersionRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TemplateActiveVersionRequest {
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTemplateActiveVersionResponse {
    #[serde(rename = "MessageBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_body: Option<MessageBody>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateVoiceChannelRequest {
    #[serde(rename = "VoiceChannelRequest")]
    #[serde(default)]
    pub voice_channel_request: VoiceChannelRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VoiceChannelRequest {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateVoiceChannelResponse {
    #[serde(rename = "VoiceChannelResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_channel_response: Option<VoiceChannelResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateVoiceTemplateRequest {
    #[serde(rename = "VoiceTemplateRequest")]
    #[serde(default)]
    pub voice_template_request: VoiceTemplateRequest,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateVoiceTemplateResponse {
    #[serde(rename = "MessageBody")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_body: Option<MessageBody>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VerifyOTPMessageRequest {
    #[serde(rename = "VerifyOTPMessageRequestParameters")]
    #[serde(default)]
    pub verify_o_t_p_message_request_parameters: VerifyOTPMessageRequestParameters,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VerifyOTPMessageRequestParameters {
    #[serde(rename = "DestinationIdentity")]
    #[serde(default)]
    pub destination_identity: String,
    #[serde(rename = "Otp")]
    #[serde(default)]
    pub otp: String,
    #[serde(rename = "ReferenceId")]
    #[serde(default)]
    pub reference_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VerifyOTPMessageResponse {
    #[serde(rename = "VerificationResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_response: Option<VerificationResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VerificationResponse {
    #[serde(rename = "Valid")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
}
