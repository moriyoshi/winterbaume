//! Serde-compatible view types for Bedrock Agent state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::BedrockAgentService;
use crate::state::BedrockAgentState;

/// Serializable view of the entire Bedrock Agent state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BedrockAgentStateView {
    #[serde(default)]
    pub agents: HashMap<String, AgentView>,
    #[serde(default)]
    pub knowledge_bases: HashMap<String, KnowledgeBaseView>,
    #[serde(default)]
    pub tags: HashMap<String, HashMap<String, String>>,
    #[serde(default)]
    pub agent_aliases: HashMap<String, Vec<AgentAliasView>>,
    #[serde(default)]
    pub agent_action_groups: HashMap<String, Vec<serde_json::Value>>,
    #[serde(default)]
    pub agent_knowledge_bases: HashMap<String, Vec<crate::model::AgentKnowledgeBase>>,
    #[serde(default)]
    pub agent_collaborators: HashMap<String, Vec<serde_json::Value>>,
    #[serde(default)]
    pub data_sources: HashMap<String, Vec<serde_json::Value>>,
    #[serde(default)]
    pub ingestion_jobs: HashMap<String, Vec<serde_json::Value>>,
    #[serde(default)]
    pub flows: HashMap<String, FlowView>,
    #[serde(default)]
    pub flow_aliases: HashMap<String, Vec<FlowAliasView>>,
    #[serde(default)]
    pub flow_versions: HashMap<String, Vec<FlowVersionView>>,
    #[serde(default)]
    pub prompts: HashMap<String, PromptView>,
    #[serde(default)]
    pub kb_documents: HashMap<String, Vec<KnowledgeBaseDocumentView>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeBaseDocumentView {
    pub knowledge_base_id: String,
    pub data_source_id: String,
    pub data_source_type: String,
    pub identifier_uri: Option<String>,
    pub identifier_custom_id: Option<String>,
    pub status: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentView {
    pub agent_id: String,
    pub agent_name: String,
    pub agent_arn: String,
    pub agent_version: String,
    pub client_token: Option<String>,
    pub instruction: Option<String>,
    pub agent_status: String,
    pub foundation_model: Option<String>,
    pub description: Option<String>,
    pub idle_session_ttl_in_seconds: Option<i64>,
    pub agent_resource_role_arn: String,
    pub customer_encryption_key_arn: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub prepared_at: String,
    #[serde(default)]
    pub failure_reasons: Vec<String>,
    #[serde(default)]
    pub recommended_actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeBaseView {
    pub knowledge_base_id: String,
    pub name: String,
    pub knowledge_base_arn: String,
    pub description: Option<String>,
    pub role_arn: String,
    pub knowledge_base_configuration: serde_json::Value,
    pub storage_configuration: serde_json::Value,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    #[serde(default)]
    pub failure_reasons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentAliasView {
    pub agent_id: String,
    pub agent_alias_id: String,
    pub agent_alias_name: String,
    pub agent_alias_arn: String,
    pub agent_alias_status: String,
    pub created_at: String,
    pub updated_at: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowView {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub description: Option<String>,
    pub execution_role_arn: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowAliasView {
    pub flow_id: String,
    pub id: String,
    pub arn: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowVersionView {
    pub flow_id: String,
    pub version: String,
    pub arn: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptView {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
    pub description: Option<String>,
}

// --- From internal types to view types ---

impl From<&BedrockAgentState> for BedrockAgentStateView {
    fn from(state: &BedrockAgentState) -> Self {
        BedrockAgentStateView {
            agents: state
                .agents
                .iter()
                .map(|(k, a)| {
                    (
                        k.clone(),
                        AgentView {
                            agent_id: a.agent_id.clone(),
                            agent_name: a.agent_name.clone(),
                            agent_arn: a.agent_arn.clone(),
                            agent_version: a.agent_version.clone(),
                            client_token: a.client_token.clone(),
                            instruction: a.instruction.clone(),
                            agent_status: a.agent_status.clone(),
                            foundation_model: a.foundation_model.clone(),
                            description: a.description.clone(),
                            idle_session_ttl_in_seconds: a.idle_session_ttl_in_seconds,
                            agent_resource_role_arn: a.agent_resource_role_arn.clone(),
                            customer_encryption_key_arn: a.customer_encryption_key_arn.clone(),
                            created_at: a.created_at.clone(),
                            updated_at: a.updated_at.clone(),
                            prepared_at: a.prepared_at.clone(),
                            failure_reasons: a.failure_reasons.clone(),
                            recommended_actions: a.recommended_actions.clone(),
                        },
                    )
                })
                .collect(),
            knowledge_bases: state
                .knowledge_bases
                .iter()
                .map(|(k, kb)| {
                    (
                        k.clone(),
                        KnowledgeBaseView {
                            knowledge_base_id: kb.knowledge_base_id.clone(),
                            name: kb.name.clone(),
                            knowledge_base_arn: kb.knowledge_base_arn.clone(),
                            description: kb.description.clone(),
                            role_arn: kb.role_arn.clone(),
                            knowledge_base_configuration: kb.knowledge_base_configuration.clone(),
                            storage_configuration: kb.storage_configuration.clone(),
                            status: kb.status.clone(),
                            created_at: kb.created_at.clone(),
                            updated_at: kb.updated_at.clone(),
                            failure_reasons: kb.failure_reasons.clone(),
                        },
                    )
                })
                .collect(),
            tags: state.tags.clone(),
            agent_aliases: state
                .agent_aliases
                .iter()
                .map(|(k, aliases)| {
                    (
                        k.clone(),
                        aliases
                            .iter()
                            .map(|a| AgentAliasView {
                                agent_id: a.agent_id.clone(),
                                agent_alias_id: a.agent_alias_id.clone(),
                                agent_alias_name: a.agent_alias_name.clone(),
                                agent_alias_arn: a.agent_alias_arn.clone(),
                                agent_alias_status: a.agent_alias_status.clone(),
                                created_at: a.created_at.clone(),
                                updated_at: a.updated_at.clone(),
                                description: a.description.clone(),
                            })
                            .collect(),
                    )
                })
                .collect(),
            agent_action_groups: state
                .agent_action_groups
                .iter()
                .map(|(k, groups)| {
                    (
                        k.clone(),
                        groups
                            .iter()
                            .map(|g| serde_json::to_value(g).unwrap_or(serde_json::Value::Null))
                            .collect(),
                    )
                })
                .collect(),
            agent_knowledge_bases: state.agent_knowledge_bases.clone(),
            agent_collaborators: state
                .agent_collaborators
                .iter()
                .map(|(k, collabs)| {
                    (
                        k.clone(),
                        collabs
                            .iter()
                            .map(|c| serde_json::to_value(c).unwrap_or(serde_json::Value::Null))
                            .collect(),
                    )
                })
                .collect(),
            data_sources: state
                .data_sources
                .iter()
                .map(|(k, sources)| {
                    (
                        k.clone(),
                        sources
                            .iter()
                            .map(|s| serde_json::to_value(s).unwrap_or(serde_json::Value::Null))
                            .collect(),
                    )
                })
                .collect(),
            ingestion_jobs: state
                .ingestion_jobs
                .iter()
                .map(|(k, jobs)| {
                    (
                        k.clone(),
                        jobs.iter()
                            .map(|j| serde_json::to_value(j).unwrap_or(serde_json::Value::Null))
                            .collect(),
                    )
                })
                .collect(),
            flows: state
                .flows
                .iter()
                .map(|(k, f)| {
                    (
                        k.clone(),
                        FlowView {
                            id: f.id.clone(),
                            arn: f.arn.clone(),
                            name: f.name.clone(),
                            status: f.status.clone(),
                            created_at: f.created_at.clone(),
                            updated_at: f.updated_at.clone(),
                            description: f.description.clone(),
                            execution_role_arn: f.execution_role_arn.clone(),
                        },
                    )
                })
                .collect(),
            flow_aliases: state
                .flow_aliases
                .iter()
                .map(|(k, aliases)| {
                    (
                        k.clone(),
                        aliases
                            .iter()
                            .map(|a| FlowAliasView {
                                flow_id: a.flow_id.clone(),
                                id: a.id.clone(),
                                arn: a.arn.clone(),
                                name: a.name.clone(),
                                created_at: a.created_at.clone(),
                                updated_at: a.updated_at.clone(),
                            })
                            .collect(),
                    )
                })
                .collect(),
            flow_versions: state
                .flow_versions
                .iter()
                .map(|(k, versions)| {
                    (
                        k.clone(),
                        versions
                            .iter()
                            .map(|v| FlowVersionView {
                                flow_id: v.flow_id.clone(),
                                version: v.version.clone(),
                                arn: v.arn.clone(),
                                status: v.status.clone(),
                                created_at: v.created_at.clone(),
                            })
                            .collect(),
                    )
                })
                .collect(),
            prompts: state
                .prompts
                .iter()
                .map(|(k, p)| {
                    (
                        k.clone(),
                        PromptView {
                            id: p.id.clone(),
                            arn: p.arn.clone(),
                            name: p.name.clone(),
                            created_at: p.created_at.clone(),
                            updated_at: p.updated_at.clone(),
                            description: p.description.clone(),
                        },
                    )
                })
                .collect(),
            kb_documents: state
                .kb_documents
                .iter()
                .map(|(k, docs)| {
                    (
                        k.clone(),
                        docs.iter()
                            .map(|d| KnowledgeBaseDocumentView {
                                knowledge_base_id: d.knowledge_base_id.clone(),
                                data_source_id: d.data_source_id.clone(),
                                data_source_type: d.data_source_type.clone(),
                                identifier_uri: d.identifier_uri.clone(),
                                identifier_custom_id: d.identifier_custom_id.clone(),
                                status: d.status.clone(),
                                updated_at: d.updated_at.clone(),
                            })
                            .collect(),
                    )
                })
                .collect(),
        }
    }
}

// --- From view types to internal types ---

impl From<BedrockAgentStateView> for BedrockAgentState {
    fn from(view: BedrockAgentStateView) -> Self {
        BedrockAgentState {
            agents: view
                .agents
                .into_iter()
                .map(|(k, a)| {
                    (
                        k,
                        crate::types::Agent {
                            agent_id: a.agent_id,
                            agent_name: a.agent_name,
                            agent_arn: a.agent_arn,
                            agent_version: a.agent_version,
                            client_token: a.client_token,
                            instruction: a.instruction,
                            agent_status: a.agent_status,
                            foundation_model: a.foundation_model,
                            description: a.description,
                            idle_session_ttl_in_seconds: a.idle_session_ttl_in_seconds,
                            agent_resource_role_arn: a.agent_resource_role_arn,
                            customer_encryption_key_arn: a.customer_encryption_key_arn,
                            created_at: a.created_at,
                            updated_at: a.updated_at,
                            prepared_at: a.prepared_at,
                            failure_reasons: a.failure_reasons,
                            recommended_actions: a.recommended_actions,
                        },
                    )
                })
                .collect(),
            knowledge_bases: view
                .knowledge_bases
                .into_iter()
                .map(|(k, kb)| {
                    (
                        k,
                        crate::types::KnowledgeBase {
                            knowledge_base_id: kb.knowledge_base_id,
                            name: kb.name,
                            knowledge_base_arn: kb.knowledge_base_arn,
                            description: kb.description,
                            role_arn: kb.role_arn,
                            knowledge_base_configuration: kb.knowledge_base_configuration,
                            storage_configuration: kb.storage_configuration,
                            status: kb.status,
                            created_at: kb.created_at,
                            updated_at: kb.updated_at,
                            failure_reasons: kb.failure_reasons,
                        },
                    )
                })
                .collect(),
            tags: view.tags,
            agent_aliases: view
                .agent_aliases
                .into_iter()
                .map(|(k, aliases)| {
                    (
                        k,
                        aliases
                            .into_iter()
                            .map(|a| crate::state::AgentAliasState {
                                agent_id: a.agent_id,
                                agent_alias_id: a.agent_alias_id,
                                agent_alias_name: a.agent_alias_name,
                                agent_alias_arn: a.agent_alias_arn,
                                agent_alias_status: a.agent_alias_status,
                                created_at: a.created_at,
                                updated_at: a.updated_at,
                                description: a.description,
                            })
                            .collect(),
                    )
                })
                .collect(),
            agent_action_groups: view
                .agent_action_groups
                .into_iter()
                .map(|(k, groups)| {
                    (
                        k,
                        groups
                            .into_iter()
                            .filter_map(|g| serde_json::from_value(g).ok())
                            .collect(),
                    )
                })
                .collect(),
            agent_knowledge_bases: view.agent_knowledge_bases,
            agent_collaborators: view
                .agent_collaborators
                .into_iter()
                .map(|(k, collabs)| {
                    (
                        k,
                        collabs
                            .into_iter()
                            .filter_map(|c| serde_json::from_value(c).ok())
                            .collect(),
                    )
                })
                .collect(),
            data_sources: view
                .data_sources
                .into_iter()
                .map(|(k, sources)| {
                    (
                        k,
                        sources
                            .into_iter()
                            .filter_map(|s| serde_json::from_value(s).ok())
                            .collect(),
                    )
                })
                .collect(),
            ingestion_jobs: view
                .ingestion_jobs
                .into_iter()
                .map(|(k, jobs)| {
                    (
                        k,
                        jobs.into_iter()
                            .filter_map(|j| serde_json::from_value(j).ok())
                            .collect(),
                    )
                })
                .collect(),
            flows: view
                .flows
                .into_iter()
                .map(|(k, f)| {
                    (
                        k,
                        crate::state::FlowState {
                            id: f.id,
                            arn: f.arn,
                            name: f.name,
                            status: f.status,
                            created_at: f.created_at,
                            updated_at: f.updated_at,
                            description: f.description,
                            execution_role_arn: f.execution_role_arn,
                        },
                    )
                })
                .collect(),
            flow_aliases: view
                .flow_aliases
                .into_iter()
                .map(|(k, aliases)| {
                    (
                        k,
                        aliases
                            .into_iter()
                            .map(|a| crate::state::FlowAliasState {
                                flow_id: a.flow_id,
                                id: a.id,
                                arn: a.arn,
                                name: a.name,
                                created_at: a.created_at,
                                updated_at: a.updated_at,
                            })
                            .collect(),
                    )
                })
                .collect(),
            flow_versions: view
                .flow_versions
                .into_iter()
                .map(|(k, versions)| {
                    (
                        k,
                        versions
                            .into_iter()
                            .map(|v| crate::state::FlowVersionState {
                                flow_id: v.flow_id,
                                version: v.version,
                                arn: v.arn,
                                status: v.status,
                                created_at: v.created_at,
                            })
                            .collect(),
                    )
                })
                .collect(),
            prompts: view
                .prompts
                .into_iter()
                .map(|(k, p)| {
                    (
                        k,
                        crate::state::PromptState {
                            id: p.id,
                            arn: p.arn,
                            name: p.name,
                            created_at: p.created_at,
                            updated_at: p.updated_at,
                            description: p.description,
                        },
                    )
                })
                .collect(),
            kb_documents: view
                .kb_documents
                .into_iter()
                .map(|(k, docs)| {
                    (
                        k,
                        docs.into_iter()
                            .map(|d| crate::state::KnowledgeBaseDocumentState {
                                knowledge_base_id: d.knowledge_base_id,
                                data_source_id: d.data_source_id,
                                data_source_type: d.data_source_type,
                                identifier_uri: d.identifier_uri,
                                identifier_custom_id: d.identifier_custom_id,
                                status: d.status,
                                updated_at: d.updated_at,
                            })
                            .collect(),
                    )
                })
                .collect(),
            ..Default::default()
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for BedrockAgentService {
    type StateView = BedrockAgentStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        BedrockAgentStateView::from(&*guard)
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            *guard = BedrockAgentState::from(view);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            let merged = BedrockAgentState::from(view);
            for (k, v) in merged.agents {
                guard.agents.insert(k, v);
            }
            for (k, v) in merged.knowledge_bases {
                guard.knowledge_bases.insert(k, v);
            }
            for (k, v) in merged.tags {
                guard.tags.insert(k, v);
            }
            for (k, v) in merged.agent_aliases {
                guard.agent_aliases.insert(k, v);
            }
            for (k, v) in merged.agent_action_groups {
                guard.agent_action_groups.insert(k, v);
            }
            for (k, v) in merged.agent_knowledge_bases {
                guard.agent_knowledge_bases.insert(k, v);
            }
            for (k, v) in merged.agent_collaborators {
                guard.agent_collaborators.insert(k, v);
            }
            for (k, v) in merged.data_sources {
                guard.data_sources.insert(k, v);
            }
            for (k, v) in merged.ingestion_jobs {
                guard.ingestion_jobs.insert(k, v);
            }
            for (k, v) in merged.flows {
                guard.flows.insert(k, v);
            }
            for (k, v) in merged.flow_aliases {
                guard.flow_aliases.insert(k, v);
            }
            for (k, v) in merged.flow_versions {
                guard.flow_versions.insert(k, v);
            }
            for (k, v) in merged.prompts {
                guard.prompts.insert(k, v);
            }
            for (k, v) in merged.kb_documents {
                guard.kb_documents.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
