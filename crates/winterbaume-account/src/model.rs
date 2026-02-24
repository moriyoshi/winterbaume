//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-account

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAlternateContactResponse {
    #[serde(rename = "AlternateContact")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate_contact: Option<AlternateContact>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AlternateContact {
    #[serde(rename = "AlternateContactType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate_contact_type: Option<String>,
    #[serde(rename = "EmailAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PhoneNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccountInformationResponse {
    #[serde(rename = "AccountCreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_created_date: Option<String>,
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "AccountName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPrimaryEmailRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRegionOptStatusRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "RegionName")]
    #[serde(default)]
    pub region_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRegionOptStatusResponse {
    #[serde(rename = "RegionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
    #[serde(rename = "RegionOptStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_opt_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPrimaryEmailResponse {
    #[serde(rename = "PrimaryEmail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_email: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAlternateContactRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "AlternateContactType")]
    #[serde(default)]
    pub alternate_contact_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAlternateContactRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "AlternateContactType")]
    #[serde(default)]
    pub alternate_contact_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartPrimaryEmailUpdateResponse {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceptPrimaryEmailUpdateRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "Otp")]
    #[serde(default)]
    pub otp: String,
    #[serde(rename = "PrimaryEmail")]
    #[serde(default)]
    pub primary_email: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAccountNameRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "AccountName")]
    #[serde(default)]
    pub account_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableRegionRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "RegionName")]
    #[serde(default)]
    pub region_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRegionsRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RegionOptStatusContains")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_opt_status_contains: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableRegionRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "RegionName")]
    #[serde(default)]
    pub region_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetContactInformationRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetContactInformationResponse {
    #[serde(rename = "ContactInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_information: Option<ContactInformation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContactInformation {
    #[serde(rename = "AddressLine1")]
    #[serde(default)]
    pub address_line1: String,
    #[serde(rename = "AddressLine2")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<String>,
    #[serde(rename = "AddressLine3")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line3: Option<String>,
    #[serde(rename = "City")]
    #[serde(default)]
    pub city: String,
    #[serde(rename = "CompanyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[serde(rename = "CountryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(rename = "DistrictOrCounty")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub district_or_county: Option<String>,
    #[serde(rename = "FullName")]
    #[serde(default)]
    pub full_name: String,
    #[serde(rename = "PhoneNumber")]
    #[serde(default)]
    pub phone_number: String,
    #[serde(rename = "PostalCode")]
    #[serde(default)]
    pub postal_code: String,
    #[serde(rename = "StateOrRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_or_region: Option<String>,
    #[serde(rename = "WebsiteUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetGovCloudAccountInformationRequest {
    #[serde(rename = "StandardAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_account_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAccountInformationRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutAlternateContactRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "AlternateContactType")]
    #[serde(default)]
    pub alternate_contact_type: String,
    #[serde(rename = "EmailAddress")]
    #[serde(default)]
    pub email_address: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "PhoneNumber")]
    #[serde(default)]
    pub phone_number: String,
    #[serde(rename = "Title")]
    #[serde(default)]
    pub title: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartPrimaryEmailUpdateRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "PrimaryEmail")]
    #[serde(default)]
    pub primary_email: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceptPrimaryEmailUpdateResponse {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetGovCloudAccountInformationResponse {
    #[serde(rename = "AccountState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_state: Option<String>,
    #[serde(rename = "GovCloudAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gov_cloud_account_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRegionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Regions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<Region>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Region {
    #[serde(rename = "RegionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
    #[serde(rename = "RegionOptStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_opt_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutContactInformationRequest {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "ContactInformation")]
    #[serde(default)]
    pub contact_information: ContactInformation,
}
