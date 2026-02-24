//! Serde-compatible view types for Athena state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::AthenaService;
use crate::state::AthenaState;
use crate::types::{
    CapacityReservationData, DataCatalogData, NamedQueryData, PreparedStatementData, WorkGroup,
};

/// Serializable view of the entire Athena state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AthenaStateView {
    #[serde(default)]
    pub work_groups: HashMap<String, WorkGroupView>,
    #[serde(default)]
    pub capacity_reservations: HashMap<String, CapacityReservationView>,
    #[serde(default)]
    pub data_catalogs: HashMap<String, DataCatalogView>,
    #[serde(default)]
    pub named_queries: HashMap<String, NamedQueryView>,
    /// Key: "{work_group}/{statement_name}"
    #[serde(default)]
    pub prepared_statements: HashMap<String, PreparedStatementView>,
    /// Resource ARN -> tags
    #[serde(default)]
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkGroupView {
    pub name: String,
    pub state: String,
    pub description: String,
    pub creation_time: Option<String>,
    pub output_location: String,
    pub enforce_work_group_configuration: bool,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityReservationView {
    pub name: String,
    pub target_dpus: i32,
    pub allocated_dpus: i32,
    pub status: String,
    pub creation_time: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataCatalogView {
    pub name: String,
    pub catalog_type: String,
    pub description: String,
    #[serde(default)]
    pub parameters: HashMap<String, String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedQueryView {
    pub id: String,
    pub name: String,
    pub description: String,
    pub database: String,
    pub query_string: String,
    pub work_group: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreparedStatementView {
    pub statement_name: String,
    pub work_group_name: String,
    pub query_statement: String,
    pub description: String,
    pub last_modified_time: Option<String>,
}

// ---------------------------------------------------------------------------
// From conversions
// ---------------------------------------------------------------------------

impl From<&WorkGroup> for WorkGroupView {
    fn from(wg: &WorkGroup) -> Self {
        WorkGroupView {
            name: wg.name.clone(),
            state: wg.state.clone(),
            description: wg.description.clone(),
            creation_time: Some(wg.creation_time.to_rfc3339()),
            output_location: wg.output_location.clone(),
            enforce_work_group_configuration: wg.enforce_work_group_configuration,
            tags: wg.tags.clone(),
        }
    }
}

impl From<&CapacityReservationData> for CapacityReservationView {
    fn from(cr: &CapacityReservationData) -> Self {
        CapacityReservationView {
            name: cr.name.clone(),
            target_dpus: cr.target_dpus,
            allocated_dpus: cr.allocated_dpus,
            status: cr.status.clone(),
            creation_time: Some(cr.creation_time.to_rfc3339()),
            tags: cr.tags.clone(),
        }
    }
}

impl From<&DataCatalogData> for DataCatalogView {
    fn from(dc: &DataCatalogData) -> Self {
        DataCatalogView {
            name: dc.name.clone(),
            catalog_type: dc.catalog_type.clone(),
            description: dc.description.clone(),
            parameters: dc.parameters.clone(),
            tags: dc.tags.clone(),
        }
    }
}

impl From<&NamedQueryData> for NamedQueryView {
    fn from(nq: &NamedQueryData) -> Self {
        NamedQueryView {
            id: nq.id.clone(),
            name: nq.name.clone(),
            description: nq.description.clone(),
            database: nq.database.clone(),
            query_string: nq.query_string.clone(),
            work_group: nq.work_group.clone(),
        }
    }
}

impl From<&PreparedStatementData> for PreparedStatementView {
    fn from(ps: &PreparedStatementData) -> Self {
        PreparedStatementView {
            statement_name: ps.statement_name.clone(),
            work_group_name: ps.work_group_name.clone(),
            query_statement: ps.query_statement.clone(),
            description: ps.description.clone(),
            last_modified_time: Some(ps.last_modified_time.to_rfc3339()),
        }
    }
}

// ---------------------------------------------------------------------------
// StatefulService implementation
// ---------------------------------------------------------------------------

impl StatefulService for AthenaService {
    type StateView = AthenaStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;

        let work_groups = guard
            .work_groups
            .values()
            .map(|wg| (wg.name.clone(), WorkGroupView::from(wg)))
            .collect();

        let capacity_reservations = guard
            .capacity_reservations
            .values()
            .map(|cr| (cr.name.clone(), CapacityReservationView::from(cr)))
            .collect();

        let data_catalogs = guard
            .data_catalogs
            .values()
            .map(|dc| (dc.name.clone(), DataCatalogView::from(dc)))
            .collect();

        let named_queries = guard
            .named_queries
            .values()
            .map(|nq| (nq.id.clone(), NamedQueryView::from(nq)))
            .collect();

        let prepared_statements = guard
            .prepared_statements
            .iter()
            .map(|((wg, stmt), ps)| (format!("{wg}/{stmt}"), PreparedStatementView::from(ps)))
            .collect();

        AthenaStateView {
            work_groups,
            capacity_reservations,
            data_catalogs,
            named_queries,
            prepared_statements,
            tags: guard.tags.clone(),
        }
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let mut new_state = AthenaState::default();

        for (_, wgv) in view.work_groups {
            let creation_time = wgv
                .creation_time
                .as_deref()
                .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(Utc::now);
            new_state.work_groups.insert(
                wgv.name.clone(),
                WorkGroup {
                    name: wgv.name,
                    state: wgv.state,
                    description: wgv.description,
                    creation_time,
                    output_location: wgv.output_location,
                    enforce_work_group_configuration: wgv.enforce_work_group_configuration,
                    tags: wgv.tags,
                },
            );
        }

        for (_, crv) in view.capacity_reservations {
            let creation_time = crv
                .creation_time
                .as_deref()
                .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(Utc::now);
            new_state.capacity_reservations.insert(
                crv.name.clone(),
                CapacityReservationData {
                    name: crv.name,
                    target_dpus: crv.target_dpus,
                    allocated_dpus: crv.allocated_dpus,
                    status: crv.status,
                    creation_time,
                    tags: crv.tags,
                },
            );
        }

        for (_, dcv) in view.data_catalogs {
            new_state.data_catalogs.insert(
                dcv.name.clone(),
                DataCatalogData {
                    name: dcv.name,
                    catalog_type: dcv.catalog_type,
                    description: dcv.description,
                    parameters: dcv.parameters,
                    tags: dcv.tags,
                },
            );
        }

        for (_, nqv) in view.named_queries {
            new_state.named_queries.insert(
                nqv.id.clone(),
                NamedQueryData {
                    id: nqv.id,
                    name: nqv.name,
                    description: nqv.description,
                    database: nqv.database,
                    query_string: nqv.query_string,
                    work_group: nqv.work_group,
                },
            );
        }

        for (key, psv) in view.prepared_statements {
            let mut parts = key.splitn(2, '/');
            let wg = parts.next().unwrap_or("").to_string();
            let stmt = parts.next().unwrap_or("").to_string();
            let last_modified_time = psv
                .last_modified_time
                .as_deref()
                .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(Utc::now);
            new_state.prepared_statements.insert(
                (wg, stmt),
                PreparedStatementData {
                    statement_name: psv.statement_name,
                    work_group_name: psv.work_group_name,
                    query_statement: psv.query_statement,
                    description: psv.description,
                    last_modified_time,
                },
            );
        }

        new_state.tags = view.tags;

        {
            let state = self.state.get(account_id, region);
            *state.write().await = new_state;
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

            for (_, wgv) in view.work_groups {
                let creation_time = wgv
                    .creation_time
                    .as_deref()
                    .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(Utc::now);
                guard.work_groups.insert(
                    wgv.name.clone(),
                    WorkGroup {
                        name: wgv.name,
                        state: wgv.state,
                        description: wgv.description,
                        creation_time,
                        output_location: wgv.output_location,
                        enforce_work_group_configuration: wgv.enforce_work_group_configuration,
                        tags: wgv.tags,
                    },
                );
            }

            for (_, crv) in view.capacity_reservations {
                let creation_time = crv
                    .creation_time
                    .as_deref()
                    .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(Utc::now);
                guard.capacity_reservations.insert(
                    crv.name.clone(),
                    CapacityReservationData {
                        name: crv.name,
                        target_dpus: crv.target_dpus,
                        allocated_dpus: crv.allocated_dpus,
                        status: crv.status,
                        creation_time,
                        tags: crv.tags,
                    },
                );
            }

            for (_, dcv) in view.data_catalogs {
                guard.data_catalogs.insert(
                    dcv.name.clone(),
                    DataCatalogData {
                        name: dcv.name,
                        catalog_type: dcv.catalog_type,
                        description: dcv.description,
                        parameters: dcv.parameters,
                        tags: dcv.tags,
                    },
                );
            }

            for (_, nqv) in view.named_queries {
                guard.named_queries.insert(
                    nqv.id.clone(),
                    NamedQueryData {
                        id: nqv.id,
                        name: nqv.name,
                        description: nqv.description,
                        database: nqv.database,
                        query_string: nqv.query_string,
                        work_group: nqv.work_group,
                    },
                );
            }

            for (key, psv) in view.prepared_statements {
                let mut parts = key.splitn(2, '/');
                let wg = parts.next().unwrap_or("").to_string();
                let stmt = parts.next().unwrap_or("").to_string();
                let last_modified_time = psv
                    .last_modified_time
                    .as_deref()
                    .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(Utc::now);
                guard.prepared_statements.insert(
                    (wg, stmt),
                    PreparedStatementData {
                        statement_name: psv.statement_name,
                        work_group_name: psv.work_group_name,
                        query_statement: psv.query_statement,
                        description: psv.description,
                        last_modified_time,
                    },
                );
            }

            for (arn, tags) in view.tags {
                guard.tags.insert(arn, tags);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
