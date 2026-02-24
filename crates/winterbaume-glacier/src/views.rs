//! Serde-compatible view types for Glacier state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{BlobBackedService, StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::GlacierService;
use crate::state::GlacierState;
use crate::types::{
    Archive, DataRetrievalPolicy, DataRetrievalRule, Job, JobType, MultipartPart, MultipartUpload,
    ProvisionedCapacity, Vault, VaultLock, VaultLockState, VaultNotificationConfig,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GlacierStateView {
    #[serde(default)]
    pub vaults: HashMap<String, VaultView>,
    #[serde(default)]
    pub data_retrieval_policy: Option<DataRetrievalPolicyView>,
    #[serde(default)]
    pub provisioned_capacity: Vec<ProvisionedCapacityView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultView {
    pub vault_name: String,
    pub arn: String,
    pub created_at: String,
    #[serde(default)]
    pub archives: HashMap<String, ArchiveView>,
    #[serde(default)]
    pub jobs: HashMap<String, JobView>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub access_policy: Option<String>,
    #[serde(default)]
    pub notification_config: Option<VaultNotificationConfigView>,
    #[serde(default)]
    pub vault_lock: Option<VaultLockView>,
    #[serde(default)]
    pub multipart_uploads: HashMap<String, MultipartUploadView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveView {
    pub archive_id: String,
    pub blob_key: String,
    pub size: usize,
    pub sha256: String,
    pub description: String,
    pub creation_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobView {
    pub job_id: String,
    pub job_type: String,
    pub tier: String,
    pub vault_arn: String,
    pub archive_id: Option<String>,
    pub created_at: String,
    pub completed_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultNotificationConfigView {
    pub sns_topic: Option<String>,
    pub events: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultLockView {
    pub lock_id: String,
    pub policy: Option<String>,
    pub state: String,
    pub creation_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultipartUploadView {
    pub upload_id: String,
    pub vault_arn: String,
    pub description: Option<String>,
    pub part_size: Option<i64>,
    pub creation_date: String,
    #[serde(default)]
    pub parts: HashMap<String, MultipartPartView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultipartPartView {
    pub range_in_bytes: String,
    pub sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRetrievalPolicyView {
    pub rules: Vec<DataRetrievalRuleView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRetrievalRuleView {
    pub bytes_per_hour: Option<i64>,
    pub strategy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvisionedCapacityView {
    pub capacity_id: String,
    pub start_date: String,
    pub expiration_date: String,
}

fn parse_dt(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

// --- From internal types to view types ---

impl From<&GlacierState> for GlacierStateView {
    fn from(state: &GlacierState) -> Self {
        GlacierStateView {
            vaults: state
                .vaults
                .iter()
                .map(|(k, v)| (k.clone(), VaultView::from(v)))
                .collect(),
            data_retrieval_policy: state
                .data_retrieval_policy
                .as_ref()
                .map(DataRetrievalPolicyView::from),
            provisioned_capacity: state
                .provisioned_capacity
                .iter()
                .map(ProvisionedCapacityView::from)
                .collect(),
        }
    }
}

impl From<&Vault> for VaultView {
    fn from(v: &Vault) -> Self {
        VaultView {
            vault_name: v.vault_name.clone(),
            arn: v.arn.clone(),
            created_at: v.created_at.to_rfc3339(),
            archives: v
                .archives
                .iter()
                .map(|(k, a)| (k.clone(), ArchiveView::from(a)))
                .collect(),
            jobs: v
                .jobs
                .iter()
                .map(|(k, j)| (k.clone(), JobView::from(j)))
                .collect(),
            tags: v.tags.clone(),
            access_policy: v.access_policy.clone(),
            notification_config: v
                .notification_config
                .as_ref()
                .map(VaultNotificationConfigView::from),
            vault_lock: v.vault_lock.as_ref().map(VaultLockView::from),
            multipart_uploads: v
                .multipart_uploads
                .iter()
                .map(|(k, u)| (k.clone(), MultipartUploadView::from(u)))
                .collect(),
        }
    }
}

impl From<&VaultNotificationConfig> for VaultNotificationConfigView {
    fn from(c: &VaultNotificationConfig) -> Self {
        VaultNotificationConfigView {
            sns_topic: c.sns_topic.clone(),
            events: c.events.clone(),
        }
    }
}

impl From<&VaultLock> for VaultLockView {
    fn from(l: &VaultLock) -> Self {
        VaultLockView {
            lock_id: l.lock_id.clone(),
            policy: l.policy.clone(),
            state: match l.state {
                VaultLockState::InProgress => "InProgress".to_string(),
                VaultLockState::Locked => "Locked".to_string(),
            },
            creation_date: l.creation_date.to_rfc3339(),
        }
    }
}

impl From<&MultipartUpload> for MultipartUploadView {
    fn from(u: &MultipartUpload) -> Self {
        MultipartUploadView {
            upload_id: u.upload_id.clone(),
            vault_arn: u.vault_arn.clone(),
            description: u.description.clone(),
            part_size: u.part_size,
            creation_date: u.creation_date.to_rfc3339(),
            parts: u
                .parts
                .iter()
                .map(|(k, p)| (k.clone(), MultipartPartView::from(p)))
                .collect(),
        }
    }
}

impl From<&MultipartPart> for MultipartPartView {
    fn from(p: &MultipartPart) -> Self {
        MultipartPartView {
            range_in_bytes: p.range_in_bytes.clone(),
            sha256: p.sha256.clone(),
        }
    }
}

impl From<&DataRetrievalPolicy> for DataRetrievalPolicyView {
    fn from(p: &DataRetrievalPolicy) -> Self {
        DataRetrievalPolicyView {
            rules: p.rules.iter().map(DataRetrievalRuleView::from).collect(),
        }
    }
}

impl From<&DataRetrievalRule> for DataRetrievalRuleView {
    fn from(r: &DataRetrievalRule) -> Self {
        DataRetrievalRuleView {
            bytes_per_hour: r.bytes_per_hour,
            strategy: r.strategy.clone(),
        }
    }
}

impl From<&ProvisionedCapacity> for ProvisionedCapacityView {
    fn from(c: &ProvisionedCapacity) -> Self {
        ProvisionedCapacityView {
            capacity_id: c.capacity_id.clone(),
            start_date: c.start_date.to_rfc3339(),
            expiration_date: c.expiration_date.to_rfc3339(),
        }
    }
}

impl From<&Archive> for ArchiveView {
    fn from(a: &Archive) -> Self {
        ArchiveView {
            archive_id: a.archive_id.clone(),
            blob_key: a.blob_key.clone(),
            size: a.size,
            sha256: a.sha256.clone(),
            description: a.description.clone(),
            creation_date: a.creation_date.to_rfc3339(),
        }
    }
}

impl From<&Job> for JobView {
    fn from(j: &Job) -> Self {
        JobView {
            job_id: j.job_id.clone(),
            job_type: match j.job_type {
                JobType::ArchiveRetrieval => "archive-retrieval".to_string(),
                JobType::InventoryRetrieval => "inventory-retrieval".to_string(),
            },
            tier: j.tier.clone(),
            vault_arn: j.vault_arn.clone(),
            archive_id: j.archive_id.clone(),
            created_at: j.created_at.to_rfc3339(),
            completed_at: j.completed_at.to_rfc3339(),
        }
    }
}

// --- From view types to internal types ---

impl From<GlacierStateView> for GlacierState {
    fn from(view: GlacierStateView) -> Self {
        GlacierState {
            vaults: view
                .vaults
                .into_iter()
                .map(|(k, v)| (k, Vault::from(v)))
                .collect(),
            data_retrieval_policy: view.data_retrieval_policy.map(DataRetrievalPolicy::from),
            provisioned_capacity: view
                .provisioned_capacity
                .into_iter()
                .map(ProvisionedCapacity::from)
                .collect(),
        }
    }
}

impl From<VaultView> for Vault {
    fn from(v: VaultView) -> Self {
        Vault {
            vault_name: v.vault_name,
            arn: v.arn,
            created_at: parse_dt(&v.created_at),
            archives: v
                .archives
                .into_iter()
                .map(|(k, a)| (k, Archive::from(a)))
                .collect(),
            jobs: v.jobs.into_iter().map(|(k, j)| (k, Job::from(j))).collect(),
            tags: v.tags,
            access_policy: v.access_policy,
            notification_config: v.notification_config.map(VaultNotificationConfig::from),
            vault_lock: v.vault_lock.map(VaultLock::from),
            multipart_uploads: v
                .multipart_uploads
                .into_iter()
                .map(|(k, u)| (k, MultipartUpload::from(u)))
                .collect(),
        }
    }
}

impl From<VaultNotificationConfigView> for VaultNotificationConfig {
    fn from(v: VaultNotificationConfigView) -> Self {
        VaultNotificationConfig {
            sns_topic: v.sns_topic,
            events: v.events,
        }
    }
}

impl From<VaultLockView> for VaultLock {
    fn from(v: VaultLockView) -> Self {
        VaultLock {
            lock_id: v.lock_id,
            policy: v.policy,
            state: if v.state == "Locked" {
                VaultLockState::Locked
            } else {
                VaultLockState::InProgress
            },
            creation_date: parse_dt(&v.creation_date),
        }
    }
}

impl From<MultipartUploadView> for MultipartUpload {
    fn from(v: MultipartUploadView) -> Self {
        MultipartUpload {
            upload_id: v.upload_id,
            vault_arn: v.vault_arn,
            description: v.description,
            part_size: v.part_size,
            creation_date: parse_dt(&v.creation_date),
            parts: v
                .parts
                .into_iter()
                .map(|(k, p)| (k, MultipartPart::from(p)))
                .collect(),
        }
    }
}

impl From<MultipartPartView> for MultipartPart {
    fn from(v: MultipartPartView) -> Self {
        MultipartPart {
            range_in_bytes: v.range_in_bytes,
            sha256: v.sha256,
        }
    }
}

impl From<DataRetrievalPolicyView> for DataRetrievalPolicy {
    fn from(v: DataRetrievalPolicyView) -> Self {
        DataRetrievalPolicy {
            rules: v.rules.into_iter().map(DataRetrievalRule::from).collect(),
        }
    }
}

impl From<DataRetrievalRuleView> for DataRetrievalRule {
    fn from(v: DataRetrievalRuleView) -> Self {
        DataRetrievalRule {
            bytes_per_hour: v.bytes_per_hour,
            strategy: v.strategy,
        }
    }
}

impl From<ProvisionedCapacityView> for ProvisionedCapacity {
    fn from(v: ProvisionedCapacityView) -> Self {
        ProvisionedCapacity {
            capacity_id: v.capacity_id,
            start_date: parse_dt(&v.start_date),
            expiration_date: parse_dt(&v.expiration_date),
        }
    }
}

impl From<ArchiveView> for Archive {
    fn from(a: ArchiveView) -> Self {
        Archive {
            archive_id: a.archive_id,
            blob_key: a.blob_key,
            size: a.size,
            sha256: a.sha256,
            description: a.description,
            creation_date: parse_dt(&a.creation_date),
        }
    }
}

impl From<JobView> for Job {
    fn from(j: JobView) -> Self {
        Job {
            job_id: j.job_id,
            job_type: if j.job_type == "inventory-retrieval" {
                JobType::InventoryRetrieval
            } else {
                JobType::ArchiveRetrieval
            },
            tier: j.tier,
            vault_arn: j.vault_arn,
            archive_id: j.archive_id,
            created_at: parse_dt(&j.created_at),
            completed_at: parse_dt(&j.completed_at),
        }
    }
}

// --- Shared snapshot logic ---

impl GlacierService {
    fn snapshot_inner(state: &GlacierState) -> (GlacierStateView, Vec<String>) {
        let view = GlacierStateView::from(state);
        let keys = view
            .vaults
            .values()
            .flat_map(|v| v.archives.values())
            .filter(|a| !a.blob_key.is_empty())
            .map(|a| a.blob_key.clone())
            .collect();
        (view, keys)
    }
}

// --- StatefulService implementation ---

impl StatefulService for GlacierService {
    type StateView = GlacierStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        Self::snapshot_inner(&guard).0
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
            *guard = GlacierState::from(view);
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
            for (name, vault_view) in view.vaults {
                guard.vaults.insert(name, Vault::from(vault_view));
            }
            if let Some(policy) = view.data_retrieval_policy {
                guard.data_retrieval_policy = Some(DataRetrievalPolicy::from(policy));
            }
            for cap in view.provisioned_capacity {
                guard
                    .provisioned_capacity
                    .push(ProvisionedCapacity::from(cap));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}

impl BlobBackedService for GlacierService {
    async fn snapshot_with_blobs(
        &self,
        account_id: &str,
        region: &str,
        visitor: &mut dyn winterbaume_core::BlobVisitor,
    ) -> Result<GlacierStateView, winterbaume_core::StateViewError> {
        let blobs = self.blobs.get(account_id, region);
        let state = self.state.get(account_id, region);
        let state = state.read().await;
        let (view, keys) = Self::snapshot_inner(&state);
        for chunk in keys.chunks(winterbaume_core::DEFAULT_BLOB_BATCH_SIZE) {
            let mut entries = Vec::with_capacity(chunk.len());
            for key in chunk {
                let size = blobs
                    .stat(key)
                    .await
                    .map_err(winterbaume_core::StateViewError::Blob)?
                    .map(|s| s.size);
                let reader = blobs
                    .get_reader(key)
                    .await
                    .map_err(winterbaume_core::StateViewError::Blob)?;
                entries.push(winterbaume_core::BlobExportEntry {
                    key: key.clone(),
                    reader,
                    size,
                });
            }
            visitor
                .visit(entries)
                .await
                .map_err(winterbaume_core::StateViewError::Blob)?;
        }
        Ok(view)
    }

    async fn restore_with_blobs(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
        source: &mut dyn winterbaume_core::BlobSource,
    ) -> Result<(), winterbaume_core::StateViewError> {
        let blobs = self.blobs.get(account_id, region);

        // When no snapshot is available (empty view), reconstruct state entirely
        // from the BlobStore sidecars written during previous mutations.
        if view.vaults.is_empty()
            && view.data_retrieval_policy.is_none()
            && view.provisioned_capacity.is_empty()
        {
            let recovered = GlacierService::recover_state_from_blobs(&blobs)
                .await
                .map_err(winterbaume_core::StateViewError::Blob)?;
            let state = self.state.get(account_id, region);
            let mut guard = state.write().await;
            *guard = recovered;
            return Ok(());
        }

        let new_state = GlacierState::from(view);
        // Import blobs referenced by the incoming view before replacing state.
        for archive in new_state
            .vaults
            .values()
            .flat_map(|vault| vault.archives.values())
        {
            if !archive.blob_key.is_empty() {
                let mut reader = source.fetch(archive.blob_key.clone()).await?;
                blobs
                    .put_from(&archive.blob_key, &mut reader)
                    .await
                    .map_err(winterbaume_core::StateViewError::Blob)?;
            }
        }
        let state = self.state.get(account_id, region);
        let mut guard = state.write().await;
        *guard = new_state;
        Ok(())
    }
}
