use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::AccessAnalyzerService;
use crate::state::{AccessAnalyzerState, AnalyzerState};
use crate::types::{Analyzer, ArchiveRule, CriterionValue};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccessAnalyzerStateView {
    #[serde(default)]
    pub analyzers: HashMap<String, AnalyzerView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzerView {
    pub arn: String,
    pub name: String,
    pub analyzer_type: String,
    pub status: String,
    pub created_at: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub archive_rules: HashMap<String, ArchiveRuleView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveRuleView {
    pub rule_name: String,
    pub filter: HashMap<String, CriterionValueView>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CriterionValueView {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eq: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neq: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exists: Option<bool>,
}

// --- From internal types to view types ---

impl From<&AccessAnalyzerState> for AccessAnalyzerStateView {
    fn from(state: &AccessAnalyzerState) -> Self {
        AccessAnalyzerStateView {
            analyzers: state
                .analyzers
                .iter()
                .map(|(k, v)| (k.clone(), AnalyzerView::from(v)))
                .collect(),
        }
    }
}

impl From<&AnalyzerState> for AnalyzerView {
    fn from(s: &AnalyzerState) -> Self {
        AnalyzerView {
            arn: s.analyzer.arn.clone(),
            name: s.analyzer.name.clone(),
            analyzer_type: s.analyzer.analyzer_type.clone(),
            status: s.analyzer.status.clone(),
            created_at: s.analyzer.created_at.clone(),
            tags: s.analyzer.tags.clone(),
            archive_rules: s
                .archive_rules
                .iter()
                .map(|(k, v)| (k.clone(), ArchiveRuleView::from(v)))
                .collect(),
        }
    }
}

impl From<&ArchiveRule> for ArchiveRuleView {
    fn from(r: &ArchiveRule) -> Self {
        ArchiveRuleView {
            rule_name: r.rule_name.clone(),
            filter: r
                .filter
                .iter()
                .map(|(k, v)| (k.clone(), CriterionValueView::from(v)))
                .collect(),
            created_at: r.created_at.clone(),
            updated_at: r.updated_at.clone(),
        }
    }
}

impl From<&CriterionValue> for CriterionValueView {
    fn from(c: &CriterionValue) -> Self {
        CriterionValueView {
            eq: c.eq.clone(),
            neq: c.neq.clone(),
            contains: c.contains.clone(),
            exists: c.exists,
        }
    }
}

// --- From view types to internal types ---

impl From<AccessAnalyzerStateView> for AccessAnalyzerState {
    fn from(view: AccessAnalyzerStateView) -> Self {
        AccessAnalyzerState {
            analyzers: view
                .analyzers
                .into_iter()
                .map(|(k, v)| (k, AnalyzerState::from(v)))
                .collect(),
        }
    }
}

impl From<AnalyzerView> for AnalyzerState {
    fn from(v: AnalyzerView) -> Self {
        AnalyzerState {
            analyzer: Analyzer {
                arn: v.arn,
                name: v.name,
                analyzer_type: v.analyzer_type,
                status: v.status,
                created_at: v.created_at,
                tags: v.tags,
            },
            archive_rules: v
                .archive_rules
                .into_iter()
                .map(|(k, v)| (k, ArchiveRule::from(v)))
                .collect(),
        }
    }
}

impl From<ArchiveRuleView> for ArchiveRule {
    fn from(v: ArchiveRuleView) -> Self {
        ArchiveRule {
            rule_name: v.rule_name,
            filter: v
                .filter
                .into_iter()
                .map(|(k, v)| (k, CriterionValue::from(v)))
                .collect(),
            created_at: v.created_at,
            updated_at: v.updated_at,
        }
    }
}

impl From<CriterionValueView> for CriterionValue {
    fn from(v: CriterionValueView) -> Self {
        CriterionValue {
            eq: v.eq,
            neq: v.neq,
            contains: v.contains,
            exists: v.exists,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for AccessAnalyzerService {
    type StateView = AccessAnalyzerStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        AccessAnalyzerStateView::from(&*guard)
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
            *guard = AccessAnalyzerState::from(view);
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
            for (k, v) in view.analyzers {
                guard.analyzers.insert(k, AnalyzerState::from(v));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
