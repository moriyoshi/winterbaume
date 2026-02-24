use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use serde_json::{Value, json};
use winterbaume_core::{
    BackendState, DEFAULT_ACCOUNT_ID, MockRequest, MockResponse, MockService, StateChangeNotifier,
    extract_path, extract_query_string, parse_query_string,
};

use crate::state::{AgentAliasState, BedrockAgentError, BedrockAgentState};
use crate::types::{Agent, KnowledgeBase};
use crate::views::BedrockAgentStateView;
use crate::wire;

const X_AMZN_ERRORTYPE: http::header::HeaderName =
    http::header::HeaderName::from_static("x-amzn-errortype");

pub struct BedrockAgentService {
    pub(crate) state: Arc<BackendState<BedrockAgentState>>,
    pub(crate) notifier: StateChangeNotifier<BedrockAgentStateView>,
}

impl BedrockAgentService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(BackendState::new()),
            notifier: StateChangeNotifier::new(),
        }
    }
}

impl Default for BedrockAgentService {
    fn default() -> Self {
        Self::new()
    }
}

impl MockService for BedrockAgentService {
    fn service_name(&self) -> &str {
        "bedrock"
    }

    fn url_patterns(&self) -> Vec<&str> {
        vec![
            r"https?://bedrock-agent\..*\.amazonaws\.com",
            r"https?://bedrock-agent\.amazonaws\.com",
        ]
    }

    fn handle(
        &self,
        request: MockRequest,
    ) -> Pin<Box<dyn Future<Output = MockResponse> + Send + '_>> {
        Box::pin(async move { self.dispatch(request).await })
    }
}

impl BedrockAgentService {
    async fn dispatch(&self, request: MockRequest) -> MockResponse {
        let region = winterbaume_core::auth::extract_region_from_uri(&request.uri);
        let account_id = DEFAULT_ACCOUNT_ID;
        let state = self.state.get(account_id, &region);

        let path = extract_path(&request.uri);
        let query_string = extract_query_string(&request.uri);
        let query_map: HashMap<String, String> = parse_query_string(query_string);
        let method = request.method.as_str();

        let segments: Vec<&str> = path
            .trim_start_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();

        if segments.is_empty() {
            return rest_json_error(404, "UnknownOperationException", "Not found");
        }

        match segments[0] {
            "agents" => {
                self.handle_agents(
                    method, &segments, &request, &query_map, &state, account_id, &region,
                )
                .await
            }
            "knowledgebases" => {
                self.handle_knowledge_bases(
                    method, &segments, &request, &query_map, &state, account_id, &region,
                )
                .await
            }
            "tags" => {
                self.handle_tags(method, &segments, &request, &query_map, &state)
                    .await
            }
            "flows" => {
                self.handle_flows(
                    method, &segments, &request, &query_map, &state, account_id, &region,
                )
                .await
            }
            "prompts" => {
                self.handle_prompts(
                    method, &segments, &request, &query_map, &state, account_id, &region,
                )
                .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn handle_agents(
        &self,
        method: &str,
        segments: &[&str],
        request: &MockRequest,
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        // Routing table:
        let _ = query;
        // PUT  /agents/                                                                    -> CreateAgent
        // POST /agents/                                                                    -> ListAgents
        // GET  /agents/{agentId}/                                                          -> GetAgent
        // PUT  /agents/{agentId}/                                                          -> UpdateAgent
        // DELETE /agents/{agentId}/                                                        -> DeleteAgent
        // POST /agents/{agentId}/                                                          -> PrepareAgent
        // POST /agents/{agentId}/agentaliases/                                             -> ListAgentAliases
        // PUT  /agents/{agentId}/agentaliases/                                             -> CreateAgentAlias
        // GET  /agents/{agentId}/agentaliases/{agentAliasId}/                              -> GetAgentAlias
        // PUT  /agents/{agentId}/agentaliases/{agentAliasId}/                              -> UpdateAgentAlias
        // DELETE /agents/{agentId}/agentaliases/{agentAliasId}/                            -> DeleteAgentAlias
        // POST /agents/{agentId}/agentversions/                                            -> ListAgentVersions
        // GET  /agents/{agentId}/agentversions/{agentVersion}/                             -> GetAgentVersion
        // DELETE /agents/{agentId}/agentversions/{agentVersion}/                           -> DeleteAgentVersion
        // POST /agents/{agentId}/agentversions/{agentVersion}/actiongroups/               -> ListAgentActionGroups
        // PUT  /agents/{agentId}/agentversions/{agentVersion}/actiongroups/               -> CreateAgentActionGroup
        // GET  /agents/{agentId}/agentversions/{agentVersion}/actiongroups/{agId}/        -> GetAgentActionGroup
        // PUT  /agents/{agentId}/agentversions/{agentVersion}/actiongroups/{agId}/        -> UpdateAgentActionGroup
        // DELETE /agents/{agentId}/agentversions/{agentVersion}/actiongroups/{agId}/      -> DeleteAgentActionGroup
        // POST /agents/{agentId}/agentversions/{agentVersion}/knowledgebases/             -> ListAgentKnowledgeBases
        // PUT  /agents/{agentId}/agentversions/{agentVersion}/knowledgebases/             -> AssociateAgentKnowledgeBase
        // GET  /agents/{agentId}/agentversions/{agentVersion}/knowledgebases/{kbId}/      -> GetAgentKnowledgeBase
        // PUT  /agents/{agentId}/agentversions/{agentVersion}/knowledgebases/{kbId}/      -> UpdateAgentKnowledgeBase
        // DELETE /agents/{agentId}/agentversions/{agentVersion}/knowledgebases/{kbId}/    -> DisassociateAgentKnowledgeBase
        // POST /agents/{agentId}/agentversions/{agentVersion}/agentcollaborators/         -> ListAgentCollaborators
        // PUT  /agents/{agentId}/agentversions/{agentVersion}/agentcollaborators/         -> AssociateAgentCollaborator
        // GET  /agents/{agentId}/agentversions/{agentVersion}/agentcollaborators/{colId}/ -> GetAgentCollaborator
        // PUT  /agents/{agentId}/agentversions/{agentVersion}/agentcollaborators/{colId}/ -> UpdateAgentCollaborator
        // DELETE /agents/{agentId}/agentversions/{agentVersion}/agentcollaborators/{colId}/ -> DisassociateAgentCollaborator

        match segments.len() {
            1 => match method {
                "PUT" => {
                    self.handle_create_agent(request, &[], query, state, account_id, region)
                        .await
                }
                "POST" => self.handle_list_agents(request, &[], query, state).await,
                _ => rest_json_error(405, "MethodNotAllowedException", "Method not allowed"),
            },
            2 => match method {
                "GET" => self.handle_get_agent(segments[1], state).await,
                "PUT" => {
                    self.handle_update_agent(segments[1], request, &[], query, state)
                        .await
                }
                "DELETE" => {
                    self.handle_delete_agent(segments[1], request, &[], query, state)
                        .await
                }
                "POST" => self.handle_prepare_agent(segments[1], state).await,
                _ => rest_json_error(405, "MethodNotAllowedException", "Method not allowed"),
            },
            3 => {
                let agent_id = segments[1];
                match segments[2] {
                    "agentaliases" => match method {
                        "PUT" => {
                            self.handle_create_agent_alias(
                                agent_id,
                                request,
                                &[],
                                query,
                                state,
                                account_id,
                                region,
                            )
                            .await
                        }
                        "POST" => {
                            self.handle_list_agent_aliases(agent_id, request, &[], query, state)
                                .await
                        }
                        _ => {
                            rest_json_error(405, "MethodNotAllowedException", "Method not allowed")
                        }
                    },
                    "agentversions" => match method {
                        "POST" => {
                            self.handle_list_agent_versions(agent_id, request, &[], query, state)
                                .await
                        }
                        _ => {
                            rest_json_error(405, "MethodNotAllowedException", "Method not allowed")
                        }
                    },
                    _ => rest_json_error(404, "UnknownOperationException", "Not found"),
                }
            }
            4 => {
                let agent_id = segments[1];
                match segments[2] {
                    "agentaliases" => {
                        let alias_id = segments[3];
                        match method {
                            "GET" => self.handle_get_agent_alias(agent_id, alias_id, state).await,
                            "PUT" => {
                                self.handle_update_agent_alias(
                                    agent_id,
                                    alias_id,
                                    request,
                                    &[],
                                    query,
                                    state,
                                )
                                .await
                            }
                            "DELETE" => {
                                self.handle_delete_agent_alias(agent_id, alias_id, state)
                                    .await
                            }
                            _ => rest_json_error(
                                405,
                                "MethodNotAllowedException",
                                "Method not allowed",
                            ),
                        }
                    }
                    "agentversions" => {
                        let agent_version = segments[3];
                        match method {
                            "GET" => {
                                self.handle_get_agent_version(agent_id, agent_version, state)
                                    .await
                            }
                            "DELETE" => {
                                self.handle_delete_agent_version(
                                    agent_id,
                                    agent_version,
                                    request,
                                    &[],
                                    query,
                                    state,
                                )
                                .await
                            }
                            _ => rest_json_error(
                                405,
                                "MethodNotAllowedException",
                                "Method not allowed",
                            ),
                        }
                    }
                    _ => rest_json_error(404, "UnknownOperationException", "Not found"),
                }
            }
            5 => {
                let agent_id = segments[1];
                let agent_version = segments[3];
                match segments[4] {
                    "actiongroups" => match method {
                        "POST" => {
                            self.handle_list_agent_action_groups(
                                agent_id,
                                agent_version,
                                request,
                                &[],
                                query,
                                state,
                            )
                            .await
                        }
                        "PUT" => {
                            self.handle_create_agent_action_group(
                                agent_id,
                                agent_version,
                                request,
                                &[],
                                query,
                                state,
                                account_id,
                                region,
                            )
                            .await
                        }
                        _ => {
                            rest_json_error(405, "MethodNotAllowedException", "Method not allowed")
                        }
                    },
                    "knowledgebases" => match method {
                        "POST" => {
                            self.handle_list_agent_knowledge_bases(
                                agent_id,
                                agent_version,
                                request,
                                &[],
                                query,
                                state,
                            )
                            .await
                        }
                        "PUT" => {
                            self.handle_associate_agent_knowledge_base(
                                agent_id,
                                agent_version,
                                request,
                                &[],
                                query,
                                state,
                            )
                            .await
                        }
                        _ => {
                            rest_json_error(405, "MethodNotAllowedException", "Method not allowed")
                        }
                    },
                    "agentcollaborators" => match method {
                        "POST" => {
                            self.handle_list_agent_collaborators(
                                agent_id,
                                agent_version,
                                request,
                                &[],
                                query,
                                state,
                            )
                            .await
                        }
                        "PUT" => {
                            self.handle_associate_agent_collaborator(
                                agent_id,
                                agent_version,
                                request,
                                &[],
                                query,
                                state,
                            )
                            .await
                        }
                        _ => {
                            rest_json_error(405, "MethodNotAllowedException", "Method not allowed")
                        }
                    },
                    _ => rest_json_error(404, "UnknownOperationException", "Not found"),
                }
            }
            6 => {
                let agent_id = segments[1];
                let agent_version = segments[3];
                let resource_id = segments[5];
                match segments[4] {
                    "actiongroups" => match method {
                        "GET" => {
                            self.handle_get_agent_action_group(
                                agent_id,
                                agent_version,
                                resource_id,
                                state,
                            )
                            .await
                        }
                        "PUT" => {
                            self.handle_update_agent_action_group(
                                agent_id,
                                agent_version,
                                resource_id,
                                request,
                                &[],
                                query,
                                state,
                            )
                            .await
                        }
                        "DELETE" => {
                            self.handle_delete_agent_action_group(
                                agent_id,
                                agent_version,
                                resource_id,
                                state,
                            )
                            .await
                        }
                        _ => {
                            rest_json_error(405, "MethodNotAllowedException", "Method not allowed")
                        }
                    },
                    "knowledgebases" => match method {
                        "GET" => {
                            self.handle_get_agent_knowledge_base(
                                agent_id,
                                agent_version,
                                resource_id,
                                state,
                            )
                            .await
                        }
                        "PUT" => {
                            self.handle_update_agent_knowledge_base(
                                agent_id,
                                agent_version,
                                resource_id,
                                request,
                                &[],
                                query,
                                state,
                            )
                            .await
                        }
                        "DELETE" => {
                            self.handle_disassociate_agent_knowledge_base(
                                agent_id,
                                agent_version,
                                resource_id,
                                state,
                            )
                            .await
                        }
                        _ => {
                            rest_json_error(405, "MethodNotAllowedException", "Method not allowed")
                        }
                    },
                    "agentcollaborators" => match method {
                        "GET" => {
                            self.handle_get_agent_collaborator(
                                agent_id,
                                agent_version,
                                resource_id,
                                state,
                            )
                            .await
                        }
                        "PUT" => {
                            self.handle_update_agent_collaborator(
                                agent_id,
                                agent_version,
                                resource_id,
                                request,
                                &[],
                                query,
                                state,
                            )
                            .await
                        }
                        "DELETE" => {
                            self.handle_disassociate_agent_collaborator(
                                agent_id,
                                agent_version,
                                resource_id,
                                state,
                            )
                            .await
                        }
                        _ => {
                            rest_json_error(405, "MethodNotAllowedException", "Method not allowed")
                        }
                    },
                    _ => rest_json_error(404, "UnknownOperationException", "Not found"),
                }
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn handle_knowledge_bases(
        &self,
        method: &str,
        segments: &[&str],
        request: &MockRequest,
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let _ = query;
        // POST /knowledgebases/{kbId}/datasources/{dsId}/ingestionjobs/{jobId}/stop -> StopIngestionJob
        // First check if there's a "stop" suffix
        if segments.len() == 7 && segments[6] == "stop" {
            // POST /knowledgebases/{kbId}/datasources/{dsId}/ingestionjobs/{jobId}/stop
            let kb_id = segments[1];
            let ds_id = segments[3];
            let job_id = segments[5];
            if method == "POST" {
                return self
                    .handle_stop_ingestion_job(kb_id, ds_id, job_id, state)
                    .await;
            }
        }

        // Check for document operations (deleteDocuments, getDocuments)
        if segments.len() == 6 && segments[4] == "documents" {
            let kb_id = segments[1];
            let ds_id = segments[3];
            match segments[5] {
                "deleteDocuments" if method == "POST" => {
                    return self
                        .handle_delete_knowledge_base_documents(kb_id, ds_id, request, state)
                        .await;
                }
                "getDocuments" if method == "POST" => {
                    return self
                        .handle_get_knowledge_base_documents(kb_id, ds_id, request, state)
                        .await;
                }
                _ => {}
            }
        }

        match segments.len() {
            1 => match method {
                "PUT" => {
                    self.handle_create_knowledge_base(
                        request,
                        &[],
                        query,
                        state,
                        account_id,
                        region,
                    )
                    .await
                }
                "POST" => {
                    self.handle_list_knowledge_bases(request, &[], query, state)
                        .await
                }
                _ => rest_json_error(405, "MethodNotAllowedException", "Method not allowed"),
            },
            2 => {
                let kb_id = segments[1];
                match method {
                    "GET" => self.handle_get_knowledge_base(kb_id, state).await,
                    "PUT" => {
                        self.handle_update_knowledge_base(kb_id, request, &[], query, state)
                            .await
                    }
                    "DELETE" => self.handle_delete_knowledge_base(kb_id, state).await,
                    _ => rest_json_error(405, "MethodNotAllowedException", "Method not allowed"),
                }
            }
            3 => {
                // /knowledgebases/{kbId}/datasources/
                let kb_id = segments[1];
                match segments[2] {
                    "datasources" => match method {
                        "PUT" => {
                            self.handle_create_data_source(
                                kb_id,
                                request,
                                &[],
                                query,
                                state,
                                account_id,
                                region,
                            )
                            .await
                        }
                        "POST" => {
                            self.handle_list_data_sources(kb_id, request, &[], query, state)
                                .await
                        }
                        _ => {
                            rest_json_error(405, "MethodNotAllowedException", "Method not allowed")
                        }
                    },
                    _ => rest_json_error(404, "UnknownOperationException", "Not found"),
                }
            }
            4 => {
                // /knowledgebases/{kbId}/datasources/{dsId}
                let kb_id = segments[1];
                let ds_id = segments[3];
                match method {
                    "GET" => self.handle_get_data_source(kb_id, ds_id, state).await,
                    "PUT" => {
                        self.handle_update_data_source(kb_id, ds_id, request, &[], query, state)
                            .await
                    }
                    "DELETE" => self.handle_delete_data_source(kb_id, ds_id, state).await,
                    _ => rest_json_error(405, "MethodNotAllowedException", "Method not allowed"),
                }
            }
            5 => {
                let kb_id = segments[1];
                let ds_id = segments[3];
                match segments[4] {
                    "ingestionjobs" => match method {
                        "PUT" => {
                            self.handle_start_ingestion_job(
                                kb_id,
                                ds_id,
                                request,
                                &[],
                                query,
                                state,
                            )
                            .await
                        }
                        "POST" => {
                            self.handle_list_ingestion_jobs(
                                kb_id,
                                ds_id,
                                request,
                                &[],
                                query,
                                state,
                            )
                            .await
                        }
                        _ => {
                            rest_json_error(405, "MethodNotAllowedException", "Method not allowed")
                        }
                    },
                    "documents" => match method {
                        "PUT" => {
                            self.handle_ingest_knowledge_base_documents(
                                kb_id, ds_id, request, state,
                            )
                            .await
                        }
                        "POST" => {
                            self.handle_list_knowledge_base_documents(kb_id, ds_id, request, state)
                                .await
                        }
                        _ => {
                            rest_json_error(405, "MethodNotAllowedException", "Method not allowed")
                        }
                    },
                    _ => rest_json_error(404, "UnknownOperationException", "Not found"),
                }
            }
            6 => {
                let kb_id = segments[1];
                let ds_id = segments[3];
                let job_id = segments[5];
                match segments[4] {
                    "ingestionjobs" => match method {
                        "GET" => {
                            self.handle_get_ingestion_job(kb_id, ds_id, job_id, state)
                                .await
                        }
                        _ => {
                            rest_json_error(405, "MethodNotAllowedException", "Method not allowed")
                        }
                    },
                    _ => rest_json_error(404, "UnknownOperationException", "Not found"),
                }
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn handle_flows(
        &self,
        method: &str,
        segments: &[&str],
        request: &MockRequest,
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let _ = query;
        // GET  /flows/                                              -> ListFlows
        // POST /flows/                                              -> CreateFlow
        // POST /flows/validate-definition                           -> ValidateFlowDefinition
        // GET  /flows/{flowId}/                                     -> GetFlow
        // PUT  /flows/{flowId}/                                     -> UpdateFlow
        // DELETE /flows/{flowId}/                                   -> DeleteFlow
        // POST /flows/{flowId}/                                     -> PrepareFlow
        // GET  /flows/{flowId}/aliases                              -> ListFlowAliases
        // POST /flows/{flowId}/aliases                              -> CreateFlowAlias
        // GET  /flows/{flowId}/aliases/{aliasId}                    -> GetFlowAlias
        // PUT  /flows/{flowId}/aliases/{aliasId}                    -> UpdateFlowAlias
        // DELETE /flows/{flowId}/aliases/{aliasId}                  -> DeleteFlowAlias
        // GET  /flows/{flowId}/versions                             -> ListFlowVersions
        // POST /flows/{flowId}/versions                             -> CreateFlowVersion
        // GET  /flows/{flowId}/versions/{version}/                  -> GetFlowVersion
        // DELETE /flows/{flowId}/versions/{version}/                -> DeleteFlowVersion

        match segments.len() {
            1 => match method {
                "GET" => self.handle_list_flows(request, &[], query, state).await,
                "POST" => {
                    self.handle_create_flow(request, &[], query, state, account_id, region)
                        .await
                }
                _ => rest_json_error(405, "MethodNotAllowedException", "Method not allowed"),
            },
            2 => {
                if segments[1] == "validate-definition" {
                    if method == "POST" {
                        return self
                            .handle_validate_flow_definition(request, &[], query, state)
                            .await;
                    }
                }
                let flow_id = segments[1];
                match method {
                    "GET" => self.handle_get_flow(flow_id, state).await,
                    "PUT" => {
                        self.handle_update_flow(flow_id, request, &[], query, state)
                            .await
                    }
                    "DELETE" => self.handle_delete_flow(flow_id, state).await,
                    "POST" => self.handle_prepare_flow(flow_id, state).await,
                    _ => rest_json_error(405, "MethodNotAllowedException", "Method not allowed"),
                }
            }
            3 => {
                let flow_id = segments[1];
                match segments[2] {
                    "aliases" => match method {
                        "GET" => self.handle_list_flow_aliases(flow_id, request, state).await,
                        "POST" => {
                            self.handle_create_flow_alias(
                                flow_id,
                                request,
                                &[],
                                query,
                                state,
                                account_id,
                                region,
                            )
                            .await
                        }
                        _ => {
                            rest_json_error(405, "MethodNotAllowedException", "Method not allowed")
                        }
                    },
                    "versions" => match method {
                        "GET" => {
                            self.handle_list_flow_versions(flow_id, request, state)
                                .await
                        }
                        "POST" => {
                            self.handle_create_flow_version(flow_id, request, state)
                                .await
                        }
                        _ => {
                            rest_json_error(405, "MethodNotAllowedException", "Method not allowed")
                        }
                    },
                    _ => rest_json_error(404, "UnknownOperationException", "Not found"),
                }
            }
            4 => {
                let flow_id = segments[1];
                let resource_id = segments[3];
                match segments[2] {
                    "aliases" => match method {
                        "GET" => {
                            self.handle_get_flow_alias(flow_id, resource_id, state)
                                .await
                        }
                        "PUT" => {
                            self.handle_update_flow_alias(
                                flow_id,
                                resource_id,
                                request,
                                &[],
                                query,
                                state,
                            )
                            .await
                        }
                        "DELETE" => {
                            self.handle_delete_flow_alias(flow_id, resource_id, state)
                                .await
                        }
                        _ => {
                            rest_json_error(405, "MethodNotAllowedException", "Method not allowed")
                        }
                    },
                    "versions" => match method {
                        "GET" => {
                            self.handle_get_flow_version(flow_id, resource_id, state)
                                .await
                        }
                        "DELETE" => {
                            self.handle_delete_flow_version(flow_id, resource_id, state)
                                .await
                        }
                        _ => {
                            rest_json_error(405, "MethodNotAllowedException", "Method not allowed")
                        }
                    },
                    _ => rest_json_error(404, "UnknownOperationException", "Not found"),
                }
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn handle_prompts(
        &self,
        method: &str,
        segments: &[&str],
        request: &MockRequest,
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let _ = query;
        // GET  /prompts/                                   -> ListPrompts
        // POST /prompts/                                   -> CreatePrompt
        // GET  /prompts/{promptId}/                        -> GetPrompt
        // PUT  /prompts/{promptId}/                        -> UpdatePrompt
        // DELETE /prompts/{promptId}/                      -> DeletePrompt
        // POST /prompts/{promptId}/versions                -> CreatePromptVersion

        match segments.len() {
            1 => match method {
                "GET" => self.handle_list_prompts(request, state).await,
                "POST" => {
                    self.handle_create_prompt(request, &[], query, state, account_id, region)
                        .await
                }
                _ => rest_json_error(405, "MethodNotAllowedException", "Method not allowed"),
            },
            2 => {
                let prompt_id = segments[1];
                match method {
                    "GET" => self.handle_get_prompt(prompt_id, state).await,
                    "PUT" => {
                        self.handle_update_prompt(prompt_id, request, &[], query, state)
                            .await
                    }
                    "DELETE" => self.handle_delete_prompt(prompt_id, state).await,
                    _ => rest_json_error(405, "MethodNotAllowedException", "Method not allowed"),
                }
            }
            3 => {
                let prompt_id = segments[1];
                match segments[2] {
                    "versions" => match method {
                        "POST" => {
                            self.handle_create_prompt_version(prompt_id, request, state)
                                .await
                        }
                        _ => {
                            rest_json_error(405, "MethodNotAllowedException", "Method not allowed")
                        }
                    },
                    _ => rest_json_error(404, "UnknownOperationException", "Not found"),
                }
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    async fn handle_tags(
        &self,
        method: &str,
        segments: &[&str],
        request: &MockRequest,
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let _ = query;
        if segments.len() < 2 {
            return rest_json_error(400, "ValidationException", "Missing resource ARN");
        }

        let resource_arn = segments[1..].join("/");
        let resource_arn = urldecode(&resource_arn);

        match method {
            "POST" => {
                self.handle_tag_resource(&resource_arn, request, state)
                    .await
            }
            "GET" => {
                self.handle_list_tags_for_resource(&resource_arn, state)
                    .await
            }
            "DELETE" => {
                self.handle_untag_resource(&resource_arn, request, state)
                    .await
            }
            _ => rest_json_error(404, "UnknownOperationException", "Not found"),
        }
    }

    async fn handle_tag_resource(
        &self,
        resource_arn: &str,
        request: &MockRequest,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let labels: &[(&str, &str)] = &[("resourceArn", resource_arn)];
        let query: HashMap<String, String> = HashMap::new();
        let input = match wire::deserialize_tag_resource_request(request, labels, &query) {
            Ok(v) => v,
            Err(_) => {
                return rest_json_error(400, "SerializationException", "Invalid JSON");
            }
        };
        let tags: HashMap<String, String> = input.tags;

        let mut s = state.write().await;
        match s.tag_resource(resource_arn, &tags) {
            Ok(()) => wire::serialize_tag_resource_response(&wire::TagResourceResponse {}),
            Err(e) => ba_error_response(&e),
        }
    }

    async fn handle_list_tags_for_resource(
        &self,
        resource_arn: &str,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let s = state.read().await;
        match s.list_tags_for_resource(resource_arn) {
            Ok(tags) => wire::serialize_list_tags_for_resource_response(
                &wire::ListTagsForResourceResponse { tags: Some(tags) },
            ),
            Err(e) => ba_error_response(&e),
        }
    }

    async fn handle_untag_resource(
        &self,
        resource_arn: &str,
        request: &MockRequest,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let query = extract_query(&request.uri);
        let tag_keys: Vec<String> = query
            .iter()
            .filter(|(k, _)| k == "tagKeys")
            .map(|(_, v)| v.clone())
            .collect();

        let mut s = state.write().await;
        match s.untag_resource(resource_arn, &tag_keys) {
            Ok(()) => wire::serialize_untag_resource_response(&wire::UntagResourceResponse {}),
            Err(e) => ba_error_response(&e),
        }
    }

    // ─── Agent operations ────────────────────────────────────────────

    #[allow(clippy::too_many_arguments)]
    async fn handle_create_agent(
        &self,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_agent_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON"),
        };

        let agent_name = input.agent_name.as_str();
        let agent_resource_role_arn = input.agent_resource_role_arn.as_deref().unwrap_or("");
        let client_token = input.client_token.as_deref();
        let instruction = input.instruction.as_deref();
        let foundation_model = input.foundation_model.as_deref();
        let description = input.description.as_deref();
        let idle_ttl = input.idle_session_t_t_l_in_seconds.map(|v| v as i64);
        let customer_key = input.customer_encryption_key_arn.as_deref();
        let tags = input.tags;

        let mut s = state.write().await;
        let agent = s.create_agent(
            agent_name,
            agent_resource_role_arn,
            account_id,
            region,
            client_token,
            instruction,
            foundation_model,
            description,
            idle_ttl,
            customer_key,
        );
        let agent_wire = agent_to_wire(agent);
        let agent_arn = agent.agent_arn.clone();

        if let Some(tag_map) = tags {
            let _ = s.tag_resource(&agent_arn, &tag_map);
        }

        wire::serialize_create_agent_response(&wire::CreateAgentResponse {
            agent: Some(agent_wire),
        })
    }

    async fn handle_list_agents(
        &self,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_agents_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON"),
        };
        let max_results = input.max_results.map(|v| v as i64);
        let next_token = input.next_token.as_deref();

        let s = state.read().await;
        let (summaries, token) = s.list_agents(max_results, next_token);

        let agent_summaries: Vec<wire::AgentSummary> =
            summaries.iter().map(agent_summary_from_value).collect();

        wire::serialize_list_agents_response(&wire::ListAgentsResponse {
            agent_summaries: Some(agent_summaries),
            next_token: token,
        })
    }

    async fn handle_get_agent(
        &self,
        agent_id: &str,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_agent(agent_id) {
            Ok(agent) => wire::serialize_get_agent_response(&wire::GetAgentResponse {
                agent: Some(agent_to_wire(agent)),
            }),
            Err(e) => ba_error_response(&e),
        }
    }

    async fn handle_update_agent(
        &self,
        agent_id: &str,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_agent_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON"),
        };
        let body = serde_json::to_value(&input).unwrap_or(Value::Null);

        let mut s = state.write().await;
        match s.update_agent(agent_id, &body) {
            Ok(agent) => wire::serialize_update_agent_response(&wire::UpdateAgentResponse {
                agent: Some(agent_to_wire(agent)),
            }),
            Err(e) => ba_error_response(&e),
        }
    }

    async fn handle_delete_agent(
        &self,
        agent_id: &str,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let _input = match wire::deserialize_delete_agent_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON"),
        };
        let skip = request.uri.contains("skipResourceInUseCheck=true");
        let mut s = state.write().await;
        match s.delete_agent(agent_id, skip) {
            Ok((id, status)) => wire::serialize_delete_agent_response(&wire::DeleteAgentResponse {
                agent_id: Some(id),
                agent_status: Some(status),
            }),
            Err(e) => ba_error_response(&e),
        }
    }

    async fn handle_prepare_agent(
        &self,
        agent_id: &str,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.prepare_agent(agent_id) {
            Ok(agent) => wire::serialize_prepare_agent_response(&wire::PrepareAgentResponse {
                agent_id: Some(agent.agent_id.clone()),
                agent_status: Some(agent.agent_status.clone()),
                agent_version: Some(agent.agent_version.clone()),
                prepared_at: Some(agent.prepared_at.clone()),
            }),
            Err(e) => ba_error_response(&e),
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn handle_create_agent_alias(
        &self,
        agent_id: &str,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_agent_alias_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON"),
        };
        let name = if input.agent_alias_name.is_empty() {
            "default"
        } else {
            input.agent_alias_name.as_str()
        };
        let description = input.description.as_deref();

        let mut s = state.write().await;
        match s.create_agent_alias(agent_id, name, description, account_id, region) {
            Ok(alias) => {
                wire::serialize_create_agent_alias_response(&wire::CreateAgentAliasResponse {
                    agent_alias: Some(agent_alias_to_wire(alias)),
                })
            }
            Err(e) => ba_error_response(&e),
        }
    }

    async fn handle_list_agent_aliases(
        &self,
        agent_id: &str,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_agent_aliases_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON"),
        };
        let max_results = input.max_results.map(|v| v as i64);
        let next_token = input.next_token.as_deref();

        let s = state.read().await;
        let (summaries, token) = s.list_agent_aliases(agent_id, max_results, next_token);

        wire::serialize_list_agent_aliases_response(&wire::ListAgentAliasesResponse {
            agent_alias_summaries: Some(summaries),
            next_token: token,
        })
    }

    async fn handle_get_agent_alias(
        &self,
        agent_id: &str,
        alias_id: &str,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_agent_alias(agent_id, alias_id) {
            Ok(alias) => wire::serialize_get_agent_alias_response(&wire::GetAgentAliasResponse {
                agent_alias: Some(agent_alias_to_wire(alias)),
            }),
            Err(e) => ba_error_response(&e),
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn handle_update_agent_alias(
        &self,
        agent_id: &str,
        alias_id: &str,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_agent_alias_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON"),
        };
        let name = if input.agent_alias_name.is_empty() {
            "default"
        } else {
            input.agent_alias_name.as_str()
        };

        let mut s = state.write().await;
        match s.update_agent_alias(agent_id, alias_id, name) {
            Ok(alias) => {
                wire::serialize_update_agent_alias_response(&wire::UpdateAgentAliasResponse {
                    agent_alias: Some(agent_alias_to_wire(alias)),
                })
            }
            Err(e) => ba_error_response(&e),
        }
    }

    async fn handle_delete_agent_alias(
        &self,
        agent_id: &str,
        alias_id: &str,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.delete_agent_alias(agent_id, alias_id) {
            Ok((aid, alid, status)) => {
                wire::serialize_delete_agent_alias_response(&wire::DeleteAgentAliasResponse {
                    agent_id: Some(aid),
                    agent_alias_id: Some(alid),
                    agent_alias_status: Some(status),
                })
            }
            Err(e) => ba_error_response(&e),
        }
    }

    async fn handle_list_agent_versions(
        &self,
        agent_id: &str,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_agent_versions_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON"),
        };
        let max_results = input.max_results.map(|v| v as i64);
        let next_token = input.next_token.as_deref();

        let s = state.read().await;
        let (summaries, token) = s.list_agent_versions(agent_id, max_results, next_token);

        wire::serialize_list_agent_versions_response(&wire::ListAgentVersionsResponse {
            agent_version_summaries: Some(summaries),
            next_token: token,
        })
    }

    async fn handle_get_agent_version(
        &self,
        agent_id: &str,
        agent_version: &str,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_agent(agent_id) {
            Ok(agent) => {
                wire::serialize_get_agent_version_response(&wire::GetAgentVersionResponse {
                    agent_version: Some(wire::AgentVersion {
                        agent_id: Some(agent.agent_id.clone()),
                        agent_name: Some(agent.agent_name.clone()),
                        agent_arn: Some(agent.agent_arn.clone()),
                        agent_resource_role_arn: Some(agent.agent_resource_role_arn.clone()),
                        agent_status: Some(agent.agent_status.clone()),
                        version: Some(agent_version.to_string()),
                        created_at: Some(agent.created_at.clone()),
                        updated_at: Some(agent.updated_at.clone()),
                        ..Default::default()
                    }),
                })
            }
            Err(e) => ba_error_response(&e),
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn handle_delete_agent_version(
        &self,
        agent_id: &str,
        agent_version: &str,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let _input = match wire::deserialize_delete_agent_version_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON"),
        };
        let skip = request.uri.contains("skipResourceInUseCheck=true");
        let s = state.read().await;
        match s.get_agent(agent_id) {
            Ok(agent) => {
                let _ = skip;
                wire::serialize_delete_agent_version_response(&wire::DeleteAgentVersionResponse {
                    agent_id: Some(agent.agent_id.clone()),
                    agent_status: Some("DELETING".to_string()),
                    agent_version: Some(agent_version.to_string()),
                })
            }
            Err(e) => ba_error_response(&e),
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn handle_list_agent_action_groups(
        &self,
        agent_id: &str,
        agent_version: &str,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_agent_action_groups_request(request, labels, query)
        {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON"),
        };
        let max_results = input.max_results.map(|v| v as i64);
        let next_token = input.next_token.as_deref();

        let s = state.read().await;
        let (summaries, token) =
            s.list_agent_action_groups(agent_id, agent_version, max_results, next_token);

        wire::serialize_list_agent_action_groups_response(&wire::ListAgentActionGroupsResponse {
            action_group_summaries: Some(summaries),
            next_token: token,
        })
    }

    #[allow(clippy::too_many_arguments)]
    async fn handle_create_agent_action_group(
        &self,
        agent_id: &str,
        agent_version: &str,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input =
            match wire::deserialize_create_agent_action_group_request(request, labels, query) {
                Ok(v) => v,
                Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON"),
            };
        let name = if input.action_group_name.is_empty() {
            "default".to_string()
        } else {
            input.action_group_name.clone()
        };
        let body = serde_json::to_value(&input).unwrap_or(Value::Null);

        let mut s = state.write().await;
        match s.create_agent_action_group(agent_id, agent_version, &name, &body, account_id, region)
        {
            Ok(ag) => wire::serialize_create_agent_action_group_response(
                &wire::CreateAgentActionGroupResponse {
                    agent_action_group: Some(ag),
                },
            ),
            Err(e) => ba_error_response(&e),
        }
    }

    async fn handle_get_agent_action_group(
        &self,
        agent_id: &str,
        agent_version: &str,
        action_group_id: &str,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_agent_action_group(agent_id, agent_version, action_group_id) {
            Ok(ag) => wire::serialize_get_agent_action_group_response(
                &wire::GetAgentActionGroupResponse {
                    agent_action_group: Some(ag),
                },
            ),
            Err(e) => ba_error_response(&e),
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn handle_update_agent_action_group(
        &self,
        agent_id: &str,
        agent_version: &str,
        action_group_id: &str,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_update_agent_action_group_request(request, labels, query) {
                Ok(v) => v,
                Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON"),
            };
        let body = serde_json::to_value(&input).unwrap_or(Value::Null);

        let mut s = state.write().await;
        match s.update_agent_action_group(agent_id, agent_version, action_group_id, &body) {
            Ok(ag) => wire::serialize_update_agent_action_group_response(
                &wire::UpdateAgentActionGroupResponse {
                    agent_action_group: Some(ag),
                },
            ),
            Err(e) => ba_error_response(&e),
        }
    }

    async fn handle_delete_agent_action_group(
        &self,
        agent_id: &str,
        agent_version: &str,
        action_group_id: &str,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.delete_agent_action_group(agent_id, agent_version, action_group_id) {
            Ok(()) => wire::serialize_delete_agent_action_group_response(
                &wire::DeleteAgentActionGroupResponse {},
            ),
            Err(e) => ba_error_response(&e),
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn handle_list_agent_knowledge_bases(
        &self,
        agent_id: &str,
        agent_version: &str,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_list_agent_knowledge_bases_request(request, labels, query) {
                Ok(v) => v,
                Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON"),
            };
        let max_results = input.max_results.map(|v| v as i64);
        let next_token = input.next_token.as_deref();

        let s = state.read().await;
        let (summaries, token) =
            s.list_agent_knowledge_bases(agent_id, agent_version, max_results, next_token);

        wire::serialize_list_agent_knowledge_bases_response(
            &wire::ListAgentKnowledgeBasesResponse {
                agent_knowledge_base_summaries: Some(summaries),
                next_token: token,
            },
        )
    }

    #[allow(clippy::too_many_arguments)]
    async fn handle_associate_agent_knowledge_base(
        &self,
        agent_id: &str,
        agent_version: &str,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let input = match wire::deserialize_associate_agent_knowledge_base_request(
            request, labels, query,
        ) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON"),
        };
        let kb_id = input.knowledge_base_id.as_str();
        let description = input.description.as_str();

        let mut s = state.write().await;
        match s.associate_agent_knowledge_base(agent_id, agent_version, kb_id, description) {
            Ok(akb) => wire::serialize_associate_agent_knowledge_base_response(
                &wire::AssociateAgentKnowledgeBaseResponse {
                    agent_knowledge_base: Some(akb),
                },
            ),
            Err(e) => ba_error_response(&e),
        }
    }

    async fn handle_get_agent_knowledge_base(
        &self,
        agent_id: &str,
        agent_version: &str,
        kb_id: &str,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_agent_knowledge_base(agent_id, agent_version, kb_id) {
            Ok(akb) => wire::serialize_get_agent_knowledge_base_response(
                &wire::GetAgentKnowledgeBaseResponse {
                    agent_knowledge_base: Some(akb),
                },
            ),
            Err(e) => ba_error_response(&e),
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn handle_update_agent_knowledge_base(
        &self,
        agent_id: &str,
        agent_version: &str,
        kb_id: &str,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_update_agent_knowledge_base_request(request, labels, query) {
                Ok(v) => v,
                Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON"),
            };
        let body = serde_json::to_value(&input).unwrap_or(Value::Null);

        let mut s = state.write().await;
        match s.update_agent_knowledge_base(agent_id, agent_version, kb_id, &body) {
            Ok(akb) => wire::serialize_update_agent_knowledge_base_response(
                &wire::UpdateAgentKnowledgeBaseResponse {
                    agent_knowledge_base: Some(akb),
                },
            ),
            Err(e) => ba_error_response(&e),
        }
    }

    async fn handle_disassociate_agent_knowledge_base(
        &self,
        agent_id: &str,
        agent_version: &str,
        kb_id: &str,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.disassociate_agent_knowledge_base(agent_id, agent_version, kb_id) {
            Ok(()) => wire::serialize_disassociate_agent_knowledge_base_response(
                &wire::DisassociateAgentKnowledgeBaseResponse {},
            ),
            Err(e) => ba_error_response(&e),
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn handle_list_agent_collaborators(
        &self,
        agent_id: &str,
        agent_version: &str,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_agent_collaborators_request(request, labels, query)
        {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON"),
        };
        let max_results = input.max_results.map(|v| v as i64);
        let next_token = input.next_token.as_deref();

        let s = state.read().await;
        let (summaries, token) =
            s.list_agent_collaborators(agent_id, agent_version, max_results, next_token);

        wire::serialize_list_agent_collaborators_response(&wire::ListAgentCollaboratorsResponse {
            agent_collaborator_summaries: Some(summaries),
            next_token: token,
        })
    }

    #[allow(clippy::too_many_arguments)]
    async fn handle_associate_agent_collaborator(
        &self,
        agent_id: &str,
        agent_version: &str,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_associate_agent_collaborator_request(request, labels, query) {
                Ok(v) => v,
                Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON"),
            };
        let body = serde_json::to_value(&input).unwrap_or(Value::Null);

        let mut s = state.write().await;
        match s.associate_agent_collaborator(agent_id, agent_version, &body) {
            Ok(ac) => wire::serialize_associate_agent_collaborator_response(
                &wire::AssociateAgentCollaboratorResponse {
                    agent_collaborator: Some(ac),
                },
            ),
            Err(e) => ba_error_response(&e),
        }
    }

    async fn handle_get_agent_collaborator(
        &self,
        agent_id: &str,
        agent_version: &str,
        collaborator_id: &str,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_agent_collaborator(agent_id, agent_version, collaborator_id) {
            Ok(ac) => wire::serialize_get_agent_collaborator_response(
                &wire::GetAgentCollaboratorResponse {
                    agent_collaborator: Some(ac),
                },
            ),
            Err(e) => ba_error_response(&e),
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn handle_update_agent_collaborator(
        &self,
        agent_id: &str,
        agent_version: &str,
        collaborator_id: &str,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let input =
            match wire::deserialize_update_agent_collaborator_request(request, labels, query) {
                Ok(v) => v,
                Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON"),
            };
        let body = serde_json::to_value(&input).unwrap_or(Value::Null);

        let mut s = state.write().await;
        match s.update_agent_collaborator(agent_id, agent_version, collaborator_id, &body) {
            Ok(ac) => wire::serialize_update_agent_collaborator_response(
                &wire::UpdateAgentCollaboratorResponse {
                    agent_collaborator: Some(ac),
                },
            ),
            Err(e) => ba_error_response(&e),
        }
    }

    async fn handle_disassociate_agent_collaborator(
        &self,
        agent_id: &str,
        agent_version: &str,
        collaborator_id: &str,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.disassociate_agent_collaborator(agent_id, agent_version, collaborator_id) {
            Ok(()) => wire::serialize_disassociate_agent_collaborator_response(
                &wire::DisassociateAgentCollaboratorResponse {},
            ),
            Err(e) => ba_error_response(&e),
        }
    }

    // ─── KnowledgeBase operations ────────────────────────────────────

    #[allow(clippy::too_many_arguments)]
    async fn handle_create_knowledge_base(
        &self,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_knowledge_base_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON"),
        };

        let name = input.name.as_str();
        let role_arn = input.role_arn.as_str();
        let description = input.description.as_deref();
        let kb_config =
            serde_json::to_value(&input.knowledge_base_configuration).unwrap_or(json!({}));
        let storage_config =
            serde_json::to_value(&input.storage_configuration).unwrap_or(json!({}));
        let tags = input.tags;

        let mut s = state.write().await;
        match s.create_knowledge_base(
            name,
            role_arn,
            account_id,
            region,
            &kb_config,
            &storage_config,
            description,
        ) {
            Ok(kb) => {
                let kb_wire = knowledge_base_to_wire(kb);
                let kb_arn = kb.knowledge_base_arn.clone();

                if let Some(tag_map) = tags {
                    let _ = s.tag_resource(&kb_arn, &tag_map);
                }

                wire::serialize_create_knowledge_base_response(&wire::CreateKnowledgeBaseResponse {
                    knowledge_base: Some(kb_wire),
                })
            }
            Err(e) => ba_error_response(&e),
        }
    }

    async fn handle_list_knowledge_bases(
        &self,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_knowledge_bases_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON"),
        };
        let max_results = input.max_results.map(|v| v as i64);
        let next_token = input.next_token.as_deref();

        let s = state.read().await;
        let (summaries, token) = s.list_knowledge_bases(max_results, next_token);

        let kb_summaries: Vec<wire::KnowledgeBaseSummary> =
            summaries.iter().map(kb_summary_from_value).collect();

        wire::serialize_list_knowledge_bases_response(&wire::ListKnowledgeBasesResponse {
            knowledge_base_summaries: Some(kb_summaries),
            next_token: token,
        })
    }

    async fn handle_get_knowledge_base(
        &self,
        kb_id: &str,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_knowledge_base(kb_id) {
            Ok(kb) => {
                wire::serialize_get_knowledge_base_response(&wire::GetKnowledgeBaseResponse {
                    knowledge_base: Some(knowledge_base_to_wire(kb)),
                })
            }
            Err(e) => ba_error_response(&e),
        }
    }

    async fn handle_update_knowledge_base(
        &self,
        kb_id: &str,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_knowledge_base_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON"),
        };
        let body = serde_json::to_value(&input).unwrap_or(Value::Null);

        let mut s = state.write().await;
        match s.update_knowledge_base(kb_id, &body) {
            Ok(kb) => {
                wire::serialize_update_knowledge_base_response(&wire::UpdateKnowledgeBaseResponse {
                    knowledge_base: Some(knowledge_base_to_wire(kb)),
                })
            }
            Err(e) => ba_error_response(&e),
        }
    }

    async fn handle_delete_knowledge_base(
        &self,
        kb_id: &str,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.delete_knowledge_base(kb_id) {
            Ok((id, status)) => {
                wire::serialize_delete_knowledge_base_response(&wire::DeleteKnowledgeBaseResponse {
                    knowledge_base_id: Some(id),
                    status: Some(status),
                })
            }
            Err(e) => ba_error_response(&e),
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn handle_create_data_source(
        &self,
        kb_id: &str,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_data_source_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON"),
        };
        let name = if input.name.is_empty() {
            "ds".to_string()
        } else {
            input.name.clone()
        };
        let body = serde_json::to_value(&input).unwrap_or(Value::Null);

        let mut s = state.write().await;
        match s.create_data_source(kb_id, &name, &body, account_id, region) {
            Ok(ds) => {
                wire::serialize_create_data_source_response(&wire::CreateDataSourceResponse {
                    data_source: Some(ds),
                })
            }
            Err(e) => ba_error_response(&e),
        }
    }

    async fn handle_list_data_sources(
        &self,
        kb_id: &str,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_data_sources_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON"),
        };
        let max_results = input.max_results.map(|v| v as i64);
        let next_token = input.next_token.as_deref();

        let s = state.read().await;
        let (summaries, token) = s.list_data_sources(kb_id, max_results, next_token);

        wire::serialize_list_data_sources_response(&wire::ListDataSourcesResponse {
            data_source_summaries: Some(summaries),
            next_token: token,
        })
    }

    async fn handle_get_data_source(
        &self,
        kb_id: &str,
        ds_id: &str,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_data_source(kb_id, ds_id) {
            Ok(ds) => wire::serialize_get_data_source_response(&wire::GetDataSourceResponse {
                data_source: Some(ds),
            }),
            Err(e) => ba_error_response(&e),
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn handle_update_data_source(
        &self,
        kb_id: &str,
        ds_id: &str,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_data_source_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON"),
        };
        let body = serde_json::to_value(&input).unwrap_or(Value::Null);

        let mut s = state.write().await;
        match s.update_data_source(kb_id, ds_id, &body) {
            Ok(ds) => {
                wire::serialize_update_data_source_response(&wire::UpdateDataSourceResponse {
                    data_source: Some(ds),
                })
            }
            Err(e) => ba_error_response(&e),
        }
    }

    async fn handle_delete_data_source(
        &self,
        kb_id: &str,
        ds_id: &str,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.delete_data_source(kb_id, ds_id) {
            Ok((kid, did, status)) => {
                wire::serialize_delete_data_source_response(&wire::DeleteDataSourceResponse {
                    knowledge_base_id: Some(kid),
                    data_source_id: Some(did),
                    status: Some(status),
                })
            }
            Err(e) => ba_error_response(&e),
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn handle_start_ingestion_job(
        &self,
        kb_id: &str,
        ds_id: &str,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let input = match wire::deserialize_start_ingestion_job_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON"),
        };
        let description = input.description.as_deref();

        let mut s = state.write().await;
        match s.start_ingestion_job(kb_id, ds_id, description) {
            Ok(job) => {
                wire::serialize_start_ingestion_job_response(&wire::StartIngestionJobResponse {
                    ingestion_job: Some(job),
                })
            }
            Err(e) => ba_error_response(&e),
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn handle_list_ingestion_jobs(
        &self,
        kb_id: &str,
        ds_id: &str,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let input = match wire::deserialize_list_ingestion_jobs_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON"),
        };
        let max_results = input.max_results.map(|v| v as i64);
        let next_token = input.next_token.as_deref();

        let s = state.read().await;
        let (summaries, token) = s.list_ingestion_jobs(kb_id, ds_id, max_results, next_token);

        wire::serialize_list_ingestion_jobs_response(&wire::ListIngestionJobsResponse {
            ingestion_job_summaries: Some(summaries),
            next_token: token,
        })
    }

    async fn handle_get_ingestion_job(
        &self,
        kb_id: &str,
        ds_id: &str,
        job_id: &str,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_ingestion_job(kb_id, ds_id, job_id) {
            Ok(job) => wire::serialize_get_ingestion_job_response(&wire::GetIngestionJobResponse {
                ingestion_job: Some(job),
            }),
            Err(e) => ba_error_response(&e),
        }
    }

    async fn handle_stop_ingestion_job(
        &self,
        kb_id: &str,
        ds_id: &str,
        job_id: &str,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.stop_ingestion_job(kb_id, ds_id, job_id) {
            Ok(job) => {
                wire::serialize_stop_ingestion_job_response(&wire::StopIngestionJobResponse {
                    ingestion_job: Some(job),
                })
            }
            Err(e) => ba_error_response(&e),
        }
    }

    async fn handle_ingest_knowledge_base_documents(
        &self,
        kb_id: &str,
        ds_id: &str,
        request: &MockRequest,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let body: serde_json::Value = match serde_json::from_slice(&request.body) {
            Ok(v) => v,
            Err(_) => return ba_json_error(400, "ValidationException", "Invalid JSON body"),
        };
        let ingest_req: wire::IngestKnowledgeBaseDocumentsRequest =
            match serde_json::from_value(body) {
                Ok(r) => r,
                Err(_) => return ba_json_error(400, "ValidationException", "Invalid request body"),
            };

        let mut s = state.write().await;
        match s.ingest_knowledge_base_documents(kb_id, ds_id, &ingest_req.documents) {
            Ok(docs) => {
                let details: Vec<wire::KnowledgeBaseDocumentDetail> =
                    docs.iter().map(doc_state_to_wire).collect();
                wire::serialize_ingest_knowledge_base_documents_response(
                    &wire::IngestKnowledgeBaseDocumentsResponse {
                        document_details: Some(details),
                    },
                )
            }
            Err(e) => ba_error_response(&e),
        }
    }

    async fn handle_list_knowledge_base_documents(
        &self,
        kb_id: &str,
        ds_id: &str,
        _request: &MockRequest,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let s = state.read().await;
        match s.list_knowledge_base_documents(kb_id, ds_id) {
            Ok(docs) => {
                let details: Vec<wire::KnowledgeBaseDocumentDetail> =
                    docs.iter().map(|d| doc_state_to_wire(d)).collect();
                wire::serialize_list_knowledge_base_documents_response(
                    &wire::ListKnowledgeBaseDocumentsResponse {
                        document_details: Some(details),
                        next_token: None,
                    },
                )
            }
            Err(e) => ba_error_response(&e),
        }
    }

    async fn handle_get_knowledge_base_documents(
        &self,
        kb_id: &str,
        ds_id: &str,
        request: &MockRequest,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let body: serde_json::Value = match serde_json::from_slice(&request.body) {
            Ok(v) => v,
            Err(_) => return ba_json_error(400, "ValidationException", "Invalid JSON body"),
        };
        let get_req: wire::GetKnowledgeBaseDocumentsRequest = match serde_json::from_value(body) {
            Ok(r) => r,
            Err(_) => return ba_json_error(400, "ValidationException", "Invalid request body"),
        };

        let s = state.read().await;
        match s.get_knowledge_base_documents(kb_id, ds_id, &get_req.document_identifiers) {
            Ok(docs) => {
                let details: Vec<wire::KnowledgeBaseDocumentDetail> =
                    docs.iter().map(|d| doc_state_to_wire(d)).collect();
                wire::serialize_get_knowledge_base_documents_response(
                    &wire::GetKnowledgeBaseDocumentsResponse {
                        document_details: Some(details),
                    },
                )
            }
            Err(e) => ba_error_response(&e),
        }
    }

    async fn handle_delete_knowledge_base_documents(
        &self,
        kb_id: &str,
        ds_id: &str,
        request: &MockRequest,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let body: serde_json::Value = match serde_json::from_slice(&request.body) {
            Ok(v) => v,
            Err(_) => return ba_json_error(400, "ValidationException", "Invalid JSON body"),
        };
        let del_req: wire::DeleteKnowledgeBaseDocumentsRequest = match serde_json::from_value(body)
        {
            Ok(r) => r,
            Err(_) => return ba_json_error(400, "ValidationException", "Invalid request body"),
        };

        let mut s = state.write().await;
        match s.delete_knowledge_base_documents(kb_id, ds_id, &del_req.document_identifiers) {
            Ok(docs) => {
                let details: Vec<wire::KnowledgeBaseDocumentDetail> =
                    docs.iter().map(doc_state_to_wire).collect();
                wire::serialize_delete_knowledge_base_documents_response(
                    &wire::DeleteKnowledgeBaseDocumentsResponse {
                        document_details: Some(details),
                    },
                )
            }
            Err(e) => ba_error_response(&e),
        }
    }

    // ─── Flow operations ─────────────────────────────────────────────

    #[allow(clippy::too_many_arguments)]
    async fn handle_create_flow(
        &self,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_flow_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON"),
        };
        let name = if input.name.is_empty() {
            "flow".to_string()
        } else {
            input.name.clone()
        };
        let body = serde_json::to_value(&input).unwrap_or(Value::Null);

        let mut s = state.write().await;
        match s.create_flow(&name, &body, account_id, region) {
            Ok(flow) => wire::serialize_create_flow_response(&wire::CreateFlowResponse { ..flow }),
            Err(e) => ba_error_response(&e),
        }
    }

    async fn handle_list_flows(
        &self,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let _input = match wire::deserialize_list_flows_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON"),
        };
        let s = state.read().await;
        let (summaries, token) = s.list_flows();

        wire::serialize_list_flows_response(&wire::ListFlowsResponse {
            flow_summaries: Some(summaries),
            next_token: token,
        })
    }

    async fn handle_get_flow(
        &self,
        flow_id: &str,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_flow(flow_id) {
            Ok(flow) => wire::serialize_get_flow_response(&wire::GetFlowResponse { ..flow }),
            Err(e) => ba_error_response(&e),
        }
    }

    async fn handle_update_flow(
        &self,
        flow_id: &str,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_flow_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON"),
        };
        let body = serde_json::to_value(&input).unwrap_or(Value::Null);

        let mut s = state.write().await;
        match s.update_flow(flow_id, &body) {
            Ok(flow) => wire::serialize_update_flow_response(&wire::UpdateFlowResponse { ..flow }),
            Err(e) => ba_error_response(&e),
        }
    }

    async fn handle_delete_flow(
        &self,
        flow_id: &str,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.delete_flow(flow_id) {
            Ok(id) => {
                wire::serialize_delete_flow_response(&wire::DeleteFlowResponse { id: Some(id) })
            }
            Err(e) => ba_error_response(&e),
        }
    }

    async fn handle_prepare_flow(
        &self,
        flow_id: &str,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_flow(flow_id) {
            Ok(flow) => wire::serialize_prepare_flow_response(&wire::PrepareFlowResponse {
                id: flow.id,
                status: Some("PREPARED".to_string()),
            }),
            Err(e) => ba_error_response(&e),
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn handle_create_flow_alias(
        &self,
        flow_id: &str,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_flow_alias_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON"),
        };
        let name = if input.name.is_empty() {
            "alias".to_string()
        } else {
            input.name.clone()
        };
        let body = serde_json::to_value(&input).unwrap_or(Value::Null);

        let mut s = state.write().await;
        match s.create_flow_alias(flow_id, &name, &body, account_id, region) {
            Ok(alias) => {
                wire::serialize_create_flow_alias_response(&wire::CreateFlowAliasResponse {
                    ..alias
                })
            }
            Err(e) => ba_error_response(&e),
        }
    }

    async fn handle_list_flow_aliases(
        &self,
        flow_id: &str,
        _request: &MockRequest,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let s = state.read().await;
        let (summaries, token) = s.list_flow_aliases(flow_id);

        wire::serialize_list_flow_aliases_response(&wire::ListFlowAliasesResponse {
            flow_alias_summaries: Some(summaries),
            next_token: token,
        })
    }

    async fn handle_get_flow_alias(
        &self,
        flow_id: &str,
        alias_id: &str,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_flow_alias(flow_id, alias_id) {
            Ok(alias) => {
                wire::serialize_get_flow_alias_response(&wire::GetFlowAliasResponse { ..alias })
            }
            Err(e) => ba_error_response(&e),
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn handle_update_flow_alias(
        &self,
        flow_id: &str,
        alias_id: &str,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_flow_alias_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON"),
        };
        let body = serde_json::to_value(&input).unwrap_or(Value::Null);

        let mut s = state.write().await;
        match s.update_flow_alias(flow_id, alias_id, &body) {
            Ok(alias) => {
                wire::serialize_update_flow_alias_response(&wire::UpdateFlowAliasResponse {
                    ..alias
                })
            }
            Err(e) => ba_error_response(&e),
        }
    }

    async fn handle_delete_flow_alias(
        &self,
        flow_id: &str,
        alias_id: &str,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.delete_flow_alias(flow_id, alias_id) {
            Ok((fid, aid)) => {
                wire::serialize_delete_flow_alias_response(&wire::DeleteFlowAliasResponse {
                    flow_id: Some(fid),
                    id: Some(aid),
                })
            }
            Err(e) => ba_error_response(&e),
        }
    }

    async fn handle_create_flow_version(
        &self,
        flow_id: &str,
        _request: &MockRequest,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.create_flow_version(flow_id) {
            Ok(ver) => {
                wire::serialize_create_flow_version_response(&wire::CreateFlowVersionResponse {
                    ..ver
                })
            }
            Err(e) => ba_error_response(&e),
        }
    }

    async fn handle_list_flow_versions(
        &self,
        flow_id: &str,
        _request: &MockRequest,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let s = state.read().await;
        let (summaries, token) = s.list_flow_versions(flow_id);

        wire::serialize_list_flow_versions_response(&wire::ListFlowVersionsResponse {
            flow_version_summaries: Some(summaries),
            next_token: token,
        })
    }

    async fn handle_get_flow_version(
        &self,
        flow_id: &str,
        version: &str,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_flow_version(flow_id, version) {
            Ok(ver) => {
                wire::serialize_get_flow_version_response(&wire::GetFlowVersionResponse { ..ver })
            }
            Err(e) => ba_error_response(&e),
        }
    }

    async fn handle_delete_flow_version(
        &self,
        flow_id: &str,
        version: &str,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.delete_flow_version(flow_id, version) {
            Ok((fid, ver)) => {
                wire::serialize_delete_flow_version_response(&wire::DeleteFlowVersionResponse {
                    id: Some(fid),
                    version: Some(ver),
                })
            }
            Err(e) => ba_error_response(&e),
        }
    }

    async fn handle_validate_flow_definition(
        &self,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        _state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let input = match wire::deserialize_validate_flow_definition_request(request, labels, query)
        {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON"),
        };
        let definition = serde_json::to_value(&input.definition).unwrap_or(Value::Null);

        let result = match winterbaume_bedrock_flow_validator::parse_and_validate(
            &definition,
            &winterbaume_bedrock_flow_validator::ValidateOptions::default(),
        ) {
            Ok(r) => r,
            Err(_) => {
                // Parse failure — return empty validations (matches AWS behaviour
                // for structurally broken definitions).
                return wire::serialize_validate_flow_definition_response(
                    &wire::ValidateFlowDefinitionResponse {
                        validations: Some(vec![]),
                    },
                );
            }
        };

        let validations: Vec<wire::FlowValidation> = result
            .diagnostics
            .iter()
            .map(map_flow_diagnostic_to_wire)
            .collect();

        wire::serialize_validate_flow_definition_response(&wire::ValidateFlowDefinitionResponse {
            validations: Some(validations),
        })
    }

    // ─── Prompt operations ────────────────────────────────────────────

    #[allow(clippy::too_many_arguments)]
    async fn handle_create_prompt(
        &self,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
        account_id: &str,
        region: &str,
    ) -> MockResponse {
        let input = match wire::deserialize_create_prompt_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON"),
        };
        let name = if input.name.is_empty() {
            "prompt".to_string()
        } else {
            input.name.clone()
        };
        let body = serde_json::to_value(&input).unwrap_or(Value::Null);

        let mut s = state.write().await;
        match s.create_prompt(&name, &body, account_id, region) {
            Ok(prompt) => {
                wire::serialize_create_prompt_response(&wire::CreatePromptResponse { ..prompt })
            }
            Err(e) => ba_error_response(&e),
        }
    }

    async fn handle_list_prompts(
        &self,
        _request: &MockRequest,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let s = state.read().await;
        let (summaries, token) = s.list_prompts();

        wire::serialize_list_prompts_response(&wire::ListPromptsResponse {
            prompt_summaries: Some(summaries),
            next_token: token,
        })
    }

    async fn handle_get_prompt(
        &self,
        prompt_id: &str,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let s = state.read().await;
        match s.get_prompt(prompt_id) {
            Ok(prompt) => {
                wire::serialize_get_prompt_response(&wire::GetPromptResponse { ..prompt })
            }
            Err(e) => ba_error_response(&e),
        }
    }

    async fn handle_update_prompt(
        &self,
        prompt_id: &str,
        request: &MockRequest,
        labels: &[(&str, &str)],
        query: &HashMap<String, String>,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let input = match wire::deserialize_update_prompt_request(request, labels, query) {
            Ok(v) => v,
            Err(_) => return rest_json_error(400, "SerializationException", "Invalid JSON"),
        };
        let body = serde_json::to_value(&input).unwrap_or(Value::Null);

        let mut s = state.write().await;
        match s.update_prompt(prompt_id, &body) {
            Ok(prompt) => {
                wire::serialize_update_prompt_response(&wire::UpdatePromptResponse { ..prompt })
            }
            Err(e) => ba_error_response(&e),
        }
    }

    async fn handle_delete_prompt(
        &self,
        prompt_id: &str,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.delete_prompt(prompt_id) {
            Ok((id, version)) => {
                wire::serialize_delete_prompt_response(&wire::DeletePromptResponse {
                    id: Some(id),
                    version,
                })
            }
            Err(e) => ba_error_response(&e),
        }
    }

    async fn handle_create_prompt_version(
        &self,
        prompt_id: &str,
        _request: &MockRequest,
        state: &Arc<tokio::sync::RwLock<BedrockAgentState>>,
    ) -> MockResponse {
        let mut s = state.write().await;
        match s.create_prompt_version(prompt_id) {
            Ok(ver) => {
                wire::serialize_create_prompt_version_response(&wire::CreatePromptVersionResponse {
                    ..ver
                })
            }
            Err(e) => ba_error_response(&e),
        }
    }
}

// ─── State-to-wire helpers ──────────────────────────────────────────

fn agent_to_wire(agent: &Agent) -> wire::Agent {
    wire::Agent {
        agent_id: Some(agent.agent_id.clone()),
        agent_name: Some(agent.agent_name.clone()),
        agent_arn: Some(agent.agent_arn.clone()),
        agent_version: Some(agent.agent_version.clone()),
        agent_status: Some(agent.agent_status.clone()),
        agent_resource_role_arn: Some(agent.agent_resource_role_arn.clone()),
        created_at: Some(agent.created_at.clone()),
        updated_at: Some(agent.updated_at.clone()),
        prepared_at: Some(agent.prepared_at.clone()),
        failure_reasons: Some(agent.failure_reasons.clone()),
        recommended_actions: Some(agent.recommended_actions.clone()),
        foundation_model: agent.foundation_model.clone(),
        instruction: agent.instruction.clone(),
        description: agent.description.clone(),
        idle_session_t_t_l_in_seconds: agent.idle_session_ttl_in_seconds.map(|v| v as i32),
        customer_encryption_key_arn: agent.customer_encryption_key_arn.clone(),
        client_token: agent.client_token.clone(),
        // Always include an empty promptOverrideConfiguration so that the
        // Terraform AWS provider does not nil-dereference in removeDefaultPrompts.
        prompt_override_configuration: Some(wire::PromptOverrideConfiguration {
            prompt_configurations: vec![],
            override_lambda: None,
        }),
        ..Default::default()
    }
}

fn agent_summary_from_value(v: &Value) -> wire::AgentSummary {
    wire::AgentSummary {
        agent_id: v
            .get("agentId")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        agent_name: v
            .get("agentName")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        agent_status: v
            .get("agentStatus")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        updated_at: v
            .get("updatedAt")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        latest_agent_version: v
            .get("latestAgentVersion")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        description: v
            .get("description")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        guardrail_configuration: None,
    }
}

fn agent_alias_to_wire(alias: &AgentAliasState) -> wire::AgentAlias {
    wire::AgentAlias {
        agent_id: Some(alias.agent_id.clone()),
        agent_alias_id: Some(alias.agent_alias_id.clone()),
        agent_alias_name: Some(alias.agent_alias_name.clone()),
        agent_alias_arn: Some(alias.agent_alias_arn.clone()),
        agent_alias_status: Some(alias.agent_alias_status.clone()),
        created_at: Some(alias.created_at.clone()),
        updated_at: Some(alias.updated_at.clone()),
        description: alias.description.clone(),
        ..Default::default()
    }
}

fn knowledge_base_to_wire(kb: &KnowledgeBase) -> wire::KnowledgeBase {
    let knowledge_base_configuration: Option<wire::KnowledgeBaseConfiguration> =
        serde_json::from_value(kb.knowledge_base_configuration.clone()).ok();
    let storage_configuration: Option<wire::StorageConfiguration> =
        serde_json::from_value(kb.storage_configuration.clone()).ok();
    wire::KnowledgeBase {
        knowledge_base_id: Some(kb.knowledge_base_id.clone()),
        name: Some(kb.name.clone()),
        knowledge_base_arn: Some(kb.knowledge_base_arn.clone()),
        role_arn: Some(kb.role_arn.clone()),
        status: Some(kb.status.clone()),
        created_at: Some(kb.created_at.clone()),
        updated_at: Some(kb.updated_at.clone()),
        failure_reasons: Some(kb.failure_reasons.clone()),
        description: kb.description.clone(),
        knowledge_base_configuration,
        storage_configuration,
        ..Default::default()
    }
}

fn kb_summary_from_value(v: &Value) -> wire::KnowledgeBaseSummary {
    wire::KnowledgeBaseSummary {
        knowledge_base_id: v
            .get("knowledgeBaseId")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        name: v
            .get("name")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        status: v
            .get("status")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        updated_at: v
            .get("updatedAt")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        description: v
            .get("description")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
    }
}

// ─── URI helpers ────────────────────────────────────────────────────

fn extract_query(uri: &str) -> Vec<(String, String)> {
    let mut pairs = Vec::new();
    if let Some(pos) = uri.find('?') {
        let query = &uri[pos + 1..];
        for pair in query.split('&') {
            if let Some(eq_pos) = pair.find('=') {
                let key = urldecode(&pair[..eq_pos]);
                let value = urldecode(&pair[eq_pos + 1..]);
                pairs.push((key, value));
            }
        }
    }
    pairs
}

fn urldecode(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.bytes();
    while let Some(b) = chars.next() {
        if b == b'%' {
            let h1 = chars.next().unwrap_or(0);
            let h2 = chars.next().unwrap_or(0);
            let hex = [h1, h2];
            if let Ok(hex_str) = std::str::from_utf8(&hex)
                && let Ok(val) = u8::from_str_radix(hex_str, 16)
            {
                result.push(val as char);
                continue;
            }
            result.push('%');
            result.push(h1 as char);
            result.push(h2 as char);
        } else {
            result.push(b as char);
        }
    }
    result
}

// ─── Error helpers ──────────────────────────────────────────────────

fn ba_error_response(err: &BedrockAgentError) -> MockResponse {
    let (status, error_type) = match err {
        BedrockAgentError::AgentNotFound(_) => (404, "ResourceNotFoundException"),
        BedrockAgentError::AgentAliasNotFound(_) => (404, "ResourceNotFoundException"),
        BedrockAgentError::ActionGroupNotFound(_) => (404, "ResourceNotFoundException"),
        BedrockAgentError::AgentKnowledgeBaseNotFound(_) => (404, "ResourceNotFoundException"),
        BedrockAgentError::CollaboratorNotFound(_) => (404, "ResourceNotFoundException"),
        BedrockAgentError::InvalidKnowledgeBaseType(_) => (400, "ValidationException"),
        BedrockAgentError::InvalidStorageType(_) => (400, "ValidationException"),
        BedrockAgentError::KnowledgeBaseNotFound(_) => (404, "ResourceNotFoundException"),
        BedrockAgentError::DataSourceNotFound(_) => (404, "ResourceNotFoundException"),
        BedrockAgentError::IngestionJobNotFound(_) => (404, "ResourceNotFoundException"),
        BedrockAgentError::FlowNotFound(_) => (404, "ResourceNotFoundException"),
        BedrockAgentError::FlowAliasNotFound(_) => (404, "ResourceNotFoundException"),
        BedrockAgentError::FlowVersionNotFound(_) => (404, "ResourceNotFoundException"),
        BedrockAgentError::PromptNotFound(_) => (404, "ResourceNotFoundException"),
        BedrockAgentError::ResourceNotFound(_) => (404, "ResourceNotFoundException"),
    };
    let msg = err.to_string();
    let body = json!({
        "__type": error_type,
        "message": msg,
        "Message": msg,
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers
        .insert(X_AMZN_ERRORTYPE, error_type.parse().unwrap());
    resp
}

fn ba_json_error(status: u16, code: &str, message: &str) -> MockResponse {
    rest_json_error(status, code, message)
}

fn doc_state_to_wire(
    doc: &crate::state::KnowledgeBaseDocumentState,
) -> wire::KnowledgeBaseDocumentDetail {
    let identifier = wire::DocumentIdentifier {
        data_source_type: doc.data_source_type.clone(),
        custom: doc
            .identifier_custom_id
            .as_ref()
            .map(|id| wire::CustomDocumentIdentifier { id: id.clone() }),
        s3: doc
            .identifier_uri
            .as_ref()
            .map(|uri| wire::S3Location { uri: uri.clone() }),
    };
    wire::KnowledgeBaseDocumentDetail {
        knowledge_base_id: Some(doc.knowledge_base_id.clone()),
        data_source_id: Some(doc.data_source_id.clone()),
        identifier: Some(identifier),
        status: Some(doc.status.clone()),
        updated_at: Some(doc.updated_at.clone()),
        status_reason: None,
    }
}

fn rest_json_error(status: u16, code: &str, message: &str) -> MockResponse {
    let body = json!({
        "__type": code,
        "message": message,
        "Message": message,
    });
    let mut resp = MockResponse::rest_json(status, body.to_string());
    resp.headers.insert(X_AMZN_ERRORTYPE, code.parse().unwrap());
    resp
}

// ─── Flow validation mapping ───────────────────────────────────────────

fn map_flow_diagnostic_to_wire(
    d: &winterbaume_bedrock_flow_validator::Diagnostic,
) -> wire::FlowValidation {
    use winterbaume_bedrock_flow_validator::DiagnosticDetails;

    let mut details = wire::FlowValidationDetails::default();

    match &d.details {
        DiagnosticDetails::CyclicConnection { connection } => {
            details.cyclic_connection = Some(wire::CyclicConnectionFlowValidationDetails {
                connection: connection.clone(),
            });
        }
        DiagnosticDetails::DuplicateConnections { source, target } => {
            details.duplicate_connections = Some(wire::DuplicateConnectionsFlowValidationDetails {
                source: source.clone(),
                target: target.clone(),
            });
        }
        DiagnosticDetails::DuplicateConditionExpression { node, expression } => {
            details.duplicate_condition_expression =
                Some(wire::DuplicateConditionExpressionFlowValidationDetails {
                    node: node.clone(),
                    expression: expression.clone(),
                });
        }
        DiagnosticDetails::IncompatibleConnectionDataType { connection } => {
            details.incompatible_connection_data_type =
                Some(wire::IncompatibleConnectionDataTypeFlowValidationDetails {
                    connection: connection.clone(),
                });
        }
        DiagnosticDetails::MissingConnectionConfiguration { connection } => {
            details.missing_connection_configuration =
                Some(wire::MissingConnectionConfigurationFlowValidationDetails {
                    connection: connection.clone(),
                });
        }
        DiagnosticDetails::MissingDefaultCondition { node } => {
            details.missing_default_condition =
                Some(wire::MissingDefaultConditionFlowValidationDetails { node: node.clone() });
        }
        DiagnosticDetails::MissingEndingNodes => {
            details.missing_ending_nodes = Some(wire::MissingEndingNodesFlowValidationDetails {});
        }
        DiagnosticDetails::MissingNodeConfiguration { node } => {
            details.missing_node_configuration =
                Some(wire::MissingNodeConfigurationFlowValidationDetails { node: node.clone() });
        }
        DiagnosticDetails::MissingNodeInput { node, input } => {
            details.missing_node_input = Some(wire::MissingNodeInputFlowValidationDetails {
                node: node.clone(),
                input: input.clone(),
            });
        }
        DiagnosticDetails::MissingNodeOutput { node, output } => {
            details.missing_node_output = Some(wire::MissingNodeOutputFlowValidationDetails {
                node: node.clone(),
                output: output.clone(),
            });
        }
        DiagnosticDetails::MissingStartingNodes => {
            details.missing_starting_nodes =
                Some(wire::MissingStartingNodesFlowValidationDetails {});
        }
        DiagnosticDetails::MismatchedNodeInputType {
            node,
            input,
            expected_type,
        } => {
            details.mismatched_node_input_type =
                Some(wire::MismatchedNodeInputTypeFlowValidationDetails {
                    node: node.clone(),
                    input: input.clone(),
                    expected_type: expected_type.clone(),
                });
        }
        DiagnosticDetails::MismatchedNodeOutputType {
            node,
            output,
            expected_type,
        } => {
            details.mismatched_node_output_type =
                Some(wire::MismatchedNodeOutputTypeFlowValidationDetails {
                    node: node.clone(),
                    output: output.clone(),
                    expected_type: expected_type.clone(),
                });
        }
        DiagnosticDetails::MultipleNodeInputConnections { node, input } => {
            details.multiple_node_input_connections =
                Some(wire::MultipleNodeInputConnectionsFlowValidationDetails {
                    node: node.clone(),
                    input: input.clone(),
                });
        }
        DiagnosticDetails::UnfulfilledNodeInput { node, input } => {
            details.unfulfilled_node_input =
                Some(wire::UnfulfilledNodeInputFlowValidationDetails {
                    node: node.clone(),
                    input: input.clone(),
                });
        }
        DiagnosticDetails::UnknownConnectionCondition { connection } => {
            details.unknown_connection_condition =
                Some(wire::UnknownConnectionConditionFlowValidationDetails {
                    connection: connection.clone(),
                });
        }
        DiagnosticDetails::UnknownConnectionSource { connection } => {
            details.unknown_connection_source =
                Some(wire::UnknownConnectionSourceFlowValidationDetails {
                    connection: connection.clone(),
                });
        }
        DiagnosticDetails::UnknownConnectionSourceOutput { connection } => {
            details.unknown_connection_source_output =
                Some(wire::UnknownConnectionSourceOutputFlowValidationDetails {
                    connection: connection.clone(),
                });
        }
        DiagnosticDetails::UnknownConnectionTarget { connection } => {
            details.unknown_connection_target =
                Some(wire::UnknownConnectionTargetFlowValidationDetails {
                    connection: connection.clone(),
                });
        }
        DiagnosticDetails::UnknownConnectionTargetInput { connection } => {
            details.unknown_connection_target_input =
                Some(wire::UnknownConnectionTargetInputFlowValidationDetails {
                    connection: connection.clone(),
                });
        }
        DiagnosticDetails::UnknownNodeInput { node, input } => {
            details.unknown_node_input = Some(wire::UnknownNodeInputFlowValidationDetails {
                node: node.clone(),
                input: input.clone(),
            });
        }
        DiagnosticDetails::UnknownNodeOutput { node, output } => {
            details.unknown_node_output = Some(wire::UnknownNodeOutputFlowValidationDetails {
                node: node.clone(),
                output: output.clone(),
            });
        }
        DiagnosticDetails::UnreachableNode { node } => {
            details.unreachable_node =
                Some(wire::UnreachableNodeFlowValidationDetails { node: node.clone() });
        }
        DiagnosticDetails::UnsatisfiedConnectionConditions { connection } => {
            details.unsatisfied_connection_conditions =
                Some(wire::UnsatisfiedConnectionConditionsFlowValidationDetails {
                    connection: connection.clone(),
                });
        }
        DiagnosticDetails::MissingLoopInputNode { loop_node } => {
            details.missing_loop_input_node =
                Some(wire::MissingLoopInputNodeFlowValidationDetails {
                    loop_node: loop_node.clone(),
                });
        }
        DiagnosticDetails::MissingLoopControllerNode { loop_node } => {
            details.missing_loop_controller_node =
                Some(wire::MissingLoopControllerNodeFlowValidationDetails {
                    loop_node: loop_node.clone(),
                });
        }
        DiagnosticDetails::MultipleLoopInputNodes { loop_node } => {
            details.multiple_loop_input_nodes =
                Some(wire::MultipleLoopInputNodesFlowValidationDetails {
                    loop_node: loop_node.clone(),
                });
        }
        DiagnosticDetails::MultipleLoopControllerNodes { loop_node } => {
            details.multiple_loop_controller_nodes =
                Some(wire::MultipleLoopControllerNodesFlowValidationDetails {
                    loop_node: loop_node.clone(),
                });
        }
        DiagnosticDetails::LoopIncompatibleNodeType {
            node,
            incompatible_node_name,
            incompatible_node_type,
        } => {
            details.loop_incompatible_node_type =
                Some(wire::LoopIncompatibleNodeTypeFlowValidationDetails {
                    node: node.clone(),
                    incompatible_node_name: incompatible_node_name.clone(),
                    incompatible_node_type: incompatible_node_type.clone(),
                });
        }
        DiagnosticDetails::InvalidLoopBoundary {
            connection,
            source,
            target,
        } => {
            details.invalid_loop_boundary = Some(wire::InvalidLoopBoundaryFlowValidationDetails {
                connection: connection.clone(),
                source: source.clone(),
                target: target.clone(),
            });
        }
        DiagnosticDetails::Unspecified => {
            details.unspecified = Some(wire::UnspecifiedFlowValidationDetails {});
        }
    }

    wire::FlowValidation {
        r#type: Some(d.code.to_string()),
        severity: Some(d.severity.to_string()),
        message: Some(d.message.clone()),
        details: Some(details),
    }
}
