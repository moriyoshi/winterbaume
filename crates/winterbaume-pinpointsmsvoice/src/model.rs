//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-pinpointsmsvoice

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConfigurationSetEventDestinationRequest {
    #[serde(rename = "EventDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_destination: Option<EventDestinationDefinition>,
    #[serde(rename = "EventDestinationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_destination_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventDestinationDefinition {
    #[serde(rename = "CloudWatchLogsDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_destination: Option<CloudWatchLogsDestination>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "KinesisFirehoseDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_firehose_destination: Option<KinesisFirehoseDestination>,
    #[serde(rename = "MatchingEventTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub matching_event_types: Option<Vec<String>>,
    #[serde(rename = "SnsDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_destination: Option<SnsDestination>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudWatchLogsDestination {
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    #[serde(rename = "LogGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KinesisFirehoseDestination {
    #[serde(rename = "DeliveryStreamArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_stream_arn: Option<String>,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnsDestination {
    #[serde(rename = "TopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConfigurationSetEventDestinationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConfigurationSetRequest {
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_set_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConfigurationSetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConfigurationSetEventDestinationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConfigurationSetEventDestinationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConfigurationSetRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConfigurationSetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConfigurationSetEventDestinationsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConfigurationSetEventDestinationsResponse {
    #[serde(rename = "EventDestinations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_destinations: Option<Vec<EventDestination>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventDestination {
    #[serde(rename = "CloudWatchLogsDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_destination: Option<CloudWatchLogsDestination>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
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
    #[serde(rename = "SnsDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_destination: Option<SnsDestination>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConfigurationSetsRequest {}

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
pub struct SendVoiceMessageRequest {
    #[serde(rename = "CallerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caller_id: Option<String>,
    #[serde(rename = "ConfigurationSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_set_name: Option<String>,
    #[serde(rename = "Content")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<VoiceMessageContent>,
    #[serde(rename = "DestinationPhoneNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_phone_number: Option<String>,
    #[serde(rename = "OriginationPhoneNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origination_phone_number: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VoiceMessageContent {
    #[serde(rename = "CallInstructionsMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_instructions_message: Option<CallInstructionsMessageType>,
    #[serde(rename = "PlainTextMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plain_text_message: Option<PlainTextMessageType>,
    #[serde(rename = "SSMLMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_s_m_l_message: Option<SSMLMessageType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CallInstructionsMessageType {
    #[serde(rename = "Text")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PlainTextMessageType {
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "Text")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "VoiceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SSMLMessageType {
    #[serde(rename = "LanguageCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    #[serde(rename = "Text")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "VoiceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SendVoiceMessageResponse {
    #[serde(rename = "MessageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConfigurationSetEventDestinationRequest {
    #[serde(rename = "EventDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_destination: Option<EventDestinationDefinition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConfigurationSetEventDestinationResponse {}
