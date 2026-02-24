//! Serde-compatible view types for SDB state snapshots.

use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::SimpleDbV2Service;
use crate::state::SdbState;
use crate::types::Export;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SdbStateView {
    #[serde(default)]
    pub exports: HashMap<String, ExportView>,
    #[serde(default)]
    pub domains: HashSet<String>,
    #[serde(default)]
    pub next_export_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportView {
    pub export_arn: String,
    pub client_token: String,
    pub export_status: String,
    pub domain_name: String,
    pub requested_at: DateTime<Utc>,
    pub s3_bucket: String,
    pub s3_key_prefix: Option<String>,
    pub s3_sse_algorithm: Option<String>,
    pub s3_sse_kms_key_id: Option<String>,
    pub s3_bucket_owner: Option<String>,
    pub failure_code: Option<String>,
    pub failure_message: Option<String>,
    pub export_manifest: Option<String>,
    pub items_count: Option<i64>,
    pub export_data_cutoff_time: Option<DateTime<Utc>>,
}

impl From<&SdbState> for SdbStateView {
    fn from(state: &SdbState) -> Self {
        SdbStateView {
            exports: state
                .exports
                .iter()
                .map(|(k, v)| (k.clone(), ExportView::from(v)))
                .collect(),
            domains: state.domains.clone(),
            next_export_id: state.next_export_id,
        }
    }
}

impl From<&Export> for ExportView {
    fn from(e: &Export) -> Self {
        ExportView {
            export_arn: e.export_arn.clone(),
            client_token: e.client_token.clone(),
            export_status: e.export_status.clone(),
            domain_name: e.domain_name.clone(),
            requested_at: e.requested_at,
            s3_bucket: e.s3_bucket.clone(),
            s3_key_prefix: e.s3_key_prefix.clone(),
            s3_sse_algorithm: e.s3_sse_algorithm.clone(),
            s3_sse_kms_key_id: e.s3_sse_kms_key_id.clone(),
            s3_bucket_owner: e.s3_bucket_owner.clone(),
            failure_code: e.failure_code.clone(),
            failure_message: e.failure_message.clone(),
            export_manifest: e.export_manifest.clone(),
            items_count: e.items_count,
            export_data_cutoff_time: e.export_data_cutoff_time,
        }
    }
}

impl From<SdbStateView> for SdbState {
    fn from(view: SdbStateView) -> Self {
        SdbState {
            exports: view
                .exports
                .into_iter()
                .map(|(k, v)| (k, Export::from(v)))
                .collect(),
            domains: view.domains,
            next_export_id: view.next_export_id,
        }
    }
}

impl From<ExportView> for Export {
    fn from(v: ExportView) -> Self {
        Export {
            export_arn: v.export_arn,
            client_token: v.client_token,
            export_status: v.export_status,
            domain_name: v.domain_name,
            requested_at: v.requested_at,
            s3_bucket: v.s3_bucket,
            s3_key_prefix: v.s3_key_prefix,
            s3_sse_algorithm: v.s3_sse_algorithm,
            s3_sse_kms_key_id: v.s3_sse_kms_key_id,
            s3_bucket_owner: v.s3_bucket_owner,
            failure_code: v.failure_code,
            failure_message: v.failure_message,
            export_manifest: v.export_manifest,
            items_count: v.items_count,
            export_data_cutoff_time: v.export_data_cutoff_time,
        }
    }
}

impl StatefulService for SimpleDbV2Service {
    type StateView = SdbStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        SdbStateView::from(&*guard)
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
            *guard = SdbState::from(view);
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
            for (k, v) in view.exports {
                guard.exports.insert(k, Export::from(v));
            }
            for domain in view.domains {
                guard.domains.insert(domain);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
