//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-savingsplans

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSavingsPlanRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    pub commitment: String,
    #[serde(rename = "purchaseTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_time: Option<f64>,
    #[serde(rename = "savingsPlanOfferingId")]
    #[serde(default)]
    pub savings_plan_offering_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "upfrontPaymentAmount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upfront_payment_amount: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSavingsPlanResponse {
    #[serde(rename = "savingsPlanId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plan_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteQueuedSavingsPlanRequest {
    #[serde(rename = "savingsPlanId")]
    #[serde(default)]
    pub savings_plan_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteQueuedSavingsPlanResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSavingsPlanRatesRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<SavingsPlanRateFilter>>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "savingsPlanId")]
    #[serde(default)]
    pub savings_plan_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SavingsPlanRateFilter {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSavingsPlanRatesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "savingsPlanId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plan_id: Option<String>,
    #[serde(rename = "searchResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_results: Option<Vec<SavingsPlanRate>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SavingsPlanRate {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(rename = "productType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<SavingsPlanRateProperty>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,
    #[serde(rename = "serviceCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "usageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SavingsPlanRateProperty {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSavingsPlansOfferingRatesRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<SavingsPlanOfferingRateFilterElement>>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<String>>,
    #[serde(rename = "savingsPlanOfferingIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plan_offering_ids: Option<Vec<String>>,
    #[serde(rename = "savingsPlanPaymentOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plan_payment_options: Option<Vec<String>>,
    #[serde(rename = "savingsPlanTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plan_types: Option<Vec<String>>,
    #[serde(rename = "serviceCodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_codes: Option<Vec<String>>,
    #[serde(rename = "usageTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SavingsPlanOfferingRateFilterElement {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSavingsPlansOfferingRatesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "searchResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_results: Option<Vec<SavingsPlanOfferingRate>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SavingsPlanOfferingRate {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(rename = "productType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<SavingsPlanOfferingRateProperty>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,
    #[serde(rename = "savingsPlanOffering")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plan_offering: Option<ParentSavingsPlanOffering>,
    #[serde(rename = "serviceCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "usageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SavingsPlanOfferingRateProperty {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParentSavingsPlanOffering {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "durationSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i64>,
    #[serde(rename = "offeringId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_id: Option<String>,
    #[serde(rename = "paymentOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_option: Option<String>,
    #[serde(rename = "planDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_description: Option<String>,
    #[serde(rename = "planType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSavingsPlansOfferingsRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currencies: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub durations: Option<Vec<i64>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<SavingsPlanOfferingFilterElement>>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "offeringIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_ids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<String>>,
    #[serde(rename = "paymentOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_options: Option<Vec<String>>,
    #[serde(rename = "planTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_types: Option<Vec<String>>,
    #[serde(rename = "productType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
    #[serde(rename = "serviceCodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_codes: Option<Vec<String>>,
    #[serde(rename = "usageTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SavingsPlanOfferingFilterElement {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSavingsPlansOfferingsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "searchResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_results: Option<Vec<SavingsPlanOffering>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SavingsPlanOffering {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "durationSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i64>,
    #[serde(rename = "offeringId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(rename = "paymentOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_option: Option<String>,
    #[serde(rename = "planType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<String>,
    #[serde(rename = "productTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_types: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<SavingsPlanOfferingProperty>>,
    #[serde(rename = "serviceCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
    #[serde(rename = "usageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SavingsPlanOfferingProperty {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSavingsPlansRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<SavingsPlanFilter>>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "savingsPlanArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plan_arns: Option<Vec<String>>,
    #[serde(rename = "savingsPlanIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plan_ids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SavingsPlanFilter {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSavingsPlansResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "savingsPlans")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plans: Option<Vec<SavingsPlan>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SavingsPlan {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ec2InstanceFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_instance_family: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    #[serde(rename = "offeringId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_id: Option<String>,
    #[serde(rename = "paymentOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_option: Option<String>,
    #[serde(rename = "productTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_types: Option<Vec<String>>,
    #[serde(rename = "recurringPaymentAmount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_payment_amount: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "returnableUntil")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returnable_until: Option<String>,
    #[serde(rename = "savingsPlanArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plan_arn: Option<String>,
    #[serde(rename = "savingsPlanId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plan_id: Option<String>,
    #[serde(rename = "savingsPlanType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plan_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "termDurationInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term_duration_in_seconds: Option<i64>,
    #[serde(rename = "upfrontPaymentAmount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upfront_payment_amount: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReturnSavingsPlanRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "savingsPlanId")]
    #[serde(default)]
    pub savings_plan_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReturnSavingsPlanResponse {
    #[serde(rename = "savingsPlanId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub savings_plan_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "tagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}
