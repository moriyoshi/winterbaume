//! Serde-compatible view types for Timestream InfluxDB state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::TimestreamInfluxDbService;
use crate::state::TimestreamInfluxDbState;
use crate::types::{DbCluster, DbInstance, DbParameterGroup};

/// Serializable view of the entire Timestream InfluxDB state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TimestreamInfluxDbStateView {
    #[serde(default)]
    pub db_instances: HashMap<String, DbInstanceView>,
    #[serde(default)]
    pub db_clusters: HashMap<String, DbClusterView>,
    #[serde(default)]
    pub db_parameter_groups: HashMap<String, DbParameterGroupView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbInstanceView {
    pub id: String,
    pub name: String,
    pub arn: String,
    pub status: String,
    pub endpoint: Option<String>,
    pub port: Option<i32>,
    pub db_instance_type: String,
    pub db_storage_type: Option<String>,
    pub allocated_storage: i32,
    pub deployment_type: Option<String>,
    #[serde(default)]
    pub vpc_subnet_ids: Vec<String>,
    #[serde(default)]
    pub vpc_security_group_ids: Vec<String>,
    pub publicly_accessible: Option<bool>,
    pub db_parameter_group_identifier: Option<String>,
    pub availability_zone: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbClusterView {
    pub id: String,
    pub name: String,
    pub arn: String,
    pub status: String,
    pub endpoint: Option<String>,
    pub reader_endpoint: Option<String>,
    pub port: Option<i32>,
    pub deployment_type: Option<String>,
    pub db_instance_type: Option<String>,
    pub network_type: Option<String>,
    pub db_storage_type: Option<String>,
    pub allocated_storage: Option<i32>,
    pub publicly_accessible: Option<bool>,
    pub db_parameter_group_identifier: Option<String>,
    #[serde(default)]
    pub vpc_subnet_ids: Vec<String>,
    #[serde(default)]
    pub vpc_security_group_ids: Vec<String>,
    pub failover_mode: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbParameterGroupView {
    pub id: String,
    pub name: String,
    pub arn: String,
    pub description: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

// --- From internal types to view types ---

impl From<&TimestreamInfluxDbState> for TimestreamInfluxDbStateView {
    fn from(state: &TimestreamInfluxDbState) -> Self {
        TimestreamInfluxDbStateView {
            db_instances: state
                .db_instances
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        DbInstanceView {
                            id: v.id.clone(),
                            name: v.name.clone(),
                            arn: v.arn.clone(),
                            status: v.status.clone(),
                            endpoint: v.endpoint.clone(),
                            port: v.port,
                            db_instance_type: v.db_instance_type.clone(),
                            db_storage_type: v.db_storage_type.clone(),
                            allocated_storage: v.allocated_storage,
                            deployment_type: v.deployment_type.clone(),
                            vpc_subnet_ids: v.vpc_subnet_ids.clone(),
                            vpc_security_group_ids: v.vpc_security_group_ids.clone(),
                            publicly_accessible: v.publicly_accessible,
                            db_parameter_group_identifier: v.db_parameter_group_identifier.clone(),
                            availability_zone: v.availability_zone.clone(),
                            tags: v.tags.clone(),
                        },
                    )
                })
                .collect(),
            db_clusters: state
                .db_clusters
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        DbClusterView {
                            id: v.id.clone(),
                            name: v.name.clone(),
                            arn: v.arn.clone(),
                            status: v.status.clone(),
                            endpoint: v.endpoint.clone(),
                            reader_endpoint: v.reader_endpoint.clone(),
                            port: v.port,
                            deployment_type: v.deployment_type.clone(),
                            db_instance_type: v.db_instance_type.clone(),
                            network_type: v.network_type.clone(),
                            db_storage_type: v.db_storage_type.clone(),
                            allocated_storage: v.allocated_storage,
                            publicly_accessible: v.publicly_accessible,
                            db_parameter_group_identifier: v.db_parameter_group_identifier.clone(),
                            vpc_subnet_ids: v.vpc_subnet_ids.clone(),
                            vpc_security_group_ids: v.vpc_security_group_ids.clone(),
                            failover_mode: v.failover_mode.clone(),
                            tags: v.tags.clone(),
                        },
                    )
                })
                .collect(),
            db_parameter_groups: state
                .db_parameter_groups
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        DbParameterGroupView {
                            id: v.id.clone(),
                            name: v.name.clone(),
                            arn: v.arn.clone(),
                            description: v.description.clone(),
                            tags: v.tags.clone(),
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- From view types to internal types ---

impl From<TimestreamInfluxDbStateView> for TimestreamInfluxDbState {
    fn from(view: TimestreamInfluxDbStateView) -> Self {
        TimestreamInfluxDbState {
            db_instances: view
                .db_instances
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        DbInstance {
                            id: v.id,
                            name: v.name,
                            arn: v.arn,
                            status: v.status,
                            endpoint: v.endpoint,
                            port: v.port,
                            db_instance_type: v.db_instance_type,
                            db_storage_type: v.db_storage_type,
                            allocated_storage: v.allocated_storage,
                            deployment_type: v.deployment_type,
                            vpc_subnet_ids: v.vpc_subnet_ids,
                            vpc_security_group_ids: v.vpc_security_group_ids,
                            publicly_accessible: v.publicly_accessible,
                            db_parameter_group_identifier: v.db_parameter_group_identifier,
                            availability_zone: v.availability_zone,
                            tags: v.tags,
                        },
                    )
                })
                .collect(),
            db_clusters: view
                .db_clusters
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        DbCluster {
                            id: v.id,
                            name: v.name,
                            arn: v.arn,
                            status: v.status,
                            endpoint: v.endpoint,
                            reader_endpoint: v.reader_endpoint,
                            port: v.port,
                            deployment_type: v.deployment_type,
                            db_instance_type: v.db_instance_type,
                            network_type: v.network_type,
                            db_storage_type: v.db_storage_type,
                            allocated_storage: v.allocated_storage,
                            publicly_accessible: v.publicly_accessible,
                            db_parameter_group_identifier: v.db_parameter_group_identifier,
                            vpc_subnet_ids: v.vpc_subnet_ids,
                            vpc_security_group_ids: v.vpc_security_group_ids,
                            failover_mode: v.failover_mode,
                            tags: v.tags,
                        },
                    )
                })
                .collect(),
            db_parameter_groups: view
                .db_parameter_groups
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        DbParameterGroup {
                            id: v.id,
                            name: v.name,
                            arn: v.arn,
                            description: v.description,
                            tags: v.tags,
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for TimestreamInfluxDbService {
    type StateView = TimestreamInfluxDbStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        TimestreamInfluxDbStateView::from(&*guard)
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
            *guard = TimestreamInfluxDbState::from(view);
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
            let new_state = TimestreamInfluxDbState::from(view);
            guard.db_instances.extend(new_state.db_instances);
            guard.db_clusters.extend(new_state.db_clusters);
            guard
                .db_parameter_groups
                .extend(new_state.db_parameter_groups);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
