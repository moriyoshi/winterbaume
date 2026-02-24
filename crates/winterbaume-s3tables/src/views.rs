use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::S3TablesService;
use crate::state::S3TablesState;
use crate::types::{EncryptionConfig, Namespace, StorageClassConfig, Table, TableBucket};

/// Serializable view of the entire S3 Tables state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct S3TablesStateView {
    #[serde(default)]
    pub table_buckets: HashMap<String, TableBucketView>,
    #[serde(default)]
    pub namespaces: Vec<NamespaceView>,
    #[serde(default)]
    pub tables: Vec<TableView>,
    #[serde(default)]
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TableBucketView {
    pub name: String,
    pub arn: String,
    pub owner_account_id: String,
    pub created_at: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption_sse_algorithm: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encryption_kms_key_arn: Option<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub maintenance_config: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics_config: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replication_config: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamespaceView {
    pub table_bucket_arn: String,
    pub namespace: Vec<String>,
    pub name: String,
    pub owner_account_id: String,
    pub created_at: String,
    pub created_by: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableView {
    pub table_bucket_arn: String,
    pub namespace: String,
    pub name: String,
    pub arn: String,
    pub version_token: String,
    pub format: String,
    pub warehouse_location: String,
    pub owner_account_id: String,
    pub created_at: String,
    pub created_by: String,
    pub modified_at: String,
    pub modified_by: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata_location: Option<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub maintenance_config: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replication_config: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_expiration_config: Option<String>,
}

impl From<&S3TablesState> for S3TablesStateView {
    fn from(state: &S3TablesState) -> Self {
        S3TablesStateView {
            table_buckets: state
                .table_buckets
                .iter()
                .map(|(k, v)| (k.clone(), TableBucketView::from(v)))
                .collect(),
            namespaces: state.namespaces.values().map(NamespaceView::from).collect(),
            tables: state.tables.values().map(TableView::from).collect(),
            tags: state.tags.clone(),
        }
    }
}

impl From<&TableBucket> for TableBucketView {
    fn from(b: &TableBucket) -> Self {
        TableBucketView {
            name: b.name.clone(),
            arn: b.arn.clone(),
            owner_account_id: b.owner_account_id.clone(),
            created_at: b.created_at.to_rfc3339(),
            tags: b.tags.clone(),
            encryption_sse_algorithm: b.encryption.as_ref().map(|e| e.sse_algorithm.clone()),
            encryption_kms_key_arn: b.encryption.as_ref().and_then(|e| e.kms_key_arn.clone()),
            maintenance_config: b.maintenance_config.clone(),
            metrics_config: b.metrics_config.clone(),
            policy: b.policy.clone(),
            storage_class: b.storage_class.as_ref().map(|s| s.storage_class.clone()),
            replication_config: b.replication_config.clone(),
        }
    }
}

impl From<&Namespace> for NamespaceView {
    fn from(n: &Namespace) -> Self {
        NamespaceView {
            table_bucket_arn: n.table_bucket_arn.clone(),
            namespace: n.namespace.clone(),
            name: n.name.clone(),
            owner_account_id: n.owner_account_id.clone(),
            created_at: n.created_at.to_rfc3339(),
            created_by: n.created_by.clone(),
            tags: n.tags.clone(),
        }
    }
}

impl From<&Table> for TableView {
    fn from(t: &Table) -> Self {
        TableView {
            table_bucket_arn: t.table_bucket_arn.clone(),
            namespace: t.namespace.clone(),
            name: t.name.clone(),
            arn: t.arn.clone(),
            version_token: t.version_token.clone(),
            format: t.format.clone(),
            warehouse_location: t.warehouse_location.clone(),
            owner_account_id: t.owner_account_id.clone(),
            created_at: t.created_at.to_rfc3339(),
            created_by: t.created_by.clone(),
            modified_at: t.modified_at.to_rfc3339(),
            modified_by: t.modified_by.clone(),
            tags: t.tags.clone(),
            policy: t.policy.clone(),
            metadata_location: t.metadata_location.clone(),
            maintenance_config: t.maintenance_config.clone(),
            storage_class: t.storage_class.as_ref().map(|s| s.storage_class.clone()),
            replication_config: t.replication_config.clone(),
            record_expiration_config: t.record_expiration_config.clone(),
        }
    }
}

impl From<S3TablesStateView> for S3TablesState {
    fn from(view: S3TablesStateView) -> Self {
        use chrono::{DateTime, Utc};
        let parse_dt = |s: &str| -> DateTime<Utc> {
            DateTime::parse_from_rfc3339(s)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now())
        };

        let table_buckets = view
            .table_buckets
            .into_iter()
            .map(|(k, v)| {
                let encryption = v.encryption_sse_algorithm.map(|algo| EncryptionConfig {
                    sse_algorithm: algo,
                    kms_key_arn: v.encryption_kms_key_arn,
                });
                let storage_class = v
                    .storage_class
                    .map(|sc| StorageClassConfig { storage_class: sc });
                (
                    k,
                    TableBucket {
                        name: v.name,
                        arn: v.arn,
                        owner_account_id: v.owner_account_id,
                        created_at: parse_dt(&v.created_at),
                        tags: v.tags,
                        encryption,
                        maintenance_config: v.maintenance_config,
                        metrics_config: v.metrics_config,
                        policy: v.policy,
                        storage_class,
                        replication_config: v.replication_config,
                    },
                )
            })
            .collect();

        let namespaces = view
            .namespaces
            .into_iter()
            .map(|v| {
                let key = (v.table_bucket_arn.clone(), v.name.clone());
                (
                    key,
                    Namespace {
                        table_bucket_arn: v.table_bucket_arn,
                        namespace: v.namespace,
                        name: v.name,
                        owner_account_id: v.owner_account_id,
                        created_at: parse_dt(&v.created_at),
                        created_by: v.created_by,
                        tags: v.tags,
                    },
                )
            })
            .collect();

        let tables = view
            .tables
            .into_iter()
            .map(|v| {
                let key = (
                    v.table_bucket_arn.clone(),
                    v.namespace.clone(),
                    v.name.clone(),
                );
                let storage_class = v
                    .storage_class
                    .map(|sc| StorageClassConfig { storage_class: sc });
                (
                    key,
                    Table {
                        table_bucket_arn: v.table_bucket_arn,
                        namespace: v.namespace,
                        name: v.name,
                        arn: v.arn,
                        version_token: v.version_token,
                        format: v.format,
                        warehouse_location: v.warehouse_location,
                        owner_account_id: v.owner_account_id,
                        created_at: parse_dt(&v.created_at),
                        created_by: v.created_by,
                        modified_at: parse_dt(&v.modified_at),
                        modified_by: v.modified_by,
                        tags: v.tags,
                        policy: v.policy,
                        metadata_location: v.metadata_location,
                        maintenance_config: v.maintenance_config,
                        storage_class,
                        replication_config: v.replication_config,
                        record_expiration_config: v.record_expiration_config,
                    },
                )
            })
            .collect();

        S3TablesState {
            table_buckets,
            namespaces,
            tables,
            tags: view.tags,
        }
    }
}

impl StatefulService for S3TablesService {
    type StateView = S3TablesStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        S3TablesStateView::from(&*guard)
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
            *guard = S3TablesState::from(view);
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
        use chrono::{DateTime, Utc};
        let parse_dt = |s: &str| -> DateTime<Utc> {
            DateTime::parse_from_rfc3339(s)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now())
        };

        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (k, v) in view.table_buckets {
                let encryption = v.encryption_sse_algorithm.map(|algo| EncryptionConfig {
                    sse_algorithm: algo,
                    kms_key_arn: v.encryption_kms_key_arn,
                });
                let storage_class = v
                    .storage_class
                    .map(|sc| StorageClassConfig { storage_class: sc });
                guard.table_buckets.entry(k).or_insert_with(|| TableBucket {
                    name: v.name,
                    arn: v.arn,
                    owner_account_id: v.owner_account_id,
                    created_at: parse_dt(&v.created_at),
                    tags: v.tags,
                    encryption,
                    maintenance_config: v.maintenance_config,
                    metrics_config: v.metrics_config,
                    policy: v.policy,
                    storage_class,
                    replication_config: v.replication_config,
                });
            }
            for v in view.namespaces {
                let key = (v.table_bucket_arn.clone(), v.name.clone());
                guard.namespaces.entry(key).or_insert_with(|| Namespace {
                    table_bucket_arn: v.table_bucket_arn,
                    namespace: v.namespace,
                    name: v.name,
                    owner_account_id: v.owner_account_id,
                    created_at: parse_dt(&v.created_at),
                    created_by: v.created_by,
                    tags: v.tags,
                });
            }
            for v in view.tables {
                let key = (
                    v.table_bucket_arn.clone(),
                    v.namespace.clone(),
                    v.name.clone(),
                );
                let storage_class = v
                    .storage_class
                    .map(|sc| StorageClassConfig { storage_class: sc });
                guard.tables.entry(key).or_insert_with(|| Table {
                    table_bucket_arn: v.table_bucket_arn,
                    namespace: v.namespace,
                    name: v.name,
                    arn: v.arn,
                    version_token: v.version_token,
                    format: v.format,
                    warehouse_location: v.warehouse_location,
                    owner_account_id: v.owner_account_id,
                    created_at: parse_dt(&v.created_at),
                    created_by: v.created_by,
                    modified_at: parse_dt(&v.modified_at),
                    modified_by: v.modified_by,
                    tags: v.tags,
                    policy: v.policy,
                    metadata_location: v.metadata_location,
                    maintenance_config: v.maintenance_config,
                    storage_class,
                    replication_config: v.replication_config,
                    record_expiration_config: v.record_expiration_config,
                });
            }
            for (arn, new_tags) in view.tags {
                guard.tags.entry(arn).or_default().extend(new_tags);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
