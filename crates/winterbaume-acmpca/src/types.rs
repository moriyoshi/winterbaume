use std::collections::HashMap;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct CertificateAuthority {
    pub arn: String,
    pub owner_account: String,
    pub ca_type: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub last_state_change_at: Option<DateTime<Utc>>,
    pub not_before: Option<DateTime<Utc>>,
    pub not_after: Option<DateTime<Utc>>,
    pub ca_config: CaConfiguration,
    pub key_storage_security_standard: String,
    pub revocation_configuration: Option<RevocationConfiguration>,
    pub tags: Vec<Tag>,
    /// PEM-encoded private key (kept in memory for signing)
    pub private_key_pem: String,
    /// PEM-encoded CSR
    pub csr_pem: String,
    /// PEM-encoded CA certificate (set after import)
    pub certificate_pem: Option<String>,
    /// PEM-encoded certificate chain (set after import)
    pub certificate_chain_pem: Option<String>,
    /// Issued certificates, keyed by certificate ARN
    pub issued_certificates: HashMap<String, IssuedCertificate>,
    /// Revoked certificates, keyed by serial number
    pub revoked_certificates: HashMap<String, RevokedCertificate>,
    /// Resource policy
    pub policy: Option<String>,
    /// Permissions granted on this CA, keyed by principal
    pub permissions: HashMap<String, CaPermission>,
    /// Audit reports, keyed by audit report ID
    pub audit_reports: HashMap<String, CaAuditReport>,
}

#[derive(Debug, Clone)]
pub struct CaPermission {
    pub principal: String,
    pub actions: Vec<String>,
    pub source_account: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct CaAuditReport {
    pub audit_report_id: String,
    pub s3_bucket_name: String,
    pub s3_key: String,
    pub audit_report_response_format: String,
    pub created_at: DateTime<Utc>,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct CaConfiguration {
    pub key_algorithm: String,
    pub signing_algorithm: String,
    pub subject: CaSubject,
}

#[derive(Debug, Clone)]
pub struct CaSubject {
    pub common_name: Option<String>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub organization: Option<String>,
    pub organizational_unit: Option<String>,
    pub locality: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Tag {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct IssuedCertificate {
    pub arn: String,
    pub certificate_pem: String,
    /// Whether this certificate is the CA's own certificate (root CA cert)
    pub is_ca_cert: bool,
}

#[derive(Debug, Clone)]
pub struct RevokedCertificate {
    pub serial_number: String,
    pub revocation_reason: String,
    pub revocation_time: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct RevocationConfiguration {
    pub crl_configuration: Option<CrlConfiguration>,
}

#[derive(Debug, Clone)]
pub struct CrlConfiguration {
    pub enabled: bool,
    pub s3_object_acl: String,
}
