//! Serde-compatible view types for Support state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::SupportService;
use crate::state::SupportState;

/// Serializable view of the entire Support state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SupportStateView {
    /// Support cases keyed by case ID.
    #[serde(default)]
    pub cases: HashMap<String, SupportCaseView>,
    /// Refresh call counts for trusted advisor checks, keyed by check ID.
    #[serde(default)]
    pub refresh_call_counts: HashMap<String, usize>,
}

/// Serializable view of a single support case.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportCaseView {
    pub case_id: String,
    pub display_id: String,
    pub subject: String,
    pub status: String,
    pub service_code: String,
    pub category_code: String,
    pub severity_code: String,
    pub communication_body: String,
    pub submitted_by: String,
    pub time_created: String,
    #[serde(default)]
    pub cc_email_addresses: Vec<String>,
    pub language: String,
}

// --- From internal types to view types ---

impl From<&SupportState> for SupportStateView {
    fn from(state: &SupportState) -> Self {
        SupportStateView {
            cases: state
                .cases
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        SupportCaseView {
                            case_id: v.case_id.clone(),
                            display_id: v.display_id.clone(),
                            subject: v.subject.clone(),
                            status: v.status.clone(),
                            service_code: v.service_code.clone(),
                            category_code: v.category_code.clone(),
                            severity_code: v.severity_code.clone(),
                            communication_body: v.communication_body.clone(),
                            submitted_by: v.submitted_by.clone(),
                            time_created: v.time_created.clone(),
                            cc_email_addresses: v.cc_email_addresses.clone(),
                            language: v.language.clone(),
                        },
                    )
                })
                .collect(),
            refresh_call_counts: state.refresh_call_counts.clone(),
        }
    }
}

// --- From view types to internal types ---

fn support_state_from_view(view: SupportStateView) -> SupportState {
    use crate::types::SupportCase;
    let mut state = SupportState::default();
    state.cases = view
        .cases
        .into_iter()
        .map(|(k, v)| {
            (
                k,
                SupportCase {
                    case_id: v.case_id,
                    display_id: v.display_id,
                    subject: v.subject,
                    status: v.status,
                    service_code: v.service_code,
                    category_code: v.category_code,
                    severity_code: v.severity_code,
                    communication_body: v.communication_body,
                    submitted_by: v.submitted_by,
                    time_created: v.time_created,
                    cc_email_addresses: v.cc_email_addresses,
                    language: v.language,
                },
            )
        })
        .collect();
    state.refresh_call_counts = view.refresh_call_counts;
    state
}

// --- StatefulService implementation ---

impl StatefulService for SupportService {
    type StateView = SupportStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        SupportStateView::from(&*guard)
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
            *guard = support_state_from_view(view);
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
            for (k, v) in view.cases {
                use crate::types::SupportCase;
                guard.cases.insert(
                    k,
                    SupportCase {
                        case_id: v.case_id,
                        display_id: v.display_id,
                        subject: v.subject,
                        status: v.status,
                        service_code: v.service_code,
                        category_code: v.category_code,
                        severity_code: v.severity_code,
                        communication_body: v.communication_body,
                        submitted_by: v.submitted_by,
                        time_created: v.time_created,
                        cc_email_addresses: v.cc_email_addresses,
                        language: v.language,
                    },
                );
            }
            for (k, v) in view.refresh_call_counts {
                guard.refresh_call_counts.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
