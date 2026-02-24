use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::CodeGuruSecurityService;
use crate::state::{CodeGuruSecurityState, ScanRecord};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CodeGuruSecurityStateView {
    #[serde(default)]
    pub scans: HashMap<String, ScanRecordView>,
    #[serde(default)]
    pub findings: HashMap<String, Vec<Value>>,
    #[serde(default)]
    pub account_configuration: Value,
    #[serde(default)]
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScanRecordView {
    #[serde(default)]
    pub scan: Value,
}

impl From<&ScanRecord> for ScanRecordView {
    fn from(s: &ScanRecord) -> Self {
        Self {
            scan: s.scan.clone(),
        }
    }
}

impl From<ScanRecordView> for ScanRecord {
    fn from(v: ScanRecordView) -> Self {
        Self { scan: v.scan }
    }
}

impl From<&CodeGuruSecurityState> for CodeGuruSecurityStateView {
    fn from(s: &CodeGuruSecurityState) -> Self {
        Self {
            scans: s.scans.iter().map(|(k, v)| (k.clone(), v.into())).collect(),
            findings: s.findings.clone(),
            account_configuration: s.account_configuration.clone(),
            tags: s.tags.clone(),
        }
    }
}

impl From<CodeGuruSecurityStateView> for CodeGuruSecurityState {
    fn from(v: CodeGuruSecurityStateView) -> Self {
        Self {
            scans: v.scans.into_iter().map(|(k, s)| (k, s.into())).collect(),
            findings: v.findings,
            account_configuration: v.account_configuration,
            tags: v.tags,
        }
    }
}

impl StatefulService for CodeGuruSecurityService {
    type StateView = CodeGuruSecurityStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        CodeGuruSecurityStateView::from(&*guard)
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
            *guard = CodeGuruSecurityState::from(view);
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
            for (k, v) in view.scans {
                guard.scans.insert(k, v.into());
            }
            for (k, v) in view.findings {
                guard.findings.insert(k, v);
            }
            for (k, v) in view.tags {
                guard.tags.insert(k, v);
            }
            if !view.account_configuration.is_null() {
                guard.account_configuration = view.account_configuration;
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
