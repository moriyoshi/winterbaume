//! Serde-compatible view types for ACM state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::AcmService;
use crate::state::AcmState;
use crate::types::{
    Certificate, CertificateOptions, DomainValidationOption, ExpiryEventsConfiguration, Tag,
};

/// Serializable view of the entire ACM state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AcmStateView {
    /// Certificates keyed by ARN.
    #[serde(default)]
    pub certificates: HashMap<String, CertificateView>,
    /// Account-level expiry events configuration (singleton).
    #[serde(default)]
    pub account_configuration: ExpiryEventsConfigurationView,
    /// Idempotency token mapping used to deduplicate `RequestCertificate` calls.
    #[serde(default)]
    pub idempotency_tokens: HashMap<String, String>,
}

/// Serializable view of the account-level expiry events configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpiryEventsConfigurationView {
    pub days_before_expiry: i32,
}

impl Default for ExpiryEventsConfigurationView {
    fn default() -> Self {
        Self {
            days_before_expiry: 45,
        }
    }
}

/// Serializable view of a single ACM certificate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateView {
    /// Certificate ARN.
    pub arn: String,
    /// Primary domain name.
    pub domain_name: String,
    /// Certificate status (e.g. ISSUED, PENDING_VALIDATION).
    pub status: String,
    /// Subject alternative names (SANs).
    #[serde(default)]
    pub subject_alternative_names: Vec<String>,
    /// Creation timestamp in RFC 3339 format.
    pub created_at: Option<String>,
    /// Certificate type (AMAZON_ISSUED, IMPORTED, PRIVATE).
    pub certificate_type: String,
    /// Resource tags.
    #[serde(default)]
    pub tags: Vec<TagView>,
    /// Issuer string.
    pub issuer: String,
    /// Key algorithm (e.g. RSA_2048).
    pub key_algorithm: String,
    /// Renewal eligibility.
    pub renewal_eligibility: String,
    /// Certificate options (transparency logging preference).
    pub options: Option<CertificateOptionsView>,
    /// Domain validation records.
    #[serde(default)]
    pub domain_validation_options: Vec<DomainValidationOptionView>,
    /// NotBefore timestamp in RFC 3339 format.
    pub not_before: Option<String>,
    /// NotAfter timestamp in RFC 3339 format.
    pub not_after: Option<String>,
    /// Private CA ARN if issued by PCA.
    pub certificate_authority_arn: Option<String>,
    /// PEM-encoded certificate body (populated for imported certificates).
    pub certificate_pem: Option<String>,
    /// PEM-encoded certificate chain.
    pub certificate_chain: Option<String>,
    /// PEM-encoded private key (for imported or PCA certs).
    pub private_key: Option<String>,
}

/// Serializable view of a resource tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagView {
    pub key: String,
    pub value: Option<String>,
}

/// Serializable view of certificate options (maps to Terraform `options` block).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateOptionsView {
    pub certificate_transparency_logging_preference: String,
}

/// Serializable view of a domain validation option / record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainValidationOptionView {
    pub domain_name: String,
    pub validation_domain: String,
    pub validation_status: String,
}

// --- From internal types to view types ---

impl From<&AcmState> for AcmStateView {
    fn from(state: &AcmState) -> Self {
        AcmStateView {
            certificates: state
                .certificates
                .iter()
                .map(|(k, v)| (k.clone(), CertificateView::from(v)))
                .collect(),
            account_configuration: ExpiryEventsConfigurationView::from(
                &state.account_configuration,
            ),
            idempotency_tokens: state.idempotency_tokens.clone(),
        }
    }
}

impl From<&ExpiryEventsConfiguration> for ExpiryEventsConfigurationView {
    fn from(c: &ExpiryEventsConfiguration) -> Self {
        ExpiryEventsConfigurationView {
            days_before_expiry: c.days_before_expiry,
        }
    }
}

impl From<ExpiryEventsConfigurationView> for ExpiryEventsConfiguration {
    fn from(view: ExpiryEventsConfigurationView) -> Self {
        ExpiryEventsConfiguration {
            days_before_expiry: view.days_before_expiry,
        }
    }
}

impl From<&Certificate> for CertificateView {
    fn from(cert: &Certificate) -> Self {
        CertificateView {
            arn: cert.arn.clone(),
            domain_name: cert.domain_name.clone(),
            status: cert.status.clone(),
            subject_alternative_names: cert.subject_alternative_names.clone(),
            created_at: Some(cert.created_at.to_rfc3339()),
            certificate_type: cert.certificate_type.clone(),
            tags: cert.tags.iter().map(TagView::from).collect(),
            issuer: cert.issuer.clone(),
            key_algorithm: cert.key_algorithm.clone(),
            renewal_eligibility: cert.renewal_eligibility.clone(),
            options: Some(CertificateOptionsView {
                certificate_transparency_logging_preference: cert
                    .options
                    .certificate_transparency_logging_preference
                    .clone(),
            }),
            domain_validation_options: cert
                .domain_validation_options
                .iter()
                .map(DomainValidationOptionView::from)
                .collect(),
            not_before: cert.not_before.map(|dt| dt.to_rfc3339()),
            not_after: cert.not_after.map(|dt| dt.to_rfc3339()),
            certificate_authority_arn: cert.certificate_authority_arn.clone(),
            certificate_pem: cert.certificate_pem.clone(),
            certificate_chain: cert.certificate_chain.clone(),
            private_key: cert.private_key.clone(),
        }
    }
}

impl From<&Tag> for TagView {
    fn from(tag: &Tag) -> Self {
        TagView {
            key: tag.key.clone(),
            value: tag.value.clone(),
        }
    }
}

impl From<&DomainValidationOption> for DomainValidationOptionView {
    fn from(dvo: &DomainValidationOption) -> Self {
        DomainValidationOptionView {
            domain_name: dvo.domain_name.clone(),
            validation_domain: dvo.validation_domain.clone(),
            validation_status: dvo.validation_status.clone(),
        }
    }
}

// --- From view types to internal types ---

impl From<AcmStateView> for AcmState {
    fn from(view: AcmStateView) -> Self {
        AcmState {
            certificates: view
                .certificates
                .into_iter()
                .map(|(k, v)| (k, Certificate::from(v)))
                .collect(),
            account_configuration: ExpiryEventsConfiguration::from(view.account_configuration),
            idempotency_tokens: view.idempotency_tokens,
        }
    }
}

impl From<CertificateView> for Certificate {
    fn from(view: CertificateView) -> Self {
        use chrono::{DateTime, Utc};

        let created_at = view
            .created_at
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);

        let not_before = view
            .not_before
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));

        let not_after = view
            .not_after
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));

        Certificate {
            arn: view.arn,
            domain_name: view.domain_name,
            status: view.status,
            subject_alternative_names: view.subject_alternative_names,
            created_at,
            certificate_type: view.certificate_type,
            tags: view.tags.into_iter().map(Tag::from).collect(),
            issuer: view.issuer,
            key_algorithm: view.key_algorithm,
            renewal_eligibility: view.renewal_eligibility,
            options: CertificateOptions {
                certificate_transparency_logging_preference: view
                    .options
                    .map(|o| o.certificate_transparency_logging_preference)
                    .unwrap_or_else(|| "ENABLED".to_string()),
            },
            domain_validation_options: view
                .domain_validation_options
                .into_iter()
                .map(DomainValidationOption::from)
                .collect(),
            not_before,
            not_after,
            certificate_authority_arn: view.certificate_authority_arn,
            certificate_pem: view.certificate_pem,
            certificate_chain: view.certificate_chain,
            private_key: view.private_key,
        }
    }
}

impl From<TagView> for Tag {
    fn from(view: TagView) -> Self {
        Tag {
            key: view.key,
            value: view.value,
        }
    }
}

impl From<DomainValidationOptionView> for DomainValidationOption {
    fn from(view: DomainValidationOptionView) -> Self {
        DomainValidationOption {
            domain_name: view.domain_name,
            validation_domain: view.validation_domain,
            validation_status: view.validation_status,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for AcmService {
    type StateView = AcmStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        AcmStateView::from(&*guard)
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
            *guard = AcmState::from(view);
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
            for (arn, cert_view) in view.certificates {
                guard.certificates.insert(arn, Certificate::from(cert_view));
            }
            // Singleton: overwrite with the merged-in configuration.
            guard.account_configuration =
                ExpiryEventsConfiguration::from(view.account_configuration);
            for (key, arn) in view.idempotency_tokens {
                guard.idempotency_tokens.insert(key, arn);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
