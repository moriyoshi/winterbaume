//! Terraform converter for Bedrock Agent resources.

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
use crate::util::{
    extract_region, extract_tags, optional_bool, optional_i64, optional_str, require_str,
};

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
        let agent_name = require_str(attrs, "agent_name", "aws_bedrockagent_agent")?;
        let region = extract_region(attrs, &ctx.default_region);
        let agent_id = optional_str(attrs, "agent_id")
            .or_else(|| optional_str(attrs, "id"))
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string().replace('-', "")[..10].to_string());
        let agent_arn = optional_str(attrs, "agent_arn").unwrap_or_else(|| {
            format!(
                "arn:aws:bedrock:{}:{}:agent/{}",
                region, ctx.default_account_id, agent_id
            )
        });
        let agent_version =
            optional_str(attrs, "agent_version").unwrap_or_else(|| "DRAFT".to_string());
        let client_token = optional_str(attrs, "client_token");
        let instruction = optional_str(attrs, "instruction");
        let agent_status =
            optional_str(attrs, "agent_status").unwrap_or_else(|| "NOT_PREPARED".to_string());
        let foundation_model = optional_str(attrs, "foundation_model");
        let description = optional_str(attrs, "description");
        let idle_session_ttl_in_seconds = optional_i64(attrs, "idle_session_ttl_in_seconds");
        let agent_resource_role_arn =
            optional_str(attrs, "agent_resource_role_arn").unwrap_or_default();
        let customer_encryption_key_arn = optional_str(attrs, "customer_encryption_key_arn");
        let now = chrono::Utc::now().to_rfc3339();
        let created_at = optional_str(attrs, "created_at").unwrap_or_else(|| now.clone());
        let updated_at = optional_str(attrs, "updated_at").unwrap_or_else(|| now.clone());
        let prepared_at = optional_str(attrs, "prepared_at").unwrap_or_else(|| now.clone());
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
        let _prepare_agent = optional_bool(attrs, "prepare_agent").unwrap_or(true);
        let mut _tags = extract_tags(attrs);
        if let Some(obj) = attrs.get("tags_all").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    _tags.entry(k.clone()).or_insert_with(|| s.to_string());
                }
            }
        }

        let agent_view = AgentView {
            agent_id: agent_id.clone(),
            agent_name: agent_name.to_string(),
            agent_arn,
            agent_version,
            client_token,
            instruction,
            agent_status,
            foundation_model,
            description,
            idle_session_ttl_in_seconds,
            agent_resource_role_arn,
            customer_encryption_key_arn,
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
        let name = require_str(attrs, "name", "aws_bedrockagent_knowledge_base")?;
        let region = extract_region(attrs, &ctx.default_region);
        let kb_id = optional_str(attrs, "knowledge_base_id")
            .or_else(|| optional_str(attrs, "id"))
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string().replace('-', "")[..10].to_string());
        let kb_arn = optional_str(attrs, "knowledge_base_arn").unwrap_or_else(|| {
            format!(
                "arn:aws:bedrock:{}:{}:knowledge-base/{}",
                region, ctx.default_account_id, kb_id
            )
        });
        let description = optional_str(attrs, "description");
        let role_arn = optional_str(attrs, "role_arn").unwrap_or_default();
        let knowledge_base_configuration = attrs
            .get("knowledge_base_configuration")
            .cloned()
            .unwrap_or(serde_json::Value::Object(serde_json::Map::new()));
        let storage_configuration = attrs
            .get("storage_configuration")
            .cloned()
            .unwrap_or(serde_json::Value::Object(serde_json::Map::new()));
        let status = optional_str(attrs, "status").unwrap_or_else(|| "ACTIVE".to_string());
        let now = chrono::Utc::now().to_rfc3339();
        let created_at = optional_str(attrs, "created_at").unwrap_or_else(|| now.clone());
        let updated_at = optional_str(attrs, "updated_at").unwrap_or_else(|| now.clone());
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
            name: name.to_string(),
            knowledge_base_arn: kb_arn,
            description,
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
