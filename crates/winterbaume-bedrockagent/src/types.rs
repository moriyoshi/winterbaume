use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Agent {
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
    pub failure_reasons: Vec<String>,
    pub recommended_actions: Vec<String>,
}

impl Agent {
    pub fn to_dict(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        map.insert("agentId".to_string(), serde_json::json!(self.agent_id));
        map.insert("agentName".to_string(), serde_json::json!(self.agent_name));
        map.insert("agentArn".to_string(), serde_json::json!(self.agent_arn));
        map.insert(
            "agentVersion".to_string(),
            serde_json::json!(self.agent_version),
        );
        map.insert(
            "agentStatus".to_string(),
            serde_json::json!(self.agent_status),
        );
        map.insert(
            "agentResourceRoleArn".to_string(),
            serde_json::json!(self.agent_resource_role_arn),
        );
        map.insert("createdAt".to_string(), serde_json::json!(self.created_at));
        map.insert("updatedAt".to_string(), serde_json::json!(self.updated_at));
        map.insert(
            "preparedAt".to_string(),
            serde_json::json!(self.prepared_at),
        );
        map.insert(
            "failureReasons".to_string(),
            serde_json::json!(self.failure_reasons),
        );
        map.insert(
            "recommendedActions".to_string(),
            serde_json::json!(self.recommended_actions),
        );

        if let Some(ref v) = self.client_token {
            map.insert("clientToken".to_string(), serde_json::json!(v));
        }
        if let Some(ref v) = self.instruction {
            map.insert("instruction".to_string(), serde_json::json!(v));
        }
        if let Some(ref v) = self.foundation_model {
            map.insert("foundationModel".to_string(), serde_json::json!(v));
        }
        if let Some(ref v) = self.description {
            map.insert("description".to_string(), serde_json::json!(v));
        }
        if let Some(v) = self.idle_session_ttl_in_seconds {
            map.insert("idleSessionTTLInSeconds".to_string(), serde_json::json!(v));
        }
        if let Some(ref v) = self.customer_encryption_key_arn {
            map.insert("customerEncryptionKeyArn".to_string(), serde_json::json!(v));
        }

        serde_json::Value::Object(map)
    }

    pub fn dict_summary(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        map.insert("agentId".to_string(), serde_json::json!(self.agent_id));
        map.insert("agentName".to_string(), serde_json::json!(self.agent_name));
        map.insert(
            "agentStatus".to_string(),
            serde_json::json!(self.agent_status),
        );
        map.insert("updatedAt".to_string(), serde_json::json!(self.updated_at));
        map.insert(
            "latestAgentVersion".to_string(),
            serde_json::json!(self.agent_version),
        );
        if let Some(ref v) = self.description {
            map.insert("description".to_string(), serde_json::json!(v));
        }
        serde_json::Value::Object(map)
    }
}

#[derive(Debug, Clone)]
pub struct KnowledgeBase {
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
    pub failure_reasons: Vec<String>,
}

impl KnowledgeBase {
    pub fn to_dict(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        map.insert(
            "knowledgeBaseId".to_string(),
            serde_json::json!(self.knowledge_base_id),
        );
        map.insert("name".to_string(), serde_json::json!(self.name));
        map.insert(
            "knowledgeBaseArn".to_string(),
            serde_json::json!(self.knowledge_base_arn),
        );
        map.insert("roleArn".to_string(), serde_json::json!(self.role_arn));
        map.insert(
            "knowledgeBaseConfiguration".to_string(),
            self.knowledge_base_configuration.clone(),
        );
        map.insert(
            "storageConfiguration".to_string(),
            self.storage_configuration.clone(),
        );
        map.insert("status".to_string(), serde_json::json!(self.status));
        map.insert("createdAt".to_string(), serde_json::json!(self.created_at));
        map.insert("updatedAt".to_string(), serde_json::json!(self.updated_at));
        if let Some(ref v) = self.description {
            map.insert("description".to_string(), serde_json::json!(v));
        }
        if !self.failure_reasons.is_empty() {
            map.insert(
                "failureReasons".to_string(),
                serde_json::json!(self.failure_reasons),
            );
        }
        serde_json::Value::Object(map)
    }

    pub fn dict_summary(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        map.insert(
            "knowledgeBaseId".to_string(),
            serde_json::json!(self.knowledge_base_id),
        );
        map.insert("name".to_string(), serde_json::json!(self.name));
        map.insert("status".to_string(), serde_json::json!(self.status));
        map.insert("updatedAt".to_string(), serde_json::json!(self.updated_at));
        if let Some(ref v) = self.description {
            map.insert("description".to_string(), serde_json::json!(v));
        }
        serde_json::Value::Object(map)
    }
}

/// Tags stored per resource ARN.
pub type TagStore = HashMap<String, HashMap<String, String>>;
