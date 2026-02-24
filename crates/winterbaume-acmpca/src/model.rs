//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-acmpca

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCertificateAuthorityAuditReportRequest {
    #[serde(rename = "AuditReportResponseFormat")]
    #[serde(default)]
    pub audit_report_response_format: String,
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(default)]
    pub certificate_authority_arn: String,
    #[serde(rename = "S3BucketName")]
    #[serde(default)]
    pub s3_bucket_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCertificateAuthorityAuditReportResponse {
    #[serde(rename = "AuditReportId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_report_id: Option<String>,
    #[serde(rename = "S3Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateCertificateAuthorityRequest {
    #[serde(rename = "CertificateAuthorityConfiguration")]
    #[serde(default)]
    pub certificate_authority_configuration: CertificateAuthorityConfiguration,
    #[serde(rename = "CertificateAuthorityType")]
    #[serde(default)]
    pub certificate_authority_type: String,
    #[serde(rename = "IdempotencyToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_token: Option<String>,
    #[serde(rename = "KeyStorageSecurityStandard")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_storage_security_standard: Option<String>,
    #[serde(rename = "RevocationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_configuration: Option<RevocationConfiguration>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "UsageMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CertificateAuthorityConfiguration {
    #[serde(rename = "CsrExtensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csr_extensions: Option<CsrExtensions>,
    #[serde(rename = "KeyAlgorithm")]
    #[serde(default)]
    pub key_algorithm: String,
    #[serde(rename = "SigningAlgorithm")]
    #[serde(default)]
    pub signing_algorithm: String,
    #[serde(rename = "Subject")]
    #[serde(default)]
    pub subject: ASN1Subject,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CsrExtensions {
    #[serde(rename = "KeyUsage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_usage: Option<KeyUsage>,
    #[serde(rename = "SubjectInformationAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_information_access: Option<Vec<AccessDescription>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KeyUsage {
    #[serde(rename = "CRLSign")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_r_l_sign: Option<bool>,
    #[serde(rename = "DataEncipherment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_encipherment: Option<bool>,
    #[serde(rename = "DecipherOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decipher_only: Option<bool>,
    #[serde(rename = "DigitalSignature")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digital_signature: Option<bool>,
    #[serde(rename = "EncipherOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encipher_only: Option<bool>,
    #[serde(rename = "KeyAgreement")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_agreement: Option<bool>,
    #[serde(rename = "KeyCertSign")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_cert_sign: Option<bool>,
    #[serde(rename = "KeyEncipherment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_encipherment: Option<bool>,
    #[serde(rename = "NonRepudiation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_repudiation: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessDescription {
    #[serde(rename = "AccessLocation")]
    #[serde(default)]
    pub access_location: GeneralName,
    #[serde(rename = "AccessMethod")]
    #[serde(default)]
    pub access_method: AccessMethod,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeneralName {
    #[serde(rename = "DirectoryName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_name: Option<ASN1Subject>,
    #[serde(rename = "DnsName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
    #[serde(rename = "EdiPartyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edi_party_name: Option<EdiPartyName>,
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
pub struct ASN1Subject {
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
    pub object_identifier: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EdiPartyName {
    #[serde(rename = "NameAssigner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_assigner: Option<String>,
    #[serde(rename = "PartyName")]
    #[serde(default)]
    pub party_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OtherName {
    #[serde(rename = "TypeId")]
    #[serde(default)]
    pub type_id: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessMethod {
    #[serde(rename = "AccessMethodType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_method_type: Option<String>,
    #[serde(rename = "CustomObjectIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_object_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RevocationConfiguration {
    #[serde(rename = "CrlConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crl_configuration: Option<CrlConfiguration>,
    #[serde(rename = "OcspConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocsp_configuration: Option<OcspConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CrlConfiguration {
    #[serde(rename = "CrlDistributionPointExtensionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crl_distribution_point_extension_configuration:
        Option<CrlDistributionPointExtensionConfiguration>,
    #[serde(rename = "CrlType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crl_type: Option<String>,
    #[serde(rename = "CustomCname")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_cname: Option<String>,
    #[serde(rename = "CustomPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_path: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "ExpirationInDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_in_days: Option<i32>,
    #[serde(rename = "S3BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,
    #[serde(rename = "S3ObjectAcl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object_acl: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CrlDistributionPointExtensionConfiguration {
    #[serde(rename = "OmitExtension")]
    #[serde(default)]
    pub omit_extension: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OcspConfiguration {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "OcspCustomCname")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocsp_custom_cname: Option<String>,
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
pub struct CreateCertificateAuthorityResponse {
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePermissionRequest {
    #[serde(rename = "Actions")]
    #[serde(default)]
    pub actions: Vec<String>,
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(default)]
    pub certificate_authority_arn: String,
    #[serde(rename = "Principal")]
    #[serde(default)]
    pub principal: String,
    #[serde(rename = "SourceAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_account: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteCertificateAuthorityRequest {
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(default)]
    pub certificate_authority_arn: String,
    #[serde(rename = "PermanentDeletionTimeInDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permanent_deletion_time_in_days: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePermissionRequest {
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(default)]
    pub certificate_authority_arn: String,
    #[serde(rename = "Principal")]
    #[serde(default)]
    pub principal: String,
    #[serde(rename = "SourceAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_account: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePolicyRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCertificateAuthorityAuditReportRequest {
    #[serde(rename = "AuditReportId")]
    #[serde(default)]
    pub audit_report_id: String,
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(default)]
    pub certificate_authority_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCertificateAuthorityAuditReportResponse {
    #[serde(rename = "AuditReportStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_report_status: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "S3BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,
    #[serde(rename = "S3Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCertificateAuthorityRequest {
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(default)]
    pub certificate_authority_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeCertificateAuthorityResponse {
    #[serde(rename = "CertificateAuthority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority: Option<CertificateAuthority>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CertificateAuthority {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CertificateAuthorityConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority_configuration: Option<CertificateAuthorityConfiguration>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "KeyStorageSecurityStandard")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_storage_security_standard: Option<String>,
    #[serde(rename = "LastStateChangeAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_state_change_at: Option<f64>,
    #[serde(rename = "NotAfter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_after: Option<f64>,
    #[serde(rename = "NotBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before: Option<f64>,
    #[serde(rename = "OwnerAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account: Option<String>,
    #[serde(rename = "RestorableUntil")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restorable_until: Option<f64>,
    #[serde(rename = "RevocationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_configuration: Option<RevocationConfiguration>,
    #[serde(rename = "Serial")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "UsageMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCertificateAuthorityCertificateRequest {
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(default)]
    pub certificate_authority_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCertificateAuthorityCertificateResponse {
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
pub struct GetCertificateAuthorityCsrRequest {
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(default)]
    pub certificate_authority_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCertificateAuthorityCsrResponse {
    #[serde(rename = "Csr")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csr: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCertificateRequest {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    pub certificate_arn: String,
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(default)]
    pub certificate_authority_arn: String,
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
pub struct GetPolicyRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPolicyResponse {
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportCertificateAuthorityCertificateRequest {
    #[serde(rename = "Certificate")]
    #[serde(default)]
    pub certificate: String,
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(default)]
    pub certificate_authority_arn: String,
    #[serde(rename = "CertificateChain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IssueCertificateRequest {
    #[serde(rename = "ApiPassthrough")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_passthrough: Option<ApiPassthrough>,
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(default)]
    pub certificate_authority_arn: String,
    #[serde(rename = "Csr")]
    #[serde(default)]
    pub csr: String,
    #[serde(rename = "IdempotencyToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_token: Option<String>,
    #[serde(rename = "SigningAlgorithm")]
    #[serde(default)]
    pub signing_algorithm: String,
    #[serde(rename = "TemplateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_arn: Option<String>,
    #[serde(rename = "Validity")]
    #[serde(default)]
    pub validity: Validity,
    #[serde(rename = "ValidityNotBefore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_not_before: Option<Validity>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApiPassthrough {
    #[serde(rename = "Extensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Extensions>,
    #[serde(rename = "Subject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<ASN1Subject>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Extensions {
    #[serde(rename = "CertificatePolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_policies: Option<Vec<PolicyInformation>>,
    #[serde(rename = "CustomExtensions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_extensions: Option<Vec<CustomExtension>>,
    #[serde(rename = "ExtendedKeyUsage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_key_usage: Option<Vec<ExtendedKeyUsage>>,
    #[serde(rename = "KeyUsage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_usage: Option<KeyUsage>,
    #[serde(rename = "SubjectAlternativeNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_alternative_names: Option<Vec<GeneralName>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PolicyInformation {
    #[serde(rename = "CertPolicyId")]
    #[serde(default)]
    pub cert_policy_id: String,
    #[serde(rename = "PolicyQualifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_qualifiers: Option<Vec<PolicyQualifierInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PolicyQualifierInfo {
    #[serde(rename = "PolicyQualifierId")]
    #[serde(default)]
    pub policy_qualifier_id: String,
    #[serde(rename = "Qualifier")]
    #[serde(default)]
    pub qualifier: Qualifier,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Qualifier {
    #[serde(rename = "CpsUri")]
    #[serde(default)]
    pub cps_uri: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CustomExtension {
    #[serde(rename = "Critical")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub critical: Option<bool>,
    #[serde(rename = "ObjectIdentifier")]
    #[serde(default)]
    pub object_identifier: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExtendedKeyUsage {
    #[serde(rename = "ExtendedKeyUsageObjectIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_key_usage_object_identifier: Option<String>,
    #[serde(rename = "ExtendedKeyUsageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_key_usage_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Validity {
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IssueCertificateResponse {
    #[serde(rename = "CertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCertificateAuthoritiesRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCertificateAuthoritiesResponse {
    #[serde(rename = "CertificateAuthorities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authorities: Option<Vec<CertificateAuthority>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPermissionsRequest {
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(default)]
    pub certificate_authority_arn: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPermissionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<Permission>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Permission {
    #[serde(rename = "Actions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_authority_arn: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "Principal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<String>,
    #[serde(rename = "SourceAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_account: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsRequest {
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(default)]
    pub certificate_authority_arn: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutPolicyRequest {
    #[serde(rename = "Policy")]
    #[serde(default)]
    pub policy: String,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestoreCertificateAuthorityRequest {
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(default)]
    pub certificate_authority_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RevokeCertificateRequest {
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(default)]
    pub certificate_authority_arn: String,
    #[serde(rename = "CertificateSerial")]
    #[serde(default)]
    pub certificate_serial: String,
    #[serde(rename = "RevocationReason")]
    #[serde(default)]
    pub revocation_reason: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagCertificateAuthorityRequest {
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(default)]
    pub certificate_authority_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagCertificateAuthorityRequest {
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(default)]
    pub certificate_authority_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateCertificateAuthorityRequest {
    #[serde(rename = "CertificateAuthorityArn")]
    #[serde(default)]
    pub certificate_authority_arn: String,
    #[serde(rename = "RevocationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_configuration: Option<RevocationConfiguration>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
