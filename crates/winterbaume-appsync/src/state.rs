use std::collections::HashMap;

use crate::types::*;

#[derive(Debug, Default)]
pub struct AppSyncState {
    pub apis: HashMap<String, GraphqlApi>,
    pub event_apis: HashMap<String, Api>,
    pub api_caches: HashMap<String, ApiCacheEntry>,
    /// api_id -> Vec<ApiKeyEntry>
    pub api_keys: HashMap<String, Vec<ApiKeyEntry>>,
    /// api_id -> Vec<ChannelNamespaceEntry>
    pub channel_namespaces: HashMap<String, Vec<ChannelNamespaceEntry>>,
    /// api_id -> Vec<TypeEntry>
    pub types: HashMap<String, Vec<TypeEntry>>,
    /// api_id -> SchemaStatus
    pub schema_statuses: HashMap<String, SchemaStatus>,
    /// resource_arn -> tags
    pub resource_tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, thiserror::Error)]
pub enum AppSyncError {
    #[error("GraphQL API {api_id} not found.")]
    GraphqlApiNotFound { api_id: String },

    #[error("Api {api_id} not found.")]
    ApiNotFound { api_id: String },

    #[error("API cache not found for API {api_id}.")]
    ApiCacheNotFound { api_id: String },

    #[error("API cache already exists for API {api_id}.")]
    ApiCacheAlreadyExists { api_id: String },

    #[error("API key {key_id} not found.")]
    ApiKeyNotFound { key_id: String },

    #[error("Channel namespace {name} not found.")]
    ChannelNamespaceNotFound { name: String },

    #[error("Schema not found for API {api_id}.")]
    SchemaNotFound { api_id: String },

    #[error("Type {type_name} not found.")]
    TypeNotFound { type_name: String },
}

impl AppSyncState {
    // ===================== GraphQL API (v1) =====================

    pub fn create_graphql_api(
        &mut self,
        name: &str,
        authentication_type: &str,
        account_id: &str,
        region: &str,
        tags: HashMap<String, String>,
    ) -> Result<&GraphqlApi, AppSyncError> {
        let api_id = uuid::Uuid::new_v4().to_string();
        let arn = format!("arn:aws:appsync:{region}:{account_id}:apis/{api_id}");
        let graphql_uri = format!("https://{api_id}.appsync-api.{region}.amazonaws.com/graphql");
        let realtime_uri =
            format!("wss://{api_id}.appsync-realtime-api.{region}.amazonaws.com/graphql");

        let mut uris = HashMap::new();
        uris.insert("GRAPHQL".to_string(), graphql_uri);
        uris.insert("REALTIME".to_string(), realtime_uri);

        let api = GraphqlApi {
            api_id: api_id.clone(),
            name: name.to_string(),
            authentication_type: authentication_type.to_string(),
            arn: arn.clone(),
            uris,
            tags: tags.clone(),
            xray_enabled: false,
            additional_authentication_provider: None,
            lambda_authorizer_config: None,
            user_pool_config: None,
            enhanced_metrics_config: None,
        };

        if !tags.is_empty() {
            self.resource_tags.insert(arn, tags);
        }

        self.apis.insert(api_id.clone(), api);
        Ok(self.apis.get(&api_id).unwrap())
    }

    pub fn get_graphql_api(&self, api_id: &str) -> Result<&GraphqlApi, AppSyncError> {
        self.apis
            .get(api_id)
            .ok_or_else(|| AppSyncError::GraphqlApiNotFound {
                api_id: api_id.to_string(),
            })
    }

    pub fn delete_graphql_api(&mut self, api_id: &str) -> Result<(), AppSyncError> {
        if self.apis.remove(api_id).is_none() {
            return Err(AppSyncError::GraphqlApiNotFound {
                api_id: api_id.to_string(),
            });
        }
        self.api_caches.remove(api_id);
        self.api_keys.remove(api_id);
        self.types.remove(api_id);
        self.schema_statuses.remove(api_id);
        Ok(())
    }

    pub fn list_graphql_apis(&self) -> Vec<&GraphqlApi> {
        self.apis.values().collect()
    }

    pub fn update_graphql_api(
        &mut self,
        api_id: &str,
        name: Option<&str>,
        authentication_type: Option<&str>,
    ) -> Result<&GraphqlApi, AppSyncError> {
        let api = self
            .apis
            .get_mut(api_id)
            .ok_or_else(|| AppSyncError::GraphqlApiNotFound {
                api_id: api_id.to_string(),
            })?;

        if let Some(n) = name {
            api.name = n.to_string();
        }
        if let Some(at) = authentication_type {
            api.authentication_type = at.to_string();
        }

        Ok(api)
    }

    // ===================== Event API (v2) =====================

    pub fn create_api(
        &mut self,
        name: &str,
        account_id: &str,
        region: &str,
        owner_contact: Option<&str>,
        tags: HashMap<String, String>,
    ) -> Result<&Api, AppSyncError> {
        let api_id = uuid::Uuid::new_v4().to_string();
        let api_arn = format!("arn:aws:appsync:{region}:{account_id}:apis/{api_id}");
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();

        let api = Api {
            api_id: api_id.clone(),
            name: name.to_string(),
            api_arn: api_arn.clone(),
            created: now,
            tags: tags.clone(),
            owner_contact: owner_contact.map(|s| s.to_string()),
        };

        if !tags.is_empty() {
            self.resource_tags.insert(api_arn, tags);
        }

        self.event_apis.insert(api_id.clone(), api);
        Ok(self.event_apis.get(&api_id).unwrap())
    }

    pub fn get_api(&self, api_id: &str) -> Result<&Api, AppSyncError> {
        self.event_apis
            .get(api_id)
            .ok_or_else(|| AppSyncError::ApiNotFound {
                api_id: api_id.to_string(),
            })
    }

    pub fn delete_api(&mut self, api_id: &str) -> Result<(), AppSyncError> {
        if self.event_apis.remove(api_id).is_none() {
            return Err(AppSyncError::ApiNotFound {
                api_id: api_id.to_string(),
            });
        }
        self.channel_namespaces.remove(api_id);
        Ok(())
    }

    pub fn list_apis(&self) -> Vec<&Api> {
        self.event_apis.values().collect()
    }

    // ===================== API Cache =====================

    pub fn create_api_cache(
        &mut self,
        api_id: &str,
        api_caching_behavior: &str,
        cache_type: &str,
        ttl: i64,
        at_rest_encryption_enabled: bool,
        transit_encryption_enabled: bool,
        health_metrics_config: Option<&str>,
    ) -> Result<&ApiCacheEntry, AppSyncError> {
        // Verify API exists
        if !self.apis.contains_key(api_id) {
            return Err(AppSyncError::GraphqlApiNotFound {
                api_id: api_id.to_string(),
            });
        }

        if self.api_caches.contains_key(api_id) {
            return Err(AppSyncError::ApiCacheAlreadyExists {
                api_id: api_id.to_string(),
            });
        }

        let entry = ApiCacheEntry {
            api_id: api_id.to_string(),
            api_caching_behavior: api_caching_behavior.to_string(),
            r#type: cache_type.to_string(),
            ttl,
            at_rest_encryption_enabled,
            transit_encryption_enabled,
            status: "AVAILABLE".to_string(),
            health_metrics_config: health_metrics_config.map(|s| s.to_string()),
        };

        self.api_caches.insert(api_id.to_string(), entry);
        Ok(self.api_caches.get(api_id).unwrap())
    }

    pub fn get_api_cache(&self, api_id: &str) -> Result<&ApiCacheEntry, AppSyncError> {
        self.api_caches
            .get(api_id)
            .ok_or_else(|| AppSyncError::ApiCacheNotFound {
                api_id: api_id.to_string(),
            })
    }

    pub fn delete_api_cache(&mut self, api_id: &str) -> Result<(), AppSyncError> {
        if self.api_caches.remove(api_id).is_none() {
            return Err(AppSyncError::ApiCacheNotFound {
                api_id: api_id.to_string(),
            });
        }
        Ok(())
    }

    pub fn update_api_cache(
        &mut self,
        api_id: &str,
        api_caching_behavior: &str,
        cache_type: &str,
        ttl: i64,
        health_metrics_config: Option<&str>,
    ) -> Result<&ApiCacheEntry, AppSyncError> {
        let cache =
            self.api_caches
                .get_mut(api_id)
                .ok_or_else(|| AppSyncError::ApiCacheNotFound {
                    api_id: api_id.to_string(),
                })?;

        cache.api_caching_behavior = api_caching_behavior.to_string();
        cache.r#type = cache_type.to_string();
        cache.ttl = ttl;
        if let Some(hmc) = health_metrics_config {
            cache.health_metrics_config = Some(hmc.to_string());
        }

        Ok(cache)
    }

    pub fn flush_api_cache(&self, api_id: &str) -> Result<(), AppSyncError> {
        if !self.api_caches.contains_key(api_id) {
            return Err(AppSyncError::ApiCacheNotFound {
                api_id: api_id.to_string(),
            });
        }
        // Flush is a no-op in mock
        Ok(())
    }

    // ===================== API Keys =====================

    pub fn create_api_key(
        &mut self,
        api_id: &str,
        description: Option<&str>,
        expires: i64,
    ) -> Result<&ApiKeyEntry, AppSyncError> {
        if !self.apis.contains_key(api_id) {
            return Err(AppSyncError::GraphqlApiNotFound {
                api_id: api_id.to_string(),
            });
        }

        let id = uuid::Uuid::new_v4().to_string()[..8].to_string();
        let key = ApiKeyEntry {
            id: id.clone(),
            api_id: api_id.to_string(),
            description: description.map(|s| s.to_string()),
            expires,
            deletes: expires + 60 * 60 * 24 * 2, // 2 days after expiry
        };

        let keys = self.api_keys.entry(api_id.to_string()).or_default();
        keys.push(key);
        Ok(keys.last().unwrap())
    }

    pub fn delete_api_key(&mut self, api_id: &str, key_id: &str) -> Result<(), AppSyncError> {
        let keys = self
            .api_keys
            .get_mut(api_id)
            .ok_or_else(|| AppSyncError::ApiKeyNotFound {
                key_id: key_id.to_string(),
            })?;

        let idx = keys.iter().position(|k| k.id == key_id).ok_or_else(|| {
            AppSyncError::ApiKeyNotFound {
                key_id: key_id.to_string(),
            }
        })?;

        keys.remove(idx);
        Ok(())
    }

    pub fn list_api_keys(&self, api_id: &str) -> Result<Vec<&ApiKeyEntry>, AppSyncError> {
        if !self.apis.contains_key(api_id) {
            return Err(AppSyncError::GraphqlApiNotFound {
                api_id: api_id.to_string(),
            });
        }
        Ok(self
            .api_keys
            .get(api_id)
            .map(|keys| keys.iter().collect())
            .unwrap_or_default())
    }

    pub fn update_api_key(
        &mut self,
        api_id: &str,
        key_id: &str,
        description: Option<&str>,
        expires: Option<i64>,
    ) -> Result<&ApiKeyEntry, AppSyncError> {
        let keys = self
            .api_keys
            .get_mut(api_id)
            .ok_or_else(|| AppSyncError::ApiKeyNotFound {
                key_id: key_id.to_string(),
            })?;

        let key = keys.iter_mut().find(|k| k.id == key_id).ok_or_else(|| {
            AppSyncError::ApiKeyNotFound {
                key_id: key_id.to_string(),
            }
        })?;

        if let Some(d) = description {
            key.description = Some(d.to_string());
        }
        if let Some(e) = expires {
            key.expires = e;
            key.deletes = e + 60 * 60 * 24 * 2;
        }

        Ok(key)
    }

    // ===================== Channel Namespaces (v2) =====================

    pub fn create_channel_namespace(
        &mut self,
        api_id: &str,
        name: &str,
        account_id: &str,
        region: &str,
        tags: HashMap<String, String>,
    ) -> Result<&ChannelNamespaceEntry, AppSyncError> {
        if !self.event_apis.contains_key(api_id) {
            return Err(AppSyncError::ApiNotFound {
                api_id: api_id.to_string(),
            });
        }

        let arn =
            format!("arn:aws:appsync:{region}:{account_id}:apis/{api_id}/channelNamespaces/{name}");
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();

        let entry = ChannelNamespaceEntry {
            api_id: api_id.to_string(),
            name: name.to_string(),
            channel_namespace_arn: arn.clone(),
            created: now,
            last_modified: now,
            tags: tags.clone(),
        };

        if !tags.is_empty() {
            self.resource_tags.insert(arn, tags);
        }

        let namespaces = self
            .channel_namespaces
            .entry(api_id.to_string())
            .or_default();
        namespaces.push(entry);
        Ok(namespaces.last().unwrap())
    }

    pub fn delete_channel_namespace(
        &mut self,
        api_id: &str,
        name: &str,
    ) -> Result<(), AppSyncError> {
        let namespaces = self.channel_namespaces.get_mut(api_id).ok_or_else(|| {
            AppSyncError::ChannelNamespaceNotFound {
                name: name.to_string(),
            }
        })?;

        let idx = namespaces
            .iter()
            .position(|ns| ns.name == name)
            .ok_or_else(|| AppSyncError::ChannelNamespaceNotFound {
                name: name.to_string(),
            })?;

        namespaces.remove(idx);
        Ok(())
    }

    pub fn list_channel_namespaces(
        &self,
        api_id: &str,
    ) -> Result<Vec<&ChannelNamespaceEntry>, AppSyncError> {
        if !self.event_apis.contains_key(api_id) {
            return Err(AppSyncError::ApiNotFound {
                api_id: api_id.to_string(),
            });
        }
        Ok(self
            .channel_namespaces
            .get(api_id)
            .map(|nss| nss.iter().collect())
            .unwrap_or_default())
    }

    // ===================== Schema Creation =====================

    pub fn start_schema_creation(
        &mut self,
        api_id: &str,
        _definition: &[u8],
    ) -> Result<&SchemaStatus, AppSyncError> {
        if !self.apis.contains_key(api_id) {
            return Err(AppSyncError::GraphqlApiNotFound {
                api_id: api_id.to_string(),
            });
        }

        let status = SchemaStatus {
            status: "SUCCESS".to_string(),
            details: None,
        };

        self.schema_statuses.insert(api_id.to_string(), status);
        Ok(self.schema_statuses.get(api_id).unwrap())
    }

    pub fn get_schema_creation_status(&self, api_id: &str) -> Result<&SchemaStatus, AppSyncError> {
        self.schema_statuses
            .get(api_id)
            .ok_or_else(|| AppSyncError::SchemaNotFound {
                api_id: api_id.to_string(),
            })
    }

    // ===================== Types =====================

    pub fn get_type(
        &self,
        api_id: &str,
        type_name: &str,
        _format: &str,
    ) -> Result<&TypeEntry, AppSyncError> {
        let types = self
            .types
            .get(api_id)
            .ok_or_else(|| AppSyncError::TypeNotFound {
                type_name: type_name.to_string(),
            })?;

        types
            .iter()
            .find(|t| t.name == type_name)
            .ok_or_else(|| AppSyncError::TypeNotFound {
                type_name: type_name.to_string(),
            })
    }

    // ===================== Tag Operations =====================

    pub fn tag_resource(&mut self, arn: &str, tags: HashMap<String, String>) {
        let entry = self.resource_tags.entry(arn.to_string()).or_default();
        entry.extend(tags);
    }

    pub fn untag_resource(&mut self, arn: &str, tag_keys: &[String]) {
        if let Some(tags) = self.resource_tags.get_mut(arn) {
            for key in tag_keys {
                tags.remove(key);
            }
        }
    }

    pub fn list_tags_for_resource(&self, arn: &str) -> HashMap<String, String> {
        self.resource_tags.get(arn).cloned().unwrap_or_default()
    }
}
