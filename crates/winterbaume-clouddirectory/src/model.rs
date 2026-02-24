//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-clouddirectory

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddFacetToObjectRequest {
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    pub directory_arn: String,
    #[serde(rename = "ObjectAttributeList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_attribute_list: Option<Vec<AttributeKeyAndValue>>,
    #[serde(rename = "ObjectReference")]
    #[serde(default)]
    pub object_reference: ObjectReference,
    #[serde(rename = "SchemaFacet")]
    #[serde(default)]
    pub schema_facet: SchemaFacet,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttributeKeyAndValue {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: AttributeKey,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: TypedAttributeValue,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttributeKey {
    #[serde(rename = "FacetName")]
    #[serde(default)]
    pub facet_name: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "SchemaArn")]
    #[serde(default)]
    pub schema_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TypedAttributeValue {
    #[serde(rename = "BinaryValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_value: Option<String>,
    #[serde(rename = "BooleanValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boolean_value: Option<bool>,
    #[serde(rename = "DatetimeValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datetime_value: Option<f64>,
    #[serde(rename = "NumberValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_value: Option<String>,
    #[serde(rename = "StringValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ObjectReference {
    #[serde(rename = "Selector")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SchemaFacet {
    #[serde(rename = "FacetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facet_name: Option<String>,
    #[serde(rename = "SchemaArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddFacetToObjectResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplySchemaRequest {
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    pub directory_arn: String,
    #[serde(rename = "PublishedSchemaArn")]
    #[serde(default)]
    pub published_schema_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplySchemaResponse {
    #[serde(rename = "AppliedSchemaArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_schema_arn: Option<String>,
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttachObjectRequest {
    #[serde(rename = "ChildReference")]
    #[serde(default)]
    pub child_reference: ObjectReference,
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    pub directory_arn: String,
    #[serde(rename = "LinkName")]
    #[serde(default)]
    pub link_name: String,
    #[serde(rename = "ParentReference")]
    #[serde(default)]
    pub parent_reference: ObjectReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttachObjectResponse {
    #[serde(rename = "AttachedObjectIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_object_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttachPolicyRequest {
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    pub directory_arn: String,
    #[serde(rename = "ObjectReference")]
    #[serde(default)]
    pub object_reference: ObjectReference,
    #[serde(rename = "PolicyReference")]
    #[serde(default)]
    pub policy_reference: ObjectReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttachPolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttachToIndexRequest {
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    pub directory_arn: String,
    #[serde(rename = "IndexReference")]
    #[serde(default)]
    pub index_reference: ObjectReference,
    #[serde(rename = "TargetReference")]
    #[serde(default)]
    pub target_reference: ObjectReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttachToIndexResponse {
    #[serde(rename = "AttachedObjectIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_object_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttachTypedLinkRequest {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    pub attributes: Vec<AttributeNameAndValue>,
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    pub directory_arn: String,
    #[serde(rename = "SourceObjectReference")]
    #[serde(default)]
    pub source_object_reference: ObjectReference,
    #[serde(rename = "TargetObjectReference")]
    #[serde(default)]
    pub target_object_reference: ObjectReference,
    #[serde(rename = "TypedLinkFacet")]
    #[serde(default)]
    pub typed_link_facet: TypedLinkSchemaAndFacetName,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttributeNameAndValue {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    pub attribute_name: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: TypedAttributeValue,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TypedLinkSchemaAndFacetName {
    #[serde(rename = "SchemaArn")]
    #[serde(default)]
    pub schema_arn: String,
    #[serde(rename = "TypedLinkName")]
    #[serde(default)]
    pub typed_link_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttachTypedLinkResponse {
    #[serde(rename = "TypedLinkSpecifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typed_link_specifier: Option<TypedLinkSpecifier>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TypedLinkSpecifier {
    #[serde(rename = "IdentityAttributeValues")]
    #[serde(default)]
    pub identity_attribute_values: Vec<AttributeNameAndValue>,
    #[serde(rename = "SourceObjectReference")]
    #[serde(default)]
    pub source_object_reference: ObjectReference,
    #[serde(rename = "TargetObjectReference")]
    #[serde(default)]
    pub target_object_reference: ObjectReference,
    #[serde(rename = "TypedLinkFacet")]
    #[serde(default)]
    pub typed_link_facet: TypedLinkSchemaAndFacetName,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchReadRequest {
    #[serde(rename = "ConsistencyLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency_level: Option<String>,
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    pub directory_arn: String,
    #[serde(rename = "Operations")]
    #[serde(default)]
    pub operations: Vec<BatchReadOperation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchReadOperation {
    #[serde(rename = "GetLinkAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_link_attributes: Option<BatchGetLinkAttributes>,
    #[serde(rename = "GetObjectAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_object_attributes: Option<BatchGetObjectAttributes>,
    #[serde(rename = "GetObjectInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_object_information: Option<BatchGetObjectInformation>,
    #[serde(rename = "ListAttachedIndices")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_attached_indices: Option<BatchListAttachedIndices>,
    #[serde(rename = "ListIncomingTypedLinks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_incoming_typed_links: Option<BatchListIncomingTypedLinks>,
    #[serde(rename = "ListIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_index: Option<BatchListIndex>,
    #[serde(rename = "ListObjectAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_object_attributes: Option<BatchListObjectAttributes>,
    #[serde(rename = "ListObjectChildren")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_object_children: Option<BatchListObjectChildren>,
    #[serde(rename = "ListObjectParentPaths")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_object_parent_paths: Option<BatchListObjectParentPaths>,
    #[serde(rename = "ListObjectParents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_object_parents: Option<BatchListObjectParents>,
    #[serde(rename = "ListObjectPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_object_policies: Option<BatchListObjectPolicies>,
    #[serde(rename = "ListOutgoingTypedLinks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_outgoing_typed_links: Option<BatchListOutgoingTypedLinks>,
    #[serde(rename = "ListPolicyAttachments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_policy_attachments: Option<BatchListPolicyAttachments>,
    #[serde(rename = "LookupPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup_policy: Option<BatchLookupPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetLinkAttributes {
    #[serde(rename = "AttributeNames")]
    #[serde(default)]
    pub attribute_names: Vec<String>,
    #[serde(rename = "TypedLinkSpecifier")]
    #[serde(default)]
    pub typed_link_specifier: TypedLinkSpecifier,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetObjectAttributes {
    #[serde(rename = "AttributeNames")]
    #[serde(default)]
    pub attribute_names: Vec<String>,
    #[serde(rename = "ObjectReference")]
    #[serde(default)]
    pub object_reference: ObjectReference,
    #[serde(rename = "SchemaFacet")]
    #[serde(default)]
    pub schema_facet: SchemaFacet,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetObjectInformation {
    #[serde(rename = "ObjectReference")]
    #[serde(default)]
    pub object_reference: ObjectReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchListAttachedIndices {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TargetReference")]
    #[serde(default)]
    pub target_reference: ObjectReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchListIncomingTypedLinks {
    #[serde(rename = "FilterAttributeRanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_attribute_ranges: Option<Vec<TypedLinkAttributeRange>>,
    #[serde(rename = "FilterTypedLink")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_typed_link: Option<TypedLinkSchemaAndFacetName>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ObjectReference")]
    #[serde(default)]
    pub object_reference: ObjectReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TypedLinkAttributeRange {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[serde(rename = "Range")]
    #[serde(default)]
    pub range: TypedAttributeValueRange,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TypedAttributeValueRange {
    #[serde(rename = "EndMode")]
    #[serde(default)]
    pub end_mode: String,
    #[serde(rename = "EndValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_value: Option<TypedAttributeValue>,
    #[serde(rename = "StartMode")]
    #[serde(default)]
    pub start_mode: String,
    #[serde(rename = "StartValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_value: Option<TypedAttributeValue>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchListIndex {
    #[serde(rename = "IndexReference")]
    #[serde(default)]
    pub index_reference: ObjectReference,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RangesOnIndexedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranges_on_indexed_values: Option<Vec<ObjectAttributeRange>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ObjectAttributeRange {
    #[serde(rename = "AttributeKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_key: Option<AttributeKey>,
    #[serde(rename = "Range")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<TypedAttributeValueRange>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchListObjectAttributes {
    #[serde(rename = "FacetFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facet_filter: Option<SchemaFacet>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ObjectReference")]
    #[serde(default)]
    pub object_reference: ObjectReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchListObjectChildren {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ObjectReference")]
    #[serde(default)]
    pub object_reference: ObjectReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchListObjectParentPaths {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ObjectReference")]
    #[serde(default)]
    pub object_reference: ObjectReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchListObjectParents {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ObjectReference")]
    #[serde(default)]
    pub object_reference: ObjectReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchListObjectPolicies {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ObjectReference")]
    #[serde(default)]
    pub object_reference: ObjectReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchListOutgoingTypedLinks {
    #[serde(rename = "FilterAttributeRanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_attribute_ranges: Option<Vec<TypedLinkAttributeRange>>,
    #[serde(rename = "FilterTypedLink")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_typed_link: Option<TypedLinkSchemaAndFacetName>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ObjectReference")]
    #[serde(default)]
    pub object_reference: ObjectReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchListPolicyAttachments {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PolicyReference")]
    #[serde(default)]
    pub policy_reference: ObjectReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchLookupPolicy {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ObjectReference")]
    #[serde(default)]
    pub object_reference: ObjectReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchReadResponse {
    #[serde(rename = "Responses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<Vec<BatchReadOperationResponse>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchReadOperationResponse {
    #[serde(rename = "ExceptionResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exception_response: Option<BatchReadException>,
    #[serde(rename = "SuccessfulResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_response: Option<BatchReadSuccessfulResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchReadException {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchReadSuccessfulResponse {
    #[serde(rename = "GetLinkAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_link_attributes: Option<BatchGetLinkAttributesResponse>,
    #[serde(rename = "GetObjectAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_object_attributes: Option<BatchGetObjectAttributesResponse>,
    #[serde(rename = "GetObjectInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_object_information: Option<BatchGetObjectInformationResponse>,
    #[serde(rename = "ListAttachedIndices")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_attached_indices: Option<BatchListAttachedIndicesResponse>,
    #[serde(rename = "ListIncomingTypedLinks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_incoming_typed_links: Option<BatchListIncomingTypedLinksResponse>,
    #[serde(rename = "ListIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_index: Option<BatchListIndexResponse>,
    #[serde(rename = "ListObjectAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_object_attributes: Option<BatchListObjectAttributesResponse>,
    #[serde(rename = "ListObjectChildren")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_object_children: Option<BatchListObjectChildrenResponse>,
    #[serde(rename = "ListObjectParentPaths")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_object_parent_paths: Option<BatchListObjectParentPathsResponse>,
    #[serde(rename = "ListObjectParents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_object_parents: Option<BatchListObjectParentsResponse>,
    #[serde(rename = "ListObjectPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_object_policies: Option<BatchListObjectPoliciesResponse>,
    #[serde(rename = "ListOutgoingTypedLinks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_outgoing_typed_links: Option<BatchListOutgoingTypedLinksResponse>,
    #[serde(rename = "ListPolicyAttachments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_policy_attachments: Option<BatchListPolicyAttachmentsResponse>,
    #[serde(rename = "LookupPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup_policy: Option<BatchLookupPolicyResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetLinkAttributesResponse {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<AttributeKeyAndValue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetObjectAttributesResponse {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<AttributeKeyAndValue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetObjectInformationResponse {
    #[serde(rename = "ObjectIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_identifier: Option<String>,
    #[serde(rename = "SchemaFacets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_facets: Option<Vec<SchemaFacet>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchListAttachedIndicesResponse {
    #[serde(rename = "IndexAttachments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_attachments: Option<Vec<IndexAttachment>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IndexAttachment {
    #[serde(rename = "IndexedAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexed_attributes: Option<Vec<AttributeKeyAndValue>>,
    #[serde(rename = "ObjectIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchListIncomingTypedLinksResponse {
    #[serde(rename = "LinkSpecifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_specifiers: Option<Vec<TypedLinkSpecifier>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchListIndexResponse {
    #[serde(rename = "IndexAttachments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_attachments: Option<Vec<IndexAttachment>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchListObjectAttributesResponse {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<AttributeKeyAndValue>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchListObjectChildrenResponse {
    #[serde(rename = "Children")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchListObjectParentPathsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PathToObjectIdentifiersList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_to_object_identifiers_list: Option<Vec<PathToObjectIdentifiers>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PathToObjectIdentifiers {
    #[serde(rename = "ObjectIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_identifiers: Option<Vec<String>>,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchListObjectParentsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ParentLinks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_links: Option<Vec<ObjectIdentifierAndLinkNameTuple>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ObjectIdentifierAndLinkNameTuple {
    #[serde(rename = "LinkName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_name: Option<String>,
    #[serde(rename = "ObjectIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchListObjectPoliciesResponse {
    #[serde(rename = "AttachedPolicyIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_policy_ids: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchListOutgoingTypedLinksResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TypedLinkSpecifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typed_link_specifiers: Option<Vec<TypedLinkSpecifier>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchListPolicyAttachmentsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ObjectIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_identifiers: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchLookupPolicyResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PolicyToPathList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_to_path_list: Option<Vec<PolicyToPath>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PolicyToPath {
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "Policies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<PolicyAttachment>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PolicyAttachment {
    #[serde(rename = "ObjectIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_identifier: Option<String>,
    #[serde(rename = "PolicyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[serde(rename = "PolicyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchWriteRequest {
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    pub directory_arn: String,
    #[serde(rename = "Operations")]
    #[serde(default)]
    pub operations: Vec<BatchWriteOperation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchWriteOperation {
    #[serde(rename = "AddFacetToObject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_facet_to_object: Option<BatchAddFacetToObject>,
    #[serde(rename = "AttachObject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_object: Option<BatchAttachObject>,
    #[serde(rename = "AttachPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_policy: Option<BatchAttachPolicy>,
    #[serde(rename = "AttachToIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_to_index: Option<BatchAttachToIndex>,
    #[serde(rename = "AttachTypedLink")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_typed_link: Option<BatchAttachTypedLink>,
    #[serde(rename = "CreateIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_index: Option<BatchCreateIndex>,
    #[serde(rename = "CreateObject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_object: Option<BatchCreateObject>,
    #[serde(rename = "DeleteObject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_object: Option<BatchDeleteObject>,
    #[serde(rename = "DetachFromIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach_from_index: Option<BatchDetachFromIndex>,
    #[serde(rename = "DetachObject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach_object: Option<BatchDetachObject>,
    #[serde(rename = "DetachPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach_policy: Option<BatchDetachPolicy>,
    #[serde(rename = "DetachTypedLink")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach_typed_link: Option<BatchDetachTypedLink>,
    #[serde(rename = "RemoveFacetFromObject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_facet_from_object: Option<BatchRemoveFacetFromObject>,
    #[serde(rename = "UpdateLinkAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_link_attributes: Option<BatchUpdateLinkAttributes>,
    #[serde(rename = "UpdateObjectAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_object_attributes: Option<BatchUpdateObjectAttributes>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchAddFacetToObject {
    #[serde(rename = "ObjectAttributeList")]
    #[serde(default)]
    pub object_attribute_list: Vec<AttributeKeyAndValue>,
    #[serde(rename = "ObjectReference")]
    #[serde(default)]
    pub object_reference: ObjectReference,
    #[serde(rename = "SchemaFacet")]
    #[serde(default)]
    pub schema_facet: SchemaFacet,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchAttachObject {
    #[serde(rename = "ChildReference")]
    #[serde(default)]
    pub child_reference: ObjectReference,
    #[serde(rename = "LinkName")]
    #[serde(default)]
    pub link_name: String,
    #[serde(rename = "ParentReference")]
    #[serde(default)]
    pub parent_reference: ObjectReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchAttachPolicy {
    #[serde(rename = "ObjectReference")]
    #[serde(default)]
    pub object_reference: ObjectReference,
    #[serde(rename = "PolicyReference")]
    #[serde(default)]
    pub policy_reference: ObjectReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchAttachToIndex {
    #[serde(rename = "IndexReference")]
    #[serde(default)]
    pub index_reference: ObjectReference,
    #[serde(rename = "TargetReference")]
    #[serde(default)]
    pub target_reference: ObjectReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchAttachTypedLink {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    pub attributes: Vec<AttributeNameAndValue>,
    #[serde(rename = "SourceObjectReference")]
    #[serde(default)]
    pub source_object_reference: ObjectReference,
    #[serde(rename = "TargetObjectReference")]
    #[serde(default)]
    pub target_object_reference: ObjectReference,
    #[serde(rename = "TypedLinkFacet")]
    #[serde(default)]
    pub typed_link_facet: TypedLinkSchemaAndFacetName,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchCreateIndex {
    #[serde(rename = "BatchReferenceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_reference_name: Option<String>,
    #[serde(rename = "IsUnique")]
    #[serde(default)]
    pub is_unique: bool,
    #[serde(rename = "LinkName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_name: Option<String>,
    #[serde(rename = "OrderedIndexedAttributeList")]
    #[serde(default)]
    pub ordered_indexed_attribute_list: Vec<AttributeKey>,
    #[serde(rename = "ParentReference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_reference: Option<ObjectReference>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchCreateObject {
    #[serde(rename = "BatchReferenceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_reference_name: Option<String>,
    #[serde(rename = "LinkName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_name: Option<String>,
    #[serde(rename = "ObjectAttributeList")]
    #[serde(default)]
    pub object_attribute_list: Vec<AttributeKeyAndValue>,
    #[serde(rename = "ParentReference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_reference: Option<ObjectReference>,
    #[serde(rename = "SchemaFacet")]
    #[serde(default)]
    pub schema_facet: Vec<SchemaFacet>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteObject {
    #[serde(rename = "ObjectReference")]
    #[serde(default)]
    pub object_reference: ObjectReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDetachFromIndex {
    #[serde(rename = "IndexReference")]
    #[serde(default)]
    pub index_reference: ObjectReference,
    #[serde(rename = "TargetReference")]
    #[serde(default)]
    pub target_reference: ObjectReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDetachObject {
    #[serde(rename = "BatchReferenceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_reference_name: Option<String>,
    #[serde(rename = "LinkName")]
    #[serde(default)]
    pub link_name: String,
    #[serde(rename = "ParentReference")]
    #[serde(default)]
    pub parent_reference: ObjectReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDetachPolicy {
    #[serde(rename = "ObjectReference")]
    #[serde(default)]
    pub object_reference: ObjectReference,
    #[serde(rename = "PolicyReference")]
    #[serde(default)]
    pub policy_reference: ObjectReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDetachTypedLink {
    #[serde(rename = "TypedLinkSpecifier")]
    #[serde(default)]
    pub typed_link_specifier: TypedLinkSpecifier,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchRemoveFacetFromObject {
    #[serde(rename = "ObjectReference")]
    #[serde(default)]
    pub object_reference: ObjectReference,
    #[serde(rename = "SchemaFacet")]
    #[serde(default)]
    pub schema_facet: SchemaFacet,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateLinkAttributes {
    #[serde(rename = "AttributeUpdates")]
    #[serde(default)]
    pub attribute_updates: Vec<LinkAttributeUpdate>,
    #[serde(rename = "TypedLinkSpecifier")]
    #[serde(default)]
    pub typed_link_specifier: TypedLinkSpecifier,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LinkAttributeUpdate {
    #[serde(rename = "AttributeAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_action: Option<LinkAttributeAction>,
    #[serde(rename = "AttributeKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_key: Option<AttributeKey>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LinkAttributeAction {
    #[serde(rename = "AttributeActionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_action_type: Option<String>,
    #[serde(rename = "AttributeUpdateValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_update_value: Option<TypedAttributeValue>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateObjectAttributes {
    #[serde(rename = "AttributeUpdates")]
    #[serde(default)]
    pub attribute_updates: Vec<ObjectAttributeUpdate>,
    #[serde(rename = "ObjectReference")]
    #[serde(default)]
    pub object_reference: ObjectReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ObjectAttributeUpdate {
    #[serde(rename = "ObjectAttributeAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_attribute_action: Option<ObjectAttributeAction>,
    #[serde(rename = "ObjectAttributeKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_attribute_key: Option<AttributeKey>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ObjectAttributeAction {
    #[serde(rename = "ObjectAttributeActionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_attribute_action_type: Option<String>,
    #[serde(rename = "ObjectAttributeUpdateValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_attribute_update_value: Option<TypedAttributeValue>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchWriteResponse {
    #[serde(rename = "Responses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<Vec<BatchWriteOperationResponse>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchWriteOperationResponse {
    #[serde(rename = "AddFacetToObject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_facet_to_object: Option<BatchAddFacetToObjectResponse>,
    #[serde(rename = "AttachObject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_object: Option<BatchAttachObjectResponse>,
    #[serde(rename = "AttachPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_policy: Option<BatchAttachPolicyResponse>,
    #[serde(rename = "AttachToIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_to_index: Option<BatchAttachToIndexResponse>,
    #[serde(rename = "AttachTypedLink")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_typed_link: Option<BatchAttachTypedLinkResponse>,
    #[serde(rename = "CreateIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_index: Option<BatchCreateIndexResponse>,
    #[serde(rename = "CreateObject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_object: Option<BatchCreateObjectResponse>,
    #[serde(rename = "DeleteObject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_object: Option<BatchDeleteObjectResponse>,
    #[serde(rename = "DetachFromIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach_from_index: Option<BatchDetachFromIndexResponse>,
    #[serde(rename = "DetachObject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach_object: Option<BatchDetachObjectResponse>,
    #[serde(rename = "DetachPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach_policy: Option<BatchDetachPolicyResponse>,
    #[serde(rename = "DetachTypedLink")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach_typed_link: Option<BatchDetachTypedLinkResponse>,
    #[serde(rename = "RemoveFacetFromObject")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_facet_from_object: Option<BatchRemoveFacetFromObjectResponse>,
    #[serde(rename = "UpdateLinkAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_link_attributes: Option<BatchUpdateLinkAttributesResponse>,
    #[serde(rename = "UpdateObjectAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_object_attributes: Option<BatchUpdateObjectAttributesResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchAddFacetToObjectResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchAttachObjectResponse {
    #[serde(rename = "attachedObjectIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_object_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchAttachPolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchAttachToIndexResponse {
    #[serde(rename = "AttachedObjectIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_object_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchAttachTypedLinkResponse {
    #[serde(rename = "TypedLinkSpecifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typed_link_specifier: Option<TypedLinkSpecifier>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchCreateIndexResponse {
    #[serde(rename = "ObjectIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchCreateObjectResponse {
    #[serde(rename = "ObjectIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDeleteObjectResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDetachFromIndexResponse {
    #[serde(rename = "DetachedObjectIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detached_object_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDetachObjectResponse {
    #[serde(rename = "detachedObjectIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detached_object_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDetachPolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchDetachTypedLinkResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchRemoveFacetFromObjectResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateLinkAttributesResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchUpdateObjectAttributesResponse {
    #[serde(rename = "ObjectIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDirectoryRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "SchemaArn")]
    #[serde(default)]
    pub schema_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDirectoryResponse {
    #[serde(rename = "AppliedSchemaArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_schema_arn: Option<String>,
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ObjectIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFacetRequest {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<FacetAttribute>>,
    #[serde(rename = "FacetStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facet_style: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ObjectType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    #[serde(rename = "SchemaArn")]
    #[serde(default)]
    pub schema_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FacetAttribute {
    #[serde(rename = "AttributeDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_definition: Option<FacetAttributeDefinition>,
    #[serde(rename = "AttributeReference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_reference: Option<FacetAttributeReference>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RequiredBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_behavior: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FacetAttributeDefinition {
    #[serde(rename = "DefaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<TypedAttributeValue>,
    #[serde(rename = "IsImmutable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_immutable: Option<bool>,
    #[serde(rename = "Rules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<std::collections::HashMap<String, Rule>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Rule {
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FacetAttributeReference {
    #[serde(rename = "TargetAttributeName")]
    #[serde(default)]
    pub target_attribute_name: String,
    #[serde(rename = "TargetFacetName")]
    #[serde(default)]
    pub target_facet_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFacetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIndexRequest {
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    pub directory_arn: String,
    #[serde(rename = "IsUnique")]
    #[serde(default)]
    pub is_unique: bool,
    #[serde(rename = "LinkName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_name: Option<String>,
    #[serde(rename = "OrderedIndexedAttributeList")]
    #[serde(default)]
    pub ordered_indexed_attribute_list: Vec<AttributeKey>,
    #[serde(rename = "ParentReference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_reference: Option<ObjectReference>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIndexResponse {
    #[serde(rename = "ObjectIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateObjectRequest {
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    pub directory_arn: String,
    #[serde(rename = "LinkName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_name: Option<String>,
    #[serde(rename = "ObjectAttributeList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_attribute_list: Option<Vec<AttributeKeyAndValue>>,
    #[serde(rename = "ParentReference")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_reference: Option<ObjectReference>,
    #[serde(rename = "SchemaFacets")]
    #[serde(default)]
    pub schema_facets: Vec<SchemaFacet>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateObjectResponse {
    #[serde(rename = "ObjectIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSchemaRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSchemaResponse {
    #[serde(rename = "SchemaArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTypedLinkFacetRequest {
    #[serde(rename = "Facet")]
    #[serde(default)]
    pub facet: TypedLinkFacet,
    #[serde(rename = "SchemaArn")]
    #[serde(default)]
    pub schema_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TypedLinkFacet {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    pub attributes: Vec<TypedLinkAttributeDefinition>,
    #[serde(rename = "IdentityAttributeOrder")]
    #[serde(default)]
    pub identity_attribute_order: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TypedLinkAttributeDefinition {
    #[serde(rename = "DefaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<TypedAttributeValue>,
    #[serde(rename = "IsImmutable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_immutable: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RequiredBehavior")]
    #[serde(default)]
    pub required_behavior: String,
    #[serde(rename = "Rules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<std::collections::HashMap<String, Rule>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTypedLinkFacetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDirectoryRequest {
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    pub directory_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDirectoryResponse {
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFacetRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "SchemaArn")]
    #[serde(default)]
    pub schema_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFacetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteObjectRequest {
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    pub directory_arn: String,
    #[serde(rename = "ObjectReference")]
    #[serde(default)]
    pub object_reference: ObjectReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteObjectResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSchemaRequest {
    #[serde(rename = "SchemaArn")]
    #[serde(default)]
    pub schema_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSchemaResponse {
    #[serde(rename = "SchemaArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTypedLinkFacetRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "SchemaArn")]
    #[serde(default)]
    pub schema_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTypedLinkFacetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetachFromIndexRequest {
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    pub directory_arn: String,
    #[serde(rename = "IndexReference")]
    #[serde(default)]
    pub index_reference: ObjectReference,
    #[serde(rename = "TargetReference")]
    #[serde(default)]
    pub target_reference: ObjectReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetachFromIndexResponse {
    #[serde(rename = "DetachedObjectIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detached_object_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetachObjectRequest {
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    pub directory_arn: String,
    #[serde(rename = "LinkName")]
    #[serde(default)]
    pub link_name: String,
    #[serde(rename = "ParentReference")]
    #[serde(default)]
    pub parent_reference: ObjectReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetachObjectResponse {
    #[serde(rename = "DetachedObjectIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detached_object_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetachPolicyRequest {
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    pub directory_arn: String,
    #[serde(rename = "ObjectReference")]
    #[serde(default)]
    pub object_reference: ObjectReference,
    #[serde(rename = "PolicyReference")]
    #[serde(default)]
    pub policy_reference: ObjectReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetachPolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetachTypedLinkRequest {
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    pub directory_arn: String,
    #[serde(rename = "TypedLinkSpecifier")]
    #[serde(default)]
    pub typed_link_specifier: TypedLinkSpecifier,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableDirectoryRequest {
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    pub directory_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableDirectoryResponse {
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableDirectoryRequest {
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    pub directory_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableDirectoryResponse {
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAppliedSchemaVersionRequest {
    #[serde(rename = "SchemaArn")]
    #[serde(default)]
    pub schema_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAppliedSchemaVersionResponse {
    #[serde(rename = "AppliedSchemaArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_schema_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDirectoryRequest {
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    pub directory_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDirectoryResponse {
    #[serde(rename = "Directory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory: Option<Directory>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Directory {
    #[serde(rename = "CreationDateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFacetRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "SchemaArn")]
    #[serde(default)]
    pub schema_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFacetResponse {
    #[serde(rename = "Facet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facet: Option<Facet>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Facet {
    #[serde(rename = "FacetStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facet_style: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ObjectType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLinkAttributesRequest {
    #[serde(rename = "AttributeNames")]
    #[serde(default)]
    pub attribute_names: Vec<String>,
    #[serde(rename = "ConsistencyLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency_level: Option<String>,
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    pub directory_arn: String,
    #[serde(rename = "TypedLinkSpecifier")]
    #[serde(default)]
    pub typed_link_specifier: TypedLinkSpecifier,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetLinkAttributesResponse {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<AttributeKeyAndValue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetObjectAttributesRequest {
    #[serde(rename = "AttributeNames")]
    #[serde(default)]
    pub attribute_names: Vec<String>,
    #[serde(rename = "ConsistencyLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency_level: Option<String>,
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    pub directory_arn: String,
    #[serde(rename = "ObjectReference")]
    #[serde(default)]
    pub object_reference: ObjectReference,
    #[serde(rename = "SchemaFacet")]
    #[serde(default)]
    pub schema_facet: SchemaFacet,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetObjectAttributesResponse {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<AttributeKeyAndValue>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetObjectInformationRequest {
    #[serde(rename = "ConsistencyLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency_level: Option<String>,
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    pub directory_arn: String,
    #[serde(rename = "ObjectReference")]
    #[serde(default)]
    pub object_reference: ObjectReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetObjectInformationResponse {
    #[serde(rename = "ObjectIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_identifier: Option<String>,
    #[serde(rename = "SchemaFacets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_facets: Option<Vec<SchemaFacet>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSchemaAsJsonRequest {
    #[serde(rename = "SchemaArn")]
    #[serde(default)]
    pub schema_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSchemaAsJsonResponse {
    #[serde(rename = "Document")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTypedLinkFacetInformationRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "SchemaArn")]
    #[serde(default)]
    pub schema_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTypedLinkFacetInformationResponse {
    #[serde(rename = "IdentityAttributeOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_attribute_order: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAppliedSchemaArnsRequest {
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    pub directory_arn: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SchemaArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAppliedSchemaArnsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SchemaArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arns: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAttachedIndicesRequest {
    #[serde(rename = "ConsistencyLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency_level: Option<String>,
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    pub directory_arn: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TargetReference")]
    #[serde(default)]
    pub target_reference: ObjectReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAttachedIndicesResponse {
    #[serde(rename = "IndexAttachments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_attachments: Option<Vec<IndexAttachment>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDevelopmentSchemaArnsRequest {
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
pub struct ListDevelopmentSchemaArnsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SchemaArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arns: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDirectoriesRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDirectoriesResponse {
    #[serde(rename = "Directories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directories: Option<Vec<Directory>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFacetAttributesRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SchemaArn")]
    #[serde(default)]
    pub schema_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFacetAttributesResponse {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<FacetAttribute>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFacetNamesRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SchemaArn")]
    #[serde(default)]
    pub schema_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFacetNamesResponse {
    #[serde(rename = "FacetNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facet_names: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIncomingTypedLinksRequest {
    #[serde(rename = "ConsistencyLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency_level: Option<String>,
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    pub directory_arn: String,
    #[serde(rename = "FilterAttributeRanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_attribute_ranges: Option<Vec<TypedLinkAttributeRange>>,
    #[serde(rename = "FilterTypedLink")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_typed_link: Option<TypedLinkSchemaAndFacetName>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ObjectReference")]
    #[serde(default)]
    pub object_reference: ObjectReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIncomingTypedLinksResponse {
    #[serde(rename = "LinkSpecifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_specifiers: Option<Vec<TypedLinkSpecifier>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIndexRequest {
    #[serde(rename = "ConsistencyLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency_level: Option<String>,
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    pub directory_arn: String,
    #[serde(rename = "IndexReference")]
    #[serde(default)]
    pub index_reference: ObjectReference,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RangesOnIndexedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranges_on_indexed_values: Option<Vec<ObjectAttributeRange>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIndexResponse {
    #[serde(rename = "IndexAttachments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_attachments: Option<Vec<IndexAttachment>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListManagedSchemaArnsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SchemaArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListManagedSchemaArnsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SchemaArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arns: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListObjectAttributesRequest {
    #[serde(rename = "ConsistencyLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency_level: Option<String>,
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    pub directory_arn: String,
    #[serde(rename = "FacetFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facet_filter: Option<SchemaFacet>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ObjectReference")]
    #[serde(default)]
    pub object_reference: ObjectReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListObjectAttributesResponse {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<AttributeKeyAndValue>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListObjectChildrenRequest {
    #[serde(rename = "ConsistencyLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency_level: Option<String>,
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    pub directory_arn: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ObjectReference")]
    #[serde(default)]
    pub object_reference: ObjectReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListObjectChildrenResponse {
    #[serde(rename = "Children")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListObjectParentPathsRequest {
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    pub directory_arn: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ObjectReference")]
    #[serde(default)]
    pub object_reference: ObjectReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListObjectParentPathsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PathToObjectIdentifiersList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_to_object_identifiers_list: Option<Vec<PathToObjectIdentifiers>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListObjectParentsRequest {
    #[serde(rename = "ConsistencyLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency_level: Option<String>,
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    pub directory_arn: String,
    #[serde(rename = "IncludeAllLinksToEachParent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_all_links_to_each_parent: Option<bool>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ObjectReference")]
    #[serde(default)]
    pub object_reference: ObjectReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListObjectParentsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ParentLinks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_links: Option<Vec<ObjectIdentifierAndLinkNameTuple>>,
    #[serde(rename = "Parents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parents: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListObjectPoliciesRequest {
    #[serde(rename = "ConsistencyLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency_level: Option<String>,
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    pub directory_arn: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ObjectReference")]
    #[serde(default)]
    pub object_reference: ObjectReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListObjectPoliciesResponse {
    #[serde(rename = "AttachedPolicyIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_policy_ids: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOutgoingTypedLinksRequest {
    #[serde(rename = "ConsistencyLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency_level: Option<String>,
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    pub directory_arn: String,
    #[serde(rename = "FilterAttributeRanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_attribute_ranges: Option<Vec<TypedLinkAttributeRange>>,
    #[serde(rename = "FilterTypedLink")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_typed_link: Option<TypedLinkSchemaAndFacetName>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ObjectReference")]
    #[serde(default)]
    pub object_reference: ObjectReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListOutgoingTypedLinksResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TypedLinkSpecifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typed_link_specifiers: Option<Vec<TypedLinkSpecifier>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPolicyAttachmentsRequest {
    #[serde(rename = "ConsistencyLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consistency_level: Option<String>,
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    pub directory_arn: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PolicyReference")]
    #[serde(default)]
    pub policy_reference: ObjectReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPolicyAttachmentsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ObjectIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_identifiers: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPublishedSchemaArnsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SchemaArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPublishedSchemaArnsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SchemaArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arns: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
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
pub struct ListTypedLinkFacetAttributesRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SchemaArn")]
    #[serde(default)]
    pub schema_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTypedLinkFacetAttributesResponse {
    #[serde(rename = "Attributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<TypedLinkAttributeDefinition>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTypedLinkFacetNamesRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SchemaArn")]
    #[serde(default)]
    pub schema_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTypedLinkFacetNamesResponse {
    #[serde(rename = "FacetNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facet_names: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LookupPolicyRequest {
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    pub directory_arn: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ObjectReference")]
    #[serde(default)]
    pub object_reference: ObjectReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LookupPolicyResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PolicyToPathList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_to_path_list: Option<Vec<PolicyToPath>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PublishSchemaRequest {
    #[serde(rename = "DevelopmentSchemaArn")]
    #[serde(default)]
    pub development_schema_arn: String,
    #[serde(rename = "MinorVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor_version: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Version")]
    #[serde(default)]
    pub version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PublishSchemaResponse {
    #[serde(rename = "PublishedSchemaArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_schema_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutSchemaFromJsonRequest {
    #[serde(rename = "Document")]
    #[serde(default)]
    pub document: String,
    #[serde(rename = "SchemaArn")]
    #[serde(default)]
    pub schema_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutSchemaFromJsonResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveFacetFromObjectRequest {
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    pub directory_arn: String,
    #[serde(rename = "ObjectReference")]
    #[serde(default)]
    pub object_reference: ObjectReference,
    #[serde(rename = "SchemaFacet")]
    #[serde(default)]
    pub schema_facet: SchemaFacet,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveFacetFromObjectResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
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
pub struct UpdateFacetRequest {
    #[serde(rename = "AttributeUpdates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_updates: Option<Vec<FacetAttributeUpdate>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ObjectType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    #[serde(rename = "SchemaArn")]
    #[serde(default)]
    pub schema_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FacetAttributeUpdate {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "Attribute")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<FacetAttribute>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFacetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLinkAttributesRequest {
    #[serde(rename = "AttributeUpdates")]
    #[serde(default)]
    pub attribute_updates: Vec<LinkAttributeUpdate>,
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    pub directory_arn: String,
    #[serde(rename = "TypedLinkSpecifier")]
    #[serde(default)]
    pub typed_link_specifier: TypedLinkSpecifier,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateLinkAttributesResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateObjectAttributesRequest {
    #[serde(rename = "AttributeUpdates")]
    #[serde(default)]
    pub attribute_updates: Vec<ObjectAttributeUpdate>,
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    pub directory_arn: String,
    #[serde(rename = "ObjectReference")]
    #[serde(default)]
    pub object_reference: ObjectReference,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateObjectAttributesResponse {
    #[serde(rename = "ObjectIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSchemaRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "SchemaArn")]
    #[serde(default)]
    pub schema_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSchemaResponse {
    #[serde(rename = "SchemaArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTypedLinkFacetRequest {
    #[serde(rename = "AttributeUpdates")]
    #[serde(default)]
    pub attribute_updates: Vec<TypedLinkFacetAttributeUpdate>,
    #[serde(rename = "IdentityAttributeOrder")]
    #[serde(default)]
    pub identity_attribute_order: Vec<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "SchemaArn")]
    #[serde(default)]
    pub schema_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TypedLinkFacetAttributeUpdate {
    #[serde(rename = "Action")]
    #[serde(default)]
    pub action: String,
    #[serde(rename = "Attribute")]
    #[serde(default)]
    pub attribute: TypedLinkAttributeDefinition,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTypedLinkFacetResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpgradeAppliedSchemaRequest {
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    pub directory_arn: String,
    #[serde(rename = "DryRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "PublishedSchemaArn")]
    #[serde(default)]
    pub published_schema_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpgradeAppliedSchemaResponse {
    #[serde(rename = "DirectoryArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_arn: Option<String>,
    #[serde(rename = "UpgradedSchemaArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgraded_schema_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpgradePublishedSchemaRequest {
    #[serde(rename = "DevelopmentSchemaArn")]
    #[serde(default)]
    pub development_schema_arn: String,
    #[serde(rename = "DryRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "MinorVersion")]
    #[serde(default)]
    pub minor_version: String,
    #[serde(rename = "PublishedSchemaArn")]
    #[serde(default)]
    pub published_schema_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpgradePublishedSchemaResponse {
    #[serde(rename = "UpgradedSchemaArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgraded_schema_arn: Option<String>,
}
