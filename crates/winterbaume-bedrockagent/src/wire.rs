//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-bedrockagent

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

#[allow(unused_imports)]
use http::header::HeaderName;
use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for restJson protocol.
pub fn serialize_associate_agent_collaborator_response(
    result: &AssociateAgentCollaboratorResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_associate_agent_knowledge_base_response(
    result: &AssociateAgentKnowledgeBaseResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_agent_response(result: &CreateAgentResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_agent_action_group_response(
    result: &CreateAgentActionGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_agent_alias_response(result: &CreateAgentAliasResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_data_source_response(result: &CreateDataSourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_flow_response(result: &CreateFlowResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_flow_alias_response(result: &CreateFlowAliasResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_flow_version_response(result: &CreateFlowVersionResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_knowledge_base_response(
    result: &CreateKnowledgeBaseResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_prompt_response(result: &CreatePromptResponse) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_prompt_version_response(
    result: &CreatePromptVersionResponse,
) -> MockResponse {
    let status = 201_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_agent_response(result: &DeleteAgentResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_agent_action_group_response(
    result: &DeleteAgentActionGroupResponse,
) -> MockResponse {
    let status = 204_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_agent_alias_response(result: &DeleteAgentAliasResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_agent_version_response(
    result: &DeleteAgentVersionResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_data_source_response(result: &DeleteDataSourceResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_flow_response(result: &DeleteFlowResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_flow_alias_response(result: &DeleteFlowAliasResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_flow_version_response(result: &DeleteFlowVersionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_knowledge_base_response(
    result: &DeleteKnowledgeBaseResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_knowledge_base_documents_response(
    result: &DeleteKnowledgeBaseDocumentsResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_prompt_response(result: &DeletePromptResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_disassociate_agent_collaborator_response(
    result: &DisassociateAgentCollaboratorResponse,
) -> MockResponse {
    let status = 204_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_disassociate_agent_knowledge_base_response(
    result: &DisassociateAgentKnowledgeBaseResponse,
) -> MockResponse {
    let status = 204_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_agent_response(result: &GetAgentResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_agent_action_group_response(
    result: &GetAgentActionGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_agent_alias_response(result: &GetAgentAliasResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_agent_collaborator_response(
    result: &GetAgentCollaboratorResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_agent_knowledge_base_response(
    result: &GetAgentKnowledgeBaseResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_agent_version_response(result: &GetAgentVersionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_data_source_response(result: &GetDataSourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_flow_response(result: &GetFlowResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_flow_alias_response(result: &GetFlowAliasResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_flow_version_response(result: &GetFlowVersionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_ingestion_job_response(result: &GetIngestionJobResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_knowledge_base_response(result: &GetKnowledgeBaseResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_knowledge_base_documents_response(
    result: &GetKnowledgeBaseDocumentsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_prompt_response(result: &GetPromptResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_ingest_knowledge_base_documents_response(
    result: &IngestKnowledgeBaseDocumentsResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_agent_action_groups_response(
    result: &ListAgentActionGroupsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_agent_aliases_response(result: &ListAgentAliasesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_agent_collaborators_response(
    result: &ListAgentCollaboratorsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_agent_knowledge_bases_response(
    result: &ListAgentKnowledgeBasesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_agent_versions_response(result: &ListAgentVersionsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_agents_response(result: &ListAgentsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_data_sources_response(result: &ListDataSourcesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_flow_aliases_response(result: &ListFlowAliasesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_flow_versions_response(result: &ListFlowVersionsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_flows_response(result: &ListFlowsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_ingestion_jobs_response(result: &ListIngestionJobsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_knowledge_base_documents_response(
    result: &ListKnowledgeBaseDocumentsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_knowledge_bases_response(
    result: &ListKnowledgeBasesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_prompts_response(result: &ListPromptsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_tags_for_resource_response(
    result: &ListTagsForResourceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_prepare_agent_response(result: &PrepareAgentResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_prepare_flow_response(result: &PrepareFlowResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_ingestion_job_response(result: &StartIngestionJobResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_stop_ingestion_job_response(result: &StopIngestionJobResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_tag_resource_response(result: &TagResourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_untag_resource_response(result: &UntagResourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_agent_response(result: &UpdateAgentResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_agent_action_group_response(
    result: &UpdateAgentActionGroupResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_agent_alias_response(result: &UpdateAgentAliasResponse) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_agent_collaborator_response(
    result: &UpdateAgentCollaboratorResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_agent_knowledge_base_response(
    result: &UpdateAgentKnowledgeBaseResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_data_source_response(result: &UpdateDataSourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_flow_response(result: &UpdateFlowResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_flow_alias_response(result: &UpdateFlowAliasResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_knowledge_base_response(
    result: &UpdateKnowledgeBaseResponse,
) -> MockResponse {
    let status = 202_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_prompt_response(result: &UpdatePromptResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_validate_flow_definition_response(
    result: &ValidateFlowDefinitionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_agent_collaborator_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateAgentCollaboratorRequest, String> {
    let mut input = AssociateAgentCollaboratorRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateAgentCollaboratorRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize AssociateAgentCollaborator request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "agentId" => {
                input.agent_id = value.to_string();
            }
            "agentVersion" => {
                input.agent_version = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_agent_knowledge_base_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateAgentKnowledgeBaseRequest, String> {
    let mut input = AssociateAgentKnowledgeBaseRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateAgentKnowledgeBaseRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize AssociateAgentKnowledgeBase request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "agentId" => {
                input.agent_id = value.to_string();
            }
            "agentVersion" => {
                input.agent_version = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_agent_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAgentRequest, String> {
    let mut input = CreateAgentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateAgentRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateAgent request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_agent_action_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAgentActionGroupRequest, String> {
    let mut input = CreateAgentActionGroupRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateAgentActionGroupRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateAgentActionGroup request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "agentId" => {
                input.agent_id = value.to_string();
            }
            "agentVersion" => {
                input.agent_version = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_agent_alias_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateAgentAliasRequest, String> {
    let mut input = CreateAgentAliasRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateAgentAliasRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateAgentAlias request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "agentId" => {
                input.agent_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_data_source_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateDataSourceRequest, String> {
    let mut input = CreateDataSourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateDataSourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateDataSource request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "knowledgeBaseId" => {
                input.knowledge_base_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_flow_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateFlowRequest, String> {
    let mut input = CreateFlowRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateFlowRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateFlow request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_flow_alias_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateFlowAliasRequest, String> {
    let mut input = CreateFlowAliasRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateFlowAliasRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateFlowAlias request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "flowIdentifier" => {
                input.flow_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_flow_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateFlowVersionRequest, String> {
    let mut input = CreateFlowVersionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateFlowVersionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateFlowVersion request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "flowIdentifier" => {
                input.flow_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_knowledge_base_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateKnowledgeBaseRequest, String> {
    let mut input = CreateKnowledgeBaseRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateKnowledgeBaseRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateKnowledgeBase request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_prompt_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreatePromptRequest, String> {
    let mut input = CreatePromptRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreatePromptRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreatePrompt request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_prompt_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreatePromptVersionRequest, String> {
    let mut input = CreatePromptVersionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreatePromptVersionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreatePromptVersion request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "promptIdentifier" => {
                input.prompt_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_agent_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAgentRequest, String> {
    let mut input = DeleteAgentRequest::default();
    for (name, value) in labels {
        match *name {
            "agentId" => {
                input.agent_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("skipResourceInUseCheck") {
        input.skip_resource_in_use_check = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_agent_action_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAgentActionGroupRequest, String> {
    let mut input = DeleteAgentActionGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "actionGroupId" => {
                input.action_group_id = value.to_string();
            }
            "agentId" => {
                input.agent_id = value.to_string();
            }
            "agentVersion" => {
                input.agent_version = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("skipResourceInUseCheck") {
        input.skip_resource_in_use_check = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_agent_alias_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAgentAliasRequest, String> {
    let mut input = DeleteAgentAliasRequest::default();
    for (name, value) in labels {
        match *name {
            "agentAliasId" => {
                input.agent_alias_id = value.to_string();
            }
            "agentId" => {
                input.agent_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_agent_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteAgentVersionRequest, String> {
    let mut input = DeleteAgentVersionRequest::default();
    for (name, value) in labels {
        match *name {
            "agentId" => {
                input.agent_id = value.to_string();
            }
            "agentVersion" => {
                input.agent_version = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("skipResourceInUseCheck") {
        input.skip_resource_in_use_check = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_data_source_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteDataSourceRequest, String> {
    let mut input = DeleteDataSourceRequest::default();
    for (name, value) in labels {
        match *name {
            "dataSourceId" => {
                input.data_source_id = value.to_string();
            }
            "knowledgeBaseId" => {
                input.knowledge_base_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_flow_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteFlowRequest, String> {
    let mut input = DeleteFlowRequest::default();
    for (name, value) in labels {
        match *name {
            "flowIdentifier" => {
                input.flow_identifier = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("skipResourceInUseCheck") {
        input.skip_resource_in_use_check = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_flow_alias_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteFlowAliasRequest, String> {
    let mut input = DeleteFlowAliasRequest::default();
    for (name, value) in labels {
        match *name {
            "aliasIdentifier" => {
                input.alias_identifier = value.to_string();
            }
            "flowIdentifier" => {
                input.flow_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_flow_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteFlowVersionRequest, String> {
    let mut input = DeleteFlowVersionRequest::default();
    for (name, value) in labels {
        match *name {
            "flowIdentifier" => {
                input.flow_identifier = value.to_string();
            }
            "flowVersion" => {
                input.flow_version = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("skipResourceInUseCheck") {
        input.skip_resource_in_use_check = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_knowledge_base_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteKnowledgeBaseRequest, String> {
    let mut input = DeleteKnowledgeBaseRequest::default();
    for (name, value) in labels {
        match *name {
            "knowledgeBaseId" => {
                input.knowledge_base_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_knowledge_base_documents_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteKnowledgeBaseDocumentsRequest, String> {
    let mut input = DeleteKnowledgeBaseDocumentsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<DeleteKnowledgeBaseDocumentsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize DeleteKnowledgeBaseDocuments request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "dataSourceId" => {
                input.data_source_id = value.to_string();
            }
            "knowledgeBaseId" => {
                input.knowledge_base_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_prompt_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeletePromptRequest, String> {
    let mut input = DeletePromptRequest::default();
    for (name, value) in labels {
        match *name {
            "promptIdentifier" => {
                input.prompt_identifier = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("promptVersion") {
        input.prompt_version = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_agent_collaborator_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateAgentCollaboratorRequest, String> {
    let mut input = DisassociateAgentCollaboratorRequest::default();
    for (name, value) in labels {
        match *name {
            "agentId" => {
                input.agent_id = value.to_string();
            }
            "agentVersion" => {
                input.agent_version = value.to_string();
            }
            "collaboratorId" => {
                input.collaborator_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_agent_knowledge_base_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateAgentKnowledgeBaseRequest, String> {
    let mut input = DisassociateAgentKnowledgeBaseRequest::default();
    for (name, value) in labels {
        match *name {
            "agentId" => {
                input.agent_id = value.to_string();
            }
            "agentVersion" => {
                input.agent_version = value.to_string();
            }
            "knowledgeBaseId" => {
                input.knowledge_base_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_agent_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAgentRequest, String> {
    let mut input = GetAgentRequest::default();
    for (name, value) in labels {
        match *name {
            "agentId" => {
                input.agent_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_agent_action_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAgentActionGroupRequest, String> {
    let mut input = GetAgentActionGroupRequest::default();
    for (name, value) in labels {
        match *name {
            "actionGroupId" => {
                input.action_group_id = value.to_string();
            }
            "agentId" => {
                input.agent_id = value.to_string();
            }
            "agentVersion" => {
                input.agent_version = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_agent_alias_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAgentAliasRequest, String> {
    let mut input = GetAgentAliasRequest::default();
    for (name, value) in labels {
        match *name {
            "agentAliasId" => {
                input.agent_alias_id = value.to_string();
            }
            "agentId" => {
                input.agent_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_agent_collaborator_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAgentCollaboratorRequest, String> {
    let mut input = GetAgentCollaboratorRequest::default();
    for (name, value) in labels {
        match *name {
            "agentId" => {
                input.agent_id = value.to_string();
            }
            "agentVersion" => {
                input.agent_version = value.to_string();
            }
            "collaboratorId" => {
                input.collaborator_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_agent_knowledge_base_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAgentKnowledgeBaseRequest, String> {
    let mut input = GetAgentKnowledgeBaseRequest::default();
    for (name, value) in labels {
        match *name {
            "agentId" => {
                input.agent_id = value.to_string();
            }
            "agentVersion" => {
                input.agent_version = value.to_string();
            }
            "knowledgeBaseId" => {
                input.knowledge_base_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_agent_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetAgentVersionRequest, String> {
    let mut input = GetAgentVersionRequest::default();
    for (name, value) in labels {
        match *name {
            "agentId" => {
                input.agent_id = value.to_string();
            }
            "agentVersion" => {
                input.agent_version = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_data_source_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDataSourceRequest, String> {
    let mut input = GetDataSourceRequest::default();
    for (name, value) in labels {
        match *name {
            "dataSourceId" => {
                input.data_source_id = value.to_string();
            }
            "knowledgeBaseId" => {
                input.knowledge_base_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_flow_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFlowRequest, String> {
    let mut input = GetFlowRequest::default();
    for (name, value) in labels {
        match *name {
            "flowIdentifier" => {
                input.flow_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_flow_alias_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFlowAliasRequest, String> {
    let mut input = GetFlowAliasRequest::default();
    for (name, value) in labels {
        match *name {
            "aliasIdentifier" => {
                input.alias_identifier = value.to_string();
            }
            "flowIdentifier" => {
                input.flow_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_flow_version_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFlowVersionRequest, String> {
    let mut input = GetFlowVersionRequest::default();
    for (name, value) in labels {
        match *name {
            "flowIdentifier" => {
                input.flow_identifier = value.to_string();
            }
            "flowVersion" => {
                input.flow_version = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_ingestion_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetIngestionJobRequest, String> {
    let mut input = GetIngestionJobRequest::default();
    for (name, value) in labels {
        match *name {
            "dataSourceId" => {
                input.data_source_id = value.to_string();
            }
            "ingestionJobId" => {
                input.ingestion_job_id = value.to_string();
            }
            "knowledgeBaseId" => {
                input.knowledge_base_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_knowledge_base_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetKnowledgeBaseRequest, String> {
    let mut input = GetKnowledgeBaseRequest::default();
    for (name, value) in labels {
        match *name {
            "knowledgeBaseId" => {
                input.knowledge_base_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_knowledge_base_documents_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetKnowledgeBaseDocumentsRequest, String> {
    let mut input = GetKnowledgeBaseDocumentsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<GetKnowledgeBaseDocumentsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize GetKnowledgeBaseDocuments request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "dataSourceId" => {
                input.data_source_id = value.to_string();
            }
            "knowledgeBaseId" => {
                input.knowledge_base_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_prompt_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetPromptRequest, String> {
    let mut input = GetPromptRequest::default();
    for (name, value) in labels {
        match *name {
            "promptIdentifier" => {
                input.prompt_identifier = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("promptVersion") {
        input.prompt_version = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_ingest_knowledge_base_documents_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<IngestKnowledgeBaseDocumentsRequest, String> {
    let mut input = IngestKnowledgeBaseDocumentsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<IngestKnowledgeBaseDocumentsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize IngestKnowledgeBaseDocuments request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "dataSourceId" => {
                input.data_source_id = value.to_string();
            }
            "knowledgeBaseId" => {
                input.knowledge_base_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_agent_action_groups_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAgentActionGroupsRequest, String> {
    let mut input = ListAgentActionGroupsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListAgentActionGroupsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListAgentActionGroups request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "agentId" => {
                input.agent_id = value.to_string();
            }
            "agentVersion" => {
                input.agent_version = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_agent_aliases_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAgentAliasesRequest, String> {
    let mut input = ListAgentAliasesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListAgentAliasesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListAgentAliases request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "agentId" => {
                input.agent_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_agent_collaborators_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAgentCollaboratorsRequest, String> {
    let mut input = ListAgentCollaboratorsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListAgentCollaboratorsRequest>(&request.body).map_err(
            |err| format!("failed to deserialize ListAgentCollaborators request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "agentId" => {
                input.agent_id = value.to_string();
            }
            "agentVersion" => {
                input.agent_version = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_agent_knowledge_bases_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAgentKnowledgeBasesRequest, String> {
    let mut input = ListAgentKnowledgeBasesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListAgentKnowledgeBasesRequest>(&request.body).map_err(
            |err| format!("failed to deserialize ListAgentKnowledgeBases request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "agentId" => {
                input.agent_id = value.to_string();
            }
            "agentVersion" => {
                input.agent_version = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_agent_versions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAgentVersionsRequest, String> {
    let mut input = ListAgentVersionsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListAgentVersionsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListAgentVersions request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "agentId" => {
                input.agent_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_agents_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListAgentsRequest, String> {
    let mut input = ListAgentsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListAgentsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListAgents request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_data_sources_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDataSourcesRequest, String> {
    let mut input = ListDataSourcesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListDataSourcesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListDataSources request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "knowledgeBaseId" => {
                input.knowledge_base_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_flow_aliases_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListFlowAliasesRequest, String> {
    let mut input = ListFlowAliasesRequest::default();
    for (name, value) in labels {
        match *name {
            "flowIdentifier" => {
                input.flow_identifier = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_flow_versions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListFlowVersionsRequest, String> {
    let mut input = ListFlowVersionsRequest::default();
    for (name, value) in labels {
        match *name {
            "flowIdentifier" => {
                input.flow_identifier = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_flows_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListFlowsRequest, String> {
    let mut input = ListFlowsRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_ingestion_jobs_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListIngestionJobsRequest, String> {
    let mut input = ListIngestionJobsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListIngestionJobsRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListIngestionJobs request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "dataSourceId" => {
                input.data_source_id = value.to_string();
            }
            "knowledgeBaseId" => {
                input.knowledge_base_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_knowledge_base_documents_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListKnowledgeBaseDocumentsRequest, String> {
    let mut input = ListKnowledgeBaseDocumentsRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListKnowledgeBaseDocumentsRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize ListKnowledgeBaseDocuments request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "dataSourceId" => {
                input.data_source_id = value.to_string();
            }
            "knowledgeBaseId" => {
                input.knowledge_base_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_knowledge_bases_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListKnowledgeBasesRequest, String> {
    let mut input = ListKnowledgeBasesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ListKnowledgeBasesRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize ListKnowledgeBases request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_prompts_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListPromptsRequest, String> {
    let mut input = ListPromptsRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("promptIdentifier") {
        input.prompt_identifier = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_tags_for_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTagsForResourceRequest, String> {
    let mut input = ListTagsForResourceRequest::default();
    for (name, value) in labels {
        match *name {
            "resourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_prepare_agent_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PrepareAgentRequest, String> {
    let mut input = PrepareAgentRequest::default();
    for (name, value) in labels {
        match *name {
            "agentId" => {
                input.agent_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_prepare_flow_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PrepareFlowRequest, String> {
    let mut input = PrepareFlowRequest::default();
    for (name, value) in labels {
        match *name {
            "flowIdentifier" => {
                input.flow_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_ingestion_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartIngestionJobRequest, String> {
    let mut input = StartIngestionJobRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartIngestionJobRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StartIngestionJob request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "dataSourceId" => {
                input.data_source_id = value.to_string();
            }
            "knowledgeBaseId" => {
                input.knowledge_base_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_stop_ingestion_job_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StopIngestionJobRequest, String> {
    let mut input = StopIngestionJobRequest::default();
    for (name, value) in labels {
        match *name {
            "dataSourceId" => {
                input.data_source_id = value.to_string();
            }
            "ingestionJobId" => {
                input.ingestion_job_id = value.to_string();
            }
            "knowledgeBaseId" => {
                input.knowledge_base_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_tag_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<TagResourceRequest, String> {
    let mut input = TagResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<TagResourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize TagResource request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "resourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_untag_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UntagResourceRequest, String> {
    let mut input = UntagResourceRequest::default();
    for (name, value) in labels {
        match *name {
            "resourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("tagKeys") {
        input.tag_keys = value
            .split(',')
            .filter(|item| !item.trim().is_empty())
            .map(|item| Ok(item.trim().to_string()))
            .collect::<Result<Vec<_>, String>>()?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_agent_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAgentRequest, String> {
    let mut input = UpdateAgentRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAgentRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateAgent request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "agentId" => {
                input.agent_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_agent_action_group_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAgentActionGroupRequest, String> {
    let mut input = UpdateAgentActionGroupRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAgentActionGroupRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateAgentActionGroup request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "actionGroupId" => {
                input.action_group_id = value.to_string();
            }
            "agentId" => {
                input.agent_id = value.to_string();
            }
            "agentVersion" => {
                input.agent_version = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_agent_alias_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAgentAliasRequest, String> {
    let mut input = UpdateAgentAliasRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAgentAliasRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateAgentAlias request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "agentAliasId" => {
                input.agent_alias_id = value.to_string();
            }
            "agentId" => {
                input.agent_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_agent_collaborator_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAgentCollaboratorRequest, String> {
    let mut input = UpdateAgentCollaboratorRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAgentCollaboratorRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateAgentCollaborator request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "agentId" => {
                input.agent_id = value.to_string();
            }
            "agentVersion" => {
                input.agent_version = value.to_string();
            }
            "collaboratorId" => {
                input.collaborator_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_agent_knowledge_base_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateAgentKnowledgeBaseRequest, String> {
    let mut input = UpdateAgentKnowledgeBaseRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateAgentKnowledgeBaseRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateAgentKnowledgeBase request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "agentId" => {
                input.agent_id = value.to_string();
            }
            "agentVersion" => {
                input.agent_version = value.to_string();
            }
            "knowledgeBaseId" => {
                input.knowledge_base_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_data_source_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDataSourceRequest, String> {
    let mut input = UpdateDataSourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateDataSourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateDataSource request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "dataSourceId" => {
                input.data_source_id = value.to_string();
            }
            "knowledgeBaseId" => {
                input.knowledge_base_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_flow_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateFlowRequest, String> {
    let mut input = UpdateFlowRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateFlowRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateFlow request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "flowIdentifier" => {
                input.flow_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_flow_alias_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateFlowAliasRequest, String> {
    let mut input = UpdateFlowAliasRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateFlowAliasRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateFlowAlias request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "aliasIdentifier" => {
                input.alias_identifier = value.to_string();
            }
            "flowIdentifier" => {
                input.flow_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_knowledge_base_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateKnowledgeBaseRequest, String> {
    let mut input = UpdateKnowledgeBaseRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateKnowledgeBaseRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateKnowledgeBase request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "knowledgeBaseId" => {
                input.knowledge_base_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_prompt_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdatePromptRequest, String> {
    let mut input = UpdatePromptRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdatePromptRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdatePrompt request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "promptIdentifier" => {
                input.prompt_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_validate_flow_definition_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ValidateFlowDefinitionRequest, String> {
    let mut input = ValidateFlowDefinitionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<ValidateFlowDefinitionRequest>(&request.body).map_err(
            |err| format!("failed to deserialize ValidateFlowDefinition request: {err}"),
        )?;
    }
    Ok(input)
}
