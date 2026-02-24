//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-route53domains

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceptDomainTransferFromAnotherAwsAccountRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "Password")]
    #[serde(default)]
    pub password: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceptDomainTransferFromAnotherAwsAccountResponse {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateDelegationSignerToDomainRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "SigningAttributes")]
    #[serde(default)]
    pub signing_attributes: DnssecSigningAttributes,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DnssecSigningAttributes {
    #[serde(rename = "Algorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<i32>,
    #[serde(rename = "Flags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<i32>,
    #[serde(rename = "PublicKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateDelegationSignerToDomainResponse {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelDomainTransferToAnotherAwsAccountRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelDomainTransferToAnotherAwsAccountResponse {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CheckDomainAvailabilityRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "IdnLangCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idn_lang_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CheckDomainAvailabilityResponse {
    #[serde(rename = "Availability")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CheckDomainTransferabilityRequest {
    #[serde(rename = "AuthCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_code: Option<String>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CheckDomainTransferabilityResponse {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Transferability")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transferability: Option<DomainTransferability>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainTransferability {
    #[serde(rename = "Transferable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transferable: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDomainRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDomainResponse {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTagsForDomainRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "TagsToDelete")]
    #[serde(default)]
    pub tags_to_delete: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTagsForDomainResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableDomainAutoRenewRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableDomainAutoRenewResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableDomainTransferLockRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableDomainTransferLockResponse {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateDelegationSignerFromDomainRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateDelegationSignerFromDomainResponse {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableDomainAutoRenewRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableDomainAutoRenewResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableDomainTransferLockRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableDomainTransferLockResponse {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetContactReachabilityStatusRequest {
    #[serde(rename = "domainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetContactReachabilityStatusResponse {
    #[serde(rename = "domainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDomainDetailRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDomainDetailResponse {
    #[serde(rename = "AbuseContactEmail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abuse_contact_email: Option<String>,
    #[serde(rename = "AbuseContactPhone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abuse_contact_phone: Option<String>,
    #[serde(rename = "AdminContact")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_contact: Option<ContactDetail>,
    #[serde(rename = "AdminPrivacy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_privacy: Option<bool>,
    #[serde(rename = "AutoRenew")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
    #[serde(rename = "BillingContact")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_contact: Option<ContactDetail>,
    #[serde(rename = "BillingPrivacy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_privacy: Option<bool>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "DnsSec")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_sec: Option<String>,
    #[serde(rename = "DnssecKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dnssec_keys: Option<Vec<DnssecKey>>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "ExpirationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<f64>,
    #[serde(rename = "Nameservers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nameservers: Option<Vec<Nameserver>>,
    #[serde(rename = "RegistrantContact")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrant_contact: Option<ContactDetail>,
    #[serde(rename = "RegistrantPrivacy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrant_privacy: Option<bool>,
    #[serde(rename = "RegistrarName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrar_name: Option<String>,
    #[serde(rename = "RegistrarUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrar_url: Option<String>,
    #[serde(rename = "RegistryDomainId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_domain_id: Option<String>,
    #[serde(rename = "Reseller")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reseller: Option<String>,
    #[serde(rename = "StatusList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_list: Option<Vec<String>>,
    #[serde(rename = "TechContact")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tech_contact: Option<ContactDetail>,
    #[serde(rename = "TechPrivacy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tech_privacy: Option<bool>,
    #[serde(rename = "UpdatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_date: Option<f64>,
    #[serde(rename = "WhoIsServer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub who_is_server: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContactDetail {
    #[serde(rename = "AddressLine1")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<String>,
    #[serde(rename = "AddressLine2")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<String>,
    #[serde(rename = "City")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "ContactType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_type: Option<String>,
    #[serde(rename = "CountryCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(rename = "Email")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "ExtraParams")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_params: Option<Vec<ExtraParam>>,
    #[serde(rename = "Fax")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fax: Option<String>,
    #[serde(rename = "FirstName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "LastName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "OrganizationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_name: Option<String>,
    #[serde(rename = "PhoneNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "ZipCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExtraParam {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DnssecKey {
    #[serde(rename = "Algorithm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<i32>,
    #[serde(rename = "Digest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[serde(rename = "DigestType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest_type: Option<i32>,
    #[serde(rename = "Flags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<i32>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "KeyTag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_tag: Option<i32>,
    #[serde(rename = "PublicKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Nameserver {
    #[serde(rename = "GlueIps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glue_ips: Option<Vec<String>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDomainSuggestionsRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "OnlyAvailable")]
    #[serde(default)]
    pub only_available: bool,
    #[serde(rename = "SuggestionCount")]
    #[serde(default)]
    pub suggestion_count: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDomainSuggestionsResponse {
    #[serde(rename = "SuggestionsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestions_list: Option<Vec<DomainSuggestion>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainSuggestion {
    #[serde(rename = "Availability")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability: Option<String>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOperationDetailRequest {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    pub operation_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOperationDetailResponse {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "LastUpdatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<f64>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusFlag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_flag: Option<String>,
    #[serde(rename = "SubmittedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_date: Option<f64>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDomainsRequest {
    #[serde(rename = "FilterConditions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_conditions: Option<Vec<FilterCondition>>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "SortCondition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_condition: Option<SortCondition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterCondition {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Operator")]
    #[serde(default)]
    pub operator: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SortCondition {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "SortOrder")]
    #[serde(default)]
    pub sort_order: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDomainsResponse {
    #[serde(rename = "Domains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<DomainSummary>>,
    #[serde(rename = "NextPageMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainSummary {
    #[serde(rename = "AutoRenew")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "Expiry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry: Option<f64>,
    #[serde(rename = "TransferLock")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_lock: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOperationsRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "SortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "SortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Vec<String>>,
    #[serde(rename = "SubmittedSince")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_since: Option<f64>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOperationsResponse {
    #[serde(rename = "NextPageMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_marker: Option<String>,
    #[serde(rename = "Operations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<OperationSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OperationSummary {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "LastUpdatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<f64>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusFlag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_flag: Option<String>,
    #[serde(rename = "SubmittedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_date: Option<f64>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPricesRequest {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "Tld")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tld: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPricesResponse {
    #[serde(rename = "NextPageMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_marker: Option<String>,
    #[serde(rename = "Prices")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prices: Option<Vec<DomainPrice>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainPrice {
    #[serde(rename = "ChangeOwnershipPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_ownership_price: Option<PriceWithCurrency>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RegistrationPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_price: Option<PriceWithCurrency>,
    #[serde(rename = "RenewalPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_price: Option<PriceWithCurrency>,
    #[serde(rename = "RestorationPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restoration_price: Option<PriceWithCurrency>,
    #[serde(rename = "TransferPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_price: Option<PriceWithCurrency>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PriceWithCurrency {
    #[serde(rename = "Currency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "Price")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForDomainRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForDomainResponse {
    #[serde(rename = "TagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PushDomainRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "Target")]
    #[serde(default)]
    pub target: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterDomainRequest {
    #[serde(rename = "AdminContact")]
    #[serde(default)]
    pub admin_contact: ContactDetail,
    #[serde(rename = "AutoRenew")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
    #[serde(rename = "BillingContact")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_contact: Option<ContactDetail>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "DurationInYears")]
    #[serde(default)]
    pub duration_in_years: i32,
    #[serde(rename = "IdnLangCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idn_lang_code: Option<String>,
    #[serde(rename = "PrivacyProtectAdminContact")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_protect_admin_contact: Option<bool>,
    #[serde(rename = "PrivacyProtectBillingContact")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_protect_billing_contact: Option<bool>,
    #[serde(rename = "PrivacyProtectRegistrantContact")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_protect_registrant_contact: Option<bool>,
    #[serde(rename = "PrivacyProtectTechContact")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_protect_tech_contact: Option<bool>,
    #[serde(rename = "RegistrantContact")]
    #[serde(default)]
    pub registrant_contact: ContactDetail,
    #[serde(rename = "TechContact")]
    #[serde(default)]
    pub tech_contact: ContactDetail,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterDomainResponse {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RejectDomainTransferFromAnotherAwsAccountRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RejectDomainTransferFromAnotherAwsAccountResponse {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RenewDomainRequest {
    #[serde(rename = "CurrentExpiryYear")]
    #[serde(default)]
    pub current_expiry_year: i32,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "DurationInYears")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_years: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RenewDomainResponse {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResendContactReachabilityEmailRequest {
    #[serde(rename = "domainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResendContactReachabilityEmailResponse {
    #[serde(rename = "domainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "emailAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(rename = "isAlreadyVerified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_already_verified: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResendOperationAuthorizationRequest {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    pub operation_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetrieveDomainAuthCodeRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetrieveDomainAuthCodeResponse {
    #[serde(rename = "AuthCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransferDomainRequest {
    #[serde(rename = "AdminContact")]
    #[serde(default)]
    pub admin_contact: ContactDetail,
    #[serde(rename = "AuthCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_code: Option<String>,
    #[serde(rename = "AutoRenew")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_renew: Option<bool>,
    #[serde(rename = "BillingContact")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_contact: Option<ContactDetail>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "DurationInYears")]
    #[serde(default)]
    pub duration_in_years: i32,
    #[serde(rename = "IdnLangCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idn_lang_code: Option<String>,
    #[serde(rename = "Nameservers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nameservers: Option<Vec<Nameserver>>,
    #[serde(rename = "PrivacyProtectAdminContact")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_protect_admin_contact: Option<bool>,
    #[serde(rename = "PrivacyProtectBillingContact")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_protect_billing_contact: Option<bool>,
    #[serde(rename = "PrivacyProtectRegistrantContact")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_protect_registrant_contact: Option<bool>,
    #[serde(rename = "PrivacyProtectTechContact")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_protect_tech_contact: Option<bool>,
    #[serde(rename = "RegistrantContact")]
    #[serde(default)]
    pub registrant_contact: ContactDetail,
    #[serde(rename = "TechContact")]
    #[serde(default)]
    pub tech_contact: ContactDetail,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransferDomainResponse {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransferDomainToAnotherAwsAccountRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TransferDomainToAnotherAwsAccountResponse {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(rename = "Password")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDomainContactPrivacyRequest {
    #[serde(rename = "AdminPrivacy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_privacy: Option<bool>,
    #[serde(rename = "BillingPrivacy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_privacy: Option<bool>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "RegistrantPrivacy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrant_privacy: Option<bool>,
    #[serde(rename = "TechPrivacy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tech_privacy: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDomainContactPrivacyResponse {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDomainContactRequest {
    #[serde(rename = "AdminContact")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_contact: Option<ContactDetail>,
    #[serde(rename = "BillingContact")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_contact: Option<ContactDetail>,
    #[serde(rename = "Consent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent: Option<Consent>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "RegistrantContact")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registrant_contact: Option<ContactDetail>,
    #[serde(rename = "TechContact")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tech_contact: Option<ContactDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Consent {
    #[serde(rename = "Currency")]
    #[serde(default)]
    pub currency: String,
    #[serde(rename = "MaxPrice")]
    #[serde(default)]
    pub max_price: f64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDomainContactResponse {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDomainNameserversRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "FIAuthKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f_i_auth_key: Option<String>,
    #[serde(rename = "Nameservers")]
    #[serde(default)]
    pub nameservers: Vec<Nameserver>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDomainNameserversResponse {
    #[serde(rename = "OperationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTagsForDomainRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "TagsToUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags_to_update: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTagsForDomainResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ViewBillingRequest {
    #[serde(rename = "End")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i32>,
    #[serde(rename = "Start")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ViewBillingResponse {
    #[serde(rename = "BillingRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_records: Option<Vec<BillingRecord>>,
    #[serde(rename = "NextPageMarker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BillingRecord {
    #[serde(rename = "BillDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_date: Option<f64>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "InvoiceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    #[serde(rename = "Operation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(rename = "Price")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
}
