//! Terraform converters for Bedrock Agent resources.
//!
//! `AgentTfModel`, `AgentActionGroupTfModel`, `AgentAliasTfModel`,
//! `AgentCollaboratorTfModel`, `AgentKnowledgeBaseAssociationTfModel`,
//! `DataSourceTfModel`, `PromptTfModel`, and `KnowledgeBaseTfModel`
//! are generated from `specs/bedrockagent.toml`. The ARN templates,
//! the synthesised IDs, the constants (`agent_version = "DRAFT"`,
//! `agent_status = "NOT_PREPARED"`, `status = "ACTIVE"`,
//! `agent_alias_status = "PREPARED"`,
//! `knowledge_base_state = "ENABLED"`), the `Option<i64>`
//! `idle_session_ttl_in_seconds`, the `failure_reasons` /
//! `recommended_actions` Vec<String> fields, the
//! `knowledge_base_configuration` / `storage_configuration` /
//! `action_group_executor` / `api_schema` / `function_schema` /
//! `agent_descriptor` / `routing_configuration` /
//! `data_source_configuration` / `vector_ingestion_configuration` /
//! `server_side_encryption_configuration` / `variant` raw JSON
//! blobs, the composite Terraform ids, and the `tags_all` overlay are
//! wired up here.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_bedrockagent::BedrockAgentService;
use winterbaume_bedrockagent::model::{
    AgentActionGroup, AgentCollaborator, AgentKnowledgeBase, DataSource,
};
use winterbaume_bedrockagent::views::{
    AgentAliasView, AgentView, BedrockAgentStateView, KnowledgeBaseView, PromptView,
};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::bedrockagent as bedrockagent_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn minimal_state_view() -> BedrockAgentStateView {
    BedrockAgentStateView {
        agents: HashMap::new(),
        knowledge_bases: HashMap::new(),
        tags: HashMap::new(),
        agent_aliases: HashMap::new(),
        agent_action_groups: HashMap::new(),
        agent_knowledge_bases: HashMap::new(),
        agent_collaborators: HashMap::new(),
        data_sources: HashMap::new(),
        ingestion_jobs: HashMap::new(),
        flows: HashMap::new(),
        flow_aliases: HashMap::new(),
        flow_versions: HashMap::new(),
        prompts: HashMap::new(),
        kb_documents: HashMap::new(),
    }
}

fn agent_version_key(agent_id: &str, agent_version: &str) -> String {
    format!("{agent_id}:{agent_version}")
}

// ---------------------------------------------------------------------------
// aws_bedrockagent_agent
// ---------------------------------------------------------------------------

/// Converts `aws_bedrockagent_agent` Terraform resources to/from Bedrock Agent state.
pub struct AwsBedrockagentAgentConverter {
    service: Arc<BedrockAgentService>,
}

impl AwsBedrockagentAgentConverter {
    pub fn new(service: Arc<BedrockAgentService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsBedrockagentAgentConverter {
    fn resource_type(&self) -> &str {
        "aws_bedrockagent_agent"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsBedrockagentAgentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: bedrockagent_gen::AgentTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_bedrockagent_agent", e))?;

        let agent_id = model
            .agent_id
            .or(model.id)
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string().replace('-', "")[..10].to_string());
        let agent_arn = model.agent_arn.unwrap_or_else(|| {
            format!(
                "arn:aws:bedrock:{}:{}:agent/{}",
                region, ctx.default_account_id, agent_id
            )
        });
        let agent_version = model.agent_version.unwrap_or_else(|| "DRAFT".to_string());
        let agent_status = model
            .agent_status
            .unwrap_or_else(|| "NOT_PREPARED".to_string());
        let agent_resource_role_arn = model.agent_resource_role_arn.unwrap_or_default();
        let now = chrono::Utc::now().to_rfc3339();
        let created_at = model.created_at.unwrap_or_else(|| now.clone());
        let updated_at = model.updated_at.unwrap_or_else(|| now.clone());
        let prepared_at = model.prepared_at.unwrap_or_else(|| now.clone());
        let idle_session_ttl_in_seconds = attrs
            .get("idle_session_ttl_in_seconds")
            .and_then(|v| v.as_i64());
        let failure_reasons = attrs
            .get("failure_reasons")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let recommended_actions = attrs
            .get("recommended_actions")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let _prepare_agent = attrs
            .get("prepare_agent")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let mut _tags = model.tags;
        if let Some(obj) = attrs.get("tags_all").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    _tags.entry(k.clone()).or_insert_with(|| s.to_string());
                }
            }
        }

        let agent_view = AgentView {
            agent_id: agent_id.clone(),
            agent_name: model.agent_name,
            agent_arn,
            agent_version,
            client_token: model.client_token,
            instruction: model.instruction,
            agent_status,
            foundation_model: model.foundation_model,
            description: model.description,
            idle_session_ttl_in_seconds,
            agent_resource_role_arn,
            customer_encryption_key_arn: model.customer_encryption_key_arn,
            created_at,
            updated_at,
            prepared_at,
            failure_reasons,
            recommended_actions,
        };

        let mut state_view = minimal_state_view();
        state_view.agents.insert(agent_id, agent_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for agent in view.agents.values() {
            let attrs = serde_json::json!({
                "id": agent.agent_id,
                "agent_id": agent.agent_id,
                "agent_name": agent.agent_name,
                "agent_arn": agent.agent_arn,
                "agent_version": agent.agent_version,
                "client_token": agent.client_token,
                "instruction": agent.instruction,
                "agent_status": agent.agent_status,
                "foundation_model": agent.foundation_model,
                "description": agent.description,
                "idle_session_ttl_in_seconds": agent.idle_session_ttl_in_seconds,
                "agent_resource_role_arn": agent.agent_resource_role_arn,
                "customer_encryption_key_arn": agent.customer_encryption_key_arn,
                "created_at": agent.created_at,
                "updated_at": agent.updated_at,
                "prepared_at": agent.prepared_at,
                "failure_reasons": agent.failure_reasons,
                "recommended_actions": agent.recommended_actions,
                "tags_all": {},
                "prepare_agent": true,
            });
            results.push(ExtractedResource {
                name: agent.agent_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_bedrockagent_knowledge_base
// ---------------------------------------------------------------------------

/// Converts `aws_bedrockagent_knowledge_base` Terraform resources to/from Bedrock Agent state.
pub struct AwsBedrockagentKnowledgeBaseConverter {
    service: Arc<BedrockAgentService>,
}

impl AwsBedrockagentKnowledgeBaseConverter {
    pub fn new(service: Arc<BedrockAgentService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsBedrockagentKnowledgeBaseConverter {
    fn resource_type(&self) -> &str {
        "aws_bedrockagent_knowledge_base"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsBedrockagentKnowledgeBaseConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: bedrockagent_gen::KnowledgeBaseTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_bedrockagent_knowledge_base", e))?;

        let kb_id = model
            .knowledge_base_id
            .or(model.id)
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string().replace('-', "")[..10].to_string());
        let kb_arn = model.knowledge_base_arn.unwrap_or_else(|| {
            format!(
                "arn:aws:bedrock:{}:{}:knowledge-base/{}",
                region, ctx.default_account_id, kb_id
            )
        });
        let role_arn = model.role_arn.unwrap_or_default();
        let knowledge_base_configuration = attrs
            .get("knowledge_base_configuration")
            .cloned()
            .unwrap_or(serde_json::Value::Object(serde_json::Map::new()));
        let storage_configuration = attrs
            .get("storage_configuration")
            .cloned()
            .unwrap_or(serde_json::Value::Object(serde_json::Map::new()));
        let status = model.status.unwrap_or_else(|| "ACTIVE".to_string());
        let now = chrono::Utc::now().to_rfc3339();
        let created_at = model.created_at.unwrap_or_else(|| now.clone());
        let updated_at = model.updated_at.unwrap_or_else(|| now.clone());
        let failure_reasons = attrs
            .get("failure_reasons")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let kb_view = KnowledgeBaseView {
            knowledge_base_id: kb_id.clone(),
            name: model.name,
            knowledge_base_arn: kb_arn,
            description: model.description,
            role_arn,
            knowledge_base_configuration,
            storage_configuration,
            status,
            created_at,
            updated_at,
            failure_reasons,
        };

        let mut state_view = minimal_state_view();
        state_view.knowledge_bases.insert(kb_id, kb_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for kb in view.knowledge_bases.values() {
            let attrs = serde_json::json!({
                "id": kb.knowledge_base_id,
                "knowledge_base_id": kb.knowledge_base_id,
                "name": kb.name,
                "knowledge_base_arn": kb.knowledge_base_arn,
                "description": kb.description,
                "role_arn": kb.role_arn,
                "knowledge_base_configuration": kb.knowledge_base_configuration,
                "storage_configuration": kb.storage_configuration,
                "status": kb.status,
                "created_at": kb.created_at,
                "updated_at": kb.updated_at,
                "failure_reasons": kb.failure_reasons,
            });
            results.push(ExtractedResource {
                name: kb.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_bedrockagent_agent_action_group
// ---------------------------------------------------------------------------

/// Converts `aws_bedrockagent_agent_action_group` Terraform resources to/from
/// Bedrock Agent state.
pub struct AwsBedrockagentAgentActionGroupConverter {
    service: Arc<BedrockAgentService>,
}

impl AwsBedrockagentAgentActionGroupConverter {
    pub fn new(service: Arc<BedrockAgentService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsBedrockagentAgentActionGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_bedrockagent_agent_action_group"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_bedrockagent_agent"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsBedrockagentAgentActionGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: bedrockagent_gen::AgentActionGroupTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_bedrockagent_agent_action_group", e)
            })?;

        let action_group_id = model
            .action_group_id
            .clone()
            .or_else(|| {
                model
                    .id
                    .as_ref()
                    // Terraform writes "agent_id,action_group_id,agent_version".
                    .and_then(|id| id.split(',').nth(1).map(|s| s.to_string()))
            })
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string()[..8].to_string());
        let action_group_state = model
            .action_group_state
            .unwrap_or_else(|| "ENABLED".to_string());
        let now = chrono::Utc::now().to_rfc3339();
        let created_at = model.created_at.unwrap_or_else(|| now.clone());
        let updated_at = model.updated_at.unwrap_or_else(|| now.clone());
        let action_group_executor = attrs
            .get("action_group_executor")
            .and_then(|v| serde_json::from_value(v.clone()).ok());
        let api_schema = attrs
            .get("api_schema")
            .and_then(|v| serde_json::from_value(v.clone()).ok());
        let function_schema = attrs
            .get("function_schema")
            .and_then(|v| serde_json::from_value(v.clone()).ok());

        let group = AgentActionGroup {
            action_group_id: Some(action_group_id.clone()),
            action_group_name: Some(model.action_group_name),
            action_group_state: Some(action_group_state),
            agent_id: Some(model.agent_id.clone()),
            agent_version: Some(model.agent_version.clone()),
            client_token: model.client_token,
            created_at: Some(created_at),
            description: model.description,
            parent_action_signature: model.parent_action_group_signature,
            updated_at: Some(updated_at),
            action_group_executor,
            api_schema,
            function_schema,
            parent_action_group_signature_params: None,
        };

        let key = agent_version_key(&model.agent_id, &model.agent_version);
        let mut state_view = minimal_state_view();
        state_view.agent_action_groups.insert(
            key,
            vec![serde_json::to_value(&group).unwrap_or(serde_json::Value::Null)],
        );
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for (key, groups) in view.agent_action_groups.iter() {
            // key = "agent_id:agent_version"
            let (agent_id, agent_version) = key
                .split_once(':')
                .map(|(a, v)| (a.to_string(), v.to_string()))
                .unwrap_or_else(|| (key.clone(), "DRAFT".to_string()));
            for raw in groups {
                let action_group_id = raw
                    .get("actionGroupId")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                let action_group_name = raw
                    .get("actionGroupName")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                let action_group_state = raw
                    .get("actionGroupState")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let description = raw
                    .get("description")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let parent_action_group_signature = raw
                    .get("parentActionSignature")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let created_at = raw
                    .get("createdAt")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let updated_at = raw
                    .get("updatedAt")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let action_group_executor = raw.get("actionGroupExecutor").cloned();
                let api_schema = raw.get("apiSchema").cloned();
                let function_schema = raw.get("functionSchema").cloned();

                let id = format!("{agent_id},{action_group_id},{agent_version}");
                let attrs = serde_json::json!({
                    "id": id,
                    "agent_id": agent_id,
                    "agent_version": agent_version,
                    "action_group_id": action_group_id,
                    "action_group_name": action_group_name,
                    "action_group_state": action_group_state,
                    "description": description,
                    "parent_action_group_signature": parent_action_group_signature,
                    "client_token": null,
                    "created_at": created_at,
                    "updated_at": updated_at,
                    "action_group_executor": action_group_executor,
                    "api_schema": api_schema,
                    "function_schema": function_schema,
                });
                results.push(ExtractedResource {
                    name: action_group_name,
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_bedrockagent_agent_alias
// ---------------------------------------------------------------------------

/// Converts `aws_bedrockagent_agent_alias` Terraform resources to/from
/// Bedrock Agent state.
pub struct AwsBedrockagentAgentAliasConverter {
    service: Arc<BedrockAgentService>,
}

impl AwsBedrockagentAgentAliasConverter {
    pub fn new(service: Arc<BedrockAgentService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsBedrockagentAgentAliasConverter {
    fn resource_type(&self) -> &str {
        "aws_bedrockagent_agent_alias"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_bedrockagent_agent"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsBedrockagentAgentAliasConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: bedrockagent_gen::AgentAliasTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_bedrockagent_agent_alias", e))?;

        let agent_alias_id = model
            .agent_alias_id
            .clone()
            .or_else(|| {
                model
                    .id
                    .as_ref()
                    // Terraform writes "agent_alias_id,agent_id".
                    .and_then(|id| id.split(',').next().map(|s| s.to_string()))
            })
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string()[..8].to_string());
        let agent_alias_arn = model.agent_alias_arn.unwrap_or_else(|| {
            format!(
                "arn:aws:bedrock:{}:{}:agent-alias/{}/{}",
                region, ctx.default_account_id, model.agent_id, agent_alias_id
            )
        });
        let agent_alias_status = model
            .agent_alias_status
            .unwrap_or_else(|| "PREPARED".to_string());
        let now = chrono::Utc::now().to_rfc3339();
        let created_at = model.created_at.unwrap_or_else(|| now.clone());
        let updated_at = model.updated_at.unwrap_or_else(|| now.clone());
        let mut _tags = model.tags;
        if let Some(obj) = attrs.get("tags_all").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    _tags.entry(k.clone()).or_insert_with(|| s.to_string());
                }
            }
        }

        let alias = AgentAliasView {
            agent_id: model.agent_id.clone(),
            agent_alias_id: agent_alias_id.clone(),
            agent_alias_name: model.agent_alias_name,
            agent_alias_arn,
            agent_alias_status,
            created_at,
            updated_at,
            description: model.description,
        };

        let mut state_view = minimal_state_view();
        state_view.agent_aliases.insert(model.agent_id, vec![alias]);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for aliases in view.agent_aliases.values() {
            for alias in aliases {
                let id = format!("{},{}", alias.agent_alias_id, alias.agent_id);
                let attrs = serde_json::json!({
                    "id": id,
                    "agent_id": alias.agent_id,
                    "agent_alias_id": alias.agent_alias_id,
                    "agent_alias_name": alias.agent_alias_name,
                    "agent_alias_arn": alias.agent_alias_arn,
                    "agent_alias_status": alias.agent_alias_status,
                    "description": alias.description,
                    "client_token": null,
                    "created_at": alias.created_at,
                    "updated_at": alias.updated_at,
                    "tags_all": {},
                });
                results.push(ExtractedResource {
                    name: alias.agent_alias_name.clone(),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_bedrockagent_agent_collaborator
// ---------------------------------------------------------------------------

/// Converts `aws_bedrockagent_agent_collaborator` Terraform resources to/from
/// Bedrock Agent state.
pub struct AwsBedrockagentAgentCollaboratorConverter {
    service: Arc<BedrockAgentService>,
}

impl AwsBedrockagentAgentCollaboratorConverter {
    pub fn new(service: Arc<BedrockAgentService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsBedrockagentAgentCollaboratorConverter {
    fn resource_type(&self) -> &str {
        "aws_bedrockagent_agent_collaborator"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_bedrockagent_agent"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsBedrockagentAgentCollaboratorConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: bedrockagent_gen::AgentCollaboratorTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_bedrockagent_agent_collaborator", e)
            })?;

        let collaborator_id = model
            .collaborator_id
            .clone()
            .or_else(|| {
                model
                    .id
                    .as_ref()
                    // Terraform writes "agent_id,agent_version,collaborator_id".
                    .and_then(|id| id.split(',').nth(2).map(|s| s.to_string()))
            })
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string()[..8].to_string());
        let now = chrono::Utc::now().to_rfc3339();
        let created_at = model.created_at.unwrap_or_else(|| now.clone());
        let last_updated_at = model.last_updated_at.unwrap_or_else(|| now.clone());
        let agent_descriptor = attrs
            .get("agent_descriptor")
            .and_then(|v| serde_json::from_value(v.clone()).ok());

        let collaborator = AgentCollaborator {
            agent_descriptor,
            agent_id: Some(model.agent_id.clone()),
            agent_version: Some(model.agent_version.clone()),
            client_token: model.client_token,
            collaboration_instruction: model.collaboration_instruction,
            collaborator_id: Some(collaborator_id.clone()),
            collaborator_name: Some(model.collaborator_name),
            created_at: Some(created_at),
            last_updated_at: Some(last_updated_at),
            relay_conversation_history: model.relay_conversation_history,
        };

        let key = agent_version_key(&model.agent_id, &model.agent_version);
        let mut state_view = minimal_state_view();
        state_view.agent_collaborators.insert(
            key,
            vec![serde_json::to_value(&collaborator).unwrap_or(serde_json::Value::Null)],
        );
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for (key, collabs) in view.agent_collaborators.iter() {
            let (agent_id, agent_version) = key
                .split_once(':')
                .map(|(a, v)| (a.to_string(), v.to_string()))
                .unwrap_or_else(|| (key.clone(), "DRAFT".to_string()));
            for raw in collabs {
                let collaborator_id = raw
                    .get("collaboratorId")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                let collaborator_name = raw
                    .get("collaboratorName")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                let collaboration_instruction = raw
                    .get("collaborationInstruction")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let relay_conversation_history = raw
                    .get("relayConversationHistory")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let created_at = raw
                    .get("createdAt")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let last_updated_at = raw
                    .get("lastUpdatedAt")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let agent_descriptor = raw.get("agentDescriptor").cloned();

                let id = format!("{agent_id},{agent_version},{collaborator_id}");
                let attrs = serde_json::json!({
                    "id": id,
                    "agent_id": agent_id,
                    "agent_version": agent_version,
                    "collaborator_id": collaborator_id,
                    "collaborator_name": collaborator_name,
                    "collaboration_instruction": collaboration_instruction,
                    "relay_conversation_history": relay_conversation_history,
                    "client_token": null,
                    "created_at": created_at,
                    "last_updated_at": last_updated_at,
                    "agent_descriptor": agent_descriptor,
                });
                results.push(ExtractedResource {
                    name: collaborator_name,
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_bedrockagent_agent_knowledge_base_association
// ---------------------------------------------------------------------------

/// Converts `aws_bedrockagent_agent_knowledge_base_association` Terraform
/// resources to/from Bedrock Agent state.
pub struct AwsBedrockagentAgentKnowledgeBaseAssociationConverter {
    service: Arc<BedrockAgentService>,
}

impl AwsBedrockagentAgentKnowledgeBaseAssociationConverter {
    pub fn new(service: Arc<BedrockAgentService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsBedrockagentAgentKnowledgeBaseAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_bedrockagent_agent_knowledge_base_association"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_bedrockagent_agent", "aws_bedrockagent_knowledge_base"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsBedrockagentAgentKnowledgeBaseAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: bedrockagent_gen::AgentKnowledgeBaseAssociationTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_bedrockagent_agent_knowledge_base_association", e)
            })?;

        let agent_version = model.agent_version.unwrap_or_else(|| "DRAFT".to_string());
        let knowledge_base_state = model
            .knowledge_base_state
            .unwrap_or_else(|| "ENABLED".to_string());
        let now = chrono::Utc::now().to_rfc3339();
        let created_at = model.created_at.unwrap_or_else(|| now.clone());
        let updated_at = model.updated_at.unwrap_or_else(|| now.clone());

        let assoc = AgentKnowledgeBase {
            agent_id: Some(model.agent_id.clone()),
            agent_version: Some(agent_version.clone()),
            created_at: Some(created_at),
            description: Some(model.description),
            knowledge_base_id: Some(model.knowledge_base_id.clone()),
            knowledge_base_state: Some(knowledge_base_state),
            updated_at: Some(updated_at),
        };

        let key = agent_version_key(&model.agent_id, &agent_version);
        let mut state_view = minimal_state_view();
        state_view.agent_knowledge_bases.insert(key, vec![assoc]);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for (key, assocs) in view.agent_knowledge_bases.iter() {
            let (agent_id, agent_version) = key
                .split_once(':')
                .map(|(a, v)| (a.to_string(), v.to_string()))
                .unwrap_or_else(|| (key.clone(), "DRAFT".to_string()));
            for assoc in assocs {
                let kb_id = assoc.knowledge_base_id.clone().unwrap_or_default();
                let id = format!("{agent_id},{agent_version},{kb_id}");
                let attrs = serde_json::json!({
                    "id": id,
                    "agent_id": agent_id,
                    "agent_version": agent_version,
                    "knowledge_base_id": kb_id,
                    "description": assoc.description.clone().unwrap_or_default(),
                    "knowledge_base_state": assoc.knowledge_base_state,
                    "created_at": assoc.created_at,
                    "updated_at": assoc.updated_at,
                });
                results.push(ExtractedResource {
                    name: format!("{agent_id}/{kb_id}"),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_bedrockagent_data_source
// ---------------------------------------------------------------------------

/// Converts `aws_bedrockagent_data_source` Terraform resources to/from
/// Bedrock Agent state.
pub struct AwsBedrockagentDataSourceConverter {
    service: Arc<BedrockAgentService>,
}

impl AwsBedrockagentDataSourceConverter {
    pub fn new(service: Arc<BedrockAgentService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsBedrockagentDataSourceConverter {
    fn resource_type(&self) -> &str {
        "aws_bedrockagent_data_source"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_bedrockagent_knowledge_base"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsBedrockagentDataSourceConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: bedrockagent_gen::DataSourceTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_bedrockagent_data_source", e))?;

        let data_source_id = model
            .data_source_id
            .clone()
            .or_else(|| {
                model
                    .id
                    .as_ref()
                    // Terraform writes "data_source_id,knowledge_base_id".
                    .and_then(|id| id.split(',').next().map(|s| s.to_string()))
            })
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string()[..8].to_string());
        let status = model.status.unwrap_or_else(|| "AVAILABLE".to_string());
        let now = chrono::Utc::now().to_rfc3339();
        let created_at = model.created_at.unwrap_or_else(|| now.clone());
        let updated_at = model.updated_at.unwrap_or_else(|| now.clone());
        let data_source_configuration = attrs
            .get("data_source_configuration")
            .and_then(|v| serde_json::from_value(v.clone()).ok());
        let vector_ingestion_configuration = attrs
            .get("vector_ingestion_configuration")
            .and_then(|v| serde_json::from_value(v.clone()).ok());
        let server_side_encryption_configuration = attrs
            .get("server_side_encryption_configuration")
            .and_then(|v| serde_json::from_value(v.clone()).ok());

        let ds = DataSource {
            created_at: Some(created_at),
            data_deletion_policy: model.data_deletion_policy,
            data_source_configuration,
            data_source_id: Some(data_source_id.clone()),
            description: model.description,
            failure_reasons: None,
            knowledge_base_id: Some(model.knowledge_base_id.clone()),
            name: Some(model.name),
            server_side_encryption_configuration,
            status: Some(status),
            updated_at: Some(updated_at),
            vector_ingestion_configuration,
        };

        let mut state_view = minimal_state_view();
        state_view.data_sources.insert(
            model.knowledge_base_id,
            vec![serde_json::to_value(&ds).unwrap_or(serde_json::Value::Null)],
        );
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for sources in view.data_sources.values() {
            for raw in sources {
                let data_source_id = raw
                    .get("dataSourceId")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                let knowledge_base_id = raw
                    .get("knowledgeBaseId")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                let name = raw
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                let description = raw
                    .get("description")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let data_deletion_policy = raw
                    .get("dataDeletionPolicy")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let status = raw
                    .get("status")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let created_at = raw
                    .get("createdAt")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let updated_at = raw
                    .get("updatedAt")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                let data_source_configuration = raw.get("dataSourceConfiguration").cloned();
                let vector_ingestion_configuration =
                    raw.get("vectorIngestionConfiguration").cloned();
                let server_side_encryption_configuration =
                    raw.get("serverSideEncryptionConfiguration").cloned();

                let id = format!("{data_source_id},{knowledge_base_id}");
                let attrs = serde_json::json!({
                    "id": id,
                    "data_source_id": data_source_id,
                    "knowledge_base_id": knowledge_base_id,
                    "name": name,
                    "description": description,
                    "data_deletion_policy": data_deletion_policy,
                    "status": status,
                    "created_at": created_at,
                    "updated_at": updated_at,
                    "data_source_configuration": data_source_configuration,
                    "vector_ingestion_configuration": vector_ingestion_configuration,
                    "server_side_encryption_configuration": server_side_encryption_configuration,
                });
                results.push(ExtractedResource {
                    name,
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_bedrockagent_prompt
// ---------------------------------------------------------------------------

/// Converts `aws_bedrockagent_prompt` Terraform resources to/from Bedrock
/// Agent state.
pub struct AwsBedrockagentPromptConverter {
    service: Arc<BedrockAgentService>,
}

impl AwsBedrockagentPromptConverter {
    pub fn new(service: Arc<BedrockAgentService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsBedrockagentPromptConverter {
    fn resource_type(&self) -> &str {
        "aws_bedrockagent_prompt"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsBedrockagentPromptConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: bedrockagent_gen::PromptTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_bedrockagent_prompt", e))?;

        let id = model
            .id
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string()[..10].to_string());
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:bedrock:{}:{}:prompt/{}",
                region, ctx.default_account_id, id
            )
        });
        let now = chrono::Utc::now().to_rfc3339();
        let created_at = model.created_at.unwrap_or_else(|| now.clone());
        let updated_at = model.updated_at.unwrap_or_else(|| now.clone());
        let mut _tags = model.tags;
        if let Some(obj) = attrs.get("tags_all").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    _tags.entry(k.clone()).or_insert_with(|| s.to_string());
                }
            }
        }

        let prompt = PromptView {
            id: id.clone(),
            arn,
            name: model.name,
            created_at,
            updated_at,
            description: model.description,
        };

        let mut state_view = minimal_state_view();
        state_view.prompts.insert(id, prompt);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for prompt in view.prompts.values() {
            let attrs = serde_json::json!({
                "id": prompt.id,
                "arn": prompt.arn,
                "name": prompt.name,
                "description": prompt.description,
                "customer_encryption_key_arn": null,
                "default_variant": null,
                "version": "DRAFT",
                "created_at": prompt.created_at,
                "updated_at": prompt.updated_at,
                "tags_all": {},
            });
            results.push(ExtractedResource {
                name: prompt.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
