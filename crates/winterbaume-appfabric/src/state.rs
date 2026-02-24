use std::collections::HashMap;

use thiserror::Error;
use uuid::Uuid;

use crate::types::*;

#[derive(Debug, Default)]
pub struct AppFabricState {
    /// AppBundles keyed by their identifier (the trailing portion of the ARN).
    pub app_bundles: HashMap<String, AppBundle>,
}

#[derive(Debug, Error)]
pub enum AppFabricError {
    #[error("AppBundle {identifier} not found")]
    AppBundleNotFound { identifier: String },

    #[error("Resource {arn} not found")]
    ResourceNotFound { arn: String },

    #[error("{message}")]
    Validation { message: String },
}

impl AppFabricState {
    /// Resolve an identifier (which can be a bare id or a full ARN) to a bundle id.
    pub fn resolve_id(identifier: &str) -> String {
        if identifier.starts_with("arn:") {
            identifier
                .rsplit_once('/')
                .map(|(_, id)| id)
                .unwrap_or(identifier)
                .to_string()
        } else {
            identifier.to_string()
        }
    }

    pub fn create_app_bundle(
        &mut self,
        account_id: &str,
        region: &str,
        customer_managed_key_arn: Option<String>,
        tags: HashMap<String, String>,
    ) -> &AppBundle {
        let id = Uuid::new_v4().simple().to_string();
        let arn = format!(
            "arn:aws:appfabric:{}:{}:appbundle/{}",
            region, account_id, id
        );
        let bundle = AppBundle {
            arn,
            customer_managed_key_arn,
            tags,
        };
        self.app_bundles.insert(id.clone(), bundle);
        self.app_bundles.get(&id).unwrap()
    }

    pub fn get_app_bundle(&self, identifier: &str) -> Result<&AppBundle, AppFabricError> {
        let id = Self::resolve_id(identifier);
        self.app_bundles
            .get(&id)
            .ok_or(AppFabricError::AppBundleNotFound { identifier: id })
    }

    pub fn delete_app_bundle(&mut self, identifier: &str) -> Result<(), AppFabricError> {
        let id = Self::resolve_id(identifier);
        self.app_bundles
            .remove(&id)
            .ok_or(AppFabricError::AppBundleNotFound { identifier: id })?;
        Ok(())
    }

    pub fn list_app_bundles(&self) -> Vec<&AppBundle> {
        let mut all: Vec<&AppBundle> = self.app_bundles.values().collect();
        all.sort_by(|a, b| a.arn.cmp(&b.arn));
        all
    }

    pub fn bundle_by_arn_mut(&mut self, arn: &str) -> Result<&mut AppBundle, AppFabricError> {
        self.app_bundles
            .values_mut()
            .find(|b| b.arn == arn)
            .ok_or_else(|| AppFabricError::ResourceNotFound {
                arn: arn.to_string(),
            })
    }

    pub fn bundle_by_arn(&self, arn: &str) -> Result<&AppBundle, AppFabricError> {
        self.app_bundles
            .values()
            .find(|b| b.arn == arn)
            .ok_or_else(|| AppFabricError::ResourceNotFound {
                arn: arn.to_string(),
            })
    }

    pub fn tag_resource(
        &mut self,
        arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), AppFabricError> {
        let bundle = self.bundle_by_arn_mut(arn)?;
        for (k, v) in tags {
            bundle.tags.insert(k, v);
        }
        Ok(())
    }

    pub fn untag_resource(&mut self, arn: &str, keys: &[String]) -> Result<(), AppFabricError> {
        let bundle = self.bundle_by_arn_mut(arn)?;
        for k in keys {
            bundle.tags.remove(k);
        }
        Ok(())
    }

    pub fn list_tags(&self, arn: &str) -> Result<HashMap<String, String>, AppFabricError> {
        Ok(self.bundle_by_arn(arn)?.tags.clone())
    }
}
