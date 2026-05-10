//! Terraform converters for Bedrock Agent resources.
//!
//! `AgentTfModel` and `KnowledgeBaseTfModel` are generated from
//! `specs/bedrockagent.toml`. The ARN templates, the synthesised IDs,
//! the constants (`agent_version = "DRAFT"`, `agent_status =
//! "NOT_PREPARED"`, `status = "ACTIVE"`), the `Option<i64>`
//! `idle_session_ttl_in_seconds`, the `failure_reasons` /
//! `recommended_actions` Vec<String> fields, the
//! `knowledge_base_configuration` / `storage_configuration` raw JSON
//! blobs, and the `tags_all` overlay are wired up here.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_bedrockagent::BedrockAgentService;
use winterbaume_bedrockagent::views::{AgentView, BedrockAgentStateView, KnowledgeBaseView};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::bedrockagent as bedrockagent_gen;
use crate::util::{classify_deserialize_error, extract_region};

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

        let mut state_view = BedrockAgentStateView {
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
        };
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

        let mut state_view = BedrockAgentStateView {
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
        };
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
