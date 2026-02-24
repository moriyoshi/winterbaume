//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-outposts

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelCapacityTaskInput {
    #[serde(rename = "CapacityTaskId")]
    #[serde(default)]
    pub capacity_task_id: String,
    #[serde(rename = "OutpostIdentifier")]
    #[serde(default)]
    pub outpost_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelCapacityTaskOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelOrderInput {
    #[serde(rename = "OrderId")]
    #[serde(default)]
    pub order_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelOrderOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateOrderInput {
    #[serde(rename = "LineItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<LineItemRequest>>,
    #[serde(rename = "OutpostIdentifier")]
    #[serde(default)]
    pub outpost_identifier: String,
    #[serde(rename = "PaymentOption")]
    #[serde(default)]
    pub payment_option: String,
    #[serde(rename = "PaymentTerm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_term: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LineItemRequest {
    #[serde(rename = "CatalogItemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_item_id: Option<String>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateOrderOutput {
    #[serde(rename = "Order")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Order {
    #[serde(rename = "LineItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<LineItem>>,
    #[serde(rename = "OrderFulfilledDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_fulfilled_date: Option<f64>,
    #[serde(rename = "OrderId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "OrderSubmissionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_submission_date: Option<f64>,
    #[serde(rename = "OrderType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    #[serde(rename = "OutpostId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_id: Option<String>,
    #[serde(rename = "PaymentOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_option: Option<String>,
    #[serde(rename = "PaymentTerm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_term: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LineItem {
    #[serde(rename = "AssetInformationList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_information_list: Option<Vec<LineItemAssetInformation>>,
    #[serde(rename = "CatalogItemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_item_id: Option<String>,
    #[serde(rename = "LineItemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_item_id: Option<String>,
    #[serde(rename = "PreviousLineItemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_line_item_id: Option<String>,
    #[serde(rename = "PreviousOrderId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_order_id: Option<String>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i32>,
    #[serde(rename = "ShipmentInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipment_information: Option<ShipmentInformation>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LineItemAssetInformation {
    #[serde(rename = "AssetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    #[serde(rename = "MacAddressList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ShipmentInformation {
    #[serde(rename = "ShipmentCarrier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipment_carrier: Option<String>,
    #[serde(rename = "ShipmentTrackingNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipment_tracking_number: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateOutpostInput {
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "AvailabilityZoneId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "SiteId")]
    #[serde(default)]
    pub site_id: String,
    #[serde(rename = "SupportedHardwareType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_hardware_type: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateOutpostOutput {
    #[serde(rename = "Outpost")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost: Option<Outpost>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Outpost {
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "AvailabilityZoneId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LifeCycleStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub life_cycle_status: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OutpostArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_arn: Option<String>,
    #[serde(rename = "OutpostId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_id: Option<String>,
    #[serde(rename = "OwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "SiteArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_arn: Option<String>,
    #[serde(rename = "SiteId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
    #[serde(rename = "SupportedHardwareType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_hardware_type: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRenewalInput {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "OutpostIdentifier")]
    #[serde(default)]
    pub outpost_identifier: String,
    #[serde(rename = "PaymentOption")]
    #[serde(default)]
    pub payment_option: String,
    #[serde(rename = "PaymentTerm")]
    #[serde(default)]
    pub payment_term: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateRenewalOutput {
    #[serde(rename = "MonthlyRecurringPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_recurring_price: Option<f32>,
    #[serde(rename = "OutpostId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_id: Option<String>,
    #[serde(rename = "PaymentOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_option: Option<String>,
    #[serde(rename = "PaymentTerm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_term: Option<String>,
    #[serde(rename = "UpfrontPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upfront_price: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSiteInput {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Notes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "OperatingAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_address: Option<Address>,
    #[serde(rename = "RackPhysicalProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rack_physical_properties: Option<RackPhysicalProperties>,
    #[serde(rename = "ShippingAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<Address>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Address {
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
    #[serde(rename = "ContactName")]
    #[serde(default)]
    pub contact_name: String,
    #[serde(rename = "ContactPhoneNumber")]
    #[serde(default)]
    pub contact_phone_number: String,
    #[serde(rename = "CountryCode")]
    #[serde(default)]
    pub country_code: String,
    #[serde(rename = "DistrictOrCounty")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub district_or_county: Option<String>,
    #[serde(rename = "Municipality")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub municipality: Option<String>,
    #[serde(rename = "PostalCode")]
    #[serde(default)]
    pub postal_code: String,
    #[serde(rename = "StateOrRegion")]
    #[serde(default)]
    pub state_or_region: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RackPhysicalProperties {
    #[serde(rename = "FiberOpticCableType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fiber_optic_cable_type: Option<String>,
    #[serde(rename = "MaximumSupportedWeightLbs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_supported_weight_lbs: Option<String>,
    #[serde(rename = "OpticalStandard")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optical_standard: Option<String>,
    #[serde(rename = "PowerConnector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_connector: Option<String>,
    #[serde(rename = "PowerDrawKva")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_draw_kva: Option<String>,
    #[serde(rename = "PowerFeedDrop")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_feed_drop: Option<String>,
    #[serde(rename = "PowerPhase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_phase: Option<String>,
    #[serde(rename = "UplinkCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uplink_count: Option<String>,
    #[serde(rename = "UplinkGbps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uplink_gbps: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSiteOutput {
    #[serde(rename = "Site")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<Site>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Site {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Notes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "OperatingAddressCity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_address_city: Option<String>,
    #[serde(rename = "OperatingAddressCountryCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_address_country_code: Option<String>,
    #[serde(rename = "OperatingAddressStateOrRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_address_state_or_region: Option<String>,
    #[serde(rename = "RackPhysicalProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rack_physical_properties: Option<RackPhysicalProperties>,
    #[serde(rename = "SiteArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_arn: Option<String>,
    #[serde(rename = "SiteId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteOutpostInput {
    #[serde(rename = "OutpostId")]
    #[serde(default)]
    pub outpost_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteOutpostOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSiteInput {
    #[serde(rename = "SiteId")]
    #[serde(default)]
    pub site_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSiteOutput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCapacityTaskInput {
    #[serde(rename = "CapacityTaskId")]
    #[serde(default)]
    pub capacity_task_id: String,
    #[serde(rename = "OutpostIdentifier")]
    #[serde(default)]
    pub outpost_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCapacityTaskOutput {
    #[serde(rename = "AssetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    #[serde(rename = "CapacityTaskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_task_id: Option<String>,
    #[serde(rename = "CapacityTaskStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_task_status: Option<String>,
    #[serde(rename = "CompletionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<f64>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "DryRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "Failed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<CapacityTaskFailure>,
    #[serde(rename = "InstancesToExclude")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_to_exclude: Option<InstancesToExclude>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "OrderId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "OutpostId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_id: Option<String>,
    #[serde(rename = "RequestedInstancePools")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_instance_pools: Option<Vec<InstanceTypeCapacity>>,
    #[serde(rename = "TaskActionOnBlockingInstances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_action_on_blocking_instances: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CapacityTaskFailure {
    #[serde(rename = "Reason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstancesToExclude {
    #[serde(rename = "AccountIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
    #[serde(rename = "Instances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<String>>,
    #[serde(rename = "Services")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceTypeCapacity {
    #[serde(rename = "Count")]
    #[serde(default)]
    pub count: i32,
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    pub instance_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCatalogItemInput {
    #[serde(rename = "CatalogItemId")]
    #[serde(default)]
    pub catalog_item_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCatalogItemOutput {
    #[serde(rename = "CatalogItem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_item: Option<CatalogItem>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CatalogItem {
    #[serde(rename = "CatalogItemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_item_id: Option<String>,
    #[serde(rename = "EC2Capacities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_c2_capacities: Option<Vec<EC2Capacity>>,
    #[serde(rename = "ItemStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_status: Option<String>,
    #[serde(rename = "PowerKva")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_kva: Option<f32>,
    #[serde(rename = "SupportedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_storage: Option<Vec<String>>,
    #[serde(rename = "SupportedUplinkGbps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_uplink_gbps: Option<Vec<i32>>,
    #[serde(rename = "WeightLbs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_lbs: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EC2Capacity {
    #[serde(rename = "Family")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(rename = "MaxSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<String>,
    #[serde(rename = "Quantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConnectionRequest {
    #[serde(rename = "ConnectionId")]
    #[serde(default)]
    pub connection_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetConnectionResponse {
    #[serde(rename = "ConnectionDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_details: Option<ConnectionDetails>,
    #[serde(rename = "ConnectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectionDetails {
    #[serde(rename = "AllowedIps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_ips: Option<Vec<String>>,
    #[serde(rename = "ClientPublicKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_public_key: Option<String>,
    #[serde(rename = "ClientTunnelAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_tunnel_address: Option<String>,
    #[serde(rename = "ServerEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_endpoint: Option<String>,
    #[serde(rename = "ServerPublicKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_public_key: Option<String>,
    #[serde(rename = "ServerTunnelAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_tunnel_address: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOrderInput {
    #[serde(rename = "OrderId")]
    #[serde(default)]
    pub order_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOrderOutput {
    #[serde(rename = "Order")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOutpostBillingInformationInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OutpostIdentifier")]
    #[serde(default)]
    pub outpost_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOutpostBillingInformationOutput {
    #[serde(rename = "ContractEndDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_end_date: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PaymentOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_option: Option<String>,
    #[serde(rename = "PaymentTerm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_term: Option<String>,
    #[serde(rename = "Subscriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<Subscription>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Subscription {
    #[serde(rename = "BeginDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_date: Option<f64>,
    #[serde(rename = "EndDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<f64>,
    #[serde(rename = "MonthlyRecurringPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_recurring_price: Option<f64>,
    #[serde(rename = "OrderIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_ids: Option<Vec<String>>,
    #[serde(rename = "SubscriptionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    #[serde(rename = "SubscriptionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_status: Option<String>,
    #[serde(rename = "SubscriptionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_type: Option<String>,
    #[serde(rename = "UpfrontPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upfront_price: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOutpostInput {
    #[serde(rename = "OutpostId")]
    #[serde(default)]
    pub outpost_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOutpostInstanceTypesInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OutpostId")]
    #[serde(default)]
    pub outpost_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOutpostInstanceTypesOutput {
    #[serde(rename = "InstanceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_types: Option<Vec<InstanceTypeItem>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OutpostArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_arn: Option<String>,
    #[serde(rename = "OutpostId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceTypeItem {
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "VCPUs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_c_p_us: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOutpostOutput {
    #[serde(rename = "Outpost")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost: Option<Outpost>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOutpostSupportedInstanceTypesInput {
    #[serde(rename = "AssetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OrderId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "OutpostIdentifier")]
    #[serde(default)]
    pub outpost_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetOutpostSupportedInstanceTypesOutput {
    #[serde(rename = "InstanceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_types: Option<Vec<InstanceTypeItem>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRenewalPricingInput {
    #[serde(rename = "OutpostIdentifier")]
    #[serde(default)]
    pub outpost_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRenewalPricingOutput {
    #[serde(rename = "PricingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_options: Option<Vec<PricingOption>>,
    #[serde(rename = "PricingResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_result: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PricingOption {
    #[serde(rename = "PricingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_type: Option<String>,
    #[serde(rename = "SubscriptionPricingDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_pricing_details: Option<SubscriptionPricingDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubscriptionPricingDetails {
    #[serde(rename = "MonthlyRecurringPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_recurring_price: Option<f32>,
    #[serde(rename = "PaymentOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_option: Option<String>,
    #[serde(rename = "PaymentTerm")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_term: Option<String>,
    #[serde(rename = "UpfrontPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upfront_price: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSiteAddressInput {
    #[serde(rename = "AddressType")]
    #[serde(default)]
    pub address_type: String,
    #[serde(rename = "SiteId")]
    #[serde(default)]
    pub site_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSiteAddressOutput {
    #[serde(rename = "Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(rename = "AddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_type: Option<String>,
    #[serde(rename = "SiteId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSiteInput {
    #[serde(rename = "SiteId")]
    #[serde(default)]
    pub site_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSiteOutput {
    #[serde(rename = "Site")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<Site>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAssetInstancesInput {
    #[serde(rename = "AccountIdFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id_filter: Option<Vec<String>>,
    #[serde(rename = "AssetIdFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id_filter: Option<Vec<String>>,
    #[serde(rename = "AwsServiceFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_service_filter: Option<Vec<String>>,
    #[serde(rename = "InstanceTypeFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type_filter: Option<Vec<String>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OutpostIdentifier")]
    #[serde(default)]
    pub outpost_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAssetInstancesOutput {
    #[serde(rename = "AssetInstances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_instances: Option<Vec<AssetInstance>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetInstance {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "AssetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    #[serde(rename = "AwsServiceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_service_name: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAssetsInput {
    #[serde(rename = "AssetTypeFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type_filter: Option<Vec<String>>,
    #[serde(rename = "HostIdFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_id_filter: Option<Vec<String>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OutpostIdentifier")]
    #[serde(default)]
    pub outpost_identifier: String,
    #[serde(rename = "StatusFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_filter: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAssetsOutput {
    #[serde(rename = "Assets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<Vec<AssetInfo>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetInfo {
    #[serde(rename = "AssetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    #[serde(rename = "AssetLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_location: Option<AssetLocation>,
    #[serde(rename = "AssetType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<String>,
    #[serde(rename = "ComputeAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_attributes: Option<ComputeAttributes>,
    #[serde(rename = "RackId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rack_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetLocation {
    #[serde(rename = "RackElevation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rack_elevation: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComputeAttributes {
    #[serde(rename = "HostId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_id: Option<String>,
    #[serde(rename = "InstanceFamilies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_families: Option<Vec<String>>,
    #[serde(rename = "InstanceTypeCapacities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type_capacities: Option<Vec<AssetInstanceTypeCapacity>>,
    #[serde(rename = "MaxVcpus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_vcpus: Option<i32>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssetInstanceTypeCapacity {
    #[serde(rename = "Count")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBlockingInstancesForCapacityTaskInput {
    #[serde(rename = "CapacityTaskId")]
    #[serde(default)]
    pub capacity_task_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OutpostIdentifier")]
    #[serde(default)]
    pub outpost_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBlockingInstancesForCapacityTaskOutput {
    #[serde(rename = "BlockingInstances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocking_instances: Option<Vec<BlockingInstance>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BlockingInstance {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "AwsServiceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_service_name: Option<String>,
    #[serde(rename = "InstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCapacityTasksInput {
    #[serde(rename = "CapacityTaskStatusFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_task_status_filter: Option<Vec<String>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OutpostIdentifierFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_identifier_filter: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCapacityTasksOutput {
    #[serde(rename = "CapacityTasks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_tasks: Option<Vec<CapacityTaskSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CapacityTaskSummary {
    #[serde(rename = "AssetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    #[serde(rename = "CapacityTaskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_task_id: Option<String>,
    #[serde(rename = "CapacityTaskStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_task_status: Option<String>,
    #[serde(rename = "CompletionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<f64>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "OrderId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "OutpostId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCatalogItemsInput {
    #[serde(rename = "EC2FamilyFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_c2_family_filter: Option<Vec<String>>,
    #[serde(rename = "ItemClassFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_class_filter: Option<Vec<String>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SupportedStorageFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_storage_filter: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCatalogItemsOutput {
    #[serde(rename = "CatalogItems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_items: Option<Vec<CatalogItem>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOrdersInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OutpostIdentifierFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_identifier_filter: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOrdersOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Orders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<OrderSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrderSummary {
    #[serde(rename = "LineItemCountsByStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_item_counts_by_status: Option<std::collections::HashMap<String, i32>>,
    #[serde(rename = "OrderFulfilledDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_fulfilled_date: Option<f64>,
    #[serde(rename = "OrderId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "OrderSubmissionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_submission_date: Option<f64>,
    #[serde(rename = "OrderType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    #[serde(rename = "OutpostId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOutpostsInput {
    #[serde(rename = "AvailabilityZoneFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_filter: Option<Vec<String>>,
    #[serde(rename = "AvailabilityZoneIdFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_id_filter: Option<Vec<String>>,
    #[serde(rename = "LifeCycleStatusFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub life_cycle_status_filter: Option<Vec<String>>,
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
pub struct ListOutpostsOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Outposts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outposts: Option<Vec<Outpost>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSitesInput {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "OperatingAddressCityFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_address_city_filter: Option<Vec<String>>,
    #[serde(rename = "OperatingAddressCountryCodeFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_address_country_code_filter: Option<Vec<String>>,
    #[serde(rename = "OperatingAddressStateOrRegionFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_address_state_or_region_filter: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSitesOutput {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Sites")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sites: Option<Vec<Site>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartCapacityTaskInput {
    #[serde(rename = "AssetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    #[serde(rename = "DryRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "InstancePools")]
    #[serde(default)]
    pub instance_pools: Vec<InstanceTypeCapacity>,
    #[serde(rename = "InstancesToExclude")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_to_exclude: Option<InstancesToExclude>,
    #[serde(rename = "OrderId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "OutpostIdentifier")]
    #[serde(default)]
    pub outpost_identifier: String,
    #[serde(rename = "TaskActionOnBlockingInstances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_action_on_blocking_instances: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartCapacityTaskOutput {
    #[serde(rename = "AssetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    #[serde(rename = "CapacityTaskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_task_id: Option<String>,
    #[serde(rename = "CapacityTaskStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_task_status: Option<String>,
    #[serde(rename = "CompletionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<f64>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "DryRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "Failed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<CapacityTaskFailure>,
    #[serde(rename = "InstancesToExclude")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_to_exclude: Option<InstancesToExclude>,
    #[serde(rename = "LastModifiedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[serde(rename = "OrderId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "OutpostId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost_id: Option<String>,
    #[serde(rename = "RequestedInstancePools")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_instance_pools: Option<Vec<InstanceTypeCapacity>>,
    #[serde(rename = "TaskActionOnBlockingInstances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_action_on_blocking_instances: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartConnectionRequest {
    #[serde(rename = "AssetId")]
    #[serde(default)]
    pub asset_id: String,
    #[serde(rename = "ClientPublicKey")]
    #[serde(default)]
    pub client_public_key: String,
    #[serde(rename = "DeviceSerialNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_serial_number: Option<String>,
    #[serde(rename = "NetworkInterfaceDeviceIndex")]
    #[serde(default)]
    pub network_interface_device_index: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartConnectionResponse {
    #[serde(rename = "ConnectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    #[serde(rename = "UnderlayIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underlay_ip_address: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartOutpostDecommissionInput {
    #[serde(rename = "OutpostIdentifier")]
    #[serde(default)]
    pub outpost_identifier: String,
    #[serde(rename = "ValidateOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_only: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartOutpostDecommissionOutput {
    #[serde(rename = "BlockingResourceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocking_resource_types: Option<Vec<String>>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateOutpostInput {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OutpostId")]
    #[serde(default)]
    pub outpost_id: String,
    #[serde(rename = "SupportedHardwareType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_hardware_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateOutpostOutput {
    #[serde(rename = "Outpost")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outpost: Option<Outpost>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSiteAddressInput {
    #[serde(rename = "Address")]
    #[serde(default)]
    pub address: Address,
    #[serde(rename = "AddressType")]
    #[serde(default)]
    pub address_type: String,
    #[serde(rename = "SiteId")]
    #[serde(default)]
    pub site_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSiteAddressOutput {
    #[serde(rename = "Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(rename = "AddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSiteInput {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Notes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "SiteId")]
    #[serde(default)]
    pub site_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSiteOutput {
    #[serde(rename = "Site")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<Site>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSiteRackPhysicalPropertiesInput {
    #[serde(rename = "FiberOpticCableType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fiber_optic_cable_type: Option<String>,
    #[serde(rename = "MaximumSupportedWeightLbs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_supported_weight_lbs: Option<String>,
    #[serde(rename = "OpticalStandard")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optical_standard: Option<String>,
    #[serde(rename = "PowerConnector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_connector: Option<String>,
    #[serde(rename = "PowerDrawKva")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_draw_kva: Option<String>,
    #[serde(rename = "PowerFeedDrop")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_feed_drop: Option<String>,
    #[serde(rename = "PowerPhase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_phase: Option<String>,
    #[serde(rename = "SiteId")]
    #[serde(default)]
    pub site_id: String,
    #[serde(rename = "UplinkCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uplink_count: Option<String>,
    #[serde(rename = "UplinkGbps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uplink_gbps: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSiteRackPhysicalPropertiesOutput {
    #[serde(rename = "Site")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<Site>,
}
