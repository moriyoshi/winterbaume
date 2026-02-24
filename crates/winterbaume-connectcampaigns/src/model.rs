//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-connectcampaigns

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCampaignRequest {
    #[serde(rename = "connectInstanceId")]
    #[serde(default)]
    pub connect_instance_id: String,
    #[serde(rename = "dialerConfig")]
    #[serde(default)]
    pub dialer_config: DialerConfig,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "outboundCallConfig")]
    #[serde(default)]
    pub outbound_call_config: OutboundCallConfig,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DialerConfig {
    #[serde(rename = "agentlessDialerConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agentless_dialer_config: Option<AgentlessDialerConfig>,
    #[serde(rename = "predictiveDialerConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predictive_dialer_config: Option<PredictiveDialerConfig>,
    #[serde(rename = "progressiveDialerConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progressive_dialer_config: Option<ProgressiveDialerConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgentlessDialerConfig {
    #[serde(rename = "dialingCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialing_capacity: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PredictiveDialerConfig {
    #[serde(rename = "bandwidthAllocation")]
    #[serde(default)]
    pub bandwidth_allocation: f64,
    #[serde(rename = "dialingCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialing_capacity: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProgressiveDialerConfig {
    #[serde(rename = "bandwidthAllocation")]
    #[serde(default)]
    pub bandwidth_allocation: f64,
    #[serde(rename = "dialingCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialing_capacity: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutboundCallConfig {
    #[serde(rename = "answerMachineDetectionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer_machine_detection_config: Option<AnswerMachineDetectionConfig>,
    #[serde(rename = "connectContactFlowId")]
    #[serde(default)]
    pub connect_contact_flow_id: String,
    #[serde(rename = "connectQueueId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_queue_id: Option<String>,
    #[serde(rename = "connectSourcePhoneNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_source_phone_number: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnswerMachineDetectionConfig {
    #[serde(rename = "awaitAnswerMachinePrompt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub await_answer_machine_prompt: Option<bool>,
    #[serde(rename = "enableAnswerMachineDetection")]
    #[serde(default)]
    pub enable_answer_machine_detection: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCampaignResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCampaignRequest {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConnectInstanceConfigRequest {
    #[serde(rename = "connectInstanceId")]
    #[serde(default)]
    pub connect_instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteInstanceOnboardingJobRequest {
    #[serde(rename = "connectInstanceId")]
    #[serde(default)]
    pub connect_instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCampaignRequest {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCampaignResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign: Option<Campaign>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Campaign {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "connectInstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_instance_id: Option<String>,
    #[serde(rename = "dialerConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialer_config: Option<DialerConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "outboundCallConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_call_config: Option<OutboundCallConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCampaignStateBatchRequest {
    #[serde(rename = "campaignIds")]
    #[serde(default)]
    pub campaign_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCampaignStateBatchResponse {
    #[serde(rename = "failedRequests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_requests: Option<Vec<FailedCampaignStateResponse>>,
    #[serde(rename = "successfulRequests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_requests: Option<Vec<SuccessfulCampaignStateResponse>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailedCampaignStateResponse {
    #[serde(rename = "campaignId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    #[serde(rename = "failureCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SuccessfulCampaignStateResponse {
    #[serde(rename = "campaignId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCampaignStateRequest {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCampaignStateResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConnectInstanceConfigRequest {
    #[serde(rename = "connectInstanceId")]
    #[serde(default)]
    pub connect_instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConnectInstanceConfigResponse {
    #[serde(rename = "connectInstanceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_instance_config: Option<InstanceConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceConfig {
    #[serde(rename = "connectInstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_instance_id: Option<String>,
    #[serde(rename = "encryptionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_config: Option<EncryptionConfig>,
    #[serde(rename = "serviceLinkedRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_linked_role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptionConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "encryptionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    #[serde(rename = "keyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInstanceOnboardingJobStatusRequest {
    #[serde(rename = "connectInstanceId")]
    #[serde(default)]
    pub connect_instance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetInstanceOnboardingJobStatusResponse {
    #[serde(rename = "connectInstanceOnboardingJobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_instance_onboarding_job_status: Option<InstanceOnboardingJobStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceOnboardingJobStatus {
    #[serde(rename = "connectInstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_instance_id: Option<String>,
    #[serde(rename = "failureCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCampaignsRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<CampaignFilters>,
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
pub struct CampaignFilters {
    #[serde(rename = "instanceIdFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id_filter: Option<InstanceIdFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceIdFilter {
    #[serde(default)]
    pub operator: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCampaignsResponse {
    #[serde(rename = "campaignSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_summary_list: Option<Vec<CampaignSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CampaignSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "connectInstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_instance_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PauseCampaignRequest {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutDialRequestBatchRequest {
    #[serde(rename = "dialRequests")]
    #[serde(default)]
    pub dial_requests: Vec<DialRequest>,
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DialRequest {
    #[serde(default)]
    pub attributes: std::collections::HashMap<String, String>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    pub client_token: String,
    #[serde(rename = "expirationTime")]
    #[serde(default)]
    pub expiration_time: String,
    #[serde(rename = "phoneNumber")]
    #[serde(default)]
    pub phone_number: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutDialRequestBatchResponse {
    #[serde(rename = "failedRequests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_requests: Option<Vec<FailedRequest>>,
    #[serde(rename = "successfulRequests")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_requests: Option<Vec<SuccessfulRequest>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailedRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "failureCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SuccessfulRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResumeCampaignRequest {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartCampaignRequest {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartInstanceOnboardingJobRequest {
    #[serde(rename = "connectInstanceId")]
    #[serde(default)]
    pub connect_instance_id: String,
    #[serde(rename = "encryptionConfig")]
    #[serde(default)]
    pub encryption_config: EncryptionConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartInstanceOnboardingJobResponse {
    #[serde(rename = "connectInstanceOnboardingJobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_instance_onboarding_job_status: Option<InstanceOnboardingJobStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopCampaignRequest {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(default)]
    pub arn: String,
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "tagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCampaignDialerConfigRequest {
    #[serde(rename = "dialerConfig")]
    #[serde(default)]
    pub dialer_config: DialerConfig,
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCampaignNameRequest {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCampaignOutboundCallConfigRequest {
    #[serde(rename = "answerMachineDetectionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer_machine_detection_config: Option<AnswerMachineDetectionConfig>,
    #[serde(rename = "connectContactFlowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_contact_flow_id: Option<String>,
    #[serde(rename = "connectSourcePhoneNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_source_phone_number: Option<String>,
    #[serde(default)]
    pub id: String,
}
