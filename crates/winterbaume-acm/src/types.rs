use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Tag {
    pub key: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DomainValidationOption {
    pub domain_name: String,
    pub validation_domain: String,
    pub validation_status: String,
}

#[derive(Debug, Clone)]
pub struct Certificate {
    pub arn: String,
    pub domain_name: String,
    pub status: String,
    pub subject_alternative_names: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub certificate_type: String,
    pub tags: Vec<Tag>,
    pub issuer: String,
    pub key_algorithm: String,
    pub renewal_eligibility: String,
    pub options: CertificateOptions,
    pub domain_validation_options: Vec<DomainValidationOption>,
    pub not_before: Option<DateTime<Utc>>,
    pub not_after: Option<DateTime<Utc>>,
    pub certificate_authority_arn: Option<String>,
    /// PEM-encoded certificate body
    pub certificate_pem: Option<String>,
    /// PEM-encoded certificate chain
    pub certificate_chain: Option<String>,
    /// PEM-encoded private key (for imported or PCA certs)
    pub private_key: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CertificateOptions {
    pub certificate_transparency_logging_preference: String,
}

#[derive(Debug, Clone)]
pub struct ExpiryEventsConfiguration {
    pub days_before_expiry: i32,
}

impl Default for ExpiryEventsConfiguration {
    fn default() -> Self {
        Self {
            days_before_expiry: 45,
        }
    }
}
