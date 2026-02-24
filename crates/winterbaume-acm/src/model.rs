//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-acm

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddTagsToCertificateRequest {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    pub certificate_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCertificateRequest {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    pub certificate_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCertificateRequest {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    pub certificate_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCertificateResponse {
    #[serde(rename = "Certificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<CertificateDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CertificateDetail {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority_arn: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "DomainValidationOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_validation_options: Option<Vec<DomainValidation>>,
    #[serde(rename = "ExtendedKeyUsages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_key_usages: Option<Vec<ExtendedKeyUsage>>,
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "ImportedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_at: Option<f64>,
    #[serde(rename = "InUseBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_use_by: Option<Vec<String>>,
    #[serde(rename = "IssuedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<f64>,
    #[serde(rename = "Issuer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(rename = "KeyAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_algorithm: Option<String>,
    #[serde(rename = "KeyUsages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_usages: Option<Vec<KeyUsage>>,
    #[serde(rename = "ManagedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[serde(rename = "NotAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_after: Option<f64>,
    #[serde(rename = "NotBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before: Option<f64>,
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<CertificateOptions>,
    #[serde(rename = "RenewalEligibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_eligibility: Option<String>,
    #[serde(rename = "RenewalSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_summary: Option<RenewalSummary>,
    #[serde(rename = "RevocationReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_reason: Option<String>,
    #[serde(rename = "RevokedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked_at: Option<f64>,
    #[serde(rename = "Serial")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    #[serde(rename = "SignatureAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_algorithm: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Subject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(rename = "SubjectAlternativeNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_alternative_names: Option<Vec<String>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainValidation {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "HttpRedirect")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_redirect: Option<HttpRedirect>,
    #[serde(rename = "ResourceRecord")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_record: Option<ResourceRecord>,
    #[serde(rename = "ValidationDomain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_domain: Option<String>,
    #[serde(rename = "ValidationEmails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_emails: Option<Vec<String>>,
    #[serde(rename = "ValidationMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_method: Option<String>,
    #[serde(rename = "ValidationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpRedirect {
    #[serde(rename = "RedirectFrom")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_from: Option<String>,
    #[serde(rename = "RedirectTo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_to: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceRecord {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExtendedKeyUsage {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_i_d: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KeyUsage {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CertificateOptions {
    #[serde(rename = "CertificateTransparencyLoggingPreference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_transparency_logging_preference: Option<String>,
    #[serde(rename = "Export")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RenewalSummary {
    #[serde(rename = "DomainValidationOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_validation_options: Option<Vec<DomainValidation>>,
    #[serde(rename = "RenewalStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_status: Option<String>,
    #[serde(rename = "RenewalStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_status_reason: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportCertificateRequest {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    pub certificate_arn: String,
    #[serde(rename = "Passphrase")]
    #[serde(default)]
    pub passphrase: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportCertificateResponse {
    #[serde(rename = "Certificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[serde(rename = "CertificateChain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<String>,
    #[serde(rename = "PrivateKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccountConfigurationResponse {
    #[serde(rename = "ExpiryEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_events: Option<ExpiryEventsConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExpiryEventsConfiguration {
    #[serde(rename = "DaysBeforeExpiry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_before_expiry: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCertificateRequest {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    pub certificate_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCertificateResponse {
    #[serde(rename = "Certificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,
    #[serde(rename = "CertificateChain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportCertificateRequest {
    #[serde(rename = "Certificate")]
    #[serde(default)]
    pub certificate: String,
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "CertificateChain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<String>,
    #[serde(rename = "PrivateKey")]
    #[serde(default)]
    pub private_key: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportCertificateResponse {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCertificatesRequest {
    #[serde(rename = "CertificateStatuses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_statuses: Option<Vec<String>>,
    #[serde(rename = "Includes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Filters>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "SortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Filters {
    #[serde(rename = "exportOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_option: Option<String>,
    #[serde(rename = "extendedKeyUsage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_key_usage: Option<Vec<String>>,
    #[serde(rename = "keyTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_types: Option<Vec<String>>,
    #[serde(rename = "keyUsage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_usage: Option<Vec<String>>,
    #[serde(rename = "managedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCertificatesResponse {
    #[serde(rename = "CertificateSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_summary_list: Option<Vec<CertificateSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CertificateSummary {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "ExportOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_option: Option<String>,
    #[serde(rename = "Exported")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exported: Option<bool>,
    #[serde(rename = "ExtendedKeyUsages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_key_usages: Option<Vec<String>>,
    #[serde(rename = "HasAdditionalSubjectAlternativeNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_additional_subject_alternative_names: Option<bool>,
    #[serde(rename = "ImportedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_at: Option<f64>,
    #[serde(rename = "InUse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_use: Option<bool>,
    #[serde(rename = "IssuedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<f64>,
    #[serde(rename = "KeyAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_algorithm: Option<String>,
    #[serde(rename = "KeyUsages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_usages: Option<Vec<String>>,
    #[serde(rename = "ManagedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[serde(rename = "NotAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_after: Option<f64>,
    #[serde(rename = "NotBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before: Option<f64>,
    #[serde(rename = "RenewalEligibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_eligibility: Option<String>,
    #[serde(rename = "RevokedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked_at: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "SubjectAlternativeNameSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_alternative_name_summaries: Option<Vec<String>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForCertificateRequest {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    pub certificate_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForCertificateResponse {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAccountConfigurationRequest {
    #[serde(rename = "ExpiryEvents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_events: Option<ExpiryEventsConfiguration>,
    #[serde(rename = "IdempotencyToken")]
    #[serde(default)]
    pub idempotency_token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveTagsFromCertificateRequest {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    pub certificate_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RenewCertificateRequest {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    pub certificate_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RequestCertificateRequest {
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority_arn: Option<String>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "DomainValidationOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_validation_options: Option<Vec<DomainValidationOption>>,
    #[serde(rename = "IdempotencyToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_token: Option<String>,
    #[serde(rename = "KeyAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_algorithm: Option<String>,
    #[serde(rename = "ManagedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<CertificateOptions>,
    #[serde(rename = "SubjectAlternativeNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_alternative_names: Option<Vec<String>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "ValidationMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_method: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainValidationOption {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "ValidationDomain")]
    #[serde(default)]
    pub validation_domain: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RequestCertificateResponse {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResendValidationEmailRequest {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    pub certificate_arn: String,
    #[serde(rename = "Domain")]
    #[serde(default)]
    pub domain: String,
    #[serde(rename = "ValidationDomain")]
    #[serde(default)]
    pub validation_domain: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RevokeCertificateRequest {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    pub certificate_arn: String,
    #[serde(rename = "RevocationReason")]
    #[serde(default)]
    pub revocation_reason: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RevokeCertificateResponse {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchCertificatesRequest {
    #[serde(rename = "FilterStatement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_statement: Option<CertificateFilterStatement>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "SortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CertificateFilterStatement {
    #[serde(rename = "And")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<CertificateFilterStatement>>,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<CertificateFilter>,
    #[serde(rename = "Not")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not: Option<Box<CertificateFilterStatement>>,
    #[serde(rename = "Or")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or: Option<Vec<CertificateFilterStatement>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CertificateFilter {
    #[serde(rename = "AcmCertificateMetadataFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acm_certificate_metadata_filter: Option<AcmCertificateMetadataFilter>,
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "X509AttributeFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x509_attribute_filter: Option<X509AttributeFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcmCertificateMetadataFilter {
    #[serde(rename = "ExportOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_option: Option<String>,
    #[serde(rename = "Exported")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exported: Option<bool>,
    #[serde(rename = "InUse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_use: Option<bool>,
    #[serde(rename = "ManagedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[serde(rename = "RenewalStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_status: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "ValidationMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_method: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct X509AttributeFilter {
    #[serde(rename = "ExtendedKeyUsage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_key_usage: Option<String>,
    #[serde(rename = "KeyAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_algorithm: Option<String>,
    #[serde(rename = "KeyUsage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_usage: Option<String>,
    #[serde(rename = "NotAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_after: Option<TimestampRange>,
    #[serde(rename = "NotBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before: Option<TimestampRange>,
    #[serde(rename = "SerialNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(rename = "Subject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<SubjectFilter>,
    #[serde(rename = "SubjectAlternativeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_alternative_name: Option<SubjectAlternativeNameFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TimestampRange {
    #[serde(rename = "End")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,
    #[serde(rename = "Start")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubjectFilter {
    #[serde(rename = "CommonName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_name: Option<CommonNameFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CommonNameFilter {
    #[serde(rename = "ComparisonOperator")]
    #[serde(default)]
    pub comparison_operator: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubjectAlternativeNameFilter {
    #[serde(rename = "DnsName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<DnsNameFilter>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DnsNameFilter {
    #[serde(rename = "ComparisonOperator")]
    #[serde(default)]
    pub comparison_operator: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SearchCertificatesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Results")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<CertificateSearchResult>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CertificateSearchResult {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(rename = "CertificateMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_metadata: Option<CertificateMetadata>,
    #[serde(rename = "X509Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x509_attributes: Option<X509Attributes>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CertificateMetadata {
    #[serde(rename = "AcmCertificateMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acm_certificate_metadata: Option<AcmCertificateMetadata>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcmCertificateMetadata {
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "ExportOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_option: Option<String>,
    #[serde(rename = "Exported")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exported: Option<bool>,
    #[serde(rename = "ImportedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_at: Option<f64>,
    #[serde(rename = "InUse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_use: Option<bool>,
    #[serde(rename = "IssuedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<f64>,
    #[serde(rename = "ManagedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[serde(rename = "RenewalEligibility")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_eligibility: Option<String>,
    #[serde(rename = "RenewalStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_status: Option<String>,
    #[serde(rename = "RevokedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked_at: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "ValidationMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_method: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct X509Attributes {
    #[serde(rename = "ExtendedKeyUsages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_key_usages: Option<Vec<String>>,
    #[serde(rename = "Issuer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<DistinguishedName>,
    #[serde(rename = "KeyAlgorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_algorithm: Option<String>,
    #[serde(rename = "KeyUsages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_usages: Option<Vec<String>>,
    #[serde(rename = "NotAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_after: Option<f64>,
    #[serde(rename = "NotBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before: Option<f64>,
    #[serde(rename = "SerialNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(rename = "Subject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<DistinguishedName>,
    #[serde(rename = "SubjectAlternativeNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_alternative_names: Option<Vec<GeneralName>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DistinguishedName {
    #[serde(rename = "CommonName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_name: Option<String>,
    #[serde(rename = "Country")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "CustomAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_attributes: Option<Vec<CustomAttribute>>,
    #[serde(rename = "DistinguishedNameQualifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distinguished_name_qualifier: Option<String>,
    #[serde(rename = "DomainComponents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_components: Option<Vec<String>>,
    #[serde(rename = "GenerationQualifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation_qualifier: Option<String>,
    #[serde(rename = "GivenName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[serde(rename = "Initials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initials: Option<String>,
    #[serde(rename = "Locality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    #[serde(rename = "Organization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(rename = "OrganizationalUnit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit: Option<String>,
    #[serde(rename = "Pseudonym")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pseudonym: Option<String>,
    #[serde(rename = "SerialNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "Surname")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomAttribute {
    #[serde(rename = "ObjectIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_identifier: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeneralName {
    #[serde(rename = "DirectoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_name: Option<DistinguishedName>,
    #[serde(rename = "DnsName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
    #[serde(rename = "IpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "OtherName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_name: Option<OtherName>,
    #[serde(rename = "RegisteredId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_id: Option<String>,
    #[serde(rename = "Rfc822Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rfc822_name: Option<String>,
    #[serde(rename = "UniformResourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uniform_resource_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OtherName {
    #[serde(rename = "ObjectIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_identifier: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCertificateOptionsRequest {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    pub certificate_arn: String,
    #[serde(rename = "Options")]
    #[serde(default)]
    pub options: CertificateOptions,
}
