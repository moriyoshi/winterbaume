//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-taxsettings

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteTaxRegistrationRequest {
    #[serde(rename = "accountIds")]
    #[serde(default)]
    pub account_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteTaxRegistrationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<BatchDeleteTaxRegistrationError>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteTaxRegistrationError {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetTaxExemptionsRequest {
    #[serde(rename = "accountIds")]
    #[serde(default)]
    pub account_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetTaxExemptionsResponse {
    #[serde(rename = "failedAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_accounts: Option<Vec<String>>,
    #[serde(rename = "taxExemptionDetailsMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exemption_details_map: Option<std::collections::HashMap<String, TaxExemptionDetails>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaxExemptionDetails {
    #[serde(rename = "heritageObtainedDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heritage_obtained_details: Option<bool>,
    #[serde(rename = "heritageObtainedParentEntity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heritage_obtained_parent_entity: Option<String>,
    #[serde(rename = "heritageObtainedReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heritage_obtained_reason: Option<String>,
    #[serde(rename = "taxExemptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exemptions: Option<Vec<TaxExemption>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaxExemption {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authority: Option<Authority>,
    #[serde(rename = "effectiveDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<f64>,
    #[serde(rename = "expirationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "systemEffectiveDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_effective_date: Option<f64>,
    #[serde(rename = "taxExemptionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exemption_type: Option<TaxExemptionType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Authority {
    #[serde(default)]
    pub country: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaxExemptionType {
    #[serde(rename = "applicableJurisdictions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applicable_jurisdictions: Option<Vec<Authority>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchPutTaxRegistrationRequest {
    #[serde(rename = "accountIds")]
    #[serde(default)]
    pub account_ids: Vec<String>,
    #[serde(rename = "taxRegistrationEntry")]
    #[serde(default)]
    pub tax_registration_entry: TaxRegistrationEntry,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaxRegistrationEntry {
    #[serde(rename = "additionalTaxInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_tax_information: Option<AdditionalInfoRequest>,
    #[serde(rename = "certifiedEmailId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certified_email_id: Option<String>,
    #[serde(rename = "legalAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_address: Option<Address>,
    #[serde(rename = "legalName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    #[serde(rename = "registrationId")]
    #[serde(default)]
    pub registration_id: String,
    #[serde(rename = "registrationType")]
    #[serde(default)]
    pub registration_type: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sector: Option<String>,
    #[serde(rename = "verificationDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_details: Option<VerificationDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdditionalInfoRequest {
    #[serde(rename = "canadaAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canada_additional_info: Option<CanadaAdditionalInfo>,
    #[serde(rename = "egyptAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egypt_additional_info: Option<EgyptAdditionalInfo>,
    #[serde(rename = "estoniaAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estonia_additional_info: Option<EstoniaAdditionalInfo>,
    #[serde(rename = "georgiaAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub georgia_additional_info: Option<GeorgiaAdditionalInfo>,
    #[serde(rename = "greeceAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greece_additional_info: Option<GreeceAdditionalInfo>,
    #[serde(rename = "indonesiaAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indonesia_additional_info: Option<IndonesiaAdditionalInfo>,
    #[serde(rename = "israelAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub israel_additional_info: Option<IsraelAdditionalInfo>,
    #[serde(rename = "italyAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italy_additional_info: Option<ItalyAdditionalInfo>,
    #[serde(rename = "kenyaAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kenya_additional_info: Option<KenyaAdditionalInfo>,
    #[serde(rename = "malaysiaAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malaysia_additional_info: Option<MalaysiaAdditionalInfo>,
    #[serde(rename = "polandAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poland_additional_info: Option<PolandAdditionalInfo>,
    #[serde(rename = "romaniaAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub romania_additional_info: Option<RomaniaAdditionalInfo>,
    #[serde(rename = "saudiArabiaAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saudi_arabia_additional_info: Option<SaudiArabiaAdditionalInfo>,
    #[serde(rename = "southKoreaAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub south_korea_additional_info: Option<SouthKoreaAdditionalInfo>,
    #[serde(rename = "spainAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spain_additional_info: Option<SpainAdditionalInfo>,
    #[serde(rename = "turkeyAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turkey_additional_info: Option<TurkeyAdditionalInfo>,
    #[serde(rename = "ukraineAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ukraine_additional_info: Option<UkraineAdditionalInfo>,
    #[serde(rename = "uzbekistanAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uzbekistan_additional_info: Option<UzbekistanAdditionalInfo>,
    #[serde(rename = "vietnamAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vietnam_additional_info: Option<VietnamAdditionalInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CanadaAdditionalInfo {
    #[serde(rename = "canadaQuebecSalesTaxNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canada_quebec_sales_tax_number: Option<String>,
    #[serde(rename = "canadaRetailSalesTaxNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canada_retail_sales_tax_number: Option<String>,
    #[serde(rename = "isResellerAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_reseller_account: Option<bool>,
    #[serde(rename = "provincialSalesTaxId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provincial_sales_tax_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EgyptAdditionalInfo {
    #[serde(rename = "uniqueIdentificationNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_identification_number: Option<String>,
    #[serde(rename = "uniqueIdentificationNumberExpirationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_identification_number_expiration_date: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EstoniaAdditionalInfo {
    #[serde(rename = "registryCommercialCode")]
    #[serde(default)]
    pub registry_commercial_code: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GeorgiaAdditionalInfo {
    #[serde(rename = "personType")]
    #[serde(default)]
    pub person_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GreeceAdditionalInfo {
    #[serde(rename = "contractingAuthorityCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contracting_authority_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IndonesiaAdditionalInfo {
    #[serde(rename = "decisionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_number: Option<String>,
    #[serde(rename = "ppnExceptionDesignationCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ppn_exception_designation_code: Option<String>,
    #[serde(rename = "taxRegistrationNumberType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_registration_number_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IsraelAdditionalInfo {
    #[serde(rename = "customerType")]
    #[serde(default)]
    pub customer_type: String,
    #[serde(rename = "dealerType")]
    #[serde(default)]
    pub dealer_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ItalyAdditionalInfo {
    #[serde(rename = "cigNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cig_number: Option<String>,
    #[serde(rename = "cupNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cup_number: Option<String>,
    #[serde(rename = "sdiAccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdi_account_id: Option<String>,
    #[serde(rename = "taxCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KenyaAdditionalInfo {
    #[serde(rename = "personType")]
    #[serde(default)]
    pub person_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MalaysiaAdditionalInfo {
    #[serde(rename = "businessRegistrationNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_registration_number: Option<String>,
    #[serde(rename = "serviceTaxCodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tax_codes: Option<Vec<String>>,
    #[serde(rename = "taxInformationNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_information_number: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PolandAdditionalInfo {
    #[serde(rename = "individualRegistrationNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual_registration_number: Option<String>,
    #[serde(rename = "isGroupVatEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_group_vat_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RomaniaAdditionalInfo {
    #[serde(rename = "taxRegistrationNumberType")]
    #[serde(default)]
    pub tax_registration_number_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SaudiArabiaAdditionalInfo {
    #[serde(rename = "taxRegistrationNumberType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_registration_number_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SouthKoreaAdditionalInfo {
    #[serde(rename = "businessRepresentativeName")]
    #[serde(default)]
    pub business_representative_name: String,
    #[serde(rename = "itemOfBusiness")]
    #[serde(default)]
    pub item_of_business: String,
    #[serde(rename = "lineOfBusiness")]
    #[serde(default)]
    pub line_of_business: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SpainAdditionalInfo {
    #[serde(rename = "registrationType")]
    #[serde(default)]
    pub registration_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TurkeyAdditionalInfo {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub industries: Option<String>,
    #[serde(rename = "kepEmailId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kep_email_id: Option<String>,
    #[serde(rename = "secondaryTaxId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_tax_id: Option<String>,
    #[serde(rename = "taxOffice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_office: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UkraineAdditionalInfo {
    #[serde(rename = "ukraineTrnType")]
    #[serde(default)]
    pub ukraine_trn_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UzbekistanAdditionalInfo {
    #[serde(rename = "taxRegistrationNumberType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_registration_number_type: Option<String>,
    #[serde(rename = "vatRegistrationNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_registration_number: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VietnamAdditionalInfo {
    #[serde(rename = "electronicTransactionCodeNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub electronic_transaction_code_number: Option<String>,
    #[serde(rename = "enterpriseIdentificationNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enterprise_identification_number: Option<String>,
    #[serde(rename = "paymentVoucherNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_voucher_number: Option<String>,
    #[serde(rename = "paymentVoucherNumberDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_voucher_number_date: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Address {
    #[serde(rename = "addressLine1")]
    #[serde(default)]
    pub address_line1: String,
    #[serde(rename = "addressLine2")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<String>,
    #[serde(rename = "addressLine3")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line3: Option<String>,
    #[serde(default)]
    pub city: String,
    #[serde(rename = "countryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(rename = "districtOrCounty")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub district_or_county: Option<String>,
    #[serde(rename = "postalCode")]
    #[serde(default)]
    pub postal_code: String,
    #[serde(rename = "stateOrRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_or_region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VerificationDetails {
    #[serde(rename = "dateOfBirth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    #[serde(rename = "taxRegistrationDocuments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_registration_documents: Option<Vec<TaxRegistrationDocument>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaxRegistrationDocument {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<TaxRegistrationDocFile>,
    #[serde(rename = "s3Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_location: Option<SourceS3Location>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaxRegistrationDocFile {
    #[serde(rename = "fileContent")]
    #[serde(default)]
    pub file_content: String,
    #[serde(rename = "fileName")]
    #[serde(default)]
    pub file_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceS3Location {
    #[serde(default)]
    pub bucket: String,
    #[serde(default)]
    pub key: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchPutTaxRegistrationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<BatchPutTaxRegistrationError>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchPutTaxRegistrationError {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSupplementalTaxRegistrationRequest {
    #[serde(rename = "authorityId")]
    #[serde(default)]
    pub authority_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSupplementalTaxRegistrationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTaxRegistrationRequest {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTaxRegistrationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTaxExemptionTypesRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTaxExemptionTypesResponse {
    #[serde(rename = "taxExemptionTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exemption_types: Option<Vec<TaxExemptionType>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTaxInheritanceRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTaxInheritanceResponse {
    #[serde(rename = "heritageStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heritage_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTaxRegistrationDocumentRequest {
    #[serde(rename = "destinationS3Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_s3_location: Option<DestinationS3Location>,
    #[serde(rename = "taxDocumentMetadata")]
    #[serde(default)]
    pub tax_document_metadata: TaxDocumentMetadata,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DestinationS3Location {
    #[serde(default)]
    pub bucket: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaxDocumentMetadata {
    #[serde(rename = "taxDocumentAccessToken")]
    #[serde(default)]
    pub tax_document_access_token: String,
    #[serde(rename = "taxDocumentName")]
    #[serde(default)]
    pub tax_document_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTaxRegistrationDocumentResponse {
    #[serde(rename = "destinationFilePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_file_path: Option<String>,
    #[serde(rename = "presignedS3Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presigned_s3_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTaxRegistrationRequest {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTaxRegistrationResponse {
    #[serde(rename = "taxRegistration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_registration: Option<TaxRegistration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaxRegistration {
    #[serde(rename = "additionalTaxInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_tax_information: Option<AdditionalInfoResponse>,
    #[serde(rename = "certifiedEmailId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certified_email_id: Option<String>,
    #[serde(rename = "legalAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_address: Option<Address>,
    #[serde(rename = "legalName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    #[serde(rename = "registrationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_id: Option<String>,
    #[serde(rename = "registrationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sector: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "taxDocumentMetadatas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_document_metadatas: Option<Vec<TaxDocumentMetadata>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdditionalInfoResponse {
    #[serde(rename = "brazilAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brazil_additional_info: Option<BrazilAdditionalInfo>,
    #[serde(rename = "canadaAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canada_additional_info: Option<CanadaAdditionalInfo>,
    #[serde(rename = "egyptAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egypt_additional_info: Option<EgyptAdditionalInfo>,
    #[serde(rename = "estoniaAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estonia_additional_info: Option<EstoniaAdditionalInfo>,
    #[serde(rename = "georgiaAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub georgia_additional_info: Option<GeorgiaAdditionalInfo>,
    #[serde(rename = "greeceAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greece_additional_info: Option<GreeceAdditionalInfo>,
    #[serde(rename = "indiaAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub india_additional_info: Option<IndiaAdditionalInfo>,
    #[serde(rename = "indonesiaAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indonesia_additional_info: Option<IndonesiaAdditionalInfo>,
    #[serde(rename = "israelAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub israel_additional_info: Option<IsraelAdditionalInfo>,
    #[serde(rename = "italyAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italy_additional_info: Option<ItalyAdditionalInfo>,
    #[serde(rename = "kenyaAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kenya_additional_info: Option<KenyaAdditionalInfo>,
    #[serde(rename = "malaysiaAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub malaysia_additional_info: Option<MalaysiaAdditionalInfo>,
    #[serde(rename = "polandAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poland_additional_info: Option<PolandAdditionalInfo>,
    #[serde(rename = "romaniaAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub romania_additional_info: Option<RomaniaAdditionalInfo>,
    #[serde(rename = "saudiArabiaAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saudi_arabia_additional_info: Option<SaudiArabiaAdditionalInfo>,
    #[serde(rename = "southKoreaAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub south_korea_additional_info: Option<SouthKoreaAdditionalInfo>,
    #[serde(rename = "spainAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spain_additional_info: Option<SpainAdditionalInfo>,
    #[serde(rename = "turkeyAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turkey_additional_info: Option<TurkeyAdditionalInfo>,
    #[serde(rename = "ukraineAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ukraine_additional_info: Option<UkraineAdditionalInfo>,
    #[serde(rename = "uzbekistanAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uzbekistan_additional_info: Option<UzbekistanAdditionalInfo>,
    #[serde(rename = "vietnamAdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vietnam_additional_info: Option<VietnamAdditionalInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BrazilAdditionalInfo {
    #[serde(rename = "ccmCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccm_code: Option<String>,
    #[serde(rename = "legalNatureCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_nature_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IndiaAdditionalInfo {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pan: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSupplementalTaxRegistrationsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSupplementalTaxRegistrationsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "taxRegistrations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_registrations: Option<Vec<SupplementalTaxRegistration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SupplementalTaxRegistration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(rename = "authorityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authority_id: Option<String>,
    #[serde(rename = "legalName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    #[serde(rename = "registrationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_id: Option<String>,
    #[serde(rename = "registrationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTaxExemptionsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTaxExemptionsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "taxExemptionDetailsMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exemption_details_map: Option<std::collections::HashMap<String, TaxExemptionDetails>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTaxRegistrationsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTaxRegistrationsResponse {
    #[serde(rename = "accountDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_details: Option<Vec<AccountDetails>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountDetails {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "accountMetaData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_meta_data: Option<AccountMetaData>,
    #[serde(rename = "taxInheritanceDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_inheritance_details: Option<TaxInheritanceDetails>,
    #[serde(rename = "taxRegistration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_registration: Option<TaxRegistrationWithJurisdiction>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountMetaData {
    #[serde(rename = "accountName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(rename = "addressRoleMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_role_map: Option<std::collections::HashMap<String, Jurisdiction>>,
    #[serde(rename = "addressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Jurisdiction {
    #[serde(rename = "countryCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(rename = "stateOrRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_or_region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaxInheritanceDetails {
    #[serde(rename = "inheritanceObtainedReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inheritance_obtained_reason: Option<String>,
    #[serde(rename = "parentEntityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_entity_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaxRegistrationWithJurisdiction {
    #[serde(rename = "additionalTaxInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_tax_information: Option<AdditionalInfoResponse>,
    #[serde(rename = "certifiedEmailId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certified_email_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<Jurisdiction>,
    #[serde(rename = "legalName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    #[serde(rename = "registrationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_id: Option<String>,
    #[serde(rename = "registrationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sector: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "taxDocumentMetadatas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_document_metadatas: Option<Vec<TaxDocumentMetadata>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutSupplementalTaxRegistrationRequest {
    #[serde(rename = "taxRegistrationEntry")]
    #[serde(default)]
    pub tax_registration_entry: SupplementalTaxRegistrationEntry,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SupplementalTaxRegistrationEntry {
    #[serde(default)]
    pub address: Address,
    #[serde(rename = "legalName")]
    #[serde(default)]
    pub legal_name: String,
    #[serde(rename = "registrationId")]
    #[serde(default)]
    pub registration_id: String,
    #[serde(rename = "registrationType")]
    #[serde(default)]
    pub registration_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutSupplementalTaxRegistrationResponse {
    #[serde(rename = "authorityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authority_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutTaxExemptionRequest {
    #[serde(rename = "accountIds")]
    #[serde(default)]
    pub account_ids: Vec<String>,
    #[serde(default)]
    pub authority: Authority,
    #[serde(rename = "exemptionCertificate")]
    #[serde(default)]
    pub exemption_certificate: ExemptionCertificate,
    #[serde(rename = "exemptionType")]
    #[serde(default)]
    pub exemption_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExemptionCertificate {
    #[serde(rename = "documentFile")]
    #[serde(default)]
    pub document_file: String,
    #[serde(rename = "documentName")]
    #[serde(default)]
    pub document_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutTaxExemptionResponse {
    #[serde(rename = "caseId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutTaxInheritanceRequest {
    #[serde(rename = "heritageStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heritage_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutTaxInheritanceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutTaxRegistrationRequest {
    #[serde(rename = "accountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "taxRegistrationEntry")]
    #[serde(default)]
    pub tax_registration_entry: TaxRegistrationEntry,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutTaxRegistrationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
