//! Serde-compatible view types for Shield state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::ShieldService;
use crate::state::ShieldState;
use crate::types::{AutoRenew, Protection, Subscription, Tag};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShieldStateView {
    pub subscription: Option<SubscriptionView>,
    #[serde(default)]
    pub protections: HashMap<String, ProtectionView>,
    #[serde(default)]
    pub tags: HashMap<String, Vec<TagView>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionView {
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub time_commitment_in_seconds: i64,
    pub auto_renew: String,
    pub subscription_arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtectionView {
    pub id: String,
    pub name: String,
    pub resource_arn: String,
    pub protection_arn: String,
    #[serde(default)]
    pub health_check_ids: Vec<String>,
    #[serde(default)]
    pub tags: Vec<TagView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagView {
    pub key: String,
    pub value: String,
}

impl From<&ShieldState> for ShieldStateView {
    fn from(state: &ShieldState) -> Self {
        ShieldStateView {
            subscription: state.subscription.as_ref().map(|s| SubscriptionView {
                start_time: s.start_time,
                end_time: s.end_time,
                time_commitment_in_seconds: s.time_commitment_in_seconds,
                auto_renew: s.auto_renew.as_str().to_string(),
                subscription_arn: s.subscription_arn.clone(),
            }),
            protections: state
                .protections
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        ProtectionView {
                            id: v.id.clone(),
                            name: v.name.clone(),
                            resource_arn: v.resource_arn.clone(),
                            protection_arn: v.protection_arn.clone(),
                            health_check_ids: v.health_check_ids.clone(),
                            tags: v
                                .tags
                                .iter()
                                .map(|t| TagView {
                                    key: t.key.clone(),
                                    value: t.value.clone(),
                                })
                                .collect(),
                        },
                    )
                })
                .collect(),
            tags: state
                .tags
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        v.iter()
                            .map(|t| TagView {
                                key: t.key.clone(),
                                value: t.value.clone(),
                            })
                            .collect(),
                    )
                })
                .collect(),
        }
    }
}

impl From<ShieldStateView> for ShieldState {
    fn from(view: ShieldStateView) -> Self {
        ShieldState {
            subscription: view.subscription.map(|s| Subscription {
                start_time: s.start_time,
                end_time: s.end_time,
                time_commitment_in_seconds: s.time_commitment_in_seconds,
                auto_renew: if s.auto_renew == "ENABLED" {
                    AutoRenew::Enabled
                } else {
                    AutoRenew::Disabled
                },
                subscription_arn: s.subscription_arn,
            }),
            protections: view
                .protections
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        Protection {
                            id: v.id,
                            name: v.name,
                            resource_arn: v.resource_arn,
                            protection_arn: v.protection_arn,
                            health_check_ids: v.health_check_ids,
                            tags: v
                                .tags
                                .into_iter()
                                .map(|t| Tag {
                                    key: t.key,
                                    value: t.value,
                                })
                                .collect(),
                        },
                    )
                })
                .collect(),
            tags: view
                .tags
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        v.into_iter()
                            .map(|t| Tag {
                                key: t.key,
                                value: t.value,
                            })
                            .collect(),
                    )
                })
                .collect(),
        }
    }
}

impl StatefulService for ShieldService {
    type StateView = ShieldStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        ShieldStateView::from(&*guard)
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
            *guard = ShieldState::from(view);
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
            if let Some(sub) = view.subscription {
                guard.subscription = Some(Subscription {
                    start_time: sub.start_time,
                    end_time: sub.end_time,
                    time_commitment_in_seconds: sub.time_commitment_in_seconds,
                    auto_renew: if sub.auto_renew == "ENABLED" {
                        AutoRenew::Enabled
                    } else {
                        AutoRenew::Disabled
                    },
                    subscription_arn: sub.subscription_arn,
                });
            }
            for (k, v) in view.protections {
                guard.protections.insert(
                    k,
                    Protection {
                        id: v.id,
                        name: v.name,
                        resource_arn: v.resource_arn,
                        protection_arn: v.protection_arn,
                        health_check_ids: v.health_check_ids,
                        tags: v
                            .tags
                            .into_iter()
                            .map(|t| Tag {
                                key: t.key,
                                value: t.value,
                            })
                            .collect(),
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
