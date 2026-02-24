//! Serde-compatible view types for DynamoDB state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StatefulService};

use crate::handlers::DynamoDbService;
use crate::state::{DynamoDbState, TableState};
use crate::types::{
    AttributeDefinition, ContinuousBackupsConfig, ContributorInsightsConfig, DynamoDbTag,
    GlobalSecondaryIndexDef, GlobalTableInfo, Item, KeySchemaElement, KinesisStreamingDestination,
    LocalSecondaryIndexDef, ProvisionedThroughput, ResourcePolicy, Table, TtlConfig,
};

/// Serializable view of the entire DynamoDB state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DynamodbStateView {
    /// Tables keyed by table name.
    #[serde(default)]
    pub tables: HashMap<String, TableStateView>,
    /// Tags keyed by resource ARN.
    #[serde(default)]
    pub tags: HashMap<String, Vec<DynamoDbTagView>>,
    /// TTL configs keyed by table name.
    #[serde(default)]
    pub ttl_configs: HashMap<String, TtlConfigView>,
    /// Continuous backups (PITR) configs keyed by table name.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub continuous_backups: HashMap<String, ContinuousBackupsConfigView>,
    /// Resource policies keyed by resource ARN.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub resource_policies: HashMap<String, ResourcePolicyView>,
    /// Global tables (v1 API) keyed by global table name.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub global_tables: HashMap<String, GlobalTableInfoView>,
    /// Kinesis streaming destinations keyed by table name -> stream ARN.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub kinesis_destinations: HashMap<String, HashMap<String, KinesisStreamingDestinationView>>,
    /// Contributor Insights keyed by "table_name" or "table_name/index_name".
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub contributor_insights: HashMap<String, ContributorInsightsConfigView>,
}

/// Serializable view of a single DynamoDB table and its items.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableStateView {
    pub name: String,
    pub arn: String,
    pub key_schema: Vec<KeySchemaElementView>,
    pub attribute_definitions: Vec<AttributeDefinitionView>,
    pub billing_mode: String,
    pub provisioned_throughput: Option<ProvisionedThroughputView>,
    pub creation_date_time: String,
    pub table_status: String,
    pub hash_key_attr: String,
    pub hash_key_type: String,
    pub range_key_attr: Option<String>,
    pub range_key_type: Option<String>,
    /// Items stored as hash_key_value -> range_key_value -> Item (JSON map).
    #[serde(default)]
    pub items: HashMap<String, HashMap<String, Item>>,
    #[serde(default)]
    pub stream_enabled: bool,
    #[serde(default)]
    pub stream_view_type: Option<String>,
    #[serde(default)]
    pub latest_stream_arn: Option<String>,
    #[serde(default)]
    pub latest_stream_label: Option<String>,
    /// Global secondary index definitions.
    #[serde(default)]
    pub global_secondary_index: Vec<SecondaryIndexView>,
    /// Local secondary index definitions.
    #[serde(default)]
    pub local_secondary_index: Vec<SecondaryIndexView>,
    /// `replica` nested blocks.
    #[serde(default)]
    pub replica: Vec<serde_json::Value>,
    /// `import_table` nested block.
    #[serde(default)]
    pub import_table: Option<serde_json::Value>,
    /// `on_demand_throughput` nested block.
    #[serde(default)]
    pub on_demand_throughput: Option<serde_json::Value>,
}

/// Serializable view of a secondary index (GSI or LSI) definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondaryIndexView {
    pub index_name: String,
    pub key_schema: Vec<KeySchemaElementView>,
    #[serde(default = "default_projection_type")]
    pub projection_type: String,
    #[serde(default)]
    pub non_key_attributes: Vec<String>,
}

fn default_projection_type() -> String {
    "ALL".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeySchemaElementView {
    pub attribute_name: String,
    pub key_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeDefinitionView {
    pub attribute_name: String,
    pub attribute_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvisionedThroughputView {
    pub read_capacity_units: i64,
    pub write_capacity_units: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamoDbTagView {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtlConfigView {
    pub attribute_name: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinuousBackupsConfigView {
    pub point_in_time_recovery_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcePolicyView {
    pub policy: String,
    pub revision_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalTableInfoView {
    pub global_table_name: String,
    pub global_table_arn: String,
    pub global_table_status: String,
    pub creation_date_time: String,
    #[serde(default)]
    pub replication_group: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KinesisStreamingDestinationView {
    pub stream_arn: String,
    pub destination_status: String,
    #[serde(default)]
    pub approximate_creation_date_time_precision: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContributorInsightsConfigView {
    pub table_name: String,
    #[serde(default)]
    pub index_name: Option<String>,
    pub status: String,
    pub last_update_date_time: String,
}

// --- From internal types to view types ---

impl From<&DynamoDbState> for DynamodbStateView {
    fn from(state: &DynamoDbState) -> Self {
        DynamodbStateView {
            tables: state
                .tables
                .iter()
                .map(|(k, v)| (k.clone(), TableStateView::from(v)))
                .collect(),
            tags: state
                .tags
                .iter()
                .map(|(k, v)| (k.clone(), v.iter().map(DynamoDbTagView::from).collect()))
                .collect(),
            ttl_configs: state
                .ttl_configs
                .iter()
                .map(|(k, v)| (k.clone(), TtlConfigView::from(v)))
                .collect(),
            continuous_backups: state
                .continuous_backups
                .iter()
                .map(|(k, v)| (k.clone(), ContinuousBackupsConfigView::from(v)))
                .collect(),
            resource_policies: state
                .resource_policies
                .iter()
                .map(|(k, v)| (k.clone(), ResourcePolicyView::from(v)))
                .collect(),
            global_tables: state
                .global_tables
                .iter()
                .map(|(k, v)| (k.clone(), GlobalTableInfoView::from(v)))
                .collect(),
            kinesis_destinations: state
                .kinesis_destinations
                .iter()
                .map(|(table, dests)| {
                    (
                        table.clone(),
                        dests
                            .iter()
                            .map(|(arn, d)| (arn.clone(), KinesisStreamingDestinationView::from(d)))
                            .collect(),
                    )
                })
                .collect(),
            contributor_insights: state
                .contributor_insights
                .iter()
                .map(|(k, v)| (k.clone(), ContributorInsightsConfigView::from(v)))
                .collect(),
        }
    }
}

impl From<&TableState> for TableStateView {
    fn from(ts: &TableState) -> Self {
        TableStateView {
            name: ts.table.name.clone(),
            arn: ts.table.arn.clone(),
            key_schema: ts
                .table
                .key_schema
                .iter()
                .map(KeySchemaElementView::from)
                .collect(),
            attribute_definitions: ts
                .table
                .attribute_definitions
                .iter()
                .map(AttributeDefinitionView::from)
                .collect(),
            billing_mode: ts.table.billing_mode.clone(),
            provisioned_throughput: ts
                .table
                .provisioned_throughput
                .as_ref()
                .map(ProvisionedThroughputView::from),
            creation_date_time: ts.table.creation_date_time.to_rfc3339(),
            table_status: ts.table.table_status.clone(),
            hash_key_attr: ts.table.hash_key_attr.clone(),
            hash_key_type: ts.table.hash_key_type.clone(),
            range_key_attr: ts.table.range_key_attr.clone(),
            range_key_type: ts.table.range_key_type.clone(),
            items: ts.items.clone(),
            stream_enabled: ts.table.stream_enabled,
            stream_view_type: ts.table.stream_view_type.clone(),
            latest_stream_arn: ts.table.latest_stream_arn.clone(),
            latest_stream_label: ts.table.latest_stream_label.clone(),
            global_secondary_index: ts
                .table
                .global_secondary_indexes
                .iter()
                .map(SecondaryIndexView::from_gsi)
                .collect(),
            local_secondary_index: ts
                .table
                .local_secondary_indexes
                .iter()
                .map(SecondaryIndexView::from_lsi)
                .collect(),
            replica: vec![],
            import_table: None,
            on_demand_throughput: None,
        }
    }
}

impl From<&KeySchemaElement> for KeySchemaElementView {
    fn from(k: &KeySchemaElement) -> Self {
        KeySchemaElementView {
            attribute_name: k.attribute_name.clone(),
            key_type: k.key_type.clone(),
        }
    }
}

impl From<&AttributeDefinition> for AttributeDefinitionView {
    fn from(a: &AttributeDefinition) -> Self {
        AttributeDefinitionView {
            attribute_name: a.attribute_name.clone(),
            attribute_type: a.attribute_type.clone(),
        }
    }
}

impl From<&ProvisionedThroughput> for ProvisionedThroughputView {
    fn from(pt: &ProvisionedThroughput) -> Self {
        ProvisionedThroughputView {
            read_capacity_units: pt.read_capacity_units,
            write_capacity_units: pt.write_capacity_units,
        }
    }
}

impl From<&DynamoDbTag> for DynamoDbTagView {
    fn from(t: &DynamoDbTag) -> Self {
        DynamoDbTagView {
            key: t.key.clone(),
            value: t.value.clone(),
        }
    }
}

impl From<&TtlConfig> for TtlConfigView {
    fn from(c: &TtlConfig) -> Self {
        TtlConfigView {
            attribute_name: c.attribute_name.clone(),
            enabled: c.enabled,
        }
    }
}

impl From<&ContinuousBackupsConfig> for ContinuousBackupsConfigView {
    fn from(c: &ContinuousBackupsConfig) -> Self {
        ContinuousBackupsConfigView {
            point_in_time_recovery_enabled: c.point_in_time_recovery_enabled,
        }
    }
}

impl From<&ResourcePolicy> for ResourcePolicyView {
    fn from(p: &ResourcePolicy) -> Self {
        ResourcePolicyView {
            policy: p.policy.clone(),
            revision_id: p.revision_id.clone(),
        }
    }
}

impl From<&GlobalTableInfo> for GlobalTableInfoView {
    fn from(g: &GlobalTableInfo) -> Self {
        GlobalTableInfoView {
            global_table_name: g.global_table_name.clone(),
            global_table_arn: g.global_table_arn.clone(),
            global_table_status: g.global_table_status.clone(),
            creation_date_time: g.creation_date_time.to_rfc3339(),
            replication_group: g.replication_group.clone(),
        }
    }
}

impl From<&KinesisStreamingDestination> for KinesisStreamingDestinationView {
    fn from(d: &KinesisStreamingDestination) -> Self {
        KinesisStreamingDestinationView {
            stream_arn: d.stream_arn.clone(),
            destination_status: d.destination_status.clone(),
            approximate_creation_date_time_precision: d
                .approximate_creation_date_time_precision
                .clone(),
        }
    }
}

impl From<&ContributorInsightsConfig> for ContributorInsightsConfigView {
    fn from(c: &ContributorInsightsConfig) -> Self {
        ContributorInsightsConfigView {
            table_name: c.table_name.clone(),
            index_name: c.index_name.clone(),
            status: c.status.clone(),
            last_update_date_time: c.last_update_date_time.to_rfc3339(),
        }
    }
}

// --- From view types to internal types ---

impl From<DynamodbStateView> for DynamoDbState {
    fn from(view: DynamodbStateView) -> Self {
        DynamoDbState {
            tables: view
                .tables
                .into_iter()
                .map(|(k, v)| (k, TableState::from(v)))
                .collect(),
            tags: view
                .tags
                .into_iter()
                .map(|(k, v)| (k, v.into_iter().map(DynamoDbTag::from).collect()))
                .collect(),
            ttl_configs: view
                .ttl_configs
                .into_iter()
                .map(|(k, v)| (k, TtlConfig::from(v)))
                .collect(),
            backups: HashMap::new(),
            continuous_backups: view
                .continuous_backups
                .into_iter()
                .map(|(k, v)| (k, ContinuousBackupsConfig::from(v)))
                .collect(),
            resource_policies: view
                .resource_policies
                .into_iter()
                .map(|(k, v)| (k, ResourcePolicy::from(v)))
                .collect(),
            backup_counter: 0,
            global_tables: view
                .global_tables
                .into_iter()
                .map(|(k, v)| (k, GlobalTableInfo::from(v)))
                .collect(),
            kinesis_destinations: view
                .kinesis_destinations
                .into_iter()
                .map(|(table, dests)| {
                    (
                        table,
                        dests
                            .into_iter()
                            .map(|(arn, d)| (arn, KinesisStreamingDestination::from(d)))
                            .collect(),
                    )
                })
                .collect(),
            contributor_insights: view
                .contributor_insights
                .into_iter()
                .map(|(k, v)| (k, ContributorInsightsConfig::from(v)))
                .collect(),
            exports: HashMap::new(),
            export_counter: 0,
            imports: HashMap::new(),
            import_counter: 0,
        }
    }
}

impl From<TableStateView> for TableState {
    fn from(view: TableStateView) -> Self {
        let creation_date_time = view
            .creation_date_time
            .parse::<DateTime<Utc>>()
            .unwrap_or_else(|_| Utc::now());
        let item_count = view.items.values().map(|m| m.len()).sum();
        TableState {
            table: Table {
                name: view.name,
                arn: view.arn,
                key_schema: view
                    .key_schema
                    .into_iter()
                    .map(KeySchemaElement::from)
                    .collect(),
                attribute_definitions: view
                    .attribute_definitions
                    .into_iter()
                    .map(AttributeDefinition::from)
                    .collect(),
                billing_mode: view.billing_mode,
                provisioned_throughput: view
                    .provisioned_throughput
                    .map(ProvisionedThroughput::from),
                creation_date_time,
                table_status: view.table_status,
                item_count,
                hash_key_attr: view.hash_key_attr,
                hash_key_type: view.hash_key_type,
                range_key_attr: view.range_key_attr,
                range_key_type: view.range_key_type,
                stream_enabled: view.stream_enabled,
                stream_view_type: view.stream_view_type,
                latest_stream_arn: view.latest_stream_arn,
                latest_stream_label: view.latest_stream_label,
                global_secondary_indexes: view
                    .global_secondary_index
                    .iter()
                    .map(|v| v.to_gsi_def())
                    .collect(),
                local_secondary_indexes: view
                    .local_secondary_index
                    .iter()
                    .map(|v| v.to_lsi_def())
                    .collect(),
            },
            items: view.items,
            stream_records: Vec::new(),
            stream_sequence_counter: 0,
        }
    }
}

impl From<KeySchemaElementView> for KeySchemaElement {
    fn from(v: KeySchemaElementView) -> Self {
        KeySchemaElement {
            attribute_name: v.attribute_name,
            key_type: v.key_type,
        }
    }
}

impl From<AttributeDefinitionView> for AttributeDefinition {
    fn from(v: AttributeDefinitionView) -> Self {
        AttributeDefinition {
            attribute_name: v.attribute_name,
            attribute_type: v.attribute_type,
        }
    }
}

impl From<ProvisionedThroughputView> for ProvisionedThroughput {
    fn from(v: ProvisionedThroughputView) -> Self {
        ProvisionedThroughput {
            read_capacity_units: v.read_capacity_units,
            write_capacity_units: v.write_capacity_units,
        }
    }
}

impl From<DynamoDbTagView> for DynamoDbTag {
    fn from(v: DynamoDbTagView) -> Self {
        DynamoDbTag {
            key: v.key,
            value: v.value,
        }
    }
}

impl From<TtlConfigView> for TtlConfig {
    fn from(v: TtlConfigView) -> Self {
        TtlConfig {
            attribute_name: v.attribute_name,
            enabled: v.enabled,
        }
    }
}

impl From<ContinuousBackupsConfigView> for ContinuousBackupsConfig {
    fn from(v: ContinuousBackupsConfigView) -> Self {
        ContinuousBackupsConfig {
            point_in_time_recovery_enabled: v.point_in_time_recovery_enabled,
        }
    }
}

impl From<ResourcePolicyView> for ResourcePolicy {
    fn from(v: ResourcePolicyView) -> Self {
        ResourcePolicy {
            policy: v.policy,
            revision_id: v.revision_id,
        }
    }
}

impl From<GlobalTableInfoView> for GlobalTableInfo {
    fn from(v: GlobalTableInfoView) -> Self {
        let creation_date_time = v
            .creation_date_time
            .parse::<DateTime<Utc>>()
            .unwrap_or_else(|_| Utc::now());
        GlobalTableInfo {
            global_table_name: v.global_table_name,
            global_table_arn: v.global_table_arn,
            global_table_status: v.global_table_status,
            creation_date_time,
            replication_group: v.replication_group,
        }
    }
}

impl From<KinesisStreamingDestinationView> for KinesisStreamingDestination {
    fn from(v: KinesisStreamingDestinationView) -> Self {
        KinesisStreamingDestination {
            stream_arn: v.stream_arn,
            destination_status: v.destination_status,
            approximate_creation_date_time_precision: v.approximate_creation_date_time_precision,
        }
    }
}

impl From<ContributorInsightsConfigView> for ContributorInsightsConfig {
    fn from(v: ContributorInsightsConfigView) -> Self {
        let last_update_date_time = v
            .last_update_date_time
            .parse::<DateTime<Utc>>()
            .unwrap_or_else(|_| Utc::now());
        ContributorInsightsConfig {
            table_name: v.table_name,
            index_name: v.index_name,
            status: v.status,
            last_update_date_time,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for DynamoDbService {
    type StateView = DynamodbStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        self.backend
            .snapshot(account_id.to_owned(), region.to_owned())
            .await
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), winterbaume_core::StateViewError> {
        self.backend
            .restore(account_id.to_owned(), region.to_owned(), view)
            .await?;
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), winterbaume_core::StateViewError> {
        self.backend
            .merge(account_id.to_owned(), region.to_owned(), view)
            .await?;
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}

// ---------------------------------------------------------------------------
// SecondaryIndexView ↔ typed index def conversions
// ---------------------------------------------------------------------------

impl SecondaryIndexView {
    fn from_gsi(g: &GlobalSecondaryIndexDef) -> Self {
        SecondaryIndexView {
            index_name: g.index_name.clone(),
            key_schema: g
                .key_schema
                .iter()
                .map(KeySchemaElementView::from)
                .collect(),
            projection_type: g.projection_type.clone(),
            non_key_attributes: g.non_key_attributes.clone(),
        }
    }

    fn from_lsi(l: &LocalSecondaryIndexDef) -> Self {
        SecondaryIndexView {
            index_name: l.index_name.clone(),
            key_schema: l
                .key_schema
                .iter()
                .map(KeySchemaElementView::from)
                .collect(),
            projection_type: l.projection_type.clone(),
            non_key_attributes: l.non_key_attributes.clone(),
        }
    }

    fn to_gsi_def(&self) -> GlobalSecondaryIndexDef {
        GlobalSecondaryIndexDef {
            index_name: self.index_name.clone(),
            key_schema: self
                .key_schema
                .iter()
                .map(|k| KeySchemaElement::from(k.clone()))
                .collect(),
            projection_type: self.projection_type.clone(),
            non_key_attributes: self.non_key_attributes.clone(),
        }
    }

    fn to_lsi_def(&self) -> LocalSecondaryIndexDef {
        LocalSecondaryIndexDef {
            index_name: self.index_name.clone(),
            key_schema: self
                .key_schema
                .iter()
                .map(|k| KeySchemaElement::from(k.clone()))
                .collect(),
            projection_type: self.projection_type.clone(),
            non_key_attributes: self.non_key_attributes.clone(),
        }
    }
}
