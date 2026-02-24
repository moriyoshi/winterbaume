use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::AuditManagerService;
use crate::state::AuditManagerState;
use crate::types::{Assessment, Control, Framework};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuditManagerStateView {
    #[serde(default)]
    pub controls: HashMap<String, ControlView>,
    #[serde(default)]
    pub frameworks: HashMap<String, FrameworkView>,
    #[serde(default)]
    pub assessments: HashMap<String, AssessmentView>,
    #[serde(default)]
    pub account_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlView {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub control_type: String,
    pub created_at: f64,
    pub tags: HashMap<String, String>,
    /// `control_mapping_sources` nested blocks.
    #[serde(default)]
    pub control_mapping_sources: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameworkView {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub compliance_type: Option<String>,
    pub framework_type: String,
    pub created_at: f64,
    pub tags: HashMap<String, String>,
    /// `control_sets` nested blocks.
    #[serde(default)]
    pub control_sets: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssessmentView {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub framework_id: String,
    pub status: String,
    pub created_at: f64,
    pub tags: HashMap<String, String>,
}

impl From<&Control> for ControlView {
    fn from(c: &Control) -> Self {
        Self {
            id: c.id.clone(),
            name: c.name.clone(),
            description: c.description.clone(),
            control_type: c.control_type.clone(),
            created_at: c.created_at,
            tags: c.tags.clone(),
            control_mapping_sources: vec![],
        }
    }
}

impl From<ControlView> for Control {
    fn from(v: ControlView) -> Self {
        Self {
            id: v.id,
            name: v.name,
            description: v.description,
            control_type: v.control_type,
            created_at: v.created_at,
            tags: v.tags,
        }
    }
}

impl From<&Framework> for FrameworkView {
    fn from(f: &Framework) -> Self {
        Self {
            id: f.id.clone(),
            name: f.name.clone(),
            description: f.description.clone(),
            compliance_type: f.compliance_type.clone(),
            framework_type: f.framework_type.clone(),
            created_at: f.created_at,
            tags: f.tags.clone(),
            control_sets: vec![],
        }
    }
}

impl From<FrameworkView> for Framework {
    fn from(v: FrameworkView) -> Self {
        Self {
            id: v.id,
            name: v.name,
            description: v.description,
            compliance_type: v.compliance_type,
            framework_type: v.framework_type,
            created_at: v.created_at,
            tags: v.tags,
        }
    }
}

impl From<&Assessment> for AssessmentView {
    fn from(a: &Assessment) -> Self {
        Self {
            id: a.id.clone(),
            name: a.name.clone(),
            description: a.description.clone(),
            framework_id: a.framework_id.clone(),
            status: a.status.clone(),
            created_at: a.created_at,
            tags: a.tags.clone(),
        }
    }
}

impl From<AssessmentView> for Assessment {
    fn from(v: AssessmentView) -> Self {
        Self {
            id: v.id,
            name: v.name,
            description: v.description,
            framework_id: v.framework_id,
            status: v.status,
            created_at: v.created_at,
            tags: v.tags,
        }
    }
}

impl From<&AuditManagerState> for AuditManagerStateView {
    fn from(state: &AuditManagerState) -> Self {
        Self {
            controls: state
                .controls
                .iter()
                .map(|(k, v)| (k.clone(), ControlView::from(v)))
                .collect(),
            frameworks: state
                .frameworks
                .iter()
                .map(|(k, v)| (k.clone(), FrameworkView::from(v)))
                .collect(),
            assessments: state
                .assessments
                .iter()
                .map(|(k, v)| (k.clone(), AssessmentView::from(v)))
                .collect(),
            account_status: state.account_status.clone(),
        }
    }
}

impl From<AuditManagerStateView> for AuditManagerState {
    fn from(view: AuditManagerStateView) -> Self {
        Self {
            controls: view
                .controls
                .into_iter()
                .map(|(k, v)| (k, Control::from(v)))
                .collect(),
            frameworks: view
                .frameworks
                .into_iter()
                .map(|(k, v)| (k, Framework::from(v)))
                .collect(),
            assessments: view
                .assessments
                .into_iter()
                .map(|(k, v)| (k, Assessment::from(v)))
                .collect(),
            account_status: if view.account_status.is_empty() {
                "INACTIVE".to_string()
            } else {
                view.account_status
            },
        }
    }
}

impl StatefulService for AuditManagerService {
    type StateView = AuditManagerStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        AuditManagerStateView::from(&*guard)
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
            *guard = AuditManagerState::from(view);
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
            for (k, v) in view.controls {
                guard.controls.insert(k, Control::from(v));
            }
            for (k, v) in view.frameworks {
                guard.frameworks.insert(k, Framework::from(v));
            }
            for (k, v) in view.assessments {
                guard.assessments.insert(k, Assessment::from(v));
            }
            if !view.account_status.is_empty() {
                guard.account_status = view.account_status;
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
