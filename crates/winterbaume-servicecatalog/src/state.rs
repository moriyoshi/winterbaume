use std::collections::HashMap;

use chrono::Utc;
use thiserror::Error;
use uuid::Uuid;

use crate::types::*;

#[derive(Debug, Default)]
pub struct ServiceCatalogState {
    pub portfolios: HashMap<String, PortfolioDetail>,
    /// Products keyed by product ID.
    pub products: HashMap<String, ProductDetail>,
    /// Maps idempotency token -> portfolio id
    pub idempotency_tokens: HashMap<String, String>,
}

#[derive(Debug, Error)]
pub enum ServiceCatalogError {
    #[error("Portfolio {id} not found.")]
    ResourceNotFound { id: String },
}

impl ServiceCatalogState {
    pub fn create_portfolio(
        &mut self,
        display_name: &str,
        description: &str,
        provider_name: &str,
        tags: Vec<PortfolioTag>,
        idempotency_token: Option<&str>,
        account_id: &str,
        region: &str,
    ) -> Result<PortfolioDetail, ServiceCatalogError> {
        // Handle idempotency: if token was seen before, return existing portfolio
        if let Some(token) = idempotency_token {
            if let Some(existing_id) = self.idempotency_tokens.get(token) {
                if let Some(existing) = self.portfolios.get(existing_id) {
                    return Ok(existing.clone());
                }
            }
        }

        let id = format!(
            "port-{}",
            &Uuid::new_v4().to_string().replace('-', "")[..12]
        );
        let arn = format!("arn:aws:catalog:{region}:{account_id}:portfolio/{id}");

        let portfolio = PortfolioDetail {
            id: id.clone(),
            arn,
            display_name: display_name.to_string(),
            description: description.to_string(),
            created_time: Utc::now(),
            provider_name: provider_name.to_string(),
            tags,
        };

        if let Some(token) = idempotency_token {
            self.idempotency_tokens
                .insert(token.to_string(), id.clone());
        }

        self.portfolios.insert(id.clone(), portfolio.clone());
        Ok(portfolio)
    }

    pub fn describe_portfolio(&self, id: &str) -> Result<&PortfolioDetail, ServiceCatalogError> {
        self.portfolios
            .get(id)
            .ok_or_else(|| ServiceCatalogError::ResourceNotFound { id: id.to_string() })
    }

    pub fn delete_portfolio(&mut self, id: &str) -> Result<(), ServiceCatalogError> {
        if self.portfolios.remove(id).is_none() {
            return Err(ServiceCatalogError::ResourceNotFound { id: id.to_string() });
        }
        Ok(())
    }

    pub fn list_portfolios(&self) -> Vec<&PortfolioDetail> {
        self.portfolios.values().collect()
    }
}
