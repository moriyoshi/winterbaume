//! Serde-compatible view types for ServiceCatalog state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::ServiceCatalogService;
use crate::state::ServiceCatalogState;
use crate::types::{PortfolioDetail, PortfolioTag, ProductDetail};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceCatalogStateView {
    #[serde(default)]
    pub portfolios: HashMap<String, PortfolioDetailView>,
    #[serde(default)]
    pub products: HashMap<String, ProductView>,
    #[serde(default)]
    pub idempotency_tokens: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioDetailView {
    pub id: String,
    pub arn: String,
    pub display_name: String,
    pub description: String,
    pub created_time: DateTime<Utc>,
    pub provider_name: String,
    #[serde(default)]
    pub tags: Vec<PortfolioTagView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioTagView {
    pub key: String,
    pub value: String,
}

/// Serializable view of a Service Catalog product.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductView {
    pub product_id: String,
    pub product_arn: String,
    pub name: String,
    pub owner: String,
    pub description: Option<String>,
    pub product_type: String,
    pub distributor: Option<String>,
    pub support_description: Option<String>,
    pub support_email: Option<String>,
    pub support_url: Option<String>,
    pub created_time: String,
    #[serde(default)]
    pub tags: Vec<PortfolioTagView>,
}

impl From<&ServiceCatalogState> for ServiceCatalogStateView {
    fn from(state: &ServiceCatalogState) -> Self {
        ServiceCatalogStateView {
            portfolios: state
                .portfolios
                .iter()
                .map(|(k, v)| (k.clone(), PortfolioDetailView::from(v)))
                .collect(),
            products: state
                .products
                .iter()
                .map(|(k, v)| (k.clone(), ProductView::from(v)))
                .collect(),
            idempotency_tokens: state.idempotency_tokens.clone(),
        }
    }
}

impl From<&PortfolioDetail> for PortfolioDetailView {
    fn from(p: &PortfolioDetail) -> Self {
        PortfolioDetailView {
            id: p.id.clone(),
            arn: p.arn.clone(),
            display_name: p.display_name.clone(),
            description: p.description.clone(),
            created_time: p.created_time,
            provider_name: p.provider_name.clone(),
            tags: p
                .tags
                .iter()
                .map(|t| PortfolioTagView {
                    key: t.key.clone(),
                    value: t.value.clone(),
                })
                .collect(),
        }
    }
}

impl From<&ProductDetail> for ProductView {
    fn from(p: &ProductDetail) -> Self {
        ProductView {
            product_id: p.product_id.clone(),
            product_arn: p.product_arn.clone(),
            name: p.name.clone(),
            owner: p.owner.clone(),
            description: p.description.clone(),
            product_type: p.product_type.clone(),
            distributor: p.distributor.clone(),
            support_description: p.support_description.clone(),
            support_email: p.support_email.clone(),
            support_url: p.support_url.clone(),
            created_time: p.created_time.to_rfc3339(),
            tags: p
                .tags
                .iter()
                .map(|t| PortfolioTagView {
                    key: t.key.clone(),
                    value: t.value.clone(),
                })
                .collect(),
        }
    }
}

impl From<ProductView> for ProductDetail {
    fn from(v: ProductView) -> Self {
        let created_time = DateTime::parse_from_rfc3339(&v.created_time)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());
        ProductDetail {
            product_id: v.product_id,
            product_arn: v.product_arn,
            name: v.name,
            owner: v.owner,
            description: v.description,
            product_type: v.product_type,
            distributor: v.distributor,
            support_description: v.support_description,
            support_email: v.support_email,
            support_url: v.support_url,
            created_time,
            tags: v
                .tags
                .into_iter()
                .map(|t| PortfolioTag {
                    key: t.key,
                    value: t.value,
                })
                .collect(),
        }
    }
}

impl From<ServiceCatalogStateView> for ServiceCatalogState {
    fn from(view: ServiceCatalogStateView) -> Self {
        ServiceCatalogState {
            portfolios: view
                .portfolios
                .into_iter()
                .map(|(k, v)| (k, PortfolioDetail::from(v)))
                .collect(),
            products: view
                .products
                .into_iter()
                .map(|(k, v)| (k, ProductDetail::from(v)))
                .collect(),
            idempotency_tokens: view.idempotency_tokens,
        }
    }
}

impl From<PortfolioDetailView> for PortfolioDetail {
    fn from(v: PortfolioDetailView) -> Self {
        PortfolioDetail {
            id: v.id,
            arn: v.arn,
            display_name: v.display_name,
            description: v.description,
            created_time: v.created_time,
            provider_name: v.provider_name,
            tags: v
                .tags
                .into_iter()
                .map(|t| PortfolioTag {
                    key: t.key,
                    value: t.value,
                })
                .collect(),
        }
    }
}

impl StatefulService for ServiceCatalogService {
    type StateView = ServiceCatalogStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        ServiceCatalogStateView::from(&*guard)
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
            *guard = ServiceCatalogState::from(view);
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
            for (k, v) in view.portfolios {
                guard.portfolios.insert(k, PortfolioDetail::from(v));
            }
            for (k, v) in view.products {
                guard.products.insert(k, ProductDetail::from(v));
            }
            for (k, v) in view.idempotency_tokens {
                guard.idempotency_tokens.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
