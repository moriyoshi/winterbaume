//! Serde-compatible view types for Signer state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::SignerService;
use crate::state::SignerState;
use crate::types::{ProfilePermission, SigningJob, SigningProfile};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SignerStateView {
    #[serde(default)]
    pub profiles: HashMap<String, SigningProfileView>,
    #[serde(default)]
    pub jobs: HashMap<String, SigningJobView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SigningProfileView {
    pub profile_name: String,
    pub profile_version: String,
    pub profile_version_arn: String,
    pub platform_id: String,
    pub status: String,
    pub arn: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub permissions: Vec<ProfilePermission>,
    #[serde(default)]
    pub revision_id: String,
    #[serde(default)]
    pub signature_validity_period: Option<serde_json::Value>,
    #[serde(default)]
    pub signing_material_certificate_arn: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SigningJobView {
    pub job_id: String,
    pub job_arn: String,
    pub job_owner: String,
    pub profile_name: String,
    pub profile_version: String,
    pub platform_id: String,
    pub status: String,
    pub created_at: f64,
    pub is_revoked: bool,
    #[serde(default)]
    pub revocation_reason: Option<String>,
    #[serde(default)]
    pub revoked_at: Option<f64>,
}

impl From<&SignerState> for SignerStateView {
    fn from(state: &SignerState) -> Self {
        SignerStateView {
            profiles: state
                .profiles
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        SigningProfileView {
                            profile_name: v.profile_name.clone(),
                            profile_version: v.profile_version.clone(),
                            profile_version_arn: v.profile_version_arn.clone(),
                            platform_id: v.platform_id.clone(),
                            status: v.status.clone(),
                            arn: v.arn.clone(),
                            tags: v.tags.clone(),
                            permissions: v.permissions.clone(),
                            revision_id: v.revision_id.clone(),
                            signature_validity_period: v.signature_validity_period.clone(),
                            signing_material_certificate_arn: v
                                .signing_material_certificate_arn
                                .clone(),
                        },
                    )
                })
                .collect(),
            jobs: state
                .jobs
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        SigningJobView {
                            job_id: v.job_id.clone(),
                            job_arn: v.job_arn.clone(),
                            job_owner: v.job_owner.clone(),
                            profile_name: v.profile_name.clone(),
                            profile_version: v.profile_version.clone(),
                            platform_id: v.platform_id.clone(),
                            status: v.status.clone(),
                            created_at: v.created_at,
                            is_revoked: v.is_revoked,
                            revocation_reason: v.revocation_reason.clone(),
                            revoked_at: v.revoked_at,
                        },
                    )
                })
                .collect(),
        }
    }
}

impl From<SignerStateView> for SignerState {
    fn from(view: SignerStateView) -> Self {
        SignerState {
            profiles: view
                .profiles
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        SigningProfile {
                            profile_name: v.profile_name,
                            profile_version: v.profile_version,
                            profile_version_arn: v.profile_version_arn,
                            platform_id: v.platform_id,
                            status: v.status,
                            arn: v.arn,
                            tags: v.tags,
                            permissions: v.permissions,
                            revision_id: v.revision_id,
                            signature_validity_period: v.signature_validity_period,
                            signing_material_certificate_arn: v.signing_material_certificate_arn,
                        },
                    )
                })
                .collect(),
            jobs: view
                .jobs
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        SigningJob {
                            job_id: v.job_id,
                            job_arn: v.job_arn,
                            job_owner: v.job_owner,
                            profile_name: v.profile_name,
                            profile_version: v.profile_version,
                            platform_id: v.platform_id,
                            status: v.status,
                            created_at: v.created_at,
                            is_revoked: v.is_revoked,
                            revocation_reason: v.revocation_reason,
                            revoked_at: v.revoked_at,
                        },
                    )
                })
                .collect(),
        }
    }
}

impl StatefulService for SignerService {
    type StateView = SignerStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        SignerStateView::from(&*guard)
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
            *guard = SignerState::from(view);
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
            for (k, v) in view.profiles {
                guard.profiles.insert(
                    k,
                    SigningProfile {
                        profile_name: v.profile_name,
                        profile_version: v.profile_version,
                        profile_version_arn: v.profile_version_arn,
                        platform_id: v.platform_id,
                        status: v.status,
                        arn: v.arn,
                        tags: v.tags,
                        permissions: v.permissions,
                        revision_id: v.revision_id,
                        signature_validity_period: v.signature_validity_period,
                        signing_material_certificate_arn: v.signing_material_certificate_arn,
                    },
                );
            }
            for (k, v) in view.jobs {
                guard.jobs.insert(
                    k,
                    SigningJob {
                        job_id: v.job_id,
                        job_arn: v.job_arn,
                        job_owner: v.job_owner,
                        profile_name: v.profile_name,
                        profile_version: v.profile_version,
                        platform_id: v.platform_id,
                        status: v.status,
                        created_at: v.created_at,
                        is_revoked: v.is_revoked,
                        revocation_reason: v.revocation_reason,
                        revoked_at: v.revoked_at,
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
