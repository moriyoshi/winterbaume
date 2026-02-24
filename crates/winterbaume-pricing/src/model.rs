//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-pricing

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeServicesRequest {
    #[serde(rename = "FormatVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_version: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ServiceCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeServicesResponse {
    #[serde(rename = "FormatVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_version: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Services")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<Service>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Service {
    #[serde(rename = "AttributeNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_names: Option<Vec<String>>,
    #[serde(rename = "ServiceCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAttributeValuesRequest {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    pub attribute_name: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ServiceCode")]
    #[serde(default)]
    pub service_code: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAttributeValuesResponse {
    #[serde(rename = "AttributeValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_values: Option<Vec<AttributeValue>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttributeValue {
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPriceListFileUrlRequest {
    #[serde(rename = "FileFormat")]
    #[serde(default)]
    pub file_format: String,
    #[serde(rename = "PriceListArn")]
    #[serde(default)]
    pub price_list_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPriceListFileUrlResponse {
    #[serde(rename = "Url")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetProductsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "FormatVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_version: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ServiceCode")]
    #[serde(default)]
    pub service_code: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Filter {
    #[serde(rename = "Field")]
    #[serde(default)]
    pub field: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetProductsResponse {
    #[serde(rename = "FormatVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_version: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PriceList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPriceListsRequest {
    #[serde(rename = "CurrencyCode")]
    #[serde(default)]
    pub currency_code: String,
    #[serde(rename = "EffectiveDate")]
    #[serde(default)]
    pub effective_date: f64,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RegionCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_code: Option<String>,
    #[serde(rename = "ServiceCode")]
    #[serde(default)]
    pub service_code: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPriceListsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PriceLists")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_lists: Option<Vec<PriceList>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PriceList {
    #[serde(rename = "CurrencyCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(rename = "FileFormats")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_formats: Option<Vec<String>>,
    #[serde(rename = "PriceListArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_list_arn: Option<String>,
    #[serde(rename = "RegionCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_code: Option<String>,
}
