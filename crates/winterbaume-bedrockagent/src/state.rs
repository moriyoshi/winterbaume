use std::collections::HashMap;

use crate::model::{
    ActionGroupSummary, AgentActionGroup, AgentAliasSummary, AgentCollaborator,
    AgentCollaboratorSummary, AgentKnowledgeBase, AgentKnowledgeBaseSummary, AgentVersionSummary,
    CreateFlowAliasResponse, CreateFlowResponse, CreateFlowVersionResponse, CreatePromptResponse,
    CreatePromptVersionResponse, DataSource, DataSourceConfiguration, DataSourceSummary,
    FlowAliasSummary, FlowSummary, FlowVersionSummary, GetFlowAliasResponse, GetFlowResponse,
    GetFlowVersionResponse, GetPromptResponse, IngestionJob, IngestionJobSummary, PromptSummary,
    UpdateFlowAliasResponse, UpdateFlowResponse, UpdatePromptResponse,
    VectorIngestionConfiguration,
};
use crate::types::{Agent, KnowledgeBase, TagStore};

const VALID_STORAGE_TYPES: &[&str] = &[
    "OPENSEARCH_SERVERLESS",
    "PINECONE",
    "REDIS_ENTERPRISE_CLOUD",
    "RDS",
];

#[derive(Debug, Default)]
pub struct BedrockAgentState {
    pub agents: HashMap<String, Agent>,
    pub knowledge_bases: HashMap<String, KnowledgeBase>,
    pub tags: TagStore,
    // Extended state for new operations
    pub agent_aliases: HashMap<String, Vec<AgentAliasState>>,
    pub agent_action_groups: HashMap<String, Vec<AgentActionGroup>>,
    pub agent_knowledge_bases: HashMap<String, Vec<AgentKnowledgeBase>>,
    pub agent_collaborators: HashMap<String, Vec<AgentCollaborator>>,
    pub data_sources: HashMap<String, Vec<DataSource>>,
    pub ingestion_jobs: HashMap<String, Vec<IngestionJob>>,
    pub flows: HashMap<String, FlowState>,
    pub flow_aliases: HashMap<String, Vec<FlowAliasState>>,
    pub flow_versions: HashMap<String, Vec<FlowVersionState>>,
    pub prompts: HashMap<String, PromptState>,
    /// Documents keyed by "{kb_id}:{ds_id}" -> list of document details.
    pub kb_documents: HashMap<String, Vec<KnowledgeBaseDocumentState>>,
}

/// Per-document state tracked in mock for knowledge base data source documents.
#[derive(Debug, Clone)]
pub struct KnowledgeBaseDocumentState {
    pub knowledge_base_id: String,
    pub data_source_id: String,
    pub data_source_type: String,
    pub identifier_uri: Option<String>,
    pub identifier_custom_id: Option<String>,
    pub status: String,
    pub updated_at: String,
}

#[derive(Debug, Clone)]
pub struct AgentAliasState {
    pub agent_id: String,
    pub agent_alias_id: String,
    pub agent_alias_name: String,
    pub agent_alias_arn: String,
    pub agent_alias_status: String,
    pub created_at: String,
    pub updated_at: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct FlowState {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub description: Option<String>,
    pub execution_role_arn: Option<String>,
}

#[derive(Debug, Clone)]
pub struct FlowAliasState {
    pub flow_id: String,
    pub id: String,
    pub arn: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone)]
pub struct FlowVersionState {
    pub flow_id: String,
    pub version: String,
    pub arn: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Clone)]
pub struct PromptState {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
    pub description: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum BedrockAgentError {
    #[error("Agent {0} not found")]
    AgentNotFound(String),
    #[error("Agent alias {0} not found")]
    AgentAliasNotFound(String),
    #[error("Action group {0} not found")]
    ActionGroupNotFound(String),
    #[error("Agent knowledge base {0} not found")]
    AgentKnowledgeBaseNotFound(String),
    #[error("Collaborator {0} not found")]
    CollaboratorNotFound(String),
    #[error(
        "Validation error detected: Value '{0}' at 'knowledgeBaseConfiguration' failed to satisfy constraint: Member must contain 'type' as 'VECTOR'"
    )]
    InvalidKnowledgeBaseType(String),
    #[error(
        "Validation error detected: Value '{0}' at 'storageConfiguration' failed to satisfy constraint: Member 'type' must be one of: OPENSEARCH_SERVERLESS | PINECONE | REDIS_ENTERPRISE_CLOUD | RDS"
    )]
    InvalidStorageType(String),
    #[error("Knowledge base {0} not found")]
    KnowledgeBaseNotFound(String),
    #[error("Data source {0} not found")]
    DataSourceNotFound(String),
    #[error("Ingestion job {0} not found")]
    IngestionJobNotFound(String),
    #[error("Flow {0} not found")]
    FlowNotFound(String),
    #[error("Flow alias {0} not found")]
    FlowAliasNotFound(String),
    #[error("Flow version {0} not found")]
    FlowVersionNotFound(String),
    #[error("Prompt {0} not found")]
    PromptNotFound(String),
    #[error("Resource {0} not found")]
    ResourceNotFound(String),
}

fn now_str() -> String {
    chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}

fn short_id() -> String {
    uuid::Uuid::new_v4().to_string()[..8].to_string()
}

// Key for agent action groups / knowledge bases / collaborators
// "{agent_id}:{agent_version}"
fn agent_version_key(agent_id: &str, agent_version: &str) -> String {
    format!("{agent_id}:{agent_version}")
}

// Key for data sources / ingestion jobs: "{kb_id}:{ds_id}"
fn kb_ds_key(kb_id: &str, ds_id: &str) -> String {
    format!("{kb_id}:{ds_id}")
}

impl BedrockAgentState {
    fn list_arns(&self) -> Vec<String> {
        let mut arns: Vec<String> = self.agents.values().map(|a| a.agent_arn.clone()).collect();
        arns.extend(
            self.knowledge_bases
                .values()
                .map(|kb| kb.knowledge_base_arn.clone()),
        );
        for flow in self.flows.values() {
            arns.push(flow.arn.clone());
        }
        for prompt in self.prompts.values() {
            arns.push(prompt.arn.clone());
        }
        for aliases in self.agent_aliases.values() {
            for alias in aliases {
                arns.push(alias.agent_alias_arn.clone());
            }
        }
        for aliases in self.flow_aliases.values() {
            for alias in aliases {
                arns.push(alias.arn.clone());
            }
        }
        arns
    }

    // ─── Agent operations ────────────────────────────────────────────

    pub fn create_agent(
        &mut self,
        agent_name: &str,
        agent_resource_role_arn: &str,
        account_id: &str,
        region: &str,
        client_token: Option<&str>,
        instruction: Option<&str>,
        foundation_model: Option<&str>,
        description: Option<&str>,
        idle_session_ttl_in_seconds: Option<i64>,
        customer_encryption_key_arn: Option<&str>,
    ) -> &Agent {
        let now = now_str();
        let short_uuid = short_id();
        let agent_id = format!("{agent_name}{short_uuid}");
        let agent_arn = format!("arn:aws:bedrock:{region}:{account_id}:agent/{agent_id}");

        let agent = Agent {
            agent_id: agent_id.clone(),
            agent_name: agent_name.to_string(),
            agent_arn,
            agent_version: "1.0".to_string(),
            client_token: client_token.map(|s| s.to_string()),
            instruction: instruction.map(|s| s.to_string()),
            agent_status: "NOT_PREPARED".to_string(),
            foundation_model: foundation_model.map(|s| s.to_string()),
            description: description.map(|s| s.to_string()),
            idle_session_ttl_in_seconds,
            agent_resource_role_arn: agent_resource_role_arn.to_string(),
            customer_encryption_key_arn: customer_encryption_key_arn.map(|s| s.to_string()),
            created_at: now.clone(),
            updated_at: now.clone(),
            prepared_at: now,
            failure_reasons: vec![],
            recommended_actions: vec!["action".to_string()],
        };

        self.agents.insert(agent_id.clone(), agent);
        self.agents.get(&agent_id).unwrap()
    }

    pub fn get_agent(&self, agent_id: &str) -> Result<&Agent, BedrockAgentError> {
        self.agents
            .get(agent_id)
            .ok_or_else(|| BedrockAgentError::AgentNotFound(agent_id.to_string()))
    }

    pub fn prepare_agent(&mut self, agent_id: &str) -> Result<&Agent, BedrockAgentError> {
        let agent = self
            .agents
            .get_mut(agent_id)
            .ok_or_else(|| BedrockAgentError::AgentNotFound(agent_id.to_string()))?;
        agent.agent_status = "PREPARED".to_string();
        agent.prepared_at = now_str();
        Ok(self.agents.get(agent_id).unwrap())
    }

    pub fn update_agent(
        &mut self,
        agent_id: &str,
        body: &serde_json::Value,
    ) -> Result<&Agent, BedrockAgentError> {
        let agent = self
            .agents
            .get_mut(agent_id)
            .ok_or_else(|| BedrockAgentError::AgentNotFound(agent_id.to_string()))?;

        if let Some(v) = body.get("agentName").and_then(|v| v.as_str()) {
            agent.agent_name = v.to_string();
        }
        if let Some(v) = body.get("agentResourceRoleArn").and_then(|v| v.as_str()) {
            agent.agent_resource_role_arn = v.to_string();
        }
        if let Some(v) = body.get("instruction").and_then(|v| v.as_str()) {
            agent.instruction = Some(v.to_string());
        }
        if let Some(v) = body.get("foundationModel").and_then(|v| v.as_str()) {
            agent.foundation_model = Some(v.to_string());
        }
        if let Some(v) = body.get("description").and_then(|v| v.as_str()) {
            agent.description = Some(v.to_string());
        }
        agent.updated_at = now_str();
        Ok(self.agents.get(agent_id).unwrap())
    }

    pub fn list_agents(
        &self,
        max_results: Option<i64>,
        next_token: Option<&str>,
    ) -> (Vec<serde_json::Value>, Option<String>) {
        let limit = max_results.unwrap_or(100) as usize;
        let start = next_token
            .and_then(|t| t.parse::<usize>().ok())
            .unwrap_or(0);

        let all: Vec<serde_json::Value> = self.agents.values().map(|a| a.dict_summary()).collect();

        let total = all.len();
        let end = (start + limit).min(total);
        let page = all[start..end].to_vec();

        let new_token = if end < total {
            Some(end.to_string())
        } else {
            None
        };

        (page, new_token)
    }

    pub fn delete_agent(
        &mut self,
        agent_id: &str,
        _skip_resource_in_use_check: bool,
    ) -> Result<(String, String), BedrockAgentError> {
        if let Some(mut agent) = self.agents.remove(agent_id) {
            agent.agent_status = "DELETING".to_string();
            Ok((agent.agent_id, agent.agent_status))
        } else {
            Err(BedrockAgentError::AgentNotFound(agent_id.to_string()))
        }
    }

    // ─── Agent Alias operations ──────────────────────────────────────

    pub fn create_agent_alias(
        &mut self,
        agent_id: &str,
        name: &str,
        description: Option<&str>,
        account_id: &str,
        region: &str,
    ) -> Result<&AgentAliasState, BedrockAgentError> {
        if !self.agents.contains_key(agent_id) {
            return Err(BedrockAgentError::AgentNotFound(agent_id.to_string()));
        }
        let now = now_str();
        let alias_id = short_id();
        let alias_arn =
            format!("arn:aws:bedrock:{region}:{account_id}:agent-alias/{agent_id}/{alias_id}");

        let alias = AgentAliasState {
            agent_id: agent_id.to_string(),
            agent_alias_id: alias_id.clone(),
            agent_alias_name: name.to_string(),
            agent_alias_arn: alias_arn,
            agent_alias_status: "PREPARED".to_string(),
            created_at: now.clone(),
            updated_at: now,
            description: description.map(|s| s.to_string()),
        };

        let aliases = self.agent_aliases.entry(agent_id.to_string()).or_default();
        aliases.push(alias);
        let idx = aliases.len() - 1;
        Ok(&self.agent_aliases[agent_id][idx])
    }

    pub fn get_agent_alias(
        &self,
        agent_id: &str,
        alias_id: &str,
    ) -> Result<&AgentAliasState, BedrockAgentError> {
        self.agent_aliases
            .get(agent_id)
            .and_then(|aliases| aliases.iter().find(|a| a.agent_alias_id == alias_id))
            .ok_or_else(|| BedrockAgentError::AgentAliasNotFound(alias_id.to_string()))
    }

    pub fn update_agent_alias(
        &mut self,
        agent_id: &str,
        alias_id: &str,
        name: &str,
    ) -> Result<&AgentAliasState, BedrockAgentError> {
        let aliases = self
            .agent_aliases
            .get_mut(agent_id)
            .ok_or_else(|| BedrockAgentError::AgentAliasNotFound(alias_id.to_string()))?;
        let alias = aliases
            .iter_mut()
            .find(|a| a.agent_alias_id == alias_id)
            .ok_or_else(|| BedrockAgentError::AgentAliasNotFound(alias_id.to_string()))?;
        alias.agent_alias_name = name.to_string();
        alias.updated_at = now_str();
        let _ = alias;
        Ok(self.agent_aliases[agent_id]
            .iter()
            .find(|a| a.agent_alias_id == alias_id)
            .unwrap())
    }

    pub fn delete_agent_alias(
        &mut self,
        agent_id: &str,
        alias_id: &str,
    ) -> Result<(String, String, String), BedrockAgentError> {
        let aliases = self
            .agent_aliases
            .get_mut(agent_id)
            .ok_or_else(|| BedrockAgentError::AgentAliasNotFound(alias_id.to_string()))?;
        let pos = aliases
            .iter()
            .position(|a| a.agent_alias_id == alias_id)
            .ok_or_else(|| BedrockAgentError::AgentAliasNotFound(alias_id.to_string()))?;
        let alias = aliases.remove(pos);
        Ok((alias.agent_id, alias.agent_alias_id, "DELETING".to_string()))
    }

    pub fn list_agent_aliases(
        &self,
        agent_id: &str,
        max_results: Option<i64>,
        _next_token: Option<&str>,
    ) -> (Vec<AgentAliasSummary>, Option<String>) {
        let aliases = self
            .agent_aliases
            .get(agent_id)
            .cloned()
            .unwrap_or_default();
        let limit = max_results.unwrap_or(100) as usize;
        let summaries: Vec<AgentAliasSummary> = aliases
            .iter()
            .take(limit)
            .map(|a| AgentAliasSummary {
                agent_alias_id: Some(a.agent_alias_id.clone()),
                agent_alias_name: Some(a.agent_alias_name.clone()),
                agent_alias_status: Some(a.agent_alias_status.clone()),
                created_at: Some(a.created_at.clone()),
                updated_at: Some(a.updated_at.clone()),
                ..Default::default()
            })
            .collect();
        (summaries, None)
    }

    // ─── Agent Version operations ────────────────────────────────────

    pub fn list_agent_versions(
        &self,
        agent_id: &str,
        max_results: Option<i64>,
        _next_token: Option<&str>,
    ) -> (Vec<AgentVersionSummary>, Option<String>) {
        let limit = max_results.unwrap_or(100) as usize;
        if let Some(agent) = self.agents.get(agent_id) {
            let summaries = vec![AgentVersionSummary {
                agent_name: Some(agent.agent_name.clone()),
                agent_status: Some(agent.agent_status.clone()),
                agent_version: Some(agent.agent_version.clone()),
                created_at: Some(agent.created_at.clone()),
                description: agent.description.clone(),
                ..Default::default()
            }];
            let result: Vec<_> = summaries.into_iter().take(limit).collect();
            (result, None)
        } else {
            (vec![], None)
        }
    }

    // ─── Agent Action Group operations ───────────────────────────────

    pub fn create_agent_action_group(
        &mut self,
        agent_id: &str,
        agent_version: &str,
        name: &str,
        _body: &serde_json::Value,
        _account_id: &str,
        _region: &str,
    ) -> Result<AgentActionGroup, BedrockAgentError> {
        if !self.agents.contains_key(agent_id) {
            return Err(BedrockAgentError::AgentNotFound(agent_id.to_string()));
        }
        let now = now_str();
        let ag_id = short_id();
        let ag = AgentActionGroup {
            agent_id: Some(agent_id.to_string()),
            agent_version: Some(agent_version.to_string()),
            action_group_id: Some(ag_id.clone()),
            action_group_name: Some(name.to_string()),
            action_group_state: Some("ENABLED".to_string()),
            created_at: Some(now.clone()),
            updated_at: Some(now),
            ..Default::default()
        };
        let key = agent_version_key(agent_id, agent_version);
        self.agent_action_groups
            .entry(key)
            .or_default()
            .push(ag.clone());
        Ok(ag)
    }

    pub fn get_agent_action_group(
        &self,
        agent_id: &str,
        agent_version: &str,
        action_group_id: &str,
    ) -> Result<AgentActionGroup, BedrockAgentError> {
        let key = agent_version_key(agent_id, agent_version);
        self.agent_action_groups
            .get(&key)
            .and_then(|v| {
                v.iter()
                    .find(|ag| ag.action_group_id.as_deref() == Some(action_group_id))
            })
            .cloned()
            .ok_or_else(|| BedrockAgentError::ActionGroupNotFound(action_group_id.to_string()))
    }

    pub fn update_agent_action_group(
        &mut self,
        agent_id: &str,
        agent_version: &str,
        action_group_id: &str,
        body: &serde_json::Value,
    ) -> Result<AgentActionGroup, BedrockAgentError> {
        let key = agent_version_key(agent_id, agent_version);
        let groups = self
            .agent_action_groups
            .get_mut(&key)
            .ok_or_else(|| BedrockAgentError::ActionGroupNotFound(action_group_id.to_string()))?;
        let ag = groups
            .iter_mut()
            .find(|ag| ag.action_group_id.as_deref() == Some(action_group_id))
            .ok_or_else(|| BedrockAgentError::ActionGroupNotFound(action_group_id.to_string()))?;
        if let Some(v) = body.get("actionGroupName").and_then(|v| v.as_str()) {
            ag.action_group_name = Some(v.to_string());
        }
        ag.updated_at = Some(now_str());
        Ok(ag.clone())
    }

    pub fn delete_agent_action_group(
        &mut self,
        agent_id: &str,
        agent_version: &str,
        action_group_id: &str,
    ) -> Result<(), BedrockAgentError> {
        let key = agent_version_key(agent_id, agent_version);
        if let Some(groups) = self.agent_action_groups.get_mut(&key) {
            if let Some(pos) = groups
                .iter()
                .position(|ag| ag.action_group_id.as_deref() == Some(action_group_id))
            {
                groups.remove(pos);
                return Ok(());
            }
        }
        Err(BedrockAgentError::ActionGroupNotFound(
            action_group_id.to_string(),
        ))
    }

    pub fn list_agent_action_groups(
        &self,
        agent_id: &str,
        agent_version: &str,
        max_results: Option<i64>,
        _next_token: Option<&str>,
    ) -> (Vec<ActionGroupSummary>, Option<String>) {
        let key = agent_version_key(agent_id, agent_version);
        let groups = self
            .agent_action_groups
            .get(&key)
            .cloned()
            .unwrap_or_default();
        let limit = max_results.unwrap_or(100) as usize;
        let summaries: Vec<ActionGroupSummary> = groups
            .iter()
            .take(limit)
            .map(|ag| ActionGroupSummary {
                action_group_id: ag.action_group_id.clone(),
                action_group_name: ag.action_group_name.clone(),
                action_group_state: ag.action_group_state.clone(),
                description: ag.description.clone(),
                updated_at: ag.updated_at.clone(),
            })
            .collect();
        (summaries, None)
    }

    // ─── Agent KnowledgeBase association operations ──────────────────

    pub fn associate_agent_knowledge_base(
        &mut self,
        agent_id: &str,
        agent_version: &str,
        kb_id: &str,
        description: &str,
    ) -> Result<AgentKnowledgeBase, BedrockAgentError> {
        if !self.agents.contains_key(agent_id) {
            return Err(BedrockAgentError::AgentNotFound(agent_id.to_string()));
        }
        let now = now_str();
        let akb = AgentKnowledgeBase {
            agent_id: Some(agent_id.to_string()),
            agent_version: Some(agent_version.to_string()),
            knowledge_base_id: Some(kb_id.to_string()),
            description: Some(description.to_string()),
            knowledge_base_state: Some("ENABLED".to_string()),
            created_at: Some(now.clone()),
            updated_at: Some(now),
        };
        let key = agent_version_key(agent_id, agent_version);
        self.agent_knowledge_bases
            .entry(key)
            .or_default()
            .push(akb.clone());
        Ok(akb)
    }

    pub fn get_agent_knowledge_base(
        &self,
        agent_id: &str,
        agent_version: &str,
        kb_id: &str,
    ) -> Result<AgentKnowledgeBase, BedrockAgentError> {
        let key = agent_version_key(agent_id, agent_version);
        self.agent_knowledge_bases
            .get(&key)
            .and_then(|v| {
                v.iter()
                    .find(|akb| akb.knowledge_base_id.as_deref() == Some(kb_id))
            })
            .cloned()
            .ok_or_else(|| BedrockAgentError::AgentKnowledgeBaseNotFound(kb_id.to_string()))
    }

    pub fn update_agent_knowledge_base(
        &mut self,
        agent_id: &str,
        agent_version: &str,
        kb_id: &str,
        body: &serde_json::Value,
    ) -> Result<AgentKnowledgeBase, BedrockAgentError> {
        let key = agent_version_key(agent_id, agent_version);
        let kbs = self
            .agent_knowledge_bases
            .get_mut(&key)
            .ok_or_else(|| BedrockAgentError::AgentKnowledgeBaseNotFound(kb_id.to_string()))?;
        let akb = kbs
            .iter_mut()
            .find(|akb| akb.knowledge_base_id.as_deref() == Some(kb_id))
            .ok_or_else(|| BedrockAgentError::AgentKnowledgeBaseNotFound(kb_id.to_string()))?;
        if let Some(v) = body.get("description").and_then(|v| v.as_str()) {
            akb.description = Some(v.to_string());
        }
        if let Some(v) = body.get("knowledgeBaseState").and_then(|v| v.as_str()) {
            akb.knowledge_base_state = Some(v.to_string());
        }
        akb.updated_at = Some(now_str());
        Ok(akb.clone())
    }

    pub fn disassociate_agent_knowledge_base(
        &mut self,
        agent_id: &str,
        agent_version: &str,
        kb_id: &str,
    ) -> Result<(), BedrockAgentError> {
        let key = agent_version_key(agent_id, agent_version);
        if let Some(kbs) = self.agent_knowledge_bases.get_mut(&key) {
            if let Some(pos) = kbs
                .iter()
                .position(|akb| akb.knowledge_base_id.as_deref() == Some(kb_id))
            {
                kbs.remove(pos);
                return Ok(());
            }
        }
        Err(BedrockAgentError::AgentKnowledgeBaseNotFound(
            kb_id.to_string(),
        ))
    }

    pub fn list_agent_knowledge_bases(
        &self,
        agent_id: &str,
        agent_version: &str,
        max_results: Option<i64>,
        _next_token: Option<&str>,
    ) -> (Vec<AgentKnowledgeBaseSummary>, Option<String>) {
        let key = agent_version_key(agent_id, agent_version);
        let kbs = self
            .agent_knowledge_bases
            .get(&key)
            .cloned()
            .unwrap_or_default();
        let limit = max_results.unwrap_or(100) as usize;
        let summaries: Vec<AgentKnowledgeBaseSummary> = kbs
            .iter()
            .take(limit)
            .map(|akb| AgentKnowledgeBaseSummary {
                knowledge_base_id: akb.knowledge_base_id.clone(),
                description: akb.description.clone(),
                knowledge_base_state: akb.knowledge_base_state.clone(),
                updated_at: akb.updated_at.clone(),
            })
            .collect();
        (summaries, None)
    }

    // ─── Agent Collaborator operations ───────────────────────────────

    pub fn associate_agent_collaborator(
        &mut self,
        agent_id: &str,
        agent_version: &str,
        body: &serde_json::Value,
    ) -> Result<AgentCollaborator, BedrockAgentError> {
        if !self.agents.contains_key(agent_id) {
            return Err(BedrockAgentError::AgentNotFound(agent_id.to_string()));
        }
        let now = now_str();
        let col_id = short_id();
        let col_name = body
            .get("collaboratorName")
            .and_then(|v| v.as_str())
            .unwrap_or("collaborator");
        let instruction = body
            .get("collaborationInstruction")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let ac = AgentCollaborator {
            agent_id: Some(agent_id.to_string()),
            agent_version: Some(agent_version.to_string()),
            collaborator_id: Some(col_id.clone()),
            collaborator_name: Some(col_name.to_string()),
            collaboration_instruction: Some(instruction.to_string()),
            created_at: Some(now.clone()),
            last_updated_at: Some(now),
            ..Default::default()
        };
        let key = agent_version_key(agent_id, agent_version);
        self.agent_collaborators
            .entry(key)
            .or_default()
            .push(ac.clone());
        Ok(ac)
    }

    pub fn get_agent_collaborator(
        &self,
        agent_id: &str,
        agent_version: &str,
        collaborator_id: &str,
    ) -> Result<AgentCollaborator, BedrockAgentError> {
        let key = agent_version_key(agent_id, agent_version);
        self.agent_collaborators
            .get(&key)
            .and_then(|v| {
                v.iter()
                    .find(|ac| ac.collaborator_id.as_deref() == Some(collaborator_id))
            })
            .cloned()
            .ok_or_else(|| BedrockAgentError::CollaboratorNotFound(collaborator_id.to_string()))
    }

    pub fn update_agent_collaborator(
        &mut self,
        agent_id: &str,
        agent_version: &str,
        collaborator_id: &str,
        body: &serde_json::Value,
    ) -> Result<AgentCollaborator, BedrockAgentError> {
        let key = agent_version_key(agent_id, agent_version);
        let cols = self
            .agent_collaborators
            .get_mut(&key)
            .ok_or_else(|| BedrockAgentError::CollaboratorNotFound(collaborator_id.to_string()))?;
        let ac = cols
            .iter_mut()
            .find(|ac| ac.collaborator_id.as_deref() == Some(collaborator_id))
            .ok_or_else(|| BedrockAgentError::CollaboratorNotFound(collaborator_id.to_string()))?;
        if let Some(v) = body.get("collaboratorName").and_then(|v| v.as_str()) {
            ac.collaborator_name = Some(v.to_string());
        }
        if let Some(v) = body
            .get("collaborationInstruction")
            .and_then(|v| v.as_str())
        {
            ac.collaboration_instruction = Some(v.to_string());
        }
        ac.last_updated_at = Some(now_str());
        Ok(ac.clone())
    }

    pub fn disassociate_agent_collaborator(
        &mut self,
        agent_id: &str,
        agent_version: &str,
        collaborator_id: &str,
    ) -> Result<(), BedrockAgentError> {
        let key = agent_version_key(agent_id, agent_version);
        if let Some(cols) = self.agent_collaborators.get_mut(&key) {
            if let Some(pos) = cols
                .iter()
                .position(|ac| ac.collaborator_id.as_deref() == Some(collaborator_id))
            {
                cols.remove(pos);
                return Ok(());
            }
        }
        Err(BedrockAgentError::CollaboratorNotFound(
            collaborator_id.to_string(),
        ))
    }

    pub fn list_agent_collaborators(
        &self,
        agent_id: &str,
        agent_version: &str,
        max_results: Option<i64>,
        _next_token: Option<&str>,
    ) -> (Vec<AgentCollaboratorSummary>, Option<String>) {
        let key = agent_version_key(agent_id, agent_version);
        let cols = self
            .agent_collaborators
            .get(&key)
            .cloned()
            .unwrap_or_default();
        let limit = max_results.unwrap_or(100) as usize;
        let summaries: Vec<AgentCollaboratorSummary> = cols
            .iter()
            .take(limit)
            .map(|ac| AgentCollaboratorSummary {
                agent_id: ac.agent_id.clone(),
                agent_version: ac.agent_version.clone(),
                collaborator_id: ac.collaborator_id.clone(),
                collaborator_name: ac.collaborator_name.clone(),
                collaboration_instruction: ac.collaboration_instruction.clone(),
                created_at: ac.created_at.clone(),
                last_updated_at: ac.last_updated_at.clone(),
                ..Default::default()
            })
            .collect();
        (summaries, None)
    }

    // ─── KnowledgeBase operations ────────────────────────────────────

    pub fn create_knowledge_base(
        &mut self,
        name: &str,
        role_arn: &str,
        account_id: &str,
        region: &str,
        knowledge_base_configuration: &serde_json::Value,
        storage_configuration: &serde_json::Value,
        description: Option<&str>,
    ) -> Result<&KnowledgeBase, BedrockAgentError> {
        let kb_type = knowledge_base_configuration
            .get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        if kb_type != "VECTOR" {
            return Err(BedrockAgentError::InvalidKnowledgeBaseType(
                kb_type.to_string(),
            ));
        }

        let storage_type = storage_configuration
            .get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        if !VALID_STORAGE_TYPES.contains(&storage_type) {
            return Err(BedrockAgentError::InvalidStorageType(
                storage_type.to_string(),
            ));
        }

        let now = now_str();
        let short_uuid = short_id();
        let kb_id = format!("{name}{short_uuid}");
        let kb_arn = format!("arn:aws:bedrock:{region}:{account_id}:knowledge-base/{kb_id}");

        let kb = KnowledgeBase {
            knowledge_base_id: kb_id.clone(),
            name: name.to_string(),
            knowledge_base_arn: kb_arn,
            description: description.map(|s| s.to_string()),
            role_arn: role_arn.to_string(),
            knowledge_base_configuration: knowledge_base_configuration.clone(),
            storage_configuration: storage_configuration.clone(),
            status: "ACTIVE".to_string(),
            created_at: now.clone(),
            updated_at: now,
            failure_reasons: vec![],
        };

        self.knowledge_bases.insert(kb_id.clone(), kb);
        Ok(self.knowledge_bases.get(&kb_id).unwrap())
    }

    pub fn get_knowledge_base(&self, kb_id: &str) -> Result<&KnowledgeBase, BedrockAgentError> {
        self.knowledge_bases
            .get(kb_id)
            .ok_or_else(|| BedrockAgentError::KnowledgeBaseNotFound(kb_id.to_string()))
    }

    pub fn update_knowledge_base(
        &mut self,
        kb_id: &str,
        body: &serde_json::Value,
    ) -> Result<&KnowledgeBase, BedrockAgentError> {
        let kb = self
            .knowledge_bases
            .get_mut(kb_id)
            .ok_or_else(|| BedrockAgentError::KnowledgeBaseNotFound(kb_id.to_string()))?;
        if let Some(v) = body.get("name").and_then(|v| v.as_str()) {
            kb.name = v.to_string();
        }
        if let Some(v) = body.get("description").and_then(|v| v.as_str()) {
            kb.description = Some(v.to_string());
        }
        if let Some(v) = body.get("roleArn").and_then(|v| v.as_str()) {
            kb.role_arn = v.to_string();
        }
        kb.updated_at = now_str();
        Ok(self.knowledge_bases.get(kb_id).unwrap())
    }

    pub fn list_knowledge_bases(
        &self,
        max_results: Option<i64>,
        next_token: Option<&str>,
    ) -> (Vec<serde_json::Value>, Option<String>) {
        let limit = max_results.unwrap_or(100) as usize;
        let start = next_token
            .and_then(|t| t.parse::<usize>().ok())
            .unwrap_or(0);

        let all: Vec<serde_json::Value> = self
            .knowledge_bases
            .values()
            .map(|kb| kb.dict_summary())
            .collect();

        let total = all.len();
        let end = (start + limit).min(total);
        let page = all[start..end].to_vec();

        let new_token = if end < total {
            Some(end.to_string())
        } else {
            None
        };

        (page, new_token)
    }

    pub fn delete_knowledge_base(
        &mut self,
        kb_id: &str,
    ) -> Result<(String, String), BedrockAgentError> {
        if let Some(mut kb) = self.knowledge_bases.remove(kb_id) {
            kb.status = "DELETING".to_string();
            Ok((kb.knowledge_base_id, kb.status))
        } else {
            Err(BedrockAgentError::KnowledgeBaseNotFound(kb_id.to_string()))
        }
    }

    // ─── Data Source operations ──────────────────────────────────────

    pub fn create_data_source(
        &mut self,
        kb_id: &str,
        name: &str,
        body: &serde_json::Value,
        _account_id: &str,
        _region: &str,
    ) -> Result<DataSource, BedrockAgentError> {
        if !self.knowledge_bases.contains_key(kb_id) {
            return Err(BedrockAgentError::KnowledgeBaseNotFound(kb_id.to_string()));
        }
        let now = now_str();
        let ds_id = short_id();
        let data_source_configuration: Option<DataSourceConfiguration> = body
            .get("dataSourceConfiguration")
            .and_then(|v| serde_json::from_value(v.clone()).ok());
        let vector_ingestion_configuration: Option<VectorIngestionConfiguration> = body
            .get("vectorIngestionConfiguration")
            .and_then(|v| serde_json::from_value(v.clone()).ok());
        let ds = DataSource {
            knowledge_base_id: Some(kb_id.to_string()),
            data_source_id: Some(ds_id.clone()),
            name: Some(name.to_string()),
            status: Some("AVAILABLE".to_string()),
            created_at: Some(now.clone()),
            updated_at: Some(now),
            data_source_configuration,
            vector_ingestion_configuration,
            ..Default::default()
        };
        self.data_sources
            .entry(kb_id.to_string())
            .or_default()
            .push(ds.clone());
        Ok(ds)
    }

    pub fn get_data_source(
        &self,
        kb_id: &str,
        ds_id: &str,
    ) -> Result<DataSource, BedrockAgentError> {
        self.data_sources
            .get(kb_id)
            .and_then(|v| {
                v.iter()
                    .find(|ds| ds.data_source_id.as_deref() == Some(ds_id))
            })
            .cloned()
            .ok_or_else(|| BedrockAgentError::DataSourceNotFound(ds_id.to_string()))
    }

    pub fn update_data_source(
        &mut self,
        kb_id: &str,
        ds_id: &str,
        body: &serde_json::Value,
    ) -> Result<DataSource, BedrockAgentError> {
        let sources = self
            .data_sources
            .get_mut(kb_id)
            .ok_or_else(|| BedrockAgentError::DataSourceNotFound(ds_id.to_string()))?;
        let ds = sources
            .iter_mut()
            .find(|ds| ds.data_source_id.as_deref() == Some(ds_id))
            .ok_or_else(|| BedrockAgentError::DataSourceNotFound(ds_id.to_string()))?;
        if let Some(v) = body.get("name").and_then(|v| v.as_str()) {
            ds.name = Some(v.to_string());
        }
        if let Some(v) = body.get("description").and_then(|v| v.as_str()) {
            ds.description = Some(v.to_string());
        }
        if let Some(cfg) = body
            .get("dataSourceConfiguration")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
        {
            ds.data_source_configuration = Some(cfg);
        }
        if let Some(vic) = body
            .get("vectorIngestionConfiguration")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
        {
            ds.vector_ingestion_configuration = Some(vic);
        }
        ds.updated_at = Some(now_str());
        Ok(ds.clone())
    }

    pub fn delete_data_source(
        &mut self,
        kb_id: &str,
        ds_id: &str,
    ) -> Result<(String, String, String), BedrockAgentError> {
        let sources = self
            .data_sources
            .get_mut(kb_id)
            .ok_or_else(|| BedrockAgentError::DataSourceNotFound(ds_id.to_string()))?;
        let pos = sources
            .iter()
            .position(|ds| ds.data_source_id.as_deref() == Some(ds_id))
            .ok_or_else(|| BedrockAgentError::DataSourceNotFound(ds_id.to_string()))?;
        let ds = sources.remove(pos);
        Ok((
            ds.knowledge_base_id.unwrap_or_default(),
            ds.data_source_id.unwrap_or_default(),
            "DELETING".to_string(),
        ))
    }

    pub fn list_data_sources(
        &self,
        kb_id: &str,
        max_results: Option<i64>,
        _next_token: Option<&str>,
    ) -> (Vec<DataSourceSummary>, Option<String>) {
        let sources = self.data_sources.get(kb_id).cloned().unwrap_or_default();
        let limit = max_results.unwrap_or(100) as usize;
        let summaries: Vec<DataSourceSummary> = sources
            .iter()
            .take(limit)
            .map(|ds| DataSourceSummary {
                knowledge_base_id: ds.knowledge_base_id.clone(),
                data_source_id: ds.data_source_id.clone(),
                name: ds.name.clone(),
                status: ds.status.clone(),
                updated_at: ds.updated_at.clone(),
                description: ds.description.clone(),
            })
            .collect();
        (summaries, None)
    }

    // ─── Ingestion Job operations ────────────────────────────────────

    pub fn start_ingestion_job(
        &mut self,
        kb_id: &str,
        ds_id: &str,
        description: Option<&str>,
    ) -> Result<IngestionJob, BedrockAgentError> {
        // Verify data source exists
        let _ = self.get_data_source(kb_id, ds_id)?;

        let now = now_str();
        let job_id = short_id();
        let job = IngestionJob {
            knowledge_base_id: Some(kb_id.to_string()),
            data_source_id: Some(ds_id.to_string()),
            ingestion_job_id: Some(job_id.clone()),
            status: Some("STARTING".to_string()),
            description: description.map(|s| s.to_string()),
            started_at: Some(now.clone()),
            updated_at: Some(now),
            ..Default::default()
        };
        let key = kb_ds_key(kb_id, ds_id);
        self.ingestion_jobs
            .entry(key)
            .or_default()
            .push(job.clone());
        Ok(job)
    }

    pub fn get_ingestion_job(
        &self,
        kb_id: &str,
        ds_id: &str,
        job_id: &str,
    ) -> Result<IngestionJob, BedrockAgentError> {
        let key = kb_ds_key(kb_id, ds_id);
        self.ingestion_jobs
            .get(&key)
            .and_then(|v| {
                v.iter()
                    .find(|j| j.ingestion_job_id.as_deref() == Some(job_id))
            })
            .cloned()
            .ok_or_else(|| BedrockAgentError::IngestionJobNotFound(job_id.to_string()))
    }

    pub fn stop_ingestion_job(
        &mut self,
        kb_id: &str,
        ds_id: &str,
        job_id: &str,
    ) -> Result<IngestionJob, BedrockAgentError> {
        let key = kb_ds_key(kb_id, ds_id);
        let jobs = self
            .ingestion_jobs
            .get_mut(&key)
            .ok_or_else(|| BedrockAgentError::IngestionJobNotFound(job_id.to_string()))?;
        let job = jobs
            .iter_mut()
            .find(|j| j.ingestion_job_id.as_deref() == Some(job_id))
            .ok_or_else(|| BedrockAgentError::IngestionJobNotFound(job_id.to_string()))?;
        job.status = Some("STOPPED".to_string());
        Ok(job.clone())
    }

    pub fn list_ingestion_jobs(
        &self,
        kb_id: &str,
        ds_id: &str,
        max_results: Option<i64>,
        _next_token: Option<&str>,
    ) -> (Vec<IngestionJobSummary>, Option<String>) {
        let key = kb_ds_key(kb_id, ds_id);
        let jobs = self.ingestion_jobs.get(&key).cloned().unwrap_or_default();
        let limit = max_results.unwrap_or(100) as usize;
        let summaries: Vec<IngestionJobSummary> = jobs
            .iter()
            .take(limit)
            .map(|j| IngestionJobSummary {
                knowledge_base_id: j.knowledge_base_id.clone(),
                data_source_id: j.data_source_id.clone(),
                ingestion_job_id: j.ingestion_job_id.clone(),
                status: j.status.clone(),
                description: j.description.clone(),
                started_at: j.started_at.clone(),
                updated_at: j.updated_at.clone(),
                statistics: j.statistics.clone(),
            })
            .collect();
        (summaries, None)
    }

    // ─── Flow operations ─────────────────────────────────────────────

    pub fn create_flow(
        &mut self,
        name: &str,
        body: &serde_json::Value,
        account_id: &str,
        region: &str,
    ) -> Result<CreateFlowResponse, BedrockAgentError> {
        let now = now_str();
        let id = short_id();
        let arn = format!("arn:aws:bedrock:{region}:{account_id}:flow/{id}");
        let description = body
            .get("description")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let execution_role_arn = body
            .get("executionRoleArn")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let flow = FlowState {
            id: id.clone(),
            arn: arn.clone(),
            name: name.to_string(),
            status: "NOT_PREPARED".to_string(),
            created_at: now.clone(),
            updated_at: now.clone(),
            description: description.clone(),
            execution_role_arn: execution_role_arn.clone(),
        };
        self.flows.insert(id.clone(), flow);

        Ok(CreateFlowResponse {
            id: Some(id),
            arn: Some(arn),
            name: Some(name.to_string()),
            status: Some("NOT_PREPARED".to_string()),
            created_at: Some(now.clone()),
            updated_at: Some(now),
            description,
            execution_role_arn,
            version: Some("DRAFT".to_string()),
            ..Default::default()
        })
    }

    pub fn get_flow(&self, flow_id: &str) -> Result<GetFlowResponse, BedrockAgentError> {
        self.flows
            .get(flow_id)
            .map(|f| GetFlowResponse {
                id: Some(f.id.clone()),
                arn: Some(f.arn.clone()),
                name: Some(f.name.clone()),
                status: Some(f.status.clone()),
                created_at: Some(f.created_at.clone()),
                updated_at: Some(f.updated_at.clone()),
                description: f.description.clone(),
                execution_role_arn: f.execution_role_arn.clone(),
                version: Some("DRAFT".to_string()),
                ..Default::default()
            })
            .ok_or_else(|| BedrockAgentError::FlowNotFound(flow_id.to_string()))
    }

    pub fn update_flow(
        &mut self,
        flow_id: &str,
        body: &serde_json::Value,
    ) -> Result<UpdateFlowResponse, BedrockAgentError> {
        let flow = self
            .flows
            .get_mut(flow_id)
            .ok_or_else(|| BedrockAgentError::FlowNotFound(flow_id.to_string()))?;
        if let Some(v) = body.get("name").and_then(|v| v.as_str()) {
            flow.name = v.to_string();
        }
        if let Some(v) = body.get("description").and_then(|v| v.as_str()) {
            flow.description = Some(v.to_string());
        }
        flow.updated_at = now_str();
        let f = flow.clone();
        Ok(UpdateFlowResponse {
            id: Some(f.id),
            arn: Some(f.arn),
            name: Some(f.name),
            status: Some(f.status),
            created_at: Some(f.created_at),
            updated_at: Some(f.updated_at),
            description: f.description,
            execution_role_arn: f.execution_role_arn,
            version: Some("DRAFT".to_string()),
            ..Default::default()
        })
    }

    pub fn delete_flow(&mut self, flow_id: &str) -> Result<String, BedrockAgentError> {
        self.flows
            .remove(flow_id)
            .map(|f| f.id)
            .ok_or_else(|| BedrockAgentError::FlowNotFound(flow_id.to_string()))
    }

    pub fn list_flows(&self) -> (Vec<FlowSummary>, Option<String>) {
        let summaries: Vec<FlowSummary> = self
            .flows
            .values()
            .map(|f| FlowSummary {
                id: Some(f.id.clone()),
                arn: Some(f.arn.clone()),
                name: Some(f.name.clone()),
                status: Some(f.status.clone()),
                created_at: Some(f.created_at.clone()),
                updated_at: Some(f.updated_at.clone()),
                description: f.description.clone(),
                version: Some("DRAFT".to_string()),
            })
            .collect();
        (summaries, None)
    }

    // ─── Flow Alias operations ───────────────────────────────────────

    pub fn create_flow_alias(
        &mut self,
        flow_id: &str,
        name: &str,
        _body: &serde_json::Value,
        account_id: &str,
        region: &str,
    ) -> Result<CreateFlowAliasResponse, BedrockAgentError> {
        if !self.flows.contains_key(flow_id) {
            return Err(BedrockAgentError::FlowNotFound(flow_id.to_string()));
        }
        let now = now_str();
        let alias_id = short_id();
        let arn = format!("arn:aws:bedrock:{region}:{account_id}:flow/{flow_id}/alias/{alias_id}");

        let alias = FlowAliasState {
            flow_id: flow_id.to_string(),
            id: alias_id.clone(),
            arn: arn.clone(),
            name: name.to_string(),
            created_at: now.clone(),
            updated_at: now.clone(),
        };
        self.flow_aliases
            .entry(flow_id.to_string())
            .or_default()
            .push(alias);

        Ok(CreateFlowAliasResponse {
            id: Some(alias_id),
            arn: Some(arn),
            name: Some(name.to_string()),
            flow_id: Some(flow_id.to_string()),
            created_at: Some(now.clone()),
            updated_at: Some(now),
            ..Default::default()
        })
    }

    pub fn get_flow_alias(
        &self,
        flow_id: &str,
        alias_id: &str,
    ) -> Result<GetFlowAliasResponse, BedrockAgentError> {
        self.flow_aliases
            .get(flow_id)
            .and_then(|v| v.iter().find(|a| a.id == alias_id))
            .map(|a| GetFlowAliasResponse {
                id: Some(a.id.clone()),
                arn: Some(a.arn.clone()),
                name: Some(a.name.clone()),
                flow_id: Some(a.flow_id.clone()),
                created_at: Some(a.created_at.clone()),
                updated_at: Some(a.updated_at.clone()),
                ..Default::default()
            })
            .ok_or_else(|| BedrockAgentError::FlowAliasNotFound(alias_id.to_string()))
    }

    pub fn update_flow_alias(
        &mut self,
        flow_id: &str,
        alias_id: &str,
        body: &serde_json::Value,
    ) -> Result<UpdateFlowAliasResponse, BedrockAgentError> {
        let aliases = self
            .flow_aliases
            .get_mut(flow_id)
            .ok_or_else(|| BedrockAgentError::FlowAliasNotFound(alias_id.to_string()))?;
        let alias = aliases
            .iter_mut()
            .find(|a| a.id == alias_id)
            .ok_or_else(|| BedrockAgentError::FlowAliasNotFound(alias_id.to_string()))?;
        if let Some(v) = body.get("name").and_then(|v| v.as_str()) {
            alias.name = v.to_string();
        }
        alias.updated_at = now_str();
        let a = alias.clone();
        Ok(UpdateFlowAliasResponse {
            id: Some(a.id),
            arn: Some(a.arn),
            name: Some(a.name),
            flow_id: Some(a.flow_id),
            created_at: Some(a.created_at),
            updated_at: Some(a.updated_at),
            ..Default::default()
        })
    }

    pub fn delete_flow_alias(
        &mut self,
        flow_id: &str,
        alias_id: &str,
    ) -> Result<(String, String), BedrockAgentError> {
        let aliases = self
            .flow_aliases
            .get_mut(flow_id)
            .ok_or_else(|| BedrockAgentError::FlowAliasNotFound(alias_id.to_string()))?;
        let pos = aliases
            .iter()
            .position(|a| a.id == alias_id)
            .ok_or_else(|| BedrockAgentError::FlowAliasNotFound(alias_id.to_string()))?;
        let alias = aliases.remove(pos);
        Ok((alias.flow_id, alias.id))
    }

    pub fn list_flow_aliases(&self, flow_id: &str) -> (Vec<FlowAliasSummary>, Option<String>) {
        let aliases = self.flow_aliases.get(flow_id).cloned().unwrap_or_default();
        let summaries: Vec<FlowAliasSummary> = aliases
            .iter()
            .map(|a| FlowAliasSummary {
                id: Some(a.id.clone()),
                arn: Some(a.arn.clone()),
                name: Some(a.name.clone()),
                flow_id: Some(a.flow_id.clone()),
                created_at: Some(a.created_at.clone()),
                updated_at: Some(a.updated_at.clone()),
                ..Default::default()
            })
            .collect();
        (summaries, None)
    }

    // ─── Flow Version operations ─────────────────────────────────────

    pub fn create_flow_version(
        &mut self,
        flow_id: &str,
    ) -> Result<CreateFlowVersionResponse, BedrockAgentError> {
        let flow = self
            .flows
            .get(flow_id)
            .ok_or_else(|| BedrockAgentError::FlowNotFound(flow_id.to_string()))?;

        let now = now_str();
        let existing = self
            .flow_versions
            .get(flow_id)
            .map(|v| v.len())
            .unwrap_or(0);
        let version = format!("{}", existing + 1);
        let arn = format!("{}:{}", flow.arn, version);

        let ver = FlowVersionState {
            flow_id: flow_id.to_string(),
            version: version.clone(),
            arn: arn.clone(),
            status: "Available".to_string(),
            created_at: now.clone(),
        };
        self.flow_versions
            .entry(flow_id.to_string())
            .or_default()
            .push(ver);

        Ok(CreateFlowVersionResponse {
            id: Some(flow_id.to_string()),
            arn: Some(arn),
            version: Some(version),
            status: Some("Available".to_string()),
            created_at: Some(now),
            ..Default::default()
        })
    }

    pub fn get_flow_version(
        &self,
        flow_id: &str,
        version: &str,
    ) -> Result<GetFlowVersionResponse, BedrockAgentError> {
        self.flow_versions
            .get(flow_id)
            .and_then(|v| v.iter().find(|ver| ver.version == version))
            .map(|ver| GetFlowVersionResponse {
                id: Some(ver.flow_id.clone()),
                arn: Some(ver.arn.clone()),
                version: Some(ver.version.clone()),
                status: Some(ver.status.clone()),
                created_at: Some(ver.created_at.clone()),
                ..Default::default()
            })
            .ok_or_else(|| BedrockAgentError::FlowVersionNotFound(version.to_string()))
    }

    pub fn delete_flow_version(
        &mut self,
        flow_id: &str,
        version: &str,
    ) -> Result<(String, String), BedrockAgentError> {
        let versions = self
            .flow_versions
            .get_mut(flow_id)
            .ok_or_else(|| BedrockAgentError::FlowVersionNotFound(version.to_string()))?;
        let pos = versions
            .iter()
            .position(|v| v.version == version)
            .ok_or_else(|| BedrockAgentError::FlowVersionNotFound(version.to_string()))?;
        let ver = versions.remove(pos);
        Ok((ver.flow_id, ver.version))
    }

    pub fn list_flow_versions(&self, flow_id: &str) -> (Vec<FlowVersionSummary>, Option<String>) {
        let versions = self.flow_versions.get(flow_id).cloned().unwrap_or_default();
        let summaries: Vec<FlowVersionSummary> = versions
            .iter()
            .map(|v| FlowVersionSummary {
                arn: Some(v.arn.clone()),
                id: Some(v.flow_id.clone()),
                version: Some(v.version.clone()),
                status: Some(v.status.clone()),
                created_at: Some(v.created_at.clone()),
                ..Default::default()
            })
            .collect();
        (summaries, None)
    }

    // ─── Prompt operations ───────────────────────────────────────────

    pub fn create_prompt(
        &mut self,
        name: &str,
        body: &serde_json::Value,
        account_id: &str,
        region: &str,
    ) -> Result<CreatePromptResponse, BedrockAgentError> {
        let now = now_str();
        let id = short_id();
        let arn = format!("arn:aws:bedrock:{region}:{account_id}:prompt/{id}");
        let description = body
            .get("description")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let prompt = PromptState {
            id: id.clone(),
            arn: arn.clone(),
            name: name.to_string(),
            created_at: now.clone(),
            updated_at: now.clone(),
            description: description.clone(),
        };
        self.prompts.insert(id.clone(), prompt);

        Ok(CreatePromptResponse {
            id: Some(id),
            arn: Some(arn),
            name: Some(name.to_string()),
            created_at: Some(now.clone()),
            updated_at: Some(now),
            description,
            version: Some("DRAFT".to_string()),
            ..Default::default()
        })
    }

    pub fn get_prompt(&self, prompt_id: &str) -> Result<GetPromptResponse, BedrockAgentError> {
        self.prompts
            .get(prompt_id)
            .map(|p| GetPromptResponse {
                id: Some(p.id.clone()),
                arn: Some(p.arn.clone()),
                name: Some(p.name.clone()),
                created_at: Some(p.created_at.clone()),
                updated_at: Some(p.updated_at.clone()),
                description: p.description.clone(),
                version: Some("DRAFT".to_string()),
                ..Default::default()
            })
            .ok_or_else(|| BedrockAgentError::PromptNotFound(prompt_id.to_string()))
    }

    pub fn update_prompt(
        &mut self,
        prompt_id: &str,
        body: &serde_json::Value,
    ) -> Result<UpdatePromptResponse, BedrockAgentError> {
        let prompt = self
            .prompts
            .get_mut(prompt_id)
            .ok_or_else(|| BedrockAgentError::PromptNotFound(prompt_id.to_string()))?;
        if let Some(v) = body.get("name").and_then(|v| v.as_str()) {
            prompt.name = v.to_string();
        }
        if let Some(v) = body.get("description").and_then(|v| v.as_str()) {
            prompt.description = Some(v.to_string());
        }
        prompt.updated_at = now_str();
        let p = prompt.clone();
        Ok(UpdatePromptResponse {
            id: Some(p.id),
            arn: Some(p.arn),
            name: Some(p.name),
            created_at: Some(p.created_at),
            updated_at: Some(p.updated_at),
            description: p.description,
            version: Some("DRAFT".to_string()),
            ..Default::default()
        })
    }

    pub fn delete_prompt(
        &mut self,
        prompt_id: &str,
    ) -> Result<(String, Option<String>), BedrockAgentError> {
        self.prompts
            .remove(prompt_id)
            .map(|p| (p.id, None))
            .ok_or_else(|| BedrockAgentError::PromptNotFound(prompt_id.to_string()))
    }

    pub fn create_prompt_version(
        &mut self,
        prompt_id: &str,
    ) -> Result<CreatePromptVersionResponse, BedrockAgentError> {
        let prompt = self
            .prompts
            .get(prompt_id)
            .ok_or_else(|| BedrockAgentError::PromptNotFound(prompt_id.to_string()))?;
        let now = now_str();
        let version = "1".to_string();
        Ok(CreatePromptVersionResponse {
            id: Some(prompt.id.clone()),
            arn: Some(prompt.arn.clone()),
            name: Some(prompt.name.clone()),
            created_at: Some(now.clone()),
            updated_at: Some(now),
            description: prompt.description.clone(),
            version: Some(version),
            ..Default::default()
        })
    }

    pub fn list_prompts(&self) -> (Vec<PromptSummary>, Option<String>) {
        let summaries: Vec<PromptSummary> = self
            .prompts
            .values()
            .map(|p| PromptSummary {
                id: Some(p.id.clone()),
                arn: Some(p.arn.clone()),
                name: Some(p.name.clone()),
                created_at: Some(p.created_at.clone()),
                updated_at: Some(p.updated_at.clone()),
                description: p.description.clone(),
                version: Some("DRAFT".to_string()),
            })
            .collect();
        (summaries, None)
    }

    // ─── Tag operations ──────────────────────────────────────────────

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        new_tags: &HashMap<String, String>,
    ) -> Result<(), BedrockAgentError> {
        if !self.list_arns().contains(&resource_arn.to_string()) {
            return Err(BedrockAgentError::ResourceNotFound(
                resource_arn.to_string(),
            ));
        }
        let entry = self.tags.entry(resource_arn.to_string()).or_default();
        for (k, v) in new_tags {
            entry.insert(k.clone(), v.clone());
        }
        Ok(())
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), BedrockAgentError> {
        if !self.list_arns().contains(&resource_arn.to_string()) {
            return Err(BedrockAgentError::ResourceNotFound(
                resource_arn.to_string(),
            ));
        }
        if let Some(tags) = self.tags.get_mut(resource_arn) {
            for key in tag_keys {
                tags.remove(key);
            }
        }
        Ok(())
    }

    pub fn list_tags_for_resource(
        &self,
        resource_arn: &str,
    ) -> Result<HashMap<String, String>, BedrockAgentError> {
        if !self.list_arns().contains(&resource_arn.to_string()) {
            return Err(BedrockAgentError::ResourceNotFound(
                resource_arn.to_string(),
            ));
        }
        Ok(self.tags.get(resource_arn).cloned().unwrap_or_default())
    }

    // ─── Knowledge Base Document operations ────────────────────────────

    fn kb_ds_key(kb_id: &str, ds_id: &str) -> String {
        format!("{kb_id}:{ds_id}")
    }

    pub fn ingest_knowledge_base_documents(
        &mut self,
        kb_id: &str,
        ds_id: &str,
        documents: &[crate::model::KnowledgeBaseDocument],
    ) -> Result<Vec<KnowledgeBaseDocumentState>, BedrockAgentError> {
        let _ = self.get_data_source(kb_id, ds_id)?;
        let key = Self::kb_ds_key(kb_id, ds_id);
        let now = now_str();
        let docs = self.kb_documents.entry(key).or_default();

        let mut result = Vec::new();
        for doc in documents {
            let ds_type = doc.content.data_source_type.clone();
            let uri = doc
                .content
                .custom
                .as_ref()
                .and_then(|c| c.s3_location.as_ref())
                .map(|s| s.uri.clone());
            let custom_id = doc
                .content
                .custom
                .as_ref()
                .map(|c| c.custom_document_identifier.id.clone())
                .filter(|id| !id.is_empty());

            let doc_state = KnowledgeBaseDocumentState {
                knowledge_base_id: kb_id.to_string(),
                data_source_id: ds_id.to_string(),
                data_source_type: if ds_type.is_empty() {
                    "CUSTOM".to_string()
                } else {
                    ds_type
                },
                identifier_uri: uri,
                identifier_custom_id: custom_id,
                status: "INDEXED".to_string(),
                updated_at: now.clone(),
            };
            docs.push(doc_state.clone());
            result.push(doc_state);
        }
        Ok(result)
    }

    pub fn list_knowledge_base_documents(
        &self,
        kb_id: &str,
        ds_id: &str,
    ) -> Result<Vec<&KnowledgeBaseDocumentState>, BedrockAgentError> {
        let _ = self.get_data_source(kb_id, ds_id)?;
        let key = Self::kb_ds_key(kb_id, ds_id);
        Ok(self
            .kb_documents
            .get(&key)
            .map(|v| v.iter().collect())
            .unwrap_or_default())
    }

    pub fn get_knowledge_base_documents(
        &self,
        kb_id: &str,
        ds_id: &str,
        identifiers: &[crate::model::DocumentIdentifier],
    ) -> Result<Vec<&KnowledgeBaseDocumentState>, BedrockAgentError> {
        let _ = self.get_data_source(kb_id, ds_id)?;
        let key = Self::kb_ds_key(kb_id, ds_id);
        let docs = self.kb_documents.get(&key);
        let mut result = Vec::new();
        if let Some(docs) = docs {
            for ident in identifiers {
                // Match by custom ID or URI
                let custom_id = ident.custom.as_ref().map(|c| c.id.clone());
                let s3_uri = ident.s3.as_ref().map(|s| s.uri.clone());
                for doc in docs {
                    let matches_custom = custom_id
                        .as_ref()
                        .is_some_and(|id| doc.identifier_custom_id.as_ref() == Some(id));
                    let matches_uri = s3_uri
                        .as_ref()
                        .is_some_and(|uri| doc.identifier_uri.as_ref() == Some(uri));
                    if matches_custom || matches_uri {
                        result.push(doc);
                    }
                }
            }
        }
        Ok(result)
    }

    pub fn delete_knowledge_base_documents(
        &mut self,
        kb_id: &str,
        ds_id: &str,
        identifiers: &[crate::model::DocumentIdentifier],
    ) -> Result<Vec<KnowledgeBaseDocumentState>, BedrockAgentError> {
        let _ = self.get_data_source(kb_id, ds_id)?;
        let key = Self::kb_ds_key(kb_id, ds_id);
        let docs = self.kb_documents.entry(key).or_default();
        let mut deleted = Vec::new();

        for ident in identifiers {
            let custom_id = ident.custom.as_ref().map(|c| c.id.clone());
            let s3_uri = ident.s3.as_ref().map(|s| s.uri.clone());
            let mut i = 0;
            while i < docs.len() {
                let matches_custom = custom_id
                    .as_ref()
                    .is_some_and(|id| docs[i].identifier_custom_id.as_ref() == Some(id));
                let matches_uri = s3_uri
                    .as_ref()
                    .is_some_and(|uri| docs[i].identifier_uri.as_ref() == Some(uri));
                if matches_custom || matches_uri {
                    let mut removed = docs.remove(i);
                    removed.status = "DELETE_IN_PROGRESS".to_string();
                    deleted.push(removed);
                } else {
                    i += 1;
                }
            }
        }
        Ok(deleted)
    }
}
