//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-amplifyuibuilder

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateComponentRequest {
    #[serde(rename = "componentToCreate")]
    #[serde(default)]
    pub component_to_create: CreateComponentData,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateComponentData {
    #[serde(rename = "bindingProperties")]
    #[serde(default)]
    pub binding_properties: std::collections::HashMap<String, ComponentBindingPropertiesValue>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<ComponentChild>>,
    #[serde(rename = "collectionProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_properties:
        Option<std::collections::HashMap<String, ComponentDataConfiguration>>,
    #[serde(rename = "componentType")]
    #[serde(default)]
    pub component_type: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<std::collections::HashMap<String, ComponentEvent>>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub overrides: std::collections::HashMap<String, std::collections::HashMap<String, String>>,
    #[serde(default)]
    pub properties: std::collections::HashMap<String, ComponentProperty>,
    #[serde(rename = "schemaVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,
    #[serde(rename = "sourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    pub variants: Vec<ComponentVariant>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComponentBindingPropertiesValue {
    #[serde(rename = "bindingProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_properties: Option<ComponentBindingPropertiesValueProperties>,
    #[serde(rename = "defaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComponentBindingPropertiesValueProperties {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(rename = "defaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicates: Option<Vec<Predicate>>,
    #[serde(rename = "slotName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_name: Option<String>,
    #[serde(rename = "userAttribute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_attribute: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Predicate {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub and: Option<Vec<Predicate>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operand: Option<String>,
    #[serde(rename = "operandType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operand_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub or: Option<Vec<Predicate>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComponentChild {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<ComponentChild>>,
    #[serde(rename = "componentType")]
    #[serde(default)]
    pub component_type: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<std::collections::HashMap<String, ComponentEvent>>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub properties: std::collections::HashMap<String, ComponentProperty>,
    #[serde(rename = "sourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComponentEvent {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "bindingEvent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_event: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ActionParameters>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionParameters {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<ComponentProperty>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::HashMap<String, ComponentProperty>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global: Option<ComponentProperty>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ComponentProperty>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<MutationActionSetStateParameter>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<ComponentProperty>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ComponentProperty>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<ComponentProperty>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComponentProperty {
    #[serde(rename = "bindingProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_properties: Option<ComponentPropertyBindingProperties>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bindings: Option<std::collections::HashMap<String, FormBindingElement>>,
    #[serde(rename = "collectionBindingProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_binding_properties: Option<ComponentPropertyBindingProperties>,
    #[serde(rename = "componentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concat: Option<Vec<ComponentProperty>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<Box<ComponentConditionProperty>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured: Option<bool>,
    #[serde(rename = "defaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    #[serde(rename = "importedValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_value: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "userAttribute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_attribute: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComponentPropertyBindingProperties {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(default)]
    pub property: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FormBindingElement {
    #[serde(default)]
    pub element: String,
    #[serde(default)]
    pub property: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComponentConditionProperty {
    #[serde(rename = "else")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#else: Option<Box<ComponentProperty>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operand: Option<String>,
    #[serde(rename = "operandType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operand_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub then: Option<Box<ComponentProperty>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MutationActionSetStateParameter {
    #[serde(rename = "componentName")]
    #[serde(default)]
    pub component_name: String,
    #[serde(default)]
    pub property: String,
    #[serde(default)]
    pub set: ComponentProperty,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComponentDataConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<Vec<String>>,
    #[serde(default)]
    pub model: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicate: Option<Predicate>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<Vec<SortProperty>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SortProperty {
    #[serde(default)]
    pub direction: String,
    #[serde(default)]
    pub field: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComponentVariant {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides:
        Option<std::collections::HashMap<String, std::collections::HashMap<String, String>>>,
    #[serde(rename = "variantValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_values: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateComponentResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<Component>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Component {
    #[serde(rename = "appId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "bindingProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_properties:
        Option<std::collections::HashMap<String, ComponentBindingPropertiesValue>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<ComponentChild>>,
    #[serde(rename = "collectionProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_properties:
        Option<std::collections::HashMap<String, ComponentDataConfiguration>>,
    #[serde(rename = "componentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "environmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<std::collections::HashMap<String, ComponentEvent>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides:
        Option<std::collections::HashMap<String, std::collections::HashMap<String, String>>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, ComponentProperty>>,
    #[serde(rename = "schemaVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,
    #[serde(rename = "sourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<ComponentVariant>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFormRequest {
    #[serde(rename = "formToCreate")]
    #[serde(default)]
    pub form_to_create: CreateFormData,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFormData {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cta: Option<FormCTA>,
    #[serde(rename = "dataType")]
    #[serde(default)]
    pub data_type: FormDataTypeConfig,
    #[serde(default)]
    pub fields: std::collections::HashMap<String, FieldConfig>,
    #[serde(rename = "formActionType")]
    #[serde(default)]
    pub form_action_type: String,
    #[serde(rename = "labelDecorator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_decorator: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "schemaVersion")]
    #[serde(default)]
    pub schema_version: String,
    #[serde(rename = "sectionalElements")]
    #[serde(default)]
    pub sectional_elements: std::collections::HashMap<String, SectionalElement>,
    #[serde(default)]
    pub style: FormStyle,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FormCTA {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel: Option<FormButton>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear: Option<FormButton>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit: Option<FormButton>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FormButton {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<FieldPosition>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FieldPosition {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub below: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed: Option<String>,
    #[serde(rename = "rightOf")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_of: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FormDataTypeConfig {
    #[serde(rename = "dataSourceType")]
    #[serde(default)]
    pub data_source_type: String,
    #[serde(rename = "dataTypeName")]
    #[serde(default)]
    pub data_type_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FieldConfig {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded: Option<bool>,
    #[serde(rename = "inputType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_type: Option<FieldInputConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<FieldPosition>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validations: Option<Vec<FieldValidationConfiguration>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FieldInputConfig {
    #[serde(rename = "defaultChecked")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_checked: Option<bool>,
    #[serde(rename = "defaultCountryCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_country_code: Option<String>,
    #[serde(rename = "defaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "descriptiveText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_text: Option<String>,
    #[serde(rename = "fileUploaderConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_uploader_config: Option<FileUploaderFieldConfig>,
    #[serde(rename = "isArray")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_array: Option<bool>,
    #[serde(rename = "maxValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<f32>,
    #[serde(rename = "minValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<f32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    #[serde(rename = "readOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<f32>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "valueMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_mappings: Option<ValueMappings>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FileUploaderFieldConfig {
    #[serde(rename = "acceptedFileTypes")]
    #[serde(default)]
    pub accepted_file_types: Vec<String>,
    #[serde(rename = "accessLevel")]
    #[serde(default)]
    pub access_level: String,
    #[serde(rename = "isResumable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_resumable: Option<bool>,
    #[serde(rename = "maxFileCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_file_count: Option<i32>,
    #[serde(rename = "maxSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<i32>,
    #[serde(rename = "showThumbnails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_thumbnails: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValueMappings {
    #[serde(rename = "bindingProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_properties:
        Option<std::collections::HashMap<String, FormInputBindingPropertiesValue>>,
    #[serde(default)]
    pub values: Vec<ValueMapping>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FormInputBindingPropertiesValue {
    #[serde(rename = "bindingProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_properties: Option<FormInputBindingPropertiesValueProperties>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FormInputBindingPropertiesValueProperties {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValueMapping {
    #[serde(rename = "displayValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_value: Option<FormInputValueProperty>,
    #[serde(default)]
    pub value: FormInputValueProperty,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FormInputValueProperty {
    #[serde(rename = "bindingProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_properties: Option<FormInputValuePropertyBindingProperties>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concat: Option<Vec<FormInputValueProperty>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FormInputValuePropertyBindingProperties {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(default)]
    pub property: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FieldValidationConfiguration {
    #[serde(rename = "numValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_values: Option<Vec<i32>>,
    #[serde(rename = "strValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub str_values: Option<Vec<String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "validationMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SectionalElement {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<FieldPosition>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FormStyle {
    #[serde(rename = "horizontalGap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_gap: Option<FormStyleConfig>,
    #[serde(rename = "outerPadding")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_padding: Option<FormStyleConfig>,
    #[serde(rename = "verticalGap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_gap: Option<FormStyleConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FormStyleConfig {
    #[serde(rename = "tokenReference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_reference: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFormResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<Form>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Form {
    #[serde(rename = "appId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cta: Option<FormCTA>,
    #[serde(rename = "dataType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<FormDataTypeConfig>,
    #[serde(rename = "environmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::HashMap<String, FieldConfig>>,
    #[serde(rename = "formActionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_action_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "labelDecorator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_decorator: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "schemaVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,
    #[serde(rename = "sectionalElements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sectional_elements: Option<std::collections::HashMap<String, SectionalElement>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<FormStyle>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateThemeRequest {
    #[serde(rename = "themeToCreate")]
    #[serde(default)]
    pub theme_to_create: CreateThemeData,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateThemeData {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<Vec<ThemeValues>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    pub values: Vec<ThemeValues>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThemeValues {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Box<ThemeValue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThemeValue {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<ThemeValues>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateThemeResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<Theme>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Theme {
    #[serde(rename = "appId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "environmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<Vec<ThemeValues>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<ThemeValues>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteComponentRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFormRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteThemeRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExchangeCodeForTokenRequest {
    #[serde(default)]
    pub request: ExchangeCodeForTokenRequestBody,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExchangeCodeForTokenRequestBody {
    #[serde(rename = "clientId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(default)]
    pub code: String,
    #[serde(rename = "redirectUri")]
    #[serde(default)]
    pub redirect_uri: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExchangeCodeForTokenResponse {
    #[serde(rename = "accessToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "expiresIn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<i32>,
    #[serde(rename = "refreshToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportComponentsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportComponentsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<Component>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportFormsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportFormsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<Form>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportThemesRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExportThemesResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<Theme>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCodegenJobRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCodegenJobResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job: Option<CodegenJob>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodegenJob {
    #[serde(rename = "appId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset: Option<CodegenJobAsset>,
    #[serde(rename = "autoGenerateForms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_generate_forms: Option<bool>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<CodegenDependency>>,
    #[serde(rename = "environmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CodegenFeatureFlags>,
    #[serde(rename = "genericDataSchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generic_data_schema: Option<CodegenJobGenericDataSchema>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(rename = "renderConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_config: Option<CodegenJobRenderConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodegenJobAsset {
    #[serde(rename = "downloadUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodegenDependency {
    #[serde(rename = "isSemVer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_sem_ver: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "supportedVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodegenFeatureFlags {
    #[serde(rename = "isNonModelSupported")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_non_model_supported: Option<bool>,
    #[serde(rename = "isRelationshipSupported")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_relationship_supported: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodegenJobGenericDataSchema {
    #[serde(rename = "dataSourceType")]
    #[serde(default)]
    pub data_source_type: String,
    #[serde(default)]
    pub enums: std::collections::HashMap<String, CodegenGenericDataEnum>,
    #[serde(default)]
    pub models: std::collections::HashMap<String, CodegenGenericDataModel>,
    #[serde(rename = "nonModels")]
    #[serde(default)]
    pub non_models: std::collections::HashMap<String, CodegenGenericDataNonModel>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodegenGenericDataEnum {
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodegenGenericDataModel {
    #[serde(default)]
    pub fields: std::collections::HashMap<String, CodegenGenericDataField>,
    #[serde(rename = "isJoinTable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_join_table: Option<bool>,
    #[serde(rename = "primaryKeys")]
    #[serde(default)]
    pub primary_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodegenGenericDataField {
    #[serde(rename = "dataType")]
    #[serde(default)]
    pub data_type: String,
    #[serde(rename = "dataTypeValue")]
    #[serde(default)]
    pub data_type_value: String,
    #[serde(rename = "isArray")]
    #[serde(default)]
    pub is_array: bool,
    #[serde(rename = "readOnly")]
    #[serde(default)]
    pub read_only: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<CodegenGenericDataRelationshipType>,
    #[serde(default)]
    pub required: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodegenGenericDataRelationshipType {
    #[serde(rename = "associatedFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_fields: Option<Vec<String>>,
    #[serde(rename = "belongsToFieldOnRelatedModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub belongs_to_field_on_related_model: Option<String>,
    #[serde(rename = "canUnlinkAssociatedModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_unlink_associated_model: Option<bool>,
    #[serde(rename = "isHasManyIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_has_many_index: Option<bool>,
    #[serde(rename = "relatedJoinFieldName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_join_field_name: Option<String>,
    #[serde(rename = "relatedJoinTableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_join_table_name: Option<String>,
    #[serde(rename = "relatedModelFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_model_fields: Option<Vec<String>>,
    #[serde(rename = "relatedModelName")]
    #[serde(default)]
    pub related_model_name: String,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodegenGenericDataNonModel {
    #[serde(default)]
    pub fields: std::collections::HashMap<String, CodegenGenericDataField>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodegenJobRenderConfig {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub react: Option<ReactStartCodegenJobData>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReactStartCodegenJobData {
    #[serde(rename = "apiConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_configuration: Option<ApiConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "inlineSourceMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_source_map: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<String>,
    #[serde(rename = "renderTypeDeclarations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub render_type_declarations: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApiConfiguration {
    #[serde(rename = "dataStoreConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_store_config: Option<DataStoreRenderConfig>,
    #[serde(rename = "graphQLConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph_q_l_config: Option<GraphQLRenderConfig>,
    #[serde(rename = "noApiConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_api_config: Option<NoApiRenderConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataStoreRenderConfig {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GraphQLRenderConfig {
    #[serde(rename = "fragmentsFilePath")]
    #[serde(default)]
    pub fragments_file_path: String,
    #[serde(rename = "mutationsFilePath")]
    #[serde(default)]
    pub mutations_file_path: String,
    #[serde(rename = "queriesFilePath")]
    #[serde(default)]
    pub queries_file_path: String,
    #[serde(rename = "subscriptionsFilePath")]
    #[serde(default)]
    pub subscriptions_file_path: String,
    #[serde(rename = "typesFilePath")]
    #[serde(default)]
    pub types_file_path: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NoApiRenderConfig {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetComponentRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetComponentResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<Component>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFormRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFormResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<Form>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMetadataRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetMetadataResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetThemeRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetThemeResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<Theme>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCodegenJobsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListCodegenJobsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<CodegenJobSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodegenJobSummary {
    #[serde(rename = "appId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "environmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "modifiedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListComponentsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListComponentsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<ComponentSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComponentSummary {
    #[serde(rename = "appId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "componentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<String>,
    #[serde(rename = "environmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFormsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFormsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<FormSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FormSummary {
    #[serde(rename = "appId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "dataType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<FormDataTypeConfig>,
    #[serde(rename = "environmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    #[serde(rename = "formActionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_action_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListThemesRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListThemesResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<ThemeSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ThemeSummary {
    #[serde(rename = "appId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "environmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutMetadataFlagRequest {
    #[serde(default)]
    pub body: PutMetadataFlagBody,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutMetadataFlagBody {
    #[serde(rename = "newValue")]
    #[serde(default)]
    pub new_value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RefreshTokenRequest {
    #[serde(rename = "refreshTokenBody")]
    #[serde(default)]
    pub refresh_token_body: RefreshTokenRequestBody,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RefreshTokenRequestBody {
    #[serde(rename = "clientId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(default)]
    pub token: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RefreshTokenResponse {
    #[serde(rename = "accessToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "expiresIn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartCodegenJobRequest {
    #[serde(rename = "codegenJobToCreate")]
    #[serde(default)]
    pub codegen_job_to_create: StartCodegenJobData,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartCodegenJobData {
    #[serde(rename = "autoGenerateForms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_generate_forms: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<CodegenFeatureFlags>,
    #[serde(rename = "genericDataSchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generic_data_schema: Option<CodegenJobGenericDataSchema>,
    #[serde(rename = "renderConfig")]
    #[serde(default)]
    pub render_config: CodegenJobRenderConfig,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartCodegenJobResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<CodegenJob>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateComponentRequest {
    #[serde(rename = "updatedComponent")]
    #[serde(default)]
    pub updated_component: UpdateComponentData,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateComponentData {
    #[serde(rename = "bindingProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binding_properties:
        Option<std::collections::HashMap<String, ComponentBindingPropertiesValue>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<ComponentChild>>,
    #[serde(rename = "collectionProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_properties:
        Option<std::collections::HashMap<String, ComponentDataConfiguration>>,
    #[serde(rename = "componentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<std::collections::HashMap<String, ComponentEvent>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides:
        Option<std::collections::HashMap<String, std::collections::HashMap<String, String>>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<std::collections::HashMap<String, ComponentProperty>>,
    #[serde(rename = "schemaVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,
    #[serde(rename = "sourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<ComponentVariant>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateComponentResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<Component>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFormRequest {
    #[serde(rename = "updatedForm")]
    #[serde(default)]
    pub updated_form: UpdateFormData,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFormData {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cta: Option<FormCTA>,
    #[serde(rename = "dataType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<FormDataTypeConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::HashMap<String, FieldConfig>>,
    #[serde(rename = "formActionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_action_type: Option<String>,
    #[serde(rename = "labelDecorator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_decorator: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "schemaVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,
    #[serde(rename = "sectionalElements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sectional_elements: Option<std::collections::HashMap<String, SectionalElement>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<FormStyle>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFormResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<Form>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateThemeRequest {
    #[serde(rename = "updatedTheme")]
    #[serde(default)]
    pub updated_theme: UpdateThemeData,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateThemeData {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<Vec<ThemeValues>>,
    #[serde(default)]
    pub values: Vec<ThemeValues>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateThemeResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<Theme>,
}
