//! Serde-compatible view types for ACM PCA state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::AcmPcaService;
use crate::state::AcmPcaState;

/// Serializable view of the entire ACM PCA state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AcmPcaStateView {
    /// Certificate authorities keyed by ARN.
    #[serde(default)]
    pub certificate_authorities: HashMap<String, CertificateAuthorityView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateAuthorityView {
    pub arn: String,
    pub owner_account: String,
    pub ca_type: String,
    pub status: String,
    pub created_at: String,
    pub last_state_change_at: Option<String>,
    pub not_before: Option<String>,
    pub not_after: Option<String>,
    pub ca_config: CaConfigurationView,
    pub key_storage_security_standard: String,
    pub revocation_configuration: Option<RevocationConfigurationView>,
    #[serde(default)]
    pub tags: Vec<TagView>,
    pub private_key_pem: String,
    pub csr_pem: String,
    pub certificate_pem: Option<String>,
    pub certificate_chain_pem: Option<String>,
    #[serde(default)]
    pub issued_certificates: HashMap<String, IssuedCertificateView>,
    #[serde(default)]
    pub revoked_certificates: HashMap<String, RevokedCertificateView>,
    pub policy: Option<String>,
    #[serde(default)]
    pub permissions: HashMap<String, CaPermissionView>,
    #[serde(default)]
    pub audit_reports: HashMap<String, CaAuditReportView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaPermissionView {
    pub principal: String,
    pub actions: Vec<String>,
    pub source_account: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaAuditReportView {
    pub audit_report_id: String,
    pub s3_bucket_name: String,
    pub s3_key: String,
    pub audit_report_response_format: String,
    pub created_at: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaConfigurationView {
    pub key_algorithm: String,
    pub signing_algorithm: String,
    pub subject: CaSubjectView,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaSubjectView {
    pub common_name: Option<String>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub organization: Option<String>,
    pub organizational_unit: Option<String>,
    pub locality: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagView {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssuedCertificateView {
    pub arn: String,
    pub certificate_pem: String,
    pub is_ca_cert: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokedCertificateView {
    pub serial_number: String,
    pub revocation_reason: String,
    pub revocation_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevocationConfigurationView {
    pub crl_configuration: Option<CrlConfigurationView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrlConfigurationView {
    pub enabled: bool,
    pub s3_object_acl: String,
}

// --- From internal types to view types ---

impl From<&AcmPcaState> for AcmPcaStateView {
    fn from(state: &AcmPcaState) -> Self {
        AcmPcaStateView {
            certificate_authorities: state
                .certificate_authorities
                .iter()
                .map(|(k, v)| (k.clone(), CertificateAuthorityView::from(v)))
                .collect(),
        }
    }
}

impl From<&crate::types::CertificateAuthority> for CertificateAuthorityView {
    fn from(ca: &crate::types::CertificateAuthority) -> Self {
        CertificateAuthorityView {
            arn: ca.arn.clone(),
            owner_account: ca.owner_account.clone(),
            ca_type: ca.ca_type.clone(),
            status: ca.status.clone(),
            created_at: ca.created_at.to_rfc3339(),
            last_state_change_at: ca.last_state_change_at.map(|t| t.to_rfc3339()),
            not_before: ca.not_before.map(|t| t.to_rfc3339()),
            not_after: ca.not_after.map(|t| t.to_rfc3339()),
            ca_config: CaConfigurationView {
                key_algorithm: ca.ca_config.key_algorithm.clone(),
                signing_algorithm: ca.ca_config.signing_algorithm.clone(),
                subject: CaSubjectView {
                    common_name: ca.ca_config.subject.common_name.clone(),
                    country: ca.ca_config.subject.country.clone(),
                    state: ca.ca_config.subject.state.clone(),
                    organization: ca.ca_config.subject.organization.clone(),
                    organizational_unit: ca.ca_config.subject.organizational_unit.clone(),
                    locality: ca.ca_config.subject.locality.clone(),
                },
            },
            key_storage_security_standard: ca.key_storage_security_standard.clone(),
            revocation_configuration: ca.revocation_configuration.as_ref().map(|rc| {
                RevocationConfigurationView {
                    crl_configuration: rc.crl_configuration.as_ref().map(|crl| {
                        CrlConfigurationView {
                            enabled: crl.enabled,
                            s3_object_acl: crl.s3_object_acl.clone(),
                        }
                    }),
                }
            }),
            tags: ca
                .tags
                .iter()
                .map(|t| TagView {
                    key: t.key.clone(),
                    value: t.value.clone(),
                })
                .collect(),
            private_key_pem: ca.private_key_pem.clone(),
            csr_pem: ca.csr_pem.clone(),
            certificate_pem: ca.certificate_pem.clone(),
            certificate_chain_pem: ca.certificate_chain_pem.clone(),
            issued_certificates: ca
                .issued_certificates
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        IssuedCertificateView {
                            arn: v.arn.clone(),
                            certificate_pem: v.certificate_pem.clone(),
                            is_ca_cert: v.is_ca_cert,
                        },
                    )
                })
                .collect(),
            revoked_certificates: ca
                .revoked_certificates
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        RevokedCertificateView {
                            serial_number: v.serial_number.clone(),
                            revocation_reason: v.revocation_reason.clone(),
                            revocation_time: v.revocation_time.to_rfc3339(),
                        },
                    )
                })
                .collect(),
            policy: ca.policy.clone(),
            permissions: ca
                .permissions
                .iter()
                .map(|(k, p)| {
                    (
                        k.clone(),
                        CaPermissionView {
                            principal: p.principal.clone(),
                            actions: p.actions.clone(),
                            source_account: p.source_account.clone(),
                            created_at: p.created_at.to_rfc3339(),
                        },
                    )
                })
                .collect(),
            audit_reports: ca
                .audit_reports
                .iter()
                .map(|(k, r)| {
                    (
                        k.clone(),
                        CaAuditReportView {
                            audit_report_id: r.audit_report_id.clone(),
                            s3_bucket_name: r.s3_bucket_name.clone(),
                            s3_key: r.s3_key.clone(),
                            audit_report_response_format: r.audit_report_response_format.clone(),
                            created_at: r.created_at.to_rfc3339(),
                            status: r.status.clone(),
                        },
                    )
                })
                .collect(),
        }
    }
}

fn parse_datetime(s: &str) -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .unwrap_or_else(|_| chrono::Utc::now())
}

fn parse_datetime_opt(s: Option<&str>) -> Option<chrono::DateTime<chrono::Utc>> {
    s.map(parse_datetime)
}

// --- From view types to internal types ---

impl From<AcmPcaStateView> for AcmPcaState {
    fn from(view: AcmPcaStateView) -> Self {
        AcmPcaState {
            certificate_authorities: view
                .certificate_authorities
                .into_iter()
                .map(|(k, v)| (k, crate::types::CertificateAuthority::from(v)))
                .collect(),
        }
    }
}

impl From<CertificateAuthorityView> for crate::types::CertificateAuthority {
    fn from(v: CertificateAuthorityView) -> Self {
        crate::types::CertificateAuthority {
            arn: v.arn,
            owner_account: v.owner_account,
            ca_type: v.ca_type,
            status: v.status,
            created_at: parse_datetime(&v.created_at),
            last_state_change_at: parse_datetime_opt(v.last_state_change_at.as_deref()),
            not_before: parse_datetime_opt(v.not_before.as_deref()),
            not_after: parse_datetime_opt(v.not_after.as_deref()),
            ca_config: crate::types::CaConfiguration {
                key_algorithm: v.ca_config.key_algorithm,
                signing_algorithm: v.ca_config.signing_algorithm,
                subject: crate::types::CaSubject {
                    common_name: v.ca_config.subject.common_name,
                    country: v.ca_config.subject.country,
                    state: v.ca_config.subject.state,
                    organization: v.ca_config.subject.organization,
                    organizational_unit: v.ca_config.subject.organizational_unit,
                    locality: v.ca_config.subject.locality,
                },
            },
            key_storage_security_standard: v.key_storage_security_standard,
            revocation_configuration: v.revocation_configuration.map(|rc| {
                crate::types::RevocationConfiguration {
                    crl_configuration: rc.crl_configuration.map(|crl| {
                        crate::types::CrlConfiguration {
                            enabled: crl.enabled,
                            s3_object_acl: crl.s3_object_acl,
                        }
                    }),
                }
            }),
            tags: v
                .tags
                .into_iter()
                .map(|t| crate::types::Tag {
                    key: t.key,
                    value: t.value,
                })
                .collect(),
            private_key_pem: v.private_key_pem,
            csr_pem: v.csr_pem,
            certificate_pem: v.certificate_pem,
            certificate_chain_pem: v.certificate_chain_pem,
            issued_certificates: v
                .issued_certificates
                .into_iter()
                .map(|(k, ic)| {
                    (
                        k,
                        crate::types::IssuedCertificate {
                            arn: ic.arn,
                            certificate_pem: ic.certificate_pem,
                            is_ca_cert: ic.is_ca_cert,
                        },
                    )
                })
                .collect(),
            revoked_certificates: v
                .revoked_certificates
                .into_iter()
                .map(|(k, rc)| {
                    (
                        k,
                        crate::types::RevokedCertificate {
                            serial_number: rc.serial_number,
                            revocation_reason: rc.revocation_reason,
                            revocation_time: parse_datetime(&rc.revocation_time),
                        },
                    )
                })
                .collect(),
            policy: v.policy,
            permissions: v
                .permissions
                .into_iter()
                .map(|(k, p)| {
                    (
                        k,
                        crate::types::CaPermission {
                            principal: p.principal,
                            actions: p.actions,
                            source_account: p.source_account,
                            created_at: parse_datetime(&p.created_at),
                        },
                    )
                })
                .collect(),
            audit_reports: v
                .audit_reports
                .into_iter()
                .map(|(k, r)| {
                    (
                        k,
                        crate::types::CaAuditReport {
                            audit_report_id: r.audit_report_id,
                            s3_bucket_name: r.s3_bucket_name,
                            s3_key: r.s3_key,
                            audit_report_response_format: r.audit_report_response_format,
                            created_at: parse_datetime(&r.created_at),
                            status: r.status,
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for AcmPcaService {
    type StateView = AcmPcaStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        AcmPcaStateView::from(&*guard)
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
            *guard = AcmPcaState::from(view);
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
            for (k, v) in view.certificate_authorities {
                guard
                    .certificate_authorities
                    .insert(k, crate::types::CertificateAuthority::from(v));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
