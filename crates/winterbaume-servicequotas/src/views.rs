//! Serde-compatible view types for ServiceQuotas state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::ServiceQuotasService;
use crate::state::ServiceQuotasState;
use crate::types::{ServiceEntry, ServiceQuotaEntry};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceQuotasStateView {
    #[serde(default)]
    pub quotas: HashMap<String, ServiceQuotaEntryView>,
    #[serde(default)]
    pub services: HashMap<String, ServiceEntryView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceQuotaEntryView {
    pub service_code: String,
    pub service_name: String,
    pub quota_code: String,
    pub quota_name: String,
    pub quota_arn: String,
    pub value: f64,
    pub unit: String,
    pub adjustable: bool,
    pub global_quota: bool,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEntryView {
    pub service_code: String,
    pub service_name: String,
}

impl From<&ServiceQuotasState> for ServiceQuotasStateView {
    fn from(state: &ServiceQuotasState) -> Self {
        ServiceQuotasStateView {
            quotas: state
                .quotas
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        ServiceQuotaEntryView {
                            service_code: v.service_code.clone(),
                            service_name: v.service_name.clone(),
                            quota_code: v.quota_code.clone(),
                            quota_name: v.quota_name.clone(),
                            quota_arn: v.quota_arn.clone(),
                            value: v.value,
                            unit: v.unit.clone(),
                            adjustable: v.adjustable,
                            global_quota: v.global_quota,
                            description: v.description.clone(),
                        },
                    )
                })
                .collect(),
            services: state
                .services
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        ServiceEntryView {
                            service_code: v.service_code.clone(),
                            service_name: v.service_name.clone(),
                        },
                    )
                })
                .collect(),
        }
    }
}

impl From<ServiceQuotasStateView> for ServiceQuotasState {
    fn from(view: ServiceQuotasStateView) -> Self {
        ServiceQuotasState {
            quotas: view
                .quotas
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        ServiceQuotaEntry {
                            service_code: v.service_code,
                            service_name: v.service_name,
                            quota_code: v.quota_code,
                            quota_name: v.quota_name,
                            quota_arn: v.quota_arn,
                            value: v.value,
                            unit: v.unit,
                            adjustable: v.adjustable,
                            global_quota: v.global_quota,
                            description: v.description,
                        },
                    )
                })
                .collect(),
            services: view
                .services
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        ServiceEntry {
                            service_code: v.service_code,
                            service_name: v.service_name,
                        },
                    )
                })
                .collect(),
        }
    }
}

impl StatefulService for ServiceQuotasService {
    type StateView = ServiceQuotasStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        ServiceQuotasStateView::from(&*guard)
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
            *guard = ServiceQuotasState::from(view);
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
            for (k, v) in view.quotas {
                guard.quotas.insert(
                    k,
                    ServiceQuotaEntry {
                        service_code: v.service_code,
                        service_name: v.service_name,
                        quota_code: v.quota_code,
                        quota_name: v.quota_name,
                        quota_arn: v.quota_arn,
                        value: v.value,
                        unit: v.unit,
                        adjustable: v.adjustable,
                        global_quota: v.global_quota,
                        description: v.description,
                    },
                );
            }
            for (k, v) in view.services {
                guard.services.insert(
                    k,
                    ServiceEntry {
                        service_code: v.service_code,
                        service_name: v.service_name,
                    },
                );
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
