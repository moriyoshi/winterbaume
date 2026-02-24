use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::ArtifactService;
use crate::state::ArtifactState;
use crate::types::{AccountSettings, CustomerAgreement, Report};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ArtifactStateView {
    #[serde(default)]
    pub notification_subscription_status: Option<String>,
    #[serde(default)]
    pub reports: HashMap<String, ReportView>,
    #[serde(default)]
    pub customer_agreements: HashMap<String, CustomerAgreementView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportView {
    pub id: String,
    pub version: i64,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub series: Option<String>,
    pub state: String,
    pub arn: String,
    pub document_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerAgreementView {
    pub name: String,
    pub arn: String,
    pub id: String,
    pub agreement_arn: String,
    pub aws_account_id: String,
    #[serde(default)]
    pub organization_arn: Option<String>,
    #[serde(default)]
    pub effective_start: Option<i64>,
    #[serde(default)]
    pub effective_end: Option<i64>,
    pub state: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

impl From<&Report> for ReportView {
    fn from(r: &Report) -> Self {
        Self {
            id: r.id.clone(),
            version: r.version,
            name: r.name.clone(),
            description: r.description.clone(),
            category: r.category.clone(),
            series: r.series.clone(),
            state: r.state.clone(),
            arn: r.arn.clone(),
            document_url: r.document_url.clone(),
        }
    }
}

impl From<ReportView> for Report {
    fn from(v: ReportView) -> Self {
        Self {
            id: v.id,
            version: v.version,
            name: v.name,
            description: v.description,
            category: v.category,
            series: v.series,
            state: v.state,
            arn: v.arn,
            document_url: v.document_url,
        }
    }
}

impl From<&CustomerAgreement> for CustomerAgreementView {
    fn from(c: &CustomerAgreement) -> Self {
        Self {
            name: c.name.clone(),
            arn: c.arn.clone(),
            id: c.id.clone(),
            agreement_arn: c.agreement_arn.clone(),
            aws_account_id: c.aws_account_id.clone(),
            organization_arn: c.organization_arn.clone(),
            effective_start: c.effective_start,
            effective_end: c.effective_end,
            state: c.state.clone(),
            tags: c.tags.clone(),
        }
    }
}

impl From<CustomerAgreementView> for CustomerAgreement {
    fn from(v: CustomerAgreementView) -> Self {
        Self {
            name: v.name,
            arn: v.arn,
            id: v.id,
            agreement_arn: v.agreement_arn,
            aws_account_id: v.aws_account_id,
            organization_arn: v.organization_arn,
            effective_start: v.effective_start,
            effective_end: v.effective_end,
            state: v.state,
            tags: v.tags,
        }
    }
}

impl From<&ArtifactState> for ArtifactStateView {
    fn from(state: &ArtifactState) -> Self {
        Self {
            notification_subscription_status: state
                .account_settings
                .notification_subscription_status
                .clone(),
            reports: state
                .reports
                .iter()
                .map(|(k, v)| (k.clone(), ReportView::from(v)))
                .collect(),
            customer_agreements: state
                .customer_agreements
                .iter()
                .map(|(k, v)| (k.clone(), CustomerAgreementView::from(v)))
                .collect(),
        }
    }
}

impl From<ArtifactStateView> for ArtifactState {
    fn from(view: ArtifactStateView) -> Self {
        Self {
            account_settings: AccountSettings {
                notification_subscription_status: view.notification_subscription_status,
            },
            reports: view
                .reports
                .into_iter()
                .map(|(k, v)| (k, Report::from(v)))
                .collect(),
            customer_agreements: view
                .customer_agreements
                .into_iter()
                .map(|(k, v)| (k, CustomerAgreement::from(v)))
                .collect(),
            term_tokens: HashMap::new(),
        }
    }
}

impl StatefulService for ArtifactService {
    type StateView = ArtifactStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        ArtifactStateView::from(&*guard)
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
            *guard = ArtifactState::from(view);
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
            if let Some(s) = view.notification_subscription_status {
                guard.account_settings.notification_subscription_status = Some(s);
            }
            for (k, v) in view.reports {
                guard.reports.insert(k, Report::from(v));
            }
            for (k, v) in view.customer_agreements {
                guard
                    .customer_agreements
                    .insert(k, CustomerAgreement::from(v));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
