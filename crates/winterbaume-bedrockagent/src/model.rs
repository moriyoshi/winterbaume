//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-bedrockagent

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateAgentCollaboratorRequest {
    #[serde(rename = "agentDescriptor")]
    #[serde(default)]
    pub agent_descriptor: AgentDescriptor,
    #[serde(rename = "agentId")]
    #[serde(default)]
    pub agent_id: String,
    #[serde(rename = "agentVersion")]
    #[serde(default)]
    pub agent_version: String,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "collaborationInstruction")]
    #[serde(default)]
    pub collaboration_instruction: String,
    #[serde(rename = "collaboratorName")]
    #[serde(default)]
    pub collaborator_name: String,
    #[serde(rename = "relayConversationHistory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relay_conversation_history: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgentDescriptor {
    #[serde(rename = "aliasArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateAgentCollaboratorResponse {
    #[serde(rename = "agentCollaborator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_collaborator: Option<AgentCollaborator>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgentCollaborator {
    #[serde(rename = "agentDescriptor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_descriptor: Option<AgentDescriptor>,
    #[serde(rename = "agentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(rename = "agentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "collaborationInstruction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collaboration_instruction: Option<String>,
    #[serde(rename = "collaboratorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collaborator_id: Option<String>,
    #[serde(rename = "collaboratorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collaborator_name: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    #[serde(rename = "relayConversationHistory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relay_conversation_history: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateAgentKnowledgeBaseRequest {
    #[serde(rename = "agentId")]
    #[serde(default)]
    pub agent_id: String,
    #[serde(rename = "agentVersion")]
    #[serde(default)]
    pub agent_version: String,
    #[serde(default)]
    pub description: String,
    #[serde(rename = "knowledgeBaseId")]
    #[serde(default)]
    pub knowledge_base_id: String,
    #[serde(rename = "knowledgeBaseState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateAgentKnowledgeBaseResponse {
    #[serde(rename = "agentKnowledgeBase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_knowledge_base: Option<AgentKnowledgeBase>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgentKnowledgeBase {
    #[serde(rename = "agentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(rename = "agentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "knowledgeBaseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base_id: Option<String>,
    #[serde(rename = "knowledgeBaseState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base_state: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAgentActionGroupRequest {
    #[serde(rename = "actionGroupExecutor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_group_executor: Option<ActionGroupExecutor>,
    #[serde(rename = "actionGroupName")]
    #[serde(default)]
    pub action_group_name: String,
    #[serde(rename = "actionGroupState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_group_state: Option<String>,
    #[serde(rename = "agentId")]
    #[serde(default)]
    pub agent_id: String,
    #[serde(rename = "agentVersion")]
    #[serde(default)]
    pub agent_version: String,
    #[serde(rename = "apiSchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_schema: Option<APISchema>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "functionSchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_schema: Option<FunctionSchema>,
    #[serde(rename = "parentActionGroupSignature")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_action_group_signature: Option<String>,
    #[serde(rename = "parentActionGroupSignatureParams")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_action_group_signature_params: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionGroupExecutor {
    #[serde(rename = "customControl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_control: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct APISchema {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3: Option<S3Identifier>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Identifier {
    #[serde(rename = "s3BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,
    #[serde(rename = "s3ObjectKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FunctionSchema {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<Function>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Function {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, ParameterDetail>>,
    #[serde(rename = "requireConfirmation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_confirmation: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParameterDetail {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAgentActionGroupResponse {
    #[serde(rename = "agentActionGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_action_group: Option<AgentActionGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgentActionGroup {
    #[serde(rename = "actionGroupExecutor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_group_executor: Option<ActionGroupExecutor>,
    #[serde(rename = "actionGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_group_id: Option<String>,
    #[serde(rename = "actionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_group_name: Option<String>,
    #[serde(rename = "actionGroupState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_group_state: Option<String>,
    #[serde(rename = "agentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(rename = "agentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[serde(rename = "apiSchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_schema: Option<APISchema>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "functionSchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_schema: Option<FunctionSchema>,
    #[serde(rename = "parentActionGroupSignatureParams")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_action_group_signature_params: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "parentActionSignature")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_action_signature: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAgentAliasRequest {
    #[serde(rename = "agentAliasName")]
    #[serde(default)]
    pub agent_alias_name: String,
    #[serde(rename = "agentId")]
    #[serde(default)]
    pub agent_id: String,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "routingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_configuration: Option<Vec<AgentAliasRoutingConfigurationListItem>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgentAliasRoutingConfigurationListItem {
    #[serde(rename = "agentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[serde(rename = "provisionedThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_throughput: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAgentAliasResponse {
    #[serde(rename = "agentAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_alias: Option<AgentAlias>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgentAlias {
    #[serde(rename = "agentAliasArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_alias_arn: Option<String>,
    #[serde(rename = "agentAliasHistoryEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_alias_history_events: Option<Vec<AgentAliasHistoryEvent>>,
    #[serde(rename = "agentAliasId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_alias_id: Option<String>,
    #[serde(rename = "agentAliasName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_alias_name: Option<String>,
    #[serde(rename = "agentAliasStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_alias_status: Option<String>,
    #[serde(rename = "agentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(rename = "aliasInvocationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_invocation_state: Option<String>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "failureReasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reasons: Option<Vec<String>>,
    #[serde(rename = "routingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_configuration: Option<Vec<AgentAliasRoutingConfigurationListItem>>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgentAliasHistoryEvent {
    #[serde(rename = "endDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "routingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_configuration: Option<Vec<AgentAliasRoutingConfigurationListItem>>,
    #[serde(rename = "startDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAgentRequest {
    #[serde(rename = "agentCollaboration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_collaboration: Option<String>,
    #[serde(rename = "agentName")]
    #[serde(default)]
    pub agent_name: String,
    #[serde(rename = "agentResourceRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_resource_role_arn: Option<String>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "customOrchestration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_orchestration: Option<CustomOrchestration>,
    #[serde(rename = "customerEncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_encryption_key_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "foundationModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foundation_model: Option<String>,
    #[serde(rename = "guardrailConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrail_configuration: Option<GuardrailConfiguration>,
    #[serde(rename = "idleSessionTTLInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_session_t_t_l_in_seconds: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instruction: Option<String>,
    #[serde(rename = "memoryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_configuration: Option<MemoryConfiguration>,
    #[serde(rename = "orchestrationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orchestration_type: Option<String>,
    #[serde(rename = "promptOverrideConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_override_configuration: Option<PromptOverrideConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomOrchestration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executor: Option<OrchestrationExecutor>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrchestrationExecutor {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GuardrailConfiguration {
    #[serde(rename = "guardrailIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrail_identifier: Option<String>,
    #[serde(rename = "guardrailVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrail_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MemoryConfiguration {
    #[serde(rename = "enabledMemoryTypes")]
    #[serde(default)]
    pub enabled_memory_types: Vec<String>,
    #[serde(rename = "sessionSummaryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_summary_configuration: Option<SessionSummaryConfiguration>,
    #[serde(rename = "storageDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_days: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SessionSummaryConfiguration {
    #[serde(rename = "maxRecentSessions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_recent_sessions: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PromptOverrideConfiguration {
    #[serde(rename = "overrideLambda")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_lambda: Option<String>,
    #[serde(rename = "promptConfigurations")]
    #[serde(default)]
    pub prompt_configurations: Vec<PromptConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PromptConfiguration {
    #[serde(rename = "additionalModelRequestFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_model_request_fields: Option<serde_json::Value>,
    #[serde(rename = "basePromptTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_prompt_template: Option<String>,
    #[serde(rename = "foundationModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foundation_model: Option<String>,
    #[serde(rename = "inferenceConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_configuration: Option<InferenceConfiguration>,
    #[serde(rename = "parserMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parser_mode: Option<String>,
    #[serde(rename = "promptCreationMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_creation_mode: Option<String>,
    #[serde(rename = "promptState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_state: Option<String>,
    #[serde(rename = "promptType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InferenceConfiguration {
    #[serde(rename = "maximumLength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_length: Option<i32>,
    #[serde(rename = "stopSequences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(rename = "topK")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i32>,
    #[serde(rename = "topP")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAgentResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<Agent>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Agent {
    #[serde(rename = "agentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_arn: Option<String>,
    #[serde(rename = "agentCollaboration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_collaboration: Option<String>,
    #[serde(rename = "agentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(rename = "agentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<String>,
    #[serde(rename = "agentResourceRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_resource_role_arn: Option<String>,
    #[serde(rename = "agentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_status: Option<String>,
    #[serde(rename = "agentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "customOrchestration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_orchestration: Option<CustomOrchestration>,
    #[serde(rename = "customerEncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_encryption_key_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "failureReasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reasons: Option<Vec<String>>,
    #[serde(rename = "foundationModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foundation_model: Option<String>,
    #[serde(rename = "guardrailConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrail_configuration: Option<GuardrailConfiguration>,
    #[serde(rename = "idleSessionTTLInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_session_t_t_l_in_seconds: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instruction: Option<String>,
    #[serde(rename = "memoryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_configuration: Option<MemoryConfiguration>,
    #[serde(rename = "orchestrationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orchestration_type: Option<String>,
    #[serde(rename = "preparedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepared_at: Option<String>,
    #[serde(rename = "promptOverrideConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_override_configuration: Option<PromptOverrideConfiguration>,
    #[serde(rename = "recommendedActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_actions: Option<Vec<String>>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDataSourceRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "dataDeletionPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_deletion_policy: Option<String>,
    #[serde(rename = "dataSourceConfiguration")]
    #[serde(default)]
    pub data_source_configuration: DataSourceConfiguration,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "knowledgeBaseId")]
    #[serde(default)]
    pub knowledge_base_id: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "serverSideEncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_side_encryption_configuration: Option<ServerSideEncryptionConfiguration>,
    #[serde(rename = "vectorIngestionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_ingestion_configuration: Option<VectorIngestionConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSourceConfiguration {
    #[serde(rename = "confluenceConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confluence_configuration: Option<ConfluenceDataSourceConfiguration>,
    #[serde(rename = "s3Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_configuration: Option<S3DataSourceConfiguration>,
    #[serde(rename = "salesforceConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salesforce_configuration: Option<SalesforceDataSourceConfiguration>,
    #[serde(rename = "sharePointConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_point_configuration: Option<SharePointDataSourceConfiguration>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "webConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_configuration: Option<WebDataSourceConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfluenceDataSourceConfiguration {
    #[serde(rename = "crawlerConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler_configuration: Option<ConfluenceCrawlerConfiguration>,
    #[serde(rename = "sourceConfiguration")]
    #[serde(default)]
    pub source_configuration: ConfluenceSourceConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfluenceCrawlerConfiguration {
    #[serde(rename = "filterConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_configuration: Option<CrawlFilterConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CrawlFilterConfiguration {
    #[serde(rename = "patternObjectFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern_object_filter: Option<PatternObjectFilterConfiguration>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PatternObjectFilterConfiguration {
    #[serde(default)]
    pub filters: Vec<PatternObjectFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PatternObjectFilter {
    #[serde(rename = "exclusionFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusion_filters: Option<Vec<String>>,
    #[serde(rename = "inclusionFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusion_filters: Option<Vec<String>>,
    #[serde(rename = "objectType")]
    #[serde(default)]
    pub object_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfluenceSourceConfiguration {
    #[serde(rename = "authType")]
    #[serde(default)]
    pub auth_type: String,
    #[serde(rename = "credentialsSecretArn")]
    #[serde(default)]
    pub credentials_secret_arn: String,
    #[serde(rename = "hostType")]
    #[serde(default)]
    pub host_type: String,
    #[serde(rename = "hostUrl")]
    #[serde(default)]
    pub host_url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3DataSourceConfiguration {
    #[serde(rename = "bucketArn")]
    #[serde(default)]
    pub bucket_arn: String,
    #[serde(rename = "bucketOwnerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_owner_account_id: Option<String>,
    #[serde(rename = "inclusionPrefixes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusion_prefixes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SalesforceDataSourceConfiguration {
    #[serde(rename = "crawlerConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler_configuration: Option<SalesforceCrawlerConfiguration>,
    #[serde(rename = "sourceConfiguration")]
    #[serde(default)]
    pub source_configuration: SalesforceSourceConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SalesforceCrawlerConfiguration {
    #[serde(rename = "filterConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_configuration: Option<CrawlFilterConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SalesforceSourceConfiguration {
    #[serde(rename = "authType")]
    #[serde(default)]
    pub auth_type: String,
    #[serde(rename = "credentialsSecretArn")]
    #[serde(default)]
    pub credentials_secret_arn: String,
    #[serde(rename = "hostUrl")]
    #[serde(default)]
    pub host_url: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SharePointDataSourceConfiguration {
    #[serde(rename = "crawlerConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler_configuration: Option<SharePointCrawlerConfiguration>,
    #[serde(rename = "sourceConfiguration")]
    #[serde(default)]
    pub source_configuration: SharePointSourceConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SharePointCrawlerConfiguration {
    #[serde(rename = "filterConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_configuration: Option<CrawlFilterConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SharePointSourceConfiguration {
    #[serde(rename = "authType")]
    #[serde(default)]
    pub auth_type: String,
    #[serde(rename = "credentialsSecretArn")]
    #[serde(default)]
    pub credentials_secret_arn: String,
    #[serde(default)]
    pub domain: String,
    #[serde(rename = "hostType")]
    #[serde(default)]
    pub host_type: String,
    #[serde(rename = "siteUrls")]
    #[serde(default)]
    pub site_urls: Vec<String>,
    #[serde(rename = "tenantId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WebDataSourceConfiguration {
    #[serde(rename = "crawlerConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler_configuration: Option<WebCrawlerConfiguration>,
    #[serde(rename = "sourceConfiguration")]
    #[serde(default)]
    pub source_configuration: WebSourceConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WebCrawlerConfiguration {
    #[serde(rename = "crawlerLimits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crawler_limits: Option<WebCrawlerLimits>,
    #[serde(rename = "exclusionFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusion_filters: Option<Vec<String>>,
    #[serde(rename = "inclusionFilters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusion_filters: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(rename = "userAgent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    #[serde(rename = "userAgentHeader")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent_header: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WebCrawlerLimits {
    #[serde(rename = "maxPages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_pages: Option<i32>,
    #[serde(rename = "rateLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WebSourceConfiguration {
    #[serde(rename = "urlConfiguration")]
    #[serde(default)]
    pub url_configuration: UrlConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UrlConfiguration {
    #[serde(rename = "seedUrls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_urls: Option<Vec<SeedUrl>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SeedUrl {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServerSideEncryptionConfiguration {
    #[serde(rename = "kmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VectorIngestionConfiguration {
    #[serde(rename = "chunkingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunking_configuration: Option<ChunkingConfiguration>,
    #[serde(rename = "contextEnrichmentConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_enrichment_configuration: Option<ContextEnrichmentConfiguration>,
    #[serde(rename = "customTransformationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_transformation_configuration: Option<CustomTransformationConfiguration>,
    #[serde(rename = "parsingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parsing_configuration: Option<ParsingConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChunkingConfiguration {
    #[serde(rename = "chunkingStrategy")]
    #[serde(default)]
    pub chunking_strategy: String,
    #[serde(rename = "fixedSizeChunkingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_size_chunking_configuration: Option<FixedSizeChunkingConfiguration>,
    #[serde(rename = "hierarchicalChunkingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hierarchical_chunking_configuration: Option<HierarchicalChunkingConfiguration>,
    #[serde(rename = "semanticChunkingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_chunking_configuration: Option<SemanticChunkingConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FixedSizeChunkingConfiguration {
    #[serde(rename = "maxTokens")]
    #[serde(default)]
    pub max_tokens: i32,
    #[serde(rename = "overlapPercentage")]
    #[serde(default)]
    pub overlap_percentage: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HierarchicalChunkingConfiguration {
    #[serde(rename = "levelConfigurations")]
    #[serde(default)]
    pub level_configurations: Vec<HierarchicalChunkingLevelConfiguration>,
    #[serde(rename = "overlapTokens")]
    #[serde(default)]
    pub overlap_tokens: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HierarchicalChunkingLevelConfiguration {
    #[serde(rename = "maxTokens")]
    #[serde(default)]
    pub max_tokens: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SemanticChunkingConfiguration {
    #[serde(rename = "breakpointPercentileThreshold")]
    #[serde(default)]
    pub breakpoint_percentile_threshold: i32,
    #[serde(rename = "bufferSize")]
    #[serde(default)]
    pub buffer_size: i32,
    #[serde(rename = "maxTokens")]
    #[serde(default)]
    pub max_tokens: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContextEnrichmentConfiguration {
    #[serde(rename = "bedrockFoundationModelConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bedrock_foundation_model_configuration:
        Option<BedrockFoundationModelContextEnrichmentConfiguration>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BedrockFoundationModelContextEnrichmentConfiguration {
    #[serde(rename = "enrichmentStrategyConfiguration")]
    #[serde(default)]
    pub enrichment_strategy_configuration: EnrichmentStrategyConfiguration,
    #[serde(rename = "modelArn")]
    #[serde(default)]
    pub model_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnrichmentStrategyConfiguration {
    #[serde(default)]
    pub method: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomTransformationConfiguration {
    #[serde(rename = "intermediateStorage")]
    #[serde(default)]
    pub intermediate_storage: IntermediateStorage,
    #[serde(default)]
    pub transformations: Vec<Transformation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntermediateStorage {
    #[serde(rename = "s3Location")]
    #[serde(default)]
    pub s3_location: S3Location,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Location {
    #[serde(default)]
    pub uri: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Transformation {
    #[serde(rename = "stepToApply")]
    #[serde(default)]
    pub step_to_apply: String,
    #[serde(rename = "transformationFunction")]
    #[serde(default)]
    pub transformation_function: TransformationFunction,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransformationFunction {
    #[serde(rename = "transformationLambdaConfiguration")]
    #[serde(default)]
    pub transformation_lambda_configuration: TransformationLambdaConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransformationLambdaConfiguration {
    #[serde(rename = "lambdaArn")]
    #[serde(default)]
    pub lambda_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParsingConfiguration {
    #[serde(rename = "bedrockDataAutomationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bedrock_data_automation_configuration: Option<BedrockDataAutomationConfiguration>,
    #[serde(rename = "bedrockFoundationModelConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bedrock_foundation_model_configuration: Option<BedrockFoundationModelConfiguration>,
    #[serde(rename = "parsingStrategy")]
    #[serde(default)]
    pub parsing_strategy: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BedrockDataAutomationConfiguration {
    #[serde(rename = "parsingModality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parsing_modality: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BedrockFoundationModelConfiguration {
    #[serde(rename = "modelArn")]
    #[serde(default)]
    pub model_arn: String,
    #[serde(rename = "parsingModality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parsing_modality: Option<String>,
    #[serde(rename = "parsingPrompt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parsing_prompt: Option<ParsingPrompt>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParsingPrompt {
    #[serde(rename = "parsingPromptText")]
    #[serde(default)]
    pub parsing_prompt_text: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDataSourceResponse {
    #[serde(rename = "dataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<DataSource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSource {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "dataDeletionPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_deletion_policy: Option<String>,
    #[serde(rename = "dataSourceConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_configuration: Option<DataSourceConfiguration>,
    #[serde(rename = "dataSourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "failureReasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reasons: Option<Vec<String>>,
    #[serde(rename = "knowledgeBaseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "serverSideEncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_side_encryption_configuration: Option<ServerSideEncryptionConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "vectorIngestionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_ingestion_configuration: Option<VectorIngestionConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFlowAliasRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "concurrencyConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency_configuration: Option<FlowAliasConcurrencyConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "flowIdentifier")]
    #[serde(default)]
    pub flow_identifier: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "routingConfiguration")]
    #[serde(default)]
    pub routing_configuration: Vec<FlowAliasRoutingConfigurationListItem>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlowAliasConcurrencyConfiguration {
    #[serde(rename = "maxConcurrency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_concurrency: Option<i32>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlowAliasRoutingConfigurationListItem {
    #[serde(rename = "flowVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFlowAliasResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "concurrencyConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency_configuration: Option<FlowAliasConcurrencyConfiguration>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "flowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "routingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_configuration: Option<Vec<FlowAliasRoutingConfigurationListItem>>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFlowRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "customerEncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_encryption_key_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<FlowDefinition>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "executionRoleArn")]
    #[serde(default)]
    pub execution_role_arn: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlowDefinition {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<FlowConnection>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<FlowNode>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlowConnection {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<FlowConnectionConfiguration>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub target: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlowConnectionConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional: Option<FlowConditionalConnectionConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<FlowDataConnectionConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlowConditionalConnectionConfiguration {
    #[serde(default)]
    pub condition: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlowDataConnectionConfiguration {
    #[serde(rename = "sourceOutput")]
    #[serde(default)]
    pub source_output: String,
    #[serde(rename = "targetInput")]
    #[serde(default)]
    pub target_input: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlowNode {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<Box<FlowNodeConfiguration>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<FlowNodeInput>>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<FlowNodeOutput>>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlowNodeConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<AgentFlowNodeConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collector: Option<CollectorFlowNodeConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<ConditionFlowNodeConfiguration>,
    #[serde(rename = "inlineCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_code: Option<InlineCodeFlowNodeConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<InputFlowNodeConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterator: Option<IteratorFlowNodeConfiguration>,
    #[serde(rename = "knowledgeBase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base: Option<KnowledgeBaseFlowNodeConfiguration>,
    #[serde(rename = "lambdaFunction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_function: Option<LambdaFunctionFlowNodeConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lex: Option<LexFlowNodeConfiguration>,
    #[serde(rename = "loop")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#loop: Option<Box<LoopFlowNodeConfiguration>>,
    #[serde(rename = "loopController")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loop_controller: Option<LoopControllerFlowNodeConfiguration>,
    #[serde(rename = "loopInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loop_input: Option<LoopInputFlowNodeConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<OutputFlowNodeConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<PromptFlowNodeConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieval: Option<RetrievalFlowNodeConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<StorageFlowNodeConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgentFlowNodeConfiguration {
    #[serde(rename = "agentAliasArn")]
    #[serde(default)]
    pub agent_alias_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CollectorFlowNodeConfiguration {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConditionFlowNodeConfiguration {
    #[serde(default)]
    pub conditions: Vec<FlowCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlowCondition {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InlineCodeFlowNodeConfiguration {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub language: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputFlowNodeConfiguration {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IteratorFlowNodeConfiguration {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KnowledgeBaseFlowNodeConfiguration {
    #[serde(rename = "guardrailConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrail_configuration: Option<GuardrailConfiguration>,
    #[serde(rename = "inferenceConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_configuration: Option<PromptInferenceConfiguration>,
    #[serde(rename = "knowledgeBaseId")]
    #[serde(default)]
    pub knowledge_base_id: String,
    #[serde(rename = "modelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    #[serde(rename = "numberOfResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_results: Option<i32>,
    #[serde(rename = "orchestrationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orchestration_configuration: Option<KnowledgeBaseOrchestrationConfiguration>,
    #[serde(rename = "promptTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_template: Option<KnowledgeBasePromptTemplate>,
    #[serde(rename = "rerankingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reranking_configuration: Option<VectorSearchRerankingConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PromptInferenceConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<PromptModelInferenceConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PromptModelInferenceConfiguration {
    #[serde(rename = "maxTokens")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,
    #[serde(rename = "stopSequences")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(rename = "topP")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KnowledgeBaseOrchestrationConfiguration {
    #[serde(rename = "additionalModelRequestFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_model_request_fields:
        Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "inferenceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_config: Option<PromptInferenceConfiguration>,
    #[serde(rename = "performanceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_config: Option<PerformanceConfiguration>,
    #[serde(rename = "promptTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_template: Option<KnowledgeBasePromptTemplate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PerformanceConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KnowledgeBasePromptTemplate {
    #[serde(rename = "textPromptTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_prompt_template: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VectorSearchRerankingConfiguration {
    #[serde(rename = "bedrockRerankingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bedrock_reranking_configuration: Option<VectorSearchBedrockRerankingConfiguration>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VectorSearchBedrockRerankingConfiguration {
    #[serde(rename = "metadataConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_configuration: Option<MetadataConfigurationForReranking>,
    #[serde(rename = "modelConfiguration")]
    #[serde(default)]
    pub model_configuration: VectorSearchBedrockRerankingModelConfiguration,
    #[serde(rename = "numberOfRerankedResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_reranked_results: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetadataConfigurationForReranking {
    #[serde(rename = "selectionMode")]
    #[serde(default)]
    pub selection_mode: String,
    #[serde(rename = "selectiveModeConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective_mode_configuration: Option<RerankingMetadataSelectiveModeConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RerankingMetadataSelectiveModeConfiguration {
    #[serde(rename = "fieldsToExclude")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields_to_exclude: Option<Vec<FieldForReranking>>,
    #[serde(rename = "fieldsToInclude")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields_to_include: Option<Vec<FieldForReranking>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FieldForReranking {
    #[serde(rename = "fieldName")]
    #[serde(default)]
    pub field_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VectorSearchBedrockRerankingModelConfiguration {
    #[serde(rename = "additionalModelRequestFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_model_request_fields:
        Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "modelArn")]
    #[serde(default)]
    pub model_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LambdaFunctionFlowNodeConfiguration {
    #[serde(rename = "lambdaArn")]
    #[serde(default)]
    pub lambda_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LexFlowNodeConfiguration {
    #[serde(rename = "botAliasArn")]
    #[serde(default)]
    pub bot_alias_arn: String,
    #[serde(rename = "localeId")]
    #[serde(default)]
    pub locale_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoopFlowNodeConfiguration {
    #[serde(default)]
    pub definition: Box<FlowDefinition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoopControllerFlowNodeConfiguration {
    #[serde(rename = "continueCondition")]
    #[serde(default)]
    pub continue_condition: FlowCondition,
    #[serde(rename = "maxIterations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_iterations: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoopInputFlowNodeConfiguration {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutputFlowNodeConfiguration {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PromptFlowNodeConfiguration {
    #[serde(rename = "guardrailConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrail_configuration: Option<GuardrailConfiguration>,
    #[serde(rename = "sourceConfiguration")]
    #[serde(default)]
    pub source_configuration: PromptFlowNodeSourceConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PromptFlowNodeSourceConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline: Option<PromptFlowNodeInlineConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<PromptFlowNodeResourceConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PromptFlowNodeInlineConfiguration {
    #[serde(rename = "additionalModelRequestFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_model_request_fields: Option<serde_json::Value>,
    #[serde(rename = "inferenceConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_configuration: Option<PromptInferenceConfiguration>,
    #[serde(rename = "modelId")]
    #[serde(default)]
    pub model_id: String,
    #[serde(rename = "templateConfiguration")]
    #[serde(default)]
    pub template_configuration: PromptTemplateConfiguration,
    #[serde(rename = "templateType")]
    #[serde(default)]
    pub template_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PromptTemplateConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat: Option<ChatPromptTemplateConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextPromptTemplateConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChatPromptTemplateConfiguration {
    #[serde(rename = "inputVariables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_variables: Option<Vec<PromptInputVariable>>,
    #[serde(default)]
    pub messages: Vec<Message>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<Vec<SystemContentBlock>>,
    #[serde(rename = "toolConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_configuration: Option<ToolConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PromptInputVariable {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Message {
    #[serde(default)]
    pub content: Vec<ContentBlock>,
    #[serde(default)]
    pub role: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContentBlock {
    #[serde(rename = "cachePoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_point: Option<CachePointBlock>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CachePointBlock {
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SystemContentBlock {
    #[serde(rename = "cachePoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_point: Option<CachePointBlock>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ToolConfiguration {
    #[serde(rename = "toolChoice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    #[serde(default)]
    pub tools: Vec<Tool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ToolChoice {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub any: Option<AnyToolChoice>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto: Option<AutoToolChoice>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool: Option<SpecificToolChoice>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AnyToolChoice {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoToolChoice {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SpecificToolChoice {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tool {
    #[serde(rename = "cachePoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_point: Option<CachePointBlock>,
    #[serde(rename = "toolSpec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_spec: Option<ToolSpecification>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ToolSpecification {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "inputSchema")]
    #[serde(default)]
    pub input_schema: ToolInputSchema,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ToolInputSchema {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TextPromptTemplateConfiguration {
    #[serde(rename = "cachePoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_point: Option<CachePointBlock>,
    #[serde(rename = "inputVariables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_variables: Option<Vec<PromptInputVariable>>,
    #[serde(default)]
    pub text: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PromptFlowNodeResourceConfiguration {
    #[serde(rename = "promptArn")]
    #[serde(default)]
    pub prompt_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetrievalFlowNodeConfiguration {
    #[serde(rename = "serviceConfiguration")]
    #[serde(default)]
    pub service_configuration: RetrievalFlowNodeServiceConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetrievalFlowNodeServiceConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3: Option<RetrievalFlowNodeS3Configuration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetrievalFlowNodeS3Configuration {
    #[serde(rename = "bucketName")]
    #[serde(default)]
    pub bucket_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StorageFlowNodeConfiguration {
    #[serde(rename = "serviceConfiguration")]
    #[serde(default)]
    pub service_configuration: StorageFlowNodeServiceConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StorageFlowNodeServiceConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3: Option<StorageFlowNodeS3Configuration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StorageFlowNodeS3Configuration {
    #[serde(rename = "bucketName")]
    #[serde(default)]
    pub bucket_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlowNodeInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default)]
    pub expression: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlowNodeOutput {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFlowResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "customerEncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_encryption_key_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<FlowDefinition>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "executionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFlowVersionRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "flowIdentifier")]
    #[serde(default)]
    pub flow_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFlowVersionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "customerEncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_encryption_key_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<FlowDefinition>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "executionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateKnowledgeBaseRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "knowledgeBaseConfiguration")]
    #[serde(default)]
    pub knowledge_base_configuration: KnowledgeBaseConfiguration,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "storageConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_configuration: Option<StorageConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KnowledgeBaseConfiguration {
    #[serde(rename = "kendraKnowledgeBaseConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kendra_knowledge_base_configuration: Option<KendraKnowledgeBaseConfiguration>,
    #[serde(rename = "sqlKnowledgeBaseConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql_knowledge_base_configuration: Option<SqlKnowledgeBaseConfiguration>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "vectorKnowledgeBaseConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_knowledge_base_configuration: Option<VectorKnowledgeBaseConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KendraKnowledgeBaseConfiguration {
    #[serde(rename = "kendraIndexArn")]
    #[serde(default)]
    pub kendra_index_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SqlKnowledgeBaseConfiguration {
    #[serde(rename = "redshiftConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_configuration: Option<RedshiftConfiguration>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RedshiftConfiguration {
    #[serde(rename = "queryEngineConfiguration")]
    #[serde(default)]
    pub query_engine_configuration: RedshiftQueryEngineConfiguration,
    #[serde(rename = "queryGenerationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_generation_configuration: Option<QueryGenerationConfiguration>,
    #[serde(rename = "storageConfigurations")]
    #[serde(default)]
    pub storage_configurations: Vec<RedshiftQueryEngineStorageConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RedshiftQueryEngineConfiguration {
    #[serde(rename = "provisionedConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_configuration: Option<RedshiftProvisionedConfiguration>,
    #[serde(rename = "serverlessConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless_configuration: Option<RedshiftServerlessConfiguration>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RedshiftProvisionedConfiguration {
    #[serde(rename = "authConfiguration")]
    #[serde(default)]
    pub auth_configuration: RedshiftProvisionedAuthConfiguration,
    #[serde(rename = "clusterIdentifier")]
    #[serde(default)]
    pub cluster_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RedshiftProvisionedAuthConfiguration {
    #[serde(rename = "databaseUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_user: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "usernamePasswordSecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username_password_secret_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RedshiftServerlessConfiguration {
    #[serde(rename = "authConfiguration")]
    #[serde(default)]
    pub auth_configuration: RedshiftServerlessAuthConfiguration,
    #[serde(rename = "workgroupArn")]
    #[serde(default)]
    pub workgroup_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RedshiftServerlessAuthConfiguration {
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "usernamePasswordSecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username_password_secret_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryGenerationConfiguration {
    #[serde(rename = "executionTimeoutSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_timeout_seconds: Option<i32>,
    #[serde(rename = "generationContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_context: Option<QueryGenerationContext>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryGenerationContext {
    #[serde(rename = "curatedQueries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curated_queries: Option<Vec<CuratedQuery>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables: Option<Vec<QueryGenerationTable>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CuratedQuery {
    #[serde(rename = "naturalLanguage")]
    #[serde(default)]
    pub natural_language: String,
    #[serde(default)]
    pub sql: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryGenerationTable {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<QueryGenerationColumn>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusion: Option<String>,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryGenerationColumn {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inclusion: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RedshiftQueryEngineStorageConfiguration {
    #[serde(rename = "awsDataCatalogConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_data_catalog_configuration:
        Option<RedshiftQueryEngineAwsDataCatalogStorageConfiguration>,
    #[serde(rename = "redshiftConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_configuration: Option<RedshiftQueryEngineRedshiftStorageConfiguration>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RedshiftQueryEngineAwsDataCatalogStorageConfiguration {
    #[serde(rename = "tableNames")]
    #[serde(default)]
    pub table_names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RedshiftQueryEngineRedshiftStorageConfiguration {
    #[serde(rename = "databaseName")]
    #[serde(default)]
    pub database_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VectorKnowledgeBaseConfiguration {
    #[serde(rename = "embeddingModelArn")]
    #[serde(default)]
    pub embedding_model_arn: String,
    #[serde(rename = "embeddingModelConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_model_configuration: Option<EmbeddingModelConfiguration>,
    #[serde(rename = "supplementalDataStorageConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplemental_data_storage_configuration: Option<SupplementalDataStorageConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EmbeddingModelConfiguration {
    #[serde(rename = "bedrockEmbeddingModelConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bedrock_embedding_model_configuration: Option<BedrockEmbeddingModelConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BedrockEmbeddingModelConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Vec<AudioConfiguration>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<i32>,
    #[serde(rename = "embeddingDataType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_data_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Vec<VideoConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AudioConfiguration {
    #[serde(rename = "segmentationConfiguration")]
    #[serde(default)]
    pub segmentation_configuration: AudioSegmentationConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AudioSegmentationConfiguration {
    #[serde(rename = "fixedLengthDuration")]
    #[serde(default)]
    pub fixed_length_duration: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VideoConfiguration {
    #[serde(rename = "segmentationConfiguration")]
    #[serde(default)]
    pub segmentation_configuration: VideoSegmentationConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VideoSegmentationConfiguration {
    #[serde(rename = "fixedLengthDuration")]
    #[serde(default)]
    pub fixed_length_duration: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SupplementalDataStorageConfiguration {
    #[serde(rename = "storageLocations")]
    #[serde(default)]
    pub storage_locations: Vec<SupplementalDataStorageLocation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SupplementalDataStorageLocation {
    #[serde(rename = "s3Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_location: Option<S3Location>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StorageConfiguration {
    #[serde(rename = "mongoDbAtlasConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mongo_db_atlas_configuration: Option<MongoDbAtlasConfiguration>,
    #[serde(rename = "neptuneAnalyticsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neptune_analytics_configuration: Option<NeptuneAnalyticsConfiguration>,
    #[serde(rename = "opensearchManagedClusterConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opensearch_managed_cluster_configuration: Option<OpenSearchManagedClusterConfiguration>,
    #[serde(rename = "opensearchServerlessConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opensearch_serverless_configuration: Option<OpenSearchServerlessConfiguration>,
    #[serde(rename = "pineconeConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinecone_configuration: Option<PineconeConfiguration>,
    #[serde(rename = "rdsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_configuration: Option<RdsConfiguration>,
    #[serde(rename = "redisEnterpriseCloudConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis_enterprise_cloud_configuration: Option<RedisEnterpriseCloudConfiguration>,
    #[serde(rename = "s3VectorsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_vectors_configuration: Option<S3VectorsConfiguration>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MongoDbAtlasConfiguration {
    #[serde(rename = "collectionName")]
    #[serde(default)]
    pub collection_name: String,
    #[serde(rename = "credentialsSecretArn")]
    #[serde(default)]
    pub credentials_secret_arn: String,
    #[serde(rename = "databaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(default)]
    pub endpoint: String,
    #[serde(rename = "endpointServiceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_service_name: Option<String>,
    #[serde(rename = "fieldMapping")]
    #[serde(default)]
    pub field_mapping: MongoDbAtlasFieldMapping,
    #[serde(rename = "textIndexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_index_name: Option<String>,
    #[serde(rename = "vectorIndexName")]
    #[serde(default)]
    pub vector_index_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MongoDbAtlasFieldMapping {
    #[serde(rename = "metadataField")]
    #[serde(default)]
    pub metadata_field: String,
    #[serde(rename = "textField")]
    #[serde(default)]
    pub text_field: String,
    #[serde(rename = "vectorField")]
    #[serde(default)]
    pub vector_field: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NeptuneAnalyticsConfiguration {
    #[serde(rename = "fieldMapping")]
    #[serde(default)]
    pub field_mapping: NeptuneAnalyticsFieldMapping,
    #[serde(rename = "graphArn")]
    #[serde(default)]
    pub graph_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NeptuneAnalyticsFieldMapping {
    #[serde(rename = "metadataField")]
    #[serde(default)]
    pub metadata_field: String,
    #[serde(rename = "textField")]
    #[serde(default)]
    pub text_field: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpenSearchManagedClusterConfiguration {
    #[serde(rename = "domainArn")]
    #[serde(default)]
    pub domain_arn: String,
    #[serde(rename = "domainEndpoint")]
    #[serde(default)]
    pub domain_endpoint: String,
    #[serde(rename = "fieldMapping")]
    #[serde(default)]
    pub field_mapping: OpenSearchManagedClusterFieldMapping,
    #[serde(rename = "vectorIndexName")]
    #[serde(default)]
    pub vector_index_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpenSearchManagedClusterFieldMapping {
    #[serde(rename = "metadataField")]
    #[serde(default)]
    pub metadata_field: String,
    #[serde(rename = "textField")]
    #[serde(default)]
    pub text_field: String,
    #[serde(rename = "vectorField")]
    #[serde(default)]
    pub vector_field: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpenSearchServerlessConfiguration {
    #[serde(rename = "collectionArn")]
    #[serde(default)]
    pub collection_arn: String,
    #[serde(rename = "fieldMapping")]
    #[serde(default)]
    pub field_mapping: OpenSearchServerlessFieldMapping,
    #[serde(rename = "vectorIndexName")]
    #[serde(default)]
    pub vector_index_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpenSearchServerlessFieldMapping {
    #[serde(rename = "metadataField")]
    #[serde(default)]
    pub metadata_field: String,
    #[serde(rename = "textField")]
    #[serde(default)]
    pub text_field: String,
    #[serde(rename = "vectorField")]
    #[serde(default)]
    pub vector_field: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PineconeConfiguration {
    #[serde(rename = "connectionString")]
    #[serde(default)]
    pub connection_string: String,
    #[serde(rename = "credentialsSecretArn")]
    #[serde(default)]
    pub credentials_secret_arn: String,
    #[serde(rename = "fieldMapping")]
    #[serde(default)]
    pub field_mapping: PineconeFieldMapping,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PineconeFieldMapping {
    #[serde(rename = "metadataField")]
    #[serde(default)]
    pub metadata_field: String,
    #[serde(rename = "textField")]
    #[serde(default)]
    pub text_field: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RdsConfiguration {
    #[serde(rename = "credentialsSecretArn")]
    #[serde(default)]
    pub credentials_secret_arn: String,
    #[serde(rename = "databaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "fieldMapping")]
    #[serde(default)]
    pub field_mapping: RdsFieldMapping,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "tableName")]
    #[serde(default)]
    pub table_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RdsFieldMapping {
    #[serde(rename = "customMetadataField")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_metadata_field: Option<String>,
    #[serde(rename = "metadataField")]
    #[serde(default)]
    pub metadata_field: String,
    #[serde(rename = "primaryKeyField")]
    #[serde(default)]
    pub primary_key_field: String,
    #[serde(rename = "textField")]
    #[serde(default)]
    pub text_field: String,
    #[serde(rename = "vectorField")]
    #[serde(default)]
    pub vector_field: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RedisEnterpriseCloudConfiguration {
    #[serde(rename = "credentialsSecretArn")]
    #[serde(default)]
    pub credentials_secret_arn: String,
    #[serde(default)]
    pub endpoint: String,
    #[serde(rename = "fieldMapping")]
    #[serde(default)]
    pub field_mapping: RedisEnterpriseCloudFieldMapping,
    #[serde(rename = "vectorIndexName")]
    #[serde(default)]
    pub vector_index_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RedisEnterpriseCloudFieldMapping {
    #[serde(rename = "metadataField")]
    #[serde(default)]
    pub metadata_field: String,
    #[serde(rename = "textField")]
    #[serde(default)]
    pub text_field: String,
    #[serde(rename = "vectorField")]
    #[serde(default)]
    pub vector_field: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3VectorsConfiguration {
    #[serde(rename = "indexArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_arn: Option<String>,
    #[serde(rename = "indexName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[serde(rename = "vectorBucketArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_bucket_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateKnowledgeBaseResponse {
    #[serde(rename = "knowledgeBase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base: Option<KnowledgeBase>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KnowledgeBase {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "failureReasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reasons: Option<Vec<String>>,
    #[serde(rename = "knowledgeBaseArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base_arn: Option<String>,
    #[serde(rename = "knowledgeBaseConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base_configuration: Option<KnowledgeBaseConfiguration>,
    #[serde(rename = "knowledgeBaseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "storageConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_configuration: Option<StorageConfiguration>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePromptRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "customerEncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_encryption_key_arn: Option<String>,
    #[serde(rename = "defaultVariant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_variant: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<PromptVariant>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PromptVariant {
    #[serde(rename = "additionalModelRequestFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_model_request_fields: Option<serde_json::Value>,
    #[serde(rename = "genAiResource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gen_ai_resource: Option<PromptGenAiResource>,
    #[serde(rename = "inferenceConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inference_configuration: Option<PromptInferenceConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Vec<PromptMetadataEntry>>,
    #[serde(rename = "modelId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "templateConfiguration")]
    #[serde(default)]
    pub template_configuration: PromptTemplateConfiguration,
    #[serde(rename = "templateType")]
    #[serde(default)]
    pub template_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PromptGenAiResource {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<PromptAgentResource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PromptAgentResource {
    #[serde(rename = "agentIdentifier")]
    #[serde(default)]
    pub agent_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PromptMetadataEntry {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePromptResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "customerEncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_encryption_key_arn: Option<String>,
    #[serde(rename = "defaultVariant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_variant: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<PromptVariant>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePromptVersionRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "promptIdentifier")]
    #[serde(default)]
    pub prompt_identifier: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePromptVersionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "customerEncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_encryption_key_arn: Option<String>,
    #[serde(rename = "defaultVariant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_variant: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<PromptVariant>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAgentActionGroupRequest {
    #[serde(rename = "actionGroupId")]
    #[serde(default)]
    pub action_group_id: String,
    #[serde(rename = "agentId")]
    #[serde(default)]
    pub agent_id: String,
    #[serde(rename = "agentVersion")]
    #[serde(default)]
    pub agent_version: String,
    #[serde(rename = "skipResourceInUseCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_resource_in_use_check: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAgentActionGroupResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAgentAliasRequest {
    #[serde(rename = "agentAliasId")]
    #[serde(default)]
    pub agent_alias_id: String,
    #[serde(rename = "agentId")]
    #[serde(default)]
    pub agent_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAgentAliasResponse {
    #[serde(rename = "agentAliasId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_alias_id: Option<String>,
    #[serde(rename = "agentAliasStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_alias_status: Option<String>,
    #[serde(rename = "agentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAgentRequest {
    #[serde(rename = "agentId")]
    #[serde(default)]
    pub agent_id: String,
    #[serde(rename = "skipResourceInUseCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_resource_in_use_check: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAgentResponse {
    #[serde(rename = "agentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(rename = "agentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAgentVersionRequest {
    #[serde(rename = "agentId")]
    #[serde(default)]
    pub agent_id: String,
    #[serde(rename = "agentVersion")]
    #[serde(default)]
    pub agent_version: String,
    #[serde(rename = "skipResourceInUseCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_resource_in_use_check: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAgentVersionResponse {
    #[serde(rename = "agentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(rename = "agentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_status: Option<String>,
    #[serde(rename = "agentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDataSourceRequest {
    #[serde(rename = "dataSourceId")]
    #[serde(default)]
    pub data_source_id: String,
    #[serde(rename = "knowledgeBaseId")]
    #[serde(default)]
    pub knowledge_base_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDataSourceResponse {
    #[serde(rename = "dataSourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,
    #[serde(rename = "knowledgeBaseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFlowAliasRequest {
    #[serde(rename = "aliasIdentifier")]
    #[serde(default)]
    pub alias_identifier: String,
    #[serde(rename = "flowIdentifier")]
    #[serde(default)]
    pub flow_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFlowAliasResponse {
    #[serde(rename = "flowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFlowRequest {
    #[serde(rename = "flowIdentifier")]
    #[serde(default)]
    pub flow_identifier: String,
    #[serde(rename = "skipResourceInUseCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_resource_in_use_check: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFlowResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFlowVersionRequest {
    #[serde(rename = "flowIdentifier")]
    #[serde(default)]
    pub flow_identifier: String,
    #[serde(rename = "flowVersion")]
    #[serde(default)]
    pub flow_version: String,
    #[serde(rename = "skipResourceInUseCheck")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_resource_in_use_check: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFlowVersionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteKnowledgeBaseDocumentsRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "dataSourceId")]
    #[serde(default)]
    pub data_source_id: String,
    #[serde(rename = "documentIdentifiers")]
    #[serde(default)]
    pub document_identifiers: Vec<DocumentIdentifier>,
    #[serde(rename = "knowledgeBaseId")]
    #[serde(default)]
    pub knowledge_base_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentIdentifier {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<CustomDocumentIdentifier>,
    #[serde(rename = "dataSourceType")]
    #[serde(default)]
    pub data_source_type: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3: Option<S3Location>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomDocumentIdentifier {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteKnowledgeBaseDocumentsResponse {
    #[serde(rename = "documentDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_details: Option<Vec<KnowledgeBaseDocumentDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KnowledgeBaseDocumentDetail {
    #[serde(rename = "dataSourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<DocumentIdentifier>,
    #[serde(rename = "knowledgeBaseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteKnowledgeBaseRequest {
    #[serde(rename = "knowledgeBaseId")]
    #[serde(default)]
    pub knowledge_base_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteKnowledgeBaseResponse {
    #[serde(rename = "knowledgeBaseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePromptRequest {
    #[serde(rename = "promptIdentifier")]
    #[serde(default)]
    pub prompt_identifier: String,
    #[serde(rename = "promptVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePromptResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateAgentCollaboratorRequest {
    #[serde(rename = "agentId")]
    #[serde(default)]
    pub agent_id: String,
    #[serde(rename = "agentVersion")]
    #[serde(default)]
    pub agent_version: String,
    #[serde(rename = "collaboratorId")]
    #[serde(default)]
    pub collaborator_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateAgentCollaboratorResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateAgentKnowledgeBaseRequest {
    #[serde(rename = "agentId")]
    #[serde(default)]
    pub agent_id: String,
    #[serde(rename = "agentVersion")]
    #[serde(default)]
    pub agent_version: String,
    #[serde(rename = "knowledgeBaseId")]
    #[serde(default)]
    pub knowledge_base_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateAgentKnowledgeBaseResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAgentActionGroupRequest {
    #[serde(rename = "actionGroupId")]
    #[serde(default)]
    pub action_group_id: String,
    #[serde(rename = "agentId")]
    #[serde(default)]
    pub agent_id: String,
    #[serde(rename = "agentVersion")]
    #[serde(default)]
    pub agent_version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAgentActionGroupResponse {
    #[serde(rename = "agentActionGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_action_group: Option<AgentActionGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAgentAliasRequest {
    #[serde(rename = "agentAliasId")]
    #[serde(default)]
    pub agent_alias_id: String,
    #[serde(rename = "agentId")]
    #[serde(default)]
    pub agent_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAgentAliasResponse {
    #[serde(rename = "agentAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_alias: Option<AgentAlias>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAgentCollaboratorRequest {
    #[serde(rename = "agentId")]
    #[serde(default)]
    pub agent_id: String,
    #[serde(rename = "agentVersion")]
    #[serde(default)]
    pub agent_version: String,
    #[serde(rename = "collaboratorId")]
    #[serde(default)]
    pub collaborator_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAgentCollaboratorResponse {
    #[serde(rename = "agentCollaborator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_collaborator: Option<AgentCollaborator>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAgentKnowledgeBaseRequest {
    #[serde(rename = "agentId")]
    #[serde(default)]
    pub agent_id: String,
    #[serde(rename = "agentVersion")]
    #[serde(default)]
    pub agent_version: String,
    #[serde(rename = "knowledgeBaseId")]
    #[serde(default)]
    pub knowledge_base_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAgentKnowledgeBaseResponse {
    #[serde(rename = "agentKnowledgeBase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_knowledge_base: Option<AgentKnowledgeBase>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAgentRequest {
    #[serde(rename = "agentId")]
    #[serde(default)]
    pub agent_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAgentResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<Agent>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAgentVersionRequest {
    #[serde(rename = "agentId")]
    #[serde(default)]
    pub agent_id: String,
    #[serde(rename = "agentVersion")]
    #[serde(default)]
    pub agent_version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAgentVersionResponse {
    #[serde(rename = "agentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<AgentVersion>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgentVersion {
    #[serde(rename = "agentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_arn: Option<String>,
    #[serde(rename = "agentCollaboration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_collaboration: Option<String>,
    #[serde(rename = "agentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(rename = "agentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<String>,
    #[serde(rename = "agentResourceRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_resource_role_arn: Option<String>,
    #[serde(rename = "agentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_status: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "customerEncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_encryption_key_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "failureReasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reasons: Option<Vec<String>>,
    #[serde(rename = "foundationModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foundation_model: Option<String>,
    #[serde(rename = "guardrailConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrail_configuration: Option<GuardrailConfiguration>,
    #[serde(rename = "idleSessionTTLInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_session_t_t_l_in_seconds: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instruction: Option<String>,
    #[serde(rename = "memoryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_configuration: Option<MemoryConfiguration>,
    #[serde(rename = "promptOverrideConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_override_configuration: Option<PromptOverrideConfiguration>,
    #[serde(rename = "recommendedActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_actions: Option<Vec<String>>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataSourceRequest {
    #[serde(rename = "dataSourceId")]
    #[serde(default)]
    pub data_source_id: String,
    #[serde(rename = "knowledgeBaseId")]
    #[serde(default)]
    pub knowledge_base_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataSourceResponse {
    #[serde(rename = "dataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<DataSource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFlowAliasRequest {
    #[serde(rename = "aliasIdentifier")]
    #[serde(default)]
    pub alias_identifier: String,
    #[serde(rename = "flowIdentifier")]
    #[serde(default)]
    pub flow_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFlowAliasResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "concurrencyConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency_configuration: Option<FlowAliasConcurrencyConfiguration>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "flowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "routingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_configuration: Option<Vec<FlowAliasRoutingConfigurationListItem>>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFlowRequest {
    #[serde(rename = "flowIdentifier")]
    #[serde(default)]
    pub flow_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFlowResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "customerEncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_encryption_key_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<FlowDefinition>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "executionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validations: Option<Vec<FlowValidation>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlowValidation {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<FlowValidationDetails>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlowValidationDetails {
    #[serde(rename = "cyclicConnection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cyclic_connection: Option<CyclicConnectionFlowValidationDetails>,
    #[serde(rename = "duplicateConditionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_condition_expression: Option<DuplicateConditionExpressionFlowValidationDetails>,
    #[serde(rename = "duplicateConnections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_connections: Option<DuplicateConnectionsFlowValidationDetails>,
    #[serde(rename = "incompatibleConnectionDataType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incompatible_connection_data_type:
        Option<IncompatibleConnectionDataTypeFlowValidationDetails>,
    #[serde(rename = "invalidLoopBoundary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invalid_loop_boundary: Option<InvalidLoopBoundaryFlowValidationDetails>,
    #[serde(rename = "loopIncompatibleNodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loop_incompatible_node_type: Option<LoopIncompatibleNodeTypeFlowValidationDetails>,
    #[serde(rename = "malformedConditionExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malformed_condition_expression: Option<MalformedConditionExpressionFlowValidationDetails>,
    #[serde(rename = "malformedNodeInputExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malformed_node_input_expression: Option<MalformedNodeInputExpressionFlowValidationDetails>,
    #[serde(rename = "mismatchedNodeInputType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mismatched_node_input_type: Option<MismatchedNodeInputTypeFlowValidationDetails>,
    #[serde(rename = "mismatchedNodeOutputType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mismatched_node_output_type: Option<MismatchedNodeOutputTypeFlowValidationDetails>,
    #[serde(rename = "missingConnectionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_connection_configuration:
        Option<MissingConnectionConfigurationFlowValidationDetails>,
    #[serde(rename = "missingDefaultCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_default_condition: Option<MissingDefaultConditionFlowValidationDetails>,
    #[serde(rename = "missingEndingNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_ending_nodes: Option<MissingEndingNodesFlowValidationDetails>,
    #[serde(rename = "missingLoopControllerNode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_loop_controller_node: Option<MissingLoopControllerNodeFlowValidationDetails>,
    #[serde(rename = "missingLoopInputNode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_loop_input_node: Option<MissingLoopInputNodeFlowValidationDetails>,
    #[serde(rename = "missingNodeConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_node_configuration: Option<MissingNodeConfigurationFlowValidationDetails>,
    #[serde(rename = "missingNodeInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_node_input: Option<MissingNodeInputFlowValidationDetails>,
    #[serde(rename = "missingNodeOutput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_node_output: Option<MissingNodeOutputFlowValidationDetails>,
    #[serde(rename = "missingStartingNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_starting_nodes: Option<MissingStartingNodesFlowValidationDetails>,
    #[serde(rename = "multipleLoopControllerNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_loop_controller_nodes: Option<MultipleLoopControllerNodesFlowValidationDetails>,
    #[serde(rename = "multipleLoopInputNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_loop_input_nodes: Option<MultipleLoopInputNodesFlowValidationDetails>,
    #[serde(rename = "multipleNodeInputConnections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_node_input_connections: Option<MultipleNodeInputConnectionsFlowValidationDetails>,
    #[serde(rename = "unfulfilledNodeInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unfulfilled_node_input: Option<UnfulfilledNodeInputFlowValidationDetails>,
    #[serde(rename = "unknownConnectionCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_connection_condition: Option<UnknownConnectionConditionFlowValidationDetails>,
    #[serde(rename = "unknownConnectionSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_connection_source: Option<UnknownConnectionSourceFlowValidationDetails>,
    #[serde(rename = "unknownConnectionSourceOutput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_connection_source_output:
        Option<UnknownConnectionSourceOutputFlowValidationDetails>,
    #[serde(rename = "unknownConnectionTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_connection_target: Option<UnknownConnectionTargetFlowValidationDetails>,
    #[serde(rename = "unknownConnectionTargetInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_connection_target_input: Option<UnknownConnectionTargetInputFlowValidationDetails>,
    #[serde(rename = "unknownNodeInput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_node_input: Option<UnknownNodeInputFlowValidationDetails>,
    #[serde(rename = "unknownNodeOutput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unknown_node_output: Option<UnknownNodeOutputFlowValidationDetails>,
    #[serde(rename = "unreachableNode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unreachable_node: Option<UnreachableNodeFlowValidationDetails>,
    #[serde(rename = "unsatisfiedConnectionConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsatisfied_connection_conditions:
        Option<UnsatisfiedConnectionConditionsFlowValidationDetails>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unspecified: Option<UnspecifiedFlowValidationDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CyclicConnectionFlowValidationDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DuplicateConditionExpressionFlowValidationDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DuplicateConnectionsFlowValidationDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IncompatibleConnectionDataTypeFlowValidationDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InvalidLoopBoundaryFlowValidationDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LoopIncompatibleNodeTypeFlowValidationDetails {
    #[serde(rename = "incompatibleNodeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incompatible_node_name: Option<String>,
    #[serde(rename = "incompatibleNodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incompatible_node_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MalformedConditionExpressionFlowValidationDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MalformedNodeInputExpressionFlowValidationDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MismatchedNodeInputTypeFlowValidationDetails {
    #[serde(rename = "expectedType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MismatchedNodeOutputTypeFlowValidationDetails {
    #[serde(rename = "expectedType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MissingConnectionConfigurationFlowValidationDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MissingDefaultConditionFlowValidationDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MissingEndingNodesFlowValidationDetails {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MissingLoopControllerNodeFlowValidationDetails {
    #[serde(rename = "loopNode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loop_node: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MissingLoopInputNodeFlowValidationDetails {
    #[serde(rename = "loopNode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loop_node: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MissingNodeConfigurationFlowValidationDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MissingNodeInputFlowValidationDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MissingNodeOutputFlowValidationDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MissingStartingNodesFlowValidationDetails {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultipleLoopControllerNodesFlowValidationDetails {
    #[serde(rename = "loopNode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loop_node: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultipleLoopInputNodesFlowValidationDetails {
    #[serde(rename = "loopNode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loop_node: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MultipleNodeInputConnectionsFlowValidationDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnfulfilledNodeInputFlowValidationDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnknownConnectionConditionFlowValidationDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnknownConnectionSourceFlowValidationDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnknownConnectionSourceOutputFlowValidationDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnknownConnectionTargetFlowValidationDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnknownConnectionTargetInputFlowValidationDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnknownNodeInputFlowValidationDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnknownNodeOutputFlowValidationDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnreachableNodeFlowValidationDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnsatisfiedConnectionConditionsFlowValidationDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UnspecifiedFlowValidationDetails {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFlowVersionRequest {
    #[serde(rename = "flowIdentifier")]
    #[serde(default)]
    pub flow_identifier: String,
    #[serde(rename = "flowVersion")]
    #[serde(default)]
    pub flow_version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFlowVersionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "customerEncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_encryption_key_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<FlowDefinition>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "executionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIngestionJobRequest {
    #[serde(rename = "dataSourceId")]
    #[serde(default)]
    pub data_source_id: String,
    #[serde(rename = "ingestionJobId")]
    #[serde(default)]
    pub ingestion_job_id: String,
    #[serde(rename = "knowledgeBaseId")]
    #[serde(default)]
    pub knowledge_base_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIngestionJobResponse {
    #[serde(rename = "ingestionJob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_job: Option<IngestionJob>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IngestionJob {
    #[serde(rename = "dataSourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "failureReasons")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reasons: Option<Vec<String>>,
    #[serde(rename = "ingestionJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_job_id: Option<String>,
    #[serde(rename = "knowledgeBaseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base_id: Option<String>,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<IngestionJobStatistics>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IngestionJobStatistics {
    #[serde(rename = "numberOfDocumentsDeleted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_documents_deleted: Option<i64>,
    #[serde(rename = "numberOfDocumentsFailed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_documents_failed: Option<i64>,
    #[serde(rename = "numberOfDocumentsScanned")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_documents_scanned: Option<i64>,
    #[serde(rename = "numberOfMetadataDocumentsModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_metadata_documents_modified: Option<i64>,
    #[serde(rename = "numberOfMetadataDocumentsScanned")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_metadata_documents_scanned: Option<i64>,
    #[serde(rename = "numberOfModifiedDocumentsIndexed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_modified_documents_indexed: Option<i64>,
    #[serde(rename = "numberOfNewDocumentsIndexed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_new_documents_indexed: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetKnowledgeBaseDocumentsRequest {
    #[serde(rename = "dataSourceId")]
    #[serde(default)]
    pub data_source_id: String,
    #[serde(rename = "documentIdentifiers")]
    #[serde(default)]
    pub document_identifiers: Vec<DocumentIdentifier>,
    #[serde(rename = "knowledgeBaseId")]
    #[serde(default)]
    pub knowledge_base_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetKnowledgeBaseDocumentsResponse {
    #[serde(rename = "documentDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_details: Option<Vec<KnowledgeBaseDocumentDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetKnowledgeBaseRequest {
    #[serde(rename = "knowledgeBaseId")]
    #[serde(default)]
    pub knowledge_base_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetKnowledgeBaseResponse {
    #[serde(rename = "knowledgeBase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base: Option<KnowledgeBase>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPromptRequest {
    #[serde(rename = "promptIdentifier")]
    #[serde(default)]
    pub prompt_identifier: String,
    #[serde(rename = "promptVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPromptResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "customerEncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_encryption_key_arn: Option<String>,
    #[serde(rename = "defaultVariant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_variant: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<PromptVariant>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IngestKnowledgeBaseDocumentsRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "dataSourceId")]
    #[serde(default)]
    pub data_source_id: String,
    #[serde(default)]
    pub documents: Vec<KnowledgeBaseDocument>,
    #[serde(rename = "knowledgeBaseId")]
    #[serde(default)]
    pub knowledge_base_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KnowledgeBaseDocument {
    #[serde(default)]
    pub content: DocumentContent,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<DocumentMetadata>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentContent {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<CustomContent>,
    #[serde(rename = "dataSourceType")]
    #[serde(default)]
    pub data_source_type: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3: Option<S3Content>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomContent {
    #[serde(rename = "customDocumentIdentifier")]
    #[serde(default)]
    pub custom_document_identifier: CustomDocumentIdentifier,
    #[serde(rename = "inlineContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_content: Option<InlineContent>,
    #[serde(rename = "s3Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_location: Option<CustomS3Location>,
    #[serde(rename = "sourceType")]
    #[serde(default)]
    pub source_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InlineContent {
    #[serde(rename = "byteContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byte_content: Option<ByteContentDoc>,
    #[serde(rename = "textContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_content: Option<TextContentDoc>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ByteContentDoc {
    #[serde(default)]
    pub data: String,
    #[serde(rename = "mimeType")]
    #[serde(default)]
    pub mime_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TextContentDoc {
    #[serde(default)]
    pub data: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomS3Location {
    #[serde(rename = "bucketOwnerAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_owner_account_id: Option<String>,
    #[serde(default)]
    pub uri: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Content {
    #[serde(rename = "s3Location")]
    #[serde(default)]
    pub s3_location: S3Location,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DocumentMetadata {
    #[serde(rename = "inlineAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_attributes: Option<Vec<MetadataAttribute>>,
    #[serde(rename = "s3Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_location: Option<CustomS3Location>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetadataAttribute {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub value: MetadataAttributeValue,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MetadataAttributeValue {
    #[serde(rename = "booleanValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boolean_value: Option<bool>,
    #[serde(rename = "numberValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_value: Option<f64>,
    #[serde(rename = "stringListValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_list_value: Option<Vec<String>>,
    #[serde(rename = "stringValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IngestKnowledgeBaseDocumentsResponse {
    #[serde(rename = "documentDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_details: Option<Vec<KnowledgeBaseDocumentDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAgentActionGroupsRequest {
    #[serde(rename = "agentId")]
    #[serde(default)]
    pub agent_id: String,
    #[serde(rename = "agentVersion")]
    #[serde(default)]
    pub agent_version: String,
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
pub struct ListAgentActionGroupsResponse {
    #[serde(rename = "actionGroupSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_group_summaries: Option<Vec<ActionGroupSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionGroupSummary {
    #[serde(rename = "actionGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_group_id: Option<String>,
    #[serde(rename = "actionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_group_name: Option<String>,
    #[serde(rename = "actionGroupState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_group_state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAgentAliasesRequest {
    #[serde(rename = "agentId")]
    #[serde(default)]
    pub agent_id: String,
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
pub struct ListAgentAliasesResponse {
    #[serde(rename = "agentAliasSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_alias_summaries: Option<Vec<AgentAliasSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgentAliasSummary {
    #[serde(rename = "agentAliasId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_alias_id: Option<String>,
    #[serde(rename = "agentAliasName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_alias_name: Option<String>,
    #[serde(rename = "agentAliasStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_alias_status: Option<String>,
    #[serde(rename = "aliasInvocationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_invocation_state: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "routingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_configuration: Option<Vec<AgentAliasRoutingConfigurationListItem>>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAgentCollaboratorsRequest {
    #[serde(rename = "agentId")]
    #[serde(default)]
    pub agent_id: String,
    #[serde(rename = "agentVersion")]
    #[serde(default)]
    pub agent_version: String,
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
pub struct ListAgentCollaboratorsResponse {
    #[serde(rename = "agentCollaboratorSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_collaborator_summaries: Option<Vec<AgentCollaboratorSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgentCollaboratorSummary {
    #[serde(rename = "agentDescriptor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_descriptor: Option<AgentDescriptor>,
    #[serde(rename = "agentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(rename = "agentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[serde(rename = "collaborationInstruction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collaboration_instruction: Option<String>,
    #[serde(rename = "collaboratorId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collaborator_id: Option<String>,
    #[serde(rename = "collaboratorName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collaborator_name: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    #[serde(rename = "relayConversationHistory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relay_conversation_history: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAgentKnowledgeBasesRequest {
    #[serde(rename = "agentId")]
    #[serde(default)]
    pub agent_id: String,
    #[serde(rename = "agentVersion")]
    #[serde(default)]
    pub agent_version: String,
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
pub struct ListAgentKnowledgeBasesResponse {
    #[serde(rename = "agentKnowledgeBaseSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_knowledge_base_summaries: Option<Vec<AgentKnowledgeBaseSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgentKnowledgeBaseSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "knowledgeBaseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base_id: Option<String>,
    #[serde(rename = "knowledgeBaseState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base_state: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAgentVersionsRequest {
    #[serde(rename = "agentId")]
    #[serde(default)]
    pub agent_id: String,
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
pub struct ListAgentVersionsResponse {
    #[serde(rename = "agentVersionSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version_summaries: Option<Vec<AgentVersionSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgentVersionSummary {
    #[serde(rename = "agentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<String>,
    #[serde(rename = "agentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_status: Option<String>,
    #[serde(rename = "agentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "guardrailConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrail_configuration: Option<GuardrailConfiguration>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAgentsRequest {
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
pub struct ListAgentsResponse {
    #[serde(rename = "agentSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_summaries: Option<Vec<AgentSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AgentSummary {
    #[serde(rename = "agentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(rename = "agentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<String>,
    #[serde(rename = "agentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "guardrailConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrail_configuration: Option<GuardrailConfiguration>,
    #[serde(rename = "latestAgentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_agent_version: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDataSourcesRequest {
    #[serde(rename = "knowledgeBaseId")]
    #[serde(default)]
    pub knowledge_base_id: String,
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
pub struct ListDataSourcesResponse {
    #[serde(rename = "dataSourceSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_summaries: Option<Vec<DataSourceSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSourceSummary {
    #[serde(rename = "dataSourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "knowledgeBaseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFlowAliasesRequest {
    #[serde(rename = "flowIdentifier")]
    #[serde(default)]
    pub flow_identifier: String,
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
pub struct ListFlowAliasesResponse {
    #[serde(rename = "flowAliasSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_alias_summaries: Option<Vec<FlowAliasSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlowAliasSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "concurrencyConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency_configuration: Option<FlowAliasConcurrencyConfiguration>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "flowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "routingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_configuration: Option<Vec<FlowAliasRoutingConfigurationListItem>>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFlowVersionsRequest {
    #[serde(rename = "flowIdentifier")]
    #[serde(default)]
    pub flow_identifier: String,
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
pub struct ListFlowVersionsResponse {
    #[serde(rename = "flowVersionSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_version_summaries: Option<Vec<FlowVersionSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlowVersionSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFlowsRequest {
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
pub struct ListFlowsResponse {
    #[serde(rename = "flowSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_summaries: Option<Vec<FlowSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlowSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIngestionJobsRequest {
    #[serde(rename = "dataSourceId")]
    #[serde(default)]
    pub data_source_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<IngestionJobFilter>>,
    #[serde(rename = "knowledgeBaseId")]
    #[serde(default)]
    pub knowledge_base_id: String,
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
    pub sort_by: Option<IngestionJobSortBy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IngestionJobFilter {
    #[serde(default)]
    pub attribute: String,
    #[serde(default)]
    pub operator: String,
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IngestionJobSortBy {
    #[serde(default)]
    pub attribute: String,
    #[serde(default)]
    pub order: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIngestionJobsResponse {
    #[serde(rename = "ingestionJobSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_job_summaries: Option<Vec<IngestionJobSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IngestionJobSummary {
    #[serde(rename = "dataSourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ingestionJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_job_id: Option<String>,
    #[serde(rename = "knowledgeBaseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base_id: Option<String>,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<IngestionJobStatistics>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListKnowledgeBaseDocumentsRequest {
    #[serde(rename = "dataSourceId")]
    #[serde(default)]
    pub data_source_id: String,
    #[serde(rename = "knowledgeBaseId")]
    #[serde(default)]
    pub knowledge_base_id: String,
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
pub struct ListKnowledgeBaseDocumentsResponse {
    #[serde(rename = "documentDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_details: Option<Vec<KnowledgeBaseDocumentDetail>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListKnowledgeBasesRequest {
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
pub struct ListKnowledgeBasesResponse {
    #[serde(rename = "knowledgeBaseSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base_summaries: Option<Vec<KnowledgeBaseSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KnowledgeBaseSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "knowledgeBaseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPromptsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "promptIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPromptsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "promptSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_summaries: Option<Vec<PromptSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PromptSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PrepareAgentRequest {
    #[serde(rename = "agentId")]
    #[serde(default)]
    pub agent_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PrepareAgentResponse {
    #[serde(rename = "agentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(rename = "agentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_status: Option<String>,
    #[serde(rename = "agentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    #[serde(rename = "preparedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prepared_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PrepareFlowRequest {
    #[serde(rename = "flowIdentifier")]
    #[serde(default)]
    pub flow_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PrepareFlowResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartIngestionJobRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "dataSourceId")]
    #[serde(default)]
    pub data_source_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "knowledgeBaseId")]
    #[serde(default)]
    pub knowledge_base_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartIngestionJobResponse {
    #[serde(rename = "ingestionJob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_job: Option<IngestionJob>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopIngestionJobRequest {
    #[serde(rename = "dataSourceId")]
    #[serde(default)]
    pub data_source_id: String,
    #[serde(rename = "ingestionJobId")]
    #[serde(default)]
    pub ingestion_job_id: String,
    #[serde(rename = "knowledgeBaseId")]
    #[serde(default)]
    pub knowledge_base_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopIngestionJobResponse {
    #[serde(rename = "ingestionJob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_job: Option<IngestionJob>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "tagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAgentActionGroupRequest {
    #[serde(rename = "actionGroupExecutor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_group_executor: Option<ActionGroupExecutor>,
    #[serde(rename = "actionGroupId")]
    #[serde(default)]
    pub action_group_id: String,
    #[serde(rename = "actionGroupName")]
    #[serde(default)]
    pub action_group_name: String,
    #[serde(rename = "actionGroupState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_group_state: Option<String>,
    #[serde(rename = "agentId")]
    #[serde(default)]
    pub agent_id: String,
    #[serde(rename = "agentVersion")]
    #[serde(default)]
    pub agent_version: String,
    #[serde(rename = "apiSchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_schema: Option<APISchema>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "functionSchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_schema: Option<FunctionSchema>,
    #[serde(rename = "parentActionGroupSignature")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_action_group_signature: Option<String>,
    #[serde(rename = "parentActionGroupSignatureParams")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_action_group_signature_params: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAgentActionGroupResponse {
    #[serde(rename = "agentActionGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_action_group: Option<AgentActionGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAgentAliasRequest {
    #[serde(rename = "agentAliasId")]
    #[serde(default)]
    pub agent_alias_id: String,
    #[serde(rename = "agentAliasName")]
    #[serde(default)]
    pub agent_alias_name: String,
    #[serde(rename = "agentId")]
    #[serde(default)]
    pub agent_id: String,
    #[serde(rename = "aliasInvocationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_invocation_state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "routingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_configuration: Option<Vec<AgentAliasRoutingConfigurationListItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAgentAliasResponse {
    #[serde(rename = "agentAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_alias: Option<AgentAlias>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAgentCollaboratorRequest {
    #[serde(rename = "agentDescriptor")]
    #[serde(default)]
    pub agent_descriptor: AgentDescriptor,
    #[serde(rename = "agentId")]
    #[serde(default)]
    pub agent_id: String,
    #[serde(rename = "agentVersion")]
    #[serde(default)]
    pub agent_version: String,
    #[serde(rename = "collaborationInstruction")]
    #[serde(default)]
    pub collaboration_instruction: String,
    #[serde(rename = "collaboratorId")]
    #[serde(default)]
    pub collaborator_id: String,
    #[serde(rename = "collaboratorName")]
    #[serde(default)]
    pub collaborator_name: String,
    #[serde(rename = "relayConversationHistory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relay_conversation_history: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAgentCollaboratorResponse {
    #[serde(rename = "agentCollaborator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_collaborator: Option<AgentCollaborator>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAgentKnowledgeBaseRequest {
    #[serde(rename = "agentId")]
    #[serde(default)]
    pub agent_id: String,
    #[serde(rename = "agentVersion")]
    #[serde(default)]
    pub agent_version: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "knowledgeBaseId")]
    #[serde(default)]
    pub knowledge_base_id: String,
    #[serde(rename = "knowledgeBaseState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAgentKnowledgeBaseResponse {
    #[serde(rename = "agentKnowledgeBase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_knowledge_base: Option<AgentKnowledgeBase>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAgentRequest {
    #[serde(rename = "agentCollaboration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_collaboration: Option<String>,
    #[serde(rename = "agentId")]
    #[serde(default)]
    pub agent_id: String,
    #[serde(rename = "agentName")]
    #[serde(default)]
    pub agent_name: String,
    #[serde(rename = "agentResourceRoleArn")]
    #[serde(default)]
    pub agent_resource_role_arn: String,
    #[serde(rename = "customOrchestration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_orchestration: Option<CustomOrchestration>,
    #[serde(rename = "customerEncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_encryption_key_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "foundationModel")]
    #[serde(default)]
    pub foundation_model: String,
    #[serde(rename = "guardrailConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guardrail_configuration: Option<GuardrailConfiguration>,
    #[serde(rename = "idleSessionTTLInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_session_t_t_l_in_seconds: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instruction: Option<String>,
    #[serde(rename = "memoryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_configuration: Option<MemoryConfiguration>,
    #[serde(rename = "orchestrationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orchestration_type: Option<String>,
    #[serde(rename = "promptOverrideConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_override_configuration: Option<PromptOverrideConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAgentResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<Agent>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDataSourceRequest {
    #[serde(rename = "dataDeletionPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_deletion_policy: Option<String>,
    #[serde(rename = "dataSourceConfiguration")]
    #[serde(default)]
    pub data_source_configuration: DataSourceConfiguration,
    #[serde(rename = "dataSourceId")]
    #[serde(default)]
    pub data_source_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "knowledgeBaseId")]
    #[serde(default)]
    pub knowledge_base_id: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "serverSideEncryptionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_side_encryption_configuration: Option<ServerSideEncryptionConfiguration>,
    #[serde(rename = "vectorIngestionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vector_ingestion_configuration: Option<VectorIngestionConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDataSourceResponse {
    #[serde(rename = "dataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<DataSource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFlowAliasRequest {
    #[serde(rename = "aliasIdentifier")]
    #[serde(default)]
    pub alias_identifier: String,
    #[serde(rename = "concurrencyConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency_configuration: Option<FlowAliasConcurrencyConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "flowIdentifier")]
    #[serde(default)]
    pub flow_identifier: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "routingConfiguration")]
    #[serde(default)]
    pub routing_configuration: Vec<FlowAliasRoutingConfigurationListItem>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFlowAliasResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "concurrencyConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency_configuration: Option<FlowAliasConcurrencyConfiguration>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "flowId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "routingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_configuration: Option<Vec<FlowAliasRoutingConfigurationListItem>>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFlowRequest {
    #[serde(rename = "customerEncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_encryption_key_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<FlowDefinition>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "executionRoleArn")]
    #[serde(default)]
    pub execution_role_arn: String,
    #[serde(rename = "flowIdentifier")]
    #[serde(default)]
    pub flow_identifier: String,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFlowResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "customerEncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_encryption_key_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<FlowDefinition>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "executionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateKnowledgeBaseRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "knowledgeBaseConfiguration")]
    #[serde(default)]
    pub knowledge_base_configuration: KnowledgeBaseConfiguration,
    #[serde(rename = "knowledgeBaseId")]
    #[serde(default)]
    pub knowledge_base_id: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "roleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "storageConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_configuration: Option<StorageConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateKnowledgeBaseResponse {
    #[serde(rename = "knowledgeBase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knowledge_base: Option<KnowledgeBase>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePromptRequest {
    #[serde(rename = "customerEncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_encryption_key_arn: Option<String>,
    #[serde(rename = "defaultVariant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_variant: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "promptIdentifier")]
    #[serde(default)]
    pub prompt_identifier: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<PromptVariant>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePromptResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "customerEncryptionKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_encryption_key_arn: Option<String>,
    #[serde(rename = "defaultVariant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_variant: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<PromptVariant>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidateFlowDefinitionRequest {
    #[serde(default)]
    pub definition: FlowDefinition,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidateFlowDefinitionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validations: Option<Vec<FlowValidation>>,
}
