//! Auto-generated wire helpers from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-cognitoidentity

#![allow(
    dead_code,
    unused_variables,
    clippy::let_and_return,
    clippy::single_match
)]

use winterbaume_core::MockResponse;

pub use super::model::*;

/// Serialize response for awsJson protocol.
pub fn serialize_create_identity_pool_response(result: &IdentityPool) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_delete_identities_response(result: &DeleteIdentitiesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_delete_identity_pool_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_identity_response(result: &IdentityDescription) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_describe_identity_pool_response(result: &IdentityPool) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_credentials_for_identity_response(
    result: &GetCredentialsForIdentityResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_id_response(result: &GetIdResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_identity_pool_roles_response(
    result: &GetIdentityPoolRolesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_open_id_token_response(result: &GetOpenIdTokenResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_open_id_token_for_developer_identity_response(
    result: &GetOpenIdTokenForDeveloperIdentityResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_get_principal_tag_attribute_map_response(
    result: &GetPrincipalTagAttributeMapResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_identities_response(result: &ListIdentitiesResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_identity_pools_response(result: &ListIdentityPoolsResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_list_tags_for_resource_response(
    result: &ListTagsForResourceResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_lookup_developer_identity_response(
    result: &LookupDeveloperIdentityResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_merge_developer_identities_response(
    result: &MergeDeveloperIdentitiesResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_set_identity_pool_roles_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_set_principal_tag_attribute_map_response(
    result: &SetPrincipalTagAttributeMapResponse,
) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_tag_resource_response(result: &TagResourceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize void response for awsJson protocol.
pub fn serialize_unlink_developer_identity_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize void response for awsJson protocol.
pub fn serialize_unlink_identity_response() -> MockResponse {
    MockResponse::json(200, "{}")
}

/// Serialize response for awsJson protocol.
pub fn serialize_untag_resource_response(result: &UntagResourceResponse) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Serialize response for awsJson protocol.
pub fn serialize_update_identity_pool_response(result: &IdentityPool) -> MockResponse {
    let body = serde_json::to_string(result).unwrap_or_else(|_| "{}".to_string());
    MockResponse::json(200, body)
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_create_identity_pool_request(
    body: &[u8],
) -> Result<CreateIdentityPoolInput, String> {
    if body.is_empty() {
        return Ok(CreateIdentityPoolInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize CreateIdentityPool request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_identities_request(body: &[u8]) -> Result<DeleteIdentitiesInput, String> {
    if body.is_empty() {
        return Ok(DeleteIdentitiesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteIdentities request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_delete_identity_pool_request(
    body: &[u8],
) -> Result<DeleteIdentityPoolInput, String> {
    if body.is_empty() {
        return Ok(DeleteIdentityPoolInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DeleteIdentityPool request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_identity_request(body: &[u8]) -> Result<DescribeIdentityInput, String> {
    if body.is_empty() {
        return Ok(DescribeIdentityInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeIdentity request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_describe_identity_pool_request(
    body: &[u8],
) -> Result<DescribeIdentityPoolInput, String> {
    if body.is_empty() {
        return Ok(DescribeIdentityPoolInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize DescribeIdentityPool request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_credentials_for_identity_request(
    body: &[u8],
) -> Result<GetCredentialsForIdentityInput, String> {
    if body.is_empty() {
        return Ok(GetCredentialsForIdentityInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetCredentialsForIdentity request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_id_request(body: &[u8]) -> Result<GetIdInput, String> {
    if body.is_empty() {
        return Ok(GetIdInput::default());
    }
    serde_json::from_slice(body).map_err(|e| format!("Failed to deserialize GetId request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_identity_pool_roles_request(
    body: &[u8],
) -> Result<GetIdentityPoolRolesInput, String> {
    if body.is_empty() {
        return Ok(GetIdentityPoolRolesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetIdentityPoolRoles request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_open_id_token_request(body: &[u8]) -> Result<GetOpenIdTokenInput, String> {
    if body.is_empty() {
        return Ok(GetOpenIdTokenInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetOpenIdToken request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_open_id_token_for_developer_identity_request(
    body: &[u8],
) -> Result<GetOpenIdTokenForDeveloperIdentityInput, String> {
    if body.is_empty() {
        return Ok(GetOpenIdTokenForDeveloperIdentityInput::default());
    }
    serde_json::from_slice(body).map_err(|e| {
        format!("Failed to deserialize GetOpenIdTokenForDeveloperIdentity request: {e}")
    })
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_get_principal_tag_attribute_map_request(
    body: &[u8],
) -> Result<GetPrincipalTagAttributeMapInput, String> {
    if body.is_empty() {
        return Ok(GetPrincipalTagAttributeMapInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize GetPrincipalTagAttributeMap request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_identities_request(body: &[u8]) -> Result<ListIdentitiesInput, String> {
    if body.is_empty() {
        return Ok(ListIdentitiesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListIdentities request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_identity_pools_request(
    body: &[u8],
) -> Result<ListIdentityPoolsInput, String> {
    if body.is_empty() {
        return Ok(ListIdentityPoolsInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListIdentityPools request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_list_tags_for_resource_request(
    body: &[u8],
) -> Result<ListTagsForResourceInput, String> {
    if body.is_empty() {
        return Ok(ListTagsForResourceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize ListTagsForResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_lookup_developer_identity_request(
    body: &[u8],
) -> Result<LookupDeveloperIdentityInput, String> {
    if body.is_empty() {
        return Ok(LookupDeveloperIdentityInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize LookupDeveloperIdentity request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_merge_developer_identities_request(
    body: &[u8],
) -> Result<MergeDeveloperIdentitiesInput, String> {
    if body.is_empty() {
        return Ok(MergeDeveloperIdentitiesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize MergeDeveloperIdentities request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_set_identity_pool_roles_request(
    body: &[u8],
) -> Result<SetIdentityPoolRolesInput, String> {
    if body.is_empty() {
        return Ok(SetIdentityPoolRolesInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SetIdentityPoolRoles request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_set_principal_tag_attribute_map_request(
    body: &[u8],
) -> Result<SetPrincipalTagAttributeMapInput, String> {
    if body.is_empty() {
        return Ok(SetPrincipalTagAttributeMapInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize SetPrincipalTagAttributeMap request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_tag_resource_request(body: &[u8]) -> Result<TagResourceInput, String> {
    if body.is_empty() {
        return Ok(TagResourceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize TagResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_unlink_developer_identity_request(
    body: &[u8],
) -> Result<UnlinkDeveloperIdentityInput, String> {
    if body.is_empty() {
        return Ok(UnlinkDeveloperIdentityInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UnlinkDeveloperIdentity request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_unlink_identity_request(body: &[u8]) -> Result<UnlinkIdentityInput, String> {
    if body.is_empty() {
        return Ok(UnlinkIdentityInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UnlinkIdentity request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_untag_resource_request(body: &[u8]) -> Result<UntagResourceInput, String> {
    if body.is_empty() {
        return Ok(UntagResourceInput::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UntagResource request: {e}"))
}

/// Deserialize request for awsJson protocol.
pub fn deserialize_update_identity_pool_request(body: &[u8]) -> Result<IdentityPool, String> {
    if body.is_empty() {
        return Ok(IdentityPool::default());
    }
    serde_json::from_slice(body)
        .map_err(|e| format!("Failed to deserialize UpdateIdentityPool request: {e}"))
}
