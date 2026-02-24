//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-appsync

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

#[allow(unused_imports)]
use http::header::HeaderName;
use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for restJson protocol.
pub fn serialize_associate_api_response(result: &AssociateApiResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_associate_merged_graphql_api_response(
    result: &AssociateMergedGraphqlApiResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_associate_source_graphql_api_response(
    result: &AssociateSourceGraphqlApiResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_api_response(result: &CreateApiResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_api_cache_response(result: &CreateApiCacheResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_api_key_response(result: &CreateApiKeyResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_channel_namespace_response(
    result: &CreateChannelNamespaceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_data_source_response(result: &CreateDataSourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_domain_name_response(result: &CreateDomainNameResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_function_response(result: &CreateFunctionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_graphql_api_response(result: &CreateGraphqlApiResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_resolver_response(result: &CreateResolverResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_create_type_response(result: &CreateTypeResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_api_response(result: &DeleteApiResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_api_cache_response(result: &DeleteApiCacheResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_api_key_response(result: &DeleteApiKeyResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_channel_namespace_response(
    result: &DeleteChannelNamespaceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_data_source_response(result: &DeleteDataSourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_domain_name_response(result: &DeleteDomainNameResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_function_response(result: &DeleteFunctionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_graphql_api_response(result: &DeleteGraphqlApiResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_resolver_response(result: &DeleteResolverResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_delete_type_response(result: &DeleteTypeResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_disassociate_api_response(result: &DisassociateApiResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_disassociate_merged_graphql_api_response(
    result: &DisassociateMergedGraphqlApiResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_disassociate_source_graphql_api_response(
    result: &DisassociateSourceGraphqlApiResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_evaluate_code_response(result: &EvaluateCodeResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_evaluate_mapping_template_response(
    result: &EvaluateMappingTemplateResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_flush_api_cache_response(result: &FlushApiCacheResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_api_response(result: &GetApiResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_api_association_response(result: &GetApiAssociationResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_api_cache_response(result: &GetApiCacheResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_channel_namespace_response(
    result: &GetChannelNamespaceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_data_source_response(result: &GetDataSourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_data_source_introspection_response(
    result: &GetDataSourceIntrospectionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_domain_name_response(result: &GetDomainNameResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_function_response(result: &GetFunctionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_graphql_api_response(result: &GetGraphqlApiResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_graphql_api_environment_variables_response(
    result: &GetGraphqlApiEnvironmentVariablesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_introspection_schema_response(
    result: &GetIntrospectionSchemaResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_resolver_response(result: &GetResolverResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_schema_creation_status_response(
    result: &GetSchemaCreationStatusResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_source_api_association_response(
    result: &GetSourceApiAssociationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_get_type_response(result: &GetTypeResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_api_keys_response(result: &ListApiKeysResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_apis_response(result: &ListApisResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_channel_namespaces_response(
    result: &ListChannelNamespacesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_data_sources_response(result: &ListDataSourcesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_domain_names_response(result: &ListDomainNamesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_functions_response(result: &ListFunctionsResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_graphql_apis_response(result: &ListGraphqlApisResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_resolvers_response(result: &ListResolversResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_resolvers_by_function_response(
    result: &ListResolversByFunctionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_source_api_associations_response(
    result: &ListSourceApiAssociationsResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_tags_for_resource_response(
    result: &ListTagsForResourceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_types_response(result: &ListTypesResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_list_types_by_association_response(
    result: &ListTypesByAssociationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_put_graphql_api_environment_variables_response(
    result: &PutGraphqlApiEnvironmentVariablesResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_data_source_introspection_response(
    result: &StartDataSourceIntrospectionResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_schema_creation_response(
    result: &StartSchemaCreationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_start_schema_merge_response(result: &StartSchemaMergeResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_tag_resource_response(result: &TagResourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_untag_resource_response(result: &UntagResourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_api_response(result: &UpdateApiResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_api_cache_response(result: &UpdateApiCacheResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_api_key_response(result: &UpdateApiKeyResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_channel_namespace_response(
    result: &UpdateChannelNamespaceResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_data_source_response(result: &UpdateDataSourceResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_domain_name_response(result: &UpdateDomainNameResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_function_response(result: &UpdateFunctionResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_graphql_api_response(result: &UpdateGraphqlApiResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_resolver_response(result: &UpdateResolverResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_source_api_association_response(
    result: &UpdateSourceApiAssociationResponse,
) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Serialize response for restJson protocol.
pub fn serialize_update_type_response(result: &UpdateTypeResponse) -> MockResponse {
    let status = 200_u16;
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::rest_json(status, body)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_api_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateApiRequest, String> {
    let mut input = AssociateApiRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateApiRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize AssociateApi request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "domainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_merged_graphql_api_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateMergedGraphqlApiRequest, String> {
    let mut input = AssociateMergedGraphqlApiRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateMergedGraphqlApiRequest>(&request.body).map_err(
            |err| format!("failed to deserialize AssociateMergedGraphqlApi request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "sourceApiIdentifier" => {
                input.source_api_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_associate_source_graphql_api_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<AssociateSourceGraphqlApiRequest, String> {
    let mut input = AssociateSourceGraphqlApiRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<AssociateSourceGraphqlApiRequest>(&request.body).map_err(
            |err| format!("failed to deserialize AssociateSourceGraphqlApi request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "mergedApiIdentifier" => {
                input.merged_api_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_api_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateApiRequest, String> {
    let mut input = CreateApiRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateApiRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateApi request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_api_cache_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateApiCacheRequest, String> {
    let mut input = CreateApiCacheRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateApiCacheRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateApiCache request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_api_key_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateApiKeyRequest, String> {
    let mut input = CreateApiKeyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateApiKeyRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateApiKey request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_channel_namespace_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateChannelNamespaceRequest, String> {
    let mut input = CreateChannelNamespaceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateChannelNamespaceRequest>(&request.body).map_err(
            |err| format!("failed to deserialize CreateChannelNamespace request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_data_source_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateDataSourceRequest, String> {
    let mut input = CreateDataSourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateDataSourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateDataSource request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_domain_name_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateDomainNameRequest, String> {
    let mut input = CreateDomainNameRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateDomainNameRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateDomainName request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_function_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateFunctionRequest, String> {
    let mut input = CreateFunctionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateFunctionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateFunction request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_graphql_api_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateGraphqlApiRequest, String> {
    let mut input = CreateGraphqlApiRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateGraphqlApiRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateGraphqlApi request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_resolver_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateResolverRequest, String> {
    let mut input = CreateResolverRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateResolverRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateResolver request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            "typeName" => {
                input.type_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_create_type_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<CreateTypeRequest, String> {
    let mut input = CreateTypeRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<CreateTypeRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize CreateType request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_api_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteApiRequest, String> {
    let mut input = DeleteApiRequest::default();
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_api_cache_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteApiCacheRequest, String> {
    let mut input = DeleteApiCacheRequest::default();
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_api_key_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteApiKeyRequest, String> {
    let mut input = DeleteApiKeyRequest::default();
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_channel_namespace_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteChannelNamespaceRequest, String> {
    let mut input = DeleteChannelNamespaceRequest::default();
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            "name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_data_source_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteDataSourceRequest, String> {
    let mut input = DeleteDataSourceRequest::default();
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            "name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_domain_name_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteDomainNameRequest, String> {
    let mut input = DeleteDomainNameRequest::default();
    for (name, value) in labels {
        match *name {
            "domainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_function_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteFunctionRequest, String> {
    let mut input = DeleteFunctionRequest::default();
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            "functionId" => {
                input.function_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_graphql_api_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteGraphqlApiRequest, String> {
    let mut input = DeleteGraphqlApiRequest::default();
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_resolver_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteResolverRequest, String> {
    let mut input = DeleteResolverRequest::default();
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            "fieldName" => {
                input.field_name = value.to_string();
            }
            "typeName" => {
                input.type_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_delete_type_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DeleteTypeRequest, String> {
    let mut input = DeleteTypeRequest::default();
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            "typeName" => {
                input.type_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_api_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateApiRequest, String> {
    let mut input = DisassociateApiRequest::default();
    for (name, value) in labels {
        match *name {
            "domainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_merged_graphql_api_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateMergedGraphqlApiRequest, String> {
    let mut input = DisassociateMergedGraphqlApiRequest::default();
    for (name, value) in labels {
        match *name {
            "associationId" => {
                input.association_id = value.to_string();
            }
            "sourceApiIdentifier" => {
                input.source_api_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_disassociate_source_graphql_api_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<DisassociateSourceGraphqlApiRequest, String> {
    let mut input = DisassociateSourceGraphqlApiRequest::default();
    for (name, value) in labels {
        match *name {
            "associationId" => {
                input.association_id = value.to_string();
            }
            "mergedApiIdentifier" => {
                input.merged_api_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_evaluate_code_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<EvaluateCodeRequest, String> {
    let mut input = EvaluateCodeRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<EvaluateCodeRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize EvaluateCode request: {err}"))?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_evaluate_mapping_template_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<EvaluateMappingTemplateRequest, String> {
    let mut input = EvaluateMappingTemplateRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<EvaluateMappingTemplateRequest>(&request.body).map_err(
            |err| format!("failed to deserialize EvaluateMappingTemplate request: {err}"),
        )?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_flush_api_cache_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<FlushApiCacheRequest, String> {
    let mut input = FlushApiCacheRequest::default();
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_api_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetApiRequest, String> {
    let mut input = GetApiRequest::default();
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_api_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetApiAssociationRequest, String> {
    let mut input = GetApiAssociationRequest::default();
    for (name, value) in labels {
        match *name {
            "domainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_api_cache_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetApiCacheRequest, String> {
    let mut input = GetApiCacheRequest::default();
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_channel_namespace_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetChannelNamespaceRequest, String> {
    let mut input = GetChannelNamespaceRequest::default();
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            "name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_data_source_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDataSourceRequest, String> {
    let mut input = GetDataSourceRequest::default();
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            "name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_data_source_introspection_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDataSourceIntrospectionRequest, String> {
    let mut input = GetDataSourceIntrospectionRequest::default();
    for (name, value) in labels {
        match *name {
            "introspectionId" => {
                input.introspection_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("includeModelsSDL") {
        input.include_models_s_d_l = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_domain_name_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetDomainNameRequest, String> {
    let mut input = GetDomainNameRequest::default();
    for (name, value) in labels {
        match *name {
            "domainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_function_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetFunctionRequest, String> {
    let mut input = GetFunctionRequest::default();
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            "functionId" => {
                input.function_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_graphql_api_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetGraphqlApiRequest, String> {
    let mut input = GetGraphqlApiRequest::default();
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_graphql_api_environment_variables_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetGraphqlApiEnvironmentVariablesRequest, String> {
    let mut input = GetGraphqlApiEnvironmentVariablesRequest::default();
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_introspection_schema_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetIntrospectionSchemaRequest, String> {
    let mut input = GetIntrospectionSchemaRequest::default();
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("format") {
        input.format = value.to_string();
    }
    if let Some(value) = query.get("includeDirectives") {
        input.include_directives = Some(
            value
                .parse::<bool>()
                .map_err(|err| format!("failed to parse bool: {err}"))?,
        );
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_resolver_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetResolverRequest, String> {
    let mut input = GetResolverRequest::default();
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            "fieldName" => {
                input.field_name = value.to_string();
            }
            "typeName" => {
                input.type_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_schema_creation_status_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetSchemaCreationStatusRequest, String> {
    let mut input = GetSchemaCreationStatusRequest::default();
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_source_api_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetSourceApiAssociationRequest, String> {
    let mut input = GetSourceApiAssociationRequest::default();
    for (name, value) in labels {
        match *name {
            "associationId" => {
                input.association_id = value.to_string();
            }
            "mergedApiIdentifier" => {
                input.merged_api_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_get_type_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<GetTypeRequest, String> {
    let mut input = GetTypeRequest::default();
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            "typeName" => {
                input.type_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("format") {
        input.format = value.to_string();
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_api_keys_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListApiKeysRequest, String> {
    let mut input = ListApiKeysRequest::default();
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_apis_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListApisRequest, String> {
    let mut input = ListApisRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_channel_namespaces_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListChannelNamespacesRequest, String> {
    let mut input = ListChannelNamespacesRequest::default();
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_data_sources_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDataSourcesRequest, String> {
    let mut input = ListDataSourcesRequest::default();
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_domain_names_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListDomainNamesRequest, String> {
    let mut input = ListDomainNamesRequest::default();
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_functions_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListFunctionsRequest, String> {
    let mut input = ListFunctionsRequest::default();
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_graphql_apis_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListGraphqlApisRequest, String> {
    let mut input = ListGraphqlApisRequest::default();
    if let Some(value) = query.get("apiType") {
        input.api_type = Some(value.to_string());
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    if let Some(value) = query.get("owner") {
        input.owner = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_resolvers_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListResolversRequest, String> {
    let mut input = ListResolversRequest::default();
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            "typeName" => {
                input.type_name = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_resolvers_by_function_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListResolversByFunctionRequest, String> {
    let mut input = ListResolversByFunctionRequest::default();
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            "functionId" => {
                input.function_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_source_api_associations_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListSourceApiAssociationsRequest, String> {
    let mut input = ListSourceApiAssociationsRequest::default();
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_tags_for_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTagsForResourceRequest, String> {
    let mut input = ListTagsForResourceRequest::default();
    for (name, value) in labels {
        match *name {
            "resourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_types_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTypesRequest, String> {
    let mut input = ListTypesRequest::default();
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("format") {
        input.format = value.to_string();
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_list_types_by_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<ListTypesByAssociationRequest, String> {
    let mut input = ListTypesByAssociationRequest::default();
    for (name, value) in labels {
        match *name {
            "associationId" => {
                input.association_id = value.to_string();
            }
            "mergedApiIdentifier" => {
                input.merged_api_identifier = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("format") {
        input.format = value.to_string();
    }
    if let Some(value) = query.get("maxResults") {
        input.max_results = Some(
            value
                .parse::<i32>()
                .map_err(|err| format!("failed to parse integer: {err}"))?,
        );
    }
    if let Some(value) = query.get("nextToken") {
        input.next_token = Some(value.to_string());
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_put_graphql_api_environment_variables_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<PutGraphqlApiEnvironmentVariablesRequest, String> {
    let mut input = PutGraphqlApiEnvironmentVariablesRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<PutGraphqlApiEnvironmentVariablesRequest>(&request.body)
            .map_err(|err| {
            format!("failed to deserialize PutGraphqlApiEnvironmentVariables request: {err}")
        })?;
    }
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_data_source_introspection_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartDataSourceIntrospectionRequest, String> {
    let mut input = StartDataSourceIntrospectionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartDataSourceIntrospectionRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize StartDataSourceIntrospection request: {err}")
            })?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_schema_creation_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartSchemaCreationRequest, String> {
    let mut input = StartSchemaCreationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<StartSchemaCreationRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize StartSchemaCreation request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_start_schema_merge_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<StartSchemaMergeRequest, String> {
    let mut input = StartSchemaMergeRequest::default();
    for (name, value) in labels {
        match *name {
            "associationId" => {
                input.association_id = value.to_string();
            }
            "mergedApiIdentifier" => {
                input.merged_api_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_tag_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<TagResourceRequest, String> {
    let mut input = TagResourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<TagResourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize TagResource request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "resourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_untag_resource_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UntagResourceRequest, String> {
    let mut input = UntagResourceRequest::default();
    for (name, value) in labels {
        match *name {
            "resourceArn" => {
                input.resource_arn = value.to_string();
            }
            _ => {}
        }
    }
    if let Some(value) = query.get("tagKeys") {
        input.tag_keys = value
            .split(',')
            .filter(|item| !item.trim().is_empty())
            .map(|item| Ok(item.trim().to_string()))
            .collect::<Result<Vec<_>, String>>()?;
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_api_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateApiRequest, String> {
    let mut input = UpdateApiRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateApiRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateApi request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_api_cache_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateApiCacheRequest, String> {
    let mut input = UpdateApiCacheRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateApiCacheRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateApiCache request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_api_key_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateApiKeyRequest, String> {
    let mut input = UpdateApiKeyRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateApiKeyRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateApiKey request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            "id" => {
                input.id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_channel_namespace_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateChannelNamespaceRequest, String> {
    let mut input = UpdateChannelNamespaceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateChannelNamespaceRequest>(&request.body).map_err(
            |err| format!("failed to deserialize UpdateChannelNamespace request: {err}"),
        )?;
    }
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            "name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_data_source_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDataSourceRequest, String> {
    let mut input = UpdateDataSourceRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateDataSourceRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateDataSource request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            "name" => {
                input.name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_domain_name_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateDomainNameRequest, String> {
    let mut input = UpdateDomainNameRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateDomainNameRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateDomainName request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "domainName" => {
                input.domain_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_function_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateFunctionRequest, String> {
    let mut input = UpdateFunctionRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateFunctionRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateFunction request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            "functionId" => {
                input.function_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_graphql_api_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateGraphqlApiRequest, String> {
    let mut input = UpdateGraphqlApiRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateGraphqlApiRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateGraphqlApi request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_resolver_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateResolverRequest, String> {
    let mut input = UpdateResolverRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateResolverRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateResolver request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            "fieldName" => {
                input.field_name = value.to_string();
            }
            "typeName" => {
                input.type_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_source_api_association_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateSourceApiAssociationRequest, String> {
    let mut input = UpdateSourceApiAssociationRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateSourceApiAssociationRequest>(&request.body)
            .map_err(|err| {
                format!("failed to deserialize UpdateSourceApiAssociation request: {err}")
            })?;
    }
    for (name, value) in labels {
        match *name {
            "associationId" => {
                input.association_id = value.to_string();
            }
            "mergedApiIdentifier" => {
                input.merged_api_identifier = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}

/// Deserialize request for restJson protocol.
pub fn deserialize_update_type_request(
    request: &winterbaume_core::MockRequest,
    labels: &[(&str, &str)],
    query: &std::collections::HashMap<String, String>,
) -> Result<UpdateTypeRequest, String> {
    let mut input = UpdateTypeRequest::default();
    if !request.body.is_empty() {
        input = serde_json::from_slice::<UpdateTypeRequest>(&request.body)
            .map_err(|err| format!("failed to deserialize UpdateType request: {err}"))?;
    }
    for (name, value) in labels {
        match *name {
            "apiId" => {
                input.api_id = value.to_string();
            }
            "typeName" => {
                input.type_name = value.to_string();
            }
            _ => {}
        }
    }
    Ok(input)
}
